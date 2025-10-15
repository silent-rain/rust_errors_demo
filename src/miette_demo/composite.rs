#![allow(unused)]

use std::fs::read_to_string;

use miette::{Diagnostic, SourceSpan};

#[derive(thiserror::Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(my_error::io))]
    Io(#[from] std::io::Error),

    #[error("Oops it blew up")]
    #[diagnostic(code(my_lib::bad_code))]
    BadThingHappened,

    #[error(transparent)]
    // Use `#[diagnostic(transparent)]` to wrap another [`Diagnostic`]. You won't see labels otherwise
    #[diagnostic(transparent)]
    AnotherError(#[from] AnotherError),

    /// Forward the diagnostic to a particular field.
    #[error("other error")]
    #[diagnostic(forward(the_actual_diagnostic))]
    EvenMoreData {
        unrelated_field_1: String,
        unrelated_field_2: usize,

        #[source]
        the_actual_diagnostic: AnotherError,
    },

    #[error("unknown data store error")]
    Unknown,
}

impl Default for Error {
    fn default() -> Self {
        Self::BadThingHappened
    }
}

#[derive(thiserror::Error, Diagnostic, Debug)]
#[error("another error")]
pub struct AnotherError {
    #[label("here")]
    pub at: SourceSpan,
}

fn read_file() -> Result<(), Error> {
    let content = read_to_string("path/to/file").map_err(Error::Io)?;

    println!("{}", content);
    Ok(())
}

fn msg1() -> Result<(), Error> {
    read_file()?;
    println!("this is a msg1");
    Ok(())
}

fn msg2() -> Result<(), Error> {
    msg1()?;
    println!("this is a msg2");
    Ok(())
}

fn msg3() -> Result<(), Error> {
    msg2()?;
    println!("this is a msg3");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple() {
        if let Err(e) = msg3() {
            println!("{e}");
            println!("========================================");
            println!("{e:?}");
            println!("========================================");
            println!("{e:#?}");
        }
    }
}
