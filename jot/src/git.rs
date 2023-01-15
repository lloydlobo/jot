use std::path::Path;

pub trait GitManagement {
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error>;
    fn checkout_branch(&mut self, branch_name: &str) -> Result<(), git2::Error>;
    fn add(&mut self) -> Result<(), git2::Error>;
    fn commit(&mut self, subject: &str) -> Result<(), git2::Error>;
    fn push(&mut self, branch_name: &str) -> Result<(), git2::Error>;
}

#[derive(Default)]
pub struct Git {
    repo: Option<git2::Repository>,
}

impl GitManagement for Git {
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error> {
        todo!()
    }

    fn checkout_branch(&mut self, branch_name: &str) -> Result<(), git2::Error> {
        todo!()
    }

    fn add(&mut self) -> Result<(), git2::Error> {
        todo!()
    }

    fn commit(&mut self, subject: &str) -> Result<(), git2::Error> {
        todo!()
    }

    fn push(&mut self, branch_name: &str) -> Result<(), git2::Error> {
        todo!()
    }
}

fn find_last_commint(repo: &git2::Repository) -> Result<git2::Commit, git2::Error> {
    todo!()
}

fn with_credentials<F>(repo: &git2::Repository, mut f: F) -> Result<(), git2::Error>
where
    F: FnMut(&mut git2::Credentials) -> Result<(), git2::Error>,
{
    todo!()
}
