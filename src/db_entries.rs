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
    pub id: i64,
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

#[derive(Serialize, Deserialize)]
pub struct ModelDbEntry {
    pub vers: Vec<Option<serde_json::Value>>,
    pub name: String,
    pub tags: Vec<Option<serde_json::Value>>,
    pub did: i64,
    pub usn: i64,
    pub req: Vec<(usize, String, Vec<usize>)>,
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
    pub name: String,
    pub media: Vec<Option<serde_json::Value>>,
    pub sticky: bool,
    pub rtl: bool,
    pub ord: i64,
    pub font: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tmpl {
    pub name: String,
    pub qfmt: String,
    pub did: Option<usize>,
    pub bafmt: String,
    pub afmt: String,
    pub ord: i64,
    pub bqfmt: String,
}
