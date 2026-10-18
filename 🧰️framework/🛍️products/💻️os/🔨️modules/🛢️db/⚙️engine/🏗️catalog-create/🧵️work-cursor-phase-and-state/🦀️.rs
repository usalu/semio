//! 🧵️ Work, cursor, phase and state: the work, the encode cursor, the phase, the driver authority, the state and its wake. See [`super`].

use super::*;

//#region 🔖️CreateDocumentCatalogCas
pub(super) struct DatabaseCreateCatalogWork {
    pub future: Option<DatabaseCreateCatalogBackendFuture>,
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub pages: Option<db_io_core::DbIoPages>,
    pub expected: EpochFence,
    #[cfg(any(test, feature = "testkit"))]
    pub storage_identity: usize,
    #[cfg(any(test, feature = "testkit"))]
    pub page_identity: Option<u64>,
}

impl DatabaseCreateCatalogWork {
    pub fn new(storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, expected: EpochFence) -> Self {
        #[cfg(any(test, feature = "testkit"))]
        let storage_identity = Arc::as_ptr(&storage) as usize;
        #[cfg(any(test, feature = "testkit"))]
        let page_identity = Some(pages.operation());
        Self {
            future: None,
            storage: Some(storage),
            pages: Some(pages),
            expected,
            #[cfg(any(test, feature = "testkit"))]
            storage_identity,
            #[cfg(any(test, feature = "testkit"))]
            page_identity,
        }
    }

