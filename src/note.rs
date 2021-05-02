use crate::card::Card;
use crate::model::{Model, ModelType};
use anyhow::anyhow;
use regex::Regex;
use rusqlite::{params, Transaction};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone)]
pub struct Note {
    model: Model,
    fields: Vec<String>,
    sort_field: bool,
    tags: Vec<String>,
    guid: Option<String>,
    cards: Vec<Card>,
}

impl Note {
    pub fn new(
        model: Model,
        fields: Vec<String>,
        sort_field: bool,
        tags: Vec<String>,
        guid: Option<String>,
    ) -> Self {
        let cards = match model.model_type() {
            ModelType::FrontBack => front_back_cards(),
            ModelType::Cloze => cloze_cards(&model, &fields),
        };
        Self {
            model,
            fields,
            sort_field,
            tags,
            guid,
            cards,
        }
    }
    pub fn model(&self) -> Model {
        self.model.clone()
    }

    fn check_number_model_fields_matches_num_fields(&self) -> Result<(), anyhow::Error> {
        if self.model.fields().len() != self.fields.len() {
            Err(anyhow!("number model field does not match num fields"))
        } else {
            Ok(())
        }
    }

    fn check_invalid_html_tags_in_fields(&self) -> Result<(), anyhow::Error> {
        //TODO:
        Ok(())
    }

    fn format_fields(&self) -> String {
        self.fields.clone().join("\x1f")
    }

    fn format_tags(&self) -> String {
        format!(" {} ", self.tags.join(" "))
    }
    pub fn write_to_db(
        &self,
        transaction: &Transaction,
        timestamp: f64,
        deck_id: usize,
    ) -> Result<(), anyhow::Error> {
        self.check_number_model_fields_matches_num_fields()?;
        self.check_invalid_html_tags_in_fields()?;
        transaction.execute(
            "INSERT INTO notes VALUES(?,?,?,?,?,?,?,?,?,?,?);",
            params![
                (timestamp * 1000.0) as usize, // id
                self.guid.as_ref().unwrap(),   // guid
                self.model.id,                 // mid
                timestamp as i64,              // mod
                -1,                            // usn
                self.format_tags(),            // TODO tags
                self.format_fields(),          // flds
                self.sort_field,               // sfld
                0,                             // csum, can be ignored
                0,                             // flags
                "",                            // data
            ],
        )?;
        let note_id = transaction.last_insert_rowid() as usize;
        for card in &self.cards {
            card.write_to_db(transaction, timestamp, deck_id, note_id)?
        }
        Ok(())
    }
}

fn cloze_cards(model: &Model, self_fields: &Vec<String>) -> Vec<Card> {
    let mut card_ords: HashSet<i64> = HashSet::new();
    let mut cloze_replacements: HashSet<String> = HashSet::new();
    cloze_replacements.extend(re_findall(
        r"\{\{[^}]*?cloze:(?:[^}]?:)*(.+?)}}",
        &model.templates()[0].qfmt,
    ));
    cloze_replacements.extend(re_findall("<%cloze:(.+?)%>", &model.templates()[0].qfmt));
    for field_name in cloze_replacements {
        let fields = model.fields();
        let mut field_index_iter = fields
            .iter()
            .filter(|field| field.name == field_name)
            .enumerate()
            .map(|(i, _)| i);
        let field_value = if let Some(field_index) = field_index_iter.next() {
            self_fields[field_index].clone()
        } else {
            "".to_string()
        };
        let updates_str = re_findall(r"\{\{c(\d+)::.+?}}", &field_value);
        let updates = updates_str
            .iter()
            .map(|m| i64::from_str(m).expect("parsed from regex") - 1)
            .filter(|&m| m >= 0);
        card_ords.extend(updates);
    }
    if card_ords.len() == 0 {
        card_ords.insert(0);
    }
    card_ords
        .iter()
        .map(|&card_ord| Card::new(card_ord, false))
        .collect()
}

fn front_back_cards() -> Vec<Card> {
    vec![]
}

fn re_findall(regex_str: &'static str, to_match: &str) -> Vec<String> {
    let regex = Regex::new(regex_str).expect("static regex");
    if let Some(caps) = regex.captures(to_match) {
        caps.iter()
            .skip(1)
            .filter_map(|m| m)
            .map(|m| m.as_str().to_string())
            .collect()
    } else {
        vec![]
    }
}
