#![allow(unused)]

use std::fs::read_to_string;

use eyre::anyhow;
use eyre::bail;
use eyre::eyre;
use eyre::{Context, Result};

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

fn anyhow_demo() -> eyre::Result<()> {
    Err(anyhow!("Missing attribute: {}", "missing"))
}

fn eyre_demo() -> eyre::Result<()> {
    Err(eyre!("Missing attribute: {}", "missing"))
}

fn bail_demo() -> eyre::Result<()> {
    bail!("Missing attribute: {}", "missing")
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

    #[test]
    fn it_anyhow_demo() {
        if let Err(e) = anyhow_demo() {
            println!("{:?}", e);
        }
    }

    #[test]
    fn it_eyre_demo() {
        if let Err(e) = eyre_demo() {
            println!("{:?}", e);
        }
    }

    #[test]
    fn it_bail_demo() {
        if let Err(e) = bail_demo() {
            println!("{:?}", e);
        }
    }
}
