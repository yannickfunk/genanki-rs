use crate::builders::Template;
use crate::db_entries::{Fld, ModelDbEntry, Req, Tmpl};
use crate::Field;
use anyhow::anyhow;
use handlebars::Handlebars;
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

#[derive(Clone)]
pub enum ModelType {
    FrontBack,
    Cloze,
}

#[derive(Clone)]
pub struct Model {
    pub id: usize,
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
    pub fn new(id: usize, name: &str, fields: Vec<Field>, templates: Vec<Template>) -> Self {
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

    pub fn new_with_options(
        id: usize,
        name: String,
        fields: Vec<Fld>,
        templates: Vec<Tmpl>,
        css: Option<String>,
        model_type: Option<ModelType>,
        latex_pre: Option<String>,
        latex_post: Option<String>,
        sort_field_index: Option<i64>,
    ) -> Self {
        Self {
            id,
            name,
            fields,
            templates,
            css: css.unwrap_or("".to_string()),
            model_type: model_type.unwrap_or(ModelType::FrontBack),
            latex_pre: latex_pre.unwrap_or(DEFAULT_LATEX_PRE.to_string()),
            latex_post: latex_post.unwrap_or(DEFAULT_LATEX_POST.to_string()),
            sort_field_index: sort_field_index.unwrap_or(0),
        }
    }

    pub fn req(&self) -> Result<Vec<Vec<Req>>, anyhow::Error> {
        let mut handlebars = Handlebars::new();
        let sentinel = "SeNtInEl".to_string();
        let field_names: Vec<String> = self.fields.iter().map(|field| field.name.clone()).collect();

        let mut req = Vec::new();
        for (template_ord, template) in self.templates.iter().enumerate() {
            handlebars.register_template_string("t1", template.qfmt.clone())?;
            let mut field_values: HashMap<String, String> = field_names
                .iter()
                .map(|field| (field.clone(), sentinel.clone()))
                .collect();
            let mut required_fields = Vec::new();
            for (field_ord, field) in field_names.iter().enumerate() {
                let mut fvcopy = field_values.clone();
                fvcopy.insert(field.clone(), "".to_string());
                let rendered = handlebars.render("t1", &fvcopy)?;
                if !rendered.contains(&sentinel) {
                    required_fields.push(field_ord);
                }
            }
            if required_fields.len() > 0 {
                req.push(vec![
                    Req::Integer(template_ord),
                    Req::String("all".to_string()),
                    Req::IntegerArray(required_fields),
                ]);
                continue;
            }
            field_values = field_names
                .iter()
                .map(|field| (field.clone(), "".to_string()))
                .collect();
            for (field_ord, field) in field_names.iter().enumerate() {
                let mut fvcopy = field_values.clone();
                fvcopy.insert(field.clone(), sentinel.clone());
                let rendered = handlebars.render("t1", &fvcopy)?;
                if rendered.contains(&sentinel) {
                    required_fields.push(field_ord);
                }
            }
            if required_fields.len() == 0 {
                return Err(anyhow!(format!("Could not compute required fields for this template; please check the formatting of \"qfmt\": {:?}", template)));
            }

            req.push(vec![
                Req::Integer(template_ord),
                Req::String("any".to_string()),
                Req::IntegerArray(required_fields),
            ])
        }
        Ok(req)
    }

    pub fn fields(&self) -> Vec<Fld> {
        self.fields.clone()
    }

    pub fn templates(&self) -> Vec<Tmpl> {
        self.templates.clone()
    }

    pub fn model_type(&self) -> ModelType {
        self.model_type.clone()
    }
    pub fn to_model_db_entry(
        &mut self,
        timestamp: f64,
        deck_id: usize,
    ) -> Result<ModelDbEntry, anyhow::Error> {
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
    pub fn to_json(&mut self, timestamp: f64, deck_id: usize) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string(
            &self.to_model_db_entry(timestamp, deck_id)?,
        )?)
    }
}
