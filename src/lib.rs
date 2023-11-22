//! This crate provides a struct [`FileTextHandler`] that acts as a mockable layer over a file
//! system. It provides write and read operations that are required by the [`TextIOHandler`]
//! trait :
//! - method `write_text` writes String content to a file or file system simulator;
//! - method `read_text` reads String content from a file or file system simulator;
//!
//! The *Text* in the names of the trait and structs mean that these entities are only meant to handle **`String`** content, as is evident from the signatures of the trait's methods.
//!
//! For unit tests - or for other applications - a mock [`MockTextHandler`] is available that also
//! implements the [`TextIOHandler`] trait, but doesn't access any file system. It stores it texts in
//! a [`HashMap`] instead.
//!
//! This means that `MockTextHandler` is more than a mere mock: with its internal persistence, 
//! it can serve as an application component in its own right,
//! providing string storage in memory where file storage isn't needed.

use std::collections::HashMap;
use std::ffi::{OsString, OsStr};
use std::io::{Error as IoError, ErrorKind, Result as IoResult};
use std::fs::{read_to_string, write};

/// Implementors provide the ability to accept [`std::string::String`] content associated with an [`std::ffi::OsStr`] name, as can be expected from entities mediating a file system or their mocks and simulators.
pub trait TextIOHandler {
    fn read_text(&self, name: &OsStr) -> IoResult<String>;
    fn write_text(&mut self, name: &OsStr, content: String) -> IoResult<()>;
}


/// FileTextHandler provides string read and write operations to file system files.
/// It has no internal persistence, as this is provided by the underlying file system.
/// Even so, calling it's write_text method still requires a FileTextHandler object
/// to be declared as mutable, as the TextIOHandler trait imposes this
/// so as to enable mocks to use internal persistence to write to their encapsulated state.
/// # Examples
/// ```
/// use std::ffi::OsStr;
/// use string_io_and_mock::{FileTextHandler, TextIOHandler};
///
/// fn main()
/// {
///     let content = String::from(
///         "Programming is to a large extent the art of correct definitions.");
///
///     let file_name = OsStr::new("tests/playground/myText.txt");
///     let mut fth = FileTextHandler::new();
///
///     fth.write_text(&file_name, content.clone()).unwrap();
///
///     // Not using the same FileTextHandler for reading back :
///     // the persistency is provided by the file system.
///     let other_fth = FileTextHandler::new();
///     let read_back = other_fth.read_text(&file_name).unwrap();
///
///     assert_eq!(content, read_back);
/// }
/// ```
pub struct FileTextHandler {}
impl FileTextHandler {
    pub fn new() -> Self {
        FileTextHandler {}
    }
}
impl TextIOHandler for FileTextHandler {

    fn read_text(&self, name: &OsStr) -> IoResult<String> {
        read_to_string(name)
    }

    fn write_text(&mut self, name: &OsStr, content: String) -> IoResult<()> {
        match write(name, content) {
            Ok(_) => Ok(()),
            Err(io_err) => Err(io_err),
        }
    }
}

/// MockTextHandler allows FileTextHandler objects to be replaced by a mock in unit tests.
/// MockTextHandler stores strings written to it in a private [`HashMap`].
/// # Examples
/// ```
/// use std::io::ErrorKind;
/// use std::ffi::OsStr;
/// use string_io_and_mock::{MockTextHandler, TextIOHandler};
///
/// fn main()
/// {
///     let content = String::from(
///         "Programming is to a huge extent the art of correct definitions.");
///
///     let file_name = OsStr::new("tests/playground/myText.txt");
///     let mut mock = MockTextHandler::new();
///
///     mock.write_text(&file_name, content.clone()).unwrap();
///
///     let read_back = mock.read_text(&file_name).unwrap();
///     assert_eq!(content, read_back);
///
///     // Not using the same MockTextHandler for reading back will yield a "not found" error.
///     // as MockTextHandler instances don't share their internal state.
///     let other_mock = MockTextHandler::new();
///     let read_result = other_mock.read_text(&file_name);
///
///     match read_result  {
///         Ok(_) => panic!("MockTextHandler.read_file of a missing string should return a std::io::Error."),
///         Err(err) => assert_eq!(err.kind(), ErrorKind::NotFound),
///     }
/// }
/// ```
pub struct MockTextHandler {
    texts: HashMap<OsString, String>,
}
impl MockTextHandler {
    pub fn new() -> Self {
        MockTextHandler {
            texts: HashMap::new(),
        }
    }
}
impl TextIOHandler for MockTextHandler {

    fn read_text(&self, name: &OsStr) -> IoResult<String> {
        match self.texts.get(&name.to_os_string()) {
            None => Err(IoError::from(ErrorKind::NotFound)),
            Some(content) => Ok(content.clone()),
        }
    }

    fn write_text(&mut self, name: &OsStr, content: String) -> IoResult<()> {
        self.texts.insert(name.to_os_string(), content);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_read_write() {
        let txt = String::from("\
As I came down by Fiddichside on a May morning
I spied Willy MacIntosh an hour before the dawning.
Turn again, turn again, turn again I bid thee,
If ye'll burn Auchindoon, Huntley he will heed thee.

- Heed me or hang me, that will never fear me.
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

        let key = OsStr::new("Auchindoon");
        let mut mock = MockTextHandler::new();
        mock.write_text(&key, txt.clone()).unwrap();
        let read_back = mock.read_text(&key).unwrap();

        assert_eq!(txt, read_back);
    }

    #[test]
    fn mock_overwrite() {
        let txt1 = String::from("Well, about the well :");
        let txt2 = String::from("One can move the city, but not the well.");

        let key = OsStr::new("The Well");
        let mut mock = MockTextHandler::new();
        mock.write_text(&key, txt1.clone()).unwrap();
        mock.write_text(&key, txt2.clone()).unwrap();
        let read_back = mock.read_text(&key).unwrap();

        assert_eq!(txt2, read_back);
    }

    #[test]
    fn mock_read_missing() {
        let mock = MockTextHandler::new();
        let result = mock.read_text(&OsStr::new("Whatever"));

        match result {
            Ok(_) => panic!("Method read_text should return an Err if no text with the passed name is found."),
            Err(err) => {
                assert_eq!(ErrorKind::NotFound, err.kind());
            },
        }
    }
}
