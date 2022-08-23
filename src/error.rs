use std::{convert::Infallible, time::SystemTimeError};

use zip::result::ZipError;

use crate::db_entries::Tmpl;

// Make sure `Error` is `Send` and `Sync`
const fn _assert_send<T: Send>() {}
const fn _assert_sync<T: Sync>() {}
const _: () = _assert_send::<Error>();
const _: () = _assert_sync::<Error>();

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Indicates an error happened with the database layer
    ///
    /// Currently the argument is a `rusqlite::Error`, but it is
    /// cast to a Box<dyn std::error::Error> so that we can change
    /// the underlying library in the future if needed without breaking
    /// client code.
    #[error(transparent)]
    Database(Box<dyn std::error::Error + Send + Sync>),
    /// Indicates an error happened with the JSON parser
    ///
    /// Currently the argument is a `serde_json::Error`, but it is
    /// cast to a Box<dyn std::error::Error> so that we can change
    /// the underlying library in the future if needed without breaking
    /// client code.
    #[error(transparent)]
    JsonParser(Box<dyn std::error::Error + Send + Sync>),
    #[error("Could not compute required fields for this template; please check the formatting of \"qfmt\": {0:?}")]
    TemplateFormat(Tmpl),
    #[error("number of model field ({0}) does not match number of fields ({1})")]
    ModelFieldCountMismatch(usize, usize),
    #[error("One of the tags contains whitespace, this is not allowed!")]
    TagContainsWhitespace,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Indicates an error with the underlying template system
    ///
    /// Currently the argument is a `ramhorns::Error`, but it is
    /// cast to a Box<dyn std::error::Error> so that we can change
    /// the underlying library in the future if needed without breaking
    /// client code.
    #[error(transparent)]
    Template(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    SystemTime(#[from] SystemTimeError),
    /// Indicates an error with zip file handling
    ///
    /// Currently the argument is a `zip::result::ZipError`, but it is
    /// cast to a Box<dyn std::error::Error> so that we can change
    /// the underlying library in the future if needed without breaking
    /// client code.
    #[error(transparent)]
    Zip(Box<dyn std::error::Error + Send + Sync>),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        // Infallible is uninhabited, so there's no way we can get to this code.
        unreachable!()
    }
}

pub(crate) fn database_error(e: rusqlite::Error) -> Error {
    Error::Template(Box::new(e))
}

pub(crate) fn json_error(e: serde_json::Error) -> Error {
    Error::Template(Box::new(e))
}

pub(crate) fn template_error(e: ramhorns::Error) -> Error {
    Error::Template(Box::new(e))
}

pub(crate) fn zip_error(e: ZipError) -> Error {
    Error::Zip(Box::new(e))
}
