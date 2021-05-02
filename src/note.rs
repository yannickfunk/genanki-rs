use crate::model::Model;
use anyhow::anyhow;
use rusqlite::{params, Transaction};

#[derive(Clone)]
pub struct Note {
    model: Option<Model>,
    fields: Vec<String>,
    sort_field: bool,
    tags: Vec<String>,
    guid: Option<String>,
}

impl Note {
    pub fn model(&self) -> Option<Model> {
        self.model.clone()
    }

    fn check_number_model_fields_matches_num_fields(&self) -> Result<(), anyhow::Error> {
        if self.model.as_ref().unwrap().fields().len() != self.fields.len() {
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
                (timestamp * 1000.0) as usize,   // id
                self.guid.as_ref().unwrap(),     // guid
                self.model.as_ref().unwrap().id, // mid
                timestamp as i64,                // mod
                -1,                              // usn
                self.format_tags(),              // TODO tags
                self.format_fields(),            // flds
                self.sort_field,                 // sfld
                0,                               // csum, can be ignored
                0,                               // flags
                "",                              // data
            ],
        )?;
        let note_id = transaction.last_insert_rowid();
        /*for card in &self.cards {
            card.write_to_db(transaction, timestamp, deck_id, note_id)
        }*/
        Ok(())
    }
}
