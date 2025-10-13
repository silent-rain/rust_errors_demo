#![allow(unused)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple() {
        let err = Error::Redaction("123".to_string());
        println!("{err}");
        println!("{err:?}");
        println!("{err:#?}");
    }
}
