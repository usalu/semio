//! 🗄️ `db_engine` — the `Database` supervisor and catalog actor: the crate that assembles every
//! other `db_*` crate into the stable, contract-frozen `Database`/`ArtifactHandle` API
//! (`Database::{open, open_at, create_document, document, catalog, health, shutdown}`;
//! `ArtifactHandle::{submit, query, subscribe, frontier, preview, history, snapshot_now}`).
//! Frozen contract: `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_engine` row + "Stable API" block).
//!
//! 🎯️ Design choice (compatibility surface): `db_artifact` (a concurrent sibling session) commits
//! explicitly, in its own module doc, to keeping the `AuthzHook`/`AllowAll` seam and its local
//! `ConflictRecord{command_id, conflicting_with, path}` shape byte-for-byte stable specifically
//! because THIS crate constructs every one of those verbatim. `SubmitOptions{durability, policy}`
//! and `CommandReceipt{.., messages}` both gained a field under
//! `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C9 — every construction site below
//! and `to_engine_receipt`'s field-for-field bridge were updated in lockstep (never a `..Default`
//! spread over a frozen-shape struct, so a future field addition here fails loud, not silent).
//!
//! 🎯️ Design choice (scope): per this wave's instructions, this crate makes `Database::open_at`
//! (zero-touch `FsStorage`) and a full submit → durable → query round trip over a REAL
//! `db_artifact::ArtifactAuthority` genuinely work end to end (see `//#region 🧪️Tests`), composing
//! the guaranteed-complete `db_state`/`db_wal`/`db_storage`/`db_artifact` crates against their real,
//! current APIs throughout. `db_cluster` is still an unimplemented stub upstream of this wave (its
//! `lib.rs` declares no public items at all) — nothing in this crate can call into it yet; every
//! cluster-shaped concern (sharding, ownership leases, quorum durability, split-brain repair) is
//! deferred wholesale, documented here rather than faked. `db_compact`/`db_sync`/`db_security`/
//! `db_observe` ARE genuinely wired, but narrowly: `Database::compact_document` drives a real
//! `db_compact::Compactor` pass, `Database::hello` drives `db_sync::handle_hello` for the wire-v2
//! handshake (no transport of its own — that is CW5/CW6's job), `SecurityAuthzHook` wraps a real
//! `db_security::SecurityGate` as an optional `AuthzHook`, and `Database::open`/`open_at` wire a
//! real `db_observe::StructuredSink`/`HealthRegistry` pair by default. `ArtifactHandle::preview`/
//! `subscribe` return `DbError::Unimplemented` (not a panic, not a fake success): `db_artifact`'s
//! own `ArtifactAuthority` mailbox (`db/document/rs/lib.rs`'s `ArtifactMessage` enum) only carries
//! `Submit`/`Query`/`Frontier` variants — there is no way to drive its preview/commit-log machinery
//! through the actor boundary without editing `db_artifact` itself, which is out of this crate's
//! ownership this wave. `snapshot_now` is likewise `Unimplemented`: `db_artifact`'s own module doc
//! documents that `DocumentState` materializes purely from the WAL suffix with no full-state
//! enumeration to serialize into a pack snapshot, and `db_snapshot` is not even a direct dependency
//! of this crate per its `Cargo.toml`. `ArtifactHandle::history` replays the WAL through the
//! document authority's retained cursor because its in-memory `commit_log` only contains live
//! submissions from the current process.

use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};

use crate::db_ids::{ActorId, ArtifactId, DbError};
use crate::*;
use db_storage::CatalogStorage as _;
use db_storage::PayloadStorage as _;
use semio_framework_async::{Lane, WorkerPool};

//#region 🔖️Reexports
pub use crate::db_durability::DurabilityClass;
pub use crate::db_policy::{DbCapabilities, DbConfig, Profile};
//#endregion 🔖️Reexports

//#region 🔖️CapabilityOpen
const DATABASE_CAPABILITY_OPEN_SLOTS: usize = 64;
const DATABASE_CAPABILITY_OPEN_ITEMS: u64 = 8;
const DATABASE_CAPABILITY_OPEN_BYTES: u64 = 16 * 1024;
const DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS: u64 = DATABASE_CAPABILITY_OPEN_ITEMS * DATABASE_CAPABILITY_OPEN_SLOTS as u64;
const DATABASE_CAPABILITY_OPEN_TOTAL_BYTES: u64 = DATABASE_CAPABILITY_OPEN_BYTES * DATABASE_CAPABILITY_OPEN_SLOTS as u64;
const DATABASE_CAPABILITY_OPEN_RETRY_MS: u64 = 1;
const DATABASE_CAPABILITY_OPEN_RETRY_LIMIT: u8 = 8;

#[derive(Clone, Copy)]
struct DatabaseCapabilityOpenAdmissionSlot {
    generation: u64,
    items: u64,
    bytes: u64,
    occupied: bool,
}

const EMPTY_DATABASE_CAPABILITY_OPEN_SLOT: DatabaseCapabilityOpenAdmissionSlot = DatabaseCapabilityOpenAdmissionSlot { generation: 0, items: 0, bytes: 0, occupied: false };

struct DatabaseCapabilityOpenAdmissionState {
    slots: [DatabaseCapabilityOpenAdmissionSlot; DATABASE_CAPABILITY_OPEN_SLOTS],
    items: u64,
    bytes: u64,
    next_generation: u64,
}

impl DatabaseCapabilityOpenAdmissionState {
    #[cfg(test)]
    fn empty() -> Self {
        Self { slots: [EMPTY_DATABASE_CAPABILITY_OPEN_SLOT; DATABASE_CAPABILITY_OPEN_SLOTS], items: 0, bytes: 0, next_generation: 1 }
    }

    fn try_claim(&mut self, items: u64, bytes: u64) -> Result<(usize, u64), DbError> {
        if items == 0 || items > DATABASE_CAPABILITY_OPEN_ITEMS {
            return Err(DbError::LimitExceeded("database capability-open item credit"));
        }
        if bytes == 0 || bytes > DATABASE_CAPABILITY_OPEN_BYTES {
            return Err(DbError::LimitExceeded("database capability-open byte credit"));
        }
        let Some(slot) = self.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable("database capability-open item capacity exhausted".to_string()));
        };
        if self.items.checked_add(items).is_none_or(|next| next > DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS) {
            return Err(DbError::Unavailable("database capability-open aggregate item capacity exhausted".to_string()));
        }
        if self.bytes.checked_add(bytes).is_none_or(|next| next > DATABASE_CAPABILITY_OPEN_TOTAL_BYTES) {
            return Err(DbError::Unavailable("database capability-open aggregate byte capacity exhausted".to_string()));
        }
        let generation = self.next_generation;
        let Some(next_generation) = generation.checked_add(1) else {
            return Err(DbError::LimitExceeded("database capability-open generation"));
        };
        self.next_generation = next_generation;
        self.slots[slot] = DatabaseCapabilityOpenAdmissionSlot { generation, items, bytes, occupied: true };
        self.items += items;
        self.bytes += bytes;
        Ok((slot, generation))
    }

    fn release(&mut self, slot: usize, generation: u64, items: u64, bytes: u64) -> bool {
        let Some(entry) = self.slots.get_mut(slot) else {
            return false;
        };
        if !entry.occupied || entry.generation != generation || entry.items != items || entry.bytes != bytes {
            return false;
        }
        *entry = EMPTY_DATABASE_CAPABILITY_OPEN_SLOT;
        self.items = self.items.checked_sub(items).expect("database capability-open item credit underflow");
        self.bytes = self.bytes.checked_sub(bytes).expect("database capability-open byte credit underflow");
        true
    }
}

static DATABASE_CAPABILITY_OPEN_ADMISSION: std::sync::Mutex<DatabaseCapabilityOpenAdmissionState> =
    std::sync::Mutex::new(DatabaseCapabilityOpenAdmissionState { slots: [EMPTY_DATABASE_CAPABILITY_OPEN_SLOT; DATABASE_CAPABILITY_OPEN_SLOTS], items: 0, bytes: 0, next_generation: 1 });

struct DatabaseCapabilityOpenAdmission {
    slot: usize,
    generation: u64,
    items: u64,
    bytes: u64,
}

impl DatabaseCapabilityOpenAdmission {
    fn try_claim(items: u64, bytes: u64) -> Result<Self, DbError> {
        let mut state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let (slot, generation) = state.try_claim(items, bytes)?;
        Ok(Self { slot, generation, items, bytes })
    }

    fn is_current(&self) -> bool {
        let state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.slots.get(self.slot).is_some_and(|entry| entry.occupied && entry.generation == self.generation && entry.items == self.items && entry.bytes == self.bytes)
    }
}

impl Drop for DatabaseCapabilityOpenAdmission {
    fn drop(&mut self) {
        let mut state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.release(self.slot, self.generation, self.items, self.bytes);
    }
}

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
    storage: Arc<db_storage::DbBackend>,
    capabilities: db_storage::StorageCapabilities,
}

impl DatabaseCapabilityOpenResult {
    pub fn into_parts(self) -> (Arc<db_storage::DbBackend>, db_storage::StorageCapabilities) {
        (self.storage, self.capabilities)
    }
}

/// ↩️ Admission rejection that returns the exact storage owner unchanged.
pub struct DatabaseCapabilityOpenRejected {
    error: Option<DbError>,
    storage: Option<Arc<db_storage::DbBackend>>,
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

type DatabaseCapabilityOpenBackendFuture = std::pin::Pin<Box<dyn Future<Output = DatabaseCapabilityOpenResult> + Send + 'static>>;

struct DatabaseCapabilityOpenWork {
    future: Option<DatabaseCapabilityOpenBackendFuture>,
    #[cfg(test)]
    storage_identity: usize,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCapabilityOpenPhase {
    Handoff,
    Poll,
    RetainWork,
    DrainWork,
    ReleaseWork,
    RetainResult,
    Publish,
    Terminal,
}

impl DatabaseCapabilityOpenPhase {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Handoff,
            1 => Self::Poll,
            2 => Self::RetainWork,
            3 => Self::DrainWork,
            4 => Self::ReleaseWork,
            5 => Self::RetainResult,
            6 => Self::Publish,
            _ => Self::Terminal,
        }
    }
}

impl DatabaseCapabilityOpenWork {
    fn new(storage: Arc<db_storage::DbBackend>) -> Self {
        #[cfg(test)]
        let storage_identity = Arc::as_ptr(&storage) as usize;
        let future = Box::pin(async move {
            let capabilities = storage.capabilities().await;
            DatabaseCapabilityOpenResult { storage, capabilities }
        });
        Self {
            future: Some(future),
            #[cfg(test)]
            storage_identity,
        }
    }

    fn poll(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<DatabaseCapabilityOpenResult> {
        self.future.as_mut().map_or(std::task::Poll::Pending, |future| future.as_mut().poll(context))
    }

    fn close_step(&mut self) -> bool {
        self.future.take().is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        self.future.is_none()
    }

    #[cfg(test)]
    fn controlled(future: DatabaseCapabilityOpenBackendFuture, storage_identity: usize) -> Self {
        Self { future: Some(future), storage_identity }
    }
}

struct DatabaseCapabilityOpenState {
    pool: Arc<WorkerPool>,
    slot: usize,
    generation: u64,
    admission: std::sync::Mutex<Option<DatabaseCapabilityOpenAdmission>>,
    work: std::sync::Mutex<Option<DatabaseCapabilityOpenWork>>,
    poll_work: std::sync::Mutex<Option<DatabaseCapabilityOpenWork>>,
    staged_result: std::sync::Mutex<Option<DatabaseCapabilityOpenResult>>,
    terminal_error: std::sync::Mutex<Option<(DbError, DatabaseCapabilityOpenProgress)>>,
    completion: std::sync::Mutex<Option<Result<DatabaseCapabilityOpenResult, DbError>>>,
    terminal_work: std::sync::Mutex<Option<DatabaseCapabilityOpenWork>>,
    terminal_result: std::sync::Mutex<Option<Result<DatabaseCapabilityOpenResult, DbError>>>,
    terminal_result_checked_out: std::sync::atomic::AtomicBool,
    terminal_completion: std::sync::Mutex<Option<Result<DatabaseCapabilityOpenResult, DbError>>>,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    terminal_job: std::sync::Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
    retry_armed: std::sync::atomic::AtomicBool,
    retry_generation: std::sync::atomic::AtomicU64,
    scheduled: std::sync::atomic::AtomicBool,
    polling: std::sync::atomic::AtomicBool,
    wake_requested: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    abandoned: std::sync::atomic::AtomicBool,
    finished: std::sync::atomic::AtomicBool,
    terminal_checked_out: std::sync::atomic::AtomicBool,
    phase: std::sync::atomic::AtomicU8,
    progress: std::sync::atomic::AtomicU8,
    #[cfg(test)]
    poll_publication_hook: std::sync::Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(test)]
    controlled_submit_hook: std::sync::Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
}

struct DatabaseCapabilityOpenWake {
    state: std::sync::Weak<DatabaseCapabilityOpenState>,
    generation: u64,
}

fn database_capability_open_registry() -> &'static std::sync::Mutex<[Option<Arc<DatabaseCapabilityOpenState>>; DATABASE_CAPABILITY_OPEN_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<[Option<Arc<DatabaseCapabilityOpenState>>; DATABASE_CAPABILITY_OPEN_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCapabilityOpenWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let Some(state) = self.state.upgrade() else {
            return;
        };
        if state.generation != self.generation || !state.is_current() || state.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        if !state.polling.load(std::sync::atomic::Ordering::Acquire) {
            state.schedule();
        }
    }
}

impl DatabaseCapabilityOpenState {
    fn phase(&self) -> DatabaseCapabilityOpenPhase {
        DatabaseCapabilityOpenPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    fn set_phase(&self, phase: DatabaseCapabilityOpenPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
    }

    fn set_progress(&self, progress: DatabaseCapabilityOpenProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCapabilityOpenAdmission::is_current)
    }

    fn wake_waiter(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    fn complete(&self, result: Result<DatabaseCapabilityOpenResult, DbError>, progress: DatabaseCapabilityOpenProgress) {
        self.set_progress(progress);
        let mut completion = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            drop(completion);
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *completion = Some(result);
            drop(completion);
            self.wake_waiter();
        }
    }

    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        if !self.is_current() {
            self.scheduled.store(false, Ordering::Release);
            self.stage_terminal(DbError::Unavailable("database capability-open generation became stale".to_string()), DatabaseCapabilityOpenProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            self.scheduled.store(false, Ordering::Release);
            self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            return;
        }
        self.set_progress(DatabaseCapabilityOpenProgress::Scheduled);
        self.submit_drive_job();
    }

    fn schedule_cleanup(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.submit_drive_job();
    }

    fn submit_drive_job(self: &Arc<Self>) {
        let state = self.clone();
        let generation = self.generation;
        let job: semio_framework_async::Job = Box::new(move || state.drive_one(generation));
        #[cfg(test)]
        let job = if let Some(submit) = self.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
            match submit(job) {
                Ok(()) => return,
                Err(job) => job,
            }
        } else {
            job
        };
        self.submit_exact(job, 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                match error.kind() {
                    semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated if attempt < DATABASE_CAPABILITY_OPEN_RETRY_LIMIT => {
                        *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                        self.arm_retry();
                    }
                    kind => {
                        *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, error.into_job()));
                        self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
                        self.complete(Err(DbError::Unavailable(format!("database capability-open WorkerPool submission failed: {kind:?}"))), DatabaseCapabilityOpenProgress::Fault);
                    }
                }
            }
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.advance_retry_generation_once();
    }

    fn advance_retry_generation_once(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let current = self.retry_generation.load(Ordering::Acquire);
        self.advance_retry_generation_observed_once(current);
    }

    fn advance_retry_generation_observed_once(self: &Arc<Self>, current: u64) {
        use std::sync::atomic::Ordering;
        let Some(generation) = current.checked_add(1) else {
            self.retry_armed.store(false, Ordering::Release);
            self.set_progress(DatabaseCapabilityOpenProgress::Fault);
            self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
            self.complete(Err(DbError::LimitExceeded("database capability-open retry generation")), DatabaseCapabilityOpenProgress::Fault);
            return;
        };
        if let Err(observed) = self.retry_generation.compare_exchange(current, generation, Ordering::AcqRel, Ordering::Acquire) {
            let state = self.clone();
            self.pool.callback_at(self.pool.now_ms().saturating_add(DATABASE_CAPABILITY_OPEN_RETRY_MS), move || {
                if state.retry_armed.load(Ordering::Acquire) && observed == state.retry_generation.load(Ordering::Acquire) {
                    state.advance_retry_generation_observed_once(observed);
                }
            });
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(DATABASE_CAPABILITY_OPEN_RETRY_MS), move || {
            if generation != state.retry_generation.load(Ordering::Acquire) {
                return;
            }
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if state.cancelled.load(Ordering::Acquire) || !state.is_current() {
                    *state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                    state.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
                } else {
                    state.scheduled.store(true, Ordering::Release);
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.generation {
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if !self.is_current() && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCapabilityOpenPhase::Terminal {
            self.stage_terminal(DbError::Unavailable("database capability-open generation became stale".to_string()), DatabaseCapabilityOpenProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCapabilityOpenPhase::Terminal {
            self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            return;
        }
        match self.phase() {
            DatabaseCapabilityOpenPhase::Handoff => {
                let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
                    self.stage_terminal(DbError::Unavailable("database capability-open handoff owner missing".to_string()), DatabaseCapabilityOpenProgress::Fault);
                    return;
                };
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.set_phase(DatabaseCapabilityOpenPhase::Poll);
                self.schedule();
            }
            DatabaseCapabilityOpenPhase::Poll => self.poll_backend_once(generation),
            DatabaseCapabilityOpenPhase::RetainWork => {
                let work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
                if let Some(work) = work {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                }
                self.set_phase(DatabaseCapabilityOpenPhase::DrainWork);
                self.schedule_cleanup();
            }
            DatabaseCapabilityOpenPhase::DrainWork => {
                let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let progressed = terminal.as_mut().is_some_and(DatabaseCapabilityOpenWork::close_step);
                let empty = terminal.as_ref().is_some_and(DatabaseCapabilityOpenWork::terminal_is_empty);
                drop(terminal);
                if progressed || empty {
                    self.set_phase(DatabaseCapabilityOpenPhase::ReleaseWork);
                    self.schedule_cleanup();
                } else {
                    self.stage_terminal(DbError::Unavailable("database capability-open retained work failed to release".to_string()), DatabaseCapabilityOpenProgress::Fault);
                }
            }
            DatabaseCapabilityOpenPhase::ReleaseWork => {
                let terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                if terminal.as_ref().is_some_and(DatabaseCapabilityOpenWork::terminal_is_empty) {
                    self.set_phase(if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() && self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
                        DatabaseCapabilityOpenPhase::RetainResult
                    } else {
                        DatabaseCapabilityOpenPhase::Publish
                    });
                    self.schedule_cleanup();
                } else {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = terminal;
                    self.stage_terminal(DbError::Unavailable("database capability-open terminal work witness failed".to_string()), DatabaseCapabilityOpenProgress::Fault);
                }
            }
            DatabaseCapabilityOpenPhase::RetainResult => {
                if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
                }
                self.set_phase(DatabaseCapabilityOpenPhase::Publish);
                self.schedule_cleanup();
            }
            DatabaseCapabilityOpenPhase::Publish => self.publish_staged(),
            DatabaseCapabilityOpenPhase::Terminal => {}
        }
    }

    fn publish_poll_terminal(&self, error: DbError, progress: DatabaseCapabilityOpenProgress) {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        self.set_progress(progress);
        let mut terminal_error = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal_error.is_none() {
            *terminal_error = Some((error, progress));
        }
        drop(terminal_error);
        self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
    }

    fn poll_terminal_if_cancelled_or_stale(&self) -> bool {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            self.publish_poll_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            true
        } else if !self.is_current() {
            self.publish_poll_terminal(DbError::Unavailable("database capability-open generation became stale during polling".to_string()), DatabaseCapabilityOpenProgress::Fault);
            true
        } else {
            false
        }
    }

    fn release_terminal_poll(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.polling.store(false, Ordering::Release);
        self.wake_requested.store(false, Ordering::Release);
        self.schedule_cleanup();
    }

    fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        let Some(mut work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            self.stage_terminal(DbError::Unavailable("database capability-open poll owner missing".to_string()), DatabaseCapabilityOpenProgress::Fault);
            return;
        };
        self.polling.store(true, Ordering::Release);
        self.set_progress(DatabaseCapabilityOpenProgress::Polling);
        let wake = std::task::Waker::from(Arc::new(DatabaseCapabilityOpenWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        #[cfg(test)]
        if let Some(hook) = self.poll_publication_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook();
        }
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                if self.poll_terminal_if_cancelled_or_stale() {
                    self.release_terminal_poll();
                } else {
                    self.set_progress(DatabaseCapabilityOpenProgress::Pending);
                    self.polling.store(false, Ordering::Release);
                    if self.wake_requested.swap(false, Ordering::AcqRel) {
                        self.schedule();
                    }
                }
            }
            Ok(std::task::Poll::Ready(output)) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(output);
                self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
                if self.poll_terminal_if_cancelled_or_stale() {
                    self.release_terminal_poll();
                } else {
                    self.polling.store(false, Ordering::Release);
                    self.wake_requested.store(false, Ordering::Release);
                    self.schedule_cleanup();
                }
            }
            Err(_) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.publish_poll_terminal(DbError::Unavailable("database capability-open backend poll panicked".to_string()), DatabaseCapabilityOpenProgress::Fault);
                self.release_terminal_poll();
            }
        }
    }

    fn stage_terminal(self: &Arc<Self>, error: DbError, progress: DatabaseCapabilityOpenProgress) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(progress);
        {
            let mut terminal_error = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal_error.is_none() {
                *terminal_error = Some((error, progress));
            }
        }
        let has_work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() || self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let has_terminal_work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let phase = if has_work {
            DatabaseCapabilityOpenPhase::RetainWork
        } else if has_terminal_work {
            match self.phase() {
                DatabaseCapabilityOpenPhase::ReleaseWork => DatabaseCapabilityOpenPhase::ReleaseWork,
                _ => DatabaseCapabilityOpenPhase::DrainWork,
            }
        } else if self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            DatabaseCapabilityOpenPhase::RetainResult
        } else {
            DatabaseCapabilityOpenPhase::Publish
        };
        self.set_phase(phase);
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.schedule_cleanup();
        } else {
            self.complete(Err(DbError::Closed), progress);
        }
    }

    fn publish_staged(self: &Arc<Self>) {
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
                return;
            }
            if !self.is_current() {
                self.stage_terminal(DbError::Unavailable("database capability-open generation became stale before publication".to_string()), DatabaseCapabilityOpenProgress::Fault);
                return;
            }
        }
        if let Some((error, progress)) = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.complete(Err(error), progress);
        } else if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.complete(Ok(result), DatabaseCapabilityOpenProgress::Completed);
        } else {
            self.complete(Err(DbError::Unavailable("database capability-open publication owner missing".to_string())), DatabaseCapabilityOpenProgress::Fault);
        }
        self.set_phase(DatabaseCapabilityOpenPhase::Terminal);
    }

    fn roots_are_empty(&self) -> bool {
        self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.terminal_result_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    fn close_step(self: &Arc<Self>) -> DatabaseCapabilityOpenCloseStep {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.retry_armed.load(Ordering::Acquire) {
            let current = self.retry_generation.load(Ordering::Acquire);
            if let Some(next) = current.checked_add(1) {
                self.retry_generation.store(next, Ordering::Release);
                self.retry_armed.store(false, Ordering::Release);
                return DatabaseCapabilityOpenCloseStep::Progress;
            }
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if self.scheduled.load(Ordering::Acquire) || self.polling.load(Ordering::Acquire) {
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        {
            let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(work) = terminal.as_mut() {
                if work.close_step() {
                    return DatabaseCapabilityOpenCloseStep::Progress;
                }
                if work.terminal_is_empty() {
                    terminal.take();
                    return DatabaseCapabilityOpenCloseStep::Progress;
                }
            }
        }
        if self.terminal_result_checked_out.load(Ordering::Acquire) {
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            self.finished.store(true, Ordering::Release);
            let mut registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
                registry[self.slot] = None;
            }
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.roots_are_empty() {
            self.finished.store(true, Ordering::Release);
            DatabaseCapabilityOpenCloseStep::Complete
        } else {
            DatabaseCapabilityOpenCloseStep::Blocked
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire) && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.roots_are_empty()
    }

    fn release_success(&self) {
        if !self.roots_are_empty() || self.scheduled.load(std::sync::atomic::Ordering::Acquire) || self.polling.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.finished.store(true, std::sync::atomic::Ordering::Release);
        let mut registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
            registry[self.slot] = None;
        }
    }

    #[cfg(test)]
    fn retained_owner_count(&self) -> usize {
        usize::from(self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
    }
}

/// 🔭️ Retained capability-probe future; backend polling occurs only on the process I/O lane.
pub struct DatabaseCapabilityOpenFuture {
    state: Arc<DatabaseCapabilityOpenState>,
    resolved: bool,
}

impl DatabaseCapabilityOpenFuture {
    pub fn try_submit(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>) -> Result<Self, DatabaseCapabilityOpenRejected> {
        Self::try_prepare(pool, storage, true)
    }

    fn try_prepare(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, schedule: bool) -> Result<Self, DatabaseCapabilityOpenRejected> {
        let admission = match DatabaseCapabilityOpenAdmission::try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCapabilityOpenRejected { error: Some(error), storage: Some(storage) }),
        };
        {
            let registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if registry[admission.slot].is_some() {
                return Err(DatabaseCapabilityOpenRejected { error: Some(DbError::Unavailable("database capability-open terminal slot remained occupied".to_string())), storage: Some(storage) });
            }
        }
        let slot = admission.slot;
        let generation = admission.generation;
        let state = Arc::new(DatabaseCapabilityOpenState {
            pool,
            slot,
            generation,
            admission: std::sync::Mutex::new(Some(admission)),
            work: std::sync::Mutex::new(Some(DatabaseCapabilityOpenWork::new(storage))),
            poll_work: std::sync::Mutex::new(None),
            staged_result: std::sync::Mutex::new(None),
            terminal_error: std::sync::Mutex::new(None),
            completion: std::sync::Mutex::new(None),
            terminal_work: std::sync::Mutex::new(None),
            terminal_result: std::sync::Mutex::new(None),
            terminal_result_checked_out: std::sync::atomic::AtomicBool::new(false),
            terminal_completion: std::sync::Mutex::new(None),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            waker: std::sync::Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            polling: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            terminal_checked_out: std::sync::atomic::AtomicBool::new(false),
            phase: std::sync::atomic::AtomicU8::new(DatabaseCapabilityOpenPhase::Handoff as u8),
            progress: std::sync::atomic::AtomicU8::new(DatabaseCapabilityOpenProgress::Admitted as u8),
            #[cfg(test)]
            poll_publication_hook: std::sync::Mutex::new(None),
            #[cfg(test)]
            controlled_submit_hook: std::sync::Mutex::new(None),
        });
        database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        if schedule {
            state.schedule();
        }
        Ok(Self { state, resolved: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> DatabaseCapabilityOpenProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => DatabaseCapabilityOpenProgress::Admitted,
            1 => DatabaseCapabilityOpenProgress::Scheduled,
            2 => DatabaseCapabilityOpenProgress::Polling,
            3 => DatabaseCapabilityOpenProgress::Pending,
            4 => DatabaseCapabilityOpenProgress::Completed,
            5 => DatabaseCapabilityOpenProgress::Cancelled,
            _ => DatabaseCapabilityOpenProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        if !self.state.scheduled.load(std::sync::atomic::Ordering::Acquire) && !self.state.polling.load(std::sync::atomic::Ordering::Acquire) {
            self.state.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
        }
    }

    #[cfg(test)]
    fn retained_storage_identity(&self) -> Option<usize> {
        self.state
            .work
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
            .map(|work| work.storage_identity)
            .or_else(|| self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|work| work.storage_identity))
            .or_else(|| self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|work| work.storage_identity))
    }
}

impl Future for DatabaseCapabilityOpenFuture {
    type Output = Result<DatabaseCapabilityOpenResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            if result.is_err() {
                self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            }
            self.state.release_success();
            return std::task::Poll::Ready(result);
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseCapabilityOpenFuture {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        if self.resolved {
            return;
        }
        let mut completion = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        self.state.abandoned.store(true, Ordering::Release);
        self.state.cancelled.store(true, Ordering::Release);
        if let Some(result) = completion.take() {
            *self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        drop(completion);
        if !self.state.scheduled.load(Ordering::Acquire) && !self.state.polling.load(Ordering::Acquire) {
            self.state.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
        }
    }
}

/// 🧯️ Public retained authority for cancellation, saturation, fault, and abandoned-open cleanup.
pub struct DatabaseCapabilityOpenTerminalHandle {
    state: Arc<DatabaseCapabilityOpenState>,
}

impl DatabaseCapabilityOpenTerminalHandle {
    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn close_step(&self) -> DatabaseCapabilityOpenCloseStep {
        self.state.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty()
    }

    pub fn take_result(&self) -> Option<DatabaseCapabilityOpenTerminalResult> {
        use std::sync::atomic::Ordering;
        if self.state.terminal_result_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        if self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.terminal_result_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(DatabaseCapabilityOpenTerminalResult { state: self.state.clone(), checked_out: true })
    }

