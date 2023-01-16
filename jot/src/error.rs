/// You can derive a `Diagnostic` from any `std::error::Error` type.
///
/// `thiserror` is a great way to define them, and plays nicely with `miette`!
use miette::{
    Diagnostic,
    SourceSpan,
};
use thiserror::Error;

// TODO:-----------------------------------------------------------------------
//
// std::io::Error::new(ErrorKind::NotFound, err))
// io::Error::new(ErrorKind::NotFound, "Could not resolve your $HOME directory")
// git2::Error
// git2::ErrorClass::Reference
// git2::ErrorCode::Exists
// git2::Error::from_str("Couldn't find commit")
// .map_err(|err| io::Error::new(ErrorKind::Other, err))?;
// env::VarError,             .or_else(|_: VarError| self.get_if_available(fallback))?;
// .map_err(|err: which::Error| std::io::Error::new(ErrorKind::NotFound, err))
//
//  ---------------------------------------------------------------------------

#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(code(oops::my::bad), url(docsrs), help("try doing it better next time?"))]
struct MyBad {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    bad_bit: SourceSpan,
}

/// Now let's define a function!
///
/// Use this `Result` type (or its expanded version) as the return type
/// throughout your app (but NOT your libraries! Those should always return
/// concrete types!).
use miette::{
    NamedSource,
    Result,
};

fn this_fails() -> Result<()> {
    // You can use plain strings as a `Source`, or anything that implements
    // the one-method `Source` trait.
    let src = "source\n  text\n    here".to_string();
    let len = src.len();

    Err(MyBad { src: NamedSource::new("bad_file.rs", src), bad_bit: (9, 4).into() })?;

    Ok(())
}

/// Now to get everything printed nicely, just return a `Result<()>`
/// and you're all set!
///
/// Note: You can swap out the default reporter for a custom one using
/// `miette::set_hook()`
pub fn pretend_this_is_main() -> Result<()> {
    // kaboom~
    this_fails()?;

    Ok(())
}
