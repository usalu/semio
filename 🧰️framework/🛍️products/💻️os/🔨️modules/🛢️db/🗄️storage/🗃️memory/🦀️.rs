//! 🗃️ Db in-memory storage backend.
#![allow(async_fn_in_trait)]

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, check_len};
use db_io_core::{CatalogStorage, DB_IO_MAX_READ_BYTES, DB_IO_OPERATION_ITEMS, DB_IO_PAGE_BYTES, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoCredit, DbIoExecutionStep, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DbIoWriterReleaseStep, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage, WalWriterPermit, db_io_backend_owner_add, db_io_backend_owner_reserve, db_io_operation_return};
use db_io_driver::{DbStorageOpenRejected, WalWriterGuard as _, close_db_io_backend, open_db_io_backend_admitted, retire_db_io_backend, submit_db_io_task_admitted};
use db_io_registry::{DbIoBackendKind, DbIoBackendRegistrationRejected, DbIoBackendRegistrationRejectedOwner, DbIoBackendRollbackReservation, register_db_io_backend_reserved_with_use};
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use std::{sync::Arc, sync::Mutex, sync::atomic::AtomicU64, sync::atomic::Ordering, task::Context};
#[cfg(any(test, feature = "testkit"))]
use db_ids::GenerationId;
#[cfg(any(test, feature = "testkit"))]
use db_io_core::{DB_IO_BACKEND_CONTROLS, DB_IO_LEDGER_OWNER, DB_IO_LIST_BACKING_BYTES, DB_IO_PROCESS_BYTES, DB_IO_PROCESS_WITH_WRITER_BACKING_BYTES, DB_IO_TASK_SLOTS, DB_IO_WRITER_STATIC_BACKING_BYTES, DbIoExecutorMode, DbIoFaultKind, DbIoLedgerOwner, DbIoPagePhase, WAL_WRITER_CAPACITY, db_io_backend_owner_admits, db_io_backend_parts, db_io_ledger_census, db_io_operation_ledger, db_io_operation_reserve, db_io_operation_slot, db_io_page_arena, db_io_result_lease_credit, db_io_slot_matches};
#[cfg(any(test, feature = "testkit"))]
use db_io_driver::{DB_IO_ADMISSION_WAIT_MS, block_on_ready, db_io_backend_admission_saturated, db_io_backend_admit_operation, db_io_maintenance_step, db_io_take_async_executor, db_io_test_pool, process_isolated_law, submit_db_io_task};
#[cfg(any(test, feature = "testkit"))]
use db_io_registry::{db_io_backend_close_step, db_io_backend_registry, db_io_backend_return_operation, db_io_page_maintenance_step, db_io_return_async_executor, db_io_writer_release_lane_step, register_db_io_backend};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_async::{Job, Lane, WorkerSubmitErrorKind};
#[cfg(any(test, feature = "testkit"))]
use std::{pin::Pin, task::Poll, task::Waker};

//#region 🔖️Memory
/// 🧠️ One in-process, non-durable segment of a document's WAL — `bytes` plus whether
/// `seal` has been called on it. Its chunk table is heap backing of the live segment only: a
/// segment charges [`memory_wal_segment_credit`] to its backend when it is created and returns
/// it when its retirement empties the slot, so an idle memory backend does not pre-pay the chunk
/// tables of all 64 segment slots.
struct MemWalSegment {
    pub chunks: Box<[Option<DbIoPages>]>,
    pub len: u64,
    sealed: bool,
}

impl MemWalSegment {
    pub fn new() -> Self {
        Self { chunks: memory_fixed_none_box(DB_IO_OPERATION_ITEMS), len: 0, sealed: false }
    }
}

/// 🧾️ Exact heap credit of one live memory WAL segment's chunk table.
const fn memory_wal_segment_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: (DB_IO_OPERATION_ITEMS * size_of::<Option<DbIoPages>>()) as u64, items: 1, controls: 0 }
}

/// 🧠️ A pure in-memory `DbStorage`: every store is a `Mutex`-guarded map, nothing ever
/// touches a filesystem. Not durable (`capabilities().durable == false`) — the backend for unit
/// tests and `db_fault_testing`'s deterministic simulation runtime, never for a real deployment. Every
/// trait method body below is synchronous (no real I/O to await), so it is simply wrapped in an
/// already-`Ready` `{ .. }` per the module doc's "Async-first" section.
const DB_IO_MEMORY_OWNERS: usize = DB_IO_OPERATION_ITEMS;

struct MemoryWalOwner {
    pub document: DbIoText,
    pub index: u64,
    segment: MemWalSegment,
}

struct MemoryPageOwner {
    pub document: DbIoText,
    pub ordinal: u64,
    pub pages: DbIoPages,
}

struct MemoryPayloadOwner {
    pub hash: ContentHash,
    pub pages: DbIoPages,
}

struct MemoryLeaseOwner {
    pub resource: DbIoText,
    pub holder: DbIoText,
    pub fence: EpochFence,
    pub expires_at_ms: u64,
}

struct MemoryDbIoExecutor {
    pub writers: std::sync::Mutex<Option<Box<db_io_driver::WalWriterTable<()>>>>,
    pub wal: std::sync::Mutex<Box<[Option<MemoryWalOwner>]>>,
    snapshots: std::sync::Mutex<Box<[Option<MemoryPageOwner>]>>,
    payloads: std::sync::Mutex<Box<[Option<MemoryPayloadOwner>]>>,
    pub catalog: std::sync::Mutex<Option<(DbIoPages, EpochFence)>>,
    index_runs: std::sync::Mutex<Box<[Option<MemoryPageOwner>]>>,
    pub leases: std::sync::Mutex<Box<[Option<MemoryLeaseOwner>]>>,
    operations: std::sync::Mutex<Box<[Option<MemoryDbIoCursor>]>>,
    retired_pages: std::sync::Mutex<Box<[Option<DbIoPages>]>>,
    retired_wal: std::sync::Mutex<Box<[Option<MemWalSegment>]>>,
    close_phase: std::sync::atomic::AtomicU8,
    backing_operation: std::sync::atomic::AtomicU64,
}

