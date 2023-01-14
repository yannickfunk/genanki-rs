use rusqlite::{params, Transaction};
use std::ops::RangeFrom;

use crate::{error::database_error, Error};

#[derive(Clone)]
pub struct Card {
    pub ord: i64,
    pub suspend: bool,
}

impl Card {
    pub fn new(ord: i64, suspend: bool) -> Self {
        Self { ord, suspend }
    }
    #[allow(dead_code)]
    pub fn ord(&self) -> i64 {
        self.ord
    }
    pub fn write_to_db(
        &self,
        transaction: &Transaction,
        timestamp: f64,
        deck_id: i64,
        note_id: usize,
        id_gen: &mut RangeFrom<usize>,
    ) -> Result<(), Error> {
        let queue = if self.suspend { -1 } else { 0 };
        transaction
            .execute(
                "INSERT INTO cards VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);",
                params![
                    id_gen.next(),    // id
                    note_id,          // nid
                    deck_id,          // did
                    self.ord,         // ord
                    timestamp as i64, // mod
                    -1,               // usn
                    0,                // type (=0 for non-Cloze)
                    queue,            // queue
                    0,                // due
                    0,                // ivl
                    0,                // factor
                    0,                // reps
                    0,                // lapses
                    0,                // left
                    0,                // odue
                    0,                // odid
                    0,                // flags
                    "",               // data
                ],
            )
            .map_err(database_error)?;
        Ok(())
    }
}
