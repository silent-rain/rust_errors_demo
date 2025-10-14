#![allow(unused)]

use snafu::{ResultExt, Snafu};
use std::fs::read_to_string;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("IO error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Unknown error"))]
    Unknown,
}

fn read_file() -> Result<(), Error> {
    let content = read_to_string("path/to/file").context(IoSnafu)?;
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