    pub fn resume(self) -> Result<DatabaseCapabilityOpenFuture, Self> {
        use std::sync::atomic::Ordering;
        if self.state.scheduled.load(Ordering::Acquire) || self.state.polling.load(Ordering::Acquire) || self.state.finished.load(Ordering::Acquire) {
            return Err(self);
        }
        let mut job = self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|(_, job)| (job, 0));
        if job.is_none() {
            let retry = self.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some(retry) = retry {
                let generation = self.state.retry_generation.load(Ordering::Acquire);
                let Some(next) = generation.checked_add(1) else {
                    *self.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(retry);
                    return Err(self);
                };
                self.state.retry_generation.store(next, Ordering::Release);
                self.state.retry_armed.store(false, Ordering::Release);
                job = Some(retry);
            }
        }
        let work = if job.is_none() { self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() } else { None };
        let result = if job.is_none() && work.is_none() { self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() } else { None };
        if let Some((job, attempt)) = job {
            self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.set_phase(if self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() { DatabaseCapabilityOpenPhase::Poll } else { DatabaseCapabilityOpenPhase::Handoff });
            self.state.abandoned.store(false, Ordering::Release);
            self.state.cancelled.store(false, Ordering::Release);
            self.state.terminal_checked_out.store(false, Ordering::Release);
            self.state.scheduled.store(true, Ordering::Release);
            self.state.submit_exact(job, attempt);
            return Ok(DatabaseCapabilityOpenFuture { state: self.state.clone(), resolved: false });
        } else if let Some(work) = work {
            *self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.state.set_phase(DatabaseCapabilityOpenPhase::Poll);
        } else if let Some(result) = result {
            *self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            self.state.set_phase(DatabaseCapabilityOpenPhase::Terminal);
        } else if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.set_phase(DatabaseCapabilityOpenPhase::Handoff);
        } else if self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.set_phase(DatabaseCapabilityOpenPhase::Poll);
        } else {
            return Err(self);
        }
        self.state.abandoned.store(false, Ordering::Release);
        self.state.cancelled.store(false, Ordering::Release);
        self.state.terminal_checked_out.store(false, Ordering::Release);
        if self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(DatabaseCapabilityOpenFuture { state: self.state.clone(), resolved: false })
    }
}

/// 📦️ Shallow checkout of one retained capability-open result owner.
pub struct DatabaseCapabilityOpenTerminalResult {
    state: Arc<DatabaseCapabilityOpenState>,
    checked_out: bool,
}

impl DatabaseCapabilityOpenTerminalResult {
    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn take(mut self) -> Option<Result<DatabaseCapabilityOpenResult, DbError>> {
        let result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
        result
    }

    pub fn resume(mut self) -> Result<DatabaseCapabilityOpenFuture, Self> {
        use std::sync::atomic::Ordering;
        if self.state.scheduled.load(Ordering::Acquire) || self.state.polling.load(Ordering::Acquire) || self.state.finished.load(Ordering::Acquire) || self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            return Err(self);
        }
        let Some(result) = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            return Err(self);
        };
        *self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        self.state.set_phase(DatabaseCapabilityOpenPhase::Terminal);
        self.checked_out = false;
        self.state.terminal_result_checked_out.store(false, Ordering::Release);
        Ok(DatabaseCapabilityOpenFuture { state: self.state.clone(), resolved: false })
    }

    pub fn close_step(&mut self) -> DatabaseCapabilityOpenCloseStep {
        if let Some(result) = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
            DatabaseCapabilityOpenCloseStep::Progress
        } else {
            DatabaseCapabilityOpenCloseStep::Complete
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }
}

impl Drop for DatabaseCapabilityOpenTerminalResult {
    fn drop(&mut self) {
        if self.checked_out {
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
    }
}

impl Drop for DatabaseCapabilityOpenTerminalHandle {
    fn drop(&mut self) {
        self.state.terminal_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

/// 🧲️ Takes the exact abandoned capability-open generation without moving its nested owners.
pub fn take_database_capability_open_terminal(generation: u64) -> Option<DatabaseCapabilityOpenTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).find(|state| state.generation == generation && state.abandoned.load(Ordering::Acquire))?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCapabilityOpenTerminalHandle { state })
}

/// 🧲️ Takes the oldest abandoned capability-open authority when its generation was not observed.
pub fn take_next_database_capability_open_terminal() -> Option<DatabaseCapabilityOpenTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).filter(|state| state.abandoned.load(Ordering::Acquire)).min_by_key(|state| state.generation)?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCapabilityOpenTerminalHandle { state })
}
//#endregion 🔖️CapabilityOpen

//#region 🔖️CatalogRootRead
const DATABASE_CATALOG_READ_SLOTS: usize = 64;
const DATABASE_CATALOG_READ_ITEMS: u64 = 8;
const DATABASE_CATALOG_READ_BYTES: u64 = 64 * 1024;
const DATABASE_CATALOG_READ_TOTAL_ITEMS: u64 = DATABASE_CATALOG_READ_ITEMS * DATABASE_CATALOG_READ_SLOTS as u64;
const DATABASE_CATALOG_READ_TOTAL_BYTES: u64 = DATABASE_CATALOG_READ_BYTES * DATABASE_CATALOG_READ_SLOTS as u64;
const DATABASE_CATALOG_READ_RETRY_LIMIT: u8 = 8;

#[derive(Clone, Copy)]
struct DatabaseCatalogReadAdmissionSlot {
    generation: u64,
    occupied: bool,
}

const EMPTY_DATABASE_CATALOG_READ_SLOT: DatabaseCatalogReadAdmissionSlot = DatabaseCatalogReadAdmissionSlot { generation: 0, occupied: false };

struct DatabaseCatalogReadAdmissionState {
    slots: [DatabaseCatalogReadAdmissionSlot; DATABASE_CATALOG_READ_SLOTS],
    items: u64,
    bytes: u64,
    next_generation: u64,
}

impl DatabaseCatalogReadAdmissionState {
    #[cfg(test)]
    fn empty() -> Self {
        Self { slots: [EMPTY_DATABASE_CATALOG_READ_SLOT; DATABASE_CATALOG_READ_SLOTS], items: 0, bytes: 0, next_generation: 1 }
    }

    fn try_claim(&mut self, items: u64, bytes: u64) -> Result<(usize, u64), DbError> {
        if items == 0 || items > DATABASE_CATALOG_READ_ITEMS {
            return Err(DbError::LimitExceeded("database catalog-read item credit"));
        }
        if bytes == 0 || bytes > DATABASE_CATALOG_READ_BYTES {
            return Err(DbError::LimitExceeded("database catalog-read byte credit"));
        }
        let slot = self.slots.iter().position(|entry| !entry.occupied).ok_or_else(|| DbError::Unavailable("database catalog-read slot capacity exhausted".to_string()))?;
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded("database catalog-read aggregate items"))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database catalog-read aggregate bytes"))?;
        if next_items > DATABASE_CATALOG_READ_TOTAL_ITEMS || next_bytes > DATABASE_CATALOG_READ_TOTAL_BYTES {
            return Err(DbError::Unavailable("database catalog-read aggregate capacity exhausted".to_string()));
        }
        let generation = self.next_generation;
        self.next_generation = generation.checked_add(1).ok_or(DbError::LimitExceeded("database catalog-read generation"))?;
        self.slots[slot] = DatabaseCatalogReadAdmissionSlot { generation, occupied: true };
        self.items = next_items;
        self.bytes = next_bytes;
        Ok((slot, generation))
    }

    fn is_current(&self, slot: usize, generation: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
    }

    fn release(&mut self, slot: usize, generation: u64) -> bool {
        if !self.is_current(slot, generation) {
            return false;
        }
        self.slots[slot] = EMPTY_DATABASE_CATALOG_READ_SLOT;
        self.items = self.items.checked_sub(DATABASE_CATALOG_READ_ITEMS).expect("database catalog-read item credit underflow");
        self.bytes = self.bytes.checked_sub(DATABASE_CATALOG_READ_BYTES).expect("database catalog-read byte credit underflow");
        true
    }
}

static DATABASE_CATALOG_READ_ADMISSION: std::sync::Mutex<DatabaseCatalogReadAdmissionState> =
    std::sync::Mutex::new(DatabaseCatalogReadAdmissionState { slots: [EMPTY_DATABASE_CATALOG_READ_SLOT; DATABASE_CATALOG_READ_SLOTS], items: 0, bytes: 0, next_generation: 1 });

struct DatabaseCatalogReadAdmission {
    slot: usize,
    generation: u64,
}

impl DatabaseCatalogReadAdmission {
    fn try_claim() -> Result<Self, DbError> {
        let (slot, generation) = DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES)?;
        Ok(Self { slot, generation })
    }

    fn is_current(&self) -> bool {
        DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation)
    }
}

impl Drop for DatabaseCatalogReadAdmission {
    fn drop(&mut self) {
        DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DatabaseCatalogRootKey(&'static str);

impl DatabaseCatalogRootKey {
    fn root() -> Self {
        Self("catalog-root")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCatalogReadProgress {
    Admitted,
    Scheduled,
    Polling,
    Pending,
    Completed,
    Cancelled,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCatalogReadCloseStep {
    Progress,
    Blocked,
    Complete,
}

pub struct DatabaseCatalogReadResult {
    storage: Arc<db_storage::DbBackend>,
    key: DatabaseCatalogRootKey,
    root: Result<Option<(db_storage::DbIoPages, EpochFence)>, DbError>,
}

impl DatabaseCatalogReadResult {
    pub fn into_parts(self) -> (Arc<db_storage::DbBackend>, DatabaseCatalogRootKey, Result<Option<(db_storage::DbIoPages, EpochFence)>, DbError>) {
        (self.storage, self.key, self.root)
    }
}

pub struct DatabaseCatalogReadRejected {
    error: Option<DbError>,
    storage: Option<Arc<db_storage::DbBackend>>,
    key: Option<DatabaseCatalogRootKey>,
}

impl std::fmt::Debug for DatabaseCatalogReadRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCatalogReadRejected").field("error", &self.error).field("storage", &self.storage.as_ref().map(Arc::as_ptr)).field("key", &self.key).finish()
    }
}

impl DatabaseCatalogReadRejected {
    pub fn retry(mut self, pool: Arc<WorkerPool>) -> Result<DatabaseCatalogReadFuture, Self> {
        let Some(storage) = self.storage.take() else { return Err(self) };
        let Some(key) = self.key.take() else {
            self.storage = Some(storage);
            return Err(self);
        };
        match DatabaseCatalogReadFuture::try_submit(pool, storage, key) {
            Ok(future) => Ok(future),
            Err(mut rejected) => {
                self.error = rejected.error.take();
                self.storage = rejected.storage.take();
                self.key = rejected.key.take();
                Err(self)
            }
        }
    }

    pub fn close_step(&mut self) -> DatabaseCatalogReadCloseStep {
        if let Some(storage) = self.storage.take() {
            drop(storage);
            DatabaseCatalogReadCloseStep::Progress
        } else if let Some(key) = self.key.take() {
            drop(key);
            DatabaseCatalogReadCloseStep::Progress
        } else {
            DatabaseCatalogReadCloseStep::Complete
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.key.is_none()
    }

    pub fn into_error_after_close(mut self) -> Result<DbError, Self> {
        if !self.terminal_is_empty() {
            return Err(self);
        }
        Ok(self.error.take().expect("database catalog-read rejection error missing"))
    }

    pub fn close_and_take_error(self, pool: Arc<WorkerPool>) -> DbError {
        self.mount_close_and_take_error(pool, true).0
    }

    fn mount_close_and_take_error(mut self, pool: Arc<WorkerPool>, schedule: bool) -> (DbError, Arc<DatabaseCatalogReadRejectedClose>) {
        let error = self.error.take().expect("database catalog-read rejection error missing");
        let close = DatabaseCatalogReadRejectedClose::mount(pool, self, schedule);
        (error, close)
    }
}

struct DatabaseCatalogReadRejectedClose {
    pool: Arc<WorkerPool>,
    owner: std::sync::Mutex<Option<DatabaseCatalogReadRejected>>,
    retry_job: std::sync::Mutex<Option<semio_framework_async::Job>>,
    scheduled: std::sync::atomic::AtomicBool,
    finished: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    controlled_submit_hook: std::sync::Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
}

impl DatabaseCatalogReadRejectedClose {
    fn mount(pool: Arc<WorkerPool>, owner: DatabaseCatalogReadRejected, schedule: bool) -> Arc<Self> {
        let state = Arc::new(Self {
            pool,
            owner: std::sync::Mutex::new(Some(owner)),
            retry_job: std::sync::Mutex::new(None),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            controlled_submit_hook: std::sync::Mutex::new(None),
        });
        if schedule {
            state.schedule();
        }
        state
    }

    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.submit_exact(Box::new(move || state.drive_one()));
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job) {
        #[cfg(test)]
        let job = if let Some(submit) = self.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
            match submit(job) {
                Ok(()) => return,
                Err(job) => job,
            }
        } else {
            job
        };
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error.into_job());
                let state = self.clone();
                self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
            }
        }
    }

    fn retry(self: Arc<Self>) {
        let job = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(job) = job {
            self.submit_exact(job);
        }
    }

    fn drive_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.scheduled.store(false, Ordering::Release);
        let owner = self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let Some(mut owner) = owner else {
            self.finished.store(true, Ordering::Release);
            return;
        };
        let step = owner.close_step();
        if step == DatabaseCatalogReadCloseStep::Progress && !owner.terminal_is_empty() {
            *self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
            self.schedule();
        } else {
            debug_assert!(owner.terminal_is_empty());
            self.finished.store(true, Ordering::Release);
        }
    }

    #[cfg(test)]
    fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire)
            && self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
    }
}

type DatabaseCatalogReadBackendFuture = std::pin::Pin<Box<dyn Future<Output = DatabaseCatalogReadResult> + Send + 'static>>;

struct DatabaseCatalogReadWork {
    future: Option<DatabaseCatalogReadBackendFuture>,
    #[cfg(test)]
    storage_identity: usize,
}

impl DatabaseCatalogReadWork {
    fn new(storage: Arc<db_storage::DbBackend>, key: DatabaseCatalogRootKey) -> Self {
        #[cfg(test)]
        let storage_identity = Arc::as_ptr(&storage) as usize;
        let future = Box::pin(async move {
            let root = storage.catalog().await.read_root().await;
            DatabaseCatalogReadResult { storage, key, root }
        });
        Self {
            future: Some(future),
            #[cfg(test)]
            storage_identity,
        }
    }

    fn poll(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<DatabaseCatalogReadResult> {
        self.future.as_mut().map_or(std::task::Poll::Pending, |future| future.as_mut().poll(context))
    }

    fn close_step(&mut self) -> bool {
        self.future.take().is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        self.future.is_none()
    }

    #[cfg(test)]
    fn controlled(future: DatabaseCatalogReadBackendFuture, storage_identity: usize) -> Self {
        Self { future: Some(future), storage_identity }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCatalogReadPhase {
    Handoff,
    Poll,
    RetainWork,
    DrainWork,
    ReleaseWork,
    RetainResult,
    Publish,
    Terminal,
}

impl DatabaseCatalogReadPhase {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Handoff,
            1 => Self::Poll,
            2 => Self::RetainWork,
            3 => Self::DrainWork,
            4 => Self::ReleaseWork,
            5 => Self::RetainResult,
            6 => Self::Publish,
            _ => Self::Terminal,
        }
    }
}

struct DatabaseCatalogReadState {
    pool: Arc<WorkerPool>,
    slot: usize,
    generation: u64,
    admission: std::sync::Mutex<Option<DatabaseCatalogReadAdmission>>,
    work: std::sync::Mutex<Option<DatabaseCatalogReadWork>>,
    poll_work: std::sync::Mutex<Option<DatabaseCatalogReadWork>>,
    staged_result: std::sync::Mutex<Option<DatabaseCatalogReadResult>>,
    terminal_error: std::sync::Mutex<Option<(DbError, DatabaseCatalogReadProgress)>>,
    completion: std::sync::Mutex<Option<Result<DatabaseCatalogReadResult, DbError>>>,
    terminal_work: std::sync::Mutex<Option<DatabaseCatalogReadWork>>,
    terminal_result: std::sync::Mutex<Option<Result<DatabaseCatalogReadResult, DbError>>>,
    terminal_completion: std::sync::Mutex<Option<Result<DatabaseCatalogReadResult, DbError>>>,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    terminal_job: std::sync::Mutex<Option<semio_framework_async::Job>>,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
    retry_armed: std::sync::atomic::AtomicBool,
    scheduled: std::sync::atomic::AtomicBool,
    polling: std::sync::atomic::AtomicBool,
    wake_requested: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    abandoned: std::sync::atomic::AtomicBool,
    finished: std::sync::atomic::AtomicBool,
    terminal_checked_out: std::sync::atomic::AtomicBool,
    terminal_result_checked_out: std::sync::atomic::AtomicBool,
    phase: std::sync::atomic::AtomicU8,
    progress: std::sync::atomic::AtomicU8,
    #[cfg(test)]
    controlled_submit_hook: std::sync::Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
    #[cfg(test)]
    controlled_publication_before_waker_hook: std::sync::Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

struct DatabaseCatalogReadWake {
    state: std::sync::Weak<DatabaseCatalogReadState>,
    generation: u64,
}

fn database_catalog_read_registry() -> &'static std::sync::Mutex<[Option<Arc<DatabaseCatalogReadState>>; DATABASE_CATALOG_READ_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<[Option<Arc<DatabaseCatalogReadState>>; DATABASE_CATALOG_READ_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCatalogReadWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let Some(state) = self.state.upgrade() else { return };
        if state.generation != self.generation || !state.is_current() || state.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        if !state.polling.load(std::sync::atomic::Ordering::Acquire) {
            state.schedule();
        }
    }
}

impl DatabaseCatalogReadState {
    fn phase(&self) -> DatabaseCatalogReadPhase {
        DatabaseCatalogReadPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    fn set_phase(&self, phase: DatabaseCatalogReadPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
    }

    fn set_progress(&self, progress: DatabaseCatalogReadProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCatalogReadAdmission::is_current)
    }

    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        if !self.is_current() {
            self.scheduled.store(false, Ordering::Release);
            self.stage_terminal(DbError::Unavailable("database catalog-read generation stale".to_string()), DatabaseCatalogReadProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            self.scheduled.store(false, Ordering::Release);
            self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
            return;
        }
        self.set_progress(DatabaseCatalogReadProgress::Scheduled);
        self.submit_drive_job();
    }

    fn schedule_cleanup(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.submit_drive_job();
    }

    fn submit_drive_job(self: &Arc<Self>) {
        let state = self.clone();
        let generation = self.generation;
        let job: semio_framework_async::Job = Box::new(move || state.drive_one(generation));
        #[cfg(test)]
        let job = if let Some(submit) = self.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
            match submit(job) {
                Ok(()) => return,
                Err(job) => job,
            }
        } else {
            job
        };
        self.submit_exact(job, 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                if matches!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated) && attempt < DATABASE_CATALOG_READ_RETRY_LIMIT {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                    self.arm_retry();
                } else {
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error.into_job());
                    self.stage_terminal(DbError::Unavailable("database catalog-read WorkerPool submission failed".to_string()), DatabaseCatalogReadProgress::Fault);
                }
            }
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if !state.is_current() {
                    *state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                    state.stage_terminal(DbError::Unavailable("database catalog-read retry generation stale".to_string()), DatabaseCatalogReadProgress::Fault);
                } else if state.cancelled.load(Ordering::Acquire) {
                    *state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                    state.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                } else {
                    state.scheduled.store(true, Ordering::Release);
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.generation {
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if !self.is_current() && self.phase() != DatabaseCatalogReadPhase::Terminal {
            self.stage_terminal(DbError::Unavailable("database catalog-read generation stale".to_string()), DatabaseCatalogReadProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && self.phase() != DatabaseCatalogReadPhase::Terminal {
            self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
            return;
        }
        match self.phase() {
            DatabaseCatalogReadPhase::Handoff => {
                let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
                    self.stage_terminal(DbError::Unavailable("database catalog-read handoff owner missing".to_string()), DatabaseCatalogReadProgress::Fault);
                    return;
                };
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.set_phase(DatabaseCatalogReadPhase::Poll);
                self.schedule();
            }
            DatabaseCatalogReadPhase::Poll => self.poll_backend_once(generation),
            DatabaseCatalogReadPhase::RetainWork => {
                let work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
                if let Some(work) = work {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                }
                self.set_phase(DatabaseCatalogReadPhase::DrainWork);
                self.schedule_cleanup();
            }
            DatabaseCatalogReadPhase::DrainWork => {
                let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let progressed = terminal.as_mut().is_some_and(DatabaseCatalogReadWork::close_step);
                let empty = terminal.as_ref().is_some_and(DatabaseCatalogReadWork::terminal_is_empty);
                drop(terminal);
                if progressed || empty {
                    self.set_phase(DatabaseCatalogReadPhase::ReleaseWork);
                    self.schedule_cleanup();
                } else {
                    self.stage_terminal(DbError::Unavailable("database catalog-read work failed to release".to_string()), DatabaseCatalogReadProgress::Fault);
                }
            }
            DatabaseCatalogReadPhase::ReleaseWork => {
                let terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                if terminal.as_ref().is_some_and(DatabaseCatalogReadWork::terminal_is_empty) {
                    let has_result = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
                    self.set_phase(if has_result { DatabaseCatalogReadPhase::RetainResult } else { DatabaseCatalogReadPhase::Publish });
                    self.schedule_cleanup();
                } else {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = terminal;
                    self.stage_terminal(DbError::Unavailable("database catalog-read work witness failed".to_string()), DatabaseCatalogReadProgress::Fault);
                }
            }
            DatabaseCatalogReadPhase::RetainResult => {
                if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
                    if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
                    }
                }
                self.set_phase(DatabaseCatalogReadPhase::Publish);
                self.schedule_cleanup();
            }
            DatabaseCatalogReadPhase::Publish => self.publish_staged(),
            DatabaseCatalogReadPhase::Terminal => {}
        }
    }

    fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        let Some(mut work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            self.stage_terminal(DbError::Unavailable("database catalog-read poll owner missing".to_string()), DatabaseCatalogReadProgress::Fault);
            return;
        };
        self.polling.store(true, Ordering::Release);
        self.set_progress(DatabaseCatalogReadProgress::Polling);
        let wake = std::task::Waker::from(Arc::new(DatabaseCatalogReadWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                if !self.is_current() {
                    self.publish_terminal(DbError::Unavailable("database catalog-read generation stale during poll".to_string()), DatabaseCatalogReadProgress::Fault);
                    self.release_terminal_poll();
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                    self.release_terminal_poll();
                } else {
                    self.set_progress(DatabaseCatalogReadProgress::Pending);
                    self.polling.store(false, Ordering::Release);
                    if self.wake_requested.swap(false, Ordering::AcqRel) {
                        self.schedule();
                    }
                }
            }
            Ok(std::task::Poll::Ready(result)) => {
                let too_large = result.root.as_ref().ok().and_then(Option::as_ref).is_some_and(|(bytes, _)| bytes.capacity() > DATABASE_CATALOG_READ_BYTES as usize);
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                self.set_phase(DatabaseCatalogReadPhase::RetainWork);
                if too_large {
                    self.publish_terminal(DbError::LimitExceeded("database catalog-read result bytes"), DatabaseCatalogReadProgress::Fault);
                } else if !self.is_current() {
                    self.publish_terminal(DbError::Unavailable("database catalog-read generation stale after result".to_string()), DatabaseCatalogReadProgress::Fault);
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                }
                self.release_terminal_poll();
            }
            Err(_) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.publish_terminal(DbError::Unavailable("database catalog-read backend poll panicked".to_string()), DatabaseCatalogReadProgress::Fault);
                self.release_terminal_poll();
            }
        }
    }

    fn publish_terminal(&self, error: DbError, progress: DatabaseCatalogReadProgress) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(progress);
        let mut terminal = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_none() {
            *terminal = Some((error, progress));
        }
        drop(terminal);
        self.set_phase(DatabaseCatalogReadPhase::RetainWork);
    }

    fn release_terminal_poll(self: &Arc<Self>) {
        self.polling.store(false, std::sync::atomic::Ordering::Release);
        self.wake_requested.store(false, std::sync::atomic::Ordering::Release);
        self.schedule_cleanup();
    }

    fn stage_terminal(self: &Arc<Self>, error: DbError, progress: DatabaseCatalogReadProgress) {
        self.publish_terminal(error, progress);
        self.schedule_cleanup();
    }

    fn publish_staged(self: &Arc<Self>) {
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            if !self.is_current() {
                self.stage_terminal(DbError::Unavailable("database catalog-read generation stale before publication".to_string()), DatabaseCatalogReadProgress::Fault);
                return;
            }
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                return;
            }
        }
        let result = if let Some((error, progress)) = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.set_progress(progress);
            Err(error)
        } else if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.set_progress(DatabaseCatalogReadProgress::Completed);
            Ok(result)
        } else {
            self.set_progress(DatabaseCatalogReadProgress::Fault);
            Err(DbError::Unavailable("database catalog-read publication owner missing".to_string()))
        };
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            self.publish_public_completion(result);
        }
        self.set_phase(DatabaseCatalogReadPhase::Terminal);
    }

    fn publish_public_completion(&self, result: Result<DatabaseCatalogReadResult, DbError>) {
        *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    fn roots_are_empty(&self) -> bool {
        self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    fn close_step(self: &Arc<Self>) -> DatabaseCatalogReadCloseStep {
        use std::sync::atomic::Ordering;
        if self.scheduled.load(Ordering::Acquire) || self.polling.load(Ordering::Acquire) || self.retry_armed.load(Ordering::Acquire) {
            return DatabaseCatalogReadCloseStep::Blocked;
        }
        if self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() || self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        {
            let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(work) = terminal.as_mut() {
                if work.close_step() {
                    return DatabaseCatalogReadCloseStep::Progress;
                }
                if work.terminal_is_empty() {
                    terminal.take();
                    return DatabaseCatalogReadCloseStep::Progress;
                }
            }
        }
        if self.terminal_result_checked_out.load(Ordering::Acquire) {
            return DatabaseCatalogReadCloseStep::Blocked;
        }
        if self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
        {
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(result) = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            self.finished.store(true, Ordering::Release);
            let mut registry = database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
                registry[self.slot] = None;
            }
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.roots_are_empty() {
            self.finished.store(true, Ordering::Release);
            DatabaseCatalogReadCloseStep::Complete
        } else {
            DatabaseCatalogReadCloseStep::Blocked
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire) && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.roots_are_empty()
    }

    fn release_success(&self) {
        if !self.roots_are_empty() || self.scheduled.load(std::sync::atomic::Ordering::Acquire) || self.polling.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.finished.store(true, std::sync::atomic::Ordering::Release);
        let mut registry = database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
            registry[self.slot] = None;
        }
    }
}

pub struct DatabaseCatalogReadFuture {
    state: Arc<DatabaseCatalogReadState>,
    resolved: bool,
}

impl DatabaseCatalogReadFuture {
    pub fn try_submit(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, key: DatabaseCatalogRootKey) -> Result<Self, DatabaseCatalogReadRejected> {
        Self::try_prepare(pool, storage, key, true)
    }

    fn try_prepare(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, key: DatabaseCatalogRootKey, schedule: bool) -> Result<Self, DatabaseCatalogReadRejected> {
        let admission = match DatabaseCatalogReadAdmission::try_claim() {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCatalogReadRejected { error: Some(error), storage: Some(storage), key: Some(key) }),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        if database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot].is_some() {
            return Err(DatabaseCatalogReadRejected { error: Some(DbError::Unavailable("database catalog-read terminal slot occupied".to_string())), storage: Some(storage), key: Some(key) });
        }
        let state = Arc::new(DatabaseCatalogReadState {
            pool,
            slot,
            generation,
            admission: std::sync::Mutex::new(Some(admission)),
            work: std::sync::Mutex::new(Some(DatabaseCatalogReadWork::new(storage, key))),
            poll_work: std::sync::Mutex::new(None),
            staged_result: std::sync::Mutex::new(None),
            terminal_error: std::sync::Mutex::new(None),
            completion: std::sync::Mutex::new(None),
            terminal_work: std::sync::Mutex::new(None),
            terminal_result: std::sync::Mutex::new(None),
            terminal_completion: std::sync::Mutex::new(None),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            waker: std::sync::Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            polling: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            terminal_checked_out: std::sync::atomic::AtomicBool::new(false),
            terminal_result_checked_out: std::sync::atomic::AtomicBool::new(false),
            phase: std::sync::atomic::AtomicU8::new(DatabaseCatalogReadPhase::Handoff as u8),
            progress: std::sync::atomic::AtomicU8::new(DatabaseCatalogReadProgress::Admitted as u8),
            #[cfg(test)]
            controlled_submit_hook: std::sync::Mutex::new(None),
            #[cfg(test)]
            controlled_publication_before_waker_hook: std::sync::Mutex::new(None),
        });
        database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        if schedule {
            state.schedule();
        }
        Ok(Self { state, resolved: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> DatabaseCatalogReadProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => DatabaseCatalogReadProgress::Admitted,
            1 => DatabaseCatalogReadProgress::Scheduled,
            2 => DatabaseCatalogReadProgress::Polling,
            3 => DatabaseCatalogReadProgress::Pending,
            4 => DatabaseCatalogReadProgress::Completed,
            5 => DatabaseCatalogReadProgress::Cancelled,
            _ => DatabaseCatalogReadProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        if !self.state.scheduled.load(std::sync::atomic::Ordering::Acquire) && !self.state.polling.load(std::sync::atomic::Ordering::Acquire) {
            self.state.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
        }
    }
}

impl Future for DatabaseCatalogReadFuture {
    type Output = Result<DatabaseCatalogReadResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.release_success();
            return std::task::Poll::Ready(result);
        }
        #[cfg(test)]
        if let Some(hook) = self.state.controlled_publication_before_waker_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook();
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.resolved = true;
            self.state.release_success();
            return std::task::Poll::Ready(result);
        }
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseCatalogReadFuture {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        if !self.state.scheduled.load(std::sync::atomic::Ordering::Acquire) && !self.state.polling.load(std::sync::atomic::Ordering::Acquire) {
            self.state.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
        }
    }
}

pub struct DatabaseCatalogReadTerminalHandle {
    state: Arc<DatabaseCatalogReadState>,
}

impl DatabaseCatalogReadTerminalHandle {
    pub fn close_step(&self) -> DatabaseCatalogReadCloseStep {
        self.state.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty()
    }

    pub fn take_result(&self) -> Option<DatabaseCatalogReadTerminalResult> {
        use std::sync::atomic::Ordering;
        if self.state.terminal_result_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        if self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.terminal_result_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(DatabaseCatalogReadTerminalResult { state: self.state.clone(), checked_out: true })
    }

