use ::std::error::Error;
use ::std::path::Path;

use ::git2::{self, Commit};

use ::ModificationKind;

pub trait RepositoryExt {
    fn last_commit_that_modified_path<'r, P: AsRef<Path>>(
        &self
      , relative_file_path: P
      , start_commit: Commit<'r>
      , mod_kind: ModificationKind
    ) -> Result<Option<Commit<'r>>, String>;

    fn did_commit_modify_path<P: AsRef<Path>>(
        &self
      , commit: &Commit
      , relative_file_path: P
      , mod_kind: ModificationKind
    ) -> Result<bool, String>;
}

impl RepositoryExt for git2::Repository {
    fn last_commit_that_modified_path<'r, P: AsRef<Path>>(
        &self
      , relative_file_path: P
      , start_commit: Commit<'r>
      , mod_kind: ModificationKind
    ) -> Result<Option<Commit<'r>>, String> {
        let relative_file_path = relative_file_path.as_ref();
        let mut start_commit = start_commit;

        if self.did_commit_modify_path(&start_commit, relative_file_path, mod_kind).map_err(|e| e)? {
            return Ok(Some(start_commit));
        }

        while let Ok(parent_commit) = start_commit.parent(0) {
            start_commit = parent_commit;

            if self.did_commit_modify_path(&start_commit, relative_file_path, mod_kind).map_err(|e| e)? {
                return Ok(Some(start_commit));
            }
        }

        Ok(None)
    }

    fn did_commit_modify_path<P: AsRef<Path>>(
        &self
      , commit: &Commit
      , relative_file_path: P
      , mod_kind: ModificationKind
    ) -> Result<bool, String> {
        let relative_file_path = relative_file_path.as_ref();
        let tree = commit.tree().map_err(|e| e.description().to_string())?;

        let parent_tree = commit.parent(0)
            .map_err(|e| e.description().to_string())?
            .tree()
            .map_err(|e| e.description().to_string())?;

        let diff = self.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)
            .map_err(|e| e.description().to_string())?;

        Ok(diff.deltas().any(|dt|
            dt.old_file().path() == Some(relative_file_path)
            && mod_kind.eq_git2(dt.status())
        ))
    }
}