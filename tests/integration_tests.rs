use std::ffi::OsString;
use file_io_and_mock::{TextIOHandler, FileTextHandler};

mod utils;

#[test]
fn list_names_nonexistent_file() {
    let fth = FileTextHandler::new();
    let outcome = fth.list_names(&OsString::from(r"~/nonsense/imnothere.fil")).unwrap();

    assert_eq!(0, outcome.len());
}

#[test]
fn list_names_existent_file() {
    let playground_name = utils::ensure_playground();

    let mut dummy_file = playground_name.clone();
    dummy_file.push(&OsString::from("/dummy1.fil"));
    utils::ensure_file(&dummy_file);

    let fth = FileTextHandler::new();
    let outcome = fth.list_names(&dummy_file).unwrap();

    assert_eq!(1, outcome.len());
    assert_eq!(&dummy_file, &outcome[0]);
}
