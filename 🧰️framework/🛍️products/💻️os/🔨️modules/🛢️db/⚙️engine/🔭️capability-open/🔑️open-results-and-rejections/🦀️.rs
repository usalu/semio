//! 🔑️ Capability open results and rejections: the progress and close steps, the open result and the rejected open. See [`super`].

use super::*;

//#region 🔖️CapabilityOpen
/// 🧭️ Progress of one retained database capability probe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCapabilityOpenProgress {
    Admitted,
    Scheduled,
    Polling,
    Pending,
    Completed,
    Cancelled,
    Fault,
}

/// 🧹️ One bounded public terminal-cleanup opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCapabilityOpenCloseStep {
    Progress,
    Blocked,
    Complete,
}

/// 📦️ Exact storage owner and capability scalar returned by the retained probe.
pub struct DatabaseCapabilityOpenResult {
    pub storage: Arc<db_storage::DbBackend>,
    pub capabilities: db_io_core::StorageCapabilities,
}

impl DatabaseCapabilityOpenResult {
    pub fn into_parts(self) -> (Arc<db_storage::DbBackend>, db_io_core::StorageCapabilities) {
        (self.storage, self.capabilities)
    }
}

/// ↩️ Admission rejection that returns the exact storage owner unchanged.
pub struct DatabaseCapabilityOpenRejected {
    pub error: Option<DbError>,
    pub storage: Option<Arc<db_storage::DbBackend>>,
}

impl DatabaseCapabilityOpenRejected {
    pub fn error(&self) -> &DbError {
        self.error.as_ref().expect("database capability-open rejection error owner missing")
    }

    pub fn take_storage(&mut self) -> Option<Arc<db_storage::DbBackend>> {
        self.storage.take()
    }

    pub fn retry(mut self, pool: Arc<WorkerPool>) -> Result<DatabaseCapabilityOpenFuture, Self> {
        let Some(storage) = self.storage.take() else {
            return Err(self);
        };
        match DatabaseCapabilityOpenFuture::try_submit(pool, storage) {
            Ok(future) => Ok(future),
            Err(rejected) => Err(rejected),
        }
    }

    pub fn close_step(&mut self) -> DatabaseCapabilityOpenCloseStep {
        if let Some(storage) = self.storage.take() {
            drop(storage);
            DatabaseCapabilityOpenCloseStep::Progress
        } else {
            DatabaseCapabilityOpenCloseStep::Complete
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none()
    }

    pub fn into_error_after_close(mut self) -> Result<DbError, Self> {
        if self.storage.is_some() {
            return Err(self);
        }
        Ok(self.error.take().expect("database capability-open rejection error owner missing"))
    }

    pub fn close_and_take_error(mut self) -> DbError {
        if let Some(storage) = self.storage.take() {
            drop(storage);
        }
        self.error.take().expect("database capability-open rejection error owner missing")
    }

    pub fn into_parts(mut self) -> (DbError, Arc<db_storage::DbBackend>) {
        (self.error.take().expect("database capability-open rejection error owner missing"), self.storage.take().expect("database capability-open rejected storage owner missing"))
    }
}

impl std::fmt::Debug for DatabaseCapabilityOpenRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCapabilityOpenRejected").field("error", &self.error).field("storage", &self.storage.as_ref().map(Arc::as_ptr)).finish()
    }
}
//#endregion 🔖️CapabilityOpen
