//! 🎬️ Executor: applying a plan to a workspace or a disk.

use super::{clean_relative, fail, Change, Entry, MoveError, Plan, Workspace};
use std::fs;
use std::path::{Path, PathBuf};

//#region 🎬️Executor

/// 🗃️ Applies the changes of a plan to some backing store.
pub trait Executor {
    /// ✏️ Applies one change.
    fn apply(&mut self, change: &Change) -> Result<(), MoveError>;
}

/// ⏮️ Applies every change of `plan` in order, stopping at the first failure.
pub fn execute(plan: &Plan, executor: &mut dyn Executor) -> Result<(), MoveError> {
    for change in &plan.changes {
        executor.apply(change)?;
    }
    Ok(())
}

impl Executor for Workspace {
    fn apply(&mut self, change: &Change) -> Result<(), MoveError> {
        match change {
            Change::Write { path, content } => {
                self.insert_file(path, content);
                Ok(())
            }
            Change::CreateDirectory { path } => {
                self.insert_directory(path);
                Ok(())
            }
            Change::Delete { path } => {
                let key = clean_relative(path);
                let prefix = format!("{key}/");
                self.entries.retain(|p, _| p != &key && !p.starts_with(&prefix));
                Ok(())
            }
            Change::Move { from, to } => {
                let source = clean_relative(from);
                let target = clean_relative(to);
                let Some(entry) = self.entries.remove(&source) else {
                    return fail(format!("Source not found: {source}"));
                };
                let prefix = format!("{source}/");
                let moved: Vec<(String, Entry)> = self
                    .entries
                    .keys()
                    .filter(|p| p.starts_with(&prefix))
                    .cloned()
                    .collect::<Vec<String>>()
                    .into_iter()
                    .filter_map(|p| self.entries.remove(&p).map(|e| (format!("{}/{}", target, &p[prefix.len()..]), e)))
                    .collect();
                self.insert_ancestors(&target);
                self.entries.insert(target, entry);
                for (path, entry) in moved {
                    self.entries.insert(path, entry);
                }
                Ok(())
            }
        }
    }
}

/// 💾️ Applies changes to a real directory tree rooted at a path.
#[derive(Debug, Clone)]
pub struct FileSystemExecutor {
    pub(super) root: PathBuf,
}

impl FileSystemExecutor {
    /// 🆕️ An executor writing under `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn absolute(&self, path: &str) -> PathBuf {
        let mut target = self.root.clone();
        for segment in clean_relative(path).split('/').filter(|s| !s.is_empty()) {
            target.push(segment);
        }
        target
    }

    fn ensure_parent(target: &Path) -> Result<(), MoveError> {
        match target.parent() {
            Some(parent) => fs::create_dir_all(parent).map_err(|error| MoveError(error.to_string())),
            None => Ok(()),
        }
    }
}

impl Executor for FileSystemExecutor {
    fn apply(&mut self, change: &Change) -> Result<(), MoveError> {
        match change {
            Change::Write { path, content } => {
                let target = self.absolute(path);
                Self::ensure_parent(&target)?;
                fs::write(&target, content).map_err(|error| MoveError(error.to_string()))
            }
            Change::CreateDirectory { path } => fs::create_dir_all(self.absolute(path)).map_err(|error| MoveError(error.to_string())),
            Change::Delete { path } => {
                let target = self.absolute(path);
                let result = if target.is_dir() { fs::remove_dir_all(&target) } else { fs::remove_file(&target) };
                result.map_err(|error| MoveError(error.to_string()))
            }
            Change::Move { from, to } => {
                let target = self.absolute(to);
                Self::ensure_parent(&target)?;
                fs::rename(self.absolute(from), &target).map_err(|error| MoveError(error.to_string()))
            }
        }
    }
}

//#endregion 🎬️Executor