    pub fn resume(self) -> Result<DatabaseCatalogReadFuture, Self> {
        use std::sync::atomic::Ordering;
        if self.state.scheduled.load(Ordering::Acquire) || self.state.polling.load(Ordering::Acquire) || self.state.finished.load(Ordering::Acquire) {
            return Err(self);
        }
        self.state.cancelled.store(false, Ordering::Release);
        self.state.abandoned.store(false, Ordering::Release);
        let terminal_error = self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = self.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.state.scheduled.store(true, Ordering::Release);
            self.state.submit_exact(job, attempt);
        } else if let Some(work) = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.state.set_phase(DatabaseCatalogReadPhase::Poll);
            self.state.schedule();
        } else {
            self.state.cancelled.store(true, Ordering::Release);
            self.state.abandoned.store(true, Ordering::Release);
            *self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = terminal_error;
            return Err(self);
        }
        self.state.terminal_checked_out.store(false, Ordering::Release);
        Ok(DatabaseCatalogReadFuture { state: self.state.clone(), resolved: false })
    }
}

impl Drop for DatabaseCatalogReadTerminalHandle {
    fn drop(&mut self) {
        self.state.terminal_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

pub struct DatabaseCatalogReadTerminalResult {
    state: Arc<DatabaseCatalogReadState>,
    checked_out: bool,
}

impl DatabaseCatalogReadTerminalResult {
    pub fn take(mut self) -> Option<Result<DatabaseCatalogReadResult, DbError>> {
        let result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
        result
    }

    pub fn resume(mut self) -> Result<DatabaseCatalogReadFuture, Self> {
        use std::sync::atomic::Ordering;
        if self.state.scheduled.load(Ordering::Acquire) || self.state.polling.load(Ordering::Acquire) || self.state.finished.load(Ordering::Acquire) || self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            return Err(self);
        }
        let Some(result) = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return Err(self) };
        *self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        self.checked_out = false;
        self.state.terminal_result_checked_out.store(false, Ordering::Release);
        Ok(DatabaseCatalogReadFuture { state: self.state.clone(), resolved: false })
    }

    pub fn close_step(&mut self) -> DatabaseCatalogReadCloseStep {
        if self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
            DatabaseCatalogReadCloseStep::Progress
        } else {
            DatabaseCatalogReadCloseStep::Complete
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }
}

impl Drop for DatabaseCatalogReadTerminalResult {
    fn drop(&mut self) {
        if self.checked_out {
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
    }
}

pub fn take_database_catalog_read_terminal(generation: u64) -> Option<DatabaseCatalogReadTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).find(|state| state.generation == generation && state.abandoned.load(Ordering::Acquire))?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCatalogReadTerminalHandle { state })
}

//#endregion 🔖️CatalogRootRead

//#region 🔖️Ids
/// @emoji 🌉️ `protocol::ArtifactId` → `ArtifactId`, the lossless single-`String` bridge
/// `db_core`'s module doc promises — see `db_artifact`'s identical helper for the rationale (this
/// crate is the other place in the family that depends on both `db_core` and `protocol`).
async fn to_core_document_id(id: &protocol::ArtifactId) -> ArtifactId {
    ArtifactId(id.0.clone())
}

/// @emoji 🌉️ `protocol::ActorId` → `ActorId`, same bridge as `to_core_document_id`.
// 🚫️async: E4 fn-pointer slot (used as `Iterator::map(to_core_actor_id)`) — see R9
fn to_core_actor_id(id: &protocol::ActorId) -> ActorId {
    ActorId(id.0.clone())
}

async fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

#[cfg(test)]
fn test_worker_pool() -> Arc<WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 4)))).clone()
}
//#endregion 🔖️Ids

//#region 🔖️Frontier
/// @emoji 🧭️ The facade-level frontier: identical shape to `Frontier` except keyed by
/// `protocol::ArtifactId` (not `ArtifactId`) — the frozen contract's exact
/// `Frontier{document, head_seq, commit_seq, chain_hash, epoch}` shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Frontier {
    pub document: protocol::ArtifactId,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub epoch: u64,
}

impl Frontier {
    /// @emoji 🏔️ True iff `self` has observed everything `other` has — mirrors
    /// `Frontier::dominates`, re-derived here since this type's `document` field has a
    /// different type than `Frontier`'s.
    // 🚫️async: E1 pure accessor consumed by a sync Iterator::filter — see R9
    pub fn dominates(&self, other: &Frontier) -> Result<bool, DbError> {
        if self.document != other.document {
            return Err(DbError::InvalidArgument(format!("frontier document mismatch: {} vs {}", self.document.0, other.document.0)));
        }
        Ok(self.head_seq >= other.head_seq && self.commit_seq >= other.commit_seq && self.epoch >= other.epoch)
    }
}

fn to_engine_frontier(core: &db_durability::Frontier, document: protocol::ArtifactId) -> Frontier {
    Frontier { document, head_seq: core.head_seq, commit_seq: core.commit_seq, chain_hash: core.chain_hash, epoch: core.epoch }
}
//#endregion 🔖️Frontier

//#region 🔖️Receipt
/// @emoji 🧾️ The frozen `CommandReceipt` shape: `ArtifactHandle::submit`'s resolved output.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandReceipt {
    pub command_id: protocol::MutationId,
    pub frontier: Frontier,
    pub durability: DurabilityClass,
    pub conflicts: Vec<db_artifact::ConflictRecord>,
    pub state_hash: Option<ContentHash>,
    pub messages: Vec<protocol::MutationMessage>,
}

fn to_engine_receipt(receipt: db_artifact::CommandReceipt, document: protocol::ArtifactId) -> CommandReceipt {
    CommandReceipt { command_id: receipt.command_id, frontier: to_engine_frontier(&receipt.frontier, document), durability: receipt.durability, conflicts: receipt.conflicts, state_hash: receipt.state_hash, messages: receipt.messages }
}
//#endregion 🔖️Receipt

//#region 🔖️Consistency
/// @emoji 🎚️ The frozen `Consistency` enum: which frontier/view `ArtifactHandle::query` must
/// resolve against.
#[derive(Clone, Debug, PartialEq)]
pub enum Consistency {
    Canonical,
    AtLeast(Frontier),
    Exact(Frontier),
    Historical(String),
    Speculative(String),
    PreviewAugmented(String),
}
//#endregion 🔖️Consistency

//#region 🔖️Query
/// @emoji 🔎️ What `ArtifactHandle::query` can ask for — this crate's own choice (the contract fixes
/// `query`'s signature, not `Query`'s shape): single or multi-path point lookups against the
/// document's schema-erased path/value convention (see `db_artifact`'s module doc), matching what
/// `ArtifactAuthority`'s mailbox actually exposes (`ArtifactMessage::Query { path, .. }`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Query {
    Get { path: String },
    GetMany { paths: Vec<String> },
}

/// @emoji 📬️ One resolved `query`: every requested path paired with its current value bytes (`None`
/// if unset/tombstoned).
#[derive(Debug)]
pub struct QueryResultEntry {
    path: db_storage::DbIoText,
    value: Option<db_query::QueryBytes>,
}

impl QueryResultEntry {
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn value(&self) -> Option<&db_query::QueryBytes> {
        self.value.as_ref()
    }

    pub fn into_parts(self) -> (db_storage::DbIoText, Option<db_query::QueryBytes>) {
        (self.path, self.value)
    }

    fn close_step(&mut self) -> Result<bool, DbError> {
        if let Some(value) = self.value.as_mut() {
            if value.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.value = None;
        Ok(self.path.close_step())
    }
}

pub struct QueryStream {
    results: [Option<QueryResultEntry>; 64],
    len: u8,
}

impl QueryStream {
    fn new() -> Self {
        Self { results: std::array::from_fn(|_| None), len: 0 }
    }

    fn push(&mut self, entry: QueryResultEntry) -> Result<(), QueryResultEntry> {
        if self.len() == self.results.len() {
            return Err(entry);
        }
        let index = self.len();
        self.results[index] = Some(entry);
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &QueryResultEntry> {
        self.results[..self.len()].iter().flatten()
    }

    pub fn get(&self, index: usize) -> Option<&QueryResultEntry> {
        self.results.get(index).and_then(Option::as_ref)
    }

    pub fn take(&mut self) -> Option<QueryResultEntry> {
        if self.len == 0 {
            return None;
        }
        let entry = self.results[0].take();
        for index in 1..self.len() {
            self.results[index - 1] = self.results[index].take();
        }
        self.len -= 1;
        entry
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len() - 1;
        let entry = self.results[index].as_mut().ok_or_else(|| DbError::Internal("engine query close lost entry".to_string()))?;
        if entry.close_step()? {
            return Ok(true);
        }
        self.results[index] = None;
        self.len -= 1;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.results.iter().all(Option::is_none)
    }
}
//#endregion 🔖️Query

//#region 🔖️History
pub use db_artifact::ArtifactHistoryEntry as HistoryEntry;

pub struct HistoryView {
    inner: Option<db_artifact::ArtifactHistoryView>,
    admission: Option<ArtifactHistoryAdmission>,
    terminal_state: Option<Arc<ArtifactHistoryState>>,
    return_on_drop: bool,
}

impl HistoryView {
    fn new(inner: db_artifact::ArtifactHistoryView, admission: Option<ArtifactHistoryAdmission>, terminal_state: Arc<ArtifactHistoryState>) -> Self {
        Self { inner: Some(inner), admission, terminal_state: Some(terminal_state), return_on_drop: true }
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        self.inner.as_ref().map_or(&[], |inner| inner.entries.as_slice())
    }

    pub fn operation_id_eq(&self, entry: usize, operation: usize, expected: &str) -> bool {
        self.inner.as_ref().is_some_and(|inner| inner.operation_id_eq(entry, operation, expected))
    }

    pub fn close_step(&mut self) -> bool {
        if self.inner.as_mut().is_some_and(db_artifact::ArtifactHistoryView::close_step) {
            return true;
        }
        if self.inner.take().is_some() {
            return true;
        }
        if self.admission.take().is_some() {
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.inner.is_none() && self.admission.is_none()
    }

    fn rearm_terminal_return(&mut self) {
        self.return_on_drop = true;
    }
}

impl Drop for HistoryView {
    fn drop(&mut self) {
        if !self.terminal_is_empty() && self.return_on_drop {
            if let Some(state) = self.terminal_state.as_ref().cloned() {
                let owner = HistoryView { inner: self.inner.take(), admission: self.admission.take(), terminal_state: Some(state.clone()), return_on_drop: false };
                state.finished.store(false, std::sync::atomic::Ordering::Release);
                register_artifact_history(&state);
                *state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(owner));
            }
        }
        assert!(self.terminal_is_empty(), "history result reached Drop before page/range/admission retirement");
    }
}

//#endregion 🔖️History

//#region 🔖️LiveQuery + Preview
/// @emoji 📡️ What `ArtifactHandle::subscribe` would filter on — defined for API-shape completeness
/// even though every construction path currently returns `DbError::Unimplemented` (see module doc).
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuerySpec {
    pub since: Option<Frontier>,
}

/// @emoji 📡️ A live subscription handle — see `LiveQuerySpec`'s doc on why this is currently
/// unreachable except through the documented `Unimplemented` error.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuery {
    pub id: String,
}

/// @emoji 🌫️ An ephemeral preview overlay handle — see `LiveQuerySpec`'s doc; same deferral reason.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewHandle {
    pub id: String,
    pub base: Frontier,
}
//#endregion 🔖️LiveQuery + Preview

//#region 🔖️Snapshot
/// @emoji 📸️ What kind of snapshot `ArtifactHandle::snapshot_now` was asked to build — defined for
/// API-shape completeness (see module doc: this crate does not yet build real pack snapshots).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotKind {
    Full,
    Incremental,
}

/// @emoji 📸️ What a successful `snapshot_now` would resolve to — currently unreachable, see above.
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotReceipt {
    pub generation: u64,
    pub frontier: Frontier,
}

pub type SnapshotFuture = db_actor::ReplyReceiver<Result<SnapshotReceipt, DbError>>;
//#endregion 🔖️Snapshot

//#region 🔖️Security
/// @emoji 🛂️ A real `db_artifact::AuthzHook` built on `db_security::SecurityGate`: resolves the
/// submitting `protocol::ActorId` to a `db_security::Principal` via an injected closure, then
/// authorizes `Action::Write` on `AuthzScope::Document { document }`. Not the default (the default
/// stays `db_artifact::AllowAll`, matching `db_artifact`'s own single-tenant default) — opt in via
/// `Database::open_with_authz`.
pub struct SecurityAuthzHook {
    gate: db_security::SecurityGate,
    principal_for: Box<dyn Fn(&protocol::ActorId) -> db_security::Principal + Send + Sync>,
}

impl SecurityAuthzHook {
    pub async fn new(gate: db_security::SecurityGate, principal_for: impl Fn(&protocol::ActorId) -> db_security::Principal + Send + Sync + 'static) -> SecurityAuthzHook {
        SecurityAuthzHook { gate, principal_for: Box::new(principal_for) }
    }
}

impl db_artifact::AuthzHook for SecurityAuthzHook {
    async fn authorize(&self, actor: &protocol::ActorId, envelope: &protocol::MutationEnvelope) -> Result<(), DbError> {
        let principal = (self.principal_for)(actor);
        self.gate.authorize(&principal, &db_security::AuthzScope::Document { document: envelope.document_id.clone() }, db_security::Action::Write).await
    }
}
//#endregion 🔖️Security

//#region 🔖️VersionGraph
/// @emoji 🌿️ The real `vcs`-backed `VersionGraph` — the ONLY place in the whole `db`
/// family allowed to depend on `vcs` (hard dependency rule; gated behind this crate's default-on
/// `vcs` Cargo feature).
#[cfg(feature = "vcs")]
pub mod vcs_integration {
    use crate::db_ids::*;
    use crate::db_version_graph::*;
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Mutex;
    use std::task::{Context, Poll, Waker};

