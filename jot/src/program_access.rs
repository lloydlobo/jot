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
    /// # Panics
    ///
    /// * `thread 'main' panicked at 'The file you are trying to open does not exist;
    ///   /home/username/path/to/jot/README.md', jot/src/program_access.rs:33:13`
    fn open_with_fallback(&self, file_path: &str, env_var: &str, fallback: &str) -> io::Result<()> {
        let program: PathBuf = env::var(env_var)
            .map(PathBuf::from)
            .or_else(|_: VarError| self.get_if_available(fallback))?;

        // Make sure the file exists
        // Given a path, query the file system to get information about a file, directory, etc.
        std::fs::metadata(&file_path).unwrap_or_else(|_| {
            panic!("The file you are trying to open does not exist; {file_path}")
        });

        Command::new(program).arg(&file_path).status().map(|_| ())
    }

    /// A Rust equivalent of Unix command `which(1)`.
    fn get_if_available(&self, program: &str) -> io::Result<PathBuf> {
        which::which(program)
            .map_err(|err: which::Error| std::io::Error::new(ErrorKind::NotFound, err))
    }
}

// in lib.rs `fn ask_for_jot(&mut self) -> io::Result<()> `
// FIXME: Fix opening EDITOR `vi` or pager `less`. Avoid either to panic.
// PERF: Directly input from stdin for now.
impl ProgramOpener for ProgramAccess {
    fn open_editor(&self, file_path: &str) -> io::Result<()> {
        // let x = self.open_with_fallback(file_path, "EDITOR", "vi");
        // match x {
        //     Ok(q) => {
        //         dbg!(&q);
        //         Ok(q)
        //     }
        //     Err(e) => {
        //         dbg!(&e);
        //         Err(e)
        //     }
        // }
        self.open_with_fallback(file_path, "EDITOR", "vi")
    }

    fn open_pager(&self, file_path: &str) -> io::Result<()> {
        self.open_with_fallback(file_path, "PAGER", "less")
    }
}
