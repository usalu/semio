//! 🚦️ Fs lifecycle state: the io error mapping, the lifecycle phases, the full sync group and epochs, the durable directories and the lifecycle with its test state. See [`super`].

use super::*;

/// 🚨️ Wraps a `std::io::Error` into `DbError::Io` — the only place `std::io::Error` is
/// allowed to appear, per the contract's no-`std::io::Error`-in-public-signatures rule.
#[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                         // 🚫️async: E4 fn-pointer slot
pub(super) fn io_err(err: std::io::Error) -> DbError {
    DbError::Io(err.to_string())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum FsLifecyclePhase {
    DirectoryCreated,
    DirectoryParentSynced,
    SegmentCreated,
    SegmentFileSynced,
    SegmentParentSynced,
    MarkerCreated,
    MarkerFileSynced,
    MarkerParentSynced,
    SegmentDeleted,
    SegmentDeleteParentSynced,
    MarkerDeleted,
    MarkerDeleteParentSynced,
    ReplacementRenamed,
    ReplacementParentSynced,
    WalFileSynced,
    WalParentSynced,
}

#[derive(Default)]
pub(super) struct FsLifecycle {
    pub(super) durable_directories: Mutex<FsDurableDirectories>,
    pub(super) full_sync: FsFullSyncGroup,
    #[cfg(test)]
    pub(super) state: Mutex<FsLifecycleState>,
}

/// 🚰️ One drive write-cache flush that every concurrent durable file sync of this root
/// shares. On Apple platforms a durable sync is the file's own `fsync` (its pages reach the
/// drive) followed by a device-wide `F_FULLFSYNC` (the drive's cache reaches the media): a flush
/// that starts after a member's `fsync` returned covers that member, so two dozen documents
/// committing at once pay a few cache flushes instead of one each, all serialised by the drive.
#[derive(Default)]
struct FsFullSyncGroup {
    pub(super) epochs: Mutex<FsFullSyncEpochs>,
    pub(super) flushed: std::sync::Condvar,
}

#[derive(Default)]
struct FsFullSyncEpochs {
    pub(super) requested: u64,
    pub(super) completed: u64,
    pub(super) flushing: bool,
}

/// 📁️ How many directories one executor remembers as durable before it forgets them all
/// and walks their ancestors again: a document owns four (wal, index, snapshot, payload), so the
/// bound covers every document a hub keeps mounted.
pub(super) const FS_DURABLE_DIRECTORIES: usize = 4_096;

/// 📁️ Directories this executor already made durable (created, every ancestor's entry
/// synced): a WAL sync or run replacement in one of them skips the whole-ancestry fsync walk,
/// which cost one directory fsync per path component on every durable write.
#[derive(Default)]
struct FsDurableDirectories {
    pub(super) paths: std::collections::HashSet<PathBuf>,
}

#[cfg(test)]
pub(super) struct FsLifecycleState {
    pub(super) trace: [Option<FsLifecyclePhase>; 256],
    pub(super) count: usize,
    pub(super) fail: Option<FsLifecyclePhase>,
}

#[cfg(test)]
impl Default for FsLifecycleState {
    fn default() -> Self {
        Self { trace: [None; 256], count: 0, fail: None }
    }
}

impl FsLifecycle {
    pub(super) fn hit(&self, phase: FsLifecyclePhase) -> Result<(), DbError> {
        #[cfg(test)]
        {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let count = state.count;
            if count < state.trace.len() {
                state.trace[count] = Some(phase);
                state.count += 1;
            }
            if state.fail == Some(phase) {
                state.fail = None;
                return Err(DbError::Io(format!("filesystem lifecycle injected after {phase:?}")));
            }
        }
        #[cfg(not(test))]
        let _ = phase;
        Ok(())
    }
}
