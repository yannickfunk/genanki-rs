# genanki-rs: A Rust Crate for Generating Anki Decks

With `genanki-rs` you can easily generate decks for the popular open source flashcard platform Anki.

*The code of this library is based on the code of [`genanki`](https://github.com/kerrickstaley/genanki), a python library to generate Anki decks.*

*This library and its author(s) are not affiliated/associated with the main Anki project in any way.*

![example workflow](https://github.com/yannickfunk/genanki-rs/actions/workflows/rust.yml/badge.svg)

## How to use
Add 
```toml
[dependencies]
genanki-rs = "0.1.0"
```
to your `Cargo.toml` or find another version on [*crates.io*](https://crates.io/crates/genanki-rs)

## Notes
The basic unit in Anki is the `Note`, which contains a fact to memorize. `Note`s correspond to one or more `Card`s.

Here's how you create a `Note`:

```rust
use genanki_rs::{Note, Error};

fn main() -> Result<(), Error> {
    // let my_model = ...
    let my_note = Note::new(my_model, vec!["Capital of Argentina", "Buenos Aires"])?;
    Ok(())
}
```

You pass in a `Model`, discussed below, and a set of `fields` (encoded as HTML).

## Models
A `Model` defines the fields and cards for a type of `Note`. For example:

```rust
use genanki_rs::{Field, Model, Template, Error};

fn main() -> Result<(), Error> {
    let my_model = Model::new(
        1607392319,
        "Simple Model",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    );
    // let my_note = ...
    Ok(())
}
```

This note-type has two fields and one card. The card displays the `Question` field on the front and the `Question` and
`Answer` fields on the back, separated by a `<hr>`. You can also pass custom `css` by calling `Model::new_with_options()` to supply custom
CSS.

```rust
let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n}\n";
let my_model_with_css = Model::new_with_options(
    1607392319,
    "Simple Model",
    vec![Field::new("Question"), Field::new("Answer")],
    vec![Template::new("Card 1")
        .qfmt("{{Question}}")
        .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    Some(custom_css),
    None,
    None,
    None,
    None,
);
```

You need to pass a model `id` and a model `name` so that Anki can keep track of your model. It's important that you use a unique model `id`
for each `Model` you define.

## Generating a Deck/Package
To import your notes into Anki, you need to add them to a `Deck`:

```rust
use genanki_rs::{Deck, Error};

fn main() -> Result<(), Error> {
    // let my_note = ...
    let mut my_deck = Deck::new(
        2059400110,
        "Country Capitals",
        "Deck for studying country capitals",
    );
    my_deck.add_note(my_note);
    Ok(())
}
```

Once again, you need a unique deck `id`, a deck `name` and a deck `description`.

Then, create a `Package` for your `Deck` and write it to a file:

```rust
my_deck.write_to_file("output.apkg")?;
```

You can then load `output.apkg` into Anki using File -> Import...

## Media Files
To add sounds or images, create a `Package` and pass the `decks` and `media_files` you want to include:

```rust
use genanki_rs::{Deck, Error, Package};

fn main() -> Result<(), Error> {
    // ...
    // my_deck.add(my_note)
    let mut my_package = Package::new(vec![my_deck], vec!["sound.mp3", "images/image.jpg"])?;
    my_package.write_to_file("output.apkg")?;
    Ok(())
}
```

`media_files` should have the path (relative or absolute) to each file. To use them in notes, first add a field to your model, and reference that field in your template:

```rust
let my_model = Model::new(
    1607392319,
    "Simple Model",
    vec![
        Field::new("Question"),
        Field::new("Answer"),
        Field::new("MyMedia"),                           // ADD THIS
    ],
    vec![Template::new("Card 1")
        .qfmt("{{Question}}{{Question}}<br>{{MyMedia}}") // AND THIS
        .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
);
```

Then, set the `MyMedia` field on your `Note` to `[sound:sound.mp3]` for audio and `<img src="image.jpg">` for images (e.g):

```rust
let my_note = Note::new(my_model, vec!["Capital of Argentina", "Buenos Aires", "[sound:sound.mp3]"])?;
// or
let my_note = Note::new(my_model, vec!["Capital of Argentina", "Buenos Aires", r#"<img src="image.jpg">"#])?;
```

You *cannot* put `<img src="{MyMedia}">` in the template and `image.jpg` in the field. See these sections in the Anki manual for more information: [Importing Media](https://docs.ankiweb.net/#/importing?id=importing-media) and [Media & LaTeX](https://docs.ankiweb.net/#/templates/fields?id=media-amp-latex).

You should only put the filename (aka basename) and not the full path in the field; `<img src="images/image.jpg">` will *not* work. Media files should have unique filenames.

## sort_field
Anki has a value for each `Note` called the `sort_field`. Anki uses this value to sort the cards in the Browse
interface. Anki also is happier if you avoid having two notes with the same `sort_field`, although this isn't strictly
necessary. By default, the `sort_field` is the first field, but you can change it by calling `Note::new_with_options()`.

You can also call `Model::new_with_options()`, passing the `sort_field_index` to change the sort field. `0` means the first field in the Note, `1` means the second, etc.

## FAQ
### My field data is getting garbled
If fields in your notes contain literal `<`, `>`, or `&` characters, you need to HTML-encode them: field data is HTML, not plain text.

For example, you should write
```rust
let fields = vec!["AT&amp;T was originally called", "Bell Telephone Company"]
```

This applies even if the content is LaTeX; for example, you should write
```rust
let fields = vec!["Piketty calls this the \"central contradiction of capitalism\".", "[latex]r &gt; g[/latex]"]
```
