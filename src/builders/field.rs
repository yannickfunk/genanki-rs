use crate::db_entries::Fld;

/// Field to be fed into a `Model`.
///
/// A `Field` can be created using the builder pattern.
///
/// Example:
///
/// ```rust
/// use genanki_rs::Field;
///
/// let field1 = Field::new("field1");
/// let field2 = Field::new("field2").font("Comic Sans").size(15);
/// ```
///
/// The builder has the following default values:
/// * `sticky` - `false`
/// * `rtl` - `false`
/// * `font` - `Liberation Sans`
/// * `size` - `20`
#[derive(Clone)]
pub struct Field {
    name: String,
    sticky: Option<bool>,
    rtl: Option<bool>,
    font: Option<String>,
    size: Option<i64>,
}

impl Field {
    /// Creates a new field with a `name`
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sticky: None,
            rtl: None,
            font: None,
            size: None,
        }
    }

    /// Sets the font of the `Field` which is currently created
    pub fn font(mut self, value: &str) -> Self {
        self.font = Some(value.to_string());
        self
    }

    /// Sets whether the currently created `Field` is right-to-left or not
    pub fn rtl(mut self, value: bool) -> Self {
        self.rtl = Some(value);
        self
    }

    /// Sets whether the currently created `Field` is sticky or not
    pub fn sticky(mut self, value: bool) -> Self {
        self.sticky = Some(value);
        self
    }

    /// Sets the font size of the currently created `Field`
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
