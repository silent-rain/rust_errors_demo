// 类型转换 Error into ErrorMsg

use std::fs::read_to_string;

// use anyhow::Context;
use color_eyre::eyre::WrapErr;

use crate::into_demo::Error;
use crate::into_demo::ErrorMsg;
use crate::into_demo::IntoErrorMsg;

fn msg1() -> Result<(), Error> {
    let content = read_to_string("path/to/file")?;
    println!("{}", content);
    Ok(())
}

fn msg2() -> Result<(), Error> {
    msg1()?;
    Ok(())
}

fn msg3() -> Result<(), Error> {
    msg2()?;
    Ok(())
}

fn msg4() -> Result<(), Error> {
    msg3()?;
    Ok(())
}

fn into_err_demo1() -> Result<(), ErrorMsg> {
    msg4().map_err(|e| {
        println!("this is a error");
        e.into_err_with_msg("this is a msg")
    })?;

    Ok(())
}

fn into_err_demo2() -> Result<(), ErrorMsg> {
    msg4().into_err_with_msg("this is a msg")?;
    println!("this is a demo2");
    Ok(())
}

fn context_demo1() -> Result<(), ErrorMsg> {
    msg4()
        .context("this is a msg")
        .into_err_with_msg("this is a msg")?;

    Ok(())
}

fn context_demo2() -> Result<(), ErrorMsg> {
    msg4()
        .context("this is a msg")
        .into_err_with_appended_msg("this is a msg")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_into_err_demo1() -> anyhow::Result<()> {
        color_eyre::install().expect("init log failed");

        if let Err(e) = into_err_demo1() {
            println!("{}", e);
            println!("====================================");
            println!("{:?}", e);
            println!("====================================");
            println!("{:#?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_into_err_demo2() -> anyhow::Result<()> {
        color_eyre::install().expect("init log failed");

        if let Err(e) = into_err_demo2() {
            println!("{}", e);
            println!("====================================");
            println!("{:?}", e);
            println!("====================================");
            println!("{:#?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_context_demo1() -> color_eyre::Result<()> {
        color_eyre::install().expect("init log failed");

        if let Err(e) = context_demo1() {
            println!("{}", e);
            println!("====================================");
            println!("{:?}", e);
            println!("====================================");
            println!("{:#?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_context_demo2() -> color_eyre::Result<()> {
        color_eyre::install().expect("init log failed");

        if let Err(e) = context_demo2() {
            println!("{}", e);
            println!("====================================");
            println!("{:?}", e);
            println!("====================================");
            println!("{:#?}", e);
        }

        Ok(())
    }
}
