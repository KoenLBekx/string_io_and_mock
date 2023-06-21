use std::ffi::OsString;
use serial_test::file_serial;
use file_io_and_mock::{TextIOHandler, FileTextHandler, PathError};

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
