#![allow(unused)]

use std::fs::read_to_string;

use color_eyre::{Result, eyre::WrapErr, eyre::anyhow, eyre::bail};

fn read_file() -> Result<()> {
    let content = read_to_string("path/to/file").wrap_err("Failed to read file")?;
    println!("{}", content);
    Ok(())
}

fn msg1() -> Result<()> {
    read_file().wrap_err("msg1 failed")?;
    println!("this is a msg1");
    Ok(())
}

fn msg2() -> Result<()> {
    msg1().wrap_err("msg2 failed")?;
    println!("this is a msg2");
    Ok(())
}

fn msg3() -> Result<()> {
    msg2().wrap_err("msg3 failed")?;
    println!("this is a msg3");
    Ok(())
}

fn anyhow_demo() -> color_eyre::Result<()> {
    Err(anyhow!("Missing attribute: {}", "missing"))
}

fn bail_demo() -> color_eyre::Result<()> {
    bail!("Missing attribute: {}", "missing")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple() -> color_eyre::Result<()> {
        color_eyre::install()?;

        if let Err(e) = msg3() {
            println!("{:?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_anyhow_demo() -> color_eyre::Result<()> {
        if let Err(e) = anyhow_demo() {
            println!("{:?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_bail_demo() -> color_eyre::Result<()> {
        if let Err(e) = bail_demo() {
            println!("{:?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_anyhow_demo2() -> color_eyre::Result<()> {
        color_eyre::install()?;

        if let Err(e) = anyhow_demo() {
            println!("{:?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_bail_demo2() -> color_eyre::Result<()> {
        color_eyre::install()?;

        if let Err(e) = bail_demo() {
            println!("{:?}", e);
        }

        Ok(())
    }
}
