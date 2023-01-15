use std::{
    env::{
        self,
        VarError,
    },
    io::{
        self,
        ErrorKind,
    },
    path::PathBuf,
    process::Command,
};

pub trait ProgramOpener {
    fn open_editor(&self, file_path: &str) -> io::Result<()>;

    fn open_pager(&self, file_path: &str) -> io::Result<()>;
}

#[derive(Default)]
pub struct ProgramAccess;

#[allow(clippy::unused_self)]
impl ProgramAccess {
    fn open_with_fallback(&self, file_path: &str, env_var: &str, fallback: &str) -> io::Result<()> {
        let program = env::var(env_var)
            .map(PathBuf::from)
            .or_else(|_: VarError| self.get_if_available(fallback))?;

        // Make sure the file exists
        // Given a path, query the file system to get information about a file, directory, etc.
        std::fs::metadata(file_path)?;

        Command::new(program).arg(file_path).status().map(|_| ())
    }

    fn get_if_available(&self, program: &str) -> io::Result<PathBuf> {
        // A Rust equivalent of Unix command `which(1)`.
        which::which(program).map_err(|err| std::io::Error::new(ErrorKind::NotFound, err))
    }
}

impl ProgramOpener for ProgramAccess {
    fn open_editor(&self, file_path: &str) -> io::Result<()> {
        self.open_with_fallback(file_path, "EDITOR", "vi")
    }

    fn open_pager(&self, file_path: &str) -> io::Result<()> {
        self.open_with_fallback(file_path, "PAGER", "less")
    }
}
