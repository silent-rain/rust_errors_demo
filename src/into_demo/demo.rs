use std::fs::read_to_string;

use anyhow::Context;
// use color_eyre::eyre::WrapErr;

use crate::into_demo::Error;
use crate::into_demo::ErrorMsg;
use crate::into_demo::error::IntoErrorMsg;

fn demo1() -> Result<(), Error> {
    let content = read_to_string("path/to/file")?;
    println!("{}", content);
    Ok(())
}

fn demo2() -> Result<(), Error> {
    demo1()?;
    Ok(())
}

fn demo3() -> Result<(), ErrorMsg> {
    demo1().map_err(|e| {
        println!("this is a error");
        e.into_err_with_msg("this is a msg")
    })?;
    demo2().into_err_with_msg("this is a msg")?;
    println!("this is a demo2");
    Ok(())
}

fn msg4() -> Result<(), ErrorMsg> {
    demo1()
        .context("this is a msg")
        .into_err_with_msg("this is a msg")?;

    demo1()
        .context("this is a msg")
        .into_err_with_appended_msg("this is a msg")?;
    Ok(())
}

fn msg5() -> Result<(), ErrorMsg> {
    demo2().context("this is a msg").into_err()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple() -> anyhow::Result<()> {
        color_eyre::install().expect("log ");

        if let Err(e) = demo3() {
            println!("{e:?}");
        }

        Ok(())
    }

    #[test]
    fn it_msg4() -> color_eyre::Result<()> {
        color_eyre::install().expect("log ");

        if let Err(e) = msg4() {
            println!("{}", e);
            println!("====================================");
            println!("{:?}", e);
            println!("====================================");
            println!("{:#?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_msg5() -> color_eyre::Result<()> {
        color_eyre::install().expect("log ");

        if let Err(e) = msg5() {
            println!("{}", e);
            println!("====================================");
            println!("{:?}", e);
            println!("====================================");
            println!("{:#?}", e);
        }

        Ok(())
    }
}
