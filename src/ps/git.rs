// This is the `git` module. It is responsible for housing
// functionality for interacting with git. Nothing in here should explicitly
// introduce patch stack concepts but obviously should be needed to support
// implementing the Patch Stack solutions at a higher level.
//
// Lets look at an example to make this more clear.
//
// fn get_commits(ps: PatchStack) -> Vec<Commit> // bad example
//
// The above is something that should NOT live in here because it introduces a
// concept specific to Patch Stack, in this case the `PatchStack` struct.
//
// We can still have the same functionality in here as it is mostly specific
// to git. If we simply write the function at the conceptual level of git
// instead it might look something like the following.
//
// fn get_comimts(head: Oid, base: Oid) -> Vec<Commit> // good example
//
// In the above two examples we can see that we are effectively providing
// the same functionality the but the API we are exposing at this level is
// constrained to the conceptual level of git and isn't aware of any Patch
// Stack specific concepts.
//
// This explicitly intended to NOT wrap libgit2. Instead it is designed to
// extend the functionality of libgit2. This means that it's functions will
// consume libgit2 types as well as potentially return libgit2 types.
//
// All code fitting that description belongs here.

use git2;

#[derive(Debug)]
pub enum GitError {
  GitError(git2::Error),
  NotFound
}

impl From<git2::Error> for GitError {
    fn from(e: git2::Error) -> Self {
        Self::GitError(e)
    }
}

/// Attempt to open an already-existing repository at or above current working
/// directory
pub fn create_cwd_repo() -> Result<git2::Repository, GitError> {
    let repo = git2::Repository::discover("./")?;
    Ok(repo)
}

/// Get Commit Summary given a repository & oid
pub fn get_summary(repo: &git2::Repository, oid: &git2::Oid) -> Result<String, GitError>{
    Ok(String::from(repo.find_commit(*oid)?
                        .summary().ok_or(GitError::NotFound)?))
}

/// Attempt to get uptream branch name given local branch name
pub fn branch_upstream_name(repo: &git2::Repository, branch_name: &str) -> Result<String, GitError> {
  let upstream_branch_name_buf = repo.branch_upstream_name(branch_name)?;
  Ok(String::from(upstream_branch_name_buf.as_str().ok_or(GitError::NotFound)?))
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_summary() {
        let (_td, repo) = crate::ps::test::repo_init();
        let head_id = repo.refname_to_id("HEAD").unwrap();

        let res = super::get_summary(&repo, &head_id);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "initial");
    }
}
