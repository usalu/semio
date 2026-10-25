//! 🛡️ Durability of the coordinator: the injected fault, the parent directory sync and the durable remove, rename and exclusive write. See [`super`].

use super::*;

//#region 🛡️Durability
/// 🚫️ The failure a store armed for fault injection raises.
pub const INJECTED_FAULT: &str = "injected durability fault";

/// 💥️ Fails the armed durable mutation and passes every other call through.
#[derive(Debug, Default)]
pub(super) struct Fault {
    pub(super) armed: usize,
    pub(super) seen: usize,
}

impl Fault {
    pub(super) fn mutate(&mut self, path: &Path) -> Result<(), StoreError> {
        if self.armed == 0 || path.extension().is_some_and(|value| value == "lock") {
            return Ok(());
        }
        self.seen += 1;
        if self.seen == self.armed {
            return Err(StoreError::Unavailable(format!("{INJECTED_FAULT} at mutation {} on {}", self.seen, path.display())));
        }
        Ok(())
    }
}

/// 💾️ Flushes the directory entry itself, so a rename or a removal survives a power loss.
#[cfg(unix)]
pub fn sync_parent(path: &Path) -> std::io::Result<()> {
    let parent = path.parent().unwrap_or(Path::new("."));
    std::fs::File::open(parent)?.sync_all()
}

/// 📂️ Flushes the directory entry itself.
///
/// A directory handle on Windows only exists with `FILE_FLAG_BACKUP_SEMANTICS`, and
/// `FlushFileBuffers` additionally refuses a handle that was not opened for writing — the same two
/// conditions the Go twin's `🪟️.go` states.
#[cfg(windows)]
pub fn sync_parent(path: &Path) -> std::io::Result<()> {
    use std::os::windows::fs::OpenOptionsExt;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    let parent = path.parent().unwrap_or(Path::new("."));
    OpenOptions::new().write(true).custom_flags(FILE_FLAG_BACKUP_SEMANTICS).open(parent)?.sync_all()
}

pub(super) fn remove_durably(path: &Path) -> Result<(), StoreError> {
    match std::fs::remove_file(path) {
        Ok(()) => sync_parent(path).map_err(|failure| StoreError::Unavailable(failure.to_string())),
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(failure) => Err(StoreError::Unavailable(failure.to_string())),
    }
}

pub(super) fn rename_durably(source: &Path, destination: &Path) -> Result<(), StoreError> {
    std::fs::rename(source, destination).map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
    sync_parent(destination).map_err(|failure| StoreError::Unavailable(failure.to_string()))
}

pub(super) fn write_synced_exclusive(path: &Path, chunks: &[&[u8]]) -> Result<(), StoreError> {
    remove_durably(path)?;
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
    for chunk in chunks {
        if let Err(failure) = file.write_all(chunk) {
            drop(file);
            let _ = std::fs::remove_file(path);
            return Err(StoreError::Unavailable(failure.to_string()));
        }
    }
    if let Err(failure) = file.sync_all() {
        drop(file);
        let _ = std::fs::remove_file(path);
        return Err(StoreError::Unavailable(failure.to_string()));
    }
    drop(file);
    sync_parent(path).map_err(|failure| StoreError::Unavailable(failure.to_string()))
}
//#endregion 🛡️Durability