enum MemoryDbIoCursor {
    WalRead { operation: u64, chunk: u8, page: u8, offset: usize, skip: usize, remaining: usize },
    PageCopy { operation: u64, page: u8 },
    List { operation: u64, after: Option<u64> },
    PayloadHash { operation: u64, page: u8, hasher: semio_framework_hash::Hasher },
    LeaseRelease { operation: u64, index: u16 },
}

impl MemoryDbIoCursor {
    pub fn operation(&self) -> u64 {
        match self {
            Self::WalRead { operation, .. } | Self::PageCopy { operation, .. } | Self::List { operation, .. } | Self::PayloadHash { operation, .. } | Self::LeaseRelease { operation, .. } => *operation,
        }
    }
}

impl Default for MemoryDbIoExecutor {
    fn default() -> Self {
        Self {
            writers: std::sync::Mutex::new(Some(Box::new(db_io_driver::WalWriterTable::unbound()))),
            wal: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            snapshots: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            payloads: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            catalog: std::sync::Mutex::new(None),
            index_runs: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            leases: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS)),
            operations: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_OPERATION_ITEMS)),
            retired_pages: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_OPERATION_ITEMS)),
            retired_wal: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_OPERATION_ITEMS)),
            close_phase: std::sync::atomic::AtomicU8::new(0),
            backing_operation: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

fn memory_fixed_none_box<T>(items: usize) -> Box<[Option<T>]> {
    std::iter::repeat_with(|| None).take(items).collect()
}

impl MemoryDbIoExecutor {
    fn backing_bytes() -> u64 {
        (size_of::<Self>()
            + size_of::<db_io_driver::WalWriterTable<()>>()
            + DB_IO_MEMORY_OWNERS * (size_of::<Option<MemoryWalOwner>>() + 2 * size_of::<Option<MemoryPageOwner>>() + size_of::<Option<MemoryPayloadOwner>>() + size_of::<Option<MemoryLeaseOwner>>())
            + DB_IO_OPERATION_ITEMS * (size_of::<Option<MemoryDbIoCursor>>() + size_of::<Option<DbIoPages>>() + size_of::<Option<MemWalSegment>>())) as u64
    }

    fn backing_operation(&self) -> Result<u64, DbError> {
        match self.backing_operation.load(std::sync::atomic::Ordering::Acquire) {
            0 => Err(DbError::Internal("memory backend lost its backing operation".to_string())),
            operation => Ok(operation),
        }
    }

    fn return_segment_credit(&self) -> Result<(), DbError> {
        db_io_operation_return(self.backing_operation()?, memory_wal_segment_credit())
    }

    fn retain_pages(&self, input: &mut DbIoPages) -> Result<DbIoPages, DbError> {
        let backend_operation = self.backing_operation()?;
        let mut pages = input.take_for_async_driver();
        pages.transfer_to_backend(backend_operation)?;
        Ok(pages)
    }

    fn retirement_step(&self) -> Result<bool, DbError> {
        let mut retired_pages = lock(&self.retired_pages);
        if let Some(slot) = retired_pages.iter_mut().find(|slot| slot.is_some()) {
            let Some(owner) = slot.as_mut() else {
                return Err(DbError::Internal("memory retired page slot lost its owner".to_string()));
            };
            if owner.close_step()?.is_some() {
                return Ok(true);
            }
            *slot = None;
            return Ok(true);
        }
        drop(retired_pages);
        let mut retired_wal = lock(&self.retired_wal);
        if let Some(slot) = retired_wal.iter_mut().find(|slot| slot.is_some()) {
            let Some(owner) = slot.as_mut() else {
                return Err(DbError::Internal("memory retired WAL slot lost its owner".to_string()));
            };
            for chunk in owner.chunks.iter_mut().flatten() {
                if chunk.close_step()?.is_some() {
                    return Ok(true);
                }
            }
            if let Some(chunk) = owner.chunks.iter_mut().find(|chunk| chunk.as_ref().is_some_and(DbIoPages::terminal_is_empty)) {
                *chunk = None;
                return Ok(true);
            }
            self.return_segment_credit()?;
            *slot = None;
            return Ok(true);
        }
        Ok(false)
    }

