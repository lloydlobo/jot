use std::io::{
    self,
    Write,
};

pub trait Print {
    fn print(&mut self, value: &str) -> io::Result<()>;
    fn println(&mut self, value: &str) -> io::Result<()>;
}
pub trait PrintColor {
    /// Prints out First time setup banner.
    fn fts_banner(&mut self) -> io::Result<()>;
    fn input_header(&mut self, value: &str) -> io::Result<()>;
    fn error(&mut self, value: &str) -> io::Result<()>;
}

pub struct Printer<W> {
    writer: W,
}

#[derive(Clone, Copy)]
pub struct PrintOptions {
    color: termcolor::Color,
    is_bold: bool,
}

impl<W> Print for Printer<W>
where
    W: Write,
{
    fn print(&mut self, value: &str) -> io::Result<()> {
        write!(self.writer, "{value}")
    }

    /// First time prints when setting up jot with local repo.
    fn println(&mut self, value: &str) -> io::Result<()> {
        writeln!(self.writer, "{value}")
    }
}

impl<W> Printer<W>
where
    W: Write + termcolor::WriteColor,
{
    pub const fn new(writer: W) -> Self {
        Self { writer }
    }

    #[allow(clippy::unused_self)]
    fn println_styled(&mut self, value: &str, opts: PrintOptions) -> Result<(), io::Error> {
        let mut color_spec = termcolor::ColorSpec::new();
        color_spec.set_fg(Some(opts.color)).set_bold(opts.is_bold);
        self.writer.set_color(&color_spec)?;
        writeln!(self.writer, "{value}")?;
        self.writer.reset()
    }
}

impl<W> PrintColor for Printer<W>
where
    W: Write + termcolor::WriteColor,
{
    fn fts_banner(&mut self) -> io::Result<()> {
        let opts = PrintOptions { color: termcolor::Color::Yellow, is_bold: false };
        let banner = format!(
            "{}\n{}{}{}{}{}\n{}",
            "#".repeat(60),
            "#".repeat(4),
            " ".repeat(18),
            "First Time Setup",
            " ".repeat(18),
            "#".repeat(4),
            "#".repeat(60)
        );
        let description = r#"
This tool requires you to have a repository with a README.md
in the root folder. The markdown file is where your jots
will be stored.
Once first time setup has completed, simply run Jot again
to start jotting down your snippets, haiku, tips & tricks.
        "#;

        self.println_styled(&format!("{}\n{}", banner.as_str(), description), opts)
    }

    fn input_header(&mut self, value: &str) -> io::Result<()> {
        let opts = PrintOptions { color: termcolor::Color::Green, is_bold: true };
        self.println_styled(value, opts)?;
        self.print("> ")?;
        self.writer.flush()
    }

    fn error(&mut self, value: &str) -> io::Result<()> {
        let opts = PrintOptions { color: termcolor::Color::Red, is_bold: false };

        self.println_styled(value, opts)?;
        self.writer.flush()
    }
}