    //#region 🔖️SchemaErasedTypes
    /// @emoji #⃣ The `VersionGraph` seam (`ChangeRecord`/`CheckpointRequest`) is already
    /// schema-erased — it carries a `pack::ContentHash`, never document semantics — so this
    /// crate drives the real `store::ArtifactStore<P, Mutation>` with the smallest concrete `P`/
    /// `Mutation` pair that can faithfully round-trip exactly that: a projection that IS the
    /// latest recorded hash, and an operation that overwrites it (its `inverse` recovering the
    /// PRIOR hash from the pre-state, a real, correct inverse — not a placeholder). This mirrors
    /// `db_artifact`'s own schema-erased-JSON convention one layer up: neither crate has (or needs)
    /// compile-time knowledge of any real document schema.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct HashProjection {
        pub latest_hash: [u8; 32],
    }

    impl store::ArtifactDsl for HashProjection {
        const EXTENSION: &'static str = "dbhash";

        fn parse_dsl(text: &str) -> Result<HashProjection, store::TextError> {
            let trimmed = text.trim();
            if trimmed.len() != 64 || !trimmed.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                return Err(store::TextError::new("expected 64 lowercase hex characters", store::TextSpan::at(1, 1)));
            }
            let mut latest_hash = [0u8; 32];
            for (index, slot) in latest_hash.iter_mut().enumerate() {
                *slot = u8::from_str_radix(&trimmed[index * 2..index * 2 + 2], 16).map_err(|_| store::TextError::new("invalid hex byte", store::TextSpan::at(1, (index * 2 + 1) as u32)))?;
            }
            Ok(HashProjection { latest_hash })
        }

        fn print_dsl(&self) -> String {
            let mut out = String::with_capacity(64);
            for byte in self.latest_hash {
                use std::fmt::Write;
                let _ = write!(out, "{byte:02x}");
            }
            out
        }
    }

    impl store::ArtifactPack for HashProjection {
        fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
            Ok(self.latest_hash.to_vec())
        }
        fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
            let latest_hash: [u8; 32] = bytes.try_into().map_err(|_| store::PackError::Schema("HashProjection pack must be exactly 32 bytes".to_string()))?;
            Ok(HashProjection { latest_hash })
        }
    }

    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct HashDiff {
        pub hash: Option<[u8; 32]>,
    }

    impl protocol::MutationDiff<HashProjection> for HashDiff {
        fn apply(&self, base: &HashProjection) -> protocol::MutationApplyResult<HashProjection> {
            Ok(match self.hash {
                Some(hash) => HashProjection { latest_hash: hash },
                None => base.clone(),
            })
        }

        fn absorb(&mut self, other: HashDiff) {
            if other.hash.is_some() {
                self.hash = other.hash;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct HashMutation {
        pub hash: [u8; 32],
        pub author: Option<protocol::ActorId>,
        pub timestamp: Option<protocol::HybridLogicalTimestamp>,
    }

    impl protocol::Mutation<HashProjection> for HashMutation {
        type Diff = HashDiff;

        fn diff(&self, _base: &HashProjection) -> protocol::MutationOutcome<HashDiff> {
            protocol::MutationOutcome::new(HashDiff { hash: Some(self.hash) })
        }

        /// @emoji ↩️ The true inverse: an operation that would restore `base`'s hash — not a
        /// no-op placeholder.
        fn inverse(&self, base: &HashProjection) -> Vec<HashMutation> {
            vec![HashMutation { hash: base.latest_hash, author: self.author.clone(), timestamp: self.timestamp }]
        }

        fn author_id(&self) -> Option<protocol::ActorId> {
            self.author.clone()
        }

        fn timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
            self.timestamp
        }
    }

    // 🚫️async: E1 pure accessor consumed synchronously inside `format!` — see R9
    fn hex_encode(bytes: &[u8; 32]) -> String {
        use std::fmt::Write;
        let mut out = String::with_capacity(64);
        for byte in bytes {
            let _ = write!(out, "{byte:02x}");
        }
        out
    }

    fn hex_decode(text: &str) -> Result<[u8; 32], String> {
        if text.len() != 64 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("expected 64 lowercase hex characters".to_string());
        }
        let mut out = [0u8; 32];
        for (index, slot) in out.iter_mut().enumerate() {
            *slot = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).map_err(|error| error.to_string())?;
        }
        Ok(out)
    }

    /// @emoji 🎯️ Single-line text form: `hash=<hex64>[ author=<id>][ ts=<actor>,<physical_ms>,<logical>]`.
    impl protocol::OpText for HashMutation {
        fn print_op(&self) -> String {
            let mut out = format!("hash={}", hex_encode(&self.hash));
            if let Some(author) = &self.author {
                out.push_str(&format!(" author={}", author.0));
            }
            if let Some(ts) = &self.timestamp {
                out.push_str(&format!(" ts={},{},{}", ts.actor, ts.physical_ms, ts.logical));
            }
            out
        }
        fn parse_op(line: &str) -> Result<Self, store::TextError> {
            let err = |detail: String| store::TextError::new(detail, store::TextSpan::at(1, 1));
            let mut hash = None;
            let mut author = None;
            let mut timestamp = None;
            for token in line.split_whitespace() {
                let (key, value) = token.split_once('=').ok_or_else(|| err(format!("malformed token '{token}'")))?;
                match key {
                    "hash" => hash = Some(hex_decode(value).map_err(err)?),
                    "author" => author = Some(protocol::ActorId(value.to_string())),
                    "ts" => {
                        let parts: Vec<&str> = value.split(',').collect();
                        if parts.len() != 3 {
                            return Err(err(format!("malformed ts '{value}'")));
                        }
                        let actor = parts[0].parse::<u64>().map_err(|error| err(error.to_string()))?;
                        let physical_ms = parts[1].parse::<u64>().map_err(|error| err(error.to_string()))?;
                        let logical = parts[2].parse::<u64>().map_err(|error| err(error.to_string()))?;
                        timestamp = Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical });
                    }
                    other => return Err(err(format!("unknown key '{other}'"))),
                }
            }
            Ok(HashMutation { hash: hash.ok_or_else(|| err("missing hash".to_string()))?, author, timestamp })
        }
    }

    /// @emoji 🎯️ Binary form: `hash 32 bytes | presence u8 (bit0=author, bit1=timestamp) | [author
    /// len varint + utf8 bytes] | [timestamp: actor/physical_ms/logical varint each]`.
    impl protocol::OpBinary for HashMutation {
        fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
            let mut out = self.hash.to_vec();
            let presence = (self.author.is_some() as u8) | ((self.timestamp.is_some() as u8) << 1);
            out.push(presence);
            if let Some(author) = &self.author {
                pack::os_pack::write_varint_u64(&mut out, author.0.len() as u64);
                out.extend_from_slice(author.0.as_bytes());
            }
            if let Some(ts) = &self.timestamp {
                pack::os_pack::write_varint_u64(&mut out, ts.actor);
                pack::os_pack::write_varint_u64(&mut out, ts.physical_ms);
                pack::os_pack::write_varint_u64(&mut out, ts.logical);
            }
            Ok(out)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
            let malformed = |detail: String| protocol::ProtocolError::Malformed { what: "hash op", offset: 0, detail };
            if bytes.len() < 33 {
                return Err(malformed("truncated hash op".to_string()));
            }
            let hash: [u8; 32] = bytes[..32].try_into().expect("checked len");
            let presence = bytes[32];
            let mut pos = 33usize;
            let author = if presence & 0b01 != 0 {
                let len = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))? as usize;
                let end = pos + len;
                let text = std::str::from_utf8(bytes.get(pos..end).ok_or_else(|| malformed("truncated author".to_string()))?).map_err(|error| malformed(error.to_string()))?.to_string();
                pos = end;
                Some(protocol::ActorId(text))
            } else {
                None
            };
            let timestamp = if presence & 0b10 != 0 {
                let actor = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
                let physical_ms = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
                let logical = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
                Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical })
            } else {
                None
            };
            Ok(HashMutation { hash, author, timestamp })
        }
    }
    //#endregion 🔖️SchemaErasedTypes

    //#region 🔖️Store
    type HashStore = store::ArtifactStore<HashProjection, HashMutation>;

    const VCS_OPERATION_ITEMS: usize = 64;
    const VCS_OPERATION_PAGE_BYTES: u64 = 16 * 1024;
    const VCS_OPERATION_PAGES: u64 = 4;
    const VCS_OPERATION_BYTES: u64 = VCS_OPERATION_PAGE_BYTES * VCS_OPERATION_PAGES;
    const VCS_TOTAL_PAGES: u64 = 256;
    const VCS_TOTAL_BYTES: u64 = VCS_OPERATION_PAGE_BYTES * VCS_TOTAL_PAGES;

    #[derive(Clone, Copy)]
    struct VcsAdmissionSlot {
        generation: u64,
        bytes: u64,
        items: usize,
        occupied: bool,
    }

    const EMPTY_VCS_ADMISSION_SLOT: VcsAdmissionSlot = VcsAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

    struct VcsAdmissionState {
        slots: [VcsAdmissionSlot; VCS_OPERATION_ITEMS],
        bytes: u64,
        next_generation: u64,
    }

    static VCS_ADMISSION: Mutex<VcsAdmissionState> = Mutex::new(VcsAdmissionState { slots: [EMPTY_VCS_ADMISSION_SLOT; VCS_OPERATION_ITEMS], bytes: 0, next_generation: 1 });

    struct VcsOperationAdmission {
        slot: usize,
        generation: u64,
        bytes: u64,
        items: usize,
    }

    impl VcsOperationAdmission {
        fn try_claim(items: usize, bytes: u64) -> Result<Self, DbError> {
            if items == 0 || items > VCS_OPERATION_ITEMS {
                return Err(DbError::LimitExceeded("vcs operation item credit"));
            }
            if bytes == 0 || bytes > VCS_OPERATION_BYTES {
                return Err(DbError::LimitExceeded("vcs operation byte credit"));
            }
            let mut state = VCS_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
                return Err(DbError::Unavailable("vcs operation capacity exhausted".to_string()));
            };
            if state.bytes.checked_add(bytes).is_none_or(|next| next > VCS_TOTAL_BYTES) {
                return Err(DbError::Unavailable("vcs operation byte capacity exhausted".to_string()));
            }
            let generation = state.next_generation;
            state.next_generation = state.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("vcs operation generation"))?;
            state.slots[slot] = VcsAdmissionSlot { generation, bytes, items, occupied: true };
            state.bytes += bytes;
            Ok(Self { slot, generation, bytes, items })
        }

        fn is_current(slot: usize, generation: u64) -> bool {
            let state = VCS_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
        }
    }

    impl Drop for VcsOperationAdmission {
        fn drop(&mut self) {
            let mut state = VCS_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let entry = &mut state.slots[self.slot];
            if !entry.occupied || entry.generation != self.generation || entry.bytes != self.bytes || entry.items != self.items {
                return;
            }
            *entry = EMPTY_VCS_ADMISSION_SLOT;
            state.bytes = state.bytes.checked_sub(self.bytes).expect("vcs operation byte credit underflow");
        }
    }

    fn vcs_credit(items: usize, owner_bytes: impl IntoIterator<Item = usize>) -> Result<(usize, u64), DbError> {
        if items == 0 || items > VCS_OPERATION_ITEMS {
            return Err(DbError::LimitExceeded("vcs operation nested item credit"));
        }
        let mut bytes = VCS_OPERATION_PAGE_BYTES;
        for owner_bytes in owner_bytes {
            bytes = bytes.checked_add(owner_bytes as u64).ok_or(DbError::LimitExceeded("vcs operation nested byte credit"))?;
        }
        let pages = bytes.checked_add(VCS_OPERATION_PAGE_BYTES - 1).ok_or(DbError::LimitExceeded("vcs operation page rounding"))? / VCS_OPERATION_PAGE_BYTES;
        let admitted = pages.checked_mul(VCS_OPERATION_PAGE_BYTES).ok_or(DbError::LimitExceeded("vcs operation page credit"))?;
        if admitted > VCS_OPERATION_BYTES {
            return Err(DbError::LimitExceeded("vcs operation byte credit"));
        }
        Ok((items, admitted))
    }

    fn record_credit(document: &ArtifactId, change: &ChangeRecord) -> Result<(usize, u64), DbError> {
        vcs_credit(1 + usize::from(change.parent.is_some()), [document.0.capacity(), change.parent.as_ref().map_or(0, String::capacity), change.author.0.capacity(), change.message.capacity(), std::mem::size_of::<HashMutation>()])
    }

    fn checkpoint_credit(document: &ArtifactId, request: &CheckpointRequest) -> Result<(usize, u64), DbError> {
        let derived_author_items = request.authors.len();
        let items = 1usize
            .checked_add(usize::from(request.parent_checkpoint.is_some()))
            .and_then(|value| value.checked_add(request.change_ids.len()))
            .and_then(|value| value.checked_add(request.authors.len()))
            .and_then(|value| value.checked_add(derived_author_items))
            .ok_or(DbError::LimitExceeded("vcs checkpoint item credit"))?;
        let change_owner_bytes = request.change_ids.capacity().checked_mul(std::mem::size_of::<String>()).ok_or(DbError::LimitExceeded("vcs checkpoint change owner bytes"))?;
        let author_owner_bytes = request.authors.capacity().checked_mul(std::mem::size_of::<ActorId>()).ok_or(DbError::LimitExceeded("vcs checkpoint author owner bytes"))?;
        let derived_author_owner_bytes = request.authors.capacity().checked_mul(std::mem::size_of::<vcs::Author>()).ok_or(DbError::LimitExceeded("vcs checkpoint derived author owner bytes"))?;
        let derived_author_id_bytes = request.authors.iter().try_fold(0usize, |bytes, author| bytes.checked_add(author.0.capacity())).ok_or(DbError::LimitExceeded("vcs checkpoint derived author id bytes"))?;
        let fixed = [document.0.capacity(), request.parent_checkpoint.as_ref().map_or(0, String::capacity), request.message.capacity(), change_owner_bytes, author_owner_bytes, derived_author_owner_bytes, derived_author_id_bytes];
        vcs_credit(items, fixed.into_iter().chain(request.change_ids.iter().map(String::capacity)).chain(request.authors.iter().map(|author| author.0.capacity())))
    }

    fn relation_credit(document: &ArtifactId, values: &[&str]) -> Result<(usize, u64), DbError> {
        vcs_credit(1 + values.len(), std::iter::once(document.0.capacity()).chain(values.iter().map(|value| value.len())))
    }

    struct VcsStoreWaiter {
        generation: u64,
        waker: Waker,
    }

    struct VcsStoreCellState {
        store: Option<HashStore>,
        busy_generation: Option<u64>,
        waiters: [Option<VcsStoreWaiter>; VCS_OPERATION_ITEMS],
    }

    struct VcsStoreCell {
        state: Mutex<VcsStoreCellState>,
    }

    impl VcsStoreCell {
        fn new() -> Self {
            Self { state: Mutex::new(VcsStoreCellState { store: None, busy_generation: None, waiters: std::array::from_fn(|_| None) }) }
        }

        fn take_next_waker(state: &mut VcsStoreCellState) -> Option<(u64, Waker)> {
            let next = state.waiters.iter().enumerate().filter_map(|(slot, waiter)| waiter.as_ref().map(|waiter| (slot, waiter.generation))).min_by_key(|(_, generation)| *generation).map(|(slot, _)| slot)?;
            state.waiters[next].take().map(|waiter| (waiter.generation, waiter.waker))
        }

        fn release(&self, generation: u64, store: Option<HashStore>) {
            let wake = {
                let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if state.busy_generation != Some(generation) {
                    return;
                }
                if let Some(store) = store {
                    state.store = Some(store);
                }
                state.busy_generation = None;
                let next = Self::take_next_waker(&mut state);
                if let Some((generation, _)) = &next {
                    state.busy_generation = Some(*generation);
                }
                next.map(|(_, waker)| waker)
            };
            if let Some(waker) = wake {
                waker.wake();
            }
        }
    }

    struct VcsStoreAcquire {
        cell: std::sync::Arc<VcsStoreCell>,
        slot: usize,
        generation: u64,
        resolved: bool,
    }

    enum VcsStoreClaim {
        Build(VcsStoreBuildPermit),
        Ready(VcsStoreLease),
    }

    struct VcsStoreBuildPermit {
        cell: std::sync::Arc<VcsStoreCell>,
        generation: u64,
        resolved: bool,
    }

    struct VcsStoreLease {
        cell: std::sync::Arc<VcsStoreCell>,
        generation: u64,
        store: Option<HashStore>,
    }

    impl Future for VcsStoreAcquire {
        type Output = Result<VcsStoreClaim, DbError>;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            if !VcsOperationAdmission::is_current(self.slot, self.generation) {
                self.resolved = true;
                return Poll::Ready(Err(DbError::Closed));
            }
            let mut state = self.cell.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.busy_generation.is_none() || state.busy_generation == Some(self.generation) {
                state.busy_generation = Some(self.generation);
                state.waiters[self.slot] = None;
                let store = state.store.take();
                drop(state);
                self.resolved = true;
                let claim = match store {
                    Some(store) => VcsStoreClaim::Ready(VcsStoreLease { cell: self.cell.clone(), generation: self.generation, store: Some(store) }),
                    None => VcsStoreClaim::Build(VcsStoreBuildPermit { cell: self.cell.clone(), generation: self.generation, resolved: false }),
                };
                return Poll::Ready(Ok(claim));
            }
            let waiter = &mut state.waiters[self.slot];
            if waiter.as_ref().is_none_or(|waiter| waiter.generation != self.generation || !waiter.waker.will_wake(context.waker())) {
                *waiter = Some(VcsStoreWaiter { generation: self.generation, waker: context.waker().clone() });
            }
            Poll::Pending
        }
    }

    impl Drop for VcsStoreAcquire {
        fn drop(&mut self) {
            if self.resolved {
                return;
            }
            let mut state = self.cell.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.waiters[self.slot].as_ref().is_some_and(|waiter| waiter.generation == self.generation) {
                state.waiters[self.slot] = None;
            }
            let wake = if state.busy_generation == Some(self.generation) {
                state.busy_generation = None;
                let next = VcsStoreCell::take_next_waker(&mut state);
                if let Some((generation, _)) = &next {
                    state.busy_generation = Some(*generation);
                }
                next.map(|(_, waker)| waker)
            } else {
                None
            };
            drop(state);
            if let Some(waker) = wake {
                waker.wake();
            }
        }
    }

    impl VcsStoreBuildPermit {
        fn install(mut self, store: HashStore) -> VcsStoreLease {
            self.resolved = true;
            VcsStoreLease { cell: self.cell.clone(), generation: self.generation, store: Some(store) }
        }
    }

    impl Drop for VcsStoreBuildPermit {
        fn drop(&mut self) {
            if !self.resolved {
                self.cell.release(self.generation, None);
            }
        }
    }

    impl VcsStoreLease {
        fn store_mut(&mut self) -> &mut HashStore {
            self.store.as_mut().expect("vcs store lease owner already returned")
        }
    }

    impl Drop for VcsStoreLease {
        fn drop(&mut self) {
            self.cell.release(self.generation, self.store.take());
        }
    }

    // 🔒️ Used as a bare fn-pointer error mapper (`.map_err(map_vcs_error)`) below — same rationale
    // as `db_artifact`'s `json_err`: `Result::map_err`'s `FnOnce(E) -> F2` bound always calls the
    // mapper with an owned `E`, so a by-reference signature would not type-check at those sites.
    #[allow(clippy::needless_pass_by_value)]
    // 🚫️async: E4 fn-pointer slot
    fn map_vcs_error(err: vcs::VcsError) -> DbError {
        DbError::Internal(format!("vcs: {err}"))
    }

    /// @emoji 🌿️ One real `store::ArtifactStore` per document, driven by real `Apply`/
    /// `CommitCheckpoint` dispatches — `VersionGraph`'s real implementation.
    pub struct VcsVersionGraph {
        stores: Mutex<HashMap<String, std::sync::Arc<VcsStoreCell>>>,
    }

    impl Default for VcsVersionGraph {
        fn default() -> VcsVersionGraph {
            VcsVersionGraph { stores: Mutex::new(HashMap::new()) }
        }
    }

    impl VcsVersionGraph {
        pub async fn new() -> VcsVersionGraph {
            VcsVersionGraph::default()
        }

        async fn store(&self, document: &ArtifactId, admission: &VcsOperationAdmission) -> Result<VcsStoreLease, DbError> {
            let cell = {
                let mut stores = self.stores.lock().map_err(|_| DbError::Internal("vcs_integration: store registry mutex poisoned".to_string()))?;
                stores.entry(document.0.clone()).or_insert_with(|| std::sync::Arc::new(VcsStoreCell::new())).clone()
            };
            match (VcsStoreAcquire { cell, slot: admission.slot, generation: admission.generation, resolved: false }).await? {
                VcsStoreClaim::Ready(lease) => Ok(lease),
                VcsStoreClaim::Build(permit) => {
                    let envelope = store::create_document_envelope::<HashProjection, HashMutation>("db_engine.version_graph", &document.0, HashProjection::default(), None);
                    let store = store::ArtifactStore::new(envelope).await.map_err(map_vcs_error)?;
                    Ok(permit.install(store))
                }
            }
        }
    }

    impl VersionGraph for VcsVersionGraph {
        async fn record_change(&self, document: &ArtifactId, change: ChangeRecord) -> Result<String, DbError> {
            let admission = record_credit(document, &change).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes))?;
            let mut lease = self.store(document, &admission).await?;
            let ChangeRecord { content_hash, author, message, timestamp_ms, .. } = change;
            let operation = HashMutation { hash: content_hash.0, author: Some(protocol::ActorId(author.0)), timestamp: Some(protocol::HybridLogicalTimestamp::new(0, timestamp_ms)) };
            let mutations = Vec::from([operation]);
            lease.store_mut().dispatch(store::ArtifactCommand::Apply { mutations, description: Some(message) }).await.map_err(map_vcs_error)?;
            Ok(lease.store_mut().envelope().await.vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default())
        }

        /// @emoji 🎯️ Design choice: `request.parent_checkpoint`/`change_ids` are NOT threaded
        /// through — `store::ArtifactCommand::CommitCheckpoint` always folds every edit applied
        /// since the store's OWN current checkpoint (tracked internally by `ArtifactStore`,
        /// advanced by `record_change`'s `Apply` calls above), which is the only value that could
        /// ever be consistent with this store's real history. `request.timestamp_ms` is similarly
        /// unused: `vcs`'s own `CommitCheckpoint` handler stamps its own `now_iso()` timestamp into
        /// the checkpoint (part of what its content-addressed id hashes over) — this crate cannot
        /// override that without reaching into `vcs`'s private state.
        async fn checkpoint(&self, document: &ArtifactId, request: CheckpointRequest) -> Result<String, DbError> {
            let admission = checkpoint_credit(document, &request).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes))?;
            let mut lease = self.store(document, &admission).await?;
            let CheckpointRequest { message, authors: source_authors, .. } = request;
            let mut authors = Vec::with_capacity(source_authors.capacity());
            for author in source_authors {
                let name = author.0;
                authors.push(vcs::Author { id: name.clone(), name, avatar: None });
            }
            lease.store_mut().dispatch(store::ArtifactCommand::CommitCheckpoint { message: Some(message), authors }).await.map_err(map_vcs_error)?;
            lease.store_mut().current_checkpoint_id().await.map(str::to_string).ok_or_else(|| DbError::Internal("vcs: commit_checkpoint produced no checkpoint id".to_string()))
        }

        async fn merge_base(&self, document: &ArtifactId, a: &str, b: &str) -> Result<Option<String>, DbError> {
            let admission = relation_credit(document, &[a, b]).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes))?;
            let mut lease = self.store(document, &admission).await?;
            Ok(store::merge_base(lease.store_mut().envelope().await, a, b).await)
        }

        async fn head(&self, document: &ArtifactId, alternative: &str) -> Result<Option<String>, DbError> {
            let admission = relation_credit(document, &[alternative]).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes))?;
            let mut lease = self.store(document, &admission).await?;
            let envelope = lease.store_mut().envelope().await;
            if let Some(found) = envelope.vcs.alternatives.iter().find(|candidate| candidate.id == alternative || candidate.name == alternative) {
                return Ok(found.checkpoint_ids.last().cloned());
            }
            Ok(lease.store_mut().current_checkpoint_id().await.map(str::to_string))
        }
    }

    #[cfg(test)]
    mod retained_tests {
        use super::*;
        use std::sync::atomic::{AtomicUsize, Ordering};

        static TEST_LOCK: Mutex<()> = Mutex::new(());

        struct CountWake(AtomicUsize);

        impl std::task::Wake for CountWake {
            fn wake(self: std::sync::Arc<Self>) {
                self.0.fetch_add(1, Ordering::AcqRel);
            }
        }

        #[test]
        fn vcs_retained_item_cap_plus_one_and_nested_bytes_plus_one_return_without_mutation() {
            let _guard = TEST_LOCK.lock().unwrap();
            let claims: Vec<VcsOperationAdmission> = (0..VCS_OPERATION_ITEMS).map(|_| VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap()).collect();
            assert!(VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).is_err());
            assert!(vcs_credit(VCS_OPERATION_ITEMS + 1, [0]).is_err());
            assert!(vcs_credit(1, [VCS_OPERATION_BYTES as usize]).is_err());
            drop(claims);
        }

        fn record_with_author_bytes(bytes: usize) -> ChangeRecord {
            let author = "a".repeat(bytes);
            assert_eq!(author.capacity(), bytes);
            ChangeRecord { parent: None, content_hash: pack::ContentHash([1; 32]), author: ActorId(author), message: String::new(), timestamp_ms: 1 }
        }

        fn checkpoint_with_author_bytes(bytes: usize) -> CheckpointRequest {
            let author = "a".repeat(bytes);
            assert_eq!(author.capacity(), bytes);
            let mut authors = Vec::with_capacity(1);
            authors.push(ActorId(author));
            CheckpointRequest { parent_checkpoint: None, change_ids: Vec::new(), message: String::new(), authors, timestamp_ms: 1 }
        }

        #[test]
        fn vcs_record_derived_owner_credit_cap_plus_one_preserves_exact_input() {
            let document = ArtifactId(String::new());
            let author_bytes = VCS_OPERATION_BYTES as usize - VCS_OPERATION_PAGE_BYTES as usize - std::mem::size_of::<HashMutation>();
            let accepted = record_with_author_bytes(author_bytes);
            assert_eq!(record_credit(&document, &accepted).unwrap(), (1, VCS_OPERATION_BYTES));
            let rejected = record_with_author_bytes(author_bytes + 1);
            let author_owner = rejected.author.0.as_ptr();
            assert!(record_credit(&document, &rejected).is_err());
            assert_eq!(rejected.author.0.as_ptr(), author_owner);
            assert_eq!(rejected.author.0.len(), author_bytes + 1);
        }

        #[test]
        fn vcs_checkpoint_derived_owner_credit_cap_plus_one_preserves_exact_input() {
            let document = ArtifactId(String::new());
            let fixed = VCS_OPERATION_PAGE_BYTES as usize + std::mem::size_of::<ActorId>() + std::mem::size_of::<vcs::Author>();
            let author_bytes = (VCS_OPERATION_BYTES as usize - fixed) / 2;
            let accepted = checkpoint_with_author_bytes(author_bytes);
            assert_eq!(checkpoint_credit(&document, &accepted).unwrap().1, VCS_OPERATION_BYTES);
            let rejected = checkpoint_with_author_bytes(author_bytes + 1);
            let author_owner = rejected.authors[0].0.as_ptr();
            assert!(checkpoint_credit(&document, &rejected).is_err());
            assert_eq!(rejected.authors[0].0.as_ptr(), author_owner);
            assert_eq!(rejected.authors[0].0.len(), author_bytes + 1);
        }

        #[test]
        fn vcs_checkpoint_derived_item_boundary_admits_31_rejects_32_and_preserves_exact_owners() {
            let document = ArtifactId(String::new());
            let request = |count: usize| CheckpointRequest { parent_checkpoint: None, change_ids: Vec::new(), message: String::new(), authors: (0..count).map(|index| ActorId(format!("author-{index}"))).collect(), timestamp_ms: 1 };
            let accepted = request(31);
            let accepted_authors = accepted.authors.as_ptr();
            let accepted_first = accepted.authors[0].0.as_ptr();
            let (items, bytes) = checkpoint_credit(&document, &accepted).unwrap();
            assert_eq!(items, 63);
            let admission = VcsOperationAdmission::try_claim(items, bytes).unwrap();
            assert_eq!(accepted.authors.as_ptr(), accepted_authors);
            assert_eq!(accepted.authors[0].0.as_ptr(), accepted_first);
            drop(admission);

            let rejected = request(32);
            let rejected_authors = rejected.authors.as_ptr();
            let rejected_first = rejected.authors[0].0.as_ptr();
            assert_eq!(1 + rejected.authors.len(), 33, "source-only formula would falsely admit");
            assert_eq!(1 + rejected.authors.len() * 2, 65, "materialized name and id owners exceed the cap");
            assert!(checkpoint_credit(&document, &rejected).is_err());
            assert_eq!(rejected.authors.as_ptr(), rejected_authors);
            assert_eq!(rejected.authors[0].0.as_ptr(), rejected_first);
            assert_eq!(rejected.authors.len(), 32);
        }

        #[test]
        fn vcs_derived_owner_process_aggregate_plus_one_rejects_without_consuming_input() {
            let _guard = TEST_LOCK.lock().unwrap();
            let document = ArtifactId(String::new());
            let author_bytes = VCS_OPERATION_BYTES as usize - VCS_OPERATION_PAGE_BYTES as usize - std::mem::size_of::<HashMutation>();
            let accepted = record_with_author_bytes(author_bytes);
            let (items, bytes) = record_credit(&document, &accepted).unwrap();
            let claims: Vec<VcsOperationAdmission> = (0..VCS_OPERATION_ITEMS).map(|_| VcsOperationAdmission::try_claim(items, bytes).unwrap()).collect();
            assert_eq!(VCS_ADMISSION.lock().unwrap().bytes, VCS_TOTAL_BYTES);
            let rejected = record_with_author_bytes(author_bytes);
            let author_owner = rejected.author.0.as_ptr();
            assert!(record_credit(&document, &rejected).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes)).is_err());
            assert_eq!(rejected.author.0.as_ptr(), author_owner);
            assert_eq!(VCS_ADMISSION.lock().unwrap().bytes, VCS_TOTAL_BYTES);
            drop(claims);

            let checkpoint = checkpoint_with_author_bytes((VCS_OPERATION_BYTES as usize - VCS_OPERATION_PAGE_BYTES as usize - std::mem::size_of::<ActorId>() - std::mem::size_of::<vcs::Author>()) / 2);
            let (items, bytes) = checkpoint_credit(&document, &checkpoint).unwrap();
            let claims: Vec<VcsOperationAdmission> = (0..VCS_OPERATION_ITEMS).map(|_| VcsOperationAdmission::try_claim(items, bytes).unwrap()).collect();
            assert_eq!(VCS_ADMISSION.lock().unwrap().bytes, VCS_TOTAL_BYTES);
            let rejected = checkpoint_with_author_bytes((VCS_OPERATION_BYTES as usize - VCS_OPERATION_PAGE_BYTES as usize - std::mem::size_of::<ActorId>() - std::mem::size_of::<vcs::Author>()) / 2);
            let author_owner = rejected.authors[0].0.as_ptr();
            assert!(checkpoint_credit(&document, &rejected).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes)).is_err());
            assert_eq!(rejected.authors[0].0.as_ptr(), author_owner);
            assert_eq!(VCS_ADMISSION.lock().unwrap().bytes, VCS_TOTAL_BYTES);
            drop(claims);
        }

        #[test]
        fn vcs_retained_pending_wake_is_fifo_one_shot_and_quiet_without_release() {
            let _guard = TEST_LOCK.lock().unwrap();
            let owner = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
            let second = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
            let third = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
            let cell = std::sync::Arc::new(VcsStoreCell::new());
            cell.state.lock().unwrap().busy_generation = Some(owner.generation);
            let second_wake = std::sync::Arc::new(CountWake(AtomicUsize::new(0)));
            let third_wake = std::sync::Arc::new(CountWake(AtomicUsize::new(0)));
            let second_waker = std::task::Waker::from(second_wake.clone());
            let third_waker = std::task::Waker::from(third_wake.clone());
            let mut second_context = Context::from_waker(&second_waker);
            let mut third_context = Context::from_waker(&third_waker);
            let mut second_acquire = VcsStoreAcquire { cell: cell.clone(), slot: second.slot, generation: second.generation, resolved: false };
            let mut third_acquire = VcsStoreAcquire { cell: cell.clone(), slot: third.slot, generation: third.generation, resolved: false };
            assert!(Pin::new(&mut second_acquire).poll(&mut second_context).is_pending());
            assert!(Pin::new(&mut third_acquire).poll(&mut third_context).is_pending());
            assert_eq!(second_wake.0.load(Ordering::Acquire), 0);
            assert_eq!(third_wake.0.load(Ordering::Acquire), 0);
            cell.release(owner.generation, None);
            assert_eq!(second_wake.0.load(Ordering::Acquire), 1);
            assert_eq!(third_wake.0.load(Ordering::Acquire), 0);
            let late = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
            let late_wake = std::sync::Arc::new(CountWake(AtomicUsize::new(0)));
            let late_waker = std::task::Waker::from(late_wake.clone());
            let mut late_context = Context::from_waker(&late_waker);
            let mut late_acquire = VcsStoreAcquire { cell: cell.clone(), slot: late.slot, generation: late.generation, resolved: false };
            assert!(Pin::new(&mut late_acquire).poll(&mut late_context).is_pending());
            let Poll::Ready(Ok(VcsStoreClaim::Build(permit))) = Pin::new(&mut second_acquire).poll(&mut second_context) else {
                panic!("second retained owner must acquire first");
            };
            drop(permit);
            assert_eq!(third_wake.0.load(Ordering::Acquire), 1);
            assert_eq!(late_wake.0.load(Ordering::Acquire), 0);
        }

        #[test]
        fn vcs_retained_cancel_clears_waiter_and_slot_aba_stays_stale() {
            let _guard = TEST_LOCK.lock().unwrap();
            let owner = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
            let waiter = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
            let cell = std::sync::Arc::new(VcsStoreCell::new());
            cell.state.lock().unwrap().busy_generation = Some(owner.generation);
            let waker = std::task::Waker::from(std::sync::Arc::new(CountWake(AtomicUsize::new(0))));
            let mut context = Context::from_waker(&waker);
            let mut acquire = VcsStoreAcquire { cell: cell.clone(), slot: waiter.slot, generation: waiter.generation, resolved: false };
            assert!(Pin::new(&mut acquire).poll(&mut context).is_pending());
            drop(acquire);
            assert!(cell.state.lock().unwrap().waiters[waiter.slot].is_none());
            let slot = waiter.slot;
            let generation = waiter.generation;
            drop(waiter);
            let replacement = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
            assert_eq!(replacement.slot, slot);
            assert_ne!(replacement.generation, generation);
            assert!(!VcsOperationAdmission::is_current(slot, generation));
            cell.release(owner.generation, None);
        }

        #[test]
        fn vcs_retained_live_source_has_no_nested_executor_or_guarded_await() {
            let source = include_str!("🦀️component.rs");
            let vcs = &source[source.find("pub mod vcs_integration").unwrap()..source.find("//#endregion 🔖️VersionGraph").unwrap()];
            let production = &vcs[..vcs.find("#[cfg(test)]\n    mod retained_tests").unwrap()];
            assert!(!production.contains("block_on("));
            assert!(!production.contains("submit_blocking"));
            assert!(!production.contains("ask_blocking"));
            assert!(!production.contains("loop {"));
            assert!(production.contains("VcsStoreAcquire"));
            assert!(production.contains("VcsStoreLease"));
            assert!(production.contains("std::mem::size_of::<HashMutation>()"));
            assert!(production.contains("let derived_author_items = request.authors.len();"));
            assert!(production.contains(".and_then(|value| value.checked_add(derived_author_items))"));
            assert!(production.contains("derived_author_owner_bytes"));
            assert!(production.contains("derived_author_id_bytes"));
            assert!(production.contains("let mutations = Vec::from([operation]);"));
            assert!(!production.contains("change.author.0.clone()"));
            assert!(!production.contains("mutations: vec![operation]"));
        }
    }
    //#endregion 🔖️Store
}
//#endregion 🔖️VersionGraph

//#region 🔖️VersionGraphs
// 🔀️ dedyn-fw-os-misc, O1/R11: closes `VersionGraph`'s 2-implementor set — `NullVersionGraph`
// always, `VcsVersionGraph` only when the `vcs` feature is on (mirrors the two `#[cfg]` branches
// `Database`'s constructors already had to pick between). `dyn_enum_close!`'s variant DSL has no
// per-variant `#[cfg]` (see `semio_framework_dispatch_macros`'s own `DynEnumVariant::parse`), so the
// whole closing site is duplicated per feature state instead of gating one variant inside it —
// still ONE concrete `VersionGraphs` type per build, never a generic thread through
// `ArtifactEngineConfig`/`ArtifactEngine`/`Database`. Replaces `Arc<dyn VersionGraph>`.
use crate::__semio_dispatch_VersionGraph;
use semio_framework_dispatch_macros::dyn_enum_close;

#[cfg(feature = "vcs")]
dyn_enum_close! {
    pub enum VersionGraphs: VersionGraph {
        Null(NullVersionGraph),
        Vcs(vcs_integration::VcsVersionGraph),
    }
}

#[cfg(not(feature = "vcs"))]
dyn_enum_close! {
    pub enum VersionGraphs: VersionGraph {
        Null(NullVersionGraph),
    }
}
//#endregion 🔖️VersionGraphs

//#region 🔖️Observe
/// @emoji 📡️ The default observability wiring `Database::open`/`open_at` build when the caller
/// doesn't supply their own: an in-memory `db_observe::StructuredSink` (real JSON-lines encoding,
/// just not flushed anywhere durable by default — a caller wanting file/pipe output constructs
/// `db_observe::WriterSink` themselves and passes it via `Database::open_with_emit`).
// 🔀️ dedyn-emit-runtime, O1/R1: concrete return type (`Database`'s `E` default matches it exactly),
// not `Arc<dyn Emit>` — every caller (`open`/`open_at`/`open_with_authz`) infers `E` from this value.
async fn default_emit() -> Arc<db_observe::StructuredSink<db_observe::MemorySink>> {
    Arc::new(db_observe::StructuredSink::new(db_observe::MemorySink::new()))
}
//#endregion 🔖️Observe

//#region 🔖️Catalog
/// @emoji 📇️ One document known to this `Database`'s catalog.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogEntry {
    pub document: protocol::ArtifactId,
    pub created_at_ms: u64,
}

/// @emoji 📇️ A point-in-time read of every document this `Database` knows about.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CatalogView {
    pub artifacts: Vec<CatalogEntry>,
}

/// @emoji 💾️ The catalog root's on-disk shape — a plain JSON array, deliberately NOT reusing
/// `CatalogEntry` directly (keeps the public type free of a `serde` bound it doesn't otherwise need).
#[derive(serde::Serialize, serde::Deserialize)]
struct CatalogRootEntry {
    document: String,
    created_at_ms: u64,
}

fn json_string_len(value: &str) -> usize {
    value
        .as_bytes()
        .iter()
        .map(|byte| match byte {
            b'"' | b'\\' | 0x08 | 0x0c | b'\n' | b'\r' | b'\t' => 2,
            0x00..=0x1f => 6,
            _ => 1,
        })
        .sum::<usize>()
        + 2
}

fn decimal_u64<'a>(value: u64, buffer: &'a mut [u8; 20]) -> &'a [u8] {
    let mut cursor = buffer.len();
    let mut remaining = value;
    loop {
        cursor -= 1;
        buffer[cursor] = b'0' + (remaining % 10) as u8;
        remaining /= 10;
        if remaining == 0 {
            return &buffer[cursor..];
        }
    }
}

async fn catalog_write(writer: &mut db_storage::DbIoPageWriter, bytes: &[u8]) -> Result<(), DbError> {
    let mut cursor = 0;
    while cursor < bytes.len() {
        cursor += writer.write_fragment(&bytes[cursor..])?;
        std::future::poll_fn(|context| {
            context.waker().wake_by_ref();
            std::task::Poll::Ready(())
        })
        .await;
    }
    Ok(())
}

async fn catalog_write_json_string(writer: &mut db_storage::DbIoPageWriter, value: &str) -> Result<(), DbError> {
    catalog_write(writer, b"\"").await?;
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in value.as_bytes() {
        let mut escape = [0u8; 6];
        let encoded: &[u8] = match *byte {
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
            _ => std::slice::from_ref(byte),
        };
        catalog_write(writer, encoded).await?;
    }
    catalog_write(writer, b"\"").await
}

async fn encode_catalog_pages(entries: &[CatalogEntry]) -> Result<db_storage::DbIoPages, DbError> {
    let mut encoded_len = 2usize;
    for (index, entry) in entries.iter().enumerate() {
        let mut decimal = [0u8; 20];
        encoded_len = encoded_len
            .checked_add(index.signum())
            .and_then(|length| length.checked_add(b"{\"document\":".len()))
            .and_then(|length| length.checked_add(json_string_len(&entry.document.0)))
            .and_then(|length| length.checked_add(b",\"created_at_ms\":".len()))
            .and_then(|length| length.checked_add(decimal_u64(entry.created_at_ms, &mut decimal).len() + 1))
            .ok_or(DbError::LimitExceeded("catalog encoded bytes"))?;
        std::future::poll_fn(|context| {
            context.waker().wake_by_ref();
            std::task::Poll::Ready(())
        })
        .await;
    }
    let mut writer = db_storage::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    catalog_write(&mut writer, b"[").await?;
    for (index, entry) in entries.iter().enumerate() {
        if index != 0 {
            catalog_write(&mut writer, b",").await?;
        }
        catalog_write(&mut writer, b"{\"document\":").await?;
        catalog_write_json_string(&mut writer, &entry.document.0).await?;
        catalog_write(&mut writer, b",\"created_at_ms\":").await?;
        let mut decimal = [0u8; 20];
        catalog_write(&mut writer, decimal_u64(entry.created_at_ms, &mut decimal)).await?;
        catalog_write(&mut writer, b"}").await?;
    }
    catalog_write(&mut writer, b"]").await?;
    writer.seal().map_err(db_storage::DbIoPageWriterRejected::into_error)
}

async fn decode_catalog(bytes: &[u8]) -> Result<Vec<CatalogEntry>, DbError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let raw: Vec<CatalogRootEntry> = serde_json::from_slice(bytes).map_err(|err| DbError::Corrupt(format!("catalog decode: {err}")))?;
    Ok(raw.into_iter().map(|entry| CatalogEntry { document: protocol::ArtifactId(entry.document), created_at_ms: entry.created_at_ms }).collect())
}

struct CatalogState {
    epoch: EpochFence,
    entries: Vec<CatalogEntry>,
}
//#endregion 🔖️Catalog

//#region 🔖️ArtifactSpec
/// @emoji 📄️ What `Database::create_document` needs to mint a brand-new document — this crate's own
/// choice (the contract fixes `create_document`'s signature, not `ArtifactSpec`'s shape).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactSpec {
    pub document: protocol::ArtifactId,
}

impl ArtifactSpec {
    pub async fn new(document: protocol::ArtifactId) -> ArtifactSpec {
        ArtifactSpec { document }
    }
}
//#endregion 🔖️ArtifactSpec

//#region 🔖️Health
/// @emoji 🩺️ The frozen `Database::health()` return shape, wrapping a real
/// `db_observe::HealthRegistry` snapshot plus this crate's own catalog-level fact (open document
/// count) that no lower crate could know.
#[derive(Clone, Debug)]
pub struct DbHealth {
    pub report: db_observe::HealthReport,
    pub open_artifacts: usize,
}
//#endregion 🔖️Health

//#region 🔖️Database
/// @emoji 🗄️ The catalog: owns the storage substrate, the shared config/capabilities/authz/
/// version-graph/observability wiring every document actor is constructed with, and the registry of
/// currently-open `ArtifactAuthority` actors.
///
/// 🎯️ Design choice: the catalog registry itself is a plain `Mutex`-guarded `HashMap`, not a
/// separate `db_actor::Actor`-driven process. `Database`'s own public surface (`open`/
/// `create_document`/`document`/`catalog`/`health`/`shutdown`) is already synchronous per the
/// frozen contract, and per-document concurrency is already provided by each `ArtifactAuthority`'s
/// own dedicated thread — the catalog only ever needs to serialize document-registry mutations and
/// a catalog-root CAS write, which a `Mutex` does directly without the mailbox's priority-lane/
/// backpressure machinery (that machinery matters for a document's WAL under load, not a rare
/// catalog-root swap).
// 🔀️ `A` is the pluggable `AuthzHook` implementation (see `db_artifact::ArtifactEngineConfig`'s own
// doc) — dedyn-fw-os-misc, R11(a): a caller-supplied, stored implementation is trivially generic;
// `AllowAll` default keeps every existing unparameterized `Database` reference (this crate's own
// `open`/`open_at`/`open_with_emit`, plus every external caller) compiling unchanged.
//
// 🔀️ `E` is the pluggable `Emit` sink — dedyn-emit-runtime, O1/R11(a): `open_with_emit`'s own doc
// ("a caller-supplied Emit sink, e.g. a `db_observe::WriterSink`") is exactly R11(a)'s "trivially
// generic" shape, the same pattern `A` above already uses. Default is `db_observe::StructuredSink<
// db_observe::MemorySink>` — the concrete type `default_emit()` has always constructed — so every
// existing unparameterized `Database`/`Database<A>` reference (this crate's own `open`/`open_at`/
// `open_with_authz`, plus `🌎️hub` and every other external caller, none of which ever names this
// type parameter) compiles unchanged. Replaces `Arc<dyn Emit>`.
pub struct Database<A: db_artifact::AuthzHook + 'static = db_artifact::AllowAll, E: Emit + 'static = db_observe::StructuredSink<db_observe::MemorySink>> {
    storage: Arc<db_storage::DbBackend>,
    config: DbConfig,
    capabilities: DbCapabilities,
    authz: Arc<A>,
    /// @emoji 🌿️ Never `None`: `NullVersionGraph` (an `Unimplemented`-on-every-call
    /// placeholder, not an `Option` layer — see its own doc) is the default when the `vcs` feature
    /// is disabled, exactly matching `db_artifact::ArtifactEngineConfig::default`'s own choice.
    version_graph: Arc<VersionGraphs>,
    emit: Arc<E>,
    health: Arc<db_observe::HealthRegistry>,
    catalog: Mutex<CatalogState>,
    open_artifacts: Mutex<HashMap<String, Arc<db_artifact::ArtifactAuthority>>>,
    /// @emoji 🧵️ The process WorkerPool every document authority and submit bridge uses.
    /// Construction without this owner is intentionally impossible: no database path may execute
    /// blocking storage or authority work inline on its caller.
    pool: Arc<WorkerPool>,
}

