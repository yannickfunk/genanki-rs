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
    use pyo3::types::PyDict;
    use pyo3::{
        types::{IntoPyDict, PyString},
        GILGuard, PyAny, PyErrArguments, PyObject, PyResult, Python, ToPyObject,
    };
    use tempfile::NamedTempFile;

    fn model() -> Model {
        Model::new(
            234567,
            "foomodel",
            vec![Field::new("AField"), Field::new("BField")],
            vec![Template::new("card1")
                .qfmt("{{AField}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{BField}}"#)],
        )
    }

    fn cn_model() -> Model {
        Model::new(
            345678,
            "Chinese",
            vec![
                Field::new("Traditional"),
                Field::new("Simplified"),
                Field::new("English"),
            ],
            vec![
                Template::new("Traditional")
                    .qfmt("{{Traditional}}")
                    .afmt(r#"{{FrontSide}}<hr id="answer">{{English}}"#),
                Template::new("Simplified")
                    .qfmt("{{Simplified}}")
                    .afmt(r#"{{FrontSide}}<hr id="answer">{{English}}"#),
            ],
        )
    }

    fn model_with_hint() -> Model {
        Model::new(
            456789,
            "with hint",
            vec![
                Field::new("Question"),
                Field::new("Hint"),
                Field::new("Answer"),
            ],
            vec![Template::new("card1")
                .qfmt("{{Question}}{{#Hint}}<br>Hint: {{Hint}}{{/Hint}}")
                .afmt("{{Answer}}")],
        )
    }

    const CUSTOM_LATEX_PRE: &str = r#"\documentclass[12pt]{article}
    \special{papersize=3in,5in}
    \usepackage[utf8]{inputenc}
    \usepackage{amssymb,amsmath,amsfonts}
    \pagestyle{empty}
    \setlength{\parindent}{0in}
    \begin{document}
    "#;

    const CUSTOM_LATEX_POST: &str = "% here is a great comment\n\\end{document}";

    fn model_with_latex() -> Model {
        Model::new_with_options(
            567890,
            "with latex",
            vec![Field::new("AField"), Field::new("Bfield")],
            vec![Template::new("card1")
                .qfmt("{{AField}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{BField}}"#)],
            None,
            None,
            Some(CUSTOM_LATEX_PRE),
            Some(CUSTOM_LATEX_POST),
            None,
        )
    }

    const CUSTOM_SORT_FIELD_INDEX: i64 = 1;

    fn model_with_sort_field_index() -> Model {
        Model::new_with_options(
            567890,
            "with latex",
            vec![Field::new("AField"), Field::new("Bfield")],
            vec![Template::new("card1")
                .qfmt("{{AField}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{BField}}"#)],
            None,
            None,
            None,
            None,
            Some(CUSTOM_SORT_FIELD_INDEX),
        )
    }

    const VALID_MP3: &[u8] =
        b"\xff\xe3\x18\xc4\x00\x00\x00\x03H\x00\x00\x00\x00LAME3.98.2\x00\x00\x00\
        \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
        \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
        \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    const VALID_JPG: &[u8] =
        b"\xff\xd8\xff\xdb\x00C\x00\x03\x02\x02\x02\x02\x02\x03\x02\x02\x02\x03\x03\
        \x03\x03\x04\x06\x04\x04\x04\x04\x04\x08\x06\x06\x05\x06\t\x08\n\n\t\x08\t\
        \t\n\x0c\x0f\x0c\n\x0b\x0e\x0b\t\t\r\x11\r\x0e\x0f\x10\x10\x11\x10\n\x0c\
        \x12\x13\x12\x10\x13\x0f\x10\x10\x10\xff\xc9\x00\x0b\x08\x00\x01\x00\x01\
        \x01\x01\x11\x00\xff\xcc\x00\x06\x00\x10\x10\x05\xff\xda\x00\x08\x01\x01\
        \x00\x00?\x00\xd2\xcf \xff\xd9";

    fn get_anki_collection(py: Python<'_>) -> &PyAny {
        let locals = PyDict::new(py);
        locals
            .set_item("tmp_path", PyString::new(py, "test.anki2"))
            .unwrap();
        locals.set_item("anki", py.import("anki").unwrap()).unwrap();
        let code = "anki.Collection(tmp_path)";
        let col = py.eval(code, None, Some(&locals)).unwrap().to_owned();
        col
    }

    fn import_package(mut package: Package, py: Python<'_>) -> &PyAny {
        let out_file = NamedTempFile::new().unwrap().into_temp_path();
        package.write_to_file(out_file.to_str().unwrap()).unwrap();
        let locals = PyDict::new(py);
        let anki_col = get_anki_collection(py);
        locals.set_item("col", anki_col).unwrap();
        locals
            .set_item("outfile", PyString::new(py, out_file.to_str().unwrap()))
            .unwrap();
        locals.set_item("anki", py.import("anki").unwrap()).unwrap();
        locals
            .set_item(
                "anki.importing.apkg",
                py.import("anki.importing.apkg").unwrap(),
            )
            .unwrap();
        let code = r#"
importer = anki.importing.apkg.AnkiPackageImporter(col, outfile)
importer.run()
res = str(col)
        "#;
        py.run(code, None, Some(locals)).unwrap();
        locals.get_item("res").unwrap()
    }

    #[test]
    fn import_anki() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        py.import("anki").unwrap();
    }

    #[test]
    fn generated_deck_can_be_imported() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut deck = Deck::new(123456, "foodeck", "");
        deck.add_note(Note::new(model(), vec!["a", "b"]).unwrap());
        let _ = import_package(Package::new(vec![deck], vec![]).unwrap(), py);
    }

    #[test]
    fn generated_deck_has_valid_cards() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut deck = Deck::new(123456, "foodeck", "");
        deck.add_note(Note::new(cn_model(), vec!["a", "b", "c"]).unwrap());
        deck.add_note(Note::new(cn_model(), vec!["d", "e", "f"]).unwrap());
        deck.add_note(Note::new(cn_model(), vec!["g", "h", "i"]).unwrap());
        let anki_col = import_package(Package::new(vec![deck], vec![]).unwrap(), py);
        let len: String = py
            .eval(
                "print(col)",
                None,
                Some([("col", anki_col)].into_py_dict(py)),
            )
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(len, "hallo");
    }
}
