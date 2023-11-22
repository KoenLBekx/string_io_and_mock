use std::ffi::OsString;
use std::io::ErrorKind;
use serial_test::file_serial;
use string_io_and_mock::{TextIOHandler, FileTextHandler};

mod utils;

#[test]
#[file_serial]
fn overwrite() {
    let playground_name = utils::ensure_playground(true);
    let mut file_name = playground_name.clone();
    file_name.push(&OsString::from("/TheWell.txt"));

    let txt1 = String::from("Well, about the well :");
    let txt2 = String::from("One can move the city, but not the well.");

    let mut fth = FileTextHandler::default();
    fth.write_text(&file_name, txt1.clone()).unwrap();
    fth.write_text(&file_name, txt2.clone()).unwrap();

    let read_back = fth.read_text(&file_name).unwrap();

    assert_eq!(txt2, read_back);
}

#[test]
#[file_serial]
fn read_and_write() {
    let playground_name = utils::ensure_playground(true);
    let mut file_name = playground_name.clone();
    file_name.push(&OsString::from("/Auchindoon.txt"));

    let txt = String::from("\
As I came down by Fiddichside on a May morning
I spied Willy MacIntosh an hour before the dawning.
Turn again, turn again, turn again I bid thee,
If ye'll burn Auchindoon, Huntley he will heed thee.

Heed me or hang me, that will never fear me.
I'll burn Auchindoon ere the life will leave me.

As I came down by Fiddichside on a May morning
Auchindoon was in a blaze an hour before the dawning.
Crawing, crawing, for all your crews are crawing,
you taint your crops and burnt your wings
an hour before the dawning.

As I came down by Fiddichside on a May morning
I spied Willy MacIntosh an hour before the dawning.
Hanging, hanging, ay the boy was hanging,
but the smoke of Auchindoon
through the air was rising.
");

    let mut fth = FileTextHandler::new();
    fth.write_text(&file_name, txt.clone()).unwrap();

    let read_back = fth.read_text(&file_name).unwrap();

    assert_eq!(txt, read_back);
}

#[test]
#[file_serial]
fn read_missing() {
    let playground_name = utils::ensure_playground(true);
    let mut file_name = playground_name.clone();
    file_name.push(&OsString::from("/missing.txt"));
    let fth = FileTextHandler::new();
    let read_result = fth.read_text(&file_name);

    match read_result {
        Ok(_) => panic!("Method read_text should return an Err if no text with the passed name is found."),
        Err(err) => {
            assert_eq!(ErrorKind::NotFound, err.kind());
        },
    }
}
