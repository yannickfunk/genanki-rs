use super::Package;
use crate::db_entries::{DeckDbEntry, ModelDbEntry};
use crate::model::Model;
use crate::note::Note;
use rusqlite::{params, Transaction};
use serde_json::json;
use std::collections::HashMap;
use std::ops::RangeFrom;

#[derive(Clone)]
pub struct Deck {
    id: usize,
    name: String,
    description: String,
    notes: Vec<Note>,
    models: HashMap<usize, Model>,
}

impl Deck {
    pub fn new(id: usize, name: &str, description: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            notes: vec![],
            models: HashMap::new(),
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.insert(model.id, model);
    }

    pub fn to_deck_db_entry(&self) -> DeckDbEntry {
        DeckDbEntry {
            collapsed: false,
            conf: 1,
            desc: self.description.clone(),
            deck_db_entry_dyn: 0,
            extend_new: 0,
            extend_rev: 50,
            id: self.id.clone(),
            lrn_today: vec![163, 2],
            deck_db_entry_mod: 1425278051,
            name: self.name.clone(),
            new_today: vec![163, 2],
            rev_today: vec![163, 0],
            time_today: vec![163, 23598],
            usn: -1,
        }
    }
    pub fn to_json(&self) -> String {
        let db_entry: DeckDbEntry = self.to_deck_db_entry();
        serde_json::to_string(&db_entry).expect("Should always serialize")
    }

    pub fn write_to_db(
        &mut self,
        transaction: &Transaction,
        timestamp: f64,
        id_gen: &mut RangeFrom<usize>,
    ) -> Result<(), anyhow::Error> {
        let decks_json_str: String =
            transaction.query_row("SELECT decks FROM col", [], |row| row.get(0))?;
        let mut decks: HashMap<usize, DeckDbEntry> = serde_json::from_str(&decks_json_str)?;
        decks.insert(self.id, self.to_deck_db_entry());
        transaction.execute(
            "UPDATE col SET decks = ?",
            params![serde_json::to_string(&decks)?],
        )?;

        let models_json_str: String =
            transaction.query_row("SELECT models FROM col", [], |row| row.get(0))?;
        let mut models: HashMap<usize, ModelDbEntry> = serde_json::from_str(&models_json_str)?;
        for note in self.notes.clone().iter() {
            self.add_model(note.model());
        }
        for (i, model) in &mut self.models {
            models.insert(*i, model.to_model_db_entry(timestamp, self.id)?);
        }
        transaction.execute(
            "UPDATE col SET models = ?",
            [serde_json::to_string(&models)?],
        )?;
        for note in &mut self.notes {
            note.write_to_db(&transaction, timestamp, self.id, id_gen)?;
        }
        Ok(())
    }

    pub fn write_to_file(self, file: &str) -> Result<(), anyhow::Error> {
        Package::new(vec![self], vec![])?.write_to_file(file)?;
        Ok(())
    }
}