// 🚫️async: E5 executor bridge — every `Database` method below is plain sync and drives its async
// storage/`ArtifactEngine` calls via `db_actor::block_on` (R4 clause 2: this crate's own db-actor
// thread bridges are sanctioned; `Database` is the facade the prior `db-trait-flip` packet already
// classified as thread-owning alongside `db_artifact`, per its report's "db_engine (per-submit
// bridge threads)"). Every `.wal()`/`.snapshot()`/`.catalog()`/`.index()`/`.payload()`/`.lease()`
// accessor call is `.await`ed inside the SAME `block_on`, never a bare synchronous call.
impl Database<db_artifact::AllowAll> {
    /// @emoji 🚀️ The frozen entry point: opens (or initializes, if `storage` is fresh) a `Database`
    /// over an arbitrary `Arc<db_storage::DbBackend>` backend, wired with the default `AllowAll` authz and
    /// (behind the default-on `vcs` feature) a real `VcsVersionGraph`.
    pub async fn open(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>) -> Result<Database<db_artifact::AllowAll>, DbError> {
        Database::open_with(pool, config, storage, Arc::new(db_artifact::AllowAll), default_emit().await).await
    }

    /// @emoji 🚀️ The zero-touch filesystem entry point. The caller supplies the process pool
    /// before storage construction, so opening can never take a pool-less inline path.
    pub async fn open_at(pool: Arc<WorkerPool>, root: &std::path::Path, profile: Profile) -> Result<Database<db_artifact::AllowAll>, DbError> {
        let fs = db_storage::FsStorage::open(pool.clone(), root).await?;
        let storage: Arc<db_storage::DbBackend> = Arc::new(db_storage::DbBackend::Fs(fs));
        Database::open(pool, DbConfig::for_profile(profile), storage).await
    }

    /// @emoji 🚀️ Like `open`, but with a caller-supplied `Emit` sink (e.g. a `db_observe::WriterSink`
    /// over a real file) instead of the default in-memory one.
    // 🔀️ dedyn-emit-runtime, O1/R11(a): generic over `E: Emit` (the function's own type param, not
    // `Database`'s default) so the returned `Database<AllowAll, E>` carries the caller's concrete
    // sink type — this fn has zero callers anywhere in the repo today (public, documented extension
    // seam per `open_with_emit`'s own doc; matches `open_with_authz`'s identical shape below).
    pub async fn open_with_emit<E: Emit + 'static>(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, emit: Arc<E>) -> Result<Database<db_artifact::AllowAll, E>, DbError> {
        Database::open_with(pool, config, storage, Arc::new(db_artifact::AllowAll), emit).await
    }
}

impl<A: db_artifact::AuthzHook + 'static> Database<A> {
    /// @emoji 🚀️ Like `open`, but with a caller-supplied `AuthzHook` (e.g. `SecurityAuthzHook`)
    /// instead of the default `AllowAll`.
    pub async fn open_with_authz(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, authz: Arc<A>) -> Result<Database<A>, DbError> {
        Database::open_with(pool, config, storage, authz, default_emit().await).await
    }
}

// 🔀️ dedyn-emit-runtime, O1/R11(a): every method below reads/writes `self.emit`, so this whole
// block (previously `impl<A: AuthzHook + 'static> Database<A>`, default-`E` only) is now generic
// over `E: Emit` too. `open_with_authz` above stays in its own default-`E` block since it never
// takes an `emit` argument and must return the SAME default-`E` `Database<A>` every unparameterized
// caller expects — Rust resolves its `Database::open_with(..)` call by inferring `E` from
// `default_emit()`'s concrete return type regardless of which `impl` block `open_with` itself lives
// in, so the split is transparent to every call site.
impl<A: db_artifact::AuthzHook + 'static, E: Emit + 'static> Database<A, E> {
    /// 🧵️ Admits the exact storage owner before probing one backend capability future on I/O.
    pub fn open_retained(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>) -> Result<DatabaseCapabilityOpenFuture, DatabaseCapabilityOpenRejected> {
        DatabaseCapabilityOpenFuture::try_submit(pool, storage)
    }

    /// 🗂️ Admits the exact storage/root-key owners before one catalog backend read on I/O.
    pub fn open_catalog_read_retained(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>) -> Result<DatabaseCatalogReadFuture, DatabaseCatalogReadRejected> {
        DatabaseCatalogReadFuture::try_submit(pool, storage, DatabaseCatalogRootKey::root())
    }

    async fn open_with(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, authz: Arc<A>, emit: Arc<E>) -> Result<Database<A, E>, DbError> {
        let capability_probe = match Self::open_retained(pool.clone(), storage) {
            Ok(probe) => probe,
            Err(rejected) => return Err(rejected.close_and_take_error()),
        };
        let (storage, storage_capabilities) = capability_probe.await?.into_parts();
        let capabilities = DbCapabilities {
            // 🧩️ Extension seam: real, honest today — see module doc on why preview/live-query
            // aren't reachable through `ArtifactAuthority`'s current mailbox surface, and why
            // `db_cluster` is still an empty stub upstream of this wave.
            preview: false,
            historical_query: true,
            live_query: false,
            cluster: false,
            max_durability: std::cmp::min(storage_capabilities.max_durability, config.capabilities.max_durability),
        };

        let health = Arc::new(db_observe::HealthRegistry::new());
        health.set("db_engine.storage", if storage_capabilities.durable { db_observe::HealthState::Healthy } else { db_observe::HealthState::Degraded("storage backend is not durable".to_string()) });

        let catalog_probe = match Self::open_catalog_read_retained(pool.clone(), storage) {
            Ok(probe) => probe,
            Err(rejected) => return Err(rejected.close_and_take_error(pool.clone())),
        };
        let (storage, _catalog_key, catalog_root) = catalog_probe.await?.into_parts();
        let (epoch, entries) = match catalog_root? {
            Some((bytes, epoch)) => {
                let prepared = db_storage::db_io_prepare_platform(&bytes)?.await?;
                (epoch, decode_catalog(prepared.as_slice()).await?)
            }
            None => {
                let pages = encode_catalog_pages(&[]).await?;
                let epoch = db_actor::block_on(async { storage.catalog().await.cas_root(EpochFence::INITIAL, pages).await })?;
                (epoch, Vec::new())
            }
        };
        health.set("db_engine.catalog", db_observe::HealthState::Healthy);

        #[cfg(feature = "vcs")]
        let version_graph: Arc<VersionGraphs> = Arc::new(VersionGraphs::Vcs(vcs_integration::VcsVersionGraph::new().await));
        #[cfg(not(feature = "vcs"))]
        let version_graph: Arc<VersionGraphs> = Arc::new(VersionGraphs::Null(NullVersionGraph));

        emit.emit(EmitEvent::new("db_engine.database_opened").field("documents", EmitField::U64(entries.len() as u64))).await;

        Ok(Database { storage, config, capabilities, authz, version_graph, emit, health, catalog: Mutex::new(CatalogState { epoch, entries }), open_artifacts: Mutex::new(HashMap::new()), pool })
    }

    /// @emoji ⚙️ Builds one `ArtifactEngineConfig`. Sets the 4 fields this crate has ALWAYS
    /// constructed (`limits`/`authz`/`version_graph`/`preview_ttl_ms`, per the module doc's
    /// compatibility-surface note) explicitly, and spreads `..db_artifact::ArtifactEngineConfig::
    /// default()` for every other field db_artifact has since grown (e.g. `security`/`emit`/
    /// `projections`) — this crate has no opinion on those yet (`db_artifact`'s own real
    /// `db_security::SecurityGate`-backed default policy already matches `AllowAll`'s permissive
    /// single-tenant spirit), and the spread keeps this call site correct across further additive
    /// growth without another coordinated edit.
    async fn document_engine_config(&self) -> db_artifact::ArtifactEngineConfig<A, VersionGraphs> {
        // 🔀️ Can't `..db_artifact::ArtifactEngineConfig::default()` spread here: that default is
        // only defined for `ArtifactEngineConfig<AllowAll, NullVersionGraph>` (see its `impl
        // Default`), a different concrete type from `ArtifactEngineConfig<A, VersionGraphs>`
        // whenever this `Database<A>` was opened via `open_with_authz` with a non-`AllowAll` hook —
        // struct-update syntax requires an exact type match. Pull the `A`/`V`-independent defaults
        // (`security`/`emit`/`projections`) from the default instantiation by value instead.
        let other_defaults = db_artifact::ArtifactEngineConfig::default();
        db_artifact::ArtifactEngineConfig {
            limits: self.config.limits.clone(),
            authz: self.authz.clone(),
            version_graph: self.version_graph.clone(),
            preview_ttl_ms: self.config.limits.max_preview_ttl_ms,
            security: other_defaults.security,
            emit: other_defaults.emit,
            projections: other_defaults.projections,
        }
    }

    async fn spawn_authority_create(&self, document: protocol::ArtifactId) -> Result<Arc<db_artifact::ArtifactAuthority>, DbError> {
        let pool = self.pool.clone();
        let storage = self.storage.clone();
        let config = self.document_engine_config().await;
        let created_at_ms = now_ms().await;
        let mailbox_capacities = self.config.mailbox_capacities;
        let authority = db_artifact::ArtifactAuthority::spawn(pool, move || db_artifact::ArtifactEngine::create_retained(document, storage, config, created_at_ms), mailbox_capacities).await?;
        Ok(Arc::new(authority))
    }

    async fn spawn_authority_open(&self, document: protocol::ArtifactId) -> Result<Arc<db_artifact::ArtifactAuthority>, DbError> {
        let pool = self.pool.clone();
        let storage = self.storage.clone();
        let config = self.document_engine_config().await;
        let opened_at_ms = now_ms().await;
        let mailbox_capacities = self.config.mailbox_capacities;
        let authority = db_artifact::ArtifactAuthority::spawn(pool, move || async move { db_artifact::ArtifactEngine::open_retained(document, storage, config, opened_at_ms).await.map(|(engine, _report)| engine) }, mailbox_capacities).await?;
        Ok(Arc::new(authority))
    }

    async fn register_handle(&self, document: protocol::ArtifactId, authority: Arc<db_artifact::ArtifactAuthority>) -> ArtifactHandle {
        self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").insert(document.0.clone(), authority.clone());
        ArtifactHandle { authority, document, pool: self.pool.clone() }
    }

    /// @emoji 🌱️ The frozen `create_document`: mints a brand-new document, durably records it in the
    /// catalog root (CAS-fenced), spawns its `ArtifactAuthority`, and returns a live handle.
    pub async fn create_document(&self, spec: ArtifactSpec) -> Result<ArtifactHandle, DbError> {
        let document = spec.document;
        {
            let mut catalog = self.catalog.lock().expect("db_engine: catalog mutex poisoned");
            if catalog.entries.iter().any(|entry| entry.document == document) {
                return Err(DbError::AlreadyExists(format!("document {} already exists", document.0)));
            }
            let mut entries = catalog.entries.clone();
            let epoch = catalog.epoch;
            // 🔒️ A real `.await` reached while `catalog`'s guard is alive would extend its
            // temporary across this whole statement-block (R7), making the enclosing future
            // non-`Send` — needed for `semio-hub`'s axum handlers, not for `wasm32`, which has no
            // multi-threaded work-stealing scheduler to demand it. So: drive `commit` via
            // `db_actor::block_on` (the same bridge `cas_root` alone used to reach for) on every
            // target that DOES need `Send`, and via a plain `.await` — `db_actor::block_on` doesn't
            // exist for `wasm32` — on the one target that doesn't. Either way `entries`/`epoch` are
            // captured by reference, not moved, so this costs nothing beyond the `#[cfg]` split.
            let commit = async {
                entries.push(CatalogEntry { document: document.clone(), created_at_ms: now_ms().await });
                let pages = encode_catalog_pages(&entries).await?;
                self.storage.catalog().await.cas_root(epoch, pages).await
            };
            #[cfg(not(target_arch = "wasm32"))]
            let new_epoch = db_actor::block_on(commit)?;
            #[cfg(target_arch = "wasm32")]
            let new_epoch = commit.await?;
            catalog.epoch = new_epoch;
            catalog.entries = entries;
        }
        let authority = self.spawn_authority_create(document.clone()).await?;
        self.emit.emit(EmitEvent::new("db_engine.document_created").with_document(to_core_document_id(&document).await)).await;
        Ok(self.register_handle(document, authority).await)
    }

    /// @emoji 📄️ The frozen `document`: returns a live handle to an already-cataloged document,
    /// reusing an already-open `ArtifactAuthority` if one exists, else recovering it fresh from its
    /// WAL.
    pub async fn document(&self, id: &protocol::ArtifactId) -> Result<ArtifactHandle, DbError> {
        // 🔒️ `.cloned()` ends the guard's temporary scope at this `let`'s semicolon — under
        // edition-2021 rules an `if let` scrutinee's temporary would otherwise extend across the
        // `to_core_document_id(id).await` below, making this future non-`Send` (R7).
        let existing = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").get(&id.0).cloned();
        if let Some(authority) = existing {
            return Ok(ArtifactHandle { authority, document: id.clone(), pool: self.pool.clone() });
        }
        let known = self.catalog.lock().expect("db_engine: catalog mutex poisoned").entries.iter().any(|entry| &entry.document == id);
        if !known {
            return Err(DbError::NotFound(format!("document {} not found", id.0)));
        }
        let authority = self.spawn_authority_open(id.clone()).await?;
        self.emit.emit(EmitEvent::new("db_engine.document_opened").with_document(to_core_document_id(id).await)).await;
        Ok(self.register_handle(id.clone(), authority).await)
    }

    /// @emoji 📇️ The frozen `catalog`: a point-in-time read of every document this `Database`
    /// knows about.
    pub async fn catalog(&self) -> CatalogView {
        CatalogView { artifacts: self.catalog.lock().expect("db_engine: catalog mutex poisoned").entries.clone() }
    }

    /// @emoji 🩺️ The frozen `health`: this `Database`'s `HealthRegistry` snapshot plus its own open
    /// document count.
    pub async fn health(&self) -> DbHealth {
        DbHealth { report: self.health.report(), open_artifacts: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() }
    }

    /// @emoji 🚪️ The frozen `shutdown`: gracefully joins every open `ArtifactAuthority` this
    /// `Database` still exclusively owns.
    ///
    /// 🧩️ Extension seam: `deadline` is currently advisory — `db_artifact::ArtifactAuthority::shutdown`
    /// has no timeout parameter of its own (out of this crate's ownership to add this wave), so this
    /// always waits for a graceful join rather than forcing one after `deadline` elapses. A document
    /// whose `ArtifactHandle` is still cloned elsewhere (this `Arc`'s strong count > 1) is skipped —
    /// its actor thread keeps running under whichever handle still holds it, which is correct
    /// (shutdown must never yank a mailbox out from under a live caller), just not exhaustive.
    pub async fn shutdown(self, _deadline: std::time::Duration) -> Result<(), DbError> {
        let authorities: Vec<Arc<db_artifact::ArtifactAuthority>> = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").drain().map(|(_, authority)| authority).collect();
        for authority in authorities {
            if let Ok(authority) = Arc::try_unwrap(authority) {
                authority.shutdown().await;
            }
        }
        self.emit.emit(EmitEvent::new("db_engine.database_shutdown")).await;
        Ok(())
    }

    /// @emoji 🧰️ What this `Database` instance negotiated at `open` time.
    pub async fn capabilities(&self) -> DbCapabilities {
        self.capabilities
    }

    /// @emoji 🔌️ The underlying storage substrate this `Database` was opened with — an escape
    /// hatch for callers below the document-actor boundary that need direct `PayloadStorage`/
    /// `WalStorage` access (e.g. `os-semio_hub`'s content-addressed blob routes, or a wire-v2 semio_hub
    /// session driving `db_sync::handle_frontier_advertise` directly). Additive: not part of the
    /// contract-frozen `Database` API surface listed in `contract.md`'s "Stable API" block, so it
    /// carries no compatibility promise beyond this crate's own semver.
    pub async fn storage(&self) -> Arc<db_storage::DbBackend> {
        self.storage.clone()
    }

    /// @emoji 🧹️ A real, bounded `db_compact::Compactor` pass over `document` — WAL segment
    /// retention below its latest snapshot's `head_seq`, ref-traced payload GC, index compaction,
    /// and (if `consolidate_snapshots`) snapshot chain consolidation. See module doc: this IS a
    /// genuine `db_compact` integration, just document-at-a-time rather than a background scheduler
    /// (deferred — this wave's instructions ask for a lighter, documented cluster/compact/sync
    /// surface, not a full online scheduler).
    pub async fn compact_document(&self, document: &protocol::ArtifactId, holder: &str, consolidate_snapshots: bool) -> Result<db_compact::CompactionReport, DbError> {
        let core_document = to_core_document_id(document).await;
        db_actor::block_on(db_compact::Compactor::new(self.storage.as_ref()).await.run_from_latest_snapshot(&core_document, holder, consolidate_snapshots, &db_compact::CompactionBudget::default(), now_ms().await))
    }

    /// @emoji 👋️ A real `db_sync::handle_hello` call for `document` — the server-side half of the
    /// wire-v2 handshake (frontier exchange / bootstrap-plan decision). No transport of its own:
    /// wiring this to an actual `protocol_wire` socket is CW5/CW6's job (framework/sync, semio_hub
    /// rebuilds), out of this crate's scope this wave.
    pub async fn hello(&self, document: &protocol::ArtifactId, hello_frontier: Option<&protocol::RuntimeFrontierSummary>, session_id: String, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> Result<db_sync::WelcomeResponse, DbError> {
        let core_document = to_core_document_id(document);
        db_actor::block_on(db_sync::handle_hello(self.storage.as_ref(), core_document.await, hello_frontier, session_id, origin, snapshot_chunk_bytes))
    }

    /// @emoji 🌿️ A real, `vcs`-backed checkpoint over every change `record_change` has recorded for
    /// `document` since its last checkpoint (see `db_artifact::ArtifactEngine::submit`'s "vcs"
    /// pipeline stage, which calls `record_change` on every commit when a `VersionGraph` is wired).
    /// Errs `Unimplemented` if the `vcs` feature is disabled (no `VersionGraph` configured).
    pub async fn checkpoint_document(&self, document: &protocol::ArtifactId, message: String, authors: &[protocol::ActorId]) -> Result<String, DbError> {
        let core_document = to_core_document_id(document).await;
        let core_authors = authors.iter().map(to_core_actor_id).collect();
        self.version_graph.checkpoint(&core_document, CheckpointRequest { parent_checkpoint: None, change_ids: Vec::new(), message, authors: core_authors, timestamp_ms: now_ms().await }).await
    }
}
//#endregion 🔖️Database

//#region 🔖️ArtifactHandle
const ARTIFACT_SUBMIT_OPERATION_ITEMS: usize = 64;
const ARTIFACT_SUBMIT_PAGE_BYTES: u64 = 16 * 1024;
const ARTIFACT_SUBMIT_OPERATION_PAGES: u64 = 64;
const ARTIFACT_SUBMIT_OPERATION_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_OPERATION_PAGES;
const ARTIFACT_SUBMIT_TOTAL_PAGES: u64 = 1024;
const ARTIFACT_SUBMIT_TOTAL_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_TOTAL_PAGES;
const ARTIFACT_SUBMIT_BATCH_ITEMS: usize = 256;
const ARTIFACT_SUBMIT_NESTED_ITEMS: usize = 4096;
const ARTIFACT_SUBMIT_RETRY_MS: u64 = 1;
const ARTIFACT_SUBMIT_RETRY_LIMIT: u8 = 8;

#[derive(Clone, Copy)]
struct ArtifactSubmitAdmissionSlot {
    generation: u64,
    bytes: u64,
    items: usize,
    occupied: bool,
}

const EMPTY_ARTIFACT_SUBMIT_SLOT: ArtifactSubmitAdmissionSlot = ArtifactSubmitAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

struct ArtifactSubmitAdmissionState {
    slots: [ArtifactSubmitAdmissionSlot; ARTIFACT_SUBMIT_OPERATION_ITEMS],
    bytes: u64,
    next_generation: u64,
}

static ARTIFACT_SUBMIT_ADMISSION: std::sync::Mutex<ArtifactSubmitAdmissionState> = std::sync::Mutex::new(ArtifactSubmitAdmissionState { slots: [EMPTY_ARTIFACT_SUBMIT_SLOT; ARTIFACT_SUBMIT_OPERATION_ITEMS], bytes: 0, next_generation: 1 });

struct ArtifactSubmitAdmission {
    slot: usize,
    generation: u64,
    bytes: u64,
    items: usize,
}

impl ArtifactSubmitAdmission {
    fn try_claim(items: usize, bytes: u64) -> Result<Self, DbError> {
        if items == 0 || items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit item credit"));
        }
        if bytes == 0 || bytes > ARTIFACT_SUBMIT_OPERATION_BYTES {
            return Err(DbError::LimitExceeded("artifact submit operation byte credit"));
        }
        let mut state = ARTIFACT_SUBMIT_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable("artifact submit item capacity exhausted".to_string()));
        };
        if state.bytes.checked_add(bytes).is_none_or(|next| next > ARTIFACT_SUBMIT_TOTAL_BYTES) {
            return Err(DbError::Unavailable("artifact submit byte capacity exhausted".to_string()));
        }
        let generation = state.next_generation;
        state.next_generation = state.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("artifact submit generation"))?;
        state.slots[slot] = ArtifactSubmitAdmissionSlot { generation, bytes, items, occupied: true };
        state.bytes += bytes;
        Ok(Self { slot, generation, bytes, items })
    }
}

impl Drop for ArtifactSubmitAdmission {
    fn drop(&mut self) {
        let mut state = ARTIFACT_SUBMIT_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let entry = &mut state.slots[self.slot];
        if !entry.occupied || entry.generation != self.generation || entry.bytes != self.bytes || entry.items != self.items {
            return;
        }
        *entry = EMPTY_ARTIFACT_SUBMIT_SLOT;
        state.bytes = state.bytes.checked_sub(self.bytes).expect("artifact submit byte credit underflow");
    }
}

fn artifact_submit_credit(batch: &db_artifact::CommandBatch) -> Result<(usize, u64), DbError> {
    if batch.envelopes.is_empty() || batch.envelopes.len() > ARTIFACT_SUBMIT_BATCH_ITEMS {
        return Err(DbError::LimitExceeded("artifact submit batch item credit"));
    }
    let mut items = batch.envelopes.len();
    let mut bytes = ARTIFACT_SUBMIT_PAGE_BYTES;
    let envelope_owner_bytes = batch.envelopes.capacity().checked_mul(std::mem::size_of::<protocol::MutationEnvelope>()).ok_or(DbError::LimitExceeded("artifact submit envelope owner bytes"))?;
    bytes = bytes.checked_add(envelope_owner_bytes as u64).ok_or(DbError::LimitExceeded("artifact submit envelope owner bytes"))?;
    for envelope in &batch.envelopes {
        items = items.checked_add(envelope.dependencies.len()).ok_or(DbError::LimitExceeded("artifact submit nested items"))?;
        if items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit nested item credit"));
        }
        let dependency_owner_bytes = envelope.dependencies.capacity().checked_mul(std::mem::size_of::<protocol::MutationId>()).ok_or(DbError::LimitExceeded("artifact submit dependency owner bytes"))?;
        bytes = bytes
            .checked_add(envelope.mutation_id.0.capacity() as u64)
            .and_then(|value| value.checked_add(envelope.document_id.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.actor.0.capacity() as u64))
            .and_then(|value| value.checked_add(dependency_owner_bytes as u64))
            .and_then(|value| value.checked_add(envelope.diff.schema.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.diff.payload.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.inverse.schema.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.inverse.payload.capacity() as u64))
            .ok_or(DbError::LimitExceeded("artifact submit nested byte credit"))?;
        for dependency in &envelope.dependencies {
            bytes = bytes.checked_add(dependency.0.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit dependency byte credit"))?;
        }
    }
    let pages = bytes.checked_add(ARTIFACT_SUBMIT_PAGE_BYTES - 1).ok_or(DbError::LimitExceeded("artifact submit page rounding"))? / ARTIFACT_SUBMIT_PAGE_BYTES;
    let admitted = pages.checked_mul(ARTIFACT_SUBMIT_PAGE_BYTES).ok_or(DbError::LimitExceeded("artifact submit page credit"))?;
    if admitted > ARTIFACT_SUBMIT_OPERATION_BYTES {
        return Err(DbError::LimitExceeded("artifact submit operation byte credit"));
    }
    Ok((items, admitted))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubmitProgress {
    Admitted,
    Scheduled,
    Waiting,
    Completed,
    Cancelled,
    Fault,
}

type ArtifactActorSubmitFuture = db_actor::AskFuture<db_artifact::ArtifactMessage, Result<db_artifact::CommandReceipt, DbError>>;
pub type ArtifactSubmitOutcome = Result<Result<CommandReceipt, DbError>, DbError>;

enum ArtifactSubmitWorkOwner {
    Request { batch: db_artifact::CommandBatch, options: db_artifact::SubmitOptions, submitted_at_ms: u64 },
    Actor(ArtifactActorSubmitFuture),
}

struct ArtifactSubmitState {
    pool: WorkerPool,
    authority: Arc<db_artifact::ArtifactAuthority>,
    document: protocol::ArtifactId,
    generation: u64,
    authority_generation: db_ids::GenerationId,
    admission: std::sync::Mutex<Option<ArtifactSubmitAdmission>>,
    work: std::sync::Mutex<Option<ArtifactSubmitWorkOwner>>,
    completion: std::sync::Mutex<Option<ArtifactSubmitOutcome>>,
    terminal_work: std::sync::Mutex<Option<ArtifactSubmitWorkOwner>>,
    terminal_result: std::sync::Mutex<Option<ArtifactSubmitOutcome>>,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    terminal_job: std::sync::Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
    retry_armed: std::sync::atomic::AtomicBool,
    retry_generation: std::sync::atomic::AtomicU64,
    scheduled: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    abandoned: std::sync::atomic::AtomicBool,
    finished: std::sync::atomic::AtomicBool,
    progress: std::sync::atomic::AtomicU8,
}

pub struct SubmitFuture {
    state: Arc<ArtifactSubmitState>,
    resolved: bool,
}

pub struct ArtifactSubmitTerminalJob {
    state: Arc<ArtifactSubmitState>,
    owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

pub struct ArtifactSubmitTerminalWork {
    state: Arc<ArtifactSubmitState>,
    owner: Option<ArtifactSubmitWorkOwner>,
}

struct ArtifactSubmitWake {
    state: std::sync::Weak<ArtifactSubmitState>,
    generation: u64,
}

impl std::task::Wake for ArtifactSubmitWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            if self.generation == state.generation {
                state.schedule();
            }
        }
    }
}

