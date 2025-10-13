#![allow(unused)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn it_simple() {
        if let Err(e) = read_to_string("path/to/file").map_err(Error::Io) {
            println!("{e}");
            println!("{e:?}");
            println!("{e:#?}");
        }
    }
}
