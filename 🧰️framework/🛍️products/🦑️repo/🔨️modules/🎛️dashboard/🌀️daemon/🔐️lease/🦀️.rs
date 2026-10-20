//! 🔐️ The lease of a workspace daemon: the lock file the operating system releases when the daemon exits and the pid file
//! `repo dashboard daemon status` reads.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs

use super::ipc;
use std::path::{Path, PathBuf};

/// 🔒 The operating system releases this workspace lease when the daemon exits.
pub struct WorkspaceLease {
    _file: std::fs::File,
    root: PathBuf,
}

impl WorkspaceLease {
    pub fn acquire(root: &Path) -> std::io::Result<Self> {
        std::fs::create_dir_all(ipc::daemon_dir(root))?;
        let file = std::fs::OpenOptions::new().create(true).read(true).write(true).truncate(false).open(ipc::lock_path(root))?;
        file.try_lock().map_err(|_| std::io::Error::new(std::io::ErrorKind::AlreadyExists, "workspace dashboard daemon is already running"))?;
        Ok(Self { _file: file, root: root.to_path_buf() })
    }
}

impl Drop for WorkspaceLease {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(ipc::pid_path(&self.root));
    }
}

/// 🏷️ Writes the pid file `repo dashboard daemon status` reads.
pub fn write_pid(root: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(ipc::daemon_dir(root))?;
    std::fs::write(ipc::pid_path(root), format!("{}\n", std::process::id()))
}

pub fn read_pid(root: &Path) -> Option<u32> {
    std::fs::read_to_string(ipc::pid_path(root)).ok()?.trim().parse().ok()
}
