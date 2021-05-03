mod apkg_col;
mod apkg_schema;
mod card;
mod db_entries;
mod deck;
mod model;
mod note;
mod package;
mod util;

pub use deck::Deck;
pub use model::Model;
pub use note::Note;
pub use package::Package;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_entries::{Fld, Tmpl};
    use std::fs::File;

    #[test]
    fn it_works() {
        let templates = vec![Tmpl {
            name: "Card 1".to_string(),
            qfmt: "{{Question}}".to_string(),
            did: None,
            bafmt: "".to_string(),
            afmt: r#"{{FrontSide}}<hr id="answer">{{Answer}}"#.to_string(),
            ord: 0,
            bqfmt: "".to_string(),
        }];
        let fields = vec![
            Fld {
                name: "Question".to_string(),
                media: vec![],
                sticky: false,
                rtl: false,
                ord: 0,
                font: "Liberation Sans".to_string(),
                size: 20,
            },
            Fld {
                name: "Answer".to_string(),
                media: vec![],
                sticky: false,
                rtl: false,
                ord: 1,
                font: "Liberation Sans".to_string(),
                size: 20,
            },
        ];
        let my_model =
            Model::new_with_defaults(1607392319, "Simple Model".to_string(), fields, templates);
        let my_note = Note::new(
            my_model,
            vec![
                "Capital of Argentina".to_string(),
                "Buenos Aires".to_string(),
            ],
            false,
            vec![],
            None,
        )
        .unwrap();

        let mut my_deck = Deck::new(
            2059400110,
            "Country Capitals".to_string(),
            "deck for capitals".to_string(),
        );
        my_deck.add_note(my_note);
        Package::new(vec![my_deck], vec![])
            .write_to_file("output.apkg", None)
            .unwrap();
    }
}
