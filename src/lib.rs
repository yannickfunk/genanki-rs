mod apkg_col;
mod apkg_schema;
mod builders;
mod builtin_models;
mod card;
mod db_entries;
mod deck;
mod model;
mod note;
mod package;
mod util;

pub use builders::{Field, Template};
pub use builtin_models::*;
pub use deck::Deck;
pub use model::Model;
pub use note::Note;
pub use package::Package;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
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
        my_deck.write_to_file("output.apkg").unwrap();
    }
}
