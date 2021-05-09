use rusqlite::{Connection, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::NamedTempFile;
use zip::{write::FileOptions, ZipWriter};

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::apkg_col::APKG_COL;
use crate::apkg_schema::APKG_SCHEMA;
use crate::deck::Deck;
use std::str::FromStr;

/// `Package` to pack `Deck`s and `media_files` and write them to a `.apkg` file
///
/// Example:
/// ```rust
/// use genanki_rs::{Package, Deck, Note, Model, Field, Template};
///
/// let model = Model::new(
///     1607392319,
///     "Simple Model",
///     vec![
///         Field::new("Question"),
///         Field::new("Answer"),
///         Field::new("MyMedia"),
///     ],
///     vec![Template::new("Card 1")
///         .qfmt("{{Question}}{{Question}}<br>{{MyMedia}}")
///         .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
/// );
///
/// let mut deck = Deck::new(1234, "Example Deck", "Example Deck with media");
/// deck.add_note(Note::new(model.clone(), vec!["What is the capital of France?", "Paris", "[sound:sound.mp3]"])?);
/// deck.add_note(Note::new(model.clone(), vec!["What is the capital of France?", "Paris", r#"<img src="image.jpg">"#])?);
///
/// let mut package = Package::new(vec![my_deck], vec!["sound.mp3", "images/image.jpg"])?;
/// package.write_to_file("output.apkg")?;
/// ```
pub struct Package {
    decks: Vec<Deck>,
    media_files: Vec<PathBuf>,
}

impl Package {
    /// Create a new package with `decks` and `media_files`
    ///
    /// Returns `Err` if `media_files` are invalid
    pub fn new(decks: Vec<Deck>, media_files: Vec<&str>) -> Result<Self, anyhow::Error> {
        let media_files = media_files
            .iter()
            .map(|&s| PathBuf::from_str(s))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { decks, media_files })
    }

    /// Writes the package to a file
    ///
    /// Returns `Err` if the `file` cannot be created
    pub fn write_to_file(&mut self, file: &str) -> Result<(), anyhow::Error> {
        self.write_to_file_maybe_timestamp(file, None)
    }

    /// Writes the package to a file using a timestamp
    ///
    /// Returns `Err` if the `file` cannot be created
    pub fn write_to_file_timestamp(
        &mut self,
        file: &str,
        timestamp: f64,
    ) -> Result<(), anyhow::Error> {
        self.write_to_file_maybe_timestamp(file, Some(timestamp))
    }

    fn write_to_file_maybe_timestamp(
        &mut self,
        file: &str,
        timestamp: Option<f64>,
    ) -> Result<(), anyhow::Error> {
        let file = File::create(&file)?;
        let db_file = NamedTempFile::new()?.into_temp_path();

        let mut conn = Connection::open(&db_file)?;
        let transaction = conn.transaction()?;

        let timestamp = if let Some(timestamp) = timestamp {
            timestamp
        } else {
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64()
        };

        self.write_to_db(&transaction, timestamp)?;
        transaction.commit()?;
        conn.close().expect("Should always close");

        let mut outzip = ZipWriter::new(file);
        outzip.start_file("collection.anki2", FileOptions::default())?;
        outzip.write_all(&read_file_bytes(db_file)?)?;

        let media_file_idx_to_path = self
            .media_files
            .iter()
            .enumerate()
            .collect::<HashMap<usize, &PathBuf>>();
        let media_map = media_file_idx_to_path
            .clone()
            .into_iter()
            .map(|(id, path)| {
                (
                    id.to_string(),
                    path.file_name()
                        .expect("Should always have a filename")
                        .to_str()
                        .expect("should always have string"),
                )
            })
            .collect::<HashMap<String, &str>>();
        let media_json = serde_json::to_string(&media_map)?;
        outzip.start_file("media", FileOptions::default())?;
        outzip.write_all(media_json.as_bytes())?;

        for (idx, &path) in &media_file_idx_to_path {
            outzip.start_file(idx.to_string(), FileOptions::default())?;
            outzip.write_all(&read_file_bytes(path)?)?;
        }
        outzip.finish()?;
        Ok(())
    }

    fn write_to_db(
        &mut self,
        transaction: &Transaction,
        timestamp: f64,
    ) -> Result<(), anyhow::Error> {
        let mut id_gen = ((timestamp * 1000.0) as usize)..;
        transaction.execute_batch(APKG_SCHEMA)?;
        transaction.execute_batch(APKG_COL)?;
        for deck in &mut self.decks {
            deck.write_to_db(&transaction, timestamp, &mut id_gen)?;
        }
        Ok(())
    }
}

fn read_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, anyhow::Error> {
    let mut handle = File::open(path)?;
    let mut data = Vec::new();
    handle.read_to_end(&mut data)?;
    Ok(data)
}
