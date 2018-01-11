//! `git2ext` provides a high-level, semantic API for dealing with git objects based on `git2`.
//!
//! The [`git2`](https://crates.io/crates/git2) crate provides low-level bindings to the [`libgit2`](https://libgit2.github.com/) library and, intentionally, not much more.
//!
//! This crate aims to provide the semantic *what* and *why* as opposed to the technical *how-to* that is present in `git2`.
//!
//! One such example of the high-level API is the ability to query a repository to determine which commit, if any, modified a path in a given way.
//!
//! `git2ext` is very much a work-in-progress. Please feel free to contribute, open issues, and discuss improvements!

pub extern crate git2;

mod repository_ext;

pub use repository_ext::RepositoryExt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ModificationKind {
    Any,
    Added,
    Deleted,
    Modified,
    Renamed,
}

impl ModificationKind {
    fn eq_git2(&self, delta: git2::Delta) -> bool {
        use ModificationKind::*;

        match (self, delta) {
              (&Any, _)
            | (&Added, git2::Delta::Added)
            | (&Deleted, git2::Delta::Deleted)
            | (&Modified, git2::Delta::Modified)
            | (&Renamed, git2::Delta::Renamed) => true,
            _ => false
        }
    }
}