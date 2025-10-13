#![allow(unused)]

use anyhow::Context;
use anyhow::anyhow;
use anyhow::bail;
use std::fs::read_to_string;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("unknown data store error")]
    Unknown,

    // #[error("an error occurred")] // 会出现噪点
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

fn read_file() -> Result<(), Error> {
    let content = read_to_string("path/to/file").context("Failed to read file")?;
    println!("{}", content);
    Ok(())
}

fn msg1() -> Result<(), Error> {
    read_file().context("msg1 failed")?;
    println!("this is a msg1");
    Ok(())
}

fn msg2() -> Result<(), Error> {
    msg1().context("msg2 failed")?;
    println!("this is a msg2");
    Ok(())
}

fn msg3() -> Result<(), Error> {
    msg2().context("msg3 failed")?;
    println!("this is a msg3");
    Ok(())
}

fn anyhow_demo() -> anyhow::Result<()> {
    Err(anyhow!("Missing attribute: {}", "missing"))
}

fn bail_demo() -> anyhow::Result<()> {
    bail!("Missing attribute: {}", "missing")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple() {
        if let Err(e) = msg3() {
            println!("{}", e);
            println!("====================================");
            println!("{:?}", e);
            println!("====================================");
            println!("{:#?}", e);
        }
    }
}
