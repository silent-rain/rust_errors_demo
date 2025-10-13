#![allow(unused)]

use anyhow::{Context, Result};
use std::fs::read_to_string;

fn read_file() -> Result<()> {
    let content = read_to_string("path/to/file").context("Failed to read file")?;
    println!("{}", content);
    Ok(())
}

fn msg1() -> Result<()> {
    read_file().context("msg1 failed")?;
    println!("this is a msg1");
    Ok(())
}

fn msg2() -> Result<()> {
    msg1().context("msg2 failed")?;
    println!("this is a msg2");
    Ok(())
}

fn msg3() -> Result<()> {
    msg2().context("msg3 failed")?;
    println!("this is a msg3");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple() {
        if let Err(e) = msg3() {
            println!("{:?}", e);
        }
    }
}
