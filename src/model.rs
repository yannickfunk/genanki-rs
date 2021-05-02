use crate::db_entries::{Fld, ModelDbEntry, Req, Tmpl};

const FRONT_BACK: i64 = 0;
const CLOZE: i64 = 1;
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

#[derive(Clone)]
pub struct Model {
    pub id: usize,
    name: String,
    fields: Vec<Fld>,
    templates: Vec<Tmpl>,
    css: String,
    model_type: i64,
    latex_pre: String,
    latex_post: String,
    sort_field_index: i64,
}

impl Model {
    pub fn new_with_defaults(
        id: usize,
        name: String,
        fields: Vec<Fld>,
        templates: Vec<Tmpl>,
    ) -> Self {
        Self {
            id,
            name,
            fields,
            templates,
            css: "".to_string(),
            model_type: FRONT_BACK,
            latex_pre: DEFAULT_LATEX_PRE.to_string(),
            latex_post: DEFAULT_LATEX_POST.to_string(),
            sort_field_index: 0,
        }
    }

    pub fn new_with_options(
        id: usize,
        name: String,
        fields: Vec<Fld>,
        templates: Vec<Tmpl>,
        css: String,
        model_type: i64,
        latex_pre: String,
        latex_post: String,
        sort_field_index: i64,
    ) -> Self {
        Self {
            id,
            name,
            fields,
            templates,
            css,
            model_type,
            latex_pre,
            latex_post,
            sort_field_index,
        }
    }

    fn req(&self) -> Vec<Vec<Req>> {
        vec![vec![]]
    }

    pub fn fields(&self) -> Vec<Fld> {
        self.fields.clone()
    }

    pub fn to_model_db_entry(&self, timestamp: f64, deck_id: usize) -> ModelDbEntry {
        ModelDbEntry {
            vers: vec![],
            name: self.name.clone(),
            tags: vec![],
            did: deck_id,
            usn: -1,
            req: self.req().clone(),
            flds: self.fields.clone(),
            sortf: self.sort_field_index.clone(),
            tmpls: self.templates.clone(),
            model_db_entry_mod: timestamp as i64,
            latex_post: self.latex_post.clone(),
            model_db_entry_type: self.model_type.clone(),
            id: self.id.to_string(),
            css: self.css.clone(),
            latex_pre: self.latex_pre.clone(),
        }
    }
    pub fn to_json(&self, timestamp: f64, deck_id: usize) -> String {
        serde_json::to_string(&self.to_model_db_entry(timestamp, deck_id))
            .expect("Should always serialize")
    }
}