    /// 🧹️ One explicit memory-backend owner/page retirement opportunity.
    pub fn close_backend_step(&self) -> Result<bool, DbError> {
        if let Some(cursor) = lock(&self.operations).iter_mut().find(|cursor| cursor.is_some()) {
            *cursor = None;
            return Ok(false);
        }
        if self.retirement_step()? {
            return Ok(false);
        }
        match self.close_phase.load(std::sync::atomic::Ordering::Acquire) {
            0 => {
                let mut wal = lock(&self.wal);
                let Some(owner) = wal.iter_mut().find_map(Option::as_mut) else {
                    self.close_phase.store(1, std::sync::atomic::Ordering::Release);
                    return Ok(false);
                };
                if let Some(chunk) = owner.segment.chunks.iter_mut().flatten().find(|chunk| !chunk.terminal_is_empty()) {
                    let _ = chunk.close_step()?;
                    return Ok(false);
                }
                if let Some(chunk) = owner.segment.chunks.iter_mut().find(|chunk| chunk.is_some()) {
                    *chunk = None;
                    return Ok(false);
                }
                if owner.document.close_step() {
                    return Ok(false);
                }
                let slot = wal
                    .iter_mut()
                    .find(|slot| slot.as_ref().is_some_and(|candidate| candidate.document.terminal_is_empty() && candidate.segment.chunks.iter().all(Option::is_none)))
                    .ok_or_else(|| DbError::Internal("memory WAL close cursor lost exact owner".to_string()))?;
                self.return_segment_credit()?;
                *slot = None;
                Ok(false)
            }
            1 => {
                let mut snapshots = lock(&self.snapshots);
                if memory_page_owner_close_step(&mut snapshots[..])? {
                    return Ok(false);
                }
                self.close_phase.store(2, std::sync::atomic::Ordering::Release);
                Ok(false)
            }
            2 => {
                let mut payloads = lock(&self.payloads);
                let Some(owner) = payloads.iter_mut().find_map(Option::as_mut) else {
                    self.close_phase.store(3, std::sync::atomic::Ordering::Release);
                    return Ok(false);
                };
                if owner.pages.close_step()?.is_some() {
                    return Ok(false);
                }
                let slot = payloads.iter_mut().find(|slot| slot.as_ref().is_some_and(|candidate| candidate.pages.terminal_is_empty())).ok_or_else(|| DbError::Internal("memory payload close cursor lost exact owner".to_string()))?;
                *slot = None;
                Ok(false)
            }
            3 => {
                let mut catalog = lock(&self.catalog);
                if let Some((pages, _)) = catalog.as_mut() {
                    if pages.close_step()?.is_some() {
                        return Ok(false);
                    }
                    *catalog = None;
                    return Ok(false);
                }
                self.close_phase.store(4, std::sync::atomic::Ordering::Release);
                Ok(false)
            }
            4 => {
                let mut runs = lock(&self.index_runs);
                if memory_page_owner_close_step(&mut runs[..])? {
                    return Ok(false);
                }
                self.close_phase.store(5, std::sync::atomic::Ordering::Release);
                Ok(false)
            }
            5 => {
                let mut leases = lock(&self.leases);
                let Some(owner) = leases.iter_mut().find_map(Option::as_mut) else {
                    self.close_phase.store(6, std::sync::atomic::Ordering::Release);
                    return Ok(false);
                };
                if owner.holder.close_step() || owner.resource.close_step() {
                    return Ok(false);
                }
                let slot = leases
                    .iter_mut()
                    .find(|slot| slot.as_ref().is_some_and(|candidate| candidate.holder.terminal_is_empty() && candidate.resource.terminal_is_empty()))
                    .ok_or_else(|| DbError::Internal("memory lease close cursor lost exact owner".to_string()))?;
                *slot = None;
                Ok(false)
            }
            _ => Ok(true),
        }
    }

    pub fn backend_terminal_is_empty(&self) -> bool {
        self.close_phase.load(std::sync::atomic::Ordering::Acquire) == 6
            && lock(&self.operations).iter().all(Option::is_none)
            && lock(&self.retired_pages).iter().all(Option::is_none)
            && lock(&self.retired_wal).iter().all(Option::is_none)
            && lock(&self.wal).iter().all(Option::is_none)
            && lock(&self.snapshots).iter().all(Option::is_none)
            && lock(&self.payloads).iter().all(Option::is_none)
            && lock(&self.catalog).is_none()
            && lock(&self.index_runs).iter().all(Option::is_none)
            && lock(&self.leases).iter().all(Option::is_none)
    }
}

fn memory_page_owner_close_step(owners: &mut [Option<MemoryPageOwner>]) -> Result<bool, DbError> {
    let Some(owner) = owners.iter_mut().find_map(Option::as_mut) else { return Ok(false) };
    if owner.pages.close_step()?.is_some() || owner.document.close_step() {
        return Ok(true);
    }
    let slot =
        owners.iter_mut().find(|slot| slot.as_ref().is_some_and(|candidate| candidate.pages.terminal_is_empty() && candidate.document.terminal_is_empty())).ok_or_else(|| DbError::Internal("memory page close cursor lost exact owner".to_string()))?;
    *slot = None;
    Ok(true)
}

/// 🩹️ Recovers a `Mutex` guard from a poisoned lock instead of panicking — a single
/// panicking mailbox/actor elsewhere in the family must not turn every other document's storage
/// access into a cascading panic.
// 🚫️async: E1 pure accessor (no suspension) — see R9
pub fn lock<'a, T>(mutex: &'a std::sync::Mutex<T>) -> std::sync::MutexGuard<'a, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn memory_cursor_index(cursors: &mut [Option<MemoryDbIoCursor>], operation: u64, initial: impl FnOnce() -> MemoryDbIoCursor) -> Result<usize, DbError> {
    if let Some(index) = cursors.iter().position(|cursor| cursor.as_ref().is_some_and(|cursor| cursor.operation() == operation)) {
        return Ok(index);
    }
    let index = cursors.iter().position(Option::is_none).ok_or_else(|| DbError::Unavailable("memory DB I/O cursor capacity exhausted".to_string()))?;
    cursors[index] = Some(initial());
    Ok(index)
}

impl DbIoTaskExecutor for MemoryDbIoExecutor {
    fn supports_writer_authority(&self) -> bool {
        true
    }

    fn bind_writer_control(&mut self, control: DbIoBackendControl) -> Result<(), DbError> {
        lock(&self.writers).as_mut().ok_or(DbError::Closed)?.bind(control)
    }

    fn writer_release_step(&self, _context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
        match lock(&self.writers).as_mut() {
            Some(table) => table.release_requested_step(),
            None => Ok(DbIoWriterReleaseStep::Idle),
        }
    }

