use rusqlite::{Connection, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::NamedTempFile;
use zip::{write::FileOptions, ZipWriter};

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use crate::apkg_col::APKG_COL;
use crate::apkg_schema::APKG_SCHEMA;
use crate::deck::Deck;
use crate::error::{database_error, json_error, zip_error};
use crate::Error;
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
    media_files: Vec<MediaFile>,
}
/// the location of the media files, either as a path on the filesystem or as bytes from memory
pub enum MediaFile {
    /// a path on the filesystem
    Path(PathBuf),
    /// bytes from memory and a filename
    Bytes(Vec<u8>, String),
}
impl MediaFile {
    /// Create a new `MediaFile` from a path on the filesystem
    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Self {
        Self::Path(path.as_ref().to_path_buf())
    }

    /// Create a new `MediaFile` from a path on the filesystem
    pub fn new_from_file_path(path: &str) -> Result<Self, Error> {
        Ok(Self::Path(PathBuf::from_str(path)?))
    }

    /// Create a new `MediaFile` from bytes from memory and a filename
    pub fn new_from_bytes(bytes: &[u8], name: &str) -> Self {
        Self::Bytes(bytes.to_vec(), name.to_owned())
    }
}
impl Package {
    /// Create a new package with `decks` and `media_files`
    ///
    /// Returns `Err` if `media_files` are invalid
    pub fn new(decks: Vec<Deck>, media_files: Vec<&str>) -> Result<Self, Error> {
        let media_files = media_files
            .iter()
            .map(|&s| PathBuf::from_str(s).map(|p| MediaFile::Path(p)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { decks, media_files })
    }

    /// Create a new package with `decks` and `media_files`,
    /// where `media_files` can be bytes from memory or a path on the filesystem
    /// 
    /// Returns `Err` if `media_files` are invalid
    pub fn new_from_memory(decks: Vec<Deck>, media_files: Vec<MediaFile>) -> Result<Self, Error> {
        Ok(Self { decks, media_files })
    }

    /// Writes the package to any writer that implements Write and Seek
    pub fn write<W: Write + Seek>(&mut self, writer: W) -> Result<(), Error> {
        self.write_maybe_timestamp(writer, None)
    }

    /// Writes the package to any writer that implements Write and Seek using a timestamp
    pub fn write_timestamp<W: Write + Seek>(
        &mut self,
        writer: W,
        timestamp: f64,
    ) -> Result<(), Error> {
        self.write_maybe_timestamp(writer, Some(timestamp))
    }

    /// Writes the package to a file
    ///
    /// Returns `Err` if the `file` cannot be created
    pub fn write_to_file(&mut self, file: &str) -> Result<(), Error> {
        let file = File::create(file)?;
        self.write_maybe_timestamp(file, None)
    }

    /// Writes the package to a file using a timestamp
    ///
    /// Returns `Err` if the `file` cannot be created
    pub fn write_to_file_timestamp(&mut self, file: &str, timestamp: f64) -> Result<(), Error> {
        let file = File::create(file)?;
        self.write_maybe_timestamp(file, Some(timestamp))
    }

    fn write_maybe_timestamp<W: Write + Seek>(
        &mut self,
        writer: W,
        timestamp: Option<f64>,
    ) -> Result<(), Error> {
        let db_file = NamedTempFile::new()?.into_temp_path();

        let mut conn = Connection::open(&db_file).map_err(database_error)?;
        let transaction = conn.transaction().map_err(database_error)?;

        let timestamp = if let Some(timestamp) = timestamp {
            timestamp
        } else {
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64()
        };

        self.write_to_db(&transaction, timestamp)?;
        transaction.commit().map_err(database_error)?;
        conn.close().expect("Should always close");

        let mut outzip = ZipWriter::new(writer);
        outzip
            .start_file("collection.anki2", FileOptions::default())
            .map_err(zip_error)?;
        outzip.write_all(&read_file_bytes(db_file)?)?;

        let media_file_idx_to_path = self
            .media_files
            .iter()
            .enumerate()
            .collect::<HashMap<usize, &MediaFile>>();
        let media_map = media_file_idx_to_path
            .clone()
            .into_iter()
            .map(|(id, media_file)| {
                (
                    id.to_string(),
                    match media_file {
                        MediaFile::Path(path) => path.file_name()
                            .expect("Should always have a filename")
                            .to_str()
                            .expect("should always have string"),
                        MediaFile::Bytes(_, name) => name,
                    },
                )
            })
            .collect::<HashMap<String, &str>>();
        let media_json = serde_json::to_string(&media_map).map_err(json_error)?;
        outzip
            .start_file("media", FileOptions::default())
            .map_err(zip_error)?;
        outzip.write_all(media_json.as_bytes())?;

        for (idx, &media_file) in &media_file_idx_to_path {
            outzip
                .start_file(idx.to_string(), FileOptions::default())
                .map_err(zip_error)?;
            outzip.write_all(&match media_file {
                MediaFile::Path(path) => read_file_bytes(path)?,
                MediaFile::Bytes(bytes, _) => bytes.clone(),
            })?;
        }
        outzip.finish().map_err(zip_error)?;
        Ok(())
    }

    fn write_to_db(&mut self, transaction: &Transaction, timestamp: f64) -> Result<(), Error> {
        let mut id_gen = ((timestamp * 1000.0) as usize)..;
        transaction
            .execute_batch(APKG_SCHEMA)
            .map_err(database_error)?;
        transaction
            .execute_batch(APKG_COL)
            .map_err(database_error)?;
        for deck in &mut self.decks {
            deck.write_to_db(&transaction, timestamp, &mut id_gen)?;
        }
        Ok(())
    }
}

fn read_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut handle = File::open(path)?;
    let mut data = Vec::new();
    handle.read_to_end(&mut data)?;
    Ok(data)
}
