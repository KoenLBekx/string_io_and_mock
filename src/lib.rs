use std::ffi::{OsString, OsStr};
use std::io::{Error as IOError, ErrorKind, Result as IOResult};
use std::path::{Path, Component};
use std::fs::metadata;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PathError {
    NonexistentParent,
    NonUtf8,
    WildcardInParent,
    IOError(ErrorKind),
    RegexError(String),
}

pub trait TextIOHandler {
    fn list_names(&self, global_name: &OsStr) -> Result<Vec<OsString>, PathError>;
    fn read_text(&self, name: &OsStr) -> Result<String, String>;
    fn write_text(&self, name: &OsStr, content: String) -> Result<(), String>;
}

fn contains_wildcards(name: &OsStr) -> Result<bool, PathError> {

    match name.to_str() {
        None => Err(PathError::NonUtf8),
        Some(name_str) => {
            lazy_static! {
                // unwrap() won't panic at runtime on this regex pattern -
                // this is proven by unit tests.
                static ref rgx:Regex = Regex::new(r"[?\*]").unwrap();
            }

            Ok(rgx.is_match(name_str))
        },
    }
}

pub struct FileTextHandler {}

impl FileTextHandler {
    pub fn new() -> Self {
        FileTextHandler {}
    }
}

impl TextIOHandler for FileTextHandler {

    fn list_names(&self, global_name: &OsStr) -> Result<Vec<OsString>, PathError> {
        let mut outcome = Vec::<OsString>::new();

        // Convert \ to / lest the entire path isn't considered one component.
        let path_str = match global_name.to_str() {
            None => return Err(PathError::NonUtf8),
            Some(pstr) => pstr.replace(r"\", "/"),
        };

        let mut path = Path::new(&path_str);

        if !contains_wildcards(&global_name)? {
            // Return path if it exists; if not return empty vector.
            if path.is_file() {
                outcome.push(OsString::from(global_name));
            }

            return Ok(outcome);
        }

        // Apparently, path.parent() yields an empty path
        // if any component in the path doesn't exist or has wildcards.
        
        // If any but the last part of the path has wildcards, it's invalid.
        // Return an Err.
        let mut is_last = true;
        let mut last_part:String = String::from("");

        for c in path.components().rev() {
            
            // #[cfg(test)]
            println!("component: {:?}", c);

            match c {
                Component::RootDir | Component::CurDir | Component::ParentDir => (),
                Component::Normal(ref part_str) =>  {

                    if (!is_last) && contains_wildcards(part_str)? {
                        return Err(PathError::WildcardInParent);
                    }

                    if is_last {
                        match part_str.to_str() {
                            None => {
                                return Err(PathError::NonUtf8);
                            },
                            Some(lpstr) => {
                                last_part = lpstr.to_string();
                            },
                        };
                    }
                },
                Component::Prefix(ref prefix_cmp) => {
                    let pcstr = prefix_cmp.as_os_str();

                    if (!is_last) && contains_wildcards(pcstr)? {
                        return Err(PathError::WildcardInParent);
                    }

                    if is_last {
                        match pcstr.to_str() {
                            None => {
                                return Err(PathError::NonUtf8);
                            },
                            Some(lpstr) => {
                                last_part = lpstr.to_string();
                            },
                        }
                    }
                },
            }

            is_last = false;
        }

        // We know the wildcards are in the last part only.

        // Isolate the parent.
        let mut components = Path::new(&path_str).components();
        components.next_back();
        path = components.as_path();

        // Check if the directory exists.
        if !path.is_dir() {
            return Err(PathError::NonexistentParent);
        }

        // Get all file entries in the directory.
        match path.read_dir() {
            Err(io_error) => return Err(PathError::IOError(io_error.kind())), 
            Ok(read_dir) => {
                let rgx_str = format!("^{}$", last_part.replace(".", r"\.").replace("*", ".+").replace("?", "."));

                // #[cfg(test)]
                println!("rgx_str : {}", rgx_str);

                let rgx = match Regex::new(&rgx_str) {
                    Err(rgxerr) => return Err(PathError::RegexError(format!("{}", rgxerr))),
                    Ok(r) => r,
                };

                // Return all files in the directory that match the global path.
                outcome = read_dir.filter_map(|entry_result| {

                    match entry_result {
                        Err(_) => None,
                        Ok(entry) => {

                            match entry.file_type() {
                                Err(_) => None,
                                Ok(ft) => {

                                    let candidate = if ft.is_file() {
                                        Some(entry.file_name())
                                    } else if ft.is_symlink() {

                                        match metadata(entry.path()) {
                                            Err(_) => None,
                                            Ok(metadt) => {

                                                if metadt.is_file() {
                                                    Some(entry.file_name())
                                                } else {
                                                    None
                                                }
                                            },
                                        }
                                    } else {
                                        None
                                    };

                                    // Filter on file name against global name having wildcards.
                                    match candidate {
                                        None => None,
                                        Some(fname) => {
                                            match fname.to_str() {
                                                None => None,
                                                Some(fnm) => {
                                                    if rgx.is_match(fnm) {
                                                        Some(entry.path().into_os_string())
                                                    } else {
                                                        None
                                                    }
                                                },
                                            }
                                        },
                                    }
                                },
                            }
                        },
                    }
                }).collect();
            },
        }

        Ok(outcome)
    }

    fn read_text(&self, name: &OsStr) -> Result<String, String> {
        // TODO : implement
        Ok(String::from("dummy"))
    }

    fn write_text(&self, name: &OsStr, content: String) -> Result<(), String> {
        // TODO : implement
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_has_wildcards_asterisk() {
        assert!(contains_wildcards(&OsString::from("book*.mpc")).unwrap());
    }

    #[test]
    fn regex_has_wildcards_question_mark() {
        assert!(contains_wildcards(&OsString::from("book?.mpc")).unwrap());
    }

    #[test]
    fn regex_has_wildcards_both() {
        assert!(contains_wildcards(&OsString::from("book?_*.mpc")).unwrap());
    }

    #[test]
    fn regex_has_wildcards_none() {
        assert!(!contains_wildcards(&OsString::from("book01.mpc")).unwrap());
    }

    #[test]
    fn file_th_list_names_wildcards_in_dir() {
        let fth = FileTextHandler::new();
        let outcome = fth.list_names(&OsString::from("/sites/myPics*/test.mpc"));

        assert_eq!(Err(PathError::WildcardInParent), outcome);
    }

    #[test]
    fn file_th_list_names_wildcards_in_filename() {
        let fth = FileTextHandler::new();
        let outcome = fth.list_names(&OsString::from("/sites/myPics/nonexistent/test*.mpc"));

        assert_ne!(Err(PathError::WildcardInParent), outcome);
    }

    #[test]
    fn file_th_list_names_wildcards_in_windows_filename() {
        let fth = FileTextHandler::new();
        let outcome = fth.list_names(&OsString::from(r"c:\sites\myPics\test*.mpc"));

        assert_ne!(Err(PathError::WildcardInParent), outcome);
    }

    #[test]
    fn file_th_list_names_wildcards_in_prefix() {
        let fth = FileTextHandler::new();
        let outcome = fth.list_names(&OsString::from(r"?:/sites/myPics/test.mpc"));

        assert_eq!(Err(PathError::WildcardInParent), outcome);
    }
}
