//! Jot allows you to take a quick note and upload it to a repository.

#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(dead_code)]
#![warn(unused_variables)]
#![warn(unused_must_use)]
#![deny(clippy::useless_format)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)]
#![allow(anonymous_parameters)]
#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_docs_in_private_items)]

extern crate dirs;
#[macro_use]
extern crate log;
extern crate core;

pub mod config_manager;
pub mod git;
pub mod printer;
pub mod program_access;
pub mod reader;

use std::{
    io::{
        self,
        Error,
        ErrorKind,
    },
    path::Path,
};

use crate::{
    config_manager::{
        ConfigManagement,
        ConfigType::Repo,
    },
    git::GitManagement,
    printer::{
        Print,
        PrintColor,
    },
    program_access::ProgramOpener,
    reader::ReadInput,
};

pub struct Jot<CM, W, R, G, PO>
where
    CM: ConfigManagement,
    W: Print + PrintColor,
    R: ReadInput,
    G: GitManagement,
    PO: ProgramOpener,
{
    /// CM: ConfigManagement.
    cm: CM,
    /// W: Print + PrintColor,
    printer: W,
    /// R: ReadInput,
    reader: R,
    /// G: GitManagement,
    git: G,
    /// PO: ProgramOpener,
    program_opener: PO,
}

#[derive(Debug)]
pub struct JotOptions {
    /// Clear the stored configuraion.
    pub clear_config: bool,

    /// Open idea document with $PAGER (fall back to `less`).
    pub view: bool,
}

impl<CM, W, R, G, PO> Jot<CM, W, R, G, PO>
where
    CM: ConfigManagement,
    W: Print + PrintColor,
    R: ReadInput,
    G: GitManagement,
    PO: ProgramOpener,
{
    pub const fn new(cm: CM, printer: W, reader: R, git: G, program_opener: PO) -> Self {
        Self { cm, printer, reader, git, program_opener }
    }

    pub fn run(&mut self, opts: JotOptions) -> io::Result<()> {
        debug!("Running with options: {:?}", &opts);

        if opts.clear_config {
            self.clear_config()?;
            debug!("Cleared config");
            return Ok(());
        }

        if opts.view {
            self.open_jot_file()?;
            debug!("Opening idea file");
            return Ok(());
        }

        // First time setup
        if self.is_config_missing() {
            debug!("Config is missing");
            // If config dir is missing - create it
            if !self.cm.config_dir_exists() {
                self.cm.config_dir_create()?;
                debug!("Created config directory");
            }
            self.printer.fts_banner()?;

            // If repo path is missing - ask for it
            if self.cm.config_read(Repo).is_err() {
                self.setup_repo_path()?;
                debug!("Repo path setup successfully");
            }

            self.printer.println("First time setup completed. Happy jotting")?;
            Ok(())
        } else {
            self.ask_for_jot()
        }
    }

    fn clear_config(&self) -> io::Result<()> {
        self.cm.config_rm()
    }

    pub(crate) fn open_jot_file(&self) -> io::Result<()> {
        self.program_opener.open_pager(&format!("{}/README.md", self.cm.config_read(Repo)?))
    }

    fn ask_for_jot(&mut self) -> io::Result<()> {
        let mut jot_summary = String::new();

        while jot_summary.is_empty() {
            self.printer.input_header(">> Jot summary")?;
            jot_summary = self.reader.read_input()?;
        }

        let repo_path = self.cm.config_read(Repo)?;

        self.program_opener
            .open_editor(&format!("{}/README.md", &repo_path))
            .and(self.git_add_commit_push(jot_summary))
    }

    fn git_add_commit_push(&mut self, commit_subject: String) -> io::Result<()> {
        let branch_name = "main";

        self.printer
            .println(&format!("Adding and committing you new jot to {}..", &branch_name))?;
        self.git
            .checkout_branch(branch_name)
            .and_then(|_| self.git.add())
            .and_then(|_| self.git.commit(commit_subject.as_str()))
            .map_err(|err| io::Error::new(ErrorKind::Other, err))?;
        self.printer.println("Added and committed:")?;

        self.printer.println("Pushing your new jot...")?;
        self.git.push(branch_name).map_err(|err| io::Error::new(ErrorKind::Other, err))?;
        self.printer.println("Pushed!")?;

        Ok(())
    }

    fn is_config_missing(&self) -> bool {
        self.cm.config_read(Repo).is_err()
    }

    fn setup_repo_path(&self) -> io::Result<()> {
        error!("not yet implemented");
        todo!()
    }
}
