extern crate git2ext;
use git2ext::{git2, RepositoryExt, ModificationKind};

use git2::{Repository};

use std::env;

fn main() {
    let mut args = env::args().skip(1);

    let path_to_repo_root = args.nth(0).expect("No repo root provided!");
    let relative_file_path = args.nth(0).expect("No relative filepath provided!");

    let repo = Repository::open(path_to_repo_root).expect("Failed to open repo");
    let head = repo.head().unwrap();
    let head_oid = head.target().unwrap();
    let head_commit = repo.find_commit(head_oid).unwrap();

    match repo.last_commit_that_modified_path(&relative_file_path, head_commit, ModificationKind::Any).expect("Failed to determine which commit last modified the given path") {
        Some(commit) => println!("Commit {} last modified {} with message {:?}", commit.id(), relative_file_path, commit.message().unwrap_or("<none>")),
        None => println!("No commit was found to contain {}", relative_file_path),
    };
}
