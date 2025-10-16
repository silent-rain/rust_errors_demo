// 类型转换 Error into ErrorMsg + 上下文信息

use std::fs::read_to_string;

use anyhow::Context;
// use color_eyre::eyre::WrapErr;

use crate::into_demo::Error;
use crate::into_demo::ErrorMsg;
use crate::into_demo::IntoErrorMsg;

fn msg1() -> Result<(), Error> {
    let content = read_to_string("path/to/file").context("this is a msg1")?;
    println!("{}", content);
    Ok(())
}

fn msg2() -> Result<(), Error> {
    msg1().context("this is a msg2")?;
    Ok(())
}

fn msg3() -> Result<(), Error> {
    msg2().context("this is a msg3")?;
    Ok(())
}

fn msg4() -> Result<(), Error> {
    msg3().context("this is a msg4")?;
    Ok(())
}

fn context_demo1() -> Result<(), ErrorMsg> {
    msg4()
        .context("this is a context_demo1")
        .into_err_with_msg("this is a context_demo1")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