    pub fn poll(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<EpochFence, DbError>> {
        if self.future.is_none() {
            let storage = self.storage.take();
            let pages = self.pages.take();
            match (storage, pages) {
                (Some(storage), Some(pages)) => {
                    let expected = self.expected;
                    self.future = Some(Box::pin(async move { storage.catalog().await.cas_root(expected, pages).await }));
                }
                (storage, pages) => {
                    self.storage = storage;
                    self.pages = pages;
                    return std::task::Poll::Ready(Err(DbError::LimitExceeded("database create-catalog work input")));
                }
            }
        }
        self.future.as_mut().map_or(std::task::Poll::Ready(Err(DbError::LimitExceeded("database create-catalog backend future"))), |future| future.as_mut().poll(context))
    }

    pub fn close_one(&mut self) -> Result<bool, DbError> {
        if let Some(pages) = self.pages.as_mut() {
            if pages.close_step()?.is_some() {
                return Ok(true);
            }
            if pages.terminal_is_empty() {
                self.pages.take();
                return Ok(true);
            }
            return Ok(false);
        }
        Ok(self.storage.take().is_some() || self.future.take().is_some())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.future.is_none() && self.storage.is_none() && self.pages.is_none()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn controlled(future: DatabaseCreateCatalogBackendFuture, storage_identity: usize, page_identity: u64) -> Self {
        Self { future: Some(future), storage: None, pages: None, expected: EpochFence::INITIAL, storage_identity, page_identity: Some(page_identity) }
    }
}

pub(super) struct DatabaseCreateCatalogEncodeCursor {
    pub entry: usize,
    pub byte: usize,
    pub phase: u8,
    pub pending: [u8; 32],
    pending_len: u8,
    pending_cursor: u8,
}

impl DatabaseCreateCatalogEncodeCursor {
    pub fn new() -> Self {
        Self { entry: 0, byte: 0, phase: 0, pending: [0; 32], pending_len: 0, pending_cursor: 0 }
    }

    fn set_pending(&mut self, source: &[u8]) -> Result<(), DbError> {
        let target = self.pending.get_mut(..source.len()).ok_or(DbError::LimitExceeded("database create-catalog encode fragment"))?;
        target.copy_from_slice(source);
        self.pending_len = source.len() as u8;
        self.pending_cursor = 0;
        Ok(())
    }

    fn flush_one(&mut self, writer: &mut db_io_core::DbIoPageWriter) -> Result<(), DbError> {
        let start = usize::from(self.pending_cursor);
        let end = usize::from(self.pending_len);
        let written = writer.write_fragment(&self.pending[start..end])?;
        if written == 0 && start != end {
            return Err(DbError::LimitExceeded("database create-catalog writer progress"));
        }
        self.pending_cursor = self.pending_cursor.checked_add(written as u8).ok_or(DbError::LimitExceeded("database create-catalog encode cursor"))?;
        if self.pending_cursor == self.pending_len {
            self.pending_cursor = 0;
            self.pending_len = 0;
        }
        Ok(())
    }

    pub fn step(&mut self, entries: &[CatalogEntry], writer: &mut db_io_core::DbIoPageWriter) -> Result<bool, DbError> {
        if self.pending_cursor < self.pending_len {
            self.flush_one(writer)?;
            return Ok(false);
        }
        match self.phase {
            0 => {
                self.set_pending(b"[")?;
                self.phase = 1;
            }
            1 if self.entry == entries.len() => {
                self.set_pending(b"]")?;
                self.phase = 7;
            }
            1 => {
                self.set_pending(if self.entry == 0 { b"{\"document\":\"" } else { b",{\"document\":\"" })?;
                self.byte = 0;
                self.phase = 2;
            }
            2 => {
                let source = entries[self.entry].document.0.as_bytes();
                if self.byte == source.len() {
                    self.set_pending(b"\",\"created_at_ms\":")?;
                    self.phase = 4;
                } else {
                    let byte = source[self.byte];
                    self.byte += 1;
                    let mut escape = [0u8; 6];
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    let encoded: &[u8] = match byte {
                        b'"' => b"\\\"",
                        b'\\' => b"\\\\",
                        0x08 => b"\\b",
                        0x0c => b"\\f",
                        b'\n' => b"\\n",
                        b'\r' => b"\\r",
                        b'\t' => b"\\t",
                        value @ 0x00..=0x1f => {
                            escape.copy_from_slice(&[b'\\', b'u', b'0', b'0', HEX[(value >> 4) as usize], HEX[(value & 0x0f) as usize]]);
                            &escape
                        }
                        _ => std::slice::from_ref(&byte),
                    };
                    self.set_pending(encoded)?;
                }
            }
            4 => {
                let mut decimal = [0u8; 20];
                self.set_pending(decimal_u64(entries[self.entry].created_at_ms, &mut decimal))?;
                self.phase = 5;
            }
            5 => {
                self.set_pending(b"}")?;
                self.entry += 1;
                self.phase = 1;
            }
            7 => return Ok(true),
            _ => return Err(DbError::LimitExceeded("database create-catalog encode phase")),
        }
        self.flush_one(writer)?;
        Ok(false)
    }
}

pub(super) struct DatabaseCreateCatalogCursor {
    pub base: Option<Arc<Vec<CatalogEntry>>>,
    pub(super) backing: DatabaseCreateCatalogBackingLedger,
    pub(super) base_epoch: EpochFence,
    pub(super) base_revision: u64,
    pub(super) base_identity: usize,
    pub created_at_ms: u64,
    pub encoded_len: usize,
    pub(super) scan_entry: usize,
    pub(super) scan_byte: usize,
    pub(super) scan_started: bool,
    pub(super) candidate: Option<Vec<CatalogEntry>>,
    pub(super) clone_entry: usize,
    pub(super) clone_byte: usize,
    pub(super) clone_text: Option<String>,
    pub snapshot: Option<Arc<Vec<CatalogEntry>>>,
    pub(super) encode: DatabaseCreateCatalogEncodeCursor,
    pub writer: Option<db_io_core::DbIoPageWriter>,
    pub pages: Option<db_io_core::DbIoPages>,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DatabaseCreateCatalogPhase {
    Scan,
    Reserve,
    Clone,
    Snapshot,
    Encode,
    Seal,
    Claim,
    Handoff,
    Poll,
    CloseWork,
    Revalidate,
    Retire,
    Publish,
    Terminal,
}

impl DatabaseCreateCatalogPhase {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Scan,
            1 => Self::Reserve,
            2 => Self::Clone,
            3 => Self::Snapshot,
            4 => Self::Encode,
            5 => Self::Seal,
            6 => Self::Claim,
            7 => Self::Handoff,
            8 => Self::Poll,
            9 => Self::CloseWork,
            10 => Self::Revalidate,
            11 => Self::Retire,
            12 => Self::Publish,
            _ => Self::Terminal,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DatabaseCreateCatalogDriverAuthority {
    Idle,
    Queued,
    Driving,
    Retry,
}

pub(super) struct DatabaseCreateCatalogState {
    pub pool: Arc<WorkerPool>,
    pub pool_use: Mutex<Option<Arc<WorkerPoolUse>>>,
    pub catalog: Arc<Mutex<CatalogState>>,
    pub slot: usize,
    pub generation: u64,
    pub admission: Mutex<Option<DatabaseCreateCatalogAdmission>>,
    pub storage: Mutex<Option<Arc<db_storage::DbBackend>>>,
    pub document: Mutex<Option<protocol::ArtifactId>>,
    pub cursor: Mutex<DatabaseCreateCatalogCursor>,
    pub work: Mutex<Option<DatabaseCreateCatalogWork>>,
    pub poll_work: Mutex<Option<DatabaseCreateCatalogWork>>,
    pub terminal_work: Mutex<Option<DatabaseCreateCatalogWork>>,
    pub outcome: Mutex<Option<Result<EpochFence, DbError>>>,
    pub completion: Mutex<Option<Result<DatabaseCreateCatalogResult, DbError>>>,
    pub terminal_completion: Mutex<Option<Result<DatabaseCreateCatalogResult, DbError>>>,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<semio_framework_async::Job>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub driver_authority: std::sync::atomic::AtomicU8,
    pub polling: std::sync::atomic::AtomicBool,
    pub wake_requested: std::sync::atomic::AtomicBool,
    pub cancelled: std::sync::atomic::AtomicBool,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub closing: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pub(super) pending_owned: std::sync::atomic::AtomicBool,
    pub terminal_checked_out: std::sync::atomic::AtomicBool,
    pub(super) retry_closing: std::sync::atomic::AtomicBool,
    pub(super) callback_close_armed: std::sync::atomic::AtomicBool,
    pub(super) catalog_contention_armed: std::sync::atomic::AtomicBool,
    pub phase: std::sync::atomic::AtomicU8,
    pub progress: std::sync::atomic::AtomicU8,
    pub deadline_ms: std::sync::atomic::AtomicU64,
    pub(super) opportunities: std::sync::atomic::AtomicU64,
    #[cfg(any(test, feature = "testkit"))]
    pub active_drivers: std::sync::atomic::AtomicUsize,
    #[cfg(any(test, feature = "testkit"))]
    pub max_active_drivers: std::sync::atomic::AtomicUsize,
    #[cfg(any(test, feature = "testkit"))]
    pub poll_worker_thread: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    pub(super) spent_submission_attempts: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) backend_polls: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) callback_worker_thread: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    pub(super) terminal_job_retirements: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) controlled_capacity_overage: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) controlled_driver_hook: Mutex<Option<Arc<dyn Fn(DatabaseCreateCatalogPhase) + Send + Sync>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_publication_before_waker_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

pub(super) struct DatabaseCreateCatalogWake {
    pub state: std::sync::Weak<DatabaseCreateCatalogState>,
    pub generation: u64,
}

pub fn database_create_catalog_registry() -> &'static Mutex<[Option<Arc<DatabaseCreateCatalogState>>; DATABASE_CREATE_CATALOG_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<Mutex<[Option<Arc<DatabaseCreateCatalogState>>; DATABASE_CREATE_CATALOG_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCreateCatalogWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let Some(state) = self.state.upgrade() else { return };
        if state.generation != self.generation || !state.is_current() {
            return;
        }
        state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        state.schedule();
    }
}
//#endregion 🔖️CreateDocumentCatalogCas
