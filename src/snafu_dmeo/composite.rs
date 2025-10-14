#![allow(unused)]

use snafu::{Backtrace, ResultExt, Snafu};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("IO error: {}", source))]
    Io {
        source: std::io::Error,
        #[snafu(backtrace)]
        thing: Backtrace,
    },

    #[snafu(display("Unable to read configuration from {}", path.display()))]
    ReadConfiguration {
        source: std::io::Error,
        path: PathBuf,
    },

    NoArgument {
        #[snafu(backtrace)]
        thing: Backtrace,
    },

    ExplicitTrue {
        #[snafu(backtrace(true))]
        thing: Backtrace,
    },

    ExplicitFalse {
        #[snafu(backtrace(false))]
        backtrace: i32,
    },

    OptionBacktrace {
        backtrace: Option<snafu::Backtrace>,
    },

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
    use std::fs;

    use snafu::ErrorCompat;

    use super::*;

    #[test]
    fn it_simple() -> Result<(), Error> {
        if let Err(e) = msg3() {
            println!("{e}");
            println!("========================================");
            println!("{e:?}");
            println!("========================================");
            println!("{e:#?}");
        }

        Ok(())
    }

    #[test]
    fn it_explicit() -> Result<(), Error> {
        NoArgumentSnafu.fail()?;
        ExplicitTrueSnafu.fail()?;
        ExplicitFalseSnafu { backtrace: 42 }.fail()?;

        Ok(())
    }

    #[test]
    fn it_read_configuration() -> Result<(), Error> {
        let path = "config.toml";
        let _configuration = fs::read_to_string(path).context(ReadConfigurationSnafu { path })?;

        Ok(())
    }

    #[test]
    fn it_option_backtrace() -> Result<(), Error> {
        let sometimes = OptionBacktraceSnafu.build();
        assert!(ErrorCompat::backtrace(&sometimes).is_none());
        Ok(())
    }
}
