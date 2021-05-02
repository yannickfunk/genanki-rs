use crate::deck::Deck;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeckDbEntry {
    pub collapsed: bool,
    pub conf: i64,
    pub desc: String,
    #[serde(rename = "dyn")]
    pub deck_db_entry_dyn: i64,
    #[serde(rename = "extendNew")]
    pub extend_new: i64,
    #[serde(rename = "extendRev")]
    pub extend_rev: i64,
    pub id: usize,
    #[serde(rename = "lrnToday")]
    pub lrn_today: Vec<i64>,
    #[serde(rename = "mod")]
    pub deck_db_entry_mod: i64,
    pub name: String,
    #[serde(rename = "newToday")]
    pub new_today: Vec<i64>,
    #[serde(rename = "revToday")]
    pub rev_today: Vec<i64>,
    #[serde(rename = "timeToday")]
    pub time_today: Vec<i64>,
    pub usn: i64,
}

impl From<Deck> for DeckDbEntry {
    fn from(deck: Deck) -> Self {
        deck.into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModelDbEntry {
    pub vers: Vec<Option<serde_json::Value>>,
    pub name: String,
    pub tags: Vec<Option<serde_json::Value>>,
    pub did: usize,
    pub usn: i64,
    pub req: Vec<Vec<Req>>,
    pub flds: Vec<Fld>,
    pub sortf: i64,
    pub tmpls: Vec<Tmpl>,
    #[serde(rename = "mod")]
    pub model_db_entry_mod: i64,
    #[serde(rename = "latexPost")]
    pub latex_post: String,
    #[serde(rename = "type")]
    pub model_db_entry_type: i64,
    pub id: String,
    pub css: String,
    #[serde(rename = "latexPre")]
    pub latex_pre: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Fld {
    name: String,
    media: Vec<Option<serde_json::Value>>,
    sticky: bool,
    rtl: bool,
    ord: i64,
    font: String,
    size: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tmpl {
    name: String,
    qfmt: String,
    did: Option<serde_json::Value>,
    bafmt: String,
    afmt: String,
    ord: i64,
    bqfmt: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Req {
    Integer(i64),
    IntegerArray(Vec<i64>),
    String(String),
}