impl ArtifactSubmitState {
    fn set_progress(&self, progress: SubmitProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    fn wake_waiter(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    fn finish(&self) {
        if !self.finished.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    fn finish_if_terminal_empty(&self) {
        if self.terminal_is_empty() && !self.scheduled.load(std::sync::atomic::Ordering::Acquire) && !self.retry_armed.load(std::sync::atomic::Ordering::Acquire) {
            self.finish();
        }
    }

    fn complete(&self, result: ArtifactSubmitOutcome, progress: SubmitProgress) {
        self.set_progress(progress);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            self.wake_waiter();
        }
    }

    fn terminalize_work(&self, result: ArtifactSubmitOutcome, progress: SubmitProgress) {
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        }
        self.complete(result, progress);
    }

    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.set_progress(SubmitProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated if attempt < ARTIFACT_SUBMIT_RETRY_LIMIT => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                    self.arm_retry();
                }
                kind => {
                    let job = error.into_job();
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                    }
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                    self.complete(Err(DbError::Unavailable(format!("artifact submit WorkerPool submission failed: {kind:?}"))), SubmitProgress::Fault);
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.fetch_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact submit retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact submit retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_SUBMIT_RETRY_MS) else {
            self.terminalize_retry_authority("artifact submit retry deadline exhausted");
            return;
        };
        let state = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != state.retry_generation.load(Ordering::Acquire) {
                return;
            }
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if state.cancelled.load(Ordering::Acquire) {
                    drop(job);
                    state.scheduled.store(false, Ordering::Release);
                    state.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
                } else {
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    fn terminalize_retry_authority(&self, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            let mut terminal = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
            } else {
                drop(terminal);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        self.terminalize_work(Err(DbError::Unavailable(detail.to_string())), SubmitProgress::Fault);
    }

    fn drive_one(self: Arc<Self>, generation: u64) {
        use std::future::Future as _;
        use std::sync::atomic::Ordering;

        if generation != self.generation {
            return;
        }
        if self.authority.generation() != self.authority_generation {
            self.scheduled.store(false, Ordering::Release);
            self.terminalize_work(Err(DbError::StaleGeneration { expected: self.authority.generation(), actual: self.authority_generation }), SubmitProgress::Fault);
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.cancelled.load(Ordering::Acquire) {
            self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
            return;
        }

        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if matches!(work.as_ref(), Some(ArtifactSubmitWorkOwner::Request { .. })) {
            let Some(ArtifactSubmitWorkOwner::Request { batch, options, submitted_at_ms }) = work.take() else {
                return;
            };
            *work = Some(ArtifactSubmitWorkOwner::Actor(self.authority.submit_retained(batch, options, submitted_at_ms)));
            drop(work);
            self.schedule();
            return;
        }

        let Some(ArtifactSubmitWorkOwner::Actor(future)) = work.as_mut() else {
            return;
        };
        let waker = std::task::Waker::from(Arc::new(ArtifactSubmitWake { state: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);
        match std::pin::Pin::new(future).poll(&mut context) {
            std::task::Poll::Pending => {
                self.set_progress(SubmitProgress::Waiting);
            }
            std::task::Poll::Ready(result) => {
                work.take();
                drop(work);
                let result = result.map(|inner| inner.map(|receipt| to_engine_receipt(receipt, self.document.clone())));
                if self.cancelled.load(Ordering::Acquire) {
                    *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                    self.complete(Err(DbError::Closed), SubmitProgress::Cancelled);
                } else {
                    self.complete(result, SubmitProgress::Completed);
                }
            }
        }
    }

    fn close_one(&self) -> bool {
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return true;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return true;
        }
        if let Some(work) = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(work);
            return true;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return true;
        }
        false
    }
}

impl SubmitFuture {
    fn submit(handle: &ArtifactHandle, batch: db_artifact::CommandBatch, options: db_artifact::SubmitOptions) -> Self {
        let credit = artifact_submit_credit(&batch).and_then(|(items, bytes)| ArtifactSubmitAdmission::try_claim(items, bytes));
        let admission_error = credit.as_ref().err().map(ToString::to_string);
        let generation = credit.as_ref().map_or(0, |admission| admission.generation);
        let request = ArtifactSubmitWorkOwner::Request { batch, options, submitted_at_ms: handle.pool.now_ms() };
        let (work, terminal_work) = if generation == 0 { (None, Some(request)) } else { (Some(request), None) };
        let state = Arc::new(ArtifactSubmitState {
            pool: handle.pool.as_ref().clone(),
            authority: handle.authority.clone(),
            document: handle.document.clone(),
            generation,
            authority_generation: handle.authority.generation(),
            admission: std::sync::Mutex::new(credit.ok()),
            work: std::sync::Mutex::new(work),
            completion: std::sync::Mutex::new(None),
            terminal_work: std::sync::Mutex::new(terminal_work),
            terminal_result: std::sync::Mutex::new(None),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            waker: std::sync::Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            progress: std::sync::atomic::AtomicU8::new(if generation == 0 { SubmitProgress::Fault as u8 } else { SubmitProgress::Admitted as u8 }),
        });
        if generation == 0 {
            *state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(DbError::Unavailable(admission_error.unwrap_or_else(|| "artifact submit admission exhausted".to_string()))));
        } else {
            state.schedule();
        }
        Self { state, resolved: false }
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> SubmitProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => SubmitProgress::Admitted,
            1 => SubmitProgress::Scheduled,
            2 => SubmitProgress::Waiting,
            3 => SubmitProgress::Completed,
            4 => SubmitProgress::Cancelled,
            _ => SubmitProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        if matches!(self.progress(), SubmitProgress::Completed | SubmitProgress::Cancelled | SubmitProgress::Fault) {
            return;
        }
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.schedule();
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactSubmitTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactSubmitTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactSubmitTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactSubmitTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactSubmitOutcome> {
        let result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority.take_terminal_job()
    }

    pub fn close_step(&self) -> bool {
        let progressed = self.state.close_one() || self.state.authority.close_step();
        self.state.finish_if_terminal_empty();
        progressed
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority.terminal_is_empty()
    }
}

impl Future for SubmitFuture {
    type Output = ArtifactSubmitOutcome;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        std::task::Poll::Pending
    }
}

impl Drop for SubmitFuture {
    fn drop(&mut self) {
        if !self.resolved {
            self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.state.schedule();
        }
        self.state.close_one();
        self.state.finish_if_terminal_empty();
    }
}

impl ArtifactSubmitTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact submit job already resolved").0
    }

    pub fn resume(mut self) {
        let (_, job) = self.owner.take().expect("terminal artifact submit job already resolved");
        if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let work = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = work;
        }
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(SubmitProgress::Scheduled);
        self.state.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.state.submit_exact(job, 0);
    }

    pub fn close(mut self) {
        let (_, job) = self.owner.take().expect("terminal artifact submit job already resolved");
        drop(job);
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactSubmitTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

impl ArtifactSubmitTerminalWork {
    pub fn resume(mut self) -> Result<(), Self> {
        if self.state.generation == 0 {
            return Err(self);
        }
        let owner = self.owner.take().expect("terminal artifact submit work already resolved");
        *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(SubmitProgress::Admitted);
        if self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(())
    }

    pub fn close(mut self) {
        drop(self.owner.take().expect("terminal artifact submit work already resolved"));
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactSubmitTerminalWork {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

const ARTIFACT_HISTORY_OPERATION_SLOTS: usize = 8;
const ARTIFACT_HISTORY_PAGE_BYTES: u64 = 16 * 1024;
const ARTIFACT_HISTORY_OPERATION_PAGES: u64 = 2_048;
const ARTIFACT_HISTORY_OPERATION_BYTES: u64 = ARTIFACT_HISTORY_PAGE_BYTES * ARTIFACT_HISTORY_OPERATION_PAGES;
const ARTIFACT_HISTORY_TOTAL_BYTES: u64 = ARTIFACT_HISTORY_OPERATION_BYTES * ARTIFACT_HISTORY_OPERATION_SLOTS as u64;
const ARTIFACT_HISTORY_OPERATION_ITEMS: usize = 20_481;

#[derive(Clone, Copy)]
struct ArtifactHistoryAdmissionSlot {
    generation: u64,
    bytes: u64,
    items: usize,
    occupied: bool,
}

const EMPTY_ARTIFACT_HISTORY_SLOT: ArtifactHistoryAdmissionSlot = ArtifactHistoryAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

struct ArtifactHistoryAdmissionState {
    slots: [ArtifactHistoryAdmissionSlot; ARTIFACT_HISTORY_OPERATION_SLOTS],
    bytes: u64,
    next_generation: u64,
}

static ARTIFACT_HISTORY_ADMISSION: std::sync::Mutex<ArtifactHistoryAdmissionState> = std::sync::Mutex::new(ArtifactHistoryAdmissionState { slots: [EMPTY_ARTIFACT_HISTORY_SLOT; ARTIFACT_HISTORY_OPERATION_SLOTS], bytes: 0, next_generation: 1 });

struct ArtifactHistoryAdmission {
    slot: usize,
    generation: u64,
    reservation: Option<db_artifact::HistoryReplayReservation>,
}

enum ArtifactHistoryAdmissionError {
    Rejected(DbError),
    Construction { admission: ArtifactHistoryAdmission, fault: db_artifact::HistoryReplayReservationConstructionFault },
}

impl std::fmt::Debug for ArtifactHistoryAdmissionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rejected(error) => formatter.debug_tuple("Rejected").field(error).finish(),
            Self::Construction { fault, .. } => formatter.debug_struct("Construction").field("fault", fault).finish(),
        }
    }
}

impl ArtifactHistoryAdmission {
    fn try_claim() -> Result<Self, ArtifactHistoryAdmissionError> {
        let (slot, generation) = {
            let mut state = ARTIFACT_HISTORY_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable("artifact history item capacity exhausted".to_string())));
            };
            let Some(next_bytes) = state.bytes.checked_add(ARTIFACT_HISTORY_OPERATION_BYTES) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::LimitExceeded("artifact history aggregate bytes")));
            };
            if next_bytes > ARTIFACT_HISTORY_TOTAL_BYTES {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable("artifact history byte capacity exhausted".to_string())));
            }
            let generation = state.next_generation;
            let Some(next_generation) = state.next_generation.checked_add(1) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::LimitExceeded("artifact history generation")));
            };
            state.next_generation = next_generation;
            state.slots[slot] = ArtifactHistoryAdmissionSlot { generation, bytes: ARTIFACT_HISTORY_OPERATION_BYTES, items: ARTIFACT_HISTORY_OPERATION_ITEMS, occupied: true };
            state.bytes = next_bytes;
            (slot, generation)
        };
        let mut claim = Self { slot, generation, reservation: None };
        match db_artifact::HistoryReplayReservation::try_new() {
            Ok(reservation) => {
                claim.reservation = Some(reservation);
                Ok(claim)
            }
            Err(fault) => Err(ArtifactHistoryAdmissionError::Construction { admission: claim, fault }),
        }
    }

    fn take_reservation(&mut self) -> Option<db_artifact::HistoryReplayReservation> {
        self.reservation.take()
    }

    fn begin_reservation_close(&mut self) -> Option<db_artifact::HistoryReplayReservationCloseCursor> {
        self.reservation.take().map(db_artifact::HistoryReplayReservationCloseCursor::new)
    }

    fn restore_reservation(&mut self, reservation: db_artifact::HistoryReplayReservation) {
        assert!(self.reservation.is_none(), "artifact history reservation restore slot was occupied");
        self.reservation = Some(reservation);
    }
}

impl Drop for ArtifactHistoryAdmission {
    fn drop(&mut self) {
        assert!(self.reservation.is_none(), "artifact history admission dropped a live replay reservation");
        let mut state = ARTIFACT_HISTORY_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let entry = &mut state.slots[self.slot];
        if !entry.occupied || entry.generation != self.generation || entry.bytes != ARTIFACT_HISTORY_OPERATION_BYTES || entry.items != ARTIFACT_HISTORY_OPERATION_ITEMS {
            return;
        }
        *entry = EMPTY_ARTIFACT_HISTORY_SLOT;
        state.bytes = state.bytes.checked_sub(ARTIFACT_HISTORY_OPERATION_BYTES).expect("artifact history byte credit underflow");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryProgress {
    Admitted,
    Scheduled,
    Waiting,
    Mapping,
    Completed,
    Cancelled,
    Fault,
}

type ArtifactActorHistoryFuture = db_actor::AskFuture<db_artifact::ArtifactMessage, Result<db_artifact::ArtifactHistoryView, DbError>>;
pub type ArtifactHistoryOutcome = Result<HistoryView, DbError>;

enum ArtifactHistoryWorkOwner {
    Request,
    Actor(ArtifactActorHistoryFuture),
}

struct ArtifactHistoryState {
    pool: WorkerPool,
    authority: Arc<db_artifact::ArtifactAuthority>,
    generation: u64,
    authority_generation: db_ids::GenerationId,
    admission: std::sync::Mutex<Option<ArtifactHistoryAdmission>>,
    work: std::sync::Mutex<Option<ArtifactHistoryWorkOwner>>,
    completion: std::sync::Mutex<Option<ArtifactHistoryOutcome>>,
    terminal_work: std::sync::Mutex<Option<ArtifactHistoryWorkOwner>>,
    terminal_result: std::sync::Mutex<Option<ArtifactHistoryOutcome>>,
    terminal_reservation: std::sync::Mutex<Option<db_artifact::HistoryReplayReservationCloseCursor>>,
    reservation_checked_out: std::sync::atomic::AtomicBool,
    terminal_construction: std::sync::Mutex<Option<db_artifact::HistoryReplayReservationConstructionFault>>,
    construction_checked_out: std::sync::atomic::AtomicBool,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    terminal_job: std::sync::Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
    retry_armed: std::sync::atomic::AtomicBool,
    retry_generation: std::sync::atomic::AtomicU64,
    scheduled: std::sync::atomic::AtomicBool,
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    abandoned: std::sync::atomic::AtomicBool,
    finished: std::sync::atomic::AtomicBool,
    progress: std::sync::atomic::AtomicU8,
}

pub struct HistoryFuture {
    state: Arc<ArtifactHistoryState>,
    resolved: bool,
}

pub struct ArtifactHistoryTerminalHandle {
    state: Arc<ArtifactHistoryState>,
}

pub struct ArtifactHistoryTerminalJob {
    state: Arc<ArtifactHistoryState>,
    owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

pub struct ArtifactHistoryTerminalWork {
    state: Arc<ArtifactHistoryState>,
    owner: Option<ArtifactHistoryWorkOwner>,
}

pub struct ArtifactHistoryTerminalReservation {
    state: Arc<ArtifactHistoryState>,
    owner: Option<db_artifact::HistoryReplayReservationCloseCursor>,
}

pub struct ArtifactHistoryTerminalConstructionFault {
    state: Arc<ArtifactHistoryState>,
    owner: Option<db_artifact::HistoryReplayReservationConstructionFault>,
}

struct ArtifactHistoryWake {
    state: std::sync::Weak<ArtifactHistoryState>,
    generation: u64,
}

fn artifact_history_registry() -> &'static std::sync::Mutex<[Option<Arc<ArtifactHistoryState>>; ARTIFACT_HISTORY_OPERATION_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<[Option<Arc<ArtifactHistoryState>>; ARTIFACT_HISTORY_OPERATION_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::array::from_fn(|_| None)))
}

fn register_artifact_history(state: &Arc<ArtifactHistoryState>) {
    if state.generation == 0 {
        return;
    }
    let mut registry = artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if registry.iter().any(|slot| slot.as_ref().is_some_and(|registered| registered.generation == state.generation)) {
        return;
    }
    if let Some(slot) = registry.iter_mut().find(|slot| slot.is_none()) {
        *slot = Some(state.clone());
    }
}

fn unregister_artifact_history(generation: u64) {
    let mut registry = artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(slot) = registry.iter_mut().find(|slot| slot.as_ref().is_some_and(|state| state.generation == generation)) {
        *slot = None;
    }
}

impl std::task::Wake for ArtifactHistoryWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            if self.generation == state.generation {
                state.schedule();
            }
        }
    }
}

impl ArtifactHistoryState {
    fn set_progress(&self, progress: HistoryProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    fn wake_waiter(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    fn take_terminal_reservation(self: &Arc<Self>) -> Option<ArtifactHistoryTerminalReservation> {
        use std::sync::atomic::Ordering;
        if self.reservation_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let owner = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if owner.is_none() {
            self.reservation_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(ArtifactHistoryTerminalReservation { state: self.clone(), owner })
    }

    fn take_terminal_construction_fault(self: &Arc<Self>) -> Option<ArtifactHistoryTerminalConstructionFault> {
        use std::sync::atomic::Ordering;
        if self.construction_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let owner = self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if owner.is_none() {
            self.construction_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(ArtifactHistoryTerminalConstructionFault { state: self.clone(), owner })
    }

    fn terminal_roots_are_empty(&self) -> bool {
        self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.construction_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire)
            && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_roots_are_empty()
            && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
            && !self.retry_armed.load(std::sync::atomic::Ordering::Acquire)
    }

    fn finish_if_terminal_empty(&self) -> bool {
        use std::sync::atomic::Ordering;
        if !self.terminal_roots_are_empty() || self.scheduled.load(Ordering::Acquire) || self.retry_armed.load(Ordering::Acquire) {
            return false;
        }
        let mut admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.finished.load(Ordering::Acquire) {
            return false;
        }
        let owner = admission.take();
        assert!(owner.as_ref().is_none_or(|owner| owner.reservation.is_none()), "artifact history admission finished before reservation retirement");
        drop(owner);
        unregister_artifact_history(self.generation);
        self.finished.store(true, Ordering::Release);
        true
    }

    fn begin_unhanded_reservation_close(&self) {
        if self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() || self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let cursor = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(ArtifactHistoryAdmission::begin_reservation_close);
        if let Some(cursor) = cursor {
            *self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(cursor);
        }
    }

    fn resume_unhanded_reservation(&self) -> bool {
        if self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire) {
            return false;
        }
        let mut terminal = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(cursor) = terminal.as_mut() else {
            return self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| owner.reservation.is_some());
        };
        let mut admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if admission.as_ref().is_none_or(|owner| owner.reservation.is_some()) {
            return false;
        }
        let Some(reservation) = cursor.resume() else {
            return false;
        };
        let admission = admission.as_mut().expect("artifact history reservation admission disappeared after preflight");
        admission.restore_reservation(reservation);
        terminal.take();
        true
    }

    fn complete(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) {
        self.set_progress(progress);
        let mut completion = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            drop(completion);
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *completion = Some(result);
            drop(completion);
            self.wake_waiter();
        }
    }

    fn terminalize_work(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) {
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            if matches!(&work, ArtifactHistoryWorkOwner::Request) {
                self.begin_unhanded_reservation_close();
            }
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        }
        self.complete(result, progress);
    }

    fn terminalize_unhanded_request(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) -> bool {
        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !work.as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request)) {
            return false;
        }
        let request = work.take().expect("artifact history request disappeared after retained preflight");
        drop(work);
        self.begin_unhanded_reservation_close();
        *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(request);
        self.complete(result, progress);
        true
    }

    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire)
            || self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.construction_checked_out.load(Ordering::Acquire)
            || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err()
        {
            return;
        }
        self.set_progress(HistoryProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated if attempt < ARTIFACT_SUBMIT_RETRY_LIMIT => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                    self.arm_retry();
                }
                kind => {
                    let job = error.into_job();
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                    self.terminalize_work(Err(DbError::Unavailable(format!("artifact history WorkerPool submission failed: {kind:?}"))), HistoryProgress::Fault);
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.fetch_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact history retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact history retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_SUBMIT_RETRY_MS) else {
            self.terminalize_retry_authority("artifact history retry deadline exhausted");
            return;
        };
        let state = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != state.retry_generation.load(Ordering::Acquire) {
                return;
            }
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if state.cancelled.load(Ordering::Acquire) {
                    state.scheduled.store(false, Ordering::Release);
                    *state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                    state.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled);
                } else {
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    fn terminalize_retry_authority(&self, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            let mut terminal = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
            } else {
                drop(terminal);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        self.terminalize_work(Err(DbError::Unavailable(detail.to_string())), HistoryProgress::Fault);
    }

    fn drive_one(self: Arc<Self>, generation: u64) {
        use std::future::Future as _;
        use std::sync::atomic::Ordering;

        if generation != self.generation {
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.authority.generation() != self.authority_generation {
            self.cancelled.store(true, Ordering::Release);
            let actor_active = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Actor(_)));
            if !actor_active {
                self.terminalize_work(Err(DbError::StaleGeneration { expected: self.authority.generation(), actual: self.authority_generation }), HistoryProgress::Fault);
                return;
            }
        }
        if self.cancelled.load(Ordering::Acquire) {
            let actor_active = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Actor(_)));
            if !actor_active {
                self.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled);
                return;
            }
        }

        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match work.as_mut() {
            Some(ArtifactHistoryWorkOwner::Request) => {
                let reservation = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(ArtifactHistoryAdmission::take_reservation);
                let Some(reservation) = reservation else {
                    drop(work);
                    self.terminalize_work(Err(DbError::Unavailable("artifact history fixed reservation owner is missing".to_string())), HistoryProgress::Fault);
                    return;
                };
                *work = Some(ArtifactHistoryWorkOwner::Actor(self.authority.history_retained(self.generation, self.cancelled.clone(), reservation)));
                drop(work);
                self.schedule();
            }
            Some(ArtifactHistoryWorkOwner::Actor(future)) => {
                let waker = std::task::Waker::from(Arc::new(ArtifactHistoryWake { state: Arc::downgrade(&self), generation }));
                let mut context = std::task::Context::from_waker(&waker);
                match std::pin::Pin::new(future).poll(&mut context) {
                    std::task::Poll::Pending => self.set_progress(HistoryProgress::Waiting),
                    std::task::Poll::Ready(Ok(Ok(view))) => {
                        work.take();
                        drop(work);
                        let admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                        match admission {
                            Some(admission) => self.complete(Ok(HistoryView::new(view, Some(admission), self.clone())), HistoryProgress::Completed),
                            None => self.complete(Ok(HistoryView::new(view, None, self.clone())), HistoryProgress::Fault),
                        }
                    }
                    std::task::Poll::Ready(Ok(Err(error))) => {
                        work.take();
                        drop(work);
                        self.complete(Err(error), HistoryProgress::Fault);
                    }
                    std::task::Poll::Ready(Err(error)) => {
                        work.take();
                        drop(work);
                        self.complete(Err(error), HistoryProgress::Fault);
                    }
                }
            }
            None => {}
        }
    }

    fn close_one(self: &Arc<Self>) -> bool {
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return true;
        }
        if self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return true;
        }
        if let Some(work) = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            match work {
                ArtifactHistoryWorkOwner::Request => self.begin_unhanded_reservation_close(),
                ArtifactHistoryWorkOwner::Actor(owner) => {
                    *self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ArtifactHistoryWorkOwner::Actor(owner));
                    self.cancelled.store(true, std::sync::atomic::Ordering::Release);
                    self.schedule();
                }
            }
            return true;
        }
        let mut reservation = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cursor) = reservation.as_mut() {
            if cursor.close_step() {
                return true;
            }
            if cursor.terminal_is_empty() {
                reservation.take();
                return true;
            }
        }
        drop(reservation);
        let mut construction = self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cursor) = construction.as_mut() {
            if cursor.close_step() {
                return true;
            }
            if cursor.terminal_is_empty() {
                construction.take();
                return true;
            }
        }
        drop(construction);
        let mut terminal = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(result) = terminal.as_mut() {
            match result {
                Ok(view) if view.close_step() => return true,
                Ok(_) | Err(_) => {
                    terminal.take();
                    return true;
                }
            }
        }
        false
    }
}

impl HistoryFuture {
    fn submit(handle: &ArtifactHandle) -> Self {
        let (admission, construction_fault, rejected_error, generation) = match ArtifactHistoryAdmission::try_claim() {
            Ok(admission) => {
                let generation = admission.generation;
                (Some(admission), None, None, generation)
            }
            Err(ArtifactHistoryAdmissionError::Construction { admission, fault }) => {
                let generation = admission.generation;
                (Some(admission), Some(fault), None, generation)
            }
            Err(ArtifactHistoryAdmissionError::Rejected(error)) => (None, None, Some(error), 0),
        };
        let ready = construction_fault.is_none() && rejected_error.is_none();
        let state = Arc::new(ArtifactHistoryState {
            pool: handle.pool.as_ref().clone(),
            authority: handle.authority.clone(),
            generation,
            authority_generation: handle.authority.generation(),
            admission: std::sync::Mutex::new(admission),
            work: std::sync::Mutex::new(ready.then_some(ArtifactHistoryWorkOwner::Request)),
            completion: std::sync::Mutex::new(None),
            terminal_work: std::sync::Mutex::new(None),
            terminal_result: std::sync::Mutex::new(None),
            terminal_reservation: std::sync::Mutex::new(None),
            reservation_checked_out: std::sync::atomic::AtomicBool::new(false),
            terminal_construction: std::sync::Mutex::new(construction_fault),
            construction_checked_out: std::sync::atomic::AtomicBool::new(false),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            waker: std::sync::Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            progress: std::sync::atomic::AtomicU8::new(if generation == 0 { HistoryProgress::Fault as u8 } else { HistoryProgress::Admitted as u8 }),
        });
        register_artifact_history(&state);
        let construction_error = state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(db_artifact::HistoryReplayReservationConstructionFault::take_error);
        if let Some(error) = rejected_error.or(construction_error) {
            *state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(error));
        } else {
            state.schedule();
        }
        Self { state, resolved: false }
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn terminal_handle(&self) -> ArtifactHistoryTerminalHandle {
        ArtifactHistoryTerminalHandle { state: self.state.clone() }
    }

    pub fn progress(&self) -> HistoryProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => HistoryProgress::Admitted,
            1 => HistoryProgress::Scheduled,
            2 => HistoryProgress::Waiting,
            3 => HistoryProgress::Mapping,
            4 => HistoryProgress::Completed,
            5 => HistoryProgress::Cancelled,
            _ => HistoryProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        if !matches!(self.progress(), HistoryProgress::Completed | HistoryProgress::Cancelled | HistoryProgress::Fault) {
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            if !self.state.terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled) {
                self.state.schedule();
            }
        }
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactHistoryTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactHistoryTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_reservation(&self) -> Option<ArtifactHistoryTerminalReservation> {
        self.state.take_terminal_reservation()
    }

    pub fn take_terminal_construction_fault(&self) -> Option<ArtifactHistoryTerminalConstructionFault> {
        self.state.take_terminal_construction_fault()
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactHistoryOutcome> {
        let mut result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(Ok(view)) = result.as_mut() {
            view.rearm_terminal_return();
        }
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority.take_terminal_job()
    }

    pub fn close_step(&self) -> bool {
        if self.state.close_one() {
            return true;
        }
        if self.state.authority.close_step() {
            return true;
        }
        self.state.finish_if_terminal_empty()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority.terminal_is_empty()
    }
}

impl ArtifactHistoryTerminalHandle {
    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactHistoryTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactHistoryTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_reservation(&self) -> Option<ArtifactHistoryTerminalReservation> {
        self.state.take_terminal_reservation()
    }

    pub fn take_terminal_construction_fault(&self) -> Option<ArtifactHistoryTerminalConstructionFault> {
        self.state.take_terminal_construction_fault()
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactHistoryOutcome> {
        let mut result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(Ok(view)) = result.as_mut() {
            view.rearm_terminal_return();
        }
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority.take_terminal_job()
    }

    pub fn close_step(&self) -> bool {
        if self.state.close_one() {
            return true;
        }
        if self.state.authority.close_step() {
            return true;
        }
        self.state.finish_if_terminal_empty()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority.terminal_is_empty()
    }
}

impl Future for HistoryFuture {
    type Output = ArtifactHistoryOutcome;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        std::task::Poll::Pending
    }
}

impl Drop for HistoryFuture {
    fn drop(&mut self) {
        if !self.resolved {
            let mut completion = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            if let Some(result) = completion.take() {
                *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            }
            drop(completion);
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            if !self.state.terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled) {
                self.state.schedule();
            }
        }
        self.state.finish_if_terminal_empty();
    }
}

impl ArtifactHistoryTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact history job already resolved").0
    }

    pub fn resume(mut self) {
        let request = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request));
        if request && !self.state.resume_unhanded_reservation() {
            return;
        }
        let (_, job) = self.owner.take().expect("terminal artifact history job already resolved");
        if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let work = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = work;
        }
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.state.submit_exact(job, 0);
    }

    pub fn close(mut self) {
        self.owner.take().expect("terminal artifact history job already resolved");
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactHistoryTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

impl ArtifactHistoryTerminalWork {
    pub fn resume(mut self) -> Result<(), Self> {
        if self.state.generation == 0 {
            return Err(self);
        }
        if self.owner.as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request)) && !self.state.resume_unhanded_reservation() {
            return Err(self);
        }
        let owner = self.owner.take().expect("terminal artifact history work already resolved");
        *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(HistoryProgress::Admitted);
        if self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(())
    }

    pub fn close(mut self) {
        let owner = self.owner.take().expect("terminal artifact history work already resolved");
        match owner {
            ArtifactHistoryWorkOwner::Request => {
                self.state.begin_unhanded_reservation_close();
                self.state.finish_if_terminal_empty();
            }
            ArtifactHistoryWorkOwner::Actor(owner) => {
                *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ArtifactHistoryWorkOwner::Actor(owner));
                self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                self.state.schedule();
            }
        }
    }
}

