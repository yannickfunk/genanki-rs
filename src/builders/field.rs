use crate::db_entries::Fld;

#[derive(Clone)]
pub struct Field {
    name: String,
    sticky: Option<bool>,
    rtl: Option<bool>,
    font: Option<String>,
    size: Option<i64>,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sticky: None,
            rtl: None,
            font: None,
            size: None,
        }
    }

    pub fn font(mut self, value: &str) -> Self {
        self.font = Some(value.to_string());
        self
    }

    pub fn rtl(mut self, value: bool) -> Self {
        self.rtl = Some(value);
        self
    }

    pub fn sticky(mut self, value: bool) -> Self {
        self.sticky = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }
}

impl Into<Fld> for Field {
    fn into(self) -> Fld {
        Fld {
            name: self.name.to_string(),
            media: vec![],
            sticky: self.sticky.unwrap_or(false),
            rtl: self.rtl.unwrap_or(false),
            ord: 0,
            font: self.font.unwrap_or("Liberation Sans".to_string()),
            size: self.size.unwrap_or(20),
        }
    }
}
