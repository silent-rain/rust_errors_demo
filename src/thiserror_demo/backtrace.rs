#![allow(unused)]

use std::backtrace::Backtrace;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // 暂不支持backtrace
    // #[error("data store disconnected")]
    // Io {
    //     #[from]
    //     source: std::io::Error,
    //     backtrace: Backtrace,
    // },

    // 暂不支持backtrace
    // #[error("data store disconnected")]
    // Io {
    //     #[backtrace]
    //     source: std::io::Error,
    // },

    // 暂不支持backtrace
    // #[error("...")]
    // Test { backtrace: Backtrace },

    // 暂不支持backtrace
    // #[error("this is a test error")]
    // Test {
    //     #[backtrace]
    //     backtrace: Backtrace,
    // },
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