impl ArtifactHistoryTerminalReservation {
    pub fn resume(mut self) -> Result<(), Self> {
        let mut admission = self.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if admission.as_ref().is_none_or(|owner| owner.reservation.is_some()) {
            drop(admission);
            return Err(self);
        }
        let Some(cursor) = self.owner.as_mut() else {
            drop(admission);
            return Err(self);
        };
        let Some(reservation) = cursor.resume() else {
            drop(admission);
            return Err(self);
        };
        let admission = admission.as_mut().expect("artifact history admission changed after reservation preflight");
        admission.restore_reservation(reservation);
        self.owner = None;
        self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn close_step(&mut self) -> bool {
        let Some(cursor) = self.owner.as_mut() else {
            return false;
        };
        if cursor.close_step() {
            return true;
        }
        if cursor.terminal_is_empty() {
            self.owner = None;
            self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
            self.state.finish_if_terminal_empty();
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.as_ref().is_none_or(db_artifact::HistoryReplayReservationCloseCursor::terminal_is_empty)
    }
}

impl Drop for ArtifactHistoryTerminalReservation {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
        self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

impl ArtifactHistoryTerminalConstructionFault {
    pub fn resume(mut self) -> Result<(), Self> {
        let mut terminal = self.state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_some() {
            drop(terminal);
            return Err(self);
        }
        *terminal = self.owner.take();
        drop(terminal);
        self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn close_step(&mut self) -> bool {
        let Some(cursor) = self.owner.as_mut() else {
            return false;
        };
        if cursor.close_step() {
            return true;
        }
        if cursor.terminal_is_empty() {
            self.owner = None;
            self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.as_ref().is_none_or(db_artifact::HistoryReplayReservationConstructionFault::terminal_is_empty)
    }
}

impl Drop for ArtifactHistoryTerminalConstructionFault {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
        self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

impl Drop for ArtifactHistoryTerminalWork {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

/// @emoji 🎭️ The frozen `ArtifactHandle`: a clone-cheap live handle to one open document.
#[derive(Clone)]
pub struct ArtifactHandle {
    authority: Arc<db_artifact::ArtifactAuthority>,
    document: protocol::ArtifactId,
    pool: Arc<WorkerPool>,
}

impl ArtifactHandle {
    /// @emoji ✍️ The frozen `submit`: commits `batch` through the document's real
    /// `ArtifactAuthority` mailbox. Admission retains the exact request owner, and every I/O-lane
    /// grant advances either request-to-mailbox handoff or one actor-future poll.
    pub fn submit(&self, batch: db_artifact::CommandBatch, options: db_artifact::SubmitOptions) -> SubmitFuture {
        SubmitFuture::submit(self, batch, options)
    }

    /// @emoji 🔎️ The frozen `query`. `Consistency::Canonical` reads the document's live state
    /// directly. `AtLeast`/`Exact` read canonical too, then verify the resulting frontier actually
    /// satisfies the request (`DbError::Unavailable` if not — a true wait-for-frontier primitive
    /// would need a `ArtifactMessage` variant `db_artifact`'s mailbox doesn't expose yet).
    /// `Historical`/`Speculative`/`PreviewAugmented` are `DbError::Unimplemented` — see module doc.
    // 🔒️ `consistency`'s by-value signature is the frozen contract API
    // (`ArtifactHandle::query(&self, query: Query, consistency: Consistency)`, contract.md's
    // "Stable API" block) — not changeable even though this revision's body only borrows it.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn query(&self, query: Query, consistency: Consistency) -> Result<QueryStream, DbError> {
        match &consistency {
            Consistency::Historical(_) | Consistency::PreviewAugmented(_) => {
                return Err(DbError::Unimplemented("historical/preview-augmented query consistency is not yet wired at the db_engine layer (db_query/db_projection integration deferred)"));
            }
            Consistency::Speculative(_) => {
                return Err(DbError::Unimplemented("speculative (preview) query consistency is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"));
            }
            Consistency::Canonical | Consistency::AtLeast(_) | Consistency::Exact(_) => {}
        }

        let paths: Vec<String> = match query {
            Query::Get { path } => vec![path],
            Query::GetMany { paths } => paths,
        };
        if paths.len() > 64 {
            return Err(DbError::LimitExceeded("engine query result entries"));
        }
        let mut results = QueryStream::new();
        for path in paths {
            let value = match self.authority.query(&path).await {
                Ok(value) => value,
                Err(error) => {
                    while results.close_step()? {}
                    return Err(error);
                }
            };
            let path = match db_storage::DbIoText::try_from_str(&path) {
                Ok(path) => path,
                Err(error) => {
                    if let Some(mut value) = value {
                        while value.close_step()?.is_some() {}
                    }
                    while results.close_step()? {}
                    return Err(error);
                }
            };
            if let Err(mut rejected) = results.push(QueryResultEntry { path, value }) {
                while rejected.close_step()? {}
                while results.close_step()? {}
                return Err(DbError::LimitExceeded("engine query result entries"));
            }
        }

        let frontier = match self.frontier().await {
            Ok(frontier) => frontier,
            Err(error) => {
                while results.close_step()? {}
                return Err(error);
            }
        };
        match &consistency {
            Consistency::AtLeast(requested) if !frontier.dominates(requested)? => {
                while results.close_step()? {}
                return Err(DbError::Unavailable("document has not yet reached the requested frontier".to_string()));
            }
            Consistency::Exact(requested) if &frontier != requested => {
                while results.close_step()? {}
                return Err(DbError::Unavailable("document frontier does not exactly match the requested frontier".to_string()));
            }
            _ => {}
        }
        Ok(results)
    }

    /// @emoji 📡️ The frozen `subscribe` — see module doc's `//🎯️ Design choice`: always
    /// `DbError::Unimplemented`, a real (not faked) extension seam pending a `ArtifactMessage`
    /// variant `db_artifact` doesn't expose yet.
    pub async fn subscribe(&self, _spec: LiveQuerySpec) -> Result<LiveQuery, DbError> {
        Err(DbError::Unimplemented("live-query subscription is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// @emoji 🧭️ The frozen `frontier`.
    pub async fn frontier(&self) -> Result<Frontier, DbError> {
        let core_frontier = self.authority.frontier().await?;
        Ok(to_engine_frontier(&core_frontier, self.document.clone()))
    }

    /// @emoji 🌫️ The frozen `preview` — see `subscribe`'s doc; same deferral reason.
    pub async fn preview(&self, _base: Frontier) -> Result<PreviewHandle, DbError> {
        Err(DbError::Unimplemented("preview publish/query is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// @emoji 📜️ Replays history through the document authority. Each accepted process-pool grant
    /// advances one retained mailbox, WAL, envelope, or result-mapping opportunity.
    pub fn history(&self) -> HistoryFuture {
        HistoryFuture::submit(self)
    }

    pub fn history_terminal(&self, generation: u64) -> Option<ArtifactHistoryTerminalHandle> {
        artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().find_map(|slot| slot.as_ref().filter(|state| state.generation == generation).map(|state| ArtifactHistoryTerminalHandle { state: state.clone() }))
    }

    /// @emoji 📸️ The frozen `snapshot_now` — see module doc's `//🎯️ Design choice`: always resolves
    /// to `DbError::Unimplemented`, a real extension seam (no full-state enumeration exists yet to
    /// serialize, and `db_snapshot` is not a direct dependency of this crate).
    pub async fn snapshot_now(&self, _kind: SnapshotKind) -> SnapshotFuture {
        let (reply_tx, reply_rx) = db_actor::oneshot();
        reply_tx.send(Err(DbError::Unimplemented("db_engine does not yet build real pack snapshots (no db_snapshot dependency this wave, and DocumentState exposes no full-state enumeration to serialize)")));
        reply_rx
    }

    pub async fn document_id(&self) -> &protocol::ArtifactId {
        &self.document
    }
}
//#endregion 🔖️ArtifactHandle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vcs_integration::{HashMutation, HashProjection};
    use protocol::{OpBinary, OpText};
    use store::ArtifactPack;

    async fn decode_query_json(mut stream: QueryStream) -> serde_json::Value {
        let mut bytes = Vec::new();
        for fragment in stream.get(0).and_then(QueryResultEntry::value).expect("retained query value").fragments() {
            bytes.extend_from_slice(fragment);
        }
        let value = db_artifact::decode_pathmap_json(&bytes).await.unwrap();
        while stream.close_step().unwrap() {}
        assert!(stream.terminal_is_empty());
        value
    }

    #[derive(Clone, Copy)]
    enum ControlledCapabilityPoll {
        Pending,
        Ready,
        Panic,
    }

    struct ControlledCapabilityFuture {
        mode: ControlledCapabilityPoll,
        storage: Option<Arc<db_storage::DbBackend>>,
        polls: Arc<std::sync::atomic::AtomicUsize>,
        waker: Arc<std::sync::Mutex<Option<std::task::Waker>>>,
        boundary: Option<Arc<(std::sync::Mutex<(bool, bool)>, std::sync::Condvar)>>,
        wake_during_poll: bool,
    }

    struct ControlledCapabilitySubmitQueue {
        slots: [Option<semio_framework_async::Job>; 8],
        head: usize,
        len: usize,
    }

    struct ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize);

    impl std::task::Wake for ControlledCatalogPublicWake {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
    }

    impl ControlledCapabilitySubmitQueue {
        fn new() -> Self {
            Self { slots: std::array::from_fn(|_| None), head: 0, len: 0 }
        }

        fn push(&mut self, job: semio_framework_async::Job) -> Result<(), semio_framework_async::Job> {
            if self.len == self.slots.len() {
                return Err(job);
            }
            let index = (self.head + self.len) % self.slots.len();
            self.slots[index] = Some(job);
            self.len += 1;
            Ok(())
        }

        fn pop(&mut self) -> Option<semio_framework_async::Job> {
            if self.len == 0 {
                return None;
            }
            let job = self.slots[self.head].take();
            self.head = (self.head + 1) % self.slots.len();
            self.len -= 1;
            job
        }
    }

    impl Future for ControlledCapabilityFuture {
        type Output = DatabaseCapabilityOpenResult;

        fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
            let poll = self.polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            *self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
            if self.wake_during_poll && poll == 0 {
                context.waker().wake_by_ref();
            }
            if let Some(boundary) = &self.boundary {
                let (state, ready) = &**boundary;
                let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                state.0 = true;
                ready.notify_one();
                while !state.1 {
                    state = ready.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
                }
            }
            match self.mode {
                ControlledCapabilityPoll::Pending => std::task::Poll::Pending,
                ControlledCapabilityPoll::Ready => std::task::Poll::Ready(DatabaseCapabilityOpenResult {
                    storage: self.storage.take().expect("controlled Ready storage owner"),
                    capabilities: db_storage::StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true },
                }),
                ControlledCapabilityPoll::Panic => panic!("controlled capability-open poll panic"),
            }
        }
    }

    async fn controlled_capability_probe(
        mode: ControlledCapabilityPoll,
        boundary: Option<Arc<(std::sync::Mutex<(bool, bool)>, std::sync::Condvar)>>,
    ) -> (DatabaseCapabilityOpenFuture, Arc<std::sync::atomic::AtomicUsize>, Arc<std::sync::Mutex<Option<std::task::Waker>>>, usize) {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage.clone(), false).expect("controlled capability-open preparation");
        let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let waker = Arc::new(std::sync::Mutex::new(None));
        let future = ControlledCapabilityFuture { mode, storage: Some(storage), polls: polls.clone(), waker: waker.clone(), boundary, wake_during_poll: true };
        *probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCapabilityOpenWork::controlled(Box::pin(future), pointer));
        let work = probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("controlled poll owner");
        *probe.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        probe.state.set_phase(DatabaseCapabilityOpenPhase::Poll);
        (probe, polls, waker, pointer)
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ControlledCatalogReadPoll {
        Pending,
        Ready,
        Panic,
    }

    struct ControlledCatalogReadFuture {
        mode: ControlledCatalogReadPoll,
        storage: Option<Arc<db_storage::DbBackend>>,
        key: Option<DatabaseCatalogRootKey>,
        polls: Arc<std::sync::atomic::AtomicUsize>,
        waker: Arc<std::sync::Mutex<Option<std::task::Waker>>>,
    }

    impl Future for ControlledCatalogReadFuture {
        type Output = DatabaseCatalogReadResult;

        fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
            let poll = self.polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            *self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
            if poll == 0 {
                context.waker().wake_by_ref();
            }
            match self.mode {
                ControlledCatalogReadPoll::Pending => std::task::Poll::Pending,
                ControlledCatalogReadPoll::Ready => {
                    std::task::Poll::Ready(DatabaseCatalogReadResult { storage: self.storage.take().expect("controlled catalog storage"), key: self.key.take().expect("controlled catalog key"), root: Ok(Some((vec![1, 2, 3], EpochFence::INITIAL))) })
                }
                ControlledCatalogReadPoll::Panic => panic!("controlled catalog-read panic"),
            }
        }
    }

    async fn controlled_catalog_read_probe(mode: ControlledCatalogReadPoll) -> (DatabaseCatalogReadFuture, Arc<std::sync::atomic::AtomicUsize>, Arc<std::sync::Mutex<Option<std::task::Waker>>>, usize) {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage.clone(), DatabaseCatalogRootKey::root(), false).expect("controlled catalog-read preparation");
        let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let waker = Arc::new(std::sync::Mutex::new(None));
        let future = ControlledCatalogReadFuture { mode, storage: Some(storage), key: Some(DatabaseCatalogRootKey::root()), polls: polls.clone(), waker: waker.clone() };
        *probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCatalogReadWork::controlled(Box::pin(future), pointer));
        let work = probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("controlled catalog-read work");
        *probe.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        probe.state.set_phase(DatabaseCatalogReadPhase::Poll);
        (probe, polls, waker, pointer)
    }

    #[test]
    fn database_capability_open_fixed_admission_cap_plus_one_and_generation_aba() {
        let mut state = DatabaseCapabilityOpenAdmissionState::empty();
        assert!(state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS + 1, DATABASE_CAPABILITY_OPEN_BYTES).is_err());
        assert!(state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES + 1).is_err());
        let mut claims = Vec::with_capacity(DATABASE_CAPABILITY_OPEN_SLOTS);
        for _ in 0..DATABASE_CAPABILITY_OPEN_SLOTS {
            claims.push(state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES).expect("fixed capability-open admission"));
        }
        assert!(state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES).is_err());
        assert_eq!(state.items, DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS);
        assert_eq!(state.bytes, DATABASE_CAPABILITY_OPEN_TOTAL_BYTES);
        let (slot, generation) = claims.remove(0);
        assert!(state.release(slot, generation, DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES));
        let replacement = state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES).expect("released fixed slot is reusable");
        assert_eq!(replacement.0, slot);
        assert_ne!(replacement.1, generation);
        assert!(!state.release(slot, generation, DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES), "stale generation cannot release the replacement");
        assert!(state.release(replacement.0, replacement.1, DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES));
        for (slot, generation) in claims {
            assert!(state.release(slot, generation, DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES));
        }
        assert_eq!((state.items, state.bytes), (0, 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn database_capability_open_success_returns_exact_storage_owner_and_scalar() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage);
        let probe = match DatabaseCapabilityOpenFuture::try_submit(test_worker_pool(), storage) {
            Ok(probe) => probe,
            Err(_) => panic!("fixed capability-open admission unexpectedly rejected"),
        };
        let output = probe.await.expect("retained capability probe");
        let (storage, capabilities) = output.into_parts();
        assert_eq!(Arc::as_ptr(&storage), pointer);
        assert!(!capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Memory);
    }