    fn pin_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        if let Some((key, backend, document)) = task.writer_stamp() {
            lock(&self.writers).as_mut().ok_or(DbError::Closed)?.pin_operation(key, backend, document, operation)?;
        }
        Ok(())
    }

    fn finish_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        if let Some((key, backend, document)) = task.writer_stamp() {
            if let Some(table) = lock(&self.writers).as_mut() {
                table.finish_operation_if_pinned(key, backend, document, operation)?;
            }
        }
        Ok(())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn bind_owner_operation(&mut self, operation: u64) -> Result<(), DbError> {
        if operation == 0 || self.backing_operation.swap(operation, std::sync::atomic::Ordering::AcqRel) != 0 {
            return Err(DbError::Internal("memory backend backing operation bound twice".to_string()));
        }
        Ok(())
    }

    fn owner_backing_bytes(&self) -> u64 {
        (size_of_val(self)
            + lock(&self.writers).as_deref().map_or(0, size_of_val)
            + size_of_val(lock(&self.wal).as_ref())
            + size_of_val(lock(&self.snapshots).as_ref())
            + size_of_val(lock(&self.payloads).as_ref())
            + size_of_val(lock(&self.index_runs).as_ref())
            + size_of_val(lock(&self.leases).as_ref())
            + size_of_val(lock(&self.operations).as_ref())
            + size_of_val(lock(&self.retired_pages).as_ref())
            + size_of_val(lock(&self.retired_wal).as_ref())) as u64
    }

    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("memory backend has no async-native driver".to_string())))
        })
    }

    fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let complete = |result| Ok((DbIoExecutionStep::Complete, Some(result)));
        let yield_step = || Ok((DbIoExecutionStep::Yield, None));
        match task {
            DbIoTask::WalWriterAcquire { document, .. } => complete(DbIoResult::WalWriter(lock(&self.writers).as_mut().ok_or(DbError::Closed)?.acquire_with(document, || Ok(()))?)),
            DbIoTask::BackendOpen { path, .. } => {
                if path.as_str() != "memory://fixed" {
                    return Err(DbError::InvalidArgument("memory backend authority mismatch".to_string()));
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::WalCreate { document, index, .. } => {
                let mut wal = lock(&self.wal);
                if wal.iter().flatten().any(|owner| owner.document == *document && owner.index == *index) {
                    return Err(DbError::AlreadyExists("memory WAL segment already exists".to_string()));
                }
                let slot = wal.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory WAL fixed owner capacity exhausted".to_string()))?;
                db_io_backend_owner_add(self.backing_operation()?, memory_wal_segment_credit())?;
                *slot = Some(MemoryWalOwner { document: document.clone(), index: *index, segment: MemWalSegment::new() });
                complete(DbIoResult::Unit)
            }
            DbIoTask::WalAppend { document, index, input, .. } => {
                let mut wal = lock(&self.wal);
                let segment = &mut wal.iter_mut().flatten().find(|owner| owner.document == *document && owner.index == *index).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?.segment;
                if segment.sealed {
                    return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
                }
                let next_len = segment.len.checked_add(input.len() as u64).ok_or(DbError::LimitExceeded("memory WAL retained length"))?;
                check_len(next_len, DB_IO_MAX_READ_BYTES, "memory WAL retained length")?;
                if !input.is_empty() {
                    let slot = segment.chunks.iter_mut().find(|chunk| chunk.is_none()).ok_or(DbError::LimitExceeded("memory WAL retained chunk items"))?;
                    *slot = Some(self.retain_pages(input)?);
                }
                segment.len = next_len;
                complete(DbIoResult::Length(next_len))
            }
            DbIoTask::WalSync { .. } => complete(DbIoResult::Unit),
            DbIoTask::WalSeal { document, index, .. } => {
                let mut wal = lock(&self.wal);
                let segment = &mut wal.iter_mut().flatten().find(|owner| owner.document == *document && owner.index == *index).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?.segment;
                segment.sealed = true;
                complete(DbIoResult::Unit)
            }
            DbIoTask::WalRead { document, index, range, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::WalRead { operation, chunk: 0, page: 0, offset: 0, skip: range.offset as usize, remaining: range.len as usize })?;
                let Some(MemoryDbIoCursor::WalRead { chunk, page, offset, skip, remaining, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory WAL cursor taxonomy mismatch".to_string()));
                };
                if *remaining == 0 {
                    return match output.seal_retained_step()? {
                        Some(pages) => {
                            cursors[cursor_index] = None;
                            complete(DbIoResult::Pages(pages))
                        }
                        None => yield_step(),
                    };
                }
                let wal = lock(&self.wal);
                let segment = &wal.iter().flatten().find(|owner| owner.document == *document && owner.index == *index).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?.segment;
                let owner = segment.chunks.get(*chunk as usize).and_then(Option::as_ref).ok_or_else(|| DbError::InvalidArgument("memory WAL range exceeds retained chunks".to_string()))?;
                let fragment = match owner.page(*page) {
                    Some(fragment) => fragment,
                    None => {
                        *chunk += 1;
                        *page = 0;
                        *offset = 0;
                        return yield_step();
                    }
                };
                if *skip >= fragment.len() {
                    *skip -= fragment.len();
                    *page += 1;
                    return yield_step();
                }
                if *skip != 0 {
                    *offset = *skip;
                    *skip = 0;
                }
                let requested = (fragment.len() - *offset).min(*remaining);
                let written = output.write_fragment(&fragment[*offset..*offset + requested])?;
                *offset += written;
                *remaining -= written;
                if *offset == fragment.len() {
                    *page += 1;
                    *offset = 0;
                }
                yield_step()
            }
            DbIoTask::WalLength { document, index, .. } => {
                let wal = lock(&self.wal);
                let length = wal.iter().flatten().find(|owner| owner.document == *document && owner.index == *index).map(|owner| owner.segment.len).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?;
                complete(DbIoResult::Length(length))
            }
            DbIoTask::WalState { document, index, .. } => {
                let wal = lock(&self.wal);
                let sealed = wal.iter().flatten().find(|owner| owner.document == *document && owner.index == *index).map(|owner| owner.segment.sealed).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?;
                complete(DbIoResult::WalSegmentState(if sealed { WalSegmentState::Sealed } else { WalSegmentState::Active }))
            }
            DbIoTask::WalList { document, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::List { operation, after: None })?;
                let Some(MemoryDbIoCursor::List { after, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory list cursor taxonomy mismatch".to_string()));
                };
                let next = lock(&self.wal).iter().flatten().filter(|owner| owner.document == *document && after.is_none_or(|after| owner.index > after)).map(|owner| owner.index).min();
                if let Some(value) = next {
                    output.push(value)?;
                    *after = Some(value);
                    return yield_step();
                }
                cursors[cursor_index] = None;
                complete(DbIoResult::List(output.take_for_result()))
            }
            DbIoTask::WalTruncate { document, index, new_len, .. } => {
                let mut retired = lock(&self.retired_pages);
                let mut wal = lock(&self.wal);
                let segment = &mut wal.iter_mut().flatten().find(|owner| owner.document == *document && owner.index == *index).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?.segment;
                if segment.sealed || *new_len > segment.len {
                    return Err(DbError::InvalidArgument("memory WAL truncate rejected".to_string()));
                }
                let mut retained = *new_len as usize;
                for chunk in &mut segment.chunks {
                    let Some(owner) = chunk.take() else { continue };
                    if retained >= owner.len() {
                        retained -= owner.len();
                        *chunk = Some(owner);
                    } else if retained > 0 {
                        *chunk = Some(owner.try_prefix(retained).map_err(|_| DbError::Internal("memory WAL prefix cursor rejected".to_string()))?);
                        retained = 0;
                    } else {
                        let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory WAL truncate retirement capacity exhausted".to_string()))?;
                        *slot = Some(owner);
                    }
                }
                segment.len = *new_len;
                complete(DbIoResult::Unit)
            }
            DbIoTask::WalDelete { document, index, .. } => {
                let mut retired = lock(&self.retired_wal);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory WAL delete retirement capacity exhausted".to_string()))?;
                let mut wal = lock(&self.wal);
                if let Some(owner) = wal.iter_mut().find(|owner| owner.as_ref().is_some_and(|owner| owner.document == *document && owner.index == *index)).and_then(Option::take) {
                    *slot = Some(owner.segment);
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::SnapshotWrite { document, generation, input, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory snapshot replacement retirement capacity exhausted".to_string()))?;
                let mut snapshots = lock(&self.snapshots);
                if let Some(owner) = snapshots.iter_mut().flatten().find(|owner| owner.document == *document && owner.ordinal == *generation) {
                    *slot = Some(std::mem::replace(&mut owner.pages, self.retain_pages(input)?));
                } else {
                    let target = snapshots.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory snapshot fixed owner capacity exhausted".to_string()))?;
                    *target = Some(MemoryPageOwner { document: document.clone(), ordinal: *generation, pages: self.retain_pages(input)? });
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::SnapshotRead { document, generation, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PageCopy { operation, page: 0 })?;
                let Some(MemoryDbIoCursor::PageCopy { page, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory page cursor taxonomy mismatch".to_string()));
                };
                let snapshots = lock(&self.snapshots);
                let owner = snapshots.iter().flatten().find(|owner| owner.document == *document && owner.ordinal == *generation).ok_or_else(|| DbError::NotFound("memory snapshot generation not found".to_string()))?;
                if let Some(fragment) = owner.pages.page(*page) {
                    if output.write_fragment(fragment)? != fragment.len() {
                        return Err(DbError::LimitExceeded("memory admitted page output"));
                    }
                    *page += 1;
                    return yield_step();
                }
                drop(snapshots);
                match output.seal_retained_step()? {
                    Some(pages) => {
                        cursors[cursor_index] = None;
                        complete(DbIoResult::Pages(pages))
                    }
                    None => yield_step(),
                }
            }
            DbIoTask::SnapshotLatest { document, .. } => complete(DbIoResult::OptionalLength(lock(&self.snapshots).iter().flatten().filter(|owner| owner.document == *document).map(|owner| owner.ordinal).max())),
            DbIoTask::SnapshotList { document, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::List { operation, after: None })?;
                let Some(MemoryDbIoCursor::List { after, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory snapshot list cursor taxonomy mismatch".to_string()));
                };
                let next = lock(&self.snapshots).iter().flatten().filter(|owner| owner.document == *document && after.is_none_or(|after| owner.ordinal > after)).map(|owner| owner.ordinal).min();
                if let Some(value) = next {
                    output.push(value)?;
                    *after = Some(value);
                    return yield_step();
                }
                cursors[cursor_index] = None;
                complete(DbIoResult::List(output.take_for_result()))
            }
            DbIoTask::SnapshotDelete { document, generation, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory snapshot delete retirement capacity exhausted".to_string()))?;
                let mut snapshots = lock(&self.snapshots);
                if let Some(owner) = snapshots.iter_mut().find(|owner| owner.as_ref().is_some_and(|owner| owner.document == *document && owner.ordinal == *generation)).and_then(Option::take) {
                    *slot = Some(owner.pages);
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::PayloadPut { input, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PayloadHash { operation, page: 0, hasher: semio_framework_hash::Hasher::new() })?;
                let Some(MemoryDbIoCursor::PayloadHash { page, hasher, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory hash cursor taxonomy mismatch".to_string()));
                };
                if let Some(fragment) = input.page(*page) {
                    hasher.update(fragment);
                    *page += 1;
                    return yield_step();
                }
                let Some(MemoryDbIoCursor::PayloadHash { hasher, .. }) = cursors[cursor_index].take() else {
                    return Err(DbError::Internal("memory hash cursor was not retained".to_string()));
                };
                let hash = ContentHash(*hasher.finalize().as_bytes());
                let mut payloads = lock(&self.payloads);
                if !payloads.iter().flatten().any(|owner| owner.hash == hash) {
                    let slot = payloads.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory payload fixed owner capacity exhausted".to_string()))?;
                    *slot = Some(MemoryPayloadOwner { hash, pages: self.retain_pages(input)? });
                }
                complete(DbIoResult::Hash(hash))
            }
            DbIoTask::PayloadGet { hash, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PageCopy { operation, page: 0 })?;
                let Some(MemoryDbIoCursor::PageCopy { page, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory payload cursor taxonomy mismatch".to_string()));
                };
                let payloads = lock(&self.payloads);
                let owner = payloads.iter().flatten().find(|owner| owner.hash == *hash).ok_or_else(|| DbError::NotFound("memory payload not found".to_string()))?;
                if let Some(fragment) = owner.pages.page(*page) {
                    if output.write_fragment(fragment)? != fragment.len() {
                        return Err(DbError::LimitExceeded("memory payload output"));
                    }
                    *page += 1;
                    return yield_step();
                }
                drop(payloads);
                match output.seal_retained_step()? {
                    Some(pages) => {
                        cursors[cursor_index] = None;
                        complete(DbIoResult::Pages(pages))
                    }
                    None => yield_step(),
                }
            }
            DbIoTask::PayloadExists { hash, .. } => complete(DbIoResult::Exists(lock(&self.payloads).iter().flatten().any(|owner| owner.hash == *hash))),
            DbIoTask::PayloadLength { hash, .. } => {
                complete(DbIoResult::Length(lock(&self.payloads).iter().flatten().find(|owner| owner.hash == *hash).map(|owner| owner.pages.len() as u64).ok_or_else(|| DbError::NotFound("memory payload not found".to_string()))?))
            }
            DbIoTask::PayloadDelete { hash, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory payload delete retirement capacity exhausted".to_string()))?;
                let mut payloads = lock(&self.payloads);
                if let Some(owner) = payloads.iter_mut().find(|owner| owner.as_ref().is_some_and(|owner| owner.hash == *hash)).and_then(Option::take) {
                    *slot = Some(owner.pages);
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::CatalogRead { output, .. } => {
                let mut cursors = lock(&self.operations);
                let catalog = lock(&self.catalog);
                let Some((owner, fence)) = catalog.as_ref() else {
                    return complete(DbIoResult::OptionalCatalog(None));
                };
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PageCopy { operation, page: 0 })?;
                let Some(MemoryDbIoCursor::PageCopy { page, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory catalog cursor taxonomy mismatch".to_string()));
                };
                if let Some(fragment) = owner.page(*page) {
                    if output.write_fragment(fragment)? != fragment.len() {
                        return Err(DbError::LimitExceeded("memory catalog output"));
                    }
                    *page += 1;
                    return yield_step();
                }
                let fence = *fence;
                drop(catalog);
                match output.seal_retained_step()? {
                    Some(pages) => {
                        cursors[cursor_index] = None;
                        complete(DbIoResult::OptionalCatalog(Some((pages, fence))))
                    }
                    None => yield_step(),
                }
            }
            DbIoTask::CatalogCas { expected, input, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory catalog replacement retirement capacity exhausted".to_string()))?;
                let mut catalog = lock(&self.catalog);
                let current = catalog.as_ref().map_or(EpochFence::INITIAL, |(_, fence)| *fence);
                expected.check(current)?;
                let next = expected.next();
                *slot = catalog.replace((self.retain_pages(input)?, next)).map(|(pages, _)| pages);
                complete(DbIoResult::Fence(next))
            }
            DbIoTask::IndexWrite { document, run_id, input, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory index replacement retirement capacity exhausted".to_string()))?;
                let mut runs = lock(&self.index_runs);
                if let Some(owner) = runs.iter_mut().flatten().find(|owner| owner.document == *document && owner.ordinal == *run_id) {
                    *slot = Some(std::mem::replace(&mut owner.pages, self.retain_pages(input)?));
                } else {
                    let target = runs.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory index fixed owner capacity exhausted".to_string()))?;
                    *target = Some(MemoryPageOwner { document: document.clone(), ordinal: *run_id, pages: self.retain_pages(input)? });
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::IndexRead { document, run_id, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PageCopy { operation, page: 0 })?;
                let Some(MemoryDbIoCursor::PageCopy { page, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory index cursor taxonomy mismatch".to_string()));
                };
                let runs = lock(&self.index_runs);
                let owner = runs.iter().flatten().find(|owner| owner.document == *document && owner.ordinal == *run_id).ok_or_else(|| DbError::NotFound("memory index run not found".to_string()))?;
                if let Some(fragment) = owner.pages.page(*page) {
                    if output.write_fragment(fragment)? != fragment.len() {
                        return Err(DbError::LimitExceeded("memory index output"));
                    }
                    *page += 1;
                    return yield_step();
                }
                drop(runs);
                match output.seal_retained_step()? {
                    Some(pages) => {
                        cursors[cursor_index] = None;
                        complete(DbIoResult::Pages(pages))
                    }
                    None => yield_step(),
                }
            }
            DbIoTask::IndexDelete { document, run_id, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory index delete retirement capacity exhausted".to_string()))?;
                let mut runs = lock(&self.index_runs);
                if let Some(owner) = runs.iter_mut().find(|owner| owner.as_ref().is_some_and(|owner| owner.document == *document && owner.ordinal == *run_id)).and_then(Option::take) {
                    *slot = Some(owner.pages);
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::IndexList { document, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::List { operation, after: None })?;
                let Some(MemoryDbIoCursor::List { after, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory index list cursor taxonomy mismatch".to_string()));
                };
                let next = lock(&self.index_runs).iter().flatten().filter(|owner| owner.document == *document && after.is_none_or(|after| owner.ordinal > after)).map(|owner| owner.ordinal).min();
                if let Some(value) = next {
                    output.push(value)?;
                    *after = Some(value);
                    return yield_step();
                }
                cursors[cursor_index] = None;
                complete(DbIoResult::List(output.take_for_result()))
            }
            DbIoTask::LeaseAcquire { document, holder, ttl_ms, now_ms, .. } => {
                let mut leases = lock(&self.leases);
                let existing = leases.iter_mut().flatten().find(|owner| owner.resource == *document);
                let fence = match existing.as_ref() {
                    Some(info) if *now_ms < info.expires_at_ms && info.holder != *holder => return Err(DbError::Conflict("memory lease is owned by another holder".to_string())),
                    Some(info) if *now_ms < info.expires_at_ms => info.fence,
                    Some(info) => info.fence.next(),
                    None => EpochFence::INITIAL,
                };
                if let Some(info) = existing {
                    info.holder = holder.clone();
                    info.fence = fence;
                    info.expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("memory lease expiry"))?;
                } else {
                    let slot = leases.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory lease fixed owner capacity exhausted".to_string()))?;
                    *slot = Some(MemoryLeaseOwner { resource: document.clone(), holder: holder.clone(), fence, expires_at_ms: now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("memory lease expiry"))? });
                }
                complete(DbIoResult::Fence(fence))
            }
            DbIoTask::LeaseRenew { document, holder, fence, ttl_ms, now_ms, .. } => {
                let mut leases = lock(&self.leases);
                let info = leases.iter_mut().flatten().find(|owner| owner.resource == *document).ok_or_else(|| DbError::NotFound("memory lease not found".to_string()))?;
                if *now_ms >= info.expires_at_ms || info.holder != *holder {
                    return Err(DbError::Unauthorized("memory lease owner or expiry mismatch".to_string()));
                }
                fence.check(info.fence)?;
                info.expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("memory lease expiry"))?;
                complete(DbIoResult::Unit)
            }
            DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                let mut cursors = lock(&self.operations);
                let mut leases = lock(&self.leases);
                let cursor_index = if let Some(index) = cursors.iter().position(|cursor| cursor.as_ref().is_some_and(|cursor| cursor.operation() == operation)) {
                    index
                } else {
                    let index = leases.iter().position(|owner| owner.as_ref().is_some_and(|owner| owner.resource == *document)).ok_or_else(|| DbError::NotFound("memory lease not found".to_string()))?;
                    let info = leases[index].as_ref().ok_or_else(|| DbError::Internal("memory lease index lost owner".to_string()))?;
                    if info.holder != *holder {
                        return Err(DbError::Unauthorized("memory lease owner mismatch".to_string()));
                    }
                    fence.check(info.fence)?;
                    memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::LeaseRelease { operation, index: index as u16 })?
                };
                let Some(MemoryDbIoCursor::LeaseRelease { index, .. }) = cursors[cursor_index].as_ref() else {
                    return Err(DbError::Internal("memory lease release cursor taxonomy mismatch".to_string()));
                };
                let index = usize::from(*index);
                let owner = leases[index].as_mut().ok_or_else(|| DbError::Internal("memory lease release lost owner".to_string()))?;
                if owner.holder.close_step() {
                    return yield_step();
                }
                if owner.resource.close_step() {
                    return yield_step();
                }
                leases[index] = None;
                cursors[cursor_index] = None;
                complete(DbIoResult::Unit)
            }
            DbIoTask::LeaseGet { document, now_ms, .. } => {
                let leases = lock(&self.leases);
                let result = leases.iter().flatten().find(|owner| owner.resource == *document).filter(|info| *now_ms < info.expires_at_ms).map(|info| DbIoLeaseResult::new(info.resource.clone(), info.holder.clone(), info.fence, info.expires_at_ms));
                complete(DbIoResult::OptionalLease(result))
            }
            DbIoTask::BackendClose { .. } => complete(DbIoResult::Unit),
        }
    }

    fn close_operation_step(&self, operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        let mut operations = lock(&self.operations);
        if let Some(index) = operations.iter().position(|cursor| cursor.as_ref().is_some_and(|cursor| cursor.operation() == operation)) {
            operations[index] = None;
            return Ok(false);
        }
        if self.retirement_step()? {
            return Ok(false);
        }
        Ok(true)
    }

    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        {
            let mut writers = lock(&self.writers);
            if let Some(table) = writers.as_mut() {
                if table.close_step()? {
                    return Ok(false);
                }
                if !table.terminal_is_empty() {
                    return Err(DbError::Internal("memory writer table lost terminal witness".to_string()));
                }
                writers.take();
                return Ok(false);
            }
        }
        if let Some(cursor) = lock(&self.operations).iter_mut().find(|cursor| cursor.is_some()) {
            *cursor = None;
            return Ok(false);
        }
        MemoryDbIoExecutor::close_backend_step(self)
    }

    fn backend_terminal_is_empty(&self) -> bool {
        lock(&self.writers).is_none() && lock(&self.operations).iter().all(Option::is_none) && MemoryDbIoExecutor::backend_terminal_is_empty(self)
    }
}

pub struct MemoryStorage {
    pub control: DbIoBackendControl,
    pub pool: Arc<WorkerPool>,
    pub closed: std::sync::atomic::AtomicBool,
}

async fn memory_execute(task: DbIoTask) -> Result<DbIoResult, DbError> {
    submit_db_io_task_admitted(task).await?.finish().await
}

fn memory_document(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn memory_output(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("memory output bytes"))?.div_ceil(DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

impl MemoryStorage {
    /// 🧠️ Opens one memory backend through [`open_db_io_backend_admitted`] (waits while the process's backend
    /// capacity is taken by backends still retiring).
    pub async fn new(pool: Arc<WorkerPool>) -> Result<Self, DbStorageOpenRejected> {
        open_db_io_backend_admitted(&pool, || Self::open_once(pool.clone())).await
    }

    /// 🎯️ One memory backend open attempt, refused at once when the backend capacity is taken.
    pub async fn open_once(pool: Arc<WorkerPool>) -> Result<Self, DbStorageOpenRejected> {
        let rollback = DbIoBackendRollbackReservation::try_reserve()?;
        let pool_use = pool.acquire_use().map_err(|error| DbError::Unavailable(format!("memory DB I/O backend WorkerPool use rejected: {error:?}")))?;
        let credit = DbIoCredit { pages: 0, bytes: MemoryDbIoExecutor::backing_bytes(), items: 1, controls: 1 }.checked_add(db_io_registry::controller_credit()).expect("fixed memory writer control credit");
        let owner = db_io_backend_owner_reserve(credit)?;
        let executor = Box::new(MemoryDbIoExecutor::default());
        let control = register_db_io_backend_reserved_with_use(DbIoBackendKind::Memory, executor, pool.clone(), pool_use, owner, credit, rollback).map_err(|failure| DbIoBackendRegistrationRejected {
            cause: failure.cause,
            cleanup_error: None,
            owner: Some(DbIoBackendRegistrationRejectedOwner::Retiring(failure.retirement)),
        })?;
        let storage = Self { control, pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = memory_execute(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("memory://fixed")? }).await {
            return Err(DbStorageOpenRejected::registered(error, control));
        }
        Ok(storage)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn fail_next_writer_release(&self) {
        let (slot, generation) = db_io_backend_parts(self.control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        assert_eq!(owner.generation, generation);
        let executor = owner.executor.as_mut().unwrap().as_any_mut().downcast_mut::<MemoryDbIoExecutor>().unwrap();
        executor.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().unwrap().fail_next_release();
    }

    pub async fn close(&self) -> Result<(), DbError> {
        let result = match memory_execute(DbIoTask::BackendClose { backend: self.control }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory backend close returned the wrong typed result".to_string())),
        };
        if result.is_ok() {
            close_db_io_backend(self.control).await?;
            self.closed.store(true, std::sync::atomic::Ordering::Release);
        }
        result
    }

    pub async fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true }
    }
}

impl Drop for MemoryStorage {
    fn drop(&mut self) {
        if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = retire_db_io_backend(self.control);
        }
    }
}

impl WalStorage for MemoryStorage {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError> {
        let document = memory_document(document)?;
        db_io_driver::admitted_acquire(self.control, std::time::Duration::from_millis(db_io_driver::WAL_WRITER_ADMISSION_WAIT_MS), || async {
            match memory_execute(DbIoTask::WalWriterAcquire { backend: self.control, document: document.clone() }).await? {
                DbIoResult::WalWriter(writer) => Ok(writer),
                _ => Err(DbError::Internal("memory WAL writer result taxonomy".to_string())),
            }
        })
        .await
    }
    async fn create_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalCreate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL create result taxonomy".to_string())),
        }
    }
    async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match memory_execute(DbIoTask::WalAppend { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, input: bytes }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL append result taxonomy".to_string())),
        }
    }
    async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalSync { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, class }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL sync result taxonomy".to_string())),
        }
    }
    async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalSeal { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL seal result taxonomy".to_string())),
        }
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        match memory_execute(DbIoTask::WalRead { backend: self.control, document: memory_document(document)?, index, range, output: memory_output(range.len)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL read result taxonomy".to_string())),
        }
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match memory_execute(DbIoTask::WalLength { backend: self.control, document: memory_document(document)?, index }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL length result taxonomy".to_string())),
        }
    }
    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        match memory_execute(DbIoTask::WalState { backend: self.control, document: memory_document(document)?, index }).await? {
            DbIoResult::WalSegmentState(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL state result taxonomy".to_string())),
        }
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(DbIoTask::WalList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL list result taxonomy".to_string())),
        }
    }
    async fn truncate_tail(&self, writer: &WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalTruncate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, new_len }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL truncate result taxonomy".to_string())),
        }
    }
    async fn delete_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalDelete { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL delete result taxonomy".to_string())),
        }
    }
}

