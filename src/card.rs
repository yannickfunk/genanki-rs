use rusqlite::{params, Transaction};

#[derive(Clone)]
pub struct Card {
    ord: i64,
    suspend: bool,
}

impl Card {
    pub fn new(ord: i64, suspend: bool) -> Self {
        Self { ord, suspend }
    }

    pub fn write_to_db(
        &self,
        transaction: &Transaction,
        timestamp: f64,
        deck_id: usize,
        note_id: usize,
    ) -> Result<(), anyhow::Error> {
        let queue = if self.suspend { -1 } else { 0 };
        transaction.execute(
            "INSERT INTO cards VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);",
            params![
                (timestamp * 1000.0 + 1.0) as usize, // id
                note_id,                             // nid
                deck_id,                             // did
                self.ord,                            // ord
                timestamp as i64,                    // mod
                -1,                                  // usn
                0,                                   // type (=0 for non-Cloze)
                queue,                               // queue
                0,                                   // due
                0,                                   // ivl
                0,                                   // factor
                0,                                   // reps
                0,                                   // lapses
                0,                                   // left
                0,                                   // odue
                0,                                   // odid
                0,                                   // flags
                "",                                  // data
            ],
        )?;
        Ok(())
    }
}
