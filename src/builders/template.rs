use crate::db_entries::Tmpl;

/// Template to be fed into a `Model`.
/// A Template represents the structure of `Notes` (Flashcards) in the deck and can be created using
/// the builder pattern.
///
/// Example:
/// ```
/// use genanki_rs::Template;
///
/// let template1 = Template::new("Card 1").qfmt("{{Question}}").afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#);
/// let template2 = Template::new("Card 2").qfmt("{{Back}}").afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Front}}");
/// ```
///
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
    /// Creates a new `Template` with a `name`
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

    /// Sets the question format of the currently created `Template`
    pub fn qfmt(mut self, qfmt: &str) -> Self {
        self.qfmt = Some(qfmt.to_string());
        self
    }

    /// Sets the deck id of the currently created `Template`
    pub fn did(mut self, did: usize) -> Self {
        self.did = Some(did);
        self
    }

    /// Sets the browser answer format of the currently created `Template`
    pub fn bafmt(mut self, bafmt: &str) -> Self {
        self.bafmt = Some(bafmt.to_string());
        self
    }

    /// Sets the answer format of the currently created `Template`
    pub fn afmt(mut self, afmt: &str) -> Self {
        self.afmt = Some(afmt.to_string());
        self
    }

    /// Sets the browser question format of the currently created template
    pub fn bqfmt(mut self, bqfmt: &str) -> Self {
        self.bqfmt = Some(bqfmt.to_string());
        self
    }
}

impl From<Template> for Tmpl {
    fn from(value: Template) -> Self {
        Tmpl {
            name: value.name,
            qfmt: value.qfmt.unwrap_or("".to_string()),
            did: value.did,
            bafmt: value.bafmt.unwrap_or("".to_string()),
            afmt: value.afmt.unwrap_or("".to_string()),
            ord: 0,
            bqfmt: value.bqfmt.unwrap_or("".to_string()),
        }
    }
}
