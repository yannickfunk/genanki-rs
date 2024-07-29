use crate::builders::Template;
use crate::db_entries::{Fld, ModelDbEntry, Tmpl};
use crate::error::{json_error, template_error};
use crate::{Error, Field};
use fancy_regex::Regex;
use ramhorns::Template as RamTemplate;
use std::collections::HashMap;

const DEFAULT_LATEX_PRE: &str = r#"
\documentclass[12pt]{article}
\special{papersize=3in,5in}
\usepackage[utf8]{inputenc}
\usepackage{amssymb,amsmath}
\pagestyle{empty}
\setlength{\parindent}{0in}
\begin{document}

"#;
const DEFAULT_LATEX_POST: &str = r"\end{document}";

/// `FrontBack` or `Cloze` to determine the type of a Model.
///
/// When creating a Model, the default is `FrontBack`
#[derive(Clone, PartialEq, Eq)]
pub enum ModelType {
    FrontBack,
    Cloze,
}

/// `Model` to determine the structure of a `Note`
#[derive(Clone)]
pub struct Model {
    pub id: i64,
    name: String,
    fields: Vec<Fld>,
    templates: Vec<Tmpl>,
    css: String,
    model_type: ModelType,
    latex_pre: String,
    latex_post: String,
    sort_field_index: i64,
}

impl Model {
    /// Creates a new model with a unique(!) `id`, a `name`, `fields` and  `templates`
    ///
    /// Example:
    ///
    /// ```
    /// use genanki_rs::{Model, Field, Template};
    /// let model = Model::new(
    ///     1607392319,
    ///     "Simple Model",
    ///     vec![Field::new("Question"), Field::new("Answer")],
    ///     vec![Template::new("Card 1")
    ///         .qfmt("{{Question}}")
    ///         .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    /// );
    /// ```
    pub fn new(id: i64, name: &str, fields: Vec<Field>, templates: Vec<Template>) -> Self {
        Self {
            id,
            name: name.to_string(),
            fields: fields.iter().cloned().map(|f| f.into()).collect(),
            templates: templates.iter().cloned().map(|t| t.into()).collect(),
            css: "".to_string(),
            model_type: ModelType::FrontBack,
            latex_pre: DEFAULT_LATEX_PRE.to_string(),
            latex_post: DEFAULT_LATEX_POST.to_string(),
            sort_field_index: 0,
        }
    }