    #[semio_framework_async_macros::async_test]
    async fn database_capability_open_cancel_and_stale_generation_retain_exact_owner_for_public_close() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage, false).expect("fixed capability-open preparation");
        let generation = probe.generation();
        assert_eq!(probe.retained_storage_identity(), Some(pointer));
        probe.cancel();
        assert_eq!(probe.progress(), DatabaseCapabilityOpenProgress::Cancelled);
        assert_eq!(probe.retained_storage_identity(), Some(pointer));
        drop(probe);
        let terminal = take_database_capability_open_terminal(generation).expect("cancelled capability-open terminal authority");
        let mut previous = terminal.state.retained_owner_count();
        loop {
            match terminal.close_step() {
                DatabaseCapabilityOpenCloseStep::Progress => {
                    let current = terminal.state.retained_owner_count();
                    assert!(previous.saturating_sub(current) <= 1, "one close grant releases at most one capability-open owner");
                    assert!(current <= previous);
                    previous = current;
                }
                DatabaseCapabilityOpenCloseStep::Blocked => std::thread::yield_now(),
                DatabaseCapabilityOpenCloseStep::Complete => break,
            }
        }
        assert!(terminal.terminal_is_empty());

        let stale_storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let stale_pointer = Arc::as_ptr(&stale_storage) as usize;
        let stale = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), stale_storage, false).expect("fixed stale capability-open preparation");
        let stale_generation = stale.generation();
        {
            let mut admission = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            admission.slots[stale.state.slot].generation = stale_generation.checked_add(1).expect("fixture generation");
        }
        stale.state.clone().drive_one(stale_generation);
        assert_eq!(stale.progress(), DatabaseCapabilityOpenProgress::Fault);
        assert_eq!(stale.retained_storage_identity(), Some(stale_pointer));
        {
            let mut admission = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            admission.slots[stale.state.slot].generation = stale_generation;
        }
        drop(stale);
        let terminal = take_database_capability_open_terminal(stale_generation).expect("stale capability-open terminal authority");
        while !terminal.terminal_is_empty() {
            assert_ne!(terminal.close_step(), DatabaseCapabilityOpenCloseStep::Complete);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn database_capability_open_saturation_and_shutdown_keep_retry_job_and_public_terminal() {
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let gate = Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
        let (started_tx, started_rx) = std::sync::mpsc::sync_channel(1);
        let worker_gate = gate.clone();
        pool.try_submit(
            Lane::Io,
            Box::new(move || {
                started_tx.send(()).expect("fixture start handoff");
                let (lock, ready) = &*worker_gate;
                let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                while !*released {
                    released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
                }
            }),
        )
        .expect("fixture blocker admission");
        started_rx.recv().expect("fixture worker entered blocker");
        loop {
            match pool.try_submit(Lane::Io, Box::new(|| {})) {
                Ok(()) => {}
                Err(error) => {
                    assert_eq!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Saturated);
                    drop(error.into_job());
                    break;
                }
            }
        }
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let probe = DatabaseCapabilityOpenFuture::try_submit(pool.clone(), storage).expect("capability operation admission remains independent of queue saturation");
        assert_eq!(probe.retained_storage_identity(), Some(pointer));
        assert!(probe.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "the exact saturated job remains retained for generation-keyed retry");
        let generation = probe.generation();
        drop(probe);
        let terminal = take_database_capability_open_terminal(generation).expect("abandoned saturated capability-open authority remains public");
        let resumed = match terminal.resume() {
            Ok(resumed) => resumed,
            Err(_) => panic!("public terminal authority failed to resume the exact retained retry job"),
        };
        {
            let (lock, ready) = &*gate;
            *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
            ready.notify_one();
        }
        let output = resumed.await.expect("resumed capability probe");
        assert_eq!(Arc::as_ptr(&output.into_parts().0) as usize, pointer);
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary() {
        for mode in [ControlledCapabilityPoll::Pending, ControlledCapabilityPoll::Ready, ControlledCapabilityPoll::Panic] {
            let (mut probe, polls, retained_waker, _) = controlled_capability_probe(mode, None).await;
            probe.resolved = true;
            let state = probe.state.clone();
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            let submitted = Arc::new(std::sync::Mutex::new(ControlledCapabilitySubmitQueue::new()));
            let submitted_hook = submitted.clone();
            *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| submitted_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
            state.schedule();
            assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "one initial governed callback");
            let initial = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("initial controlled capability callback");
            initial();
            assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1, "one governed poll opportunity");
            assert!(!state.polling.load(std::sync::atomic::Ordering::Acquire));
            assert!(state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
            match mode {
                ControlledCapabilityPoll::Pending => {
                    assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::Poll);
                    assert_eq!(state.progress.load(std::sync::atomic::Ordering::Acquire), DatabaseCapabilityOpenProgress::Pending as u8);
                }
                ControlledCapabilityPoll::Ready => {
                    assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::RetainWork);
                    assert!(state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
                }
                ControlledCapabilityPoll::Panic => {
                    assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::RetainWork);
                    assert!(state.cancelled.load(std::sync::atomic::Ordering::Acquire));
                    assert!(state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
                }
            }
            assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "during-poll wake coalesces with the exact phase successor");
            retained_waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().expect("controlled backend retained its real waker").wake_by_ref();
            assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "wake-after-release cannot admit a duplicate successor");
            let successor = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("one controlled successor callback");
            successor();
            if mode == ControlledCapabilityPoll::Pending {
                assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 2, "Pending is repolled only by its next governed successor");
                state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                state.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            } else {
                assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1, "terminal Ready/panic successor advances cleanup without repolling");
                assert!(state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "terminal successor retains the exact work owner");
                if mode == ControlledCapabilityPoll::Ready {
                    assert!(state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "Ready successor retains the exact result owner");
                }
            }
            loop {
                let Some(job) = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop() else { break };
                job();
            }
            while !state.terminal_is_empty() {
                let _ = state.close_step();
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn database_capability_open_post_ready_cancel_and_stale_retain_public_exact_result() {
        for stale in [false, true] {
            let (mut probe, polls, _, pointer) = controlled_capability_probe(ControlledCapabilityPoll::Ready, None).await;
            let state = probe.state.clone();
            probe.resolved = true;
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.scheduled.store(true, std::sync::atomic::Ordering::Release);
            if stale {
                let slot = state.slot;
                let generation = state.generation;
                *state.poll_publication_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Box::new(move || {
                    let mut admission = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    admission.slots[slot].generation = generation.checked_add(1).expect("controlled stale generation");
                }));
            } else {
                let cancel_state = state.clone();
                *state.poll_publication_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Box::new(move || {
                    DatabaseCapabilityOpenFuture { state: cancel_state, resolved: true }.cancel();
                }));
            }
            state.poll_backend_once(state.generation);
            assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
            assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::RetainWork);
            assert!(state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
            assert!(state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
            if stale {
                DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = state.generation;
            }
            state.scheduled.store(false, std::sync::atomic::Ordering::Release);
            while state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                assert_ne!(state.close_step(), DatabaseCapabilityOpenCloseStep::Complete);
            }
            let terminal = take_database_capability_open_terminal(state.generation).expect("post-Ready terminal authority");
            let checked_out = terminal.take_result().expect("post-Ready result checkout");
            drop(checked_out);
            let resumed = match terminal.take_result().expect("post-Ready result handback").resume() {
                Ok(resumed) => resumed,
                Err(_) => panic!("post-Ready exact result resume rejected"),
            };
            let output = resumed.await.expect("post-Ready retained output");
            assert_eq!(Arc::as_ptr(&output.into_parts().0) as usize, pointer);
            while !terminal.terminal_is_empty() {
                let _ = terminal.close_step();
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn database_capability_open_rejection_take_retry_and_close_preserve_exact_storage() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage);
        let rejected = DatabaseCapabilityOpenRejected { error: Some(DbError::Closed), storage: Some(storage) };
        let mut resumed = rejected.retry(test_worker_pool()).expect("rejected storage retry");
        assert_eq!(resumed.retained_storage_identity(), Some(pointer as usize));
        resumed.cancel();
        let generation = resumed.generation();
        drop(resumed);
        let terminal = take_database_capability_open_terminal(generation).expect("retried rejection terminal");
        while !terminal.terminal_is_empty() {
            let _ = terminal.close_step();
        }

        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage);
        let mut rejected = DatabaseCapabilityOpenRejected { error: Some(DbError::Closed), storage: Some(storage) };
        let returned = rejected.take_storage().expect("exact rejected storage take");
        assert_eq!(Arc::as_ptr(&returned), pointer);
        rejected.storage = Some(returned);
        assert_eq!(rejected.close_step(), DatabaseCapabilityOpenCloseStep::Progress);
        assert!(rejected.terminal_is_empty());
        assert_eq!(rejected.into_error_after_close().expect("closed rejection error"), DbError::Closed);
    }

    #[semio_framework_async_macros::async_test]
    async fn database_capability_open_terminal_result_take_resume_and_checked_out_drop_handback() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage);
        let mut probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage.clone(), false).expect("terminal-result preparation");
        probe.resolved = true;
        probe.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        *probe.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
            Some(Ok(DatabaseCapabilityOpenResult { storage, capabilities: db_storage::StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true } }));
        let terminal = DatabaseCapabilityOpenTerminalHandle { state: probe.state.clone() };
        let checked_out = terminal.take_result().expect("terminal result checkout");
        drop(checked_out);
        assert!(terminal.take_result().is_some(), "checked-out Drop returns the shallow result ticket");
        let resumed = match terminal.take_result().expect("terminal result retry checkout").resume() {
            Ok(resumed) => resumed,
            Err(_) => panic!("terminal result resume rejected"),
        };
        let output = resumed.await.expect("resumed exact terminal result");
        assert_eq!(Arc::as_ptr(&output.into_parts().0), pointer);
        while !terminal.terminal_is_empty() {
            let _ = terminal.close_step();
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn database_capability_open_retry_contention_is_one_compare_exchange_per_callback() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let mut probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage, false).expect("retry-contention preparation");
        probe.resolved = true;
        let state = probe.state.clone();
        state.retry_armed.store(true, std::sync::atomic::Ordering::Release);
        let observed = state.retry_generation.load(std::sync::atomic::Ordering::Acquire);
        state.retry_generation.store(observed.checked_add(1).expect("fixture generation"), std::sync::atomic::Ordering::Release);
        state.advance_retry_generation_observed_once(observed);
        assert!(state.retry_armed.load(std::sync::atomic::Ordering::Acquire));
        state.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        while !state.terminal_is_empty() {
            let _ = state.close_step();
        }
    }

    #[test]
    fn database_catalog_read_fixed_cap_plus_one_and_generation_aba() {
        let mut admission = DatabaseCatalogReadAdmissionState::empty();
        assert!(admission.try_claim(DATABASE_CATALOG_READ_ITEMS + 1, DATABASE_CATALOG_READ_BYTES).is_err());
        assert!(admission.try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES + 1).is_err());
        let mut claims = Vec::with_capacity(DATABASE_CATALOG_READ_SLOTS);
        for _ in 0..DATABASE_CATALOG_READ_SLOTS {
            claims.push(admission.try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES).expect("catalog-read fixed admission"));
        }
        assert!(admission.try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES).is_err());
        let (slot, generation) = claims.remove(0);
        assert!(admission.release(slot, generation));
        let replacement = admission.try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES).expect("catalog-read slot reuse");
        assert_eq!(replacement.0, slot);
        assert_ne!(replacement.1, generation);
        assert!(!admission.release(slot, generation));
    }

    #[semio_framework_async_macros::async_test]
    async fn database_catalog_read_success_returns_exact_storage_key_and_root() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage);
        let result = DatabaseCatalogReadFuture::try_submit(test_worker_pool(), storage, DatabaseCatalogRootKey::root()).expect("catalog-read success admission").await.expect("catalog-read success completion");
        let (returned_storage, returned_key, root) = result.into_parts();
        assert_eq!(Arc::as_ptr(&returned_storage), pointer);
        assert_eq!(returned_key, DatabaseCatalogRootKey::root());
        assert!(root.expect("catalog-read backend result").is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn database_catalog_read_controlled_wakes_coalesce_and_terminal_never_repolls() {
        for mode in [ControlledCatalogReadPoll::Pending, ControlledCatalogReadPoll::Ready, ControlledCatalogReadPoll::Panic] {
            let (mut probe, polls, waker, _) = controlled_catalog_read_probe(mode).await;
            probe.resolved = true;
            let state = probe.state.clone();
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            let queue = Arc::new(std::sync::Mutex::new(ControlledCapabilitySubmitQueue::new()));
            let queue_hook = queue.clone();
            *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
            state.schedule();
            let initial = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("catalog-read initial callback");
            initial();
            assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
            assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "one coalesced catalog-read successor");
            waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().expect("catalog-read retained waker").wake_by_ref();
            assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "late wake cannot overtake the retained successor");
            let successor = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("catalog-read successor callback");
            successor();
            if mode == ControlledCatalogReadPoll::Pending {
                assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 2);
                state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                state.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
            } else {
                assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
                assert!(state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
                if mode == ControlledCatalogReadPoll::Ready {
                    assert!(state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
                }
            }
            loop {
                let Some(job) = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop() else { break };
                job();
            }
            while !state.terminal_is_empty() {
                let _ = state.close_step();
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn database_catalog_read_publication_between_check_and_waker_registration_is_observed() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let mut probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage, DatabaseCatalogRootKey::root(), false).expect("public lost-wake preparation");
        let state = probe.state.clone();
        let mut work = state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("public lost-wake retained work");
        assert!(work.close_step());
        assert!(work.terminal_is_empty());
        drop(work);

        let result_storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let result_pointer = Arc::as_ptr(&result_storage) as usize;
        let publish_state = state.clone();
        *state.controlled_publication_before_waker_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Box::new(move || {
            publish_state.publish_public_completion(Ok(DatabaseCatalogReadResult { storage: result_storage, key: DatabaseCatalogRootKey::root(), root: Ok(None) }));
        }));
        let wake = Arc::new(ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize::new(0)));
        let waker = std::task::Waker::from(wake.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let ready = std::pin::Pin::new(&mut probe).poll(&mut context);
        let std::task::Poll::Ready(Ok(result)) = ready else { panic!("completion published in the registration window must be observed in the same public poll") };
        assert_eq!(Arc::as_ptr(&result.into_parts().0) as usize, result_pointer, "public recheck returns the exact published owner");
        assert_eq!(wake.0.load(std::sync::atomic::Ordering::Acquire), 0, "the recheck consumes completion without needing a later wake");
        assert!(state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none(), "Ready clears the transient public waker");
    }

    #[semio_framework_async_macros::async_test]
    async fn database_catalog_read_rejected_mount_retires_storage_and_key_on_distinct_grants() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let storage_weak = Arc::downgrade(&storage);
        let rejected = DatabaseCatalogReadRejected { error: Some(DbError::Closed), storage: Some(storage), key: Some(DatabaseCatalogRootKey::root()) };
        let (error, close) = rejected.mount_close_and_take_error(test_worker_pool(), false);
        assert_eq!(error, DbError::Closed);
        let queue = Arc::new(std::sync::Mutex::new(ControlledCapabilitySubmitQueue::new()));
        let queue_hook = queue.clone();
        *close.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
        close.schedule();
        assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1);

        queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("rejected storage close grant")();
        assert!(storage_weak.upgrade().is_none(), "first mounted close grant releases only the exact storage owner");
        {
            let owner = close.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = owner.as_ref().expect("key remains mounted after storage grant");
            assert!(owner.storage.is_none());
            assert_eq!(owner.key.as_ref(), Some(&DatabaseCatalogRootKey::root()));
        }
        assert!(!close.terminal_is_empty());
        assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "unfinished key close is retained as the next governed grant");

        queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("rejected key close grant")();
        assert!(close.terminal_is_empty(), "second mounted close grant releases the exact key and reaches terminal empty");
        assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn database_catalog_read_cancel_stale_and_rejection_preserve_exact_storage_key() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage);
        let mut rejected = DatabaseCatalogReadRejected { error: Some(DbError::Closed), storage: Some(storage), key: Some(DatabaseCatalogRootKey::root()) };
        assert_eq!(Arc::as_ptr(rejected.storage.as_ref().expect("rejected storage")), pointer);
        assert_eq!(rejected.close_step(), DatabaseCatalogReadCloseStep::Progress);
        assert!(!rejected.terminal_is_empty());
        assert_eq!(rejected.close_step(), DatabaseCatalogReadCloseStep::Progress);
        assert!(rejected.terminal_is_empty());
        assert_eq!(rejected.into_error_after_close().expect("rejected error"), DbError::Closed);

        for stale in [false, true] {
            let (probe, _, _, pointer) = controlled_catalog_read_probe(ControlledCatalogReadPoll::Pending).await;
            let state = probe.state.clone();
            let generation = state.generation;
            let queue = Arc::new(std::sync::Mutex::new(ControlledCapabilitySubmitQueue::new()));
            let queue_hook = queue.clone();
            *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
            if stale {
                DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = generation.checked_add(1).expect("stale catalog generation");
            } else {
                probe.cancel();
            }
            drop(probe);
            if stale {
                DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = generation;
                state.stage_terminal(DbError::Unavailable("stale catalog fixture".to_string()), DatabaseCatalogReadProgress::Fault);
            }
            assert_eq!(state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|work| work.storage_identity), Some(pointer));
            loop {
                let Some(job) = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop() else { break };
                job();
            }
            while !state.terminal_is_empty() {
                let _ = state.close_step();
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn database_catalog_read_terminal_result_drop_hands_back_exact_result() {
        let (mut probe, _, _, _) = controlled_catalog_read_probe(ControlledCatalogReadPoll::Ready).await;
        probe.resolved = true;
        probe.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let result_pointer = Arc::as_ptr(&storage) as usize;
        *probe.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(DatabaseCatalogReadResult { storage, key: DatabaseCatalogRootKey::root(), root: Ok(None) }));
        let terminal = DatabaseCatalogReadTerminalHandle { state: probe.state.clone() };
        let checked_out = terminal.take_result().expect("catalog result checkout");
        drop(checked_out);
        let result = terminal.take_result().expect("catalog result handback").take().expect("catalog result take").expect("catalog result owner");
        assert_eq!(Arc::as_ptr(&result.into_parts().0) as usize, result_pointer, "catalog result checkout returns the exact storage owner");
        while !terminal.terminal_is_empty() {
            let _ = terminal.close_step();
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn hash_operation_text_and_binary_round_trip_with_every_field_present_and_absent() {
        let bare = HashMutation { hash: [7u8; 32], author: None, timestamp: None };
        assert_eq!(HashMutation::parse_op(&bare.print_op()).unwrap().hash, bare.hash);
        assert!(HashMutation::parse_op(&bare.print_op()).unwrap().author.is_none());
        assert_eq!(HashMutation::decode_op(&bare.encode_op().unwrap()).unwrap(), bare);

        let full = HashMutation { hash: [9u8; 32], author: Some(protocol::ActorId("actor-1".into())), timestamp: Some(protocol::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 }) };
        let reparsed = HashMutation::parse_op(&full.print_op()).unwrap();
        assert_eq!(reparsed.hash, full.hash);
        assert_eq!(reparsed.author, full.author);
        assert_eq!(reparsed.timestamp, full.timestamp);
        let redecoded = HashMutation::decode_op(&full.encode_op().unwrap()).unwrap();
        assert_eq!(redecoded, full);
    }

    #[semio_framework_async_macros::async_test]
    async fn hash_projection_pack_round_trips() {
        let projection = HashProjection { latest_hash: [3u8; 32] };
        let bytes = projection.encode_pack();
        assert_eq!(HashProjection::decode_pack(&bytes).unwrap(), projection);
    }

    //#region 🧸️Fixtures
    async fn tempdir(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("db_engine-test-{name}-{}-{}", std::process::id(), now_ms().await));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    async fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::ArtifactId, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
        let mut payload = serde_json::Map::new();
        for (path, value) in entries {
            payload.insert((*path).to_string(), value.clone());
        }
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: document.clone(),
            actor: protocol::ActorId(actor.to_string()),
            dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(serde_json::Map::new())).await.unwrap() },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Database open/catalog
    #[semio_framework_async_macros::async_test]
    async fn open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog() {
        let root = tempdir("open-at-fresh").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        assert!(database.catalog().await.artifacts.is_empty());
        assert_eq!(database.health().await.open_artifacts, 0);
        assert!(matches!(database.health().await.report.overall, db_observe::HealthState::Healthy));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_document_registers_it_in_the_catalog_and_document_finds_it() {
        let root = tempdir("create-and-find").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

        let catalog = database.catalog().await;
        assert_eq!(catalog.artifacts.len(), 1);
        assert_eq!(catalog.artifacts[0].document, document);

        let handle = database.document(&document).await.unwrap();
        assert_eq!(handle.document_id().await, &document);
    }

    #[semio_framework_async_macros::async_test]
    async fn create_document_twice_errs_already_exists() {
        let root = tempdir("create-twice").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let result = database.create_document(ArtifactSpec::new(document).await);
        assert!(matches!(result.await, Err(DbError::AlreadyExists(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_of_an_unknown_id_errs_not_found() {
        let root = tempdir("unknown-doc").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let never_created = protocol::ArtifactId("never-created".to_string());
        let result = database.document(&never_created);
        assert!(matches!(result.await, Err(DbError::NotFound(_))));
    }
    //#endregion 🔖️Database open/catalog

    //#region 🔖️Round trip
    #[semio_framework_async_macros::async_test]
    async fn full_submit_durable_query_round_trip_over_a_real_document_authority() {
        let root = tempdir("round-trip").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
        let receipt = db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
        assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
        assert_eq!(receipt.frontier.document, document);
        assert_eq!(receipt.frontier.head_seq, 1);
        assert!(receipt.conflicts.is_empty());
        assert!(receipt.state_hash.is_some());

        let queried = handle.query(Query::Get { path: "name".to_string() }, Consistency::Canonical).await.unwrap();
        let value = decode_query_json(queried).await;
        assert_eq!(value, serde_json::json!("hello"));

        let frontier = handle.frontier().await.unwrap();
        assert_eq!(frontier.head_seq, 1);

        let mut at_least = handle.query(Query::Get { path: "name".to_string() }, Consistency::AtLeast(frontier)).await.unwrap();
        assert_eq!(at_least.len(), 1);
        while at_least.close_step().unwrap() {}
        assert!(at_least.terminal_is_empty());

        let mut history = handle.history().await.unwrap();
        assert_eq!(history.entries().len(), 1);
        assert!(history.operation_id_eq(0, 0, "op-1"));
        while history.close_step() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_history_empty_and_two_batch_replay_are_deterministic() {
        let root = tempdir("history-order").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("history-doc".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let mut empty = handle.history().await.unwrap();
        assert!(empty.entries().is_empty());
        while empty.close_step() {}
        for id in ["history-1", "history-2"] {
            let batch = db_artifact::CommandBatch::new(vec![envelope(id, &[], "alice", &document, &[("value", serde_json::json!(id))]).await]).await.unwrap();
            db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();
        }
        let mut first = handle.history().await.unwrap();
        let mut second = handle.history().await.unwrap();
        assert_eq!(first.entries(), second.entries());
        assert!(first.operation_id_eq(0, 0, "history-1"));
        assert!(first.operation_id_eq(1, 0, "history-2"));
        assert!(second.operation_id_eq(0, 0, "history-1"));
        assert!(second.operation_id_eq(1, 0, "history-2"));
        while first.close_step() {}
        while second.close_step() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root() {
        let root = tempdir("reopen").await;
        let document = protocol::ArtifactId("doc-1".to_string());
        {
            let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
            let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
            let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("count", serde_json::json!(1))]).await]).await.unwrap();
            db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
            database.shutdown(std::time::Duration::from_secs(1)).await.unwrap();
        }

        let reopened = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        assert_eq!(reopened.catalog().await.artifacts.len(), 1, "the catalog root must have survived the reopen");

        let handle = reopened.document(&document).await.unwrap();
        let queried = handle.query(Query::Get { path: "count".to_string() }, Consistency::Canonical).await.unwrap();
        let value = decode_query_json(queried).await;
        assert_eq!(value, serde_json::json!(1), "the document's committed state must have survived the reopen via WAL replay");
        assert_eq!(handle.frontier().await.unwrap().head_seq, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_consistency_rejects_a_frontier_the_document_has_moved_past() {
        let root = tempdir("exact-consistency").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let stale = handle.frontier().await.unwrap();

        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

        let result = handle.query(Query::Get { path: "x".to_string() }, Consistency::Exact(stale));
        assert!(matches!(result.await, Err(DbError::Unavailable(_))));
    }

    #[test]
    fn query_stream_max_plus_one_hands_back_exact_owner_and_close_is_terminal() {
        let mut stream = QueryStream::new();
        for index in 0..64 {
            stream.push(QueryResultEntry { path: db_storage::DbIoText::try_from_str(&format!("path-{index:02}")).unwrap(), value: None }).unwrap();
        }
        let rejected = stream.push(QueryResultEntry { path: db_storage::DbIoText::try_from_str("overflow-owner").unwrap(), value: None }).unwrap_err();
        assert_eq!(rejected.path(), "overflow-owner");
        let mut rejected = rejected;
        while rejected.close_step().unwrap() {}
        while stream.close_step().unwrap() {}
        assert!(stream.terminal_is_empty());
    }
    //#endregion 🔖️Round trip

    //#region 🔖️Deferred extension seams
    #[semio_framework_async_macros::async_test]
    async fn subscribe_preview_and_snapshot_now_are_documented_unimplemented_not_panics() {
        let root = tempdir("deferred").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();

        assert!(matches!(handle.subscribe(LiveQuerySpec { since: None }).await, Err(DbError::Unimplemented(_))));
        assert!(matches!(handle.preview(handle.frontier().await.unwrap()).await, Err(DbError::Unimplemented(_))));
        assert!(matches!(db_actor::block_on(handle.snapshot_now(SnapshotKind::Full).await), Ok(Err(DbError::Unimplemented(_)))));
    }
    //#endregion 🔖️Deferred extension seams

    //#region 🔖️VersionGraph
    #[cfg(feature = "vcs")]
    #[semio_framework_async_macros::async_test]
    async fn checkpoint_document_mints_distinct_real_vcs_content_addressed_checkpoint_ids() {
        let root = tempdir("vcs-checkpoint").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

        let batch1 = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch1, db_artifact::SubmitOptions::default())).unwrap().unwrap();
        let checkpoint_1 = database.checkpoint_document(&document, "first".to_string(), &[protocol::ActorId("alice".to_string())]).await.unwrap();
        assert!(checkpoint_1.starts_with("ck-"), "vcs checkpoint ids are content-addressed as ck-<hex16>, got {checkpoint_1:?}");

        let batch2 = db_artifact::CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &document, &[("x", serde_json::json!(2))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch2, db_artifact::SubmitOptions::default())).unwrap().unwrap();
        let checkpoint_2 = database.checkpoint_document(&document, "second".to_string(), &[protocol::ActorId("alice".to_string())]).await.unwrap();

        assert_ne!(checkpoint_1, checkpoint_2, "distinct commits must mint distinct content-addressed checkpoint ids");
    }

    #[cfg(not(feature = "vcs"))]
    #[semio_framework_async_macros::async_test]
    async fn checkpoint_document_errs_unimplemented_without_the_vcs_feature() {
        let root = tempdir("no-vcs-checkpoint").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        database.create_document(ArtifactSpec::new(document.clone())).await.unwrap();
        assert!(matches!(database.checkpoint_document(&document, "msg".to_string(), &[]).await, Err(DbError::Unimplemented(_))));
    }
    //#endregion 🔖️VersionGraph

    //#region 🔖️Compact + Sync
    #[semio_framework_async_macros::async_test]
    async fn compact_document_runs_a_real_compaction_pass_without_error() {
        let root = tempdir("compact").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

        let report = database.compact_document(&document, "holder-1", false).await.unwrap();
        assert_eq!(report.wal_segments_deleted, 0, "nothing is below the (nonexistent) snapshot floor yet, but the pass itself must succeed");
    }

    #[semio_framework_async_macros::async_test]
    async fn hello_returns_a_welcome_with_a_fresh_bootstrap_for_a_brand_new_replica() {
        let root = tempdir("hello").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

        let response = database.hello(&document, None, "session-1".to_string(), &protocol::ActorId("semio_hub".to_string()), 4096).await.unwrap();
        assert!(matches!(response.welcome, protocol::ServerFrame::Welcome { .. }));
    }

    // 🔬️ `storage()` is a real escape hatch to the same backend `Database::open_at` wired — a
    // caller below the document-actor boundary (os-semio_hub's blob routes) can round-trip a payload
    // through it directly, independent of any document actor.
    #[semio_framework_async_macros::async_test]
    async fn storage_accessor_reaches_the_same_backend_payload_store() {
        let root = tempdir("storage-accessor").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let hash = db_actor::block_on(async {
            let pages = db_storage::db_io_copy_pages(b"hello storage accessor").unwrap().await.unwrap();
            database.storage().await.payload().await.put(pages).await
        })
        .unwrap();
        assert_eq!(db_actor::block_on(async { database.storage().await.payload().await.get(&hash).await }).unwrap(), b"hello storage accessor");
    }
    //#endregion 🔖️Compact + Sync

    //#region 🔖️Retained submit authority
    fn retained_submit_source() -> &'static str {
        include_str!("🦀️component.rs")
    }

    fn retire_history_admission(mut admission: ArtifactHistoryAdmission) {
        let mut cursor = admission.begin_reservation_close().expect("history test admission retained its reservation");
        for _ in 0..50_000 {
            if cursor.terminal_is_empty() {
                break;
            }
            assert!(cursor.close_step());
        }
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn artifact_submit_late_readiness_parks_then_one_shot_wake_reschedules() {
        let source = retained_submit_source();
        assert!(source.contains("impl std::task::Wake for ArtifactSubmitWake"));
        assert!(source.contains("self.scheduled.compare_exchange(false, true"));
        assert!(source.contains("std::task::Poll::Pending =>"));
        assert!(source.contains("self.set_progress(SubmitProgress::Waiting)"));
    }

    #[test]
    fn artifact_submit_pool_saturation_without_later_ingress_retains_exact_job() {
        let source = retained_submit_source();
        assert!(source.contains("self.pool.try_submit(Lane::Io, job)"));
        assert!(source.contains("error.into_job()"));
        assert!(source.contains("self.pool.callback_at"));
        assert!(source.contains("ARTIFACT_SUBMIT_RETRY_LIMIT"));
    }

    #[test]
    fn artifact_submit_cancel_before_during_after_preserves_exact_owner() {
        let source = retained_submit_source();
        assert!(source.contains("self.state.cancelled.store(true"));
        assert!(source.contains("self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled)"));
        assert!(source.contains("*self.terminal_result.lock()"));
        assert!(source.contains("SubmitProgress::Completed | SubmitProgress::Cancelled | SubmitProgress::Fault"));
    }

    #[test]
    fn artifact_submit_stale_generation_and_slot_aba_cannot_consume_current_work() {
        let first = ArtifactSubmitAdmission::try_claim(1, ARTIFACT_SUBMIT_PAGE_BYTES).unwrap();
        let first_slot = first.slot;
        let first_generation = first.generation;
        drop(first);
        let next = ArtifactSubmitAdmission::try_claim(1, ARTIFACT_SUBMIT_PAGE_BYTES).unwrap();
        assert_eq!(next.slot, first_slot);
        assert_ne!(next.generation, first_generation);
        let source = retained_submit_source();
        let stale = source.find("if generation != self.generation").unwrap();
        let mutation = source[stale..].find("self.scheduled.store").unwrap();
        assert!(mutation > 0);
    }

    #[test]
    fn artifact_submit_missing_handle_terminalizes_without_mailbox_mutation() {
        let source = retained_submit_source();
        let stale = source.find("if self.authority.generation() != self.authority_generation").unwrap();
        let handoff = source.find("self.authority.submit_retained").unwrap();
        assert!(stale < handoff);
        assert!(source.contains("Err(DbError::StaleGeneration"));
    }

    #[test]
    fn artifact_submit_terminal_job_work_result_take_resume_and_close_one_owner() {
        let source = retained_submit_source();
        for required in ["pub fn take_terminal_job", "pub fn take_terminal_work", "pub fn take_terminal_result", "pub fn take_actor_terminal_job", "pub fn close_step", "pub fn terminal_is_empty", "pub fn resume(mut self)"] {
            assert!(source.contains(required), "missing {required}");
        }
        assert!(source.contains("fn close_one(&self) -> bool"));
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_submit_item_cap_plus_one_and_nested_bytes_plus_one_return_owner() {
        let document = protocol::ArtifactId("credit-doc".to_string());
        let one = envelope("credit-1", &[], "credit-actor", &document, &[("x", serde_json::json!(1))]).await;
        let admitted = db_artifact::CommandBatch::new(vec![one]).await.unwrap();
        assert!(artifact_submit_credit(&admitted).is_ok());

        let mut envelopes = Vec::new();
        for index in 0..=ARTIFACT_SUBMIT_BATCH_ITEMS {
            envelopes.push(envelope(&format!("credit-{index}"), &[], "credit-actor", &document, &[("x", serde_json::json!(index))]).await);
        }
        let rejected = db_artifact::CommandBatch { envelopes };
        assert!(artifact_submit_credit(&rejected).is_err());

        let mut oversize = envelope("credit-oversize", &[], "credit-actor", &document, &[("x", serde_json::json!(1))]).await;
        oversize.diff.payload = vec![0; ARTIFACT_SUBMIT_OPERATION_BYTES as usize + 1];
        assert!(artifact_submit_credit(&db_artifact::CommandBatch { envelopes: vec![oversize] }).is_err());
    }

    #[test]
    fn artifact_runner_one_grant_polls_one_turn_and_never_blocks_on() {
        let source = include_str!("../📄️artifact/🦀️component.rs");
        let runner = &source[source.find("type ArtifactBuildFuture").unwrap()..source.find("//#region 🧪️Tests").unwrap()];
        assert!(!runner.contains("block_on("));
        assert!(!runner.contains("ask_blocking"));
        assert!(runner.contains("future.as_mut().poll(&mut context)"));
        assert!(runner.contains("Self::start_turn(engine, envelope.payload)"));
        assert!(runner.contains("let closed ="));
        assert!(runner.contains("if !closed"));
    }

    #[test]
    fn artifact_history_empty_one_cap_plus_one_admission_returns_exact_request() {
        let mut claims = Vec::new();
        for _ in 0..ARTIFACT_HISTORY_OPERATION_SLOTS {
            claims.push(ArtifactHistoryAdmission::try_claim().unwrap());
        }
        assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))));
        for claim in claims {
            retire_history_admission(claim);
        }
        let source = retained_submit_source();
        assert!(source.contains("HistoryFrameToken::End"));
        assert!(source.contains("ArtifactHistoryWorkOwner::Request"));
    }

    #[test]
    fn artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release() {
        let mut cancelled = ArtifactHistoryAdmission::try_claim().unwrap();
        let cancelled_generation = cancelled.generation;
        let mut cursor = cancelled.begin_reservation_close().expect("cancelled pre-handoff request retained its full reservation");
        let mut peers = Vec::new();
        for _ in 1..ARTIFACT_HISTORY_OPERATION_SLOTS {
            peers.push(ArtifactHistoryAdmission::try_claim().unwrap());
        }
        assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))), "retirement must retain admission credit until terminal-empty");
        assert!(cursor.close_step());
        assert!(!cursor.terminal_is_empty());
        for _ in 0..50_000 {
            if cursor.terminal_is_empty() {
                break;
            }
            assert!(cursor.close_step());
        }
        assert!(cursor.terminal_is_empty());
        drop(cursor);
        drop(cancelled);
        let replacement = ArtifactHistoryAdmission::try_claim().unwrap();
        assert_ne!(replacement.generation, cancelled_generation);
        retire_history_admission(replacement);
        for peer in peers {
            retire_history_admission(peer);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty() {
        let root = tempdir("history-public-terminal-release").await;
        let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("history-public-terminal-release".to_string());
        let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();
        let mut peers = Vec::new();
        for _ in 1..ARTIFACT_HISTORY_OPERATION_SLOTS {
            peers.push(ArtifactHistoryAdmission::try_claim().unwrap());
        }
        let history = handle.history();
        let generation = history.generation();
        let terminal = history.terminal_handle();
        history.cancel();
        drop(history);
        assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))));
        assert!(!terminal.terminal_is_empty());
        let mut released = false;
        for _ in 0..100_000 {
            if terminal.terminal_is_empty() {
                break;
            }
            let roots_were_empty = terminal.state.terminal_roots_are_empty();
            let admission_was_retained = terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
            let progressed = terminal.close_step();
            let admission_is_released = terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none();
            if admission_was_retained && admission_is_released {
                assert!(roots_were_empty, "the admission release must occupy a grant after every terminal root was already empty");
                assert!(progressed);
                assert!(terminal.state.finished.load(std::sync::atomic::Ordering::Acquire));
                released = true;
            }
            if !progressed {
                std::thread::yield_now();
            }
        }
        assert!(released);
        assert!(terminal.terminal_is_empty());
        assert!(handle.history_terminal(generation).is_none(), "terminal completion must unregister the released generation");
        let replacement = ArtifactHistoryAdmission::try_claim().unwrap();
        retire_history_admission(replacement);
        for peer in peers {
            retire_history_admission(peer);
        }
    }

    #[test]
    fn artifact_history_nested_derived_item_and_byte_caps_precede_materialization() {
        let artifact = include_str!("../📄️artifact/🦀️component.rs");
        for required in ["HISTORY_REPLAY_RESULT_BYTES", "HISTORY_REPLAY_OPERATION_BYTES", "HISTORY_REPLAY_MAX_ENTRIES", "HISTORY_REPLAY_MAX_OPERATION_IDS", "history dependency item credit", "history result byte credit"] {
            assert!(artifact.contains(required), "missing {required}");
        }
        let preflight = artifact.find("operation_count >= HISTORY_REPLAY_MAX_OPERATION_IDS").unwrap();
        let publish = artifact.find("reservation.operation_ids.push(HistoryTextRange").unwrap();
        assert!(preflight < publish);
    }

    #[test]
    fn artifact_history_segment_cap_plus_one_reads_only_one_admitted_page() {
        let artifact = include_str!("../📄️artifact/🦀️component.rs");
        assert!(artifact.contains("HISTORY_REPLAY_SEGMENT_PAGES: u64 = 1_024"));
        assert!(artifact.contains(".min(HISTORY_REPLAY_PAGE_BYTES)"));
        assert!(artifact.contains("pack::ByteRange { offset, len: requested }"));
        assert!(artifact.contains("page.capacity() as u64 > HISTORY_REPLAY_PAGE_BYTES"));
        assert!(!artifact.contains("pack::ByteRange { offset: 0, len }"));
    }

    #[test]
    fn artifact_history_crc_and_frame_tokenizer_advance_one_page_per_grant() {
        let artifact = include_str!("../📄️artifact/🦀️component.rs");
        assert!(artifact.contains("protocol::codec::Crc32cCursor"));
        assert!(artifact.contains("self.crc.update_page(page)"));
        assert!(artifact.contains("self.payload_remaining.min(HISTORY_REPLAY_PAGE_BYTES)"));
        assert!(!artifact.contains("protocol::codec::crc32c(whole_frame)"));
        assert!(!artifact.contains("decode_history_token"));
    }

    #[test]
    fn artifact_history_cancel_retires_one_page_or_nested_owner_per_actor_grant() {
        let artifact = include_str!("../📄️artifact/🦀️component.rs");
        assert!(artifact.contains("HistoryReplayTransition::FaultRetire"));
        assert!(artifact.contains("self.page_count -= 1"));
        assert!(artifact.contains("self.operation_ids.pop().is_some()"));
        assert!(artifact.contains("self.result_pages.pop()"));
        assert!(!artifact.contains("pages.clear()"));
    }

    #[test]
    fn artifact_history_quiet_late_wake_and_retry_are_generation_coalesced() {
        let source = retained_submit_source();
        assert!(source.contains("impl std::task::Wake for ArtifactHistoryWake"));
        assert!(source.contains("self.generation == state.generation"));
        assert!(source.contains("self.scheduled.compare_exchange(false, true"));
        assert!(source.contains("self.pool.callback_at"));
        assert!(source.contains("retry_generation"));
    }

    #[test]
    fn artifact_history_cancel_before_during_after_retains_actor_and_result_owners() {
        let source = retained_submit_source();
        assert!(source.contains("history_retained(self.generation, self.cancelled.clone(), reservation)"));
        assert!(source.contains("terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled)"));
        assert!(source.contains("self.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled)"));
        assert!(source.contains("*self.terminal_result.lock()"));
        assert!(source.contains("HistoryProgress::Completed | HistoryProgress::Cancelled | HistoryProgress::Fault"));
    }

    #[test]
    fn artifact_history_stale_generation_and_slot_aba_precede_mailbox_mutation() {
        let first = ArtifactHistoryAdmission::try_claim().unwrap();
        let slot = first.slot;
        let generation = first.generation;
        retire_history_admission(first);
        let next = ArtifactHistoryAdmission::try_claim().unwrap();
        assert_eq!(next.slot, slot);
        assert_ne!(next.generation, generation);
        let source = retained_submit_source();
        let stale = source.find("if self.authority.generation() != self.authority_generation").unwrap();
        let handoff = source.find("self.authority.history_retained").unwrap();
        assert!(stale < handoff);
        retire_history_admission(next);
    }

    #[test]
    fn artifact_history_replay_ordering_is_segment_frame_then_result_fifo() {
        let artifact = include_str!("../📄️artifact/🦀️component.rs");
        assert!(artifact.contains("HistoryReplayPhase::Probe { index: 0 }"));
        assert!(artifact.contains("cursor: HistoryFrameCursor::new(next_offset)"));
        assert!(artifact.contains("reservation.entries.push(ArtifactHistoryEntry"));
        assert!(!retained_submit_source().contains("ArtifactHistoryWorkOwner::Map"));
    }

    #[test]
    fn artifact_history_terminal_job_work_result_take_resume_and_close_one_owner() {
        let source = retained_submit_source();
        for required in [
            "pub fn take_terminal_job",
            "pub fn take_terminal_work",
            "pub fn take_terminal_result",
            "pub fn take_actor_terminal_job",
            "pub fn close_step",
            "pub fn terminal_is_empty",
            "impl ArtifactHistoryTerminalJob",
            "impl ArtifactHistoryTerminalWork",
            "pub struct ArtifactHistoryTerminalReservation",
            "pub fn take_terminal_reservation",
        ] {
            assert!(source.contains(required), "missing {required}");
        }
        assert!(source.contains("fn close_one(self: &Arc<Self>) -> bool"));
    }

    #[test]
    fn artifact_history_construction_fault_is_public_and_admission_release_is_a_final_grant() {
        let engine = retained_submit_source();
        let artifact = include_str!("../📄️artifact/🦀️component.rs");
        for required in [
            "terminal_construction: std::sync::Mutex<Option<db_artifact::HistoryReplayReservationConstructionFault>>",
            "pub struct ArtifactHistoryTerminalConstructionFault",
            "pub fn take_terminal_construction_fault",
            "pub fn resume(mut self) -> Result<(), Self>",
            "fn terminal_roots_are_empty(&self) -> bool",
            "self.finished.load(std::sync::atomic::Ordering::Acquire)",
            "self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()",
            "self.state.finish_if_terminal_empty()",
        ] {
            assert!(engine.contains(required), "missing retained public construction/admission authority: {required}");
        }
        assert!(artifact.contains("const HISTORY_REPLAY_CONSTRUCTION_SLOTS: usize = 64"));
        assert!(artifact.contains("pub(crate) fn try_new()"));
        assert!(artifact.contains("pub(crate) struct HistoryReplayReservationConstructionFault"));
        assert!(artifact.contains("impl Drop for HistoryReplayReservationConstructionBuilder"));
        assert!(artifact.contains("impl Drop for HistoryReplayReservationConstructionFault"));
        assert!(artifact.contains("take_history_replay_reservation_construction_fault"));
        assert!(!artifact.contains("pub fn into_parts"));
        assert!(!artifact.contains("pub struct HistoryReplayReservationConstructionFaultCursor"));
        assert!(artifact.contains("try_new_with_result_page_failure"));
        let terminal = &engine[engine.find("impl ArtifactHistoryTerminalHandle").unwrap()..engine.find("impl Future for HistoryFuture").unwrap()];
        let close = &terminal[terminal.find("pub fn close_step(&self)").unwrap()..terminal.find("pub fn terminal_is_empty(&self)").unwrap()];
        assert!(close.find("self.state.close_one()").unwrap() < close.find("self.state.finish_if_terminal_empty()").unwrap());
        assert!(close.contains("return true"), "owner retirement must return before the final admission-release grant");
    }

    #[test]
    fn artifact_history_one_grant_advances_one_retained_phase_without_blocking() {
        let engine = retained_submit_source();
        let artifact = include_str!("../📄️artifact/🦀️component.rs");
        let history = &engine[engine.find("const ARTIFACT_HISTORY_OPERATION_SLOTS").unwrap()..engine.find("pub struct ArtifactHandle").unwrap()];
        let replay = &artifact[artifact.find("//#region 🔖️HistoryReplay").unwrap()..artifact.find("//#endregion 🔖️HistoryReplay").unwrap()];
        for forbidden in ["block_on(", "ask_blocking", "submit_blocking", "loop {", "while "] {
            assert!(!history.contains(forbidden), "history outer authority retained {forbidden}");
            assert!(!replay.contains(forbidden), "history replay cursor retained {forbidden}");
        }
        assert!(history.contains("std::pin::Pin::new(future).poll(&mut context)"));
        assert!(artifact.contains("future.as_mut().poll(&mut context)"));
    }

    #[test]
    fn artifact_history_drop_after_complete_moves_result_to_public_terminal_registry() {
        let source = retained_submit_source();
        let result_drop = &source[source.find("impl Drop for HistoryView").unwrap()..source.find("//#endregion 🔖️History").unwrap()];
        assert!(result_drop.contains("register_artifact_history(&state)"));
        assert!(result_drop.contains("state.terminal_result"));
        assert!(result_drop.contains("self.inner.take()"));
        assert!(source.contains("pub fn history_terminal(&self, generation: u64)"));
    }

    #[test]
    fn artifact_history_runner_close_mid_turn_retains_replay_until_terminal_empty() {
        let artifact = include_str!("../📄️artifact/🦀️component.rs");
        let runner = &artifact[artifact.find("enum ArtifactTurn").unwrap()..artifact.find("//#region 🧪️Tests").unwrap()];
        assert!(runner.contains("ArtifactTurn::History"));
        assert!(runner.contains("replay.request_close(DbError::Closed)"));
        assert!(runner.contains("!replay.terminal_is_empty()"));
        assert!(runner.contains("Pin::new(&mut *replay).poll(&mut context)"));
        let panic_close = &runner[runner.find("history replay cursor panicked").unwrap()..];
        assert!(panic_close.contains("replay.request_close"));
        assert!(panic_close.contains("self.schedule()"));
        assert!(!runner.contains("turn.take();\n                    drop(turn);\n                    self.address.close();"));
    }

    #[test]
    fn artifact_history_future_handle_drop_and_terminal_take_resume_are_exact() {
        let source = retained_submit_source();
        let future_drop = &source[source.find("impl Drop for HistoryFuture").unwrap()..source.find("impl ArtifactHistoryTerminalJob").unwrap()];
        assert!(future_drop.contains("completion.take()"));
        assert!(future_drop.contains("terminal_result"));
        assert!(!future_drop.contains("self.state.close_one()"));
        for required in ["pub struct ArtifactHistoryTerminalHandle", "pub fn terminal_handle(&self)", "pub fn take_terminal_result(&self)", "pub fn resume(mut self)", "pub fn close_step(&self)"] {
            assert!(source.contains(required), "missing {required}");
        }
    }
    //#endregion 🔖️Retained submit authority

    //#region 🔖️Security
    #[semio_framework_async_macros::async_test]
    async fn security_authz_hook_rejects_a_principal_denied_by_its_policy() {
        let policy = db_security::RoleBasedPolicy::new();
        let gate = db_security::SecurityGate::new(policy, db_security::ReplayGuard::new(60_000, 16), db_security::BudgetRegistry::new(100, 10), Arc::new(NullEmit));
        let hook = SecurityAuthzHook::new(gate, |actor| db_security::Principal::new(actor.clone(), db_security::TenantId::from("tenant-1"), vec!["viewer".to_string()])).await;

        let document = protocol::ArtifactId("doc-1".to_string());
        let envelope = envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await;
        let result = db_artifact::AuthzHook::authorize(&hook, &envelope.actor, &envelope);
        assert!(matches!(result.await, Err(DbError::Unauthorized(_))), "a default-deny policy with no grants must reject every action");
    }
    //#endregion 🔖️Security
}
//#endregion 🧪️Tests