impl SnapshotStorage for MemoryStorage {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match memory_execute(DbIoTask::SnapshotWrite { backend: self.control, document: memory_document(document)?, generation, input: bytes }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory snapshot write result taxonomy".to_string())),
        }
    }
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        match memory_execute(DbIoTask::SnapshotRead { backend: self.control, document: memory_document(document)?, generation, output: memory_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot read result taxonomy".to_string())),
        }
    }
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match memory_execute(DbIoTask::SnapshotLatest { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::OptionalLength(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot latest result taxonomy".to_string())),
        }
    }
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(DbIoTask::SnapshotList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot list result taxonomy".to_string())),
        }
    }
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::SnapshotDelete { backend: self.control, document: memory_document(document)?, generation }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory snapshot delete result taxonomy".to_string())),
        }
    }
}

impl PayloadStorage for MemoryStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match memory_execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload put result taxonomy".to_string())),
        }
    }
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        match memory_execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: memory_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload get result taxonomy".to_string())),
        }
    }
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match memory_execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload exists result taxonomy".to_string())),
        }
    }
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        match memory_execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory payload delete result taxonomy".to_string())),
        }
    }
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match memory_execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload length result taxonomy".to_string())),
        }
    }
}

impl CatalogStorage for MemoryStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match memory_execute(DbIoTask::CatalogRead { backend: self.control, output: memory_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(value) => Ok(value),
            _ => Err(DbError::Internal("memory catalog read result taxonomy".to_string())),
        }
    }
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match memory_execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(value) => Ok(value),
            _ => Err(DbError::Internal("memory catalog CAS result taxonomy".to_string())),
        }
    }
}

