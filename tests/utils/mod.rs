use std::ffi::OsString;
use std::fs::{create_dir_all, remove_dir_all, remove_file};
use std::path::Path;
use std::str::FromStr;

pub const TESTFILES_DIR: &str = "./tests/playground";

pub fn ensure_playground(remove_first: bool) -> OsString {
    let dir_path = Path::new(&TESTFILES_DIR);

    if dir_path.exists() {
        let is_dir = dir_path.is_dir();

        if remove_first && is_dir {
           remove_dir_all(&dir_path).unwrap(); 
        }

        if !is_dir {
           remove_file(&dir_path).unwrap(); 
        }
    }

    if !dir_path.exists() {
        create_dir_all(&dir_path).unwrap();
    }

    OsString::from_str(TESTFILES_DIR).unwrap()
}
