use crate::db_entries::Tmpl;

#[derive(Clone)]
pub struct Template {
    name: String,
    qfmt: Option<String>,
    did: Option<usize>,
    bafmt: Option<String>,
    afmt: Option<String>,
    bqfmt: Option<String>,
}

impl Template {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            qfmt: None,
            did: None,
            bafmt: None,
            afmt: None,
            bqfmt: None,
        }
    }

    pub fn qfmt(mut self, qfmt: &str) -> Self {
        self.qfmt = Some(qfmt.to_string());
        self
    }

    pub fn did(mut self, did: usize) -> Self {
        self.did = Some(did);
        self
    }

    pub fn bafmt(mut self, bafmt: &str) -> Self {
        self.bafmt = Some(bafmt.to_string());
        self
    }

    pub fn afmt(mut self, afmt: &str) -> Self {
        self.afmt = Some(afmt.to_string());
        self
    }

    pub fn bqfmt(mut self, bqfmt: &str) -> Self {
        self.bqfmt = Some(bqfmt.to_string());
        self
    }
}

impl Into<Tmpl> for Template {
    fn into(self) -> Tmpl {
        Tmpl {
            name: self.name,
            qfmt: self.qfmt.unwrap_or("".to_string()),
            did: self.did,
            bafmt: self.bafmt.unwrap_or("".to_string()),
            afmt: self.afmt.unwrap_or("".to_string()),
            ord: 0,
            bqfmt: self.bqfmt.unwrap_or("".to_string()),
        }
    }
}
