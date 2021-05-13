use super::{Field, Model, Template};
use crate::model::ModelType;

/// Returns a basic Front/Back `Model`.
///
/// ```rust
/// use genanki_rs::basic_model;
/// let my_model = basic_model();
/// ```
///
/// is equivalent to
/// ```rust
/// use genanki_rs::{Model, Field, Template};
/// let my_model = Model::new_with_options(
///         1559383000,
///         "Basic (genanki)",
///         vec![
///             Field::new("Front").font("Arial"),
///             Field::new("Back").font("Arial"),
///         ],
///         vec![Template::new("Card 1")
///             .qfmt("{{Front}}")
///             .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}")],
///         Some(
///             ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
///         ),
///         None,
///         None,
///         None,
///         None,
///     );
/// ```
pub fn basic_model() -> Model {
    Model::new_with_options(
        1559383000,
        "Basic (genanki)",
        vec![
            Field::new("Front").font("Arial"),
            Field::new("Back").font("Arial"),
        ],
        vec![Template::new("Card 1")
            .qfmt("{{Front}}")
            .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}")],
        Some(
            ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
        ),
        None,
        None,
        None,
        None,
    )
}

/// Returns a basic Front/Back, Back/Front `Model`.
///
/// ```rust
/// use genanki_rs::basic_and_reversed_card_model;
/// let my_model = basic_and_reversed_card_model();
/// ```
///
/// is equivalent to
/// ```rust
/// use genanki_rs::{Model, Field, Template};
/// let my_model = Model::new_with_options(
///         1485830179,
///         "Basic (and reversed card) (genanki)",
///         vec![
///             Field::new("Front").font("Arial"),
///             Field::new("Back").font("Arial"),
///         ],
///         vec![
///             Template::new("Card 1")
///                 .qfmt("{{Front}}")
///                 .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"),
///             Template::new("Card 2")
///                 .qfmt("{{Back}}")
///                 .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Front}}"),
///         ],
///         Some(
///             ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
///         ),
///         None,
///         None,
///         None,
///         None,
///     );
/// ```
pub fn basic_and_reversed_card_model() -> Model {
    Model::new_with_options(
        1485830179,
        "Basic (and reversed card) (genanki)",
        vec![
            Field::new("Front").font("Arial"),
            Field::new("Back").font("Arial"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"),
            Template::new("Card 2")
                .qfmt("{{Back}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Front}}"),
        ],
        Some(
            ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
        ),
        None,
        None,
        None,
        None,
    )
}

/// Returns a basic Front/Back, Optional Back/Front `Model`.
///
/// ```rust
/// use genanki_rs::basic_optional_reversed_card_model;
/// let my_model = basic_optional_reversed_card_model();
/// ```
///
/// is equivalent to
/// ```rust
/// use genanki_rs::{Model, Field, Template};
/// let my_model = Model::new_with_options(
///         1382232460,
///         "Basic (optional reversed card) (genanki)",
///         vec![
///             Field::new("Front").font("Arial"),
///             Field::new("Back").font("Arial"),
///             Field::new("Add Reverse").font("Arial"),
///         ],
///         vec![
///             Template::new("Card 1")
///                 .qfmt("{{Front}}")
///                 .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"),
///             Template::new("Card 2")
///                 .qfmt("{{#Add Reverse}}{{Back}}{{/Add Reverse}}")
///                 .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Front}}"),
///         ],
///         Some(
///             ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
///         ),
///         None,
///         None,
///         None,
///         None,
///     );
/// ```
pub fn basic_optional_reversed_card_model() -> Model {
    Model::new_with_options(
        1382232460,
        "Basic (optional reversed card) (genanki)",
        vec![
            Field::new("Front").font("Arial"),
            Field::new("Back").font("Arial"),
            Field::new("Add Reverse").font("Arial"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"),
            Template::new("Card 2")
                .qfmt("{{#Add Reverse}}{{Back}}{{/Add Reverse}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Front}}"),
        ],
        Some(
            ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
        ),
        None,
        None,
        None,
        None,
    )
}

/// Returns a basic `Model` for cards where you can type in the answer.
///
/// ```rust
/// use genanki_rs::basic_type_in_the_answer_model;
/// let my_model = basic_type_in_the_answer_model();
/// ```
///
/// is equivalent to
/// ```rust
/// use genanki_rs::{Model, Field, Template};
/// let my_model = Model::new_with_options(
///         1305534440,
///         "Basic (type in the answer) (genanki)",
///         vec![
///             Field::new("Front").font("Arial"),
///             Field::new("Back").font("Arial"),
///         ],
///         vec![
///             Template::new("Card 1")
///                 .qfmt("{{Front}}\n\n{{type:Back}}")
///                 .afmt("{{Front}}\n\n<hr id=answer>\n\n{{type:Back}}"),
///         ],
///         Some(
///             ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
///         ),
///         None,
///         None,
///         None,
///         None,
///     );
/// ```
pub fn basic_type_in_the_answer_model() -> Model {
    Model::new_with_options(
        1305534440,
        "Basic (type in the answer) (genanki)",
        vec![
            Field::new("Front").font("Arial"),
            Field::new("Back").font("Arial"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}\n\n{{type:Back}}")
                .afmt("{{Front}}\n\n<hr id=answer>\n\n{{type:Back}}"),
        ],
        Some(
            ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
        ),
        None,
        None,
        None,
        None,
    )
}

/// Returns a basic `Model` for cards with clozes.
///
/// ```rust
/// use genanki_rs::cloze_model;
/// let my_model = cloze_model();
/// ```
///
/// is equivalent to
/// ```rust
/// use genanki_rs::{Model, Field, Template, ModelType};
/// let my_model = Model::new_with_options(
///         1122529321,
///         "Cloze (genanki)",
///         vec![
///             Field::new("Text").font("Arial"),
///         ],
///         vec![
///             Template::new("Cloze")
///                 .qfmt("{{cloze:Text}}")
///                 .afmt("{{cloze:Text}}"),
///         ],
///         Some(
///             ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n\n \
///              .cloze {\n font-weight: bold;\n color: blue;\n}\n.nightMode .cloze {\n color: lightblue;\n}",
///         ),
///         Some(ModelType::Cloze),
///         None,
///         None,
///         None,
///     );
/// ```
pub fn cloze_model() -> Model {
    Model::new_with_options(
        1122529321,
        "Cloze (genanki)",
        vec![
            Field::new("Text").font("Arial"),
        ],
        vec![
            Template::new("Cloze")
                .qfmt("{{cloze:Text}}")
                .afmt("{{cloze:Text}}"),
        ],
        Some(
            ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n\n \
             .cloze {\n font-weight: bold;\n color: blue;\n}\n.nightMode .cloze {\n color: lightblue;\n}",
        ),
        Some(ModelType::Cloze),
        None,
        None,
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::super::{Deck, Note};
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn builtin_models() {
        let mut my_deck = Deck::new(1598559905, "Country Capitals", "");

        my_deck.add_note(
            Note::new(basic_model(), vec!["Capital of Argentina", "Buenos Aires"]).unwrap(),
        );
        my_deck.add_note(
            Note::new(
                basic_and_reversed_card_model(),
                vec!["Costa Rica", "San José"],
            )
            .unwrap(),
        );
        my_deck.add_note(
            Note::new(
                basic_optional_reversed_card_model(),
                vec!["France", "Paris", "y"],
            )
            .unwrap(),
        );
        my_deck.add_note(
            Note::new(basic_type_in_the_answer_model(), vec!["Taiwan", "Taipei"]).unwrap(),
        );
        my_deck.add_note(
            Note::new(
                cloze_model(),
                vec!["{{c1::Rome}} is the capital of {{c2::Italy}}"],
            )
            .unwrap(),
        );

        let out_file = NamedTempFile::new().unwrap().into_temp_path();
        my_deck.write_to_file(out_file.to_str().unwrap()).unwrap();
    }
}
