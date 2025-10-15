#![allow(unused)]
/*
You can derive a `Diagnostic` from any `std::error::Error` type.

`thiserror` is a great way to define them, and plays nicely with `miette`!
*/
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(
    code(oops::my::bad),
    url(docsrs),
    help("try doing it better next time?")
)]
struct MyBad {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    bad_bit: SourceSpan,
}

/*
Now let's define a function!

Use this `Result` type (or its expanded version) as the return type
throughout your app (but NOT your libraries! Those should always return
concrete types!).
*/
use miette::Result;
fn this_fails() -> Result<()> {
    // You can use plain strings as a `Source`, or anything that implements
    // the one-method `Source` trait.
    let src = "source\n  text\n    here".to_string();

    Err(MyBad {
        src: NamedSource::new("bad_file.rs", src),
        bad_bit: (9, 4).into(),
    })?;

    Ok(())
}

/*
Now to get everything printed nicely, just return a `Result<()>`
and you're all set!

Note: You can swap out the default reporter for a custom one using
`miette::set_hook()`
*/
fn pretend_this_is_main() -> Result<()> {
    // kaboom~
    this_fails()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple() {
        if let Err(e) = pretend_this_is_main() {
            println!("{e}");
            println!("========================================");
            println!("{e:?}");
            println!("========================================");
            println!("{e:#?}");
        }
    }
}
