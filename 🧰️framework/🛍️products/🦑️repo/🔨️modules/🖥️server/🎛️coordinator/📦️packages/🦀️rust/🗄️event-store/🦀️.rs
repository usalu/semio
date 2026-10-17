//! 🗄️ Event store of the coordinator: the store error, the append and recovery records, the stage, the process locks and the event store with its paths and file operations. See [`super`].

use super::*;

//#region 🗄️EventStore
/// 🚨️ Every way an append, a replay or a recovery refuses to proceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    Unavailable(String),
    Corrupt(String),
    Duplicate(String),
    SequenceConflict(String),
    Limit(String),
    NotFound(String),
    PendingCleanup(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(detail) => write!(formatter, "event store unavailable: {detail}"),
            Self::Corrupt(detail) => write!(formatter, "event store corrupt: {detail}"),
            Self::Duplicate(detail) => write!(formatter, "duplicate event: {detail}"),
            Self::SequenceConflict(detail) => write!(formatter, "expected sequence conflict: {detail}"),
            Self::Limit(detail) => write!(formatter, "event store limit exceeded: {detail}"),
            Self::NotFound(detail) => write!(formatter, "projection not found: {detail}"),
            Self::PendingCleanup(detail) => write!(formatter, "append committed with pending cleanup: {detail}"),
        }
    }
}

impl std::error::Error for StoreError {}

/// 📦️ What one append committed, or observed as already committed.
#[derive(Debug, Clone, Default)]
pub struct AppendResult {
    pub events: Vec<EventEnvelope>,
    pub duplicate: bool,
    pub committed: bool,
    pub pending_cleanup: bool,
}

/// 🛟️ The last completed artifact-recovery action.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RecoveryStatus {
    pub recovered: bool,
    pub action: String,
}

#[derive(Debug, Clone, Default)]
pub(super) struct Stage {
    pub(super) prior_exists: bool,
    pub(super) prior_size: u64,
    pub(super) prior_checksum: String,
    pub(super) next_size: u64,
    pub(super) next_checksum: String,
}

impl Stage {
    pub(super) fn to_value(&self) -> Value {
        let mut object = Map::new();
        object.insert("schema".to_string(), Value::String(STAGE_SCHEMA.to_string()));
        object.insert("prior_exists".to_string(), Value::Bool(self.prior_exists));
        object.insert("prior_size".to_string(), Value::from(self.prior_size));
        object.insert("prior_checksum".to_string(), Value::String(self.prior_checksum.clone()));
        object.insert("next_size".to_string(), Value::from(self.next_size));
        object.insert("next_checksum".to_string(), Value::String(self.next_checksum.clone()));
        Value::Object(object)
    }

    pub(super) fn from_value(value: &Value) -> Option<Self> {
        let object = value.as_object()?;
        if object.get("schema").and_then(Value::as_str)? != STAGE_SCHEMA {
            return None;
        }
        let stage = Self {
            prior_exists: object.get("prior_exists").and_then(Value::as_bool)?,
            prior_size: object.get("prior_size").and_then(Value::as_u64)?,
            prior_checksum: object.get("prior_checksum").and_then(Value::as_str)?.to_string(),
            next_size: object.get("next_size").and_then(Value::as_u64)?,
            next_checksum: object.get("next_checksum").and_then(Value::as_str)?.to_string(),
        };
        if stage.next_size <= stage.prior_size || stage.prior_checksum.is_empty() || stage.next_checksum.is_empty() {
            return None;
        }
        Some(stage)
    }
}

#[derive(Debug, Default, Clone)]
pub(super) struct PersistedFile {
    pub(super) exists: bool,
    pub(super) size: u64,
    pub(super) checksum: String,
}

/// 🔐️ The in-process half of the store lock: one holder per absolute log path.
///
/// The Go twin keeps a `sync.Map` of semaphores next to the on-disk `.lock` file for exactly this
/// reason — two goroutines of one process would otherwise both find the lock file absent.
pub(super) fn process_locks() -> &'static (Mutex<std::collections::BTreeSet<PathBuf>>, Condvar) {
    static LOCKS: OnceLock<(Mutex<std::collections::BTreeSet<PathBuf>>, Condvar)> = OnceLock::new();
    LOCKS.get_or_init(|| (Mutex::new(std::collections::BTreeSet::new()), Condvar::new()))
}

/// 🗄️ One append-only log and the crash-recovery artifacts that belong to it.
#[derive(Debug)]
pub struct EventStore {
    pub(super) path: PathBuf,
    pub(super) limits: StoreLimits,
    pub(super) fault: Mutex<Fault>,
    pub(super) recovery: Mutex<RecoveryStatus>,
}

impl EventStore {
    /// 🆕️ Opens, recovers and validates an event log.
    pub fn open(path: impl AsRef<Path>, limits: StoreLimits) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        if path.as_os_str().is_empty() {
            return Err(StoreError::Unavailable("path is required".to_string()));
        }
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
            }
        }
        let store = Self { path, limits, fault: Mutex::new(Fault::default()), recovery: Mutex::new(RecoveryStatus::default()) };
        let guard = store.lock()?;
        store.recover_locked()?;
        store.replay_locked(true)?;
        drop(guard);
        Ok(store)
    }

    /// 💥️ Makes the store's next armed durable mutation fail; `0` disarms.
    pub fn arm_fault(&self, at: usize) {
        let mut fault = self.fault.lock().expect("fault");
        fault.armed = at;
        fault.seen = 0;
    }

    /// 🛟️ The last completed recovery action.
    pub fn recovery_status(&self) -> RecoveryStatus {
        self.recovery.lock().expect("recovery").clone()
    }

    /// 📍️ The log's own path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub(super) fn stage_path(&self) -> PathBuf {
        self.suffixed("stage")
    }

    pub(super) fn next_path(&self) -> PathBuf {
        self.suffixed("next")
    }

    pub(super) fn backup_path(&self) -> PathBuf {
        self.suffixed("backup")
    }

    pub(super) fn suffixed(&self, suffix: &str) -> PathBuf {
        PathBuf::from(format!("{}.{suffix}", self.path.display()))
    }

    pub(super) fn mutate(&self, path: &Path) -> Result<(), StoreError> {
        self.fault.lock().expect("fault").mutate(path)
    }

    pub(super) fn remove(&self, path: &Path) -> Result<(), StoreError> {
        match std::fs::symlink_metadata(path) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(failure) => return Err(StoreError::Unavailable(failure.to_string())),
            Ok(_) => {}
        }
        self.mutate(path)?;
        remove_durably(path)
    }

    pub(super) fn rename(&self, source: &Path, destination: &Path) -> Result<(), StoreError> {
        self.mutate(destination)?;
        rename_durably(source, destination)
    }

    pub(super) fn write(&self, path: &Path, chunks: &[&[u8]]) -> Result<(), StoreError> {
        self.remove(path)?;
        self.mutate(path)?;
        write_synced_exclusive(path, chunks)
    }
}
//#endregion 🗄️EventStore
