mod apkg_col;
mod apkg_schema;
mod builders;
mod card;
mod db_entries;
mod deck;
mod model;
mod note;
mod package;
mod util;
pub use builders::{Field, Template};
pub use deck::Deck;
pub use model::Model;
pub use note::Note;
pub use package::Package;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let my_model = Model::new(
            1607392319,
            "Simple Model",
            vec![
                Field::new("Question"),
                Field::new("Answer"),
                Field::new("MyMedia"),
            ],
            vec![Template::new("t1")
                .qfmt("{{Question}}<br>{{MyMedia}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
        );

        let my_note = Note::new(
            my_model,
            vec![
                "Capital of Argentina",
                "Buenos Aires",
                r#"<img src="buenas.png">"#,
            ],
        )
        .unwrap();

        let mut my_deck = Deck::new(
            2059400110,
            "Country Capitals".to_string(),
            "deck for capitals".to_string(),
        );
        my_deck.add_note(my_note);
        Package::new(vec![my_deck], vec!["buenas.png"])
            .unwrap()
            .write_to_file("output.apkg")
            .unwrap();
    }
}
