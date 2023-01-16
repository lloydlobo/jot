use std::path::Path;

use anyhow::Context;

pub trait GitManagement {
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error>;
    fn checkout_branch(&mut self, branch_name: &str) -> Result<(), git2::Error>;
    fn add(&mut self) -> Result<(), git2::Error>;
    fn commit(&mut self, subject: &str) -> Result<(), git2::Error>;
    fn push(&mut self, branch_name: &str) -> Result<(), git2::Error>;
}

#[derive(Default)]
pub struct Git {
    /// Git repository `repo: Option<git2::Repository>`.
    repo: Option<git2::Repository>,
}

impl GitManagement for Git {
    /// Attempts to open an already-existing repository at `repo_path`.
    ///
    /// The path can point to either a normal or bare repository.
    ///
    /// # Panics
    ///
    /// Panics if fails to open git repository with Error `code: -3`, `class: 2`, `message: "failed
    /// to resolve path '$HOME/path/to/jot/repo': No such file or directory`.
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error> {
        let path: &Path = Path::new(&repo_path); // Directly wraps a string slice as a `Path` slice.
        let repository = git2::Repository::open(path);
        dbg!(&repository?.path());

        let path: &Path = Path::new(&repo_path); // Directly wraps a string slice as a `Path` slice.
        let repository = git2::Repository::open(path);
        match repository {
            Ok(repo) => {
                dbg!(&repo.path());
                self.repo = Some(repo);
                Ok(())
            }
            Err(err) => Err(err),
        }

        // Owned git repository, of all state associated with the underlying filesystem.
        // repository.map(|repo| self.repo = Some(repo))
    }

    /// Updates files in the index and working tree to match the content of the
    /// tree pointed at by the treeish.
    /// Make the repository HEAD point to the specified reference.
    ///
    /// * `head` - `Reference` - A structure to represent a git [reference][1].
    /// * `head()` - Retrieve and resolve the reference pointed at by HEAD.
    /// * `target()` - Get the OID pointed to by a direct reference.
    /// * `find_commit()` - Lookup a reference to one of the commits in a repository.
    /// * `Oid` - Unique identity of any object (commit, tree, blob, tag).
    ///
    /// * `repo.branch()` - Create a new branch pointing at a target commit, if it doesn't exist.
    ///   This command can fail due to an existing reference. This error can be ignored.
    fn checkout_branch(&mut self, branch_name: &str) -> miette::Result<(), git2::Error> {
        let repo = self.repo.as_ref().unwrap();

        let commit = repo
            .head()
            .map(|head: git2::Reference| head.target())
            .and_then(|old: Option<git2::Oid>| repo.find_commit(old.unwrap()))?;

        if let Err(err) = repo.branch(branch_name, &commit, false) {
            let has_err_code_class_ref =
                err.code() == git2::ErrorCode::Exists && err.class() == git2::ErrorClass::Reference;
            if !has_err_code_class_ref {
                return Err(err);
            }
        }

        let refname = format!("refs/heads/{branch_name}");
        let obj: git2::Object = repo.revparse_single(refname.as_str())?;

        repo.checkout_tree(&obj, None)?;
        repo.set_head(refname.as_str())
    }

    fn add(&mut self) -> Result<(), git2::Error> {
        let mut index: git2::Index =
            self.repo.as_ref().expect("The repository is not initialized.").index()?;

        index.add_path(Path::new("README.md"))?;
        index.write()
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
