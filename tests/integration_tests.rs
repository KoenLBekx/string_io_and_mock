use std::ffi::OsString;
use serial_test::file_serial;
use string_io_and_mock::{TextIOHandler, FileTextHandler, PathError};

mod utils;

#[test]
#[file_serial]
fn list_names_nonexistent_file() {
    let fth = FileTextHandler::new();
    let outcome = fth.list_names(&OsString::from(r"~/nonsense/imnothere.fil")).unwrap();

    assert_eq!(0, outcome.len());
}

#[test]
#[file_serial]
fn list_names_existent_file() {
    let playground_name = utils::ensure_playground(false);

    let mut dummy_file = playground_name.clone();
    dummy_file.push(&OsString::from("/dummy1.fil"));
    utils::ensure_file(&dummy_file);

    let fth = FileTextHandler::new();
    let outcome = fth.list_names(&dummy_file).unwrap();

    assert_eq!(1, outcome.len());
    assert_eq!(&dummy_file, &outcome[0]);
}

#[test]
#[file_serial]
fn list_names_nonexistent_parent() {
    let playground_name = utils::ensure_playground(true);

    let mut dummy_file = playground_name.clone();
    dummy_file.push(&OsString::from("/missing_dir/dummy*.fil"));

    let fth = FileTextHandler::new();
    let res = fth.list_names(&dummy_file);

    assert_eq!(Err(PathError::NonexistentParent), res);
}

#[test]
#[file_serial]
fn list_names_existent_parent() {
    let playground_name = utils::ensure_playground(false);

    let mut dummy_file = playground_name.clone();
    dummy_file.push(&OsString::from("/any*.fil"));

    let fth = FileTextHandler::new();
    let res = fth.list_names(&dummy_file);

    assert_ne!(Err(PathError::NonexistentParent), res);
}

#[test]
#[file_serial]
fn list_names_question_mark_wildcard() {
    let playground_name = utils::ensure_playground(true);
    
    let mut dummy_file1 = playground_name.clone();
    dummy_file1.push(&OsString::from("/dummy1.fil"));
    utils::ensure_file(&dummy_file1);

    let mut dummy_file2 = playground_name.clone();
    dummy_file2.push(&OsString::from("/dummy2.fil"));
    utils::ensure_file(&dummy_file2);

    let fth = FileTextHandler::new();
    let mut global = playground_name.clone();
    global.push(&OsString::from("/dummy?.fil"));

    // Debug
    println!("global : {:?}", global);

    let outcome = fth.list_names(&global).unwrap();

    assert_eq!(2, outcome.len());

    let mut outcome_iter = outcome.iter();
    assert!(outcome_iter.any(|f| f == &dummy_file1));
    assert!(outcome_iter.any(|f| f == &dummy_file2));
}

#[test]
#[file_serial]
fn list_names_asterisk_wildcard() {
    let playground_name = utils::ensure_playground(true);
    
    let mut dummy_file1 = playground_name.clone();
    dummy_file1.push(&OsString::from("/dummy1.fil"));
    utils::ensure_file(&dummy_file1);

    let mut dummy_file2 = playground_name.clone();
    dummy_file2.push(&OsString::from("/dummy2.fil"));
    utils::ensure_file(&dummy_file2);

    let fth = FileTextHandler::new();
    let mut global = playground_name.clone();
    global.push(&OsString::from("/dum*.fil"));

    let outcome = fth.list_names(&global).unwrap();

    assert_eq!(2, outcome.len());

    let mut outcome_iter = outcome.iter();
    assert!(outcome_iter.any(|f| f == &dummy_file1));
    assert!(outcome_iter.any(|f| f == &dummy_file2));
}

#[test]
#[file_serial]
fn list_names_directories_are_ignored() {
    let playground_name = utils::ensure_playground(true);
    
    let mut dummy_file1 = playground_name.clone();
    dummy_file1.push(&OsString::from("/dummy1.fil"));
    utils::ensure_file(&dummy_file1);

    let mut dir_path = playground_name.clone();
    dir_path.push(&OsString::from("/someSubDir"));
    utils::ensure_dir(&dir_path);

    let fth = FileTextHandler::new();
    let mut global = playground_name.clone();
    global.push(&OsString::from("/*"));

    // Debug
    println!("global : {:?}", global);

    let outcome = fth.list_names(&global).unwrap();

    assert_eq!(1, outcome.len());

    let mut outcome_iter = outcome.iter();
    assert!(outcome_iter.any(|f| f == &dummy_file1));
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
