# string_io_and_mock

*by Koen Bekx*

The `string_io_and_mock` crate provides a struct `FileTextHandler` that acts as a mockable layer over a file
system. It provides write and read operations that are required by the `TextIOHandler`
trait :
- method `write_text` writes String content to a file or file system simulator;
- method `read_text` reads String content from a file or file system simulator;

The *Text* in the names of the trait and structs mean that these entities are only meant to handle **`String`** content, as is evident from the signatures of the trait's methods.

For unit tests - or for other applications - a mock `MockTextHandler` is available that also
implements the `TextIOHandler` trait, but doesn't access any file system. It stores it texts in
a `HashMap` instead.

This means that `MockTextHandler` is more than a mere mock: with its internal persistence, 
it can serve as an application component in its own right,
providing string storage in memory where file storage isn't needed.

## Usage

In order to use this crate, add it to your project's Cargo.toml file using the command

```
cargo add string_io_and_mock
```

and add the below statement in your code :

```
use string_io_and_mock::{FileTextHandler, MockTextHandler, TextIOHandler};
```

## Examples

For examples of how to use these components in code, see the crate's code documentation or it's unit and integration tests.