    /// Creates a new model with a unique(!) `ìd`, a `name`, `fields` and  `templates` and custom parameters:
    /// * `css`: Custom css to be applied to the cards
    /// * `model_type`: `Cloze` or `FrontBack`, default is `FrontBack`
    /// * `latex_pre`: Custom latex declarations at the beginning of a card.
    /// * `latex_post`: Custom latex declarations at the end of a card.
    /// * `sort_field_index`: Custom sort field index
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_options(
        id: i64,
        name: &str,
        fields: Vec<Field>,
        templates: Vec<Template>,
        css: Option<&str>,
        model_type: Option<ModelType>,
        latex_pre: Option<&str>,
        latex_post: Option<&str>,
        sort_field_index: Option<i64>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            fields: fields.iter().cloned().map(|f| f.into()).collect(),
            templates: templates.iter().cloned().map(|t| t.into()).collect(),
            css: css.unwrap_or("").to_string(),
            model_type: model_type.unwrap_or(ModelType::FrontBack),
            latex_pre: latex_pre.unwrap_or(DEFAULT_LATEX_PRE).to_string(),
            latex_post: latex_post.unwrap_or(DEFAULT_LATEX_POST).to_string(),
            sort_field_index: sort_field_index.unwrap_or(0),
        }
    }

    /// Adds an additional field to the model
    pub fn with_field(mut self, field: Field) -> Self {
        self.fields.push(field.into());
        self
    }

    /// Adds an additional template to the model
    pub fn with_template(mut self, template: Template) -> Self {
        self.templates.push(template.into());
        self
    }

    /// Sets the custom CSS for this model
    pub fn css(self, css: impl ToString) -> Self {
        Self {
            css: css.to_string(),
            ..self
        }
    }

    /// Change the type of the model
    pub fn model_type(self, model_type: ModelType) -> Self {
        Self { model_type, ..self }
    }

    /// Sets the model's latex_pre field
    pub fn latex_pre(self, latex_pre: impl ToString) -> Self {
        Self {
            latex_pre: latex_pre.to_string(),
            ..self
        }
    }

    /// Sets the model's latex_post field
    pub fn latex_post(self, latex_post: impl ToString) -> Self {
        Self {
            latex_post: latex_post.to_string(),
            ..self
        }
    }

    /// Sets the index of the field used for sorting with this model
    pub fn sort_field_index(self, sort_field_index: i64) -> Self {
        Self {
            sort_field_index,
            ..self
        }
    }

    pub(super) fn req(&self) -> Result<Vec<(usize, String, Vec<usize>)>, Error> {
        let sentinel = "SeNtInEl".to_string();
        let field_names: Vec<String> = self.fields.iter().map(|field| field.name.clone()).collect();
        let field_values = field_names
            .iter()
            .map(|field| (field.as_str(), format!("{}{}", &field, &sentinel)));
        let mut req = Vec::new();
        for (template_ord, template) in self.templates.iter().enumerate() {
            let rendered = RamTemplate::new(template.qfmt.clone())
                .map_err(template_error)?
                .render::<HashMap<&str, String>>(&field_values.clone().collect());
            let required_fields = field_values
                .clone()
                .enumerate()
                .filter(|(_, (_, field))| !contains_other_fields(&rendered, field, &sentinel))
                .map(|(field_ord, _)| field_ord)
                .collect::<Vec<_>>();
            if !required_fields.is_empty() {
                req.push((template_ord, "all".to_string(), required_fields));
                continue;
            }
            let required_fields = field_values
                .clone()
                .enumerate()
                .filter(|(_, (_, sentinel))| rendered.contains(sentinel))
                .map(|(field_ord, _)| field_ord)
                .collect::<Vec<_>>();
            if required_fields.is_empty() {
                return Err(Error::TemplateFormat(template.clone()));
            }
            req.push((template_ord, "any".to_string(), required_fields))
        }
        Ok(req)
    }

    pub(super) fn fields(&self) -> Vec<Fld> {
        self.fields.clone()
    }
    pub(super) fn templates(&self) -> Vec<Tmpl> {
        self.templates.clone()
    }
    pub(super) fn get_model_type(&self) -> ModelType {
        self.model_type.clone()
    }
    pub(super) fn to_model_db_entry(
        &mut self,
        timestamp: f64,
        deck_id: i64,
    ) -> Result<ModelDbEntry, Error> {
        self.templates
            .iter_mut()
            .enumerate()
            .for_each(|(i, template)| {
                template.ord = i as i64;
            });
        self.fields.iter_mut().enumerate().for_each(|(i, field)| {
            field.ord = i as i64;
        });
        let model_type = match self.model_type {
            ModelType::FrontBack => 0,
            ModelType::Cloze => 1,
        };
        Ok(ModelDbEntry {
            vers: vec![],
            name: self.name.clone(),
            tags: vec![],
            did: deck_id,
            usn: -1,
            req: self.req()?.clone(),
            flds: self.fields.clone(),
            sortf: self.sort_field_index.clone(),
            tmpls: self.templates.clone(),
            model_db_entry_mod: timestamp as i64,
            latex_post: self.latex_post.clone(),
            model_db_entry_type: model_type,
            id: self.id.to_string(),
            css: self.css.clone(),
            latex_pre: self.latex_pre.clone(),
        })
    }

    #[allow(dead_code)]
    pub(super) fn to_json(&mut self, timestamp: f64, deck_id: i64) -> Result<String, Error> {
        Ok(
            serde_json::to_string(&self.to_model_db_entry(timestamp, deck_id)?)
                .map_err(json_error)?,
        )
    }
}

fn contains_other_fields(rendered: &str, current_field: &str, sentinel: &str) -> bool {
    Regex::new(&format!(
        "(?!{field}\\b)\\b(\\w)*{sentinel}+",
        field = current_field,
        sentinel = sentinel
    ))
    .unwrap()
    .is_match(rendered)
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Deck, Note};
    use std::collections::HashSet;
    use tempfile::NamedTempFile;

    fn css() -> String {
        r#".card {
 font-family: arial;
 font-size: 20px;
 text-align: center;
 color: black;
 background-color: white;
}