impl IndexStorage for MemoryStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match memory_execute(DbIoTask::IndexWrite { backend: self.control, document: memory_document(document)?, run_id, input: bytes }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory index write result taxonomy".to_string())),
        }
    }
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        match memory_execute(DbIoTask::IndexRead { backend: self.control, document: memory_document(document)?, run_id, output: memory_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory index read result taxonomy".to_string())),
        }
    }
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(DbIoTask::IndexList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory index list result taxonomy".to_string())),
        }
    }
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::IndexDelete { backend: self.control, document: memory_document(document)?, run_id }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory index delete result taxonomy".to_string())),
        }
    }
}

impl LeaseStorage for MemoryStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match memory_execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, ttl_ms, now_ms }).await? {
            DbIoResult::Fence(value) => Ok(value),
            _ => Err(DbError::Internal("memory lease acquire result taxonomy".to_string())),
        }
    }
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, ttl_ms, now_ms }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory lease renew result taxonomy".to_string())),
        }
    }
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        match memory_execute(DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory lease release result taxonomy".to_string())),
        }
    }
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match memory_execute(DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
            DbIoResult::OptionalLease(value) => Ok(value),
            _ => Err(DbError::Internal("memory lease get result taxonomy".to_string())),
        }
    }
}
//#endregion 🔖️Memory

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"]
mod db_io_retained_fixtures;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
