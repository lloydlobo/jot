use std::path::Path;

use anyhow::Context;

pub trait GitManagement {
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error>;
    fn checkout_branch(&mut self, branch_name: &str) -> Result<(), git2::Error>;
    fn add(&mut self) -> Result<(), git2::Error>;
    fn commit(&mut self, subject: &str) -> Result<git2::Oid, git2::Error>;
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
    /// Owned git repository, of all state associated with the underlying filesystem.
    ///
    /// # Panics
    ///
    /// Panics if fails to open git repository with Error `code: -3`, `class: 2`, `message: "failed
    /// to resolve path '$HOME/path/to/jot/repo': No such file or directory`.
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error> {
        let path: &Path = Path::new(&repo_path); // Directly wraps a string slice as a `Path` slice.
        let repository = git2::Repository::open(path);
        repository.map(|repo| self.repo = Some(repo))
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

    /// Get the Index file for this repository.
    ///
    /// If a custom index has not been set, the default index for the repository
    /// will be returned (the one located in .git/index).
    ///
    /// Create a new action signature with default user and now timestamp.
    fn commit(&mut self, subject: &str) -> Result<git2::Oid, git2::Error> {
        let repo: &git2::Repository = self.repo.as_ref().unwrap();
        let mut index: git2::Index = repo.index()?;

        let signature: git2::Signature = repo.signature()?; // Use default user.name and user.email

        let oid: git2::Oid = index.write_tree()?; // Unique identity of any object (commit, tree, blob, tag).
        let parent_commit: git2::Commit = find_last_commit(self.repo.as_ref().unwrap())?; // A structure to represent a git [commit][1]
        let tree: git2::Tree = repo.find_tree(oid)?; // A structure to represent a git [tree][1]

        repo.commit(
            Some("HEAD"),      // Point HEAD to our new commit
            &signature,        // Author
            &signature,        // Committer
            subject,           // Commit message
            &tree,             // Tree
            &[&parent_commit], // Parent Commit
        )
    }

    /*
    TODO:
     >> Jot summary
    > Test 1
    Git initialized successfully
    [jot/src/lib.rs:169] &t = ()
    Adding and committing you new jot to main..
    Added and committed:
    Pushing your new jot...
    Pushed! */
    fn push(&mut self, branch_name: &str) -> Result<(), git2::Error> {
        // Ok(())
        todo!()
    }
}

/// * `head` - Retrieve and resolve the reference pointed at by HEAD.
/// * `resolve` - Resolve a symbolic reference to a direct reference. This method iteratively peels
///   a symbolic reference until it resolves to a direct reference to an OID.
/// * `peel` - Peel a reference to an object - This method recursively peels the reference until it
///   reaches an object of the specified type.
///     * `git2::ObjectType::Commit` An object which corresponds to a git commit.
fn find_last_commit(repo: &git2::Repository) -> Result<git2::Commit, git2::Error> {
    let obj: git2::Object = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn with_credentials<F>(repo: &git2::Repository, mut f: F) -> Result<(), git2::Error>
where
    F: FnMut(&mut git2::Credentials) -> Result<(), git2::Error>,
{
    todo!()
    // Ok(())
}