.cloze {
 font-weight: bold;
 color: blue;
}
.nightMode .cloze {
 color: lightblue;
}
"#
        .to_owned()
    }

    fn cloze_model() -> Model {
        Model::new_with_options(
            998877661,
            "Cloze Model",
            vec![Field::new("Text"), Field::new("Extra")],
            vec![Template::new("My Cloze Card")
                .qfmt("{{cloze:Text}}")
                .afmt("{{cloze:Text}}<br>{{Extra}}")],
            Some(&css()),
            Some(ModelType::Cloze),
            None,
            None,
            None,
        )
    }

    fn multi_field_cloze_model() -> Model {
        Model::new_with_options(
            1047194615,
            "Multi Field Cloze Model",
            vec![Field::new("Text1"), Field::new("Text2")],
            vec![Template::new("Cloze")
                .qfmt("{{cloze:Text1}} and {{cloze:Text2}}")
                .afmt("{{cloze:Text1}} and {{cloze:Text2}}")],
            Some(&css()),
            Some(ModelType::Cloze),
            None,
            None,
            None,
        )
    }

    #[test]
    fn cloze() {
        let mut notes = vec![];
        let model = cloze_model();
        assert!(matches!(model.model_type, ModelType::Cloze));

        // Question: NOTE ONE: [...]
        // Answer:   NOTE ONE: single deletion
        let fields = vec!["NOTE ONE: {{c1::single deletion}}", ""];
        let cloze_note = Note::new(model.clone(), fields).unwrap();
        let card_ord_set = cloze_note
            .cards()
            .iter()
            .map(|card| card.ord())
            .collect::<HashSet<i64>>();
        assert_eq!(card_ord_set.len(), 1);
        assert_eq!(*card_ord_set.get(&0).unwrap(), 0);
        notes.push(cloze_note);

        // Question: NOTE TWO: [...]              2nd deletion     3rd deletion
        // Answer:   NOTE TWO: **1st deletion**   2nd deletion     3rd deletion
        //
        // Question: NOTE TWO: 1st deletion       [...]            3rd deletion
        // Answer:   NOTE TWO: 1st deletion     **2nd deletion**   3rd deletion
        //
        // Question: NOTE TWO: 1st deletion       2nd deletion     [...]
        // Answer:   NOTE TWO: 1st deletion       2nd deletion   **3rd deletion**
        let fields = vec![
            "NOTE TWO: {{c1::1st deletion}} {{c2::2nd deletion}} {{c3::3rd deletion}}",
            "",
        ];
        let cloze_note = Note::new(model.clone(), fields).unwrap();
        let mut sorted = cloze_note
            .cards()
            .iter()
            .map(|card| card.ord())
            .collect::<Vec<i64>>();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2]);
        notes.push(cloze_note);

        // Question: NOTE THREE: C1-CLOZE
        // Answer:   NOTE THREE: 1st deletion
        let fields = vec!["NOTE THREE: {{c1::1st deletion::C1-CLOZE}}", ""];
        let cloze_note = Note::new(model.clone(), fields).unwrap();
        let card_ord_set = cloze_note
            .cards()
            .iter()
            .map(|card| card.ord())
            .collect::<HashSet<i64>>();
        assert_eq!(card_ord_set.len(), 1);
        assert_eq!(*card_ord_set.get(&0).unwrap(), 0);
        notes.push(cloze_note);

        // Question: NOTE FOUR: [...] foo 2nd deletion bar [...]
        // Answer:   NOTE FOUR: 1st deletion foo 2nd deletion bar 3rd deletion
        let fields = vec![
            "NOTE FOUR: {{c1::1st deletion}} foo {{c2::2nd deletion}} bar {{c1::3rd deletion}}",
            "",
        ];
        let cloze_note = Note::new(model.clone(), fields).unwrap();
        let mut sorted = cloze_note
            .cards()
            .iter()
            .map(|card| card.ord())
            .collect::<Vec<i64>>();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1]);
        notes.push(cloze_note);

        let mut deck = Deck::new(0, "test", "");
        notes.iter().for_each(|note| deck.add_note(note.clone()));
        let out_file = NamedTempFile::new().unwrap().into_temp_path();
        deck.write_to_file(out_file.to_str().unwrap()).unwrap();
    }

    #[test]
    fn cloze_multi_field() {
        let fields = vec![
            "{{c1::Berlin}} is the capital of {{c2::Germany}}",
            "{{c3::Paris}} is the capital of {{c4::France}}",
        ];
        let note = Note::new(multi_field_cloze_model(), fields).unwrap();
        let mut sorted = note
            .cards()
            .iter()
            .map(|card| card.ord())
            .collect::<Vec<i64>>();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2, 3]);
    }

    #[test]
    fn cloze_indices_do_not_start_at_1() {
        let fields = vec![
            "{{c2::Mitochondria}} are the {{c3::powerhouses}} of the cell",
            "",
        ];
        let note = Note::new(cloze_model(), fields).unwrap();
        let mut sorted = note
            .cards()
            .iter()
            .map(|card| card.ord())
            .collect::<Vec<i64>>();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![1, 2]);
    }

    #[test]
    fn cloze_newlines_in_deletion() {
        let fields = vec![
            "{{c1::Washington, D.C.}} is the capital of {{c2::the\nUnited States\nof America}}",
            "",
        ];
        let note = Note::new(cloze_model(), fields).unwrap();
        let mut sorted = note
            .cards()
            .iter()
            .map(|card| card.ord())
            .collect::<Vec<i64>>();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1]);
    }

    #[test]
    fn build_all_fields() {
        // A simple test to make sure we can call all the setters on the builder.
        //
        // It doesn't actually verify any behavior, it's basically just a smoke test.
        Model::new(12345, "test model", vec![Field::new("front")], vec![])
            .with_template(Template::new("template"))
            .css(css())
            .latex_post("")
            .latex_pre("")
            .sort_field_index(1)
            .model_type(ModelType::FrontBack);
    }
}
