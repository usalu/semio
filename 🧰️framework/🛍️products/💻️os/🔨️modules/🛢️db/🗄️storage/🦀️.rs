//! 🗄️ `db_storage` — the pluggable storage substrate seam for the `db` crate family: the trait
//! family (`WalStorage`, `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`,
//! `IndexStorage`, `LeaseStorage`) every backend (this crate's `MemoryStorage`/`FsStorage`, plus
//! the sibling `db_storage_sqlite`/`db_storage_postgres`/`db_storage_neo4j` modules) implements
//! identically, so `db_engine`/the `db` facade select a backend via [`DbBackend`] at
//! `Database::open` rather than at compile time. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: this crate has no opinion on WAL record framing (`db_wal` reuses
//! `protocol::{SprWriter, FrameCursor, recover}` on top of `WalStorage`'s raw segment bytes) or
//! snapshot pack-file structure (`db_snapshot` builds `.spk` bytes handed to `SnapshotStorage`
//! whole) — every trait here stores/retrieves opaque byte blobs keyed by document + a small
//! integer, plus the two primitives (`CatalogStorage::cas_root`, `LeaseStorage`) that need
//! fencing semantics of their own. This keeps the trait family stable while the format built on
//! top of it (owned by `db_wal`/`db_snapshot`/`db_index`) can evolve independently.
//!
//! ⏳️ **Async-first, zero-dyn (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet
//! `db-dedyn`, ruling **O1**)**: every sub-trait method is a plain `async fn` returning
//! `Result<T, DbError>` directly — never a boxed `dyn Future` in trait-method return position
//! (ruling **R1**; that shape, `DbFuture<'a, T> = Pin<Box<dyn Future<..> + 'a>>`, was this family's
//! PREVIOUS state and is now deleted). Because `async fn` in a trait is not `dyn`-compatible, the
//! old `Arc<dyn DbStorage>`/`&dyn WalStorage` seams are gone too: [`DbBackend`] is a concrete enum
//! naming every backend (this crate has exactly one layer, so no layering problem — see
//! `//#region 🔖️DbBackend`), and its accessors return concrete facet-ref enums
//! ([`WalRef`]/[`SnapshotRef`]/[`PayloadRef`]/[`CatalogRef`]/[`IndexRef`]/[`LeaseRef`]) instead of
//! trait objects. `Send` on a spawned future is therefore obtained STRUCTURALLY, from the concrete
//! enum variant the compiler already knows at every call site — never from a `+ Send` bound on a
//! trait method (ruling **R3**). `#![allow(async_fn_in_trait)]` at the crate root suppresses the
//! resulting "auto trait bounds" lint (ruling **R7**): that lint's suggested fix
//! (`-> impl Future<..> + Send`) is exactly the bound R3 forbids.
//!
//! Backends split two ways: genuinely-async drivers (`db_storage_postgres`'s `sqlx`,
//! `db_storage_neo4j`'s `neo4rs`) simply `.await` their already-async bodies; genuinely-blocking
//! backends (this crate's own `FsStorage`, plus the sibling `db_storage_sqlite`) cross the
//! sync/async boundary through schema-first [`DbIoTask`] owners submitted to the ONE
//! process-wide `semio_framework_async::WorkerPool` on
//! `Lane::Io` — never a private `tokio::runtime` (this crate names no `tokio` at all; see the
//! repo's "`tokio` only in `🛎️services`" rule) and never `HostAsyncRuntime::run_blocking` (removed
//! from that trait — Phase 1 of `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`; callers now submit
//! directly to a `WorkerPool`). `MemoryStorage` (no real I/O) simply resolves immediately.
//! `FsStorage`/`SqliteStorage` construction requires that process pool; no pool-less or inline
//! blocking fallback exists, including for CLI and batch entry points.
//!
//! 🧊️ `FsStorage` (this crate's zero-touch default, behind the default `fs` feature) is native-only
//! (`std::fs`) and `#[cfg(not(target_arch = "wasm32"))]`-gated, mirroring `pack`'s own `pack_io`
//! convention — it compiles to an effectively-empty module on a `wasm32-unknown-unknown` target
//! check. `MemoryStorage` has no such gate and is always available.

use crate::db_durability::{DurabilityClass, EpochFence};
use crate::db_ids::{check_len, ArtifactId, DbError};
use crate::*;
use pack::{ByteRange, ContentHash};
use semio_framework_async::{Job, Lane, WorkerPool, WorkerSubmitErrorKind};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// @emoji 📊️ One process-wide typed DB I/O task depth signal.
static BLOCKING_QUEUE: semio_framework_trace::QueueCounter = semio_framework_trace::QueueCounter::new();

//#region 🔖️Limits
/// @emoji 🛡️ Ceiling on any single blob this crate reads into memory in one call (one WAL read
/// range, one snapshot generation, one payload, one index run, one lease record) — validated via
/// `check_len` BEFORE the read buffer is allocated, mirroring `pack_core`'s stated
/// invariant. This crate's own choice (the contract doesn't fix a number): generous enough for a
/// snapshot generation or a large payload, small enough to refuse an obviously-corrupt on-disk
/// length before trying to allocate it.
const MAX_READ_BYTES: u64 = 496 * 1024;

pub const DB_IO_PAGE_BYTES: usize = 16 * 1024;
pub const DB_IO_OPERATION_PAGES: usize = 64;
const DB_IO_TOTAL_PAGES: usize = 1024;
const DB_IO_PAGE_RETIREMENT_SLOTS: usize = DB_IO_TOTAL_PAGES * 2;
pub const DB_IO_OPERATION_ITEMS: usize = 64;
const DB_IO_BACKEND_CONTROLS: usize = 64;
const DB_IO_LEDGER_ITEMS: usize = DB_IO_OPERATION_ITEMS + DB_IO_BACKEND_CONTROLS;
const DB_IO_TASK_SLOT_BYTES: u64 = std::mem::size_of::<DbIoTaskSlot>() as u64;
const DB_IO_LIST_BACKING_BYTES: u64 = (DB_IO_LIST_ITEMS * std::mem::size_of::<u64>()) as u64;
const DB_IO_LIST_TRANSIENT_BYTES: u64 = DB_IO_LIST_BACKING_BYTES * 2;
const DB_IO_OPERATION_BYTES: u64 = (DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES * 4) as u64 + DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES;
const DB_IO_OPERATION_ITEM_CREDIT: usize = DB_IO_LIST_ITEMS * 2 + DB_IO_OPERATION_PAGES + 32;
const DB_IO_OPERATION_CONTROL_CREDIT: usize = 16;
const DB_IO_PROCESS_BYTES: u64 =
    (DB_IO_PAGE_BYTES * DB_IO_TOTAL_PAGES * 2 + DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES * DB_IO_OPERATION_ITEMS * 2) as u64 + DB_IO_OPERATION_ITEMS as u64 * (DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES);
const DB_IO_PROCESS_ITEM_CREDIT: usize = DB_IO_OPERATION_ITEMS * DB_IO_OPERATION_ITEM_CREDIT;
const DB_IO_PROCESS_CONTROL_CREDIT: usize = DB_IO_OPERATION_ITEMS * DB_IO_OPERATION_CONTROL_CREDIT;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct DbIoCredit {
    pages: usize,
    bytes: u64,
    items: usize,
    controls: usize,
}

impl DbIoCredit {
    fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self { pages: self.pages.checked_add(other.pages)?, bytes: self.bytes.checked_add(other.bytes)?, items: self.items.checked_add(other.items)?, controls: self.controls.checked_add(other.controls)? })
    }

    fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self { pages: self.pages.checked_sub(other.pages)?, bytes: self.bytes.checked_sub(other.bytes)?, items: self.items.checked_sub(other.items)?, controls: self.controls.checked_sub(other.controls)? })
    }

    fn is_zero(self) -> bool {
        self == Self::default()
    }
}

#[derive(Clone, Copy)]
struct DbIoOperationCreditSlot {
    generation: u64,
    operation: u64,
    live: DbIoCredit,
    result_leases: u16,
    task_attached: bool,
    backend_owner: bool,
}

const EMPTY_DB_IO_OPERATION_CREDIT_SLOT: DbIoOperationCreditSlot = DbIoOperationCreditSlot { generation: 0, operation: 0, live: DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 0 }, result_leases: 0, task_attached: false, backend_owner: false };

struct DbIoOperationLedger {
    slots: [DbIoOperationCreditSlot; DB_IO_LEDGER_ITEMS],
    free: [u16; DB_IO_LEDGER_ITEMS],
    free_read: usize,
    free_len: usize,
    totals: DbIoCredit,
    next_operation: u64,
    next_generation: u64,
}

impl DbIoOperationLedger {
    fn new() -> Self {
        Self { slots: [EMPTY_DB_IO_OPERATION_CREDIT_SLOT; DB_IO_LEDGER_ITEMS], free: std::array::from_fn(|index| index as u16), free_read: 0, free_len: DB_IO_LEDGER_ITEMS, totals: DbIoCredit::default(), next_operation: 1, next_generation: 1 }
    }
}

static DB_IO_OPERATION_LEDGER: std::sync::OnceLock<std::sync::Mutex<DbIoOperationLedger>> = std::sync::OnceLock::new();

fn db_io_operation_ledger() -> &'static std::sync::Mutex<DbIoOperationLedger> {
    DB_IO_OPERATION_LEDGER.get_or_init(|| std::sync::Mutex::new(DbIoOperationLedger::new()))
}

fn db_io_credit_within_limits(credit: DbIoCredit, process: bool) -> bool {
    if process {
        credit.pages <= DB_IO_TOTAL_PAGES && credit.bytes <= DB_IO_PROCESS_BYTES && credit.items <= DB_IO_PROCESS_ITEM_CREDIT && credit.controls <= DB_IO_PROCESS_CONTROL_CREDIT
    } else {
        credit.pages <= DB_IO_OPERATION_PAGES && credit.bytes <= DB_IO_OPERATION_BYTES && credit.items <= DB_IO_OPERATION_ITEM_CREDIT && credit.controls <= DB_IO_OPERATION_CONTROL_CREDIT
    }
}

fn db_io_operation_slot(ledger: &DbIoOperationLedger, operation: u64) -> Option<usize> {
    ledger.slots.iter().position(|slot| slot.operation == operation && slot.generation != 0)
}

fn db_io_operation_reserve(initial: DbIoCredit) -> Result<u64, DbError> {
    if !db_io_credit_within_limits(initial, false) {
        return Err(DbError::LimitExceeded("db_io operation aggregate credit"));
    }
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let totals = ledger.totals.checked_add(initial).ok_or(DbError::LimitExceeded("db_io process aggregate credit"))?;
    if ledger.free_len == 0 || ledger.next_operation == u64::MAX || ledger.next_generation == u64::MAX || !db_io_credit_within_limits(totals, true) {
        return Err(DbError::Unavailable("DB I/O process aggregate credit exhausted".to_string()));
    }
    let slot = ledger.free[ledger.free_read];
    ledger.free_read = (ledger.free_read + 1) % DB_IO_LEDGER_ITEMS;
    ledger.free_len -= 1;
    let operation = ledger.next_operation;
    ledger.next_operation += 1;
    let generation = ledger.next_generation;
    ledger.next_generation += 1;
    ledger.slots[slot as usize] = DbIoOperationCreditSlot { generation, operation, live: initial, result_leases: 0, task_attached: false, backend_owner: false };
    ledger.totals = totals;
    Ok(operation)
}

fn db_io_backend_owner_reserve(initial: DbIoCredit) -> Result<u64, DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let totals = ledger.totals.checked_add(initial).ok_or(DbError::LimitExceeded("DB I/O backend process credit"))?;
    if ledger.free_len == 0 || ledger.next_operation == u64::MAX || ledger.next_generation == u64::MAX || !db_io_credit_within_limits(totals, true) {
        return Err(DbError::Unavailable("DB I/O backend process credit exhausted".to_string()));
    }
    let slot = ledger.free[ledger.free_read];
    ledger.free_read = (ledger.free_read + 1) % DB_IO_LEDGER_ITEMS;
    ledger.free_len -= 1;
    let operation = ledger.next_operation;
    ledger.next_operation += 1;
    let generation = ledger.next_generation;
    ledger.next_generation += 1;
    ledger.slots[slot as usize] = DbIoOperationCreditSlot { generation, operation, live: initial, result_leases: 0, task_attached: false, backend_owner: true };
    ledger.totals = totals;
    Ok(operation)
}

fn db_io_operation_add(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O aggregate operation is not live".to_string()))?;
    let operation_total = ledger.slots[index].live.checked_add(credit).ok_or(DbError::LimitExceeded("db_io operation aggregate credit"))?;
    let process_total = ledger.totals.checked_add(credit).ok_or(DbError::LimitExceeded("db_io process aggregate credit"))?;
    if !db_io_credit_within_limits(operation_total, false) || !db_io_credit_within_limits(process_total, true) {
        return Err(DbError::Unavailable("DB I/O aggregate admission exhausted".to_string()));
    }
    ledger.slots[index].live = operation_total;
    ledger.totals = process_total;
    Ok(())
}

fn db_io_operation_attach_task(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O task lost aggregate operation".to_string()))?;
    if ledger.slots[index].task_attached {
        return Err(DbError::Internal("DB I/O aggregate operation already has a task".to_string()));
    }
    let operation_total = ledger.slots[index].live.checked_add(credit).ok_or(DbError::LimitExceeded("db_io operation aggregate credit"))?;
    let process_total = ledger.totals.checked_add(credit).ok_or(DbError::LimitExceeded("db_io process aggregate credit"))?;
    if !db_io_credit_within_limits(operation_total, false) || !db_io_credit_within_limits(process_total, true) {
        return Err(DbError::Unavailable("DB I/O aggregate task admission exhausted".to_string()));
    }
    ledger.slots[index].live = operation_total;
    ledger.slots[index].task_attached = true;
    ledger.totals = process_total;
    Ok(())
}

fn db_io_operation_mark_task(operation: u64) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O task lost aggregate operation".to_string()))?;
    if ledger.slots[index].task_attached {
        return Err(DbError::Internal("DB I/O aggregate operation already has a task".to_string()));
    }
    ledger.slots[index].task_attached = true;
    Ok(())
}

fn db_io_operation_detach_task(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O aggregate task return lost operation".to_string()))?;
    if !ledger.slots[index].task_attached {
        return Err(DbError::Internal("DB I/O aggregate task returned twice".to_string()));
    }
    ledger.slots[index].live = ledger.slots[index].live.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O aggregate task credit returned twice".to_string()))?;
    ledger.slots[index].task_attached = false;
    ledger.totals = ledger.totals.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O process task credit returned twice".to_string()))?;
    db_io_operation_try_release_locked(&mut ledger, index)
}

fn db_io_operation_terminal_is_empty(operation: u64) -> bool {
    let ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    db_io_operation_slot(&ledger, operation).is_none()
}

fn db_io_operation_add_result_lease(operation: u64) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O result lost aggregate operation".to_string()))?;
    let credit = db_io_result_lease_credit();
    let operation_total = ledger.slots[index].live.checked_add(credit).ok_or(DbError::LimitExceeded("db_io result lease aggregate credit"))?;
    let process_total = ledger.totals.checked_add(credit).ok_or(DbError::LimitExceeded("db_io result lease process credit"))?;
    if !db_io_credit_within_limits(operation_total, false) || !db_io_credit_within_limits(process_total, true) {
        return Err(DbError::Unavailable("DB I/O result lease admission exhausted".to_string()));
    }
    ledger.slots[index].result_leases = ledger.slots[index].result_leases.checked_add(1).ok_or(DbError::LimitExceeded("db_io result leases"))?;
    ledger.slots[index].live = operation_total;
    ledger.totals = process_total;
    Ok(())
}

fn db_io_operation_return_result_lease(operation: u64) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O result handback lost aggregate operation".to_string()))?;
    ledger.slots[index].result_leases = ledger.slots[index].result_leases.checked_sub(1).ok_or_else(|| DbError::Internal("DB I/O result lease returned twice".to_string()))?;
    let credit = db_io_result_lease_credit();
    ledger.slots[index].live = ledger.slots[index].live.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O result lease credit returned twice".to_string()))?;
    ledger.totals = ledger.totals.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O process result lease credit returned twice".to_string()))?;
    db_io_operation_try_release_locked(&mut ledger, index)
}

fn db_io_operation_return(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O aggregate return lost operation".to_string()))?;
    ledger.slots[index].live = ledger.slots[index].live.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O aggregate credit returned twice".to_string()))?;
    ledger.totals = ledger.totals.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O process credit returned twice".to_string()))?;
    db_io_operation_try_release_locked(&mut ledger, index)
}

fn db_io_operation_transfer_to_backend(from: u64, to: u64, credit: DbIoCredit) -> Result<(), DbError> {
    if from == to {
        return Ok(());
    }
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let from_index = db_io_operation_slot(&ledger, from).ok_or_else(|| DbError::Internal("DB I/O transfer lost source operation".to_string()))?;
    let to_index = db_io_operation_slot(&ledger, to).ok_or_else(|| DbError::Internal("DB I/O transfer lost backend operation".to_string()))?;
    if !ledger.slots[to_index].backend_owner {
        return Err(DbError::Internal("DB I/O page transfer target is not a backend owner".to_string()));
    }
    let from_live = ledger.slots[from_index].live.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O page transfer exceeded source credit".to_string()))?;
    let to_live = ledger.slots[to_index].live.checked_add(credit).ok_or(DbError::LimitExceeded("DB I/O backend retained page credit"))?;
    if !db_io_credit_within_limits(ledger.totals, true) {
        return Err(DbError::Unavailable("DB I/O backend retained page process credit exhausted".to_string()));
    }
    ledger.slots[from_index].live = from_live;
    ledger.slots[to_index].live = to_live;
    db_io_operation_try_release_locked(&mut ledger, from_index)
}

fn db_io_operation_try_release_locked(ledger: &mut DbIoOperationLedger, index: usize) -> Result<(), DbError> {
    if !ledger.slots[index].live.is_zero() || ledger.slots[index].result_leases != 0 {
        return Ok(());
    }
    let slot = index as u16;
    ledger.slots[index] = EMPTY_DB_IO_OPERATION_CREDIT_SLOT;
    let write = (ledger.free_read + ledger.free_len) % DB_IO_LEDGER_ITEMS;
    ledger.free[write] = slot;
    ledger.free_len += 1;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DbIoPagePhase {
    Free,
    CheckedOutInput,
    CheckedOutWriter,
    Queued,
    Executing,
    TerminalResult,
    Rejected,
    Closing,
}

struct DbIoPageBacking(std::cell::UnsafeCell<[u8; DB_IO_PAGE_BYTES]>);

impl DbIoPageBacking {
    const fn new() -> Self {
        Self(std::cell::UnsafeCell::new([0; DB_IO_PAGE_BYTES]))
    }
}

unsafe impl Sync for DbIoPageBacking {}

static DB_IO_PAGE_BACKINGS: [DbIoPageBacking; DB_IO_TOTAL_PAGES] = [const { DbIoPageBacking::new() }; DB_IO_TOTAL_PAGES];

#[derive(Clone, Copy)]
struct DbIoPageSlot {
    generation: u64,
    operation: u64,
    phase: DbIoPagePhase,
}

const EMPTY_DB_IO_PAGE_SLOT: DbIoPageSlot = DbIoPageSlot { generation: 0, operation: 0, phase: DbIoPagePhase::Free };

struct DbIoPageArenaState {
    slots: [DbIoPageSlot; DB_IO_TOTAL_PAGES],
    free: [u16; DB_IO_TOTAL_PAGES],
    free_read: usize,
    free_len: usize,
    retired: [Option<(u16, u64)>; DB_IO_PAGE_RETIREMENT_SLOTS],
    retired_read: usize,
    retired_len: usize,
    next_generation: u64,
}

impl DbIoPageArenaState {
    fn new() -> Self {
        Self {
            slots: [EMPTY_DB_IO_PAGE_SLOT; DB_IO_TOTAL_PAGES],
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: DB_IO_TOTAL_PAGES,
            retired: [None; DB_IO_PAGE_RETIREMENT_SLOTS],
            retired_read: 0,
            retired_len: 0,
            next_generation: 1,
        }
    }
}

static DB_IO_PAGE_ARENA: std::sync::OnceLock<std::sync::Mutex<DbIoPageArenaState>> = std::sync::OnceLock::new();

fn db_io_page_arena() -> &'static std::sync::Mutex<DbIoPageArenaState> {
    DB_IO_PAGE_ARENA.get_or_init(|| std::sync::Mutex::new(DbIoPageArenaState::new()))
}

fn db_io_page_credit(count: usize) -> DbIoCredit {
    DbIoCredit { pages: count, bytes: (count * DB_IO_PAGE_BYTES) as u64, items: count, controls: 0 }
}

fn db_io_page_shell_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: 0, items: 1, controls: 1 }
}

#[derive(Debug)]
struct DbIoPageLease {
    slot: u16,
    generation: u64,
    operation: u64,
    used: u16,
    returned: bool,
}

impl DbIoPageLease {
    fn validate_identity(&self, slot: DbIoPageSlot) -> Result<(), DbError> {
        if slot.generation != self.generation {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.generation), actual: crate::db_ids::GenerationId(slot.generation) });
        }
        if slot.operation != self.operation {
            return Err(DbError::Internal(format!("DB I/O page operation mismatch: expected {}, got {} at generation {}", self.operation, slot.operation, self.generation)));
        }
        Ok(())
    }

    fn validate_phase(&self, slot: DbIoPageSlot, expected: DbIoPagePhase) -> Result<(), DbError> {
        self.validate_identity(slot)?;
        if slot.phase != expected {
            return Err(DbError::Internal(format!("DB I/O page phase mismatch: expected {expected:?}, got {:?} for operation {} generation {}", slot.phase, self.operation, self.generation)));
        }
        Ok(())
    }

    fn phase(&self) -> Result<DbIoPagePhase, DbError> {
        let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = state.slots[self.slot as usize];
        self.validate_identity(slot)?;
        Ok(slot.phase)
    }

    fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        self.validate_phase(*slot, expected)?;
        slot.phase = next;
        Ok(())
    }

    fn bytes(&self) -> &[u8] {
        let used = usize::from(self.used);
        unsafe { &(&*DB_IO_PAGE_BACKINGS[self.slot as usize].0.get())[..used] }
    }

    fn write(&mut self, offset: usize, source: &[u8]) -> Result<(), DbError> {
        let end = offset.checked_add(source.len()).ok_or(DbError::LimitExceeded("db_io page write"))?;
        if end > DB_IO_PAGE_BYTES || offset > usize::from(self.used) {
            return Err(DbError::LimitExceeded("db_io page writer reservation"));
        }
        let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = state.slots[self.slot as usize];
        self.validate_identity(slot)?;
        if !matches!(slot.phase, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
            return Err(DbError::Internal(format!("DB I/O page phase mismatch: expected CheckedOutWriter or Executing, got {:?} for operation {} generation {}", slot.phase, self.operation, self.generation)));
        }
        unsafe { (&mut *DB_IO_PAGE_BACKINGS[self.slot as usize].0.get())[offset..end].copy_from_slice(source) };
        self.used = self.used.max(end as u16);
        Ok(())
    }

    fn return_to_arena(mut self) -> Result<usize, DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        self.validate_phase(*slot, DbIoPagePhase::Closing)?;
        *slot = EMPTY_DB_IO_PAGE_SLOT;
        let write = (state.free_read + state.free_len) % DB_IO_TOTAL_PAGES;
        state.free[write] = self.slot;
        state.free_len += 1;
        self.returned = true;
        drop(state);
        db_io_operation_return(self.operation, db_io_page_credit(1))?;
        Ok(DB_IO_PAGE_BYTES)
    }

    fn install_lost_handle(&mut self) {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        if slot.generation != self.generation || slot.operation != self.operation || slot.phase == DbIoPagePhase::Free {
            return;
        }
        slot.phase = DbIoPagePhase::Closing;
        if state.retired_len >= DB_IO_TOTAL_PAGES {
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        }
        let write = (state.retired_read + state.retired_len) % DB_IO_PAGE_RETIREMENT_SLOTS;
        state.retired[write] = Some((self.slot, self.generation));
        state.retired_len += 1;
        self.returned = true;
    }
}

impl Drop for DbIoPageLease {
    fn drop(&mut self) {
        if !self.returned {
            self.install_lost_handle();
        }
    }
}

fn db_io_checkout_pages(operation: u64, count: usize, phase: DbIoPagePhase) -> Result<[Option<DbIoPageLease>; DB_IO_OPERATION_PAGES], DbError> {
    if operation == 0 || count > DB_IO_OPERATION_PAGES {
        return Err(DbError::LimitExceeded("db_io operation page credit"));
    }
    let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    db_io_preflight_page_checkout(&state, count)?;
    let mut leases: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES] = std::array::from_fn(|_| None);
    for lease in leases.iter_mut().take(count) {
        let slot = state.free[state.free_read];
        state.free_read = (state.free_read + 1) % DB_IO_TOTAL_PAGES;
        state.free_len -= 1;
        let generation = state.next_generation;
        state.next_generation += 1;
        state.slots[slot as usize] = DbIoPageSlot { generation, operation, phase };
        *lease = Some(DbIoPageLease { slot, generation, operation, used: 0, returned: false });
    }
    Ok(leases)
}

fn db_io_preflight_page_checkout(state: &DbIoPageArenaState, count: usize) -> Result<(), DbError> {
    if count > state.free_len || state.next_generation.checked_add(count as u64).is_none() {
        return Err(DbError::Unavailable("db I/O process page capacity exhausted".to_string()));
    }
    Ok(())
}

/// @emoji ✍️ Retained writer backed only by exact fixed arena page leases.
pub struct DbIoPageWriter {
    operation: u64,
    pages: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES],
    reserved: u8,
    cursor: u8,
    total_len: usize,
    shell_returned: bool,
    seal_phase: u8,
    seal_page: u8,
    seal_visible: u8,
    seal_current: Option<DbIoPagePhase>,
}

#[derive(Debug)]
pub struct DbIoPageWriterRejected {
    error: DbError,
    writer: Option<DbIoPageWriter>,
}

/// @emoji 🧵 One retained unused-page retirement opportunity per poll before writer publication.
pub struct DbIoPageWriterSeal {
    writer: Option<DbIoPageWriter>,
}

impl DbIoPageWriter {
    pub fn try_reserve(reserved_pages: usize) -> Result<Self, DbIoPageWriterRejected> {
        let credit = db_io_page_credit(reserved_pages).checked_add(db_io_page_shell_credit()).ok_or_else(|| DbIoPageWriterRejected { error: DbError::LimitExceeded("DB I/O page-owner credit"), writer: None })?;
        let operation = db_io_operation_reserve(credit).map_err(|error| DbIoPageWriterRejected { error, writer: None })?;
        Self::checkout(operation, reserved_pages)
    }

    pub fn try_reserve_for_operation(operation: u64, reserved_pages: usize) -> Result<Self, DbIoPageWriterRejected> {
        let credit = db_io_page_credit(reserved_pages).checked_add(db_io_page_shell_credit()).ok_or_else(|| DbIoPageWriterRejected { error: DbError::LimitExceeded("DB I/O page-owner credit"), writer: None })?;
        db_io_operation_add(operation, credit).map_err(|error| DbIoPageWriterRejected { error, writer: None })?;
        Self::checkout(operation, reserved_pages)
    }

    fn checkout(operation: u64, reserved_pages: usize) -> Result<Self, DbIoPageWriterRejected> {
        let pages = match db_io_checkout_pages(operation, reserved_pages, DbIoPagePhase::CheckedOutWriter) {
            Ok(pages) => pages,
            Err(error) => {
                let credit = match db_io_page_credit(reserved_pages).checked_add(db_io_page_shell_credit()) {
                    Some(credit) => credit,
                    None => return Err(DbIoPageWriterRejected { error: DbError::LimitExceeded("DB I/O page-owner credit"), writer: None }),
                };
                let _ = db_io_operation_return(operation, credit);
                return Err(DbIoPageWriterRejected { error, writer: None });
            }
        };
        Ok(Self { operation, pages, reserved: reserved_pages as u8, cursor: 0, total_len: 0, shell_returned: false, seal_phase: 0, seal_page: 0, seal_visible: 0, seal_current: None })
    }

    pub fn operation(&self) -> u64 {
        self.operation
    }

    pub fn write_fragment(&mut self, source: &[u8]) -> Result<usize, DbError> {
        let Some(page) = self.pages.get_mut(self.cursor as usize).and_then(Option::as_mut) else {
            return Err(DbError::LimitExceeded("db_io writer page reservation"));
        };
        let available = DB_IO_PAGE_BYTES - usize::from(page.used);
        let written = available.min(source.len());
        page.write(usize::from(page.used), &source[..written])?;
        self.total_len = self.total_len.checked_add(written).ok_or(DbError::LimitExceeded("db_io writer total"))?;
        if usize::from(page.used) == DB_IO_PAGE_BYTES {
            self.cursor += 1;
        }
        Ok(written)
    }

    pub fn patch_fragment(&mut self, offset: usize, source: &[u8]) -> Result<usize, DbError> {
        if offset >= self.total_len || source.is_empty() {
            return Ok(0);
        }
        let page_index = offset / DB_IO_PAGE_BYTES;
        let page_offset = offset % DB_IO_PAGE_BYTES;
        let page = self.pages.get_mut(page_index).and_then(Option::as_mut).ok_or(DbError::LimitExceeded("db_io writer patch page"))?;
        let patchable = usize::from(page.used).checked_sub(page_offset).ok_or_else(|| DbError::Internal("db_io writer patch offset exceeded its admitted page".to_string()))?;
        let written = source.len().min(patchable);
        page.write(page_offset, &source[..written])?;
        Ok(written)
    }

    pub fn read_fragment(&self, offset: usize, output: &mut [u8]) -> Result<usize, DbError> {
        if offset >= self.total_len || output.is_empty() {
            return Ok(0);
        }
        let page_index = offset / DB_IO_PAGE_BYTES;
        let page_offset = offset % DB_IO_PAGE_BYTES;
        let page = self.pages.get(page_index).and_then(Option::as_ref).ok_or(DbError::LimitExceeded("db_io writer read page"))?;
        let source = page.bytes();
        let readable = source.len().checked_sub(page_offset).ok_or_else(|| DbError::Internal("db_io writer read offset exceeded its admitted page".to_string()))?;
        let read = output.len().min(readable);
        output[..read].copy_from_slice(&source[page_offset..page_offset + read]);
        Ok(read)
    }

    pub fn seal_retained(self) -> DbIoPageWriterSeal {
        DbIoPageWriterSeal { writer: Some(self) }
    }

    #[cfg(test)]
    pub fn seal(mut self) -> Result<DbIoPages, DbIoPageWriterRejected> {
        match self.finish() {
            Ok(pages) => Ok(pages),
            Err(error) => Err(DbIoPageWriterRejected { error, writer: Some(self) }),
        }
    }

    #[cfg(test)]
    pub fn finish(&mut self) -> Result<DbIoPages, DbError> {
        loop {
            if let Some(owner) = self.seal_retained_step()? {
                return Ok(owner);
            }
        }
    }

    /// @emoji 🪡 One retained validation, unused-page retirement, or atomic transition-and-publication opportunity.
    pub fn seal_retained_step(&mut self) -> Result<Option<DbIoPages>, DbError> {
        match self.seal_phase {
            0 if self.reserved > self.seal_visible => {
                self.seal_visible = if self.total_len == 0 { 0 } else { self.total_len.div_ceil(DB_IO_PAGE_BYTES) as u8 };
                if self.reserved <= self.seal_visible {
                    return Ok(None);
                }
                let index = usize::from(self.reserved - 1);
                let page = self.pages[index].take().ok_or_else(|| DbError::Internal("DB I/O retained writer lost an unused page".to_string()))?;
                let phase = page.phase()?;
                if !matches!(phase, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
                    return Err(DbError::Internal(format!("DB I/O unused writer page phase mismatch: expected CheckedOutWriter or Executing, got {phase:?} for operation {} generation {}", page.operation, page.generation)));
                }
                page.transition(phase, DbIoPagePhase::Closing)?;
                page.return_to_arena()?;
                self.reserved -= 1;
                Ok(None)
            }
            0 => {
                self.seal_visible = if self.total_len == 0 { 0 } else { self.total_len.div_ceil(DB_IO_PAGE_BYTES) as u8 };
                let current = match self.pages.iter().take(self.reserved as usize).flatten().next() {
                    Some(page) => page.phase()?,
                    None => DbIoPagePhase::CheckedOutWriter,
                };
                if !matches!(current, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
                    return Err(DbError::Internal("DB I/O retained writer finished outside an owned phase".to_string()));
                }
                self.seal_current = Some(current);
                self.seal_phase = 1;
                Ok(None)
            }
            1 if self.seal_page < self.reserved => {
                let page = self.pages[self.seal_page as usize].as_ref().ok_or_else(|| DbError::Internal("DB I/O retained writer validation lost a page".to_string()))?;
                let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let slot = state.slots[page.slot as usize];
                let current = self.seal_current.ok_or_else(|| DbError::Internal("DB I/O retained writer lost its source phase".to_string()))?;
                page.validate_phase(slot, current)?;
                self.seal_page += 1;
                Ok(None)
            }
            1 => {
                self.seal_phase = 2;
                self.seal_page = 0;
                Ok(None)
            }
            2 => {
                let current = self.seal_current.ok_or_else(|| DbError::Internal("DB I/O retained writer lost its transition phase".to_string()))?;
                let next = if current == DbIoPagePhase::Executing { DbIoPagePhase::TerminalResult } else { DbIoPagePhase::CheckedOutInput };
                self.transition(current, next)?;
                self.seal_phase = 3;
                let pages = std::mem::replace(&mut self.pages, std::array::from_fn(|_| None));
                let owner = DbIoPages { operation: self.operation, pages, retained: self.reserved, visible: self.seal_visible, first_offset: 0, total_len: self.total_len, shell_returned: false, result_handback: None };
                self.shell_returned = true;
                self.reserved = 0;
                self.cursor = 0;
                self.total_len = 0;
                self.seal_phase = 4;
                Ok(Some(owner))
            }
            _ => Err(DbError::Closed),
        }
    }

    pub fn len(&self) -> usize {
        self.total_len
    }

    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        let Some(page) = self.pages.iter_mut().rev().find_map(Option::take) else {
            if !self.shell_returned {
                db_io_operation_return(self.operation, db_io_page_shell_credit())?;
                self.shell_returned = true;
                return Ok(Some(0));
            }
            return Ok(None);
        };
        let phase = page.phase()?;
        page.transition(phase, DbIoPagePhase::Closing)?;
        page.return_to_arena().map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none) && self.shell_returned
    }

    fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.reserved as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            page.validate_phase(slot, expected)?;
        }
        for page in self.pages.iter().take(self.reserved as usize).flatten() {
            state.slots[page.slot as usize].phase = next;
        }
        Ok(())
    }
}

impl Future for DbIoPageWriterSeal {
    type Output = Result<DbIoPages, DbIoPageWriterRejected>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbIoPageWriterRejected { error: DbError::Internal("DB I/O retained writer lost its exact owner".to_string()), writer: None }));
        };
        match writer.seal_retained_step() {
            Ok(Some(published)) => {
                owner.writer.take();
                std::task::Poll::Ready(Ok(published))
            }
            Ok(None) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(error) => std::task::Poll::Ready(Err(DbIoPageWriterRejected { error, writer: owner.writer.take() })),
        }
    }
}

impl Drop for DbIoPageWriter {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self {
            operation: self.operation,
            pages: std::mem::replace(&mut self.pages, std::array::from_fn(|_| None)),
            reserved: self.reserved,
            cursor: self.cursor,
            total_len: self.total_len,
            shell_returned: self.shell_returned,
            seal_phase: self.seal_phase,
            seal_page: self.seal_page,
            seal_visible: self.seal_visible,
            seal_current: self.seal_current,
        };
        self.reserved = 0;
        self.cursor = 0;
        self.total_len = 0;
        self.shell_returned = true;
        if let Err(DbIoLostOwner::PageWriter(owner)) = db_io_park_lost_owner(DbIoLostOwner::PageWriter(owner)) {
            *self = owner;
        }
    }
}

impl std::fmt::Debug for DbIoPageWriter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbIoPageWriter").field("operation", &self.operation).field("reserved", &self.reserved).field("cursor", &self.cursor).field("total_len", &self.total_len).finish()
    }
}

impl DbIoPageWriterRejected {
    pub fn error(&self) -> &DbError {
        &self.error
    }

    pub fn into_writer(self) -> Option<DbIoPageWriter> {
        self.writer
    }

    pub fn into_error(self) -> DbError {
        self.error
    }

    pub fn into_parts(self) -> (DbError, Option<DbIoPageWriter>) {
        (self.error, self.writer)
    }
}

/// @emoji 📥 One-fragment-per-poll copy into already checked-out fixed pages.
pub struct DbIoPageCopy<'a> {
    source: &'a [u8],
    cursor: usize,
    writer: Option<DbIoPageWriter>,
}

pub struct DbIoPageOwnerCopy<'a> {
    source: &'a DbIoPages,
    cursor: u8,
    writer: Option<DbIoPageWriter>,
}

pub struct DbIoPageHash<'a> {
    source: &'a DbIoPages,
    cursor: u8,
    hasher: semio_framework_hash::Hasher,
}

pub fn db_io_copy_pages(source: &[u8]) -> Result<DbIoPageCopy<'_>, DbError> {
    let pages = source.len().div_ceil(DB_IO_PAGE_BYTES);
    let writer = DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(DbIoPageCopy { source, cursor: 0, writer: Some(writer) })
}

/// @emoji 🎟️ Pre-admitted capacity for one external-driver allocation owned by an aggregate operation.
pub struct DbIoDriverReservation {
    operation: u64,
    credit: DbIoCredit,
    returned: bool,
}

impl DbIoDriverReservation {
    pub fn try_reserve(operation: u64, maximum_capacity: usize) -> Result<Self, DbError> {
        let credit = DbIoCredit { pages: 0, bytes: maximum_capacity as u64, items: 1, controls: 1 };
        db_io_operation_add(operation, credit)?;
        Ok(Self { operation, credit, returned: false })
    }

    pub fn observe_capacity(&mut self, capacity: usize) -> Result<(), DbError> {
        if self.returned || capacity as u64 > self.credit.bytes {
            return Err(DbError::LimitExceeded("DB I/O external driver allocation capacity"));
        }
        let slack = self.credit.bytes - capacity as u64;
        if slack != 0 {
            db_io_operation_return(self.operation, DbIoCredit { pages: 0, bytes: slack, items: 0, controls: 0 })?;
            self.credit.bytes = capacity as u64;
        }
        Ok(())
    }

    pub fn close_step(&mut self) -> Result<(), DbError> {
        if !self.returned {
            db_io_operation_return(self.operation, self.credit)?;
            self.returned = true;
        }
        Ok(())
    }
}

impl Drop for DbIoDriverReservation {
    fn drop(&mut self) {
        if !self.returned {
            let owner = Self { operation: self.operation, credit: self.credit, returned: false };
            self.returned = true;
            if let Err(DbIoLostOwner::DriverReservation(owner)) = db_io_park_lost_owner(DbIoLostOwner::DriverReservation(owner)) {
                *self = owner;
            }
        }
    }
}

/// @emoji 🏷️ Exact fixed post-admission artifact identity retained without a heap string.
pub struct DbIoArtifactId {
    value: DbIoText,
    driver: Option<ArtifactId>,
    external: Option<DbIoExternalBytes>,
    reservation: Option<DbIoDriverReservation>,
    phase: u8,
}

impl DbIoArtifactId {
    pub fn try_from_text(operation: u64, source: &DbIoText) -> Result<Self, DbError> {
        if operation == 0 {
            return Err(DbError::InvalidArgument("artifact identity requires an admitted operation".to_string()));
        }
        let reservation = DbIoDriverReservation::try_reserve(operation, DbIoText::maximum_capacity())?;
        let mut owner = Self { value: DbIoText::try_from_str(source.as_str())?, driver: Some(ArtifactId(source.as_str().to_owned())), external: None, reservation: Some(reservation), phase: 0 };
        let capacity = owner.driver.as_ref().ok_or_else(|| DbError::Internal("artifact conversion lost its admitted driver identity".to_string()))?.0.capacity();
        owner.reservation.as_mut().ok_or_else(|| DbError::Internal("artifact conversion lost its exact reservation".to_string()))?.observe_capacity(capacity)?;
        Ok(owner)
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn as_artifact(&self) -> Result<&ArtifactId, DbError> {
        self.driver.as_ref().ok_or(DbError::Closed)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        match self.phase {
            0 => {
                let driver = self.driver.take().ok_or_else(|| DbError::Internal("artifact conversion lost its exact driver identity".to_string()))?;
                self.external = Some(DbIoExternalBytes::new(driver.0.into_bytes()));
                self.phase = 1;
                Ok(true)
            }
            1 => {
                let external = self.external.as_mut().ok_or_else(|| DbError::Internal("artifact conversion lost its external allocation".to_string()))?;
                if external.close_step() {
                    return Ok(true);
                }
                self.external.take();
                self.phase = 2;
                Ok(true)
            }
            2 => {
                let _ = self.value.close_step();
                self.phase = 3;
                Ok(true)
            }
            3 => {
                self.reservation.as_mut().ok_or_else(|| DbError::Internal("artifact conversion lost its reservation during close".to_string()))?.close_step()?;
                self.reservation.take();
                self.phase = 4;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == 4 && self.value.terminal_is_empty() && self.driver.is_none() && self.external.is_none() && self.reservation.is_none()
    }
}

impl Drop for DbIoArtifactId {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self { value: std::mem::replace(&mut self.value, DbIoText::new()), driver: self.driver.take(), external: self.external.take(), reservation: self.reservation.take(), phase: self.phase };
        self.phase = 4;
        if let Err(DbIoLostOwner::ArtifactId(owner)) = db_io_park_lost_owner(DbIoLostOwner::ArtifactId(owner)) {
            *self = owner;
        }
    }
}

/// @emoji 🧳 Retained external allocation with one page-content or backing-release close opportunity.
pub struct DbIoExternalBytes {
    value: Option<Vec<u8>>,
    phase: u8,
}

impl DbIoExternalBytes {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value: Some(value), phase: 0 }
    }

    pub fn capacity(&self) -> Result<usize, DbError> {
        self.value.as_ref().map(Vec::capacity).ok_or_else(|| DbError::Closed)
    }

    pub fn as_slice(&self) -> Result<&[u8], DbError> {
        self.value.as_deref().ok_or(DbError::Closed)
    }

    pub fn into_value(mut self) -> Result<Vec<u8>, DbError> {
        let value = self.value.take().ok_or(DbError::Closed)?;
        self.phase = 2;
        Ok(value)
    }

    pub fn close_step(&mut self) -> bool {
        let Some(value) = self.value.as_mut() else { return false };
        if !value.is_empty() {
            value.truncate(value.len().saturating_sub(DB_IO_PAGE_BYTES));
            return true;
        }
        if self.phase == 0 {
            self.phase = 1;
            return true;
        }
        self.value.take();
        self.phase = 2;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.phase == 2
    }
}

impl Drop for DbIoExternalBytes {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self { value: self.value.take(), phase: self.phase };
        self.phase = 2;
        if let Err(DbIoLostOwner::ExternalBytes(owner)) = db_io_park_lost_owner(DbIoLostOwner::ExternalBytes(owner)) {
            *self = owner;
        }
    }
}

/// @emoji 🪜 Retained external-driver byte conversion with one observed owner/page/close opportunity per poll.
pub struct DbIoObservedBytesWrite<'a> {
    reservation: Option<DbIoDriverReservation>,
    source: Option<DbIoExternalBytes>,
    output: &'a mut DbIoPageWriter,
    cursor: usize,
    limit: usize,
    phase: u8,
}

pub fn db_io_write_observed_bytes(reservation: DbIoDriverReservation, source: Vec<u8>, output: &mut DbIoPageWriter) -> DbIoObservedBytesWrite<'_> {
    let limit = source.len();
    DbIoObservedBytesWrite { reservation: Some(reservation), source: Some(DbIoExternalBytes::new(source)), output, cursor: 0, limit, phase: 0 }
}

/// @emoji 🧷 Retained ranged external-driver transfer with the same exact close authority.
pub fn db_io_write_observed_bytes_range(reservation: DbIoDriverReservation, source: Vec<u8>, offset: usize, length: usize, output: &mut DbIoPageWriter) -> Result<DbIoObservedBytesWrite<'_>, DbError> {
    let limit = offset.checked_add(length).ok_or(DbError::LimitExceeded("DB I/O observed-byte range"))?;
    if limit > source.len() {
        return Err(DbError::InvalidArgument("DB I/O observed-byte range exceeds result".to_string()));
    }
    Ok(DbIoObservedBytesWrite { reservation: Some(reservation), source: Some(DbIoExternalBytes::new(source)), output, cursor: offset, limit, phase: 0 })
}

impl Future for DbIoObservedBytesWrite<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        match owner.phase {
            0 => {
                let capacity = owner.source.as_ref().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its exact source".to_string()))?.capacity()?;
                owner.reservation.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its reservation".to_string()))?.observe_capacity(capacity)?;
                owner.phase = 1;
            }
            1 => {
                let source = owner.source.as_ref().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its source during copy".to_string()))?.as_slice()?;
                if owner.cursor < owner.limit {
                    let end = owner.cursor.checked_add(DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("DB I/O observed-byte cursor"))?.min(owner.limit);
                    let written = owner.output.write_fragment(&source[owner.cursor..end])?;
                    owner.cursor += written;
                } else {
                    owner.phase = 2;
                }
            }
            2 => {
                let source = owner.source.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its source during close".to_string()))?;
                if !source.close_step() {
                    owner.source.take();
                    owner.phase = 3;
                }
            }
            3 => {
                owner.reservation.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its reservation during close".to_string()))?.close_step()?;
                owner.reservation.take();
                owner.phase = 4;
            }
            _ => match owner.output.seal_retained_step()? {
                Some(published) => return std::task::Poll::Ready(Ok(published)),
                None => {}
            },
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

pub fn db_io_copy_page_owner(source: &DbIoPages) -> Result<DbIoPageOwnerCopy<'_>, DbError> {
    let writer = DbIoPageWriter::try_reserve(source.page_count() as usize).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(DbIoPageOwnerCopy { source, cursor: 0, writer: Some(writer) })
}

pub fn db_io_hash_pages(source: &DbIoPages) -> DbIoPageHash<'_> {
    DbIoPageHash { source, cursor: 0, hasher: semio_framework_hash::Hasher::new() }
}

/// @emoji 🔢 Persisted one-scalar transfer and one-owner close authority for driver lists.
pub struct DbIoListTransfer<'a> {
    source: Option<DbIoU64List>,
    output: &'a mut DbIoU64List,
    cursor: usize,
    phase: u8,
}

pub fn db_io_transfer_list(source: DbIoU64List, output: &mut DbIoU64List) -> DbIoListTransfer<'_> {
    DbIoListTransfer { source: Some(source), output, cursor: 0, phase: 0 }
}

impl Future for DbIoListTransfer<'_> {
    type Output = Result<DbIoU64List, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let source = owner.source.as_mut().ok_or_else(|| DbError::Internal("DB I/O list transfer lost its exact source".to_string()))?;
        match owner.phase {
            0 if owner.cursor < source.len() => {
                owner.output.push(source.as_slice()[owner.cursor])?;
                owner.cursor += 1;
            }
            0 => owner.phase = 1,
            1 if source.close_step() => {}
            1 => {
                owner.source.take();
                owner.phase = 2;
            }
            _ => return std::task::Poll::Ready(Ok(std::mem::take(owner.output))),
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.cursor == owner.source.len() {
            let Some(writer) = owner.writer.as_mut() else {
                return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-copy cursor lost its retained writer".to_string())));
            };
            return match writer.seal_retained_step()? {
                Some(pages) => {
                    owner.writer.take();
                    std::task::Poll::Ready(Ok(pages))
                }
                None => {
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            };
        }
        let start = owner.cursor;
        let end = start.checked_add(DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("DB I/O page-copy cursor"))?.min(owner.source.len());
        let source = &owner.source[start..end];
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-copy cursor lost its retained writer".to_string())));
        };
        let written = writer.write_fragment(source)?;
        owner.cursor += written;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageOwnerCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(source) = owner.source.page(owner.cursor) else {
            let Some(writer) = owner.writer.as_mut() else {
                return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-owner cursor lost its retained writer".to_string())));
            };
            return match writer.seal_retained_step()? {
                Some(pages) => {
                    owner.writer.take();
                    std::task::Poll::Ready(Ok(pages))
                }
                None => {
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            };
        };
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-owner cursor lost its retained writer".to_string())));
        };
        let written = writer.write_fragment(source)?;
        if written != source.len() {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O source fragment exceeded one fixed page".to_string())));
        }
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageHash<'_> {
    type Output = ContentHash;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(fragment) = owner.source.page(owner.cursor) else {
            let hasher = std::mem::replace(&mut owner.hasher, semio_framework_hash::Hasher::new());
            return std::task::Poll::Ready(ContentHash(*hasher.finalize().as_bytes()));
        };
        owner.hasher.update(fragment);
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

const DB_IO_PLATFORM_BUFFERS: usize = 16;
const DB_IO_PLATFORM_RETIREMENT_SLOTS: usize = DB_IO_PLATFORM_BUFFERS * 2;
const DB_IO_PLATFORM_BUFFER_BYTES: usize = MAX_READ_BYTES as usize;

struct DbIoPlatformBacking(std::cell::UnsafeCell<[u8; DB_IO_PLATFORM_BUFFER_BYTES]>);

impl DbIoPlatformBacking {
    const fn new() -> Self {
        Self(std::cell::UnsafeCell::new([0; DB_IO_PLATFORM_BUFFER_BYTES]))
    }
}

unsafe impl Sync for DbIoPlatformBacking {}

static DB_IO_PLATFORM_BACKINGS: [DbIoPlatformBacking; DB_IO_PLATFORM_BUFFERS] = [const { DbIoPlatformBacking::new() }; DB_IO_PLATFORM_BUFFERS];

#[derive(Clone, Copy)]
struct DbIoPlatformSlot {
    generation: u64,
    occupied: bool,
}

struct DbIoPlatformArena {
    slots: [DbIoPlatformSlot; DB_IO_PLATFORM_BUFFERS],
    retired: [Option<(u8, u64)>; DB_IO_PLATFORM_RETIREMENT_SLOTS],
    retired_read: usize,
    retired_len: usize,
    next_generation: u64,
}

static DB_IO_PLATFORM_ARENA: std::sync::Mutex<DbIoPlatformArena> =
    std::sync::Mutex::new(DbIoPlatformArena { slots: [DbIoPlatformSlot { generation: 0, occupied: false }; DB_IO_PLATFORM_BUFFERS], retired: [None; DB_IO_PLATFORM_RETIREMENT_SLOTS], retired_read: 0, retired_len: 0, next_generation: 1 });

/// @emoji 🧩 Explicit contiguous platform-call buffer backed by a fixed process slot.
pub struct DbIoPlatformBuffer {
    slot: u8,
    generation: u64,
    len: usize,
    copied: usize,
    credit: DbIoPageWriter,
    returned: bool,
}

pub struct DbIoPlatformCopy<'a> {
    source: &'a DbIoPages,
    cursor: u8,
    owner: Option<DbIoPlatformBuffer>,
}

/// @emoji 🚪 One retained prepared-platform close opportunity per poll.
pub struct DbIoPlatformClose {
    owner: Option<DbIoPlatformBuffer>,
}

/// @emoji 🪡 One retained prepared-platform slice fragment per poll.
pub(crate) struct DbIoPlatformSlicesCopy<'a> {
    first: &'a [u8],
    second: &'a [u8],
    source: u8,
    cursor: usize,
    owner: Option<DbIoPlatformBuffer>,
    fault: Option<DbError>,
}

pub fn db_io_prepare_platform(source: &DbIoPages) -> Result<DbIoPlatformCopy<'_>, DbError> {
    let _ = db_io_platform_maintenance_step()?;
    if source.len() > DB_IO_PLATFORM_BUFFER_BYTES {
        return Err(DbError::LimitExceeded("db_io prepared platform buffer"));
    }
    let (slot, generation) = {
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = arena.slots.iter().position(|slot| !slot.occupied).ok_or_else(|| DbError::Unavailable("DB I/O prepared platform capacity exhausted".to_string()))?;
        let generation = arena.next_generation;
        arena.next_generation = arena.next_generation.checked_add(1).filter(|next| *next != 0).ok_or(DbError::LimitExceeded("db_io platform generation"))?;
        arena.slots[slot] = DbIoPlatformSlot { generation, occupied: true };
        (slot as u8, generation)
    };
    let credit = match DbIoPageWriter::try_reserve_for_operation(source.operation(), source.page_count() as usize) {
        Ok(credit) => credit,
        Err(error) => {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
            return Err(error.into_error());
        }
    };
    let owner = DbIoPlatformBuffer { slot, generation, len: source.len(), copied: 0, credit, returned: false };
    Ok(DbIoPlatformCopy { source, cursor: 0, owner: Some(owner) })
}

impl DbIoPlatformBuffer {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &(&*DB_IO_PLATFORM_BACKINGS[self.slot as usize].0.get())[..self.len] }
    }

    pub(crate) fn as_static_driver_slice(&self) -> &'static [u8] {
        unsafe { &(&*DB_IO_PLATFORM_BACKINGS[self.slot as usize].0.get())[..self.len] }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.credit.close_step()?.is_some() {
            return Ok(true);
        }
        if self.returned {
            return Ok(false);
        }
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut arena.slots[self.slot as usize];
        if !slot.occupied || slot.generation != self.generation {
            return Err(DbError::Internal("DB I/O prepared platform slot lost ABA authority".to_string()));
        }
        *slot = DbIoPlatformSlot { generation: 0, occupied: false };
        self.returned = true;
        Ok(true)
    }
}

pub fn db_io_close_platform(owner: DbIoPlatformBuffer) -> DbIoPlatformClose {
    DbIoPlatformClose { owner: Some(owner) }
}

impl Future for DbIoPlatformClose {
    type Output = Result<(), DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(platform) = owner.owner.as_mut() else { return std::task::Poll::Ready(Ok(())) };
        if platform.close_step()? {
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        owner.owner.take();
        std::task::Poll::Ready(Ok(()))
    }
}

fn db_io_reserve_platform_buffer(operation: u64, total: usize) -> Result<DbIoPlatformBuffer, DbError> {
    if total > DB_IO_PLATFORM_BUFFER_BYTES {
        return Err(DbError::LimitExceeded("db_io prepared platform slices"));
    }
    let (slot, generation) = {
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = arena.slots.iter().position(|slot| !slot.occupied).ok_or_else(|| DbError::Unavailable("DB I/O prepared platform capacity exhausted".to_string()))?;
        let generation = arena.next_generation;
        arena.next_generation = arena.next_generation.checked_add(1).filter(|next| *next != 0).ok_or(DbError::LimitExceeded("db_io platform generation"))?;
        arena.slots[slot] = DbIoPlatformSlot { generation, occupied: true };
        (slot as u8, generation)
    };
    let credit = match DbIoPageWriter::try_reserve_for_operation(operation, total.div_ceil(DB_IO_PAGE_BYTES)) {
        Ok(credit) => credit,
        Err(error) => {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
            return Err(error.into_error());
        }
    };
    Ok(DbIoPlatformBuffer { slot, generation, len: total, copied: 0, credit, returned: false })
}

pub(crate) fn db_io_prepare_platform_slices<'a>(operation: u64, first: &'a [u8], second: &'a [u8]) -> DbIoPlatformSlicesCopy<'a> {
    let reserved = first.len().checked_add(second.len()).ok_or(DbError::LimitExceeded("db_io prepared platform slices")).and_then(|total| db_io_reserve_platform_buffer(operation, total));
    match reserved {
        Ok(owner) => DbIoPlatformSlicesCopy { first, second, source: 0, cursor: 0, owner: Some(owner), fault: None },
        Err(error) => DbIoPlatformSlicesCopy { first, second, source: 0, cursor: 0, owner: None, fault: Some(error) },
    }
}

impl Future for DbIoPlatformSlicesCopy<'_> {
    type Output = Result<DbIoPlatformBuffer, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if let Some(error) = owner.fault.take() {
            return std::task::Poll::Ready(Err(error));
        }
        if owner.source > 1 {
            return std::task::Poll::Ready(owner.owner.take().ok_or_else(|| DbError::Internal("DB I/O prepared-platform slice cursor lost its exact owner".to_string())));
        }
        let source = if owner.source == 0 { owner.first } else { owner.second };
        if owner.cursor == source.len() {
            owner.source += 1;
            owner.cursor = 0;
        } else {
            let written = (source.len() - owner.cursor).min(DB_IO_PAGE_BYTES);
            let platform = owner.owner.as_mut().ok_or_else(|| DbError::Internal("DB I/O prepared-platform slice cursor lost its owner during copy".to_string()))?;
            let end = platform.copied.checked_add(written).ok_or(DbError::LimitExceeded("db_io prepared platform slices cursor"))?;
            unsafe { (&mut *DB_IO_PLATFORM_BACKINGS[platform.slot as usize].0.get())[platform.copied..end].copy_from_slice(&source[owner.cursor..owner.cursor + written]) };
            platform.copied = end;
            owner.cursor += written;
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Drop for DbIoPlatformBuffer {
    fn drop(&mut self) {
        if !self.returned {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let slot = arena.slots[self.slot as usize];
            if slot.occupied && slot.generation == self.generation {
                if arena.retired_len >= DB_IO_PLATFORM_BUFFERS {
                    DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                let write = (arena.retired_read + arena.retired_len) % DB_IO_PLATFORM_RETIREMENT_SLOTS;
                arena.retired[write] = Some((self.slot, self.generation));
                arena.retired_len += 1;
                self.returned = true;
            }
        }
    }
}

pub fn db_io_platform_maintenance_step() -> Result<bool, DbError> {
    let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if arena.retired_len == 0 {
        return Ok(false);
    }
    let read = arena.retired_read;
    let Some((slot, generation)) = arena.retired[read].take() else {
        DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        return Err(DbError::Internal("DB I/O platform retired length lost exact owner".to_string()));
    };
    let owner = arena.slots[slot as usize];
    if !owner.occupied || owner.generation != generation {
        return Err(DbError::Internal("DB I/O platform retirement lost ABA authority".to_string()));
    }
    arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
    arena.retired_read = (read + 1) % DB_IO_PLATFORM_RETIREMENT_SLOTS;
    arena.retired_len -= 1;
    Ok(true)
}

impl Future for DbIoPlatformCopy<'_> {
    type Output = Result<DbIoPlatformBuffer, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(fragment) = owner.source.page(owner.cursor) else {
            return std::task::Poll::Ready(owner.owner.take().ok_or_else(|| DbError::Internal("DB I/O platform cursor lost its retained buffer".to_string())));
        };
        let Some(platform) = owner.owner.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O platform cursor lost its retained buffer".to_string())));
        };
        let end = platform.copied.checked_add(fragment.len()).ok_or(DbError::LimitExceeded("db_io prepared platform cursor"))?;
        unsafe { (&mut *DB_IO_PLATFORM_BACKINGS[platform.slot as usize].0.get())[platform.copied..end].copy_from_slice(fragment) };
        platform.copied = end;
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

/// @emoji 📄 Exact ordered fixed-page leases with a zero-copy movable range cursor.
#[derive(Debug)]
pub struct DbIoPages {
    operation: u64,
    pages: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES],
    retained: u8,
    visible: u8,
    first_offset: usize,
    total_len: usize,
    shell_returned: bool,
    result_handback: Option<DbIoTaskHandle>,
}

impl DbIoPages {
    pub fn operation(&self) -> u64 {
        self.operation
    }

    pub fn take_for_async_driver(&mut self) -> Self {
        let operation = self.operation;
        std::mem::replace(self, Self { operation, pages: std::array::from_fn(|_| None), retained: 0, visible: 0, first_offset: 0, total_len: 0, shell_returned: true, result_handback: None })
    }

    fn transfer_to_backend(&mut self, backend_operation: u64) -> Result<(), DbError> {
        if self.result_handback.is_some() {
            return Err(DbError::Internal("DB I/O result lease cannot transfer into backend backing".to_string()));
        }
        let page_count = self.pages.iter().flatten().count();
        let mut credit = db_io_page_credit(page_count);
        if !self.shell_returned {
            credit = credit.checked_add(db_io_page_shell_credit()).ok_or(DbError::LimitExceeded("DB I/O backend page-shell transfer"))?;
        }
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().flatten() {
            let owner = arena.slots[page.slot as usize];
            if owner.generation != page.generation || owner.operation != self.operation || owner.phase == DbIoPagePhase::Free {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(page.generation), actual: crate::db_ids::GenerationId(owner.generation) });
            }
        }
        db_io_operation_transfer_to_backend(self.operation, backend_operation, credit)?;
        for page in self.pages.iter_mut().flatten() {
            arena.slots[page.slot as usize].operation = backend_operation;
            page.operation = backend_operation;
        }
        self.operation = backend_operation;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.total_len
    }

    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    pub fn page_count(&self) -> u8 {
        self.visible
    }

    pub fn page(&self, index: u8) -> Option<&[u8]> {
        if index >= self.visible {
            return None;
        }
        let physical = usize::from(index) + self.first_offset / DB_IO_PAGE_BYTES;
        let page = self.pages.get(physical)?.as_ref()?.bytes();
        let start = if index == 0 { self.first_offset % DB_IO_PAGE_BYTES } else { 0 };
        let preceding = usize::from(index).checked_mul(DB_IO_PAGE_BYTES)?.checked_sub(if index == 0 { 0 } else { self.first_offset % DB_IO_PAGE_BYTES })?;
        let remaining = self.total_len.checked_sub(preceding)?;
        let available = page.len().checked_sub(start)?;
        let end = start.checked_add(remaining.min(available))?;
        page.get(start..end)
    }

    pub fn fragments(&self) -> DbIoPageReader<'_> {
        DbIoPageReader { pages: self, cursor: 0 }
    }

    pub fn try_range(mut self, start: usize) -> Result<Self, Self> {
        if start > self.total_len {
            return Err(self);
        }
        self.first_offset += start;
        self.total_len -= start;
        self.visible = if self.total_len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + self.total_len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(self)
    }

    pub fn try_prefix(mut self, len: usize) -> Result<Self, Self> {
        if len > self.total_len {
            return Err(self);
        }
        self.total_len = len;
        self.visible = if len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(self)
    }

    pub fn advance(&mut self, len: usize) -> Result<(), DbError> {
        if len > self.total_len {
            return Err(DbError::InvalidArgument("DB I/O page cursor advanced past its retained range".to_string()));
        }
        self.first_offset += len;
        self.total_len -= len;
        self.visible = if self.total_len == 0 { 0 } else { (self.first_offset % DB_IO_PAGE_BYTES + self.total_len).div_ceil(DB_IO_PAGE_BYTES) as u8 };
        Ok(())
    }

    fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            page.validate_phase(slot, expected)?;
        }
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            state.slots[page.slot as usize].phase = next;
        }
        Ok(())
    }

    fn admit(&self) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            let slot = state.slots[page.slot as usize];
            page.validate_identity(slot)?;
            if !matches!(slot.phase, DbIoPagePhase::CheckedOutInput | DbIoPagePhase::TerminalResult) {
                return Err(DbError::Internal(format!("DB I/O input page phase mismatch: expected CheckedOutInput or TerminalResult, got {:?} for operation {} generation {}", slot.phase, page.operation, page.generation)));
            }
        }
        for page in self.pages.iter().take(self.retained as usize).flatten() {
            state.slots[page.slot as usize].phase = DbIoPagePhase::Queued;
        }
        Ok(())
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        let Some(page) = self.pages.iter_mut().rev().find_map(Option::take) else {
            if !self.shell_returned {
                db_io_operation_return(self.operation, db_io_page_shell_credit())?;
                self.shell_returned = true;
                return Ok(Some(0));
            }
            if let Some(handle) = self.result_handback {
                db_io_result_handback(handle)?;
                self.result_handback = None;
                return Ok(Some(0));
            }
            return Ok(None);
        };
        let phase = page.phase()?;
        page.transition(phase, DbIoPagePhase::Closing)?;
        page.return_to_arena().map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none) && self.shell_returned && self.result_handback.is_none()
    }
}

impl Drop for DbIoPages {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self {
            operation: self.operation,
            pages: std::mem::replace(&mut self.pages, std::array::from_fn(|_| None)),
            retained: self.retained,
            visible: self.visible,
            first_offset: self.first_offset,
            total_len: self.total_len,
            shell_returned: self.shell_returned,
            result_handback: self.result_handback.take(),
        };
        self.retained = 0;
        self.visible = 0;
        self.first_offset = 0;
        self.total_len = 0;
        self.shell_returned = true;
        if let Err(DbIoLostOwner::Pages(owner)) = db_io_park_lost_owner(DbIoLostOwner::Pages(owner)) {
            *self = owner;
        }
    }
}

impl PartialEq<[u8]> for DbIoPages {
    fn eq(&self, expected: &[u8]) -> bool {
        if self.len() != expected.len() {
            return false;
        }
        let mut offset = 0;
        for fragment in self.fragments() {
            let end = offset + fragment.len();
            if fragment != &expected[offset..end] {
                return false;
            }
            offset = end;
        }
        true
    }
}

impl PartialEq for DbIoPages {
    fn eq(&self, expected: &Self) -> bool {
        if self.len() != expected.len() {
            return false;
        }
        let mut left = self.fragments();
        let mut right = expected.fragments();
        let (mut left_fragment, mut right_fragment) = (left.next().unwrap_or_default(), right.next().unwrap_or_default());
        let (mut left_offset, mut right_offset) = (0, 0);
        while left_offset < left_fragment.len() || right_offset < right_fragment.len() || self.len() == 0 {
            if self.len() == 0 {
                return true;
            }
            let compared = (left_fragment.len() - left_offset).min(right_fragment.len() - right_offset);
            if left_fragment[left_offset..left_offset + compared] != right_fragment[right_offset..right_offset + compared] {
                return false;
            }
            left_offset += compared;
            right_offset += compared;
            if left_offset == left_fragment.len() {
                let Some(next) = left.next() else { return right_offset == right_fragment.len() && right.next().is_none() };
                left_fragment = next;
                left_offset = 0;
            }
            if right_offset == right_fragment.len() {
                let Some(next) = right.next() else { return left_offset == left_fragment.len() && left.next().is_none() };
                right_fragment = next;
                right_offset = 0;
            }
        }
        true
    }
}

impl Eq for DbIoPages {}

impl PartialEq<&[u8]> for DbIoPages {
    fn eq(&self, expected: &&[u8]) -> bool {
        self == *expected
    }
}

impl<const N: usize> PartialEq<[u8; N]> for DbIoPages {
    fn eq(&self, expected: &[u8; N]) -> bool {
        self == expected.as_slice()
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for DbIoPages {
    fn eq(&self, expected: &&[u8; N]) -> bool {
        self == expected.as_slice()
    }
}

impl PartialEq<Vec<u8>> for DbIoPages {
    fn eq(&self, expected: &Vec<u8>) -> bool {
        self == expected.as_slice()
    }
}

pub struct DbIoPageReader<'a> {
    pages: &'a DbIoPages,
    cursor: u8,
}

impl<'a> Iterator for DbIoPageReader<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let fragment = self.pages.page(self.cursor)?;
        self.cursor += 1;
        Some(fragment)
    }
}

pub fn db_io_page_maintenance_step() -> Result<Option<usize>, DbError> {
    let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let retired_read = state.retired_read;
    let Some((slot, generation)) = state.retired[retired_read].take() else { return Ok(None) };
    state.retired_read = (state.retired_read + 1) % DB_IO_PAGE_RETIREMENT_SLOTS;
    state.retired_len -= 1;
    let owner = state.slots[slot as usize];
    if owner.generation != generation || owner.phase != DbIoPagePhase::Closing {
        return Err(DbError::Internal("db I/O page retirement lost ABA authority".to_string()));
    }
    state.slots[slot as usize] = EMPTY_DB_IO_PAGE_SLOT;
    let write = (state.free_read + state.free_len) % DB_IO_TOTAL_PAGES;
    state.free[write] = slot;
    state.free_len += 1;
    drop(state);
    db_io_operation_return(owner.operation, db_io_page_credit(1))?;
    Ok(Some(DB_IO_PAGE_BYTES))
}

const DB_IO_TEXT_BYTES: usize = 1024;
const DB_IO_LIST_ITEMS: usize = 4096;

/// @emoji 🔤 Fixed repository-owned path, document, key or fault text.
#[derive(Clone, PartialEq, Eq)]
pub struct DbIoText {
    bytes: [u8; DB_IO_TEXT_BYTES],
    len: u16,
}

impl DbIoText {
    fn new() -> Self {
        Self { bytes: [0; DB_IO_TEXT_BYTES], len: 0 }
    }

    pub fn try_from_str(value: &str) -> Result<Self, DbError> {
        if value.len() > DB_IO_TEXT_BYTES {
            return Err(DbError::LimitExceeded("db_io text authority"));
        }
        let mut bytes = [0; DB_IO_TEXT_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..usize::from(self.len)]) }
    }

    pub const fn maximum_capacity() -> usize {
        DB_IO_TEXT_BYTES
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn close_step(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.len = 0;
        true
    }
}

/// @emoji 🔤 Converts one pre-admitted external-driver string into fixed repository text.
pub async fn db_io_copy_observed_text(mut reservation: DbIoDriverReservation, source: String) -> Result<DbIoText, DbError> {
    let mut source = DbIoExternalBytes::new(source.into_bytes());
    reservation.observe_capacity(source.capacity()?)?;
    let value = std::str::from_utf8(source.as_slice()?).map_err(|_| DbError::Corrupt("DB I/O observed text is not UTF-8".to_string()))?;
    let text = DbIoText::try_from_str(value)?;
    while !source.terminal_is_empty() {
        let _ = source.close_step();
        semio_framework_async::yield_once().await;
    }
    reservation.close_step()?;
    Ok(text)
}

impl std::fmt::Write for DbIoText {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let start = usize::from(self.len);
        let end = start.checked_add(value.len()).ok_or(std::fmt::Error)?;
        let target = self.bytes.get_mut(start..end).ok_or(std::fmt::Error)?;
        target.copy_from_slice(value.as_bytes());
        self.len = end as u16;
        Ok(())
    }
}

impl std::fmt::Debug for DbIoText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple("DbIoText").field(&self.as_str()).finish()
    }
}

/// @emoji 🔢 Bounded heap-backed typed list result with incremental close ownership.
pub struct DbIoU64List {
    values: Option<Box<[u64]>>,
    len: u16,
    result_handback: Option<DbIoTaskHandle>,
}

impl DbIoU64List {
    pub fn new() -> Self {
        Self { values: None, len: 0, result_handback: None }
    }

    fn take_for_result(&mut self) -> Self {
        std::mem::replace(self, Self::new())
    }

    fn ensure_backing(&mut self) -> Result<(), DbError> {
        if self.values.is_some() {
            return Ok(());
        }
        let mut values = Vec::new();
        values.try_reserve_exact(DB_IO_LIST_ITEMS).map_err(|_| DbError::Unavailable("DB I/O list backing allocation failed".to_string()))?;
        values.resize(DB_IO_LIST_ITEMS, 0);
        self.values = Some(values.into_boxed_slice());
        Ok(())
    }

    pub fn push(&mut self, value: u64) -> Result<(), DbError> {
        let index = usize::from(self.len);
        self.ensure_backing()?;
        let Some(slot) = self.values.as_deref_mut().and_then(|values| values.get_mut(index)) else {
            return Err(DbError::LimitExceeded("db_io list authority"));
        };
        *slot = value;
        self.len += 1;
        Ok(())
    }

    pub fn as_slice(&self) -> &[u64] {
        self.values.as_deref().map_or(&[], |values| &values[..usize::from(self.len)])
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn close_step(&mut self) -> bool {
        if self.len == 0 {
            if self.values.take().is_some() {
                return true;
            }
            if let Some(handle) = self.result_handback {
                if db_io_result_handback(handle).is_ok() {
                    self.result_handback = None;
                    return true;
                }
            }
            return false;
        }
        self.len -= 1;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.values.is_none() && self.result_handback.is_none()
    }
}

impl Default for DbIoU64List {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for DbIoU64List {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl std::ops::DerefMut for DbIoU64List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = usize::from(self.len);
        self.values.as_deref_mut().map_or(&mut [], |values| &mut values[..len])
    }
}

impl std::fmt::Debug for DbIoU64List {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.as_slice()).finish()
    }
}

impl PartialEq<Vec<u64>> for DbIoU64List {
    fn eq(&self, expected: &Vec<u64>) -> bool {
        self.as_slice() == expected.as_slice()
    }
}

impl PartialEq for DbIoU64List {
    fn eq(&self, expected: &Self) -> bool {
        self.as_slice() == expected.as_slice()
    }
}

impl<'a> IntoIterator for &'a DbIoU64List {
    type Item = &'a u64;
    type IntoIter = std::slice::Iter<'a, u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<const N: usize> PartialEq<[u64; N]> for DbIoU64List {
    fn eq(&self, expected: &[u64; N]) -> bool {
        self.as_slice() == expected.as_slice()
    }
}

impl IntoIterator for DbIoU64List {
    type Item = u64;
    type IntoIter = std::iter::Take<std::vec::IntoIter<u64>>;

    fn into_iter(mut self) -> Self::IntoIter {
        let len = usize::from(self.len);
        self.values.take().map_or_else(Vec::new, Vec::from).into_iter().take(len)
    }
}

impl Drop for DbIoU64List {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self { values: self.values.take(), len: self.len, result_handback: self.result_handback.take() };
        self.len = 0;
        if let Err(DbIoLostOwner::List(owner)) = db_io_park_lost_owner(DbIoLostOwner::List(owner)) {
            *self = owner;
        }
    }
}

/// @emoji 🧭 Repository-owned backend identity; external driver values never cross this boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoBackendControl {
    Memory { slot: u16, generation: u64 },
    Filesystem { slot: u16, generation: u64 },
    Sqlite { slot: u16, generation: u64 },
    Postgres { slot: u16, generation: u64 },
    Neo4j { slot: u16, generation: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoBackendKind {
    Memory,
    Filesystem,
    Sqlite,
    Postgres,
    Neo4j,
}

/// @emoji 🗂️ Schema-first database I/O task owner.
pub enum DbIoTask {
    BackendOpen { backend: DbIoBackendControl, path: DbIoText },
    WalCreate { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalAppend { backend: DbIoBackendControl, document: DbIoText, index: u64, input: DbIoPages },
    WalSync { backend: DbIoBackendControl, document: DbIoText, index: u64, class: DurabilityClass },
    WalSeal { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalRead { backend: DbIoBackendControl, document: DbIoText, index: u64, range: ByteRange, output: DbIoPageWriter },
    WalLength { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalList { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    WalTruncate { backend: DbIoBackendControl, document: DbIoText, index: u64, new_len: u64 },
    WalDelete { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    SnapshotWrite { backend: DbIoBackendControl, document: DbIoText, generation: u64, input: DbIoPages },
    SnapshotRead { backend: DbIoBackendControl, document: DbIoText, generation: u64, output: DbIoPageWriter },
    SnapshotLatest { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    SnapshotList { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    SnapshotDelete { backend: DbIoBackendControl, document: DbIoText, generation: u64 },
    PayloadPut { backend: DbIoBackendControl, input: DbIoPages },
    PayloadGet { backend: DbIoBackendControl, hash: ContentHash, output: DbIoPageWriter },
    PayloadExists { backend: DbIoBackendControl, hash: ContentHash },
    PayloadLength { backend: DbIoBackendControl, hash: ContentHash },
    PayloadDelete { backend: DbIoBackendControl, hash: ContentHash },
    CatalogRead { backend: DbIoBackendControl, output: DbIoPageWriter },
    CatalogCas { backend: DbIoBackendControl, expected: EpochFence, input: DbIoPages },
    IndexWrite { backend: DbIoBackendControl, document: DbIoText, run_id: u64, input: DbIoPages },
    IndexRead { backend: DbIoBackendControl, document: DbIoText, run_id: u64, output: DbIoPageWriter },
    IndexList { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    IndexDelete { backend: DbIoBackendControl, document: DbIoText, run_id: u64 },
    LeaseAcquire { backend: DbIoBackendControl, document: DbIoText, holder: DbIoText, now_ms: u64, ttl_ms: u64 },
    LeaseRenew { backend: DbIoBackendControl, document: DbIoText, holder: DbIoText, fence: EpochFence, now_ms: u64, ttl_ms: u64 },
    LeaseRelease { backend: DbIoBackendControl, document: DbIoText, holder: DbIoText, fence: EpochFence },
    LeaseGet { backend: DbIoBackendControl, document: DbIoText, now_ms: u64 },
    BackendClose { backend: DbIoBackendControl },
}

/// @emoji 📬 Exact typed database I/O terminal result.
pub enum DbIoResult {
    Unit,
    Length(u64),
    OptionalLength(Option<u64>),
    Exists(bool),
    Hash(ContentHash),
    Fence(EpochFence),
    Pages(DbIoPages),
    OptionalCatalog(Option<(DbIoPages, EpochFence)>),
    List(DbIoU64List),
    Lease(DbIoLeaseResult),
    OptionalLease(Option<DbIoLeaseResult>),
}

pub struct DbIoLeaseResult {
    pub resource: DbIoText,
    pub holder: DbIoText,
    pub fence: EpochFence,
    pub expires_at_ms: u64,
    result_handback: Option<DbIoTaskHandle>,
}

impl DbIoLeaseResult {
    pub fn new(resource: DbIoText, holder: DbIoText, fence: EpochFence, expires_at_ms: u64) -> Self {
        Self { resource, holder, fence, expires_at_ms, result_handback: None }
    }

    fn handback_step(&mut self) -> bool {
        let Some(handle) = self.result_handback else { return false };
        if db_io_result_handback(handle).is_err() {
            return false;
        }
        self.result_handback = None;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.resource.terminal_is_empty() && self.holder.terminal_is_empty() && self.result_handback.is_none()
    }

    pub fn close_step(&mut self) -> bool {
        self.holder.close_step() || self.resource.close_step() || self.handback_step()
    }
}

impl std::fmt::Debug for DbIoLeaseResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbIoLeaseResult").field("resource", &self.resource).field("holder", &self.holder).field("fence", &self.fence).field("expires_at_ms", &self.expires_at_ms).finish()
    }
}

impl PartialEq for DbIoLeaseResult {
    fn eq(&self, other: &Self) -> bool {
        self.resource == other.resource && self.holder == other.holder && self.fence == other.fence && self.expires_at_ms == other.expires_at_ms
    }
}

impl Eq for DbIoLeaseResult {}

pub type LeaseInfo = DbIoLeaseResult;

impl Drop for DbIoLeaseResult {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner =
            Self { resource: std::mem::replace(&mut self.resource, DbIoText::new()), holder: std::mem::replace(&mut self.holder, DbIoText::new()), fence: self.fence, expires_at_ms: self.expires_at_ms, result_handback: self.result_handback.take() };
        if let Err(DbIoLostOwner::Lease(owner)) = db_io_park_lost_owner(DbIoLostOwner::Lease(owner)) {
            *self = owner;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoTaskPhase {
    Admitted,
    Queued,
    Executing,
    Completed,
    Cancelled,
    Faulted,
    Closing,
}

impl DbIoTask {
    pub fn backend(&self) -> DbIoBackendControl {
        match self {
            Self::BackendOpen { backend, .. }
            | Self::WalCreate { backend, .. }
            | Self::WalAppend { backend, .. }
            | Self::WalSync { backend, .. }
            | Self::WalSeal { backend, .. }
            | Self::WalRead { backend, .. }
            | Self::WalLength { backend, .. }
            | Self::WalList { backend, .. }
            | Self::WalTruncate { backend, .. }
            | Self::WalDelete { backend, .. }
            | Self::SnapshotWrite { backend, .. }
            | Self::SnapshotRead { backend, .. }
            | Self::SnapshotLatest { backend, .. }
            | Self::SnapshotList { backend, .. }
            | Self::SnapshotDelete { backend, .. }
            | Self::PayloadPut { backend, .. }
            | Self::PayloadGet { backend, .. }
            | Self::PayloadExists { backend, .. }
            | Self::PayloadLength { backend, .. }
            | Self::PayloadDelete { backend, .. }
            | Self::CatalogRead { backend, .. }
            | Self::CatalogCas { backend, .. }
            | Self::IndexWrite { backend, .. }
            | Self::IndexRead { backend, .. }
            | Self::IndexList { backend, .. }
            | Self::IndexDelete { backend, .. }
            | Self::LeaseAcquire { backend, .. }
            | Self::LeaseRenew { backend, .. }
            | Self::LeaseRelease { backend, .. }
            | Self::LeaseGet { backend, .. }
            | Self::BackendClose { backend } => *backend,
        }
    }

    fn operation(&self) -> Option<u64> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => Some(input.operation()),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => Some(output.operation()),
            _ => None,
        }
    }

    fn aggregate_credit(&self) -> DbIoCredit {
        let (bytes, items) = match self {
            Self::WalList { .. } | Self::SnapshotLatest { .. } | Self::SnapshotList { .. } | Self::IndexList { .. } => {
                (DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES, DB_IO_LIST_ITEMS * 2 + 1)
            }
            _ => (DB_IO_TASK_SLOT_BYTES, 1),
        };
        DbIoCredit { pages: 0, bytes, items, controls: 1 }
    }

    fn admit_list_backing(&mut self) -> Result<(), DbError> {
        match self {
            Self::WalList { output, .. } | Self::SnapshotLatest { output, .. } | Self::SnapshotList { output, .. } | Self::IndexList { output, .. } if output.is_empty() => output.ensure_backing(),
            Self::WalList { .. } | Self::SnapshotLatest { .. } | Self::SnapshotList { .. } | Self::IndexList { .. } => Err(DbError::Internal("DB I/O list output was not empty at admission".to_string())),
            _ => Ok(()),
        }
    }

    fn release_unstarted_list_backing(&mut self) {
        match self {
            Self::WalList { output, .. } | Self::SnapshotLatest { output, .. } | Self::SnapshotList { output, .. } | Self::IndexList { output, .. } if output.is_empty() => output.values = None,
            _ => {}
        }
    }

    fn admit_pages(&self) -> Result<(), DbError> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => input.admit(),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => {
                output.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued)
            }
            _ => Ok(()),
        }
    }

    fn transition_pages(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => input.transition(expected, next),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => output.transition(expected, next),
            _ => Ok(()),
        }
    }

    fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        match self {
            Self::WalAppend { document, input, .. } | Self::SnapshotWrite { document, input, .. } | Self::IndexWrite { document, input, .. } => {
                if let Some(bytes) = input.close_step()? {
                    return Ok(Some(bytes));
                }
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } => {
                if let Some(bytes) = input.close_step()? {
                    return Ok(Some(bytes));
                }
            }
            Self::WalRead { document, output, .. } | Self::SnapshotRead { document, output, .. } | Self::IndexRead { document, output, .. } => {
                if let Some(bytes) = output.close_step()? {
                    return Ok(Some(bytes));
                }
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } => {
                if let Some(bytes) = output.close_step()? {
                    return Ok(Some(bytes));
                }
            }
            Self::BackendOpen { path, .. } => {
                if path.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::WalList { document, output, .. } | Self::SnapshotLatest { document, output, .. } | Self::SnapshotList { document, output, .. } | Self::IndexList { document, output, .. } => {
                if output.close_step() || document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::WalCreate { document, .. }
            | Self::WalSync { document, .. }
            | Self::WalSeal { document, .. }
            | Self::WalLength { document, .. }
            | Self::WalTruncate { document, .. }
            | Self::WalDelete { document, .. }
            | Self::SnapshotDelete { document, .. }
            | Self::IndexDelete { document, .. } => {
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::LeaseAcquire { document, holder, .. } | Self::LeaseRenew { document, holder, .. } | Self::LeaseRelease { document, holder, .. } => {
                if holder.close_step() || document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::LeaseGet { document, .. } => {
                if document.close_step() {
                    return Ok(Some(0));
                }
            }
            Self::PayloadExists { .. } | Self::PayloadLength { .. } | Self::PayloadDelete { .. } | Self::BackendClose { .. } => {}
        }
        Ok(None)
    }

    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::WalAppend { document, input, .. } | Self::SnapshotWrite { document, input, .. } | Self::IndexWrite { document, input, .. } => document.terminal_is_empty() && input.terminal_is_empty(),
            Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } => input.terminal_is_empty(),
            Self::WalRead { document, output, .. } | Self::SnapshotRead { document, output, .. } | Self::IndexRead { document, output, .. } => document.terminal_is_empty() && output.terminal_is_empty(),
            Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } => output.terminal_is_empty(),
            Self::BackendOpen { path, .. } => path.terminal_is_empty(),
            Self::WalList { document, output, .. } | Self::SnapshotLatest { document, output, .. } | Self::SnapshotList { document, output, .. } | Self::IndexList { document, output, .. } => document.terminal_is_empty() && output.terminal_is_empty(),
            Self::WalCreate { document, .. }
            | Self::WalSync { document, .. }
            | Self::WalSeal { document, .. }
            | Self::WalLength { document, .. }
            | Self::WalTruncate { document, .. }
            | Self::WalDelete { document, .. }
            | Self::SnapshotDelete { document, .. }
            | Self::IndexDelete { document, .. }
            | Self::LeaseGet { document, .. } => document.terminal_is_empty(),
            Self::LeaseAcquire { document, holder, .. } | Self::LeaseRenew { document, holder, .. } | Self::LeaseRelease { document, holder, .. } => document.terminal_is_empty() && holder.terminal_is_empty(),
            Self::PayloadExists { .. } | Self::PayloadLength { .. } | Self::PayloadDelete { .. } | Self::BackendClose { .. } => true,
        }
    }
}

impl DbIoResult {
    fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        match self {
            Self::Pages(pages) => pages.close_step(),
            Self::OptionalCatalog(Some((pages, _))) => pages.close_step(),
            Self::List(list) => Ok(list.close_step().then_some(0)),
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => {
                if lease.holder.close_step() || lease.resource.close_step() || lease.handback_step() {
                    return Ok(Some(0));
                }
                Ok(None)
            }
            Self::Unit | Self::Length(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => Ok(None),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Pages(pages) => pages.terminal_is_empty(),
            Self::OptionalCatalog(Some((pages, _))) => pages.terminal_is_empty(),
            Self::List(list) => list.terminal_is_empty(),
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => lease.terminal_is_empty(),
            Self::Unit | Self::Length(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => true,
        }
    }

    fn attach_result_handback(&mut self, handle: DbIoTaskHandle) -> bool {
        match self {
            Self::Pages(pages) => {
                pages.result_handback = Some(handle);
                true
            }
            Self::OptionalCatalog(Some((pages, _))) => {
                pages.result_handback = Some(handle);
                true
            }
            Self::List(list) => {
                list.result_handback = Some(handle);
                true
            }
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => {
                lease.result_handback = Some(handle);
                true
            }
            Self::Unit | Self::Length(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoExecutionStep {
    Yield,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoExecutorMode {
    BlockingLane,
    AsyncNative,
}

pub type DbIoAsyncDriverFuture = std::pin::Pin<Box<dyn std::future::Future<Output = (Box<dyn DbIoTaskExecutor>, DbIoTask, Result<DbIoResult, DbError>)> + Send + 'static>>;

/// @emoji 🔌 Platform drivers implement one typed, resumable task step behind repository owners.
pub trait DbIoTaskExecutor: Send + Sync {
    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::BlockingLane
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn bind_owner_operation(&mut self, _operation: u64) -> Result<(), DbError> {
        Ok(())
    }

    fn owner_backing_bytes(&self) -> u64 {
        std::mem::size_of_val(self) as u64
    }

    fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError>;

    fn drive_async(self: Box<Self>, operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture;

    fn close_operation_step(&self, operation: u64, task: &DbIoTask) -> Result<bool, DbError>;

    fn close_backend_step(&mut self, context: &mut std::task::Context<'_>) -> Result<bool, DbError>;

    fn backend_terminal_is_empty(&self) -> bool;
}

struct DbIoBackendRegistrySlot {
    generation: u64,
    kind: DbIoBackendKind,
    executor: Option<Box<dyn DbIoTaskExecutor>>,
    executor_retired: bool,
    close_requested: bool,
    leased_operation: u64,
    admitted_operation: u64,
    pending_operations: usize,
    owner_operation: u64,
    owner_credit: DbIoCredit,
    mode: DbIoExecutorMode,
    pool: Option<Arc<WorkerPool>>,
    close_scheduled: bool,
    close_lane_turn: bool,
    close_wake_requested: bool,
    close_fault: Option<DbIoText>,
}

struct DbIoBackendRegistry {
    slots: [DbIoBackendRegistrySlot; DB_IO_BACKEND_CONTROLS],
    free: [u16; DB_IO_BACKEND_CONTROLS],
    free_read: usize,
    free_len: usize,
    next_generation: u64,
}

impl DbIoBackendRegistry {
    fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| DbIoBackendRegistrySlot {
                generation: 0,
                kind: DbIoBackendKind::Memory,
                executor: None,
                executor_retired: false,
                close_requested: false,
                leased_operation: 0,
                admitted_operation: 0,
                pending_operations: 0,
                owner_operation: 0,
                owner_credit: DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 0 },
                mode: DbIoExecutorMode::BlockingLane,
                pool: None,
                close_scheduled: false,
                close_lane_turn: false,
                close_wake_requested: false,
                close_fault: None,
            }),
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: DB_IO_BACKEND_CONTROLS,
            next_generation: 1,
        }
    }
}

static DB_IO_BACKEND_REGISTRY: std::sync::OnceLock<std::sync::Mutex<DbIoBackendRegistry>> = std::sync::OnceLock::new();

fn db_io_backend_registry() -> &'static std::sync::Mutex<DbIoBackendRegistry> {
    DB_IO_BACKEND_REGISTRY.get_or_init(|| std::sync::Mutex::new(DbIoBackendRegistry::new()))
}

struct DbIoRejectedBackendSlot {
    generation: u64,
    executor: Option<Box<dyn DbIoTaskExecutor>>,
    operation: u64,
    credit: DbIoCredit,
    pool: Option<Arc<WorkerPool>>,
    executor_retired: bool,
    scheduled: bool,
    lane_turn: bool,
    wake_requested: bool,
    fault: Option<DbIoText>,
}

impl DbIoRejectedBackendSlot {
    fn empty() -> Self {
        Self { generation: 0, executor: None, operation: 0, credit: DbIoCredit::default(), pool: None, executor_retired: false, scheduled: false, lane_turn: false, wake_requested: false, fault: None }
    }
}

struct DbIoRejectedBackendRegistry {
    slots: [DbIoRejectedBackendSlot; DB_IO_BACKEND_CONTROLS],
    next_generation: u64,
}

static DB_IO_REJECTED_BACKENDS: std::sync::OnceLock<std::sync::Mutex<DbIoRejectedBackendRegistry>> = std::sync::OnceLock::new();

fn db_io_rejected_backends() -> &'static std::sync::Mutex<DbIoRejectedBackendRegistry> {
    DB_IO_REJECTED_BACKENDS.get_or_init(|| std::sync::Mutex::new(DbIoRejectedBackendRegistry { slots: std::array::from_fn(|_| DbIoRejectedBackendSlot::empty()), next_generation: 1 }))
}

fn db_io_park_rejected_backend(executor: Box<dyn DbIoTaskExecutor>, operation: u64, credit: DbIoCredit, pool: Arc<WorkerPool>) -> Result<(), (Box<dyn DbIoTaskExecutor>, Arc<WorkerPool>)> {
    let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(index) = registry.slots.iter().position(|slot| slot.generation == 0) else { return Err((executor, pool)) };
    let generation = registry.next_generation;
    let Some(next_generation) = generation.checked_add(1).filter(|generation| *generation != 0) else { return Err((executor, pool)) };
    registry.next_generation = next_generation;
    registry.slots[index] = DbIoRejectedBackendSlot { generation, executor: Some(executor), operation, credit, pool: Some(pool), executor_retired: false, scheduled: false, lane_turn: false, wake_requested: false, fault: None };
    drop(registry);
    let _ = db_io_request_rejected_backend_close(index, generation);
    Ok(())
}

struct DbIoRejectedBackendWake {
    index: usize,
    generation: u64,
}

impl std::task::Wake for DbIoRejectedBackendWake {
    fn wake(self: Arc<Self>) {
        let _ = db_io_request_rejected_backend_close(self.index, self.generation);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let _ = db_io_request_rejected_backend_close(self.index, self.generation);
    }
}

fn db_io_poll_rejected_backend_on_lane_io(index: usize, generation: u64) {
    {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation {
            return;
        }
        owner.scheduled = false;
        owner.lane_turn = true;
        owner.wake_requested = false;
    }
    let waker = std::task::Waker::from(Arc::new(DbIoRejectedBackendWake { index, generation }));
    let context = &mut std::task::Context::from_waker(&waker);
    let mut terminal = false;
    let mut fault = None;
    let mut executor = {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation {
            return;
        }
        if owner.executor_retired {
            None
        } else {
            let Some(executor) = owner.executor.take() else {
                owner.fault = Some(db_io_text_literal("DB I/O rejected backend lost its exact executor"));
                owner.lane_turn = false;
                return;
            };
            Some(executor)
        }
    };
    let close = executor.as_mut().map(|executor| executor.close_backend_step(context));
    {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation {
            return;
        }
        if let Some(close) = close {
            match close {
                Ok(false) => owner.executor = executor.take(),
                Ok(true) if executor.as_ref().is_some_and(|executor| executor.backend_terminal_is_empty()) => {
                    owner.executor_retired = true;
                }
                Ok(true) => {
                    owner.executor = executor.take();
                    fault = Some(db_io_text_literal("DB I/O rejected backend returned a false terminal witness"));
                }
                Err(error) => {
                    owner.executor = executor.take();
                    fault = Some(db_io_error_text(&error));
                }
            }
        } else if owner.operation != 0 {
            match db_io_operation_return(owner.operation, owner.credit) {
                Ok(()) => {
                    owner.operation = 0;
                    owner.credit = DbIoCredit::default();
                }
                Err(error) => fault = Some(db_io_error_text(&error)),
            }
        } else {
            terminal = true;
        }
        owner.fault = fault;
        owner.lane_turn = false;
    }
    if terminal {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.slots[index].generation == generation {
            registry.slots[index] = DbIoRejectedBackendSlot::empty();
        }
    } else {
        let _ = db_io_request_rejected_backend_close(index, generation);
    }
}

fn db_io_request_rejected_backend_close(index: usize, generation: u64) -> Result<bool, DbError> {
    let pool = {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation || owner.fault.is_some() {
            return Ok(false);
        }
        if owner.scheduled || owner.lane_turn {
            owner.wake_requested = true;
            return Ok(false);
        }
        owner.scheduled = true;
        owner.pool.clone().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its shared WorkerPool".to_string()))?
    };
    match pool.try_submit(Lane::Io, Box::new(move || db_io_poll_rejected_backend_on_lane_io(index, generation))) {
        Ok(()) => Ok(true),
        Err(error) => {
            drop(error.into_job());
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[index];
            if owner.generation == generation {
                owner.scheduled = false;
                owner.wake_requested = true;
            }
            Ok(false)
        }
    }
}

fn db_io_rejected_backend_maintenance_step() -> Result<bool, DbError> {
    let candidate = {
        let registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry.slots.iter().enumerate().find(|(_, owner)| owner.generation != 0 && !owner.scheduled && !owner.lane_turn && owner.fault.is_none()).map(|(index, owner)| (index, owner.generation))
    };
    let Some((index, generation)) = candidate else { return Ok(false) };
    db_io_request_rejected_backend_close(index, generation)
}

pub fn register_db_io_backend(kind: DbIoBackendKind, executor: Box<dyn DbIoTaskExecutor>, pool: Arc<WorkerPool>) -> Result<DbIoBackendControl, DbError> {
    let owner_credit = DbIoCredit { pages: 0, bytes: executor.owner_backing_bytes(), items: 1, controls: 1 };
    let owner_operation = match db_io_backend_owner_reserve(owner_credit) {
        Ok(operation) => operation,
        Err(error) => {
            let _ = db_io_park_lost_owner(DbIoLostOwner::Backend { owner: Some(executor), operation: 0, credit: DbIoCredit::default(), pool: Some(pool) });
            return Err(error);
        }
    };
    register_db_io_backend_reserved(kind, executor, pool, owner_operation, owner_credit)
}

fn register_db_io_backend_reserved(kind: DbIoBackendKind, mut executor: Box<dyn DbIoTaskExecutor>, pool: Arc<WorkerPool>, owner_operation: u64, owner_credit: DbIoCredit) -> Result<DbIoBackendControl, DbError> {
    if executor.owner_backing_bytes() != owner_credit.bytes {
        let _ = db_io_park_lost_owner(DbIoLostOwner::Backend { owner: Some(executor), operation: owner_operation, credit: owner_credit, pool: Some(pool) });
        return Err(DbError::Internal("DB I/O backend backing differs from reserved bytes".to_string()));
    }
    if let Err(error) = executor.bind_owner_operation(owner_operation) {
        let _ = db_io_park_lost_owner(DbIoLostOwner::Backend { owner: Some(executor), operation: owner_operation, credit: owner_credit, pool: Some(pool) });
        return Err(error);
    }
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if registry.free_len == 0 || registry.next_generation == u64::MAX {
        drop(registry);
        let _ = db_io_park_lost_owner(DbIoLostOwner::Backend { owner: Some(executor), operation: owner_operation, credit: owner_credit, pool: Some(pool) });
        return Err(DbError::Unavailable("db I/O backend control capacity exhausted".to_string()));
    }
    let slot = registry.free[registry.free_read];
    registry.free_read = (registry.free_read + 1) % DB_IO_BACKEND_CONTROLS;
    registry.free_len -= 1;
    let generation = registry.next_generation;
    registry.next_generation += 1;
    let mode = executor.mode();
    registry.slots[slot as usize] = DbIoBackendRegistrySlot {
        generation,
        kind,
        executor: Some(executor),
        executor_retired: false,
        close_requested: false,
        leased_operation: 0,
        admitted_operation: 0,
        pending_operations: 0,
        owner_operation,
        owner_credit,
        mode,
        pool: Some(pool),
        close_scheduled: false,
        close_lane_turn: false,
        close_wake_requested: false,
        close_fault: None,
    };
    let control = match kind {
        DbIoBackendKind::Memory => DbIoBackendControl::Memory { slot, generation },
        DbIoBackendKind::Filesystem => DbIoBackendControl::Filesystem { slot, generation },
        DbIoBackendKind::Sqlite => DbIoBackendControl::Sqlite { slot, generation },
        DbIoBackendKind::Postgres => DbIoBackendControl::Postgres { slot, generation },
        DbIoBackendKind::Neo4j => DbIoBackendControl::Neo4j { slot, generation },
    };
    Ok(control)
}

fn db_io_backend_parts(control: DbIoBackendControl) -> (u16, u64) {
    match control {
        DbIoBackendControl::Memory { slot, generation }
        | DbIoBackendControl::Filesystem { slot, generation }
        | DbIoBackendControl::Sqlite { slot, generation }
        | DbIoBackendControl::Postgres { slot, generation }
        | DbIoBackendControl::Neo4j { slot, generation } => (slot, generation),
    }
}

fn db_io_backend_control(kind: DbIoBackendKind, slot: u16, generation: u64) -> DbIoBackendControl {
    match kind {
        DbIoBackendKind::Memory => DbIoBackendControl::Memory { slot, generation },
        DbIoBackendKind::Filesystem => DbIoBackendControl::Filesystem { slot, generation },
        DbIoBackendKind::Sqlite => DbIoBackendControl::Sqlite { slot, generation },
        DbIoBackendKind::Postgres => DbIoBackendControl::Postgres { slot, generation },
        DbIoBackendKind::Neo4j => DbIoBackendControl::Neo4j { slot, generation },
    }
}

pub fn retire_db_io_backend(control: DbIoBackendControl) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    owner.close_requested = true;
    drop(registry);
    let _ = db_io_request_backend_close(control)?;
    Ok(())
}

pub async fn close_db_io_backend(control: DbIoBackendControl) -> Result<(), DbError> {
    let _ = db_io_request_backend_close(control)?;
    std::future::poll_fn(|context| {
        let (slot, generation) = db_io_backend_parts(control);
        let (terminal, fault) = {
            let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &registry.slots[slot as usize];
            (owner.generation != generation, owner.close_fault.as_ref().map(|fault| fault.as_str().to_string()))
        };
        if let Some(fault) = fault {
            return std::task::Poll::Ready(Err(DbError::Internal(fault)));
        }
        let tasks_terminal = DB_IO_TASK_SLOTS.iter().all(|task| {
            let owner = task.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            owner.backend != Some(control)
        });
        if terminal && tasks_terminal {
            return std::task::Poll::Ready(Ok(()));
        }
        if let Err(error) = db_io_request_backend_close(control) {
            return std::task::Poll::Ready(Err(error));
        }
        if let Err(error) = db_io_maintenance_step() {
            return std::task::Poll::Ready(Err(error));
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    })
    .await
}

fn db_io_executor_execute(control: DbIoBackendControl, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    if owner.mode != DbIoExecutorMode::BlockingLane || owner.admitted_operation != 0 {
        return Err(DbError::Unavailable("DB I/O blocking backend operation authority occupied".to_string()));
    }
    owner.admitted_operation = operation;
    let execution = match owner.executor.as_ref() {
        Some(executor) => std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| executor.execute_step(operation, task))),
        None => Ok(Err(DbError::Closed)),
    };
    owner.admitted_operation = 0;
    match execution {
        Ok(result) => result,
        Err(payload) => {
            drop(registry);
            std::panic::resume_unwind(payload)
        }
    }
}

fn db_io_executor_mode(control: DbIoBackendControl) -> Result<DbIoExecutorMode, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[slot as usize];
    if owner.generation != generation {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    Ok(owner.mode)
}

fn db_io_backend_admit_operation(control: DbIoBackendControl, operation: u64) -> Result<bool, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    if owner.close_requested || owner.executor_retired {
        return Err(DbError::Closed);
    }
    if owner.mode != DbIoExecutorMode::BlockingLane && owner.admitted_operation != 0 {
        return Err(DbError::Unavailable("DB I/O async-native backend operation capacity exhausted".to_string()));
    }
    let pending = owner.pending_operations.checked_add(1).filter(|count| *count <= DB_IO_OPERATION_ITEMS).ok_or(DbError::LimitExceeded("DB I/O backend pending operations"))?;
    if owner.mode != DbIoExecutorMode::BlockingLane { owner.admitted_operation = operation; }
    owner.pending_operations = pending;
    Ok(true)
}

fn db_io_backend_return_operation(control: DbIoBackendControl, operation: u64) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation || owner.pending_operations == 0
        || owner.mode != DbIoExecutorMode::BlockingLane && (owner.admitted_operation != operation || owner.leased_operation != 0) {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    if owner.mode != DbIoExecutorMode::BlockingLane { owner.admitted_operation = 0; }
    owner.pending_operations -= 1;
    Ok(())
}

fn db_io_take_async_executor(control: DbIoBackendControl, operation: u64) -> Result<Box<dyn DbIoTaskExecutor>, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    if owner.leased_operation != 0 {
        return Err(DbError::Unavailable("DB I/O async-native backend already has an operation lease".to_string()));
    }
    let executor = owner.executor.take().ok_or(DbError::Closed)?;
    owner.leased_operation = operation;
    Ok(executor)
}

fn db_io_return_async_executor(control: DbIoBackendControl, operation: u64, executor: Box<dyn DbIoTaskExecutor>) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation || owner.leased_operation != operation || owner.executor.is_some() {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    owner.executor = Some(executor);
    owner.leased_operation = 0;
    Ok(())
}

fn db_io_executor_close_operation(control: DbIoBackendControl, operation: u64, task: &DbIoTask) -> Result<bool, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[slot as usize];
    if owner.generation != generation {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    owner.executor.as_ref().ok_or(DbError::Closed)?.close_operation_step(operation, task)
}

fn db_io_backend_close_lane_step(control: DbIoBackendControl, context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let executor = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
        }
        if owner.pending_operations != 0 || owner.admitted_operation != 0 || owner.leased_operation != 0 {
            return Ok(false);
        }
        if owner.executor_retired {
            None
        } else {
            Some(owner.executor.take().ok_or(DbError::Closed)?)
        }
    };
    if let Some(mut executor) = executor {
        let close = executor.close_backend_step(context);
        let terminal_empty = executor.backend_terminal_is_empty();
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
        }
        match close {
            Ok(false) => owner.executor = Some(executor),
            Ok(true) if terminal_empty => owner.executor_retired = true,
            Ok(true) => {
                owner.executor = Some(executor);
                return Err(DbError::Internal("DB I/O backend returned a false terminal witness".to_string()));
            }
            Err(error) => {
                owner.executor = Some(executor);
                return Err(error);
            }
        }
        return Ok(false);
    }
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    if owner.owner_operation != 0 {
        db_io_operation_return(owner.owner_operation, owner.owner_credit)?;
        owner.owner_operation = 0;
        owner.owner_credit = DbIoCredit::default();
        return Ok(false);
    }
    registry.slots[slot as usize] = DbIoBackendRegistrySlot {
        generation: 0,
        kind: DbIoBackendKind::Memory,
        executor: None,
        executor_retired: false,
        close_requested: false,
        leased_operation: 0,
        admitted_operation: 0,
        pending_operations: 0,
        owner_operation: 0,
        owner_credit: DbIoCredit::default(),
        mode: DbIoExecutorMode::BlockingLane,
        pool: None,
        close_scheduled: false,
        close_lane_turn: false,
        close_wake_requested: false,
        close_fault: None,
    };
    let write = (registry.free_read + registry.free_len) % DB_IO_BACKEND_CONTROLS;
    registry.free[write] = slot;
    registry.free_len += 1;
    Ok(true)
}

struct DbIoBackendCloseWake {
    control: DbIoBackendControl,
}

impl std::task::Wake for DbIoBackendCloseWake {
    fn wake(self: Arc<Self>) {
        let _ = db_io_request_backend_close(self.control);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let _ = db_io_request_backend_close(self.control);
    }
}

fn db_io_poll_backend_close_on_lane_io(control: DbIoBackendControl) {
    {
        let (slot, generation) = db_io_backend_parts(control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || !owner.close_requested {
            return;
        }
        owner.close_scheduled = false;
        owner.close_lane_turn = true;
        owner.close_wake_requested = false;
    }
    let waker = std::task::Waker::from(Arc::new(DbIoBackendCloseWake { control }));
    let context = &mut std::task::Context::from_waker(&waker);
    let terminal = match db_io_backend_close_lane_step(control, context) {
        Ok(terminal) => terminal,
        Err(error) => {
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            let (slot, generation) = db_io_backend_parts(control);
            let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[slot as usize];
            if owner.generation == generation {
                owner.close_fault = Some(db_io_error_text(&error));
            }
            false
        }
    };
    let retry = {
        let (slot, generation) = db_io_backend_parts(control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation {
            false
        } else {
            owner.close_lane_turn = false;
            owner.close_fault.is_none() && (owner.close_wake_requested || !terminal)
        }
    };
    if retry {
        let _ = db_io_request_backend_close(control);
    }
}

fn db_io_request_backend_close(control: DbIoBackendControl) -> Result<bool, DbError> {
    let pool = {
        let (slot, generation) = db_io_backend_parts(control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation {
            return Ok(false);
        }
        if let Some(fault) = owner.close_fault.as_ref() {
            return Err(DbError::Internal(fault.as_str().to_string()));
        }
        owner.close_requested = true;
        if owner.close_scheduled || owner.close_lane_turn {
            owner.close_wake_requested = true;
            return Ok(false);
        }
        owner.close_scheduled = true;
        owner.pool.clone().ok_or_else(|| DbError::Internal("DB I/O backend close lost its shared WorkerPool authority".to_string()))?
    };
    match pool.try_submit(Lane::Io, Box::new(move || db_io_poll_backend_close_on_lane_io(control))) {
        Ok(()) => Ok(true),
        Err(error) => {
            drop(error.into_job());
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            let (slot, generation) = db_io_backend_parts(control);
            let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[slot as usize];
            if owner.generation == generation {
                owner.close_scheduled = false;
                owner.close_wake_requested = true;
            }
            Ok(false)
        }
    }
}

static DB_IO_BACKEND_MAINTENANCE_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn db_io_backend_maintenance_step() -> Result<bool, DbError> {
    let index = DB_IO_BACKEND_MAINTENANCE_CURSOR.fetch_add(1, std::sync::atomic::Ordering::AcqRel) % DB_IO_BACKEND_CONTROLS;
    let control = {
        let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &registry.slots[index];
        if owner.generation == 0 || !owner.close_requested || owner.admitted_operation != 0 || owner.leased_operation != 0 {
            return Ok(false);
        }
        db_io_backend_control(owner.kind, index as u16, owner.generation)
    };
    db_io_request_backend_close(control)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoFaultKind {
    Backend,
    Cancelled,
    Panic,
    Saturated,
    Stale,
}

/// @emoji 🧩 Bounded storage-error taxonomy retained independently from runner provenance.
///
/// DbError::Rejected is an artifact-engine outcome above this storage executor boundary; a
/// backend returning it violates the layer contract and is retained as Internal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoFaultCause {
    Io,
    NotFound,
    AlreadyExists,
    InvalidArgument,
    LimitExceeded(&'static str),
    Conflict,
    Fenced { expected: u64, actual: u64 },
    StaleGeneration { expected: u64, actual: u64 },
    Unavailable,
    Timeout,
    Corrupt,
    Closed,
    Unauthorized,
    Unimplemented(&'static str),
    Internal,
}

impl DbIoFaultCause {
    fn from_db_error(error: &DbError) -> Self {
        match error {
            DbError::Io(_) => Self::Io,
            DbError::NotFound(_) => Self::NotFound,
            DbError::AlreadyExists(_) => Self::AlreadyExists,
            DbError::InvalidArgument(_) => Self::InvalidArgument,
            DbError::LimitExceeded(detail) => Self::LimitExceeded(detail),
            DbError::Conflict(_) => Self::Conflict,
            DbError::Fenced { expected, actual } => Self::Fenced { expected: *expected, actual: *actual },
            DbError::StaleGeneration { expected, actual } => Self::StaleGeneration { expected: expected.0, actual: actual.0 },
            DbError::Unavailable(_) => Self::Unavailable,
            DbError::Timeout(_) => Self::Timeout,
            DbError::Corrupt(_) => Self::Corrupt,
            DbError::Closed => Self::Closed,
            DbError::Unauthorized(_) => Self::Unauthorized,
            DbError::Unimplemented(detail) => Self::Unimplemented(detail),
            DbError::Internal(_) => Self::Internal,
            DbError::Rejected { .. } => Self::Internal,
        }
    }
}

pub struct DbIoFault {
    pub kind: DbIoFaultKind,
    pub cause: DbIoFaultCause,
    pub detail: DbIoText,
    result_handback: Option<DbIoTaskHandle>,
}

impl std::fmt::Debug for DbIoFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbIoFault").field("kind", &self.kind).field("cause", &self.cause).field("detail", &self.detail.as_str()).field("has_result_handback", &self.result_handback.is_some()).finish()
    }
}

impl DbIoFault {
    pub fn into_db_error(self) -> DbError {
        let detail = self.detail.as_str().to_string();
        match self.cause {
            DbIoFaultCause::Io => DbError::Io(detail),
            DbIoFaultCause::NotFound => DbError::NotFound(detail),
            DbIoFaultCause::AlreadyExists => DbError::AlreadyExists(detail),
            DbIoFaultCause::InvalidArgument => DbError::InvalidArgument(detail),
            DbIoFaultCause::LimitExceeded(detail) => DbError::LimitExceeded(detail),
            DbIoFaultCause::Conflict => DbError::Conflict(detail),
            DbIoFaultCause::Fenced { expected, actual } => DbError::Fenced { expected, actual },
            DbIoFaultCause::StaleGeneration { expected, actual } => DbError::StaleGeneration { expected: crate::db_ids::GenerationId(expected), actual: crate::db_ids::GenerationId(actual) },
            DbIoFaultCause::Unavailable => DbError::Unavailable(detail),
            DbIoFaultCause::Timeout => DbError::Timeout(detail),
            DbIoFaultCause::Corrupt => DbError::Corrupt(detail),
            DbIoFaultCause::Closed => DbError::Closed,
            DbIoFaultCause::Unauthorized => DbError::Unauthorized(detail),
            DbIoFaultCause::Unimplemented(detail) => DbError::Unimplemented(detail),
            DbIoFaultCause::Internal => DbError::Internal(detail),
        }
    }

    pub fn close_step(&mut self) -> bool {
        if self.detail.close_step() {
            return true;
        }
        if let Some(handle) = self.result_handback {
            if db_io_result_handback(handle).is_ok() {
                self.result_handback = None;
                return true;
            }
        }
        false
    }
}

impl Drop for DbIoFault {
    fn drop(&mut self) {
        if self.detail.terminal_is_empty() && self.result_handback.is_none() {
            return;
        }
        let owner = Self { kind: self.kind, cause: self.cause, detail: std::mem::replace(&mut self.detail, DbIoText::new()), result_handback: self.result_handback.take() };
        if let Err(DbIoLostOwner::Fault(owner)) = db_io_park_lost_owner(DbIoLostOwner::Fault(owner)) {
            *self = owner;
        }
    }
}

enum DbIoTerminal {
    Result(DbIoResult),
    Fault(DbIoFault),
    Cancelled(Option<DbIoResult>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DbIoTaskHandle {
    slot: u16,
    generation: u64,
    operation: u64,
}

struct DbIoTaskSlot {
    generation: u64,
    operation: u64,
    phase: DbIoTaskPhase,
    task: Option<DbIoTask>,
    terminal: Option<DbIoTerminal>,
    pool: Option<WorkerPool>,
    retry_attempt: Option<u8>,
    terminal_submit_kind: Option<WorkerSubmitErrorKind>,
    waker: Option<std::task::Waker>,
    retry_generation: u64,
    cancelled: bool,
    abandoned: bool,
    close_enqueued: bool,
    backend_cleanup_done: bool,
    backend_to_close: Option<DbIoBackendControl>,
    backend: Option<DbIoBackendControl>,
    counted: bool,
    aggregate_credit: DbIoCredit,
    aggregate_returned: bool,
    async_ready: bool,
    async_detached: bool,
    async_lane_turn: bool,
    backend_admitted: bool,
    async_driver: Option<DbIoAsyncDriverFuture>,
    async_driver_scheduled: bool,
    async_driver_wake_requested: bool,
}

impl DbIoTaskSlot {
    const fn empty() -> Self {
        Self {
            generation: 0,
            operation: 0,
            phase: DbIoTaskPhase::Closing,
            task: None,
            terminal: None,
            pool: None,
            retry_attempt: None,
            terminal_submit_kind: None,
            waker: None,
            retry_generation: 0,
            cancelled: false,
            abandoned: false,
            close_enqueued: false,
            backend_cleanup_done: false,
            backend_to_close: None,
            backend: None,
            counted: false,
            aggregate_credit: DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 0 },
            aggregate_returned: false,
            async_ready: false,
            async_detached: false,
            async_lane_turn: false,
            backend_admitted: false,
            async_driver: None,
            async_driver_scheduled: false,
            async_driver_wake_requested: false,
        }
    }
}

static DB_IO_TASK_SLOTS: [std::sync::Mutex<DbIoTaskSlot>; DB_IO_OPERATION_ITEMS] = [const { std::sync::Mutex::new(DbIoTaskSlot::empty()) }; DB_IO_OPERATION_ITEMS];

struct DbIoTaskArena {
    free: [u16; DB_IO_OPERATION_ITEMS],
    free_read: usize,
    free_len: usize,
    closing: [Option<DbIoTaskHandle>; DB_IO_OPERATION_ITEMS],
    closing_read: usize,
    closing_len: usize,
    next_generation: u64,
}

impl DbIoTaskArena {
    fn new() -> Self {
        Self { free: std::array::from_fn(|index| index as u16), free_read: 0, free_len: DB_IO_OPERATION_ITEMS, closing: [None; DB_IO_OPERATION_ITEMS], closing_read: 0, closing_len: 0, next_generation: 1 }
    }
}

static DB_IO_TASK_ARENA: std::sync::OnceLock<std::sync::Mutex<DbIoTaskArena>> = std::sync::OnceLock::new();

fn db_io_task_arena() -> &'static std::sync::Mutex<DbIoTaskArena> {
    DB_IO_TASK_ARENA.get_or_init(|| std::sync::Mutex::new(DbIoTaskArena::new()))
}

fn db_io_task_fault(kind: DbIoFaultKind, error: &DbError) -> DbIoFault {
    let cause = match kind {
        DbIoFaultKind::Cancelled => DbIoFaultCause::Closed,
        DbIoFaultKind::Panic => DbIoFaultCause::Internal,
        DbIoFaultKind::Saturated => DbIoFaultCause::Unavailable,
        DbIoFaultKind::Backend | DbIoFaultKind::Stale => DbIoFaultCause::from_db_error(error),
    };
    let detail = match error {
        DbError::Io(detail)
        | DbError::NotFound(detail)
        | DbError::AlreadyExists(detail)
        | DbError::InvalidArgument(detail)
        | DbError::Conflict(detail)
        | DbError::Unavailable(detail)
        | DbError::Timeout(detail)
        | DbError::Corrupt(detail)
        | DbError::Unauthorized(detail)
        | DbError::Internal(detail) => DbIoText::try_from_str(detail).unwrap_or_else(|_| db_io_text_literal("DB I/O fault detail exceeded fixed authority")),
        DbError::LimitExceeded(detail) | DbError::Unimplemented(detail) => db_io_text_literal(detail),
        DbError::Rejected { .. } => db_io_text_literal("DB I/O backend returned an artifact-layer rejection"),
        DbError::Fenced { .. } | DbError::StaleGeneration { .. } | DbError::Closed => db_io_error_text(error),
    };
    DbIoFault { kind, cause, detail, result_handback: None }
}

fn db_io_literal_fault(kind: DbIoFaultKind, cause: DbIoFaultCause, detail: &'static str) -> DbIoFault {
    DbIoFault { kind, cause, detail: db_io_text_literal(detail), result_handback: None }
}

fn db_io_text_literal(value: &'static str) -> DbIoText {
    match DbIoText::try_from_str(value) {
        Ok(text) => text,
        Err(_) => DbIoText::new(),
    }
}

fn db_io_error_text(error: &DbError) -> DbIoText {
    DbIoText::try_from_str(&error.to_string()).unwrap_or_else(|_| db_io_text_literal("DB I/O error detail exceeded fixed authority"))
}

fn db_io_allocate_task(pool: &WorkerPool, mut task: DbIoTask) -> Result<DbIoTaskHandle, (DbError, DbIoTask)> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if arena.free_len == 0 || arena.next_generation == u64::MAX {
        return Err((DbError::Unavailable("db I/O task capacity exhausted".to_string()), task));
    }
    let aggregate_credit = task.aggregate_credit();
    let (operation, attached) = match task.operation() {
        Some(operation) => match db_io_operation_attach_task(operation, aggregate_credit) {
            Ok(()) => (operation, true),
            Err(error) => return Err((error, task)),
        },
        None => match db_io_operation_reserve(aggregate_credit) {
            Ok(operation) => {
                if let Err(error) = db_io_operation_mark_task(operation) {
                    let _ = db_io_operation_return(operation, aggregate_credit);
                    return Err((error, task));
                }
                (operation, false)
            }
            Err(error) => return Err((error, task)),
        },
    };
    if let Err(error) = task.admit_list_backing() {
        let _ = db_io_operation_detach_task(operation, aggregate_credit);
        return Err((error, task));
    }
    let backend_admitted = match db_io_backend_admit_operation(task.backend(), operation) {
        Ok(admitted) => admitted,
        Err(error) => {
            task.release_unstarted_list_backing();
            let _ = db_io_operation_detach_task(operation, aggregate_credit);
            return Err((error, task));
        }
    };
    if let Err(error) = task.admit_pages() {
        if backend_admitted {
            let _ = db_io_backend_return_operation(task.backend(), operation);
        }
        task.release_unstarted_list_backing();
        let _ = db_io_operation_detach_task(operation, aggregate_credit);
        return Err((error, task));
    }
    let slot = arena.free[arena.free_read];
    arena.free_read = (arena.free_read + 1) % DB_IO_OPERATION_ITEMS;
    arena.free_len -= 1;
    let generation = arena.next_generation;
    arena.next_generation += 1;
    drop(arena);
    let handle = DbIoTaskHandle { slot, generation, operation };
    let mut owner = DB_IO_TASK_SLOTS[slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend_to_close = match &task {
        DbIoTask::BackendClose { backend } => Some(*backend),
        _ => None,
    };
    let backend = task.backend();
    *owner = DbIoTaskSlot {
        generation,
        operation,
        phase: DbIoTaskPhase::Admitted,
        task: Some(task),
        terminal: None,
        pool: Some(pool.clone()),
        retry_attempt: None,
        terminal_submit_kind: None,
        waker: None,
        retry_generation: 1,
        cancelled: false,
        abandoned: false,
        close_enqueued: false,
        backend_cleanup_done: false,
        backend_to_close,
        backend: Some(backend),
        counted: true,
        aggregate_credit,
        aggregate_returned: false,
        async_ready: false,
        async_detached: false,
        async_lane_turn: false,
        backend_admitted,
        async_driver: None,
        async_driver_scheduled: false,
        async_driver_wake_requested: false,
    };
    let _ = attached;
    BLOCKING_QUEUE.enqueued(aggregate_credit.bytes);
    Ok(handle)
}

fn db_io_slot_matches(slot: &DbIoTaskSlot, handle: DbIoTaskHandle) -> bool {
    slot.generation == handle.generation && slot.operation == handle.operation && handle.generation != 0 && handle.operation != 0
}

fn db_io_enqueue_close(handle: DbIoTaskHandle) -> Result<(), DbError> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !db_io_slot_matches(&owner, handle) {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    if owner.close_enqueued {
        return Ok(());
    }
    if arena.closing_len == DB_IO_OPERATION_ITEMS {
        return Err(DbError::Unavailable("DB I/O task close arena retained every admitted task".to_string()));
    }
    owner.close_enqueued = true;
    drop(owner);
    let write = (arena.closing_read + arena.closing_len) % DB_IO_OPERATION_ITEMS;
    arena.closing[write] = Some(handle);
    arena.closing_len += 1;
    Ok(())
}

fn db_io_wake(owner: &mut DbIoTaskSlot) -> Option<std::task::Waker> {
    owner.waker.take()
}

fn db_io_submit_job(handle: DbIoTaskHandle, job: Job, attempt: u8) {
    let pool = {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) {
            return;
        }
        owner.phase = DbIoTaskPhase::Queued;
        owner.pool.clone()
    };
    let Some(pool) = pool else { return };
    match pool.try_submit(Lane::Io, job) {
        Ok(()) => {}
        Err(error) => match error.kind() {
            WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated if attempt < DB_IO_RETRY_LIMIT => {
                drop(error.into_job());
                let generation = {
                    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if !db_io_slot_matches(&owner, handle) {
                        return;
                    }
                    owner.retry_attempt = Some(attempt + 1);
                    let Some(generation) = owner.retry_generation.checked_add(1).filter(|generation| *generation != 0) else {
                        owner.retry_attempt = None;
                        owner.phase = DbIoTaskPhase::Faulted;
                        owner.terminal = Some(DbIoTerminal::Fault(db_io_literal_fault(DbIoFaultKind::Saturated, DbIoFaultCause::Unavailable, "DB I/O retry generation exhausted")));
                        if let Some(waker) = db_io_wake(&mut owner) {
                            drop(owner);
                            waker.wake();
                        }
                        return;
                    };
                    owner.async_driver_scheduled = false;
                    owner.retry_generation = generation;
                    generation
                };
                pool.callback_at(pool.now_ms().saturating_add(DB_IO_RETRY_DELAY_MS), move || db_io_retry(handle, generation));
            }
            kind => {
                let job = error.into_job();
                drop(job);
                let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !db_io_slot_matches(&owner, handle) {
                    return;
                }
                owner.async_driver_scheduled = false;
                owner.phase = DbIoTaskPhase::Faulted;
                owner.terminal_submit_kind = Some(kind);
                owner.terminal = Some(DbIoTerminal::Fault(db_io_literal_fault(DbIoFaultKind::Saturated, DbIoFaultCause::Unavailable, "DB I/O WorkerPool submission failed")));
                if let Some(waker) = db_io_wake(&mut owner) {
                    drop(owner);
                    waker.wake();
                }
            }
        },
    }
}

fn db_io_retry(handle: DbIoTaskHandle, retry_generation: u64) {
    let retry = {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) || owner.retry_generation != retry_generation {
            return;
        }
        owner.retry_attempt.take()
    };
    if let Some(attempt) = retry {
        let async_driver = {
            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            db_io_slot_matches(&owner, handle) && owner.async_driver.is_some()
        };
        if async_driver {
            db_io_submit_job(handle, Box::new(move || db_io_poll_async_driver(handle)), attempt);
        } else {
            db_io_submit_job(handle, Box::new(move || db_io_drive_one(handle)), attempt);
        }
    }
}

static DB_IO_RETRY_MAINTENANCE_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn db_io_retry_maintenance_step() -> bool {
    let index = DB_IO_RETRY_MAINTENANCE_CURSOR.fetch_add(1, std::sync::atomic::Ordering::AcqRel) % DB_IO_OPERATION_ITEMS;
    let (handle, retry_generation) = {
        let owner = DB_IO_TASK_SLOTS[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if owner.generation == 0 || owner.operation == 0 || owner.retry_attempt.is_none() {
            return false;
        }
        (DbIoTaskHandle { slot: index as u16, generation: owner.generation, operation: owner.operation }, owner.retry_generation)
    };
    db_io_retry(handle, retry_generation);
    true
}

fn db_io_drive_one(handle: DbIoTaskHandle) {
    let (mut task, cancelled) = {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) {
            return;
        }
        owner.phase = DbIoTaskPhase::Executing;
        (owner.task.take(), owner.cancelled)
    };
    let Some(mut task_owner) = task.take() else { return };
    if cancelled {
        let _ = task_owner.transition_pages(DbIoPagePhase::Queued, DbIoPagePhase::TerminalResult);
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        owner.task = Some(task_owner);
        owner.phase = DbIoTaskPhase::Cancelled;
        owner.terminal = Some(DbIoTerminal::Cancelled(None));
        if let Some(waker) = db_io_wake(&mut owner) {
            drop(owner);
            waker.wake();
        }
        return;
    }
    match db_io_executor_mode(task_owner.backend()) {
        Ok(DbIoExecutorMode::AsyncNative) => {
            let transition = task_owner.transition_pages(DbIoPagePhase::Queued, DbIoPagePhase::Executing);
            let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, handle) {
                return;
            }
            owner.task = Some(task_owner);
            match transition {
                Ok(()) => {
                    owner.phase = DbIoTaskPhase::Executing;
                    owner.async_ready = true;
                }
                Err(error) => {
                    owner.phase = DbIoTaskPhase::Faulted;
                    owner.terminal = Some(DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)));
                }
            }
            if let Some(waker) = db_io_wake(&mut owner) {
                drop(owner);
                waker.wake();
            }
            return;
        }
        Ok(DbIoExecutorMode::BlockingLane) => {}
        Err(error) => {
            let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            owner.task = Some(task_owner);
            owner.phase = DbIoTaskPhase::Faulted;
            owner.terminal = Some(DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)));
            if let Some(waker) = db_io_wake(&mut owner) {
                drop(owner);
                waker.wake();
            }
            return;
        }
    }
    let mut panicked = false;
    let execution =
        task_owner.transition_pages(DbIoPagePhase::Queued, DbIoPagePhase::Executing).and_then(|()| match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| db_io_executor_execute(task_owner.backend(), handle.operation, &mut task_owner))) {
            Ok(result) => result,
            Err(_) => {
                panicked = true;
                Err(DbError::Internal("DB I/O backend panicked".to_string()))
            }
        });
    let execution = match execution {
        Ok((DbIoExecutionStep::Yield, None)) => task_owner.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::Queued).map(|()| (DbIoExecutionStep::Yield, None)),
        terminal => {
            let transition = task_owner.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
            match (terminal, transition) {
                (Ok(step), Ok(())) => Ok(step),
                (Err(error), _) | (_, Err(error)) => Err(error),
            }
        }
    };
    let mut resubmit = false;
    let mut waker = None;
    {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) {
            return;
        }
        owner.task = Some(task_owner);
        match execution {
            Ok((DbIoExecutionStep::Yield, None)) => {
                owner.phase = DbIoTaskPhase::Queued;
                resubmit = true;
            }
            Ok((DbIoExecutionStep::Complete, Some(result))) => {
                owner.phase = if owner.cancelled { DbIoTaskPhase::Cancelled } else { DbIoTaskPhase::Completed };
                owner.terminal = Some(if owner.cancelled { DbIoTerminal::Cancelled(Some(result)) } else { DbIoTerminal::Result(result) });
                waker = db_io_wake(&mut owner);
            }
            Ok(_) => {
                owner.phase = DbIoTaskPhase::Faulted;
                owner.terminal = Some(DbIoTerminal::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "DB I/O executor returned an invalid typed step")));
                waker = db_io_wake(&mut owner);
            }
            Err(error) => {
                owner.phase = DbIoTaskPhase::Faulted;
                owner.terminal = Some(DbIoTerminal::Fault(db_io_task_fault(if panicked { DbIoFaultKind::Panic } else { DbIoFaultKind::Backend }, &error)));
                waker = db_io_wake(&mut owner);
            }
        }
    }
    if let Some(waker) = waker {
        waker.wake();
    }
    if resubmit {
        db_io_submit_job(handle, Box::new(move || db_io_drive_one(handle)), 0);
    }
}

struct DbIoAsyncDriverWake {
    handle: DbIoTaskHandle,
}

impl std::task::Wake for DbIoAsyncDriverWake {
    fn wake(self: std::sync::Arc<Self>) {
        db_io_request_async_driver_poll(self.handle);
    }

    fn wake_by_ref(self: &std::sync::Arc<Self>) {
        db_io_request_async_driver_poll(self.handle);
    }
}

fn db_io_request_async_driver_poll(handle: DbIoTaskHandle) {
    let submit = {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) || owner.async_driver.is_none() {
            return;
        }
        owner.async_driver_wake_requested = true;
        if owner.async_lane_turn || owner.async_driver_scheduled {
            false
        } else {
            owner.async_driver_scheduled = true;
            true
        }
    };
    if submit {
        db_io_submit_job(handle, Box::new(move || db_io_poll_async_driver(handle)), 0);
    }
}

fn db_io_finish_async_driver(handle: DbIoTaskHandle, backend: DbIoBackendControl, executor: Box<dyn DbIoTaskExecutor>, task: DbIoTask, terminal: Result<DbIoResult, DbError>) -> Result<(), DbError> {
    db_io_return_async_executor(backend, handle.operation, executor)?;
    let transition = task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !db_io_slot_matches(&owner, handle) {
        return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
    }
    owner.async_lane_turn = false;
    owner.async_detached = false;
    owner.async_driver_scheduled = false;
    owner.async_driver_wake_requested = false;
    owner.task = Some(task);
    owner.terminal = Some(match transition {
        Err(error) => DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)),
        Ok(()) if owner.cancelled => DbIoTerminal::Cancelled(terminal.ok()),
        Ok(()) => match terminal {
            Ok(result) => DbIoTerminal::Result(result),
            Err(error) => DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)),
        },
    });
    owner.phase = match owner.terminal.as_ref() {
        Some(DbIoTerminal::Result(_)) => DbIoTaskPhase::Completed,
        Some(DbIoTerminal::Cancelled(_)) => DbIoTaskPhase::Cancelled,
        _ => DbIoTaskPhase::Faulted,
    };
    let waker = db_io_wake(&mut owner);
    drop(owner);
    db_io_operation_return(handle.operation, db_io_async_lease_credit())?;
    if let Some(waker) = waker {
        waker.wake();
    }
    Ok(())
}

fn db_io_poll_async_driver(handle: DbIoTaskHandle) {
    let (mut future, backend) = {
        let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) || owner.async_driver.is_none() {
            return;
        }
        owner.async_driver_scheduled = false;
        owner.async_driver_wake_requested = false;
        owner.async_lane_turn = true;
        let Some(backend) = owner.backend else { return };
        let Some(future) = owner.async_driver.take() else { return };
        (future, backend)
    };
    let waker = std::task::Waker::from(std::sync::Arc::new(DbIoAsyncDriverWake { handle }));
    let mut context = std::task::Context::from_waker(&waker);
    let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
    match polled {
        Ok(std::task::Poll::Ready((executor, task, terminal))) => {
            let _ = db_io_finish_async_driver(handle, backend, executor, task, terminal);
        }
        Ok(std::task::Poll::Pending) => {
            let resubmit = {
                let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !db_io_slot_matches(&owner, handle) {
                    return;
                }
                owner.async_driver = Some(future);
                owner.async_lane_turn = false;
                owner.async_driver_wake_requested
            };
            if resubmit {
                db_io_request_async_driver_poll(handle);
            }
        }
        Err(_) => {
            let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, handle) {
                return;
            }
            owner.async_driver = Some(future);
            owner.async_lane_turn = false;
            owner.cancelled = true;
            owner.phase = DbIoTaskPhase::Cancelled;
            owner.terminal = Some(DbIoTerminal::Fault(db_io_literal_fault(DbIoFaultKind::Panic, DbIoFaultCause::Internal, "DB I/O async driver panicked")));
            let waker = db_io_wake(&mut owner);
            drop(owner);
            let _ = db_io_enqueue_close(handle);
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }
}

pub struct DbIoTaskOperation {
    handle: DbIoTaskHandle,
    resolved: bool,
}

/// @emoji 🎟️ Generation-qualified terminal lease retained by its aggregate operation.
pub struct DbIoResultLease {
    handle: DbIoTaskHandle,
    result: Option<DbIoResult>,
    transferred: bool,
}

fn db_io_result_lease_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: std::mem::size_of::<DbIoResultLease>() as u64, items: 1, controls: 1 }
}

const DB_IO_LOST_OWNER_SLOTS: usize = DB_IO_TOTAL_PAGES + DB_IO_PROCESS_CONTROL_CREDIT;
const DB_IO_LOST_OWNER_OVERFLOW_SLOTS: usize = DB_IO_OPERATION_ITEMS;

enum DbIoLostOwner {
    PageWriter(DbIoPageWriter),
    Pages(DbIoPages),
    List(DbIoU64List),
    Lease(DbIoLeaseResult),
    Fault(DbIoFault),
    DriverReservation(DbIoDriverReservation),
    ExternalBytes(DbIoExternalBytes),
    ArtifactId(DbIoArtifactId),
    Backend { owner: Option<Box<dyn DbIoTaskExecutor>>, operation: u64, credit: DbIoCredit, pool: Option<Arc<WorkerPool>> },
    ResultLease { handle: DbIoTaskHandle, result: Option<DbIoResult> },
}

static DB_IO_LOST_OWNERS: std::sync::Mutex<[Option<DbIoLostOwner>; DB_IO_LOST_OWNER_SLOTS]> = std::sync::Mutex::new([const { None }; DB_IO_LOST_OWNER_SLOTS]);
static DB_IO_LOST_OWNER_OVERFLOW: std::sync::Mutex<[Option<DbIoLostOwner>; DB_IO_LOST_OWNER_OVERFLOW_SLOTS]> = std::sync::Mutex::new([const { None }; DB_IO_LOST_OWNER_OVERFLOW_SLOTS]);
static DB_IO_LOST_OWNER_QUARANTINE: std::sync::Mutex<[Option<DbIoLostOwner>; DB_IO_LOST_OWNER_OVERFLOW_SLOTS]> = std::sync::Mutex::new([const { None }; DB_IO_LOST_OWNER_OVERFLOW_SLOTS]);
static DB_IO_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn db_io_try_park_lost_owner(owner: DbIoLostOwner) -> Result<(), DbIoLostOwner> {
    let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(slot) = owners.iter_mut().find(|slot| slot.is_none()) {
        *slot = Some(owner);
        return Ok(());
    }
    Err(owner)
}

fn db_io_park_lost_owner(owner: DbIoLostOwner) -> Result<(), DbIoLostOwner> {
    if let Err(owner) = db_io_try_park_lost_owner(owner) {
        DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(slot) = overflow.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = quarantine.iter_mut().find(|slot| slot.is_none()) else { return Err(owner) };
        *slot = Some(owner);
    }
    Ok(())
}

fn db_io_lost_owner_close_step() -> Result<bool, DbError> {
    let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(index) = owners.iter().position(Option::is_some) else {
        drop(owners);
        let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = owners.iter_mut().find(|slot| slot.is_none()) else {
                drop(owners);
                let terminal = db_io_lost_owner_close_opportunity(quarantine[index].as_mut().ok_or_else(|| DbError::Internal("DB I/O quarantine retirement changed exact owner".to_string()))?)?;
                if terminal {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            *slot = quarantine[index].take();
            return Ok(true);
        };
        let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = owners.iter_mut().find(|slot| slot.is_none()) else {
            drop(owners);
            let terminal = db_io_lost_owner_close_opportunity(overflow[index].as_mut().ok_or_else(|| DbError::Internal("DB I/O overflow retirement changed exact owner".to_string()))?)?;
            if terminal {
                overflow[index] = None;
            }
            return Ok(true);
        };
        *slot = overflow[index].take();
        return Ok(true);
    };
    let terminal = db_io_lost_owner_close_opportunity(owners[index].as_mut().ok_or_else(|| DbError::Internal("DB I/O lost-owner cursor changed owner".to_string()))?)?;
    if terminal {
        owners[index] = None;
    }
    Ok(true)
}

fn db_io_lost_owner_close_opportunity(owner: &mut DbIoLostOwner) -> Result<bool, DbError> {
    let terminal = match owner {
        DbIoLostOwner::PageWriter(owner) => owner.close_step()?.is_none(),
        DbIoLostOwner::Pages(owner) => owner.close_step()?.is_none(),
        DbIoLostOwner::List(owner) => {
            let _ = owner.close_step();
            owner.terminal_is_empty()
        }
        DbIoLostOwner::Lease(owner) => {
            let _ = owner.resource.close_step() || owner.holder.close_step() || owner.handback_step();
            owner.terminal_is_empty()
        }
        DbIoLostOwner::Fault(owner) => {
            let _ = owner.close_step();
            owner.detail.terminal_is_empty() && owner.result_handback.is_none()
        }
        DbIoLostOwner::DriverReservation(owner) => {
            owner.close_step()?;
            true
        }
        DbIoLostOwner::ExternalBytes(owner) => !owner.close_step(),
        DbIoLostOwner::ArtifactId(owner) => {
            let _ = owner.close_step()?;
            owner.terminal_is_empty()
        }
        DbIoLostOwner::Backend { owner, operation, credit, pool } => {
            let backend = owner.take().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its exact executor".to_string()))?;
            let worker_pool = pool.take().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its shared WorkerPool".to_string()))?;
            match db_io_park_rejected_backend(backend, *operation, *credit, worker_pool) {
                Ok(()) => {
                    *operation = 0;
                    *credit = DbIoCredit::default();
                    true
                }
                Err((backend, worker_pool)) => {
                    *owner = Some(backend);
                    *pool = Some(worker_pool);
                    DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                    return Ok(true);
                }
            }
        }
        DbIoLostOwner::ResultLease { handle, result } => {
            if let Some(owner) = result.as_mut() {
                if owner.close_step()?.is_some() {
                    return Ok(true);
                }
                *result = None;
                return Ok(true);
            }
            db_io_result_handback(*handle)?;
            true
        }
    };
    Ok(terminal)
}

impl DbIoResultLease {
    pub fn into_result(mut self) -> Result<DbIoResult, DbError> {
        let mut result = self.result.take().ok_or_else(|| DbError::Internal("DB I/O result lease consumed twice".to_string()))?;
        if !result.attach_result_handback(self.handle) {
            db_io_result_handback(self.handle)?;
        }
        self.transferred = true;
        Ok(result)
    }
}

impl Drop for DbIoResultLease {
    fn drop(&mut self) {
        if self.transferred {
            return;
        }
        if self.result.is_some() {
            if let Err(DbIoLostOwner::ResultLease { handle, result }) = db_io_park_lost_owner(DbIoLostOwner::ResultLease { handle: self.handle, result: self.result.take() }) {
                self.handle = handle;
                self.result = result;
            }
        }
        let _ = db_io_enqueue_close(self.handle);
    }
}

fn db_io_result_handback(handle: DbIoTaskHandle) -> Result<(), DbError> {
    db_io_operation_return_result_lease(handle.operation)?;
    db_io_enqueue_close(handle)
}

/// @emoji 🌐️ Exact task/backend lease driven by an async-native platform executor after Lane::Io admission.
pub struct DbIoAsyncTaskLease {
    handle: DbIoTaskHandle,
    backend: DbIoBackendControl,
    task: Option<DbIoTask>,
    executor: Option<Box<dyn DbIoTaskExecutor>>,
    completed: bool,
    credit_returned: bool,
}

fn db_io_async_lease_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: std::mem::size_of::<DbIoAsyncTaskLease>() as u64, items: 1, controls: 1 }
}

impl DbIoAsyncTaskLease {
    pub fn operation(&self) -> u64 {
        self.handle.operation
    }

    pub fn task_mut(&mut self) -> Result<&mut DbIoTask, DbError> {
        self.task.as_mut().ok_or_else(|| DbError::Internal("DB I/O async task lease lost its typed task".to_string()))
    }

    pub fn executor_mut<T: DbIoTaskExecutor + 'static>(&mut self) -> Result<&mut T, DbError> {
        self.executor.as_mut().and_then(|executor| executor.as_any_mut().downcast_mut::<T>()).ok_or_else(|| DbError::Internal("DB I/O async executor taxonomy mismatch".to_string()))
    }

    pub fn parts_mut<T: DbIoTaskExecutor + 'static>(&mut self) -> Result<(&mut T, &mut DbIoTask), DbError> {
        let executor = self.executor.as_mut().and_then(|executor| executor.as_any_mut().downcast_mut::<T>()).ok_or_else(|| DbError::Internal("DB I/O async executor taxonomy mismatch".to_string()))?;
        let task = self.task.as_mut().ok_or_else(|| DbError::Internal("DB I/O async task lease lost task".to_string()))?;
        Ok((executor, task))
    }

    pub fn enter_lane_io_driver_turn(&self) -> Result<(), DbError> {
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) || !owner.async_detached || owner.phase != DbIoTaskPhase::Executing || owner.async_lane_turn {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
        }
        owner.async_lane_turn = true;
        Ok(())
    }

    pub fn leave_lane_io_driver_turn(&self) -> Result<(), DbError> {
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) || !owner.async_detached || !owner.async_lane_turn {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
        }
        owner.async_lane_turn = false;
        Ok(())
    }

    pub fn complete(mut self, terminal: Result<DbIoResult, DbError>) -> Result<(), DbError> {
        let executor = self.executor.take().ok_or_else(|| DbError::Internal("DB I/O async executor returned twice".to_string()))?;
        db_io_return_async_executor(self.backend, self.handle.operation, executor)?;
        let task = self.task.take().ok_or_else(|| DbError::Internal("DB I/O async task returned twice".to_string()))?;
        let transition = task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
        }
        if owner.async_lane_turn {
            return Err(DbError::Internal("DB I/O async driver completed before leaving Lane::Io authority".to_string()));
        }
        owner.async_detached = false;
        owner.task = Some(task);
        owner.terminal = Some(match transition {
            Err(error) => DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)),
            Ok(()) if owner.cancelled => DbIoTerminal::Cancelled(terminal.ok()),
            Ok(()) => match terminal {
                Ok(result) => DbIoTerminal::Result(result),
                Err(error) => DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)),
            },
        });
        owner.phase = match owner.terminal.as_ref() {
            Some(DbIoTerminal::Result(_)) => DbIoTaskPhase::Completed,
            Some(DbIoTerminal::Cancelled(_)) => DbIoTaskPhase::Cancelled,
            _ => DbIoTaskPhase::Faulted,
        };
        let waker = db_io_wake(&mut owner);
        drop(owner);
        db_io_operation_return(self.handle.operation, db_io_async_lease_credit())?;
        self.credit_returned = true;
        self.completed = true;
        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }

    pub fn start_on_lane_io(mut self) -> Result<(), DbError> {
        let executor = self.executor.take().ok_or_else(|| DbError::Internal("DB I/O async executor transferred twice".to_string()))?;
        let task = self.task.take().ok_or_else(|| DbError::Internal("DB I/O async task transferred twice".to_string()))?;
        let future = executor.drive_async(self.handle.operation, task);
        {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) || !owner.async_detached || owner.async_driver.is_some() {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
            }
            owner.async_driver = Some(future);
        }
        self.completed = true;
        self.credit_returned = true;
        db_io_request_async_driver_poll(self.handle);
        Ok(())
    }
}

impl Drop for DbIoAsyncTaskLease {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(executor) = self.executor.take() {
            let _ = db_io_return_async_executor(self.backend, self.handle.operation, executor);
        }
        if let Some(task) = self.task.take() {
            let _ = task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if db_io_slot_matches(&owner, self.handle) {
                owner.task = Some(task);
                owner.async_lane_turn = false;
                owner.async_detached = false;
                owner.cancelled = true;
                owner.phase = DbIoTaskPhase::Cancelled;
                owner.terminal = Some(DbIoTerminal::Cancelled(None));
                if let Some(waker) = db_io_wake(&mut owner) {
                    drop(owner);
                    waker.wake();
                }
            }
        }
        if !self.credit_returned {
            if db_io_operation_return(self.handle.operation, db_io_async_lease_credit()).is_ok() {
                self.credit_returned = true;
            } else {
                DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            }
        }
    }
}

pub fn submit_db_io_task(pool: &WorkerPool, task: DbIoTask) -> Result<DbIoTaskOperation, (DbError, DbIoTask)> {
    let _ = db_io_maintenance_step();
    let handle = db_io_allocate_task(pool, task)?;
    db_io_submit_job(handle, Box::new(move || db_io_drive_one(handle)), 0);
    Ok(DbIoTaskOperation { handle, resolved: false })
}

impl DbIoTaskOperation {
    /// 🧹 Finishes one retained cleanup opportunity per poll before storage-result handoff.
    pub async fn finish(self) -> Result<DbIoResult, DbError> {
        let handle = self.handle;
        match self.await {
            Ok(lease) => {
                db_io_wait_task_retirement(handle, true).await?;
                let result = lease.into_result()?;
                let retained = {
                    let ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    db_io_operation_slot(&ledger, handle.operation).is_some_and(|index| ledger.slots[index].result_leases != 0)
                };
                if !retained { db_io_wait_task_retirement(handle, false).await?; }
                Ok(result)
            }
            Err(fault) => {
                let error = fault.into_db_error();
                db_io_wait_task_retirement(handle, false).await?;
                Err(error)
            }
        }
    }

    pub fn phase(&self) -> DbIoTaskPhase {
        let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, self.handle) {
            owner.phase
        } else {
            DbIoTaskPhase::Closing
        }
    }

    pub fn cancel(&self) -> Result<(), DbError> {
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) {
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
        }
        owner.cancelled = true;
        let mut waker = None;
        if owner.async_ready {
            if let Some(task) = owner.task.as_ref() {
                task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult)?;
            }
            owner.async_ready = false;
            owner.phase = DbIoTaskPhase::Cancelled;
            owner.terminal = Some(DbIoTerminal::Cancelled(None));
            waker = db_io_wake(&mut owner);
        }
        drop(owner);
        db_io_enqueue_close(self.handle)?;
        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }

    pub async fn take_async_native(&mut self) -> Result<DbIoAsyncTaskLease, DbError> {
        std::future::poll_fn(|context| {
            let (task, backend) = {
                let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !db_io_slot_matches(&owner, self.handle) {
                    return std::task::Poll::Ready(Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) }));
                }
                if !owner.async_ready {
                    owner.waker = Some(context.waker().clone());
                    return std::task::Poll::Pending;
                }
                owner.async_ready = false;
                owner.async_detached = true;
                let Some(backend) = owner.backend else {
                    return std::task::Poll::Ready(Err(DbError::Internal("DB I/O admitted async task lost its backend".to_string())));
                };
                let Some(task) = owner.task.take() else {
                    return std::task::Poll::Ready(Err(DbError::Internal("DB I/O async-ready slot lost its typed task".to_string())));
                };
                (task, backend)
            };
            match db_io_take_async_executor(backend, self.handle.operation) {
                Ok(executor) => match db_io_operation_add(self.handle.operation, db_io_async_lease_credit()) {
                    Ok(()) => std::task::Poll::Ready(Ok(DbIoAsyncTaskLease { handle: self.handle, backend, task: Some(task), executor: Some(executor), completed: false, credit_returned: false })),
                    Err(error) => {
                        let _ = db_io_return_async_executor(backend, self.handle.operation, executor);
                        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                        owner.task = Some(task);
                        owner.async_ready = true;
                        owner.async_detached = false;
                        std::task::Poll::Ready(Err(error))
                    }
                },
                Err(error) => {
                    let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    owner.task = Some(task);
                    owner.async_ready = true;
                    owner.async_detached = false;
                    std::task::Poll::Ready(Err(error))
                }
            }
        })
        .await
    }

    pub async fn start_async_native_on_lane_io(&mut self) -> Result<(), DbError> {
        self.take_async_native().await?.start_on_lane_io()
    }

    pub fn resume(&self) -> Result<bool, DbError> {
        let retry_generation = {
            let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
            }
            if owner.retry_attempt.is_none() {
                return Ok(false);
            }
            owner.retry_generation
        };
        db_io_retry(self.handle, retry_generation);
        Ok(true)
    }

    pub fn take(&mut self) -> Result<Option<Result<DbIoResultLease, DbIoFault>>, DbError> {
        let cancelled_result_pending = {
            let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            db_io_slot_matches(&owner, self.handle) && matches!(owner.terminal, Some(DbIoTerminal::Cancelled(Some(_))))
        };
        if cancelled_result_pending {
            db_io_enqueue_close(self.handle)?;
            return Ok(None);
        }
        {
            let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
            }
            if owner.terminal.is_none() {
                return Ok(None);
            }
        }
        db_io_enqueue_close(self.handle)?;
        db_io_operation_add_result_lease(self.handle.operation)?;
        let terminal = {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                drop(owner);
                db_io_operation_return_result_lease(self.handle.operation)?;
                let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.handle.generation), actual: crate::db_ids::GenerationId(owner.generation) });
            }
            let Some(terminal) = owner.terminal.take() else {
                drop(owner);
                db_io_operation_return_result_lease(self.handle.operation)?;
                return Err(DbError::Internal("DB I/O terminal owner changed during exact take".to_string()));
            };
            owner.phase = DbIoTaskPhase::Closing;
            terminal
        };
        self.resolved = true;
        Ok(Some(match terminal {
            DbIoTerminal::Result(result) => Ok(DbIoResultLease { handle: self.handle, result: Some(result), transferred: false }),
            DbIoTerminal::Fault(mut fault) => {
                fault.result_handback = Some(self.handle);
                Err(fault)
            }
            DbIoTerminal::Cancelled(None) => {
                let mut fault = db_io_literal_fault(DbIoFaultKind::Cancelled, DbIoFaultCause::Closed, "DB I/O task cancelled");
                fault.result_handback = Some(self.handle);
                Err(fault)
            }
            DbIoTerminal::Cancelled(Some(_)) => return Err(DbError::Internal("DB I/O cancellation result escaped retained close".to_string())),
        }))
    }
}

impl Future for DbIoTaskOperation {
    type Output = Result<DbIoResultLease, DbIoFault>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let _ = db_io_maintenance_step();
        match self.take() {
            Ok(Some(terminal)) => return std::task::Poll::Ready(terminal),
            Ok(None) => {}
            Err(error) => return std::task::Poll::Ready(Err(db_io_task_fault(DbIoFaultKind::Stale, &error))),
        }
        let terminal_published = {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return std::task::Poll::Ready(Err(db_io_literal_fault(DbIoFaultKind::Stale, DbIoFaultCause::Internal, "stale DB I/O terminal handle")));
            }
            if owner.terminal.is_some() {
                true
            } else {
                owner.waker = Some(context.waker().clone());
                false
            }
        };
        if terminal_published {
            return std::task::Poll::Ready(match self.take() {
                Ok(Some(terminal)) => terminal,
                Ok(None) => Err(db_io_literal_fault(DbIoFaultKind::Stale, DbIoFaultCause::Internal, "DB I/O terminal owner changed during poll")),
                Err(error) => Err(db_io_task_fault(DbIoFaultKind::Stale, &error)),
            });
        }
        std::task::Poll::Pending
    }
}

async fn db_io_wait_task_retirement(handle: DbIoTaskHandle, result_retained: bool) -> Result<(), DbError> {
    std::future::poll_fn(|context| {
        db_io_maintenance_step()?;
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle)
            || result_retained && owner.backend_cleanup_done && owner.task.is_none() && !owner.backend_admitted && owner.backend_to_close.is_none() {
            return std::task::Poll::Ready(Ok(()));
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }).await
}

/// @emoji 🧹 One fixed mounted DB I/O retry, page, platform or terminal-close opportunity.
pub fn db_io_maintenance_step() -> Result<bool, DbError> {
    if db_io_lost_owner_close_step()? {
        return Ok(true);
    }
    if db_io_page_maintenance_step()?.is_some() {
        return Ok(true);
    }
    if db_io_platform_maintenance_step()? {
        return Ok(true);
    }
    if db_io_retry_maintenance_step() {
        return Ok(true);
    }
    if db_io_rejected_backend_maintenance_step()? {
        return Ok(true);
    }
    if db_io_backend_maintenance_step()? {
        return Ok(true);
    }
    Ok(db_io_task_close_step()?.is_some())
}

impl Drop for DbIoTaskOperation {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, self.handle) {
            owner.abandoned = true;
            owner.cancelled = true;
            if owner.async_ready {
                if let Some(task) = owner.task.as_ref() {
                    let _ = task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
                }
                owner.async_ready = false;
                owner.phase = DbIoTaskPhase::Cancelled;
                owner.terminal = Some(DbIoTerminal::Cancelled(None));
            }
        }
        drop(owner);
        let _ = db_io_enqueue_close(self.handle);
    }
}

static DB_IO_TASK_CLOSE_TURN: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn db_io_task_close_step() -> Result<Option<usize>, DbError> {
    let _turn = match DB_IO_TASK_CLOSE_TURN.try_lock() {
        Ok(turn) => turn,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(Some(0)),
        Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
    };
    let handle = {
        let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if arena.closing_len == 0 {
            return Ok(None);
        }
        arena.closing[arena.closing_read].ok_or_else(|| DbError::Internal("DB I/O close ring length lost its exact handle".to_string()))?
    };
    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !db_io_slot_matches(&owner, handle) {
        return Err(DbError::Internal("DB I/O close lost task ABA authority".to_string()));
    }
    if owner.async_driver.is_some() || owner.phase == DbIoTaskPhase::Executing || owner.phase == DbIoTaskPhase::Queued && owner.retry_attempt.is_none() {
        drop(owner);
        db_io_rotate_close_head(handle)?;
        return Ok(Some(0));
    }
    if !owner.abandoned && matches!(owner.terminal, Some(DbIoTerminal::Result(_) | DbIoTerminal::Fault(_))) {
        drop(owner);
        db_io_rotate_close_head(handle)?;
        return Ok(Some(0));
    }
    if !owner.backend_cleanup_done {
        let task = owner.task.as_ref().ok_or_else(|| DbError::Internal("DB I/O cleanup lost typed task owner".to_string()))?;
        if !db_io_executor_close_operation(task.backend(), handle.operation, task)? {
            drop(owner);
            db_io_rotate_close_head(handle)?;
            return Ok(Some(0));
        }
        owner.backend_cleanup_done = true;
        return Ok(Some(0));
    }
    if let Some(terminal) = owner.terminal.as_mut() {
        let step = match terminal {
            DbIoTerminal::Result(result) => result.close_step()?,
            DbIoTerminal::Fault(fault) => fault.detail.close_step().then_some(0),
            DbIoTerminal::Cancelled(result) => {
                if let Some(cancelled) = result.as_mut() {
                    if let Some(bytes) = cancelled.close_step()? {
                        return Ok(Some(bytes));
                    }
                    *result = None;
                    if let Some(waker) = db_io_wake(&mut owner) {
                        drop(owner);
                        waker.wake();
                    }
                    return Ok(Some(0));
                }
                if !owner.abandoned {
                    owner.close_enqueued = false;
                    drop(owner);
                    db_io_remove_close_head(handle)?;
                    return Ok(Some(0));
                }
                None
            }
        };
        if let Some(bytes) = step {
            return Ok(Some(bytes));
        }
        owner.terminal = None;
        return Ok(Some(0));
    }
    if let Some(task) = owner.task.as_mut() {
        if let Some(bytes) = task.close_step()? {
            return Ok(Some(bytes));
        }
        if !task.terminal_is_empty() {
            return Err(DbError::Internal("DB I/O task reported false terminal".to_string()));
        }
        owner.task = None;
        return Ok(Some(0));
    }
    if owner.retry_attempt.take().is_some() || owner.terminal_submit_kind.take().is_some() || owner.waker.take().is_some() || owner.pool.take().is_some() {
        return Ok(Some(0));
    }
    if owner.counted {
        owner.counted = false;
        BLOCKING_QUEUE.dequeued(owner.aggregate_credit.bytes);
        return Ok(Some(0));
    }
    if owner.backend_admitted {
        let backend = owner.backend.ok_or_else(|| DbError::Internal("DB I/O backend admission lost control".to_string()))?;
        db_io_backend_return_operation(backend, handle.operation)?;
        owner.backend_admitted = false;
        return Ok(Some(0));
    }
    if let Some(backend) = owner.backend_to_close {
        let (slot, generation) = db_io_backend_parts(backend);
        let backend_live = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[slot as usize].generation == generation;
        if backend_live {
            let _ = db_io_request_backend_close(backend)?;
            drop(owner);
            db_io_rotate_close_head(handle)?;
            return Ok(Some(0));
        }
        owner.backend_to_close = None;
        return Ok(Some(0));
    }
    if !owner.aggregate_returned {
        let retained = {
            let ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            db_io_operation_slot(&ledger, handle.operation).is_some_and(|index| ledger.slots[index].result_leases != 0)
        };
        if retained {
            drop(owner);
            db_io_rotate_close_head(handle)?;
            return Ok(Some(0));
        }
        db_io_operation_detach_task(handle.operation, owner.aggregate_credit)?;
        owner.aggregate_returned = true;
        return Ok(Some(0));
    }
    if !db_io_operation_terminal_is_empty(handle.operation) {
        drop(owner);
        db_io_rotate_close_head(handle)?;
        return Ok(Some(0));
    }
    *owner = DbIoTaskSlot::empty();
    drop(owner);
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let closing_read = arena.closing_read;
    let retired = arena.closing[closing_read].take();
    if retired != Some(handle) {
        return Err(DbError::Internal("DB I/O close queue changed exact handle".to_string()));
    }
    arena.closing_read = (arena.closing_read + 1) % DB_IO_OPERATION_ITEMS;
    arena.closing_len -= 1;
    let write = (arena.free_read + arena.free_len) % DB_IO_OPERATION_ITEMS;
    arena.free[write] = handle.slot;
    arena.free_len += 1;
    drop(arena);
    Ok(Some(0))
}

fn db_io_rotate_close_head(handle: DbIoTaskHandle) -> Result<(), DbError> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let closing_read = arena.closing_read;
    if arena.closing_len == 0 || arena.closing[closing_read] != Some(handle) {
        return Err(DbError::Internal("DB I/O close rotation lost exact ABA handle".to_string()));
    }
    arena.closing[closing_read] = None;
    arena.closing_read = (arena.closing_read + 1) % DB_IO_OPERATION_ITEMS;
    let write = (arena.closing_read + arena.closing_len - 1) % DB_IO_OPERATION_ITEMS;
    arena.closing[write] = Some(handle);
    Ok(())
}

fn db_io_remove_close_head(handle: DbIoTaskHandle) -> Result<(), DbError> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let closing_read = arena.closing_read;
    if arena.closing_len == 0 || arena.closing[closing_read] != Some(handle) {
        return Err(DbError::Internal("DB I/O close suspension lost exact ABA handle".to_string()));
    }
    arena.closing[closing_read] = None;
    arena.closing_read = (arena.closing_read + 1) % DB_IO_OPERATION_ITEMS;
    arena.closing_len -= 1;
    Ok(())
}
//#endregion 🔖️Limits

//#region 🔖️RetainedDbIo
const DB_IO_RETRY_LIMIT: u8 = 8;
const DB_IO_RETRY_DELAY_MS: u64 = 1;

#[doc(hidden)]
pub fn db_io_test_pool() -> Arc<WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)))).clone()
}

/// @emoji ✅️ Test-only sync/async bridge. 🚫️async: E5 executor bridge — poll-once: every future
/// this crate's own backends (`MemoryStorage`, and `FsStorage`/`DbBackend` driven by
/// `semio_framework_async::testkit::ManualRuntime`, whose `run_blocking` executes synchronously)
/// hand back is already `Ready` the instant it's first polled — there is no real async wait
/// anywhere in a unit test. This drives one to completion without needing a real executor,
/// mirroring `semio_framework_async`'s own `ManualRuntime` test helper. The crate's single such
/// bridge (R2 E5: "at most one per crate"); [`block_on_ready`] is a thin `Result`-shaped wrapper
/// over it, not a second one.
#[cfg(test)]
async fn poll_once<T>(fut: impl Future<Output = T>) -> T {
    fut.await
}

#[cfg(test)]
async fn block_on_ready<T>(fut: impl Future<Output = Result<T, DbError>>) -> Result<T, DbError> {
    poll_once(fut).await
}
//#endregion 🔖️RetainedDbIo

//#region 🔖️Capabilities
/// @emoji 🎚️ What a concrete `DbStorage` backend actually supports — negotiated once at
/// `Database::open` and folded into `DbCapabilities` alongside enabled Cargo features.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StorageCapabilities {
    /// @emoji 💾️ True iff data written through this backend survives a process restart.
    pub durable: bool,
    /// @emoji 🥇️ The strongest `DurabilityClass` this backend can actually deliver on `sync`.
    pub max_durability: DurabilityClass,
    /// @emoji 🔒️ True iff `WalStorage::sync`/equivalent can force data to physical storage.
    pub supports_fsync: bool,
    /// @emoji ✅️ True iff `CatalogStorage::cas_root`/`LeaseStorage` provide real compare-and-swap
    /// fencing (as opposed to a backend that could only ever serve a single writer).
    pub supports_cas: bool,
}
//#endregion 🔖️Capabilities

//#region 🔖️WalStorage
/// @emoji 📜️ Raw, per-document, per-segment append-only byte storage — `db_wal` frames its own
/// `.spr` records on top of what this trait stores; this trait never interprets a byte written
/// through it. A document's WAL is a sequence of segments identified by a dense `u64` index;
/// exactly one segment (the highest-index one not yet `seal`ed) is ever "active" at a time.
pub trait WalStorage: Send + Sync {
    /// @emoji 🆕️ Creates a new, empty, unsealed segment `index` for `document`. Errors
    /// `AlreadyExists` if `index` already exists for `document`.
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;

    /// @emoji ➕️ Appends `bytes` to the active segment `index`, returning the segment's new total
    /// length. Errors `NotFound` if the segment doesn't exist, `InvalidArgument` if it is sealed.
    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError>;

    /// @emoji 🔒️ Forces everything appended to segment `index` so far to the durability level
    /// implied by `class` — a no-op for `Memory`/`Os` (per `DurabilityClass`'s own
    /// doc: `Os` only promises "handed to the OS", not `fsync`ed), a real flush-to-disk for
    /// `Fsync`/`Quorum` (replication itself is `db_cluster`'s concern, not this trait's).
    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError>;

    /// @emoji 🏁️ Marks segment `index` sealed: no further `append`/`truncate_tail` may target it.
    /// Errors `NotFound` if the segment doesn't exist. Idempotent if already sealed.
    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;

    /// @emoji 📖️ Reads `range` of segment `index`'s bytes. Errors `NotFound` if the segment
    /// doesn't exist, `InvalidArgument` if `range` extends past the segment's current length.
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError>;

    /// @emoji 📏️ The current length in bytes of segment `index`.
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError>;

    /// @emoji 📋️ Every segment index that exists for `document`, ascending. Empty (not an error)
    /// if `document` has no WAL yet.
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// @emoji ✂️ Truncates the ACTIVE (unsealed) segment `index` down to `new_len` bytes — the
    /// crash-recovery primitive for discarding a torn/uncommitted tail write. Errors
    /// `InvalidArgument` if the segment is sealed or if `new_len` exceeds its current length
    /// (this trait never extends a segment via truncation).
    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError>;

    /// @emoji 🗑️ Deletes segment `index` entirely (both its bytes and seal marker), e.g. after
    /// `db_compact` has folded it into a later generation. Idempotent if already absent.
    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;
}
//#endregion 🔖️WalStorage

//#region 🔖️SnapshotStorage
/// @emoji 📸️ Storage for whole snapshot generations — `db_snapshot` hands this trait complete
/// `.spk` pack-file bytes (pages in `KIND_CHUNK`, descriptor in `KIND_SNAPSHOT`) per generation;
/// this trait never parses them, it only persists and retrieves them by `(document, generation)`.
pub trait SnapshotStorage: Send + Sync {
    /// @emoji ✍️ Durably writes `bytes` as generation `generation` of `document`'s snapshot
    /// history. Overwrites if the same `(document, generation)` is written twice (the caller's
    /// responsibility to pick a fresh generation number per the contract's
    /// `Footer.prev_footer_offset` incremental-generation chain).
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError>;

    /// @emoji 📖️ Reads generation `generation`'s complete bytes. Errors `NotFound` if absent.
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError>;

    /// @emoji 🥇️ The highest generation number stored for `document`, or `None` if it has no
    /// snapshot yet.
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError>;

    /// @emoji 📋️ Every generation number stored for `document`, ascending.
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// @emoji 🗑️ Deletes generation `generation`, e.g. once `db_compact`'s retention policy
    /// supersedes it. Idempotent if already absent.
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError>;
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
/// @emoji 🫙️ Content-addressed blob storage (blake3 CAS), shared across every document — large
/// command/payload bytes referenced from a WAL record or a snapshot page are stored once here and
/// referenced by `ContentHash` everywhere else, so identical payloads never duplicate on disk.
pub trait PayloadStorage: Send + Sync {
    /// @emoji ➕️ Stores `bytes` (if not already present — `put` is idempotent under content
    /// equality) and returns its `blake3` `ContentHash`.
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError>;

    /// @emoji 📖️ Reads the payload stored under `hash`. Errors `NotFound` if absent.
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError>;

    /// @emoji ❓️ True iff a payload is stored under `hash`, without reading it.
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError>;

    /// @emoji 🗑️ Deletes the payload stored under `hash` — `db_compact`'s ref-traced payload GC.
    /// Idempotent if already absent.
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError>;

    /// @emoji 📏️ The byte length of the payload stored under `hash`, without reading it. Errors
    /// `NotFound` if absent.
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError>;
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
/// @emoji 🗂️ The single catalog root blob (the family's document directory: names, ids,
/// metadata — opaque to this crate) with compare-and-swap-by-epoch writes: the split-brain gate
/// per `EpochFence`'s doc. Exactly one root exists per `DbStorage` instance.
pub trait CatalogStorage: Send + Sync {
    /// @emoji 📖️ The current root bytes and the `EpochFence` they were written under, or `None`
    /// if `cas_root` has never succeeded yet (a fresh, empty `DbStorage`).
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError>;

    /// @emoji ✅️ Compare-and-swap: succeeds only if `expected` matches the epoch of the currently
    /// stored root (or `EpochFence::INITIAL` if no root has ever been written), in which case the
    /// root becomes `new_bytes` under the next epoch (`expected.next()`), which is returned.
    /// Fails `DbError::Fenced` on any mismatch — a writer that lost leadership never silently
    /// overwrites a newer root.
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError>;
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
/// @emoji 🔍️ Storage for `db_index`'s immutable sorted runs (LSM-lite) — opaque per-document,
/// per-run byte blobs; this trait has no opinion on what's inside a run.
pub trait IndexStorage: Send + Sync {
    /// @emoji ✍️ Durably writes `bytes` as run `run_id` of `document`'s index. Overwrites if the
    /// same `(document, run_id)` is written twice.
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError>;

    /// @emoji 📖️ Reads run `run_id`'s complete bytes. Errors `NotFound` if absent.
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError>;

    /// @emoji 📋️ Every run id stored for `document`, ascending.
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// @emoji 🗑️ Deletes run `run_id`, e.g. after `db_index`'s merge policy folds it into a
    /// larger run. Idempotent if already absent.
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError>;
}
//#endregion 🔖️IndexStorage

//#region 🔖️LeaseStorage
/// @emoji ⏳️ Named, TTL'd, fenced ownership leases — `db_cluster`'s shard-ownership + epoch
/// failover primitive. A lease is keyed by an opaque `resource` string (e.g. a shard id); at most
/// one `holder` may hold a given resource's lease at a time, and every successful hand-off (a
/// fresh `acquire` after the previous holder's lease expired) bumps the resource's `EpochFence` so
/// a stale former holder's writes are fenced out by `CatalogStorage`-style checks downstream.
pub trait LeaseStorage: Send + Sync {
    /// @emoji 🤝️ Acquires (or idempotently re-acquires, if `holder` already holds an unexpired
    /// lease on `resource`) `resource` for `holder`, valid until `now_ms + ttl_ms`. Returns the
    /// resource's current `EpochFence` — unchanged on re-acquire by the same holder, bumped
    /// (`.next()`) on a genuine hand-off from an expired or absent lease. Errors `Conflict` if
    /// another holder's lease on `resource` has not yet expired.
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError>;

    /// @emoji ♻️ Extends `holder`'s existing, unexpired lease on `resource` to `now_ms + ttl_ms`.
    /// `fence` must match the lease's current `EpochFence` (`DbError::Fenced` otherwise) and
    /// `holder` must match the current holder (`DbError::Unauthorized` otherwise). Errors
    /// `NotFound`/`Unavailable` if no lease (or an already-expired one) exists on `resource`.
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError>;

    /// @emoji 🕊️ Voluntarily releases `holder`'s lease on `resource` early (`fence` and `holder`
    /// must match, same rules as `renew`), immediately freeing `resource` for another `acquire`
    /// (which will still bump the epoch, since a hand-off occurred).
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError>;

    /// @emoji 👀️ The current unexpired lease on `resource` as of `now_ms`, or `None` if unheld or
    /// expired (an expired lease is reported as absent, never as stale data).
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError>;
}
//#endregion 🔖️LeaseStorage

//#region 🔖️DbBackend
/// @emoji 🧰️ The umbrella storage substrate handle `db_engine`/the `db` facade hold as
/// `Arc<DbBackend>` (selected at `Database::open`, never compile-time-only per the contract).
/// Replaces the old `Arc<dyn DbStorage>` seam per ruling **O1** (drop dyn dispatch): every
/// former `&dyn WalStorage`/etc. accessor becomes a concrete facet-ref enum
/// ([`WalRef`]/[`SnapshotRef`]/[`PayloadRef`]/[`CatalogRef`]/[`IndexRef`]/[`LeaseRef`]) instead —
/// `Send`-ness is derived STRUCTURALLY at every spawn site (ruling **R3**) — never via a `+ Send`
/// bound on a trait method. No generic `R: HostAsyncRuntime` parameter (Phase 1 of
/// `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR` deleted it): `Fs`/`Sqlite`/`Fault` never used `R`
/// for anything but the removed `run_blocking` bridge, now [`FsStorage`]/`SqliteStorage`'s own
/// strong process `Arc<WorkerPool>` field.
pub enum DbBackend {
    Memory(MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(crate::db_storage_neo4j::Neo4jStorage),
    Fault(Box<crate::db_testkit::FaultStorage>),
}

impl DbBackend {
    /// @emoji 🔀️ This backend's [`WalRef`] facet — replaces the old `&dyn WalStorage`.
    pub async fn wal(&self) -> WalRef<'_> {
        match self {
            Self::Memory(s) => WalRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => WalRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => WalRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => WalRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => WalRef::Neo4j(s),
            Self::Fault(s) => WalRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`SnapshotRef`] facet — replaces the old `&dyn SnapshotStorage`.
    pub async fn snapshot(&self) -> SnapshotRef<'_> {
        match self {
            Self::Memory(s) => SnapshotRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => SnapshotRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => SnapshotRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => SnapshotRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => SnapshotRef::Neo4j(s),
            Self::Fault(s) => SnapshotRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`PayloadRef`] facet — replaces the old `&dyn PayloadStorage`.
    pub async fn payload(&self) -> PayloadRef<'_> {
        match self {
            Self::Memory(s) => PayloadRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => PayloadRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => PayloadRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => PayloadRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => PayloadRef::Neo4j(s),
            Self::Fault(s) => PayloadRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`CatalogRef`] facet — replaces the old `&dyn CatalogStorage`.
    pub async fn catalog(&self) -> CatalogRef<'_> {
        match self {
            Self::Memory(s) => CatalogRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => CatalogRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => CatalogRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => CatalogRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => CatalogRef::Neo4j(s),
            Self::Fault(s) => CatalogRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`IndexRef`] facet — replaces the old `&dyn IndexStorage`.
    pub async fn index(&self) -> IndexRef<'_> {
        match self {
            Self::Memory(s) => IndexRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => IndexRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => IndexRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => IndexRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => IndexRef::Neo4j(s),
            Self::Fault(s) => IndexRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`LeaseRef`] facet — replaces the old `&dyn LeaseStorage`.
    pub async fn lease(&self) -> LeaseRef<'_> {
        match self {
            Self::Memory(s) => LeaseRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => LeaseRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => LeaseRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => LeaseRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => LeaseRef::Neo4j(s),
            Self::Fault(s) => LeaseRef::Fault(&**s),
        }
    }

    /// @emoji 🎚️ What this concrete backend actually supports.
    pub async fn capabilities(&self) -> StorageCapabilities {
        match self {
            Self::Memory(s) => s.capabilities().await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.capabilities().await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.capabilities().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.capabilities().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.capabilities().await,
            // 🔀️ `FaultStorage::capabilities` calls back through `self.inner: DbBackend`
            // (`🧪️testkit`), so this arm is mutually recursive with this very fn — `Box::pin`
            // breaks the otherwise-infinitely-sized future (E0733).
            Self::Fault(s) => Box::pin(s.capabilities()).await,
        }
    }
}
//#endregion 🔖️DbBackend

//#region 🔖️WalRef
/// @emoji 🔀️ [`DbBackend::wal`]'s return shape — the enum that replaces
/// `&dyn WalStorage` per ruling **O1**.
pub enum WalRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> WalStorage for WalRef<'a> {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.create_segment(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.create_segment(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.create_segment(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.create_segment(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.create_segment(document, index).await,
            Self::Fault(s) => Box::pin(s.create_segment(document, index)).await,
        }
    }

    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.append(document, index, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.append(document, index, bytes).await,
            Self::Fault(s) => Box::pin(s.append(document, index, bytes)).await,
        }
    }

    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.sync(document, index, class).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.sync(document, index, class).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.sync(document, index, class).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.sync(document, index, class).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.sync(document, index, class).await,
            Self::Fault(s) => Box::pin(s.sync(document, index, class)).await,
        }
    }

    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.seal(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.seal(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.seal(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.seal(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.seal(document, index).await,
            Self::Fault(s) => Box::pin(s.seal(document, index)).await,
        }
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read(document, index, range).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read(document, index, range).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read(document, index, range).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read(document, index, range).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read(document, index, range).await,
            Self::Fault(s) => Box::pin(s.read(document, index, range)).await,
        }
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.segment_len(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.segment_len(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.segment_len(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.segment_len(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.segment_len(document, index).await,
            Self::Fault(s) => Box::pin(s.segment_len(document, index)).await,
        }
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_segments(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_segments(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_segments(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_segments(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_segments(document).await,
            Self::Fault(s) => Box::pin(s.list_segments(document)).await,
        }
    }

    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.truncate_tail(document, index, new_len).await,
            Self::Fault(s) => Box::pin(s.truncate_tail(document, index, new_len)).await,
        }
    }

    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_segment(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_segment(document, index).await,
            Self::Fault(s) => Box::pin(s.delete_segment(document, index)).await,
        }
    }
}
//#endregion 🔖️WalRef

//#region 🔖️SnapshotRef
/// @emoji 🔀️ [`DbBackend::snapshot`]'s return shape — the enum that replaces
/// `&dyn SnapshotStorage` per ruling **O1**.
pub enum SnapshotRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> SnapshotStorage for SnapshotRef<'a> {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.write_generation(document, generation, bytes).await,
            Self::Fault(s) => Box::pin(s.write_generation(document, generation, bytes)).await,
        }
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read_generation(document, generation).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_generation(document, generation).await,
            Self::Fault(s) => Box::pin(s.read_generation(document, generation)).await,
        }
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match self {
            Self::Memory(s) => s.latest_generation(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.latest_generation(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.latest_generation(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.latest_generation(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.latest_generation(document).await,
            Self::Fault(s) => Box::pin(s.latest_generation(document)).await,
        }
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_generations(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_generations(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_generations(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_generations(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_generations(document).await,
            Self::Fault(s) => Box::pin(s.list_generations(document)).await,
        }
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_generation(document, generation).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_generation(document, generation).await,
            Self::Fault(s) => Box::pin(s.delete_generation(document, generation)).await,
        }
    }
}
//#endregion 🔖️SnapshotRef

//#region 🔖️PayloadRef
/// @emoji 🔀️ [`DbBackend::payload`]'s return shape — the enum that replaces
/// `&dyn PayloadStorage` per ruling **O1**.
pub enum PayloadRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> PayloadStorage for PayloadRef<'a> {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match self {
            Self::Memory(s) => s.put(bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.put(bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.put(bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.put(bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.put(bytes).await,
            Self::Fault(s) => Box::pin(s.put(bytes)).await,
        }
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.get(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.get(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.get(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.get(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.get(hash).await,
            Self::Fault(s) => Box::pin(s.get(hash)).await,
        }
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match self {
            Self::Memory(s) => s.contains(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.contains(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.contains(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.contains(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.contains(hash).await,
            Self::Fault(s) => Box::pin(s.contains(hash)).await,
        }
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete(hash).await,
            Self::Fault(s) => Box::pin(s.delete(hash)).await,
        }
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.len(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.len(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.len(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.len(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.len(hash).await,
            Self::Fault(s) => Box::pin(s.len(hash)).await,
        }
    }
}
//#endregion 🔖️PayloadRef

//#region 🔖️CatalogRef
/// @emoji 🔀️ [`DbBackend::catalog`]'s return shape — the enum that replaces
/// `&dyn CatalogStorage` per ruling **O1**.
pub enum CatalogRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> CatalogStorage for CatalogRef<'a> {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match self {
            Self::Memory(s) => s.read_root().await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_root().await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_root().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_root().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_root().await,
            // 🔀️ `FaultStorage::read_root` calls back through `self.inner: DbBackend`, so this
            // arm is mutually recursive with this very fn — `Box::pin` breaks the otherwise-
            // infinitely-sized future (E0733), same as `DbBackend::capabilities`'s `Fault` arm.
            Self::Fault(s) => Box::pin(s.read_root()).await,
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match self {
            Self::Memory(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.cas_root(expected, new_bytes).await,
            Self::Fault(s) => Box::pin(s.cas_root(expected, new_bytes)).await,
        }
    }
}
//#endregion 🔖️CatalogRef

//#region 🔖️IndexRef
/// @emoji 🔀️ [`DbBackend::index`]'s return shape — the enum that replaces
/// `&dyn IndexStorage` per ruling **O1**.
pub enum IndexRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> IndexStorage for IndexRef<'a> {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.write_run(document, run_id, bytes).await,
            Self::Fault(s) => Box::pin(s.write_run(document, run_id, bytes)).await,
        }
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read_run(document, run_id).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_run(document, run_id).await,
            Self::Fault(s) => Box::pin(s.read_run(document, run_id)).await,
        }
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_runs(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_runs(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_runs(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_runs(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_runs(document).await,
            Self::Fault(s) => Box::pin(s.list_runs(document)).await,
        }
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_run(document, run_id).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_run(document, run_id).await,
            Self::Fault(s) => Box::pin(s.delete_run(document, run_id)).await,
        }
    }
}
//#endregion 🔖️IndexRef

//#region 🔖️LeaseRef
/// @emoji 🔀️ [`DbBackend::lease`]'s return shape — the enum that replaces
/// `&dyn LeaseStorage` per ruling **O1**.
pub enum LeaseRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> LeaseStorage for LeaseRef<'a> {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match self {
            Self::Memory(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            Self::Fault(s) => Box::pin(s.acquire(resource, holder, ttl_ms, now_ms)).await,
        }
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            Self::Fault(s) => Box::pin(s.renew(resource, holder, fence, ttl_ms, now_ms)).await,
        }
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.release(resource, holder, fence).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.release(resource, holder, fence).await,
            Self::Fault(s) => Box::pin(s.release(resource, holder, fence)).await,
        }
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match self {
            Self::Memory(s) => s.current(resource, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.current(resource, now_ms).await,
            Self::Fault(s) => Box::pin(s.current(resource, now_ms)).await,
        }
    }
}
//#endregion 🔖️LeaseRef

//#region 🔖️Memory
/// @emoji 🧠️ One in-process, non-durable segment of a document's WAL — `bytes` plus whether
/// `seal` has been called on it.
struct MemWalSegment {
    chunks: [Option<DbIoPages>; DB_IO_OPERATION_ITEMS],
    len: u64,
    sealed: bool,
}

impl MemWalSegment {
    fn new() -> Self {
        Self { chunks: std::array::from_fn(|_| None), len: 0, sealed: false }
    }
}

struct MemWalRangeCopy<'a> {
    segment: &'a MemWalSegment,
    writer: Option<DbIoPageWriter>,
    chunk: usize,
    page: u8,
    offset: usize,
    skip: usize,
    remaining: usize,
}

impl Future for MemWalRangeCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.remaining == 0 {
            let Some(writer) = owner.writer.as_mut() else {
                return std::task::Poll::Ready(Err(DbError::Internal("memory WAL range writer already consumed".to_string())));
            };
            return match writer.seal_retained_step()? {
                Some(pages) => {
                    owner.writer.take();
                    std::task::Poll::Ready(Ok(pages))
                }
                None => {
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            };
        }
        let Some(chunk) = owner.segment.chunks.get(owner.chunk).and_then(Option::as_ref) else {
            return std::task::Poll::Ready(Err(DbError::Corrupt("memory WAL range exceeded retained chunks".to_string())));
        };
        let Some(fragment) = chunk.page(owner.page) else {
            owner.chunk += 1;
            owner.page = 0;
            owner.offset = 0;
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        };
        if owner.skip >= fragment.len() {
            owner.skip -= fragment.len();
            owner.page += 1;
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        if owner.skip > 0 {
            owner.offset = owner.skip;
            owner.skip = 0;
        }
        let start = owner.offset;
        let available = &fragment[start..];
        let requested = available.len().min(owner.remaining);
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("memory WAL range writer was not retained".to_string())));
        };
        let written = writer.write_fragment(&available[..requested])?;
        owner.offset += written;
        owner.remaining -= written;
        if owner.offset == fragment.len() {
            owner.page += 1;
            owner.offset = 0;
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

fn mem_wal_range_copy(segment: &MemWalSegment, range: ByteRange) -> Result<MemWalRangeCopy<'_>, DbError> {
    let start = usize::try_from(range.offset).map_err(|_| DbError::InvalidArgument("memory WAL range offset exceeds usize".to_string()))?;
    let len = usize::try_from(range.len).map_err(|_| DbError::InvalidArgument("memory WAL range length exceeds usize".to_string()))?;
    let end = range.offset.checked_add(range.len).ok_or_else(|| DbError::InvalidArgument("memory WAL range overflows".to_string()))?;
    if end > segment.len {
        return Err(DbError::InvalidArgument(format!("wal read range {start}..{} out of bounds (len {})", start + len, segment.len)));
    }
    let writer = DbIoPageWriter::try_reserve(len.div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(MemWalRangeCopy { segment, writer: Some(writer), chunk: 0, page: 0, offset: 0, skip: start, remaining: len })
}

/// @emoji 🧠️ A pure in-memory `DbStorage`: every store is a `Mutex`-guarded map, nothing ever
/// touches a filesystem. Not durable (`capabilities().durable == false`) — the backend for unit
/// tests and `db_testkit`'s deterministic simulation runtime, never for a real deployment. Every
/// trait method body below is synchronous (no real I/O to await), so it is simply wrapped in an
/// already-`Ready` `{ .. }` per the module doc's "Async-first" section.
const DB_IO_MEMORY_OWNERS: usize = DB_IO_OPERATION_ITEMS;

struct MemoryWalOwner {
    document: DbIoText,
    index: u64,
    segment: MemWalSegment,
}

struct MemoryPageOwner {
    document: DbIoText,
    ordinal: u64,
    pages: DbIoPages,
}

struct MemoryPayloadOwner {
    hash: ContentHash,
    pages: DbIoPages,
}

struct MemoryLeaseOwner {
    resource: DbIoText,
    holder: DbIoText,
    fence: EpochFence,
    expires_at_ms: u64,
}

struct MemoryDbIoExecutor {
    wal: std::sync::Mutex<Box<[Option<MemoryWalOwner>]>>,
    snapshots: std::sync::Mutex<Box<[Option<MemoryPageOwner>]>>,
    payloads: std::sync::Mutex<Box<[Option<MemoryPayloadOwner>]>>,
    catalog: std::sync::Mutex<Option<(DbIoPages, EpochFence)>>,
    index_runs: std::sync::Mutex<Box<[Option<MemoryPageOwner>]>>,
    leases: std::sync::Mutex<Box<[Option<MemoryLeaseOwner>]>>,
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
    fn operation(&self) -> u64 {
        match self {
            Self::WalRead { operation, .. } | Self::PageCopy { operation, .. } | Self::List { operation, .. } | Self::PayloadHash { operation, .. } | Self::LeaseRelease { operation, .. } => *operation,
        }
    }
}

impl Default for MemoryDbIoExecutor {
    fn default() -> Self {
        Self {
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
        (std::mem::size_of::<Self>()
            + DB_IO_MEMORY_OWNERS * (std::mem::size_of::<Option<MemoryWalOwner>>() + 2 * std::mem::size_of::<Option<MemoryPageOwner>>() + std::mem::size_of::<Option<MemoryPayloadOwner>>() + std::mem::size_of::<Option<MemoryLeaseOwner>>())
            + DB_IO_OPERATION_ITEMS * (std::mem::size_of::<Option<MemoryDbIoCursor>>() + std::mem::size_of::<Option<DbIoPages>>() + std::mem::size_of::<Option<MemWalSegment>>())) as u64
    }

    fn retain_pages(&self, input: &mut DbIoPages) -> Result<DbIoPages, DbError> {
        let backend_operation = self.backing_operation.load(std::sync::atomic::Ordering::Acquire);
        if backend_operation == 0 {
            return Err(DbError::Internal("memory backend lost its backing operation".to_string()));
        }
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
            *slot = None;
            return Ok(true);
        }
        Ok(false)
    }

    /// @emoji 🧹️ One explicit memory-backend owner/page retirement opportunity.
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

/// @emoji 🩹️ Recovers a `Mutex` guard from a poisoned lock instead of panicking — a single
/// panicking mailbox/actor elsewhere in the family must not turn every other document's storage
/// access into a cascading panic.
// 🚫️async: E1 pure accessor (no suspension) — see R9
fn lock<'a, T>(mutex: &'a std::sync::Mutex<T>) -> std::sync::MutexGuard<'a, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

async fn db_io_memory_list_cursor(mut values: impl Iterator<Item = u64>) -> Result<DbIoU64List, DbError> {
    let mut output = DbIoU64List::new();
    std::future::poll_fn(|context| match values.next() {
        Some(value) => match output.push(value) {
            Ok(()) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(error) => std::task::Poll::Ready(Err(error)),
        },
        None => std::task::Poll::Ready(Ok(())),
    })
    .await?;
    Ok(output)
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
        (std::mem::size_of_val(self)
            + std::mem::size_of_val(lock(&self.wal).as_ref()) + std::mem::size_of_val(lock(&self.snapshots).as_ref())
            + std::mem::size_of_val(lock(&self.payloads).as_ref()) + std::mem::size_of_val(lock(&self.index_runs).as_ref())
            + std::mem::size_of_val(lock(&self.leases).as_ref()) + std::mem::size_of_val(lock(&self.operations).as_ref())
            + std::mem::size_of_val(lock(&self.retired_pages).as_ref()) + std::mem::size_of_val(lock(&self.retired_wal).as_ref())) as u64
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
                check_len(next_len, MAX_READ_BYTES, "memory WAL retained length")?;
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
        if let Some(cursor) = lock(&self.operations).iter_mut().find(|cursor| cursor.is_some()) {
            *cursor = None;
            return Ok(false);
        }
        MemoryDbIoExecutor::close_backend_step(self)
    }

    fn backend_terminal_is_empty(&self) -> bool {
        lock(&self.operations).iter().all(Option::is_none) && MemoryDbIoExecutor::backend_terminal_is_empty(self)
    }
}

pub struct MemoryStorage {
    control: DbIoBackendControl,
    pool: Arc<WorkerPool>,
    closed: std::sync::atomic::AtomicBool,
}

async fn memory_execute(pool: &WorkerPool, task: DbIoTask) -> Result<DbIoResult, DbError> {
    submit_db_io_task(pool, task).map_err(|(error, _)| error)?.finish().await
}

fn memory_document(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn memory_output(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("memory output bytes"))?.div_ceil(DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

impl MemoryStorage {
    pub async fn new(pool: Arc<WorkerPool>) -> Result<Self, DbError> {
        let credit = DbIoCredit { pages: 0, bytes: MemoryDbIoExecutor::backing_bytes(), items: 1, controls: 1 };
        let owner = db_io_backend_owner_reserve(credit)?;
        let executor = Box::new(MemoryDbIoExecutor::default());
        let control = register_db_io_backend_reserved(DbIoBackendKind::Memory, executor, pool.clone(), owner, credit)?;
        let storage = Self { control, pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = memory_execute(storage.pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("memory://fixed")? }).await {
            let _ = memory_execute(storage.pool.as_ref(), DbIoTask::BackendClose { backend: control }).await;
            return Err(error);
        }
        Ok(storage)
    }

    pub async fn close(&self) -> Result<(), DbError> {
        let result = match memory_execute(self.pool.as_ref(), DbIoTask::BackendClose { backend: self.control }).await? {
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
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalCreate { backend: self.control, document: memory_document(document)?, index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL create result taxonomy".to_string())),
        }
    }
    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalAppend { backend: self.control, document: memory_document(document)?, index, input: bytes }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL append result taxonomy".to_string())),
        }
    }
    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalSync { backend: self.control, document: memory_document(document)?, index, class }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL sync result taxonomy".to_string())),
        }
    }
    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalSeal { backend: self.control, document: memory_document(document)?, index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL seal result taxonomy".to_string())),
        }
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalRead { backend: self.control, document: memory_document(document)?, index, range, output: memory_output(range.len)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL read result taxonomy".to_string())),
        }
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalLength { backend: self.control, document: memory_document(document)?, index }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL length result taxonomy".to_string())),
        }
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL list result taxonomy".to_string())),
        }
    }
    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalTruncate { backend: self.control, document: memory_document(document)?, index, new_len }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL truncate result taxonomy".to_string())),
        }
    }
    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::WalDelete { backend: self.control, document: memory_document(document)?, index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL delete result taxonomy".to_string())),
        }
    }
}

impl SnapshotStorage for MemoryStorage {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::SnapshotWrite { backend: self.control, document: memory_document(document)?, generation, input: bytes }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory snapshot write result taxonomy".to_string())),
        }
    }
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::SnapshotRead { backend: self.control, document: memory_document(document)?, generation, output: memory_output(MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot read result taxonomy".to_string())),
        }
    }
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::SnapshotLatest { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::OptionalLength(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot latest result taxonomy".to_string())),
        }
    }
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::SnapshotList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot list result taxonomy".to_string())),
        }
    }
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::SnapshotDelete { backend: self.control, document: memory_document(document)?, generation }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory snapshot delete result taxonomy".to_string())),
        }
    }
}

impl PayloadStorage for MemoryStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload put result taxonomy".to_string())),
        }
    }
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: memory_output(MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload get result taxonomy".to_string())),
        }
    }
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload exists result taxonomy".to_string())),
        }
    }
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory payload delete result taxonomy".to_string())),
        }
    }
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload length result taxonomy".to_string())),
        }
    }
}

impl CatalogStorage for MemoryStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::CatalogRead { backend: self.control, output: memory_output(MAX_READ_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(value) => Ok(value),
            _ => Err(DbError::Internal("memory catalog read result taxonomy".to_string())),
        }
    }
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(value) => Ok(value),
            _ => Err(DbError::Internal("memory catalog CAS result taxonomy".to_string())),
        }
    }
}

impl IndexStorage for MemoryStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::IndexWrite { backend: self.control, document: memory_document(document)?, run_id, input: bytes }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory index write result taxonomy".to_string())),
        }
    }
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::IndexRead { backend: self.control, document: memory_document(document)?, run_id, output: memory_output(MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory index read result taxonomy".to_string())),
        }
    }
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::IndexList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory index list result taxonomy".to_string())),
        }
    }
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::IndexDelete { backend: self.control, document: memory_document(document)?, run_id }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory index delete result taxonomy".to_string())),
        }
    }
}

impl LeaseStorage for MemoryStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, ttl_ms, now_ms }).await? {
            DbIoResult::Fence(value) => Ok(value),
            _ => Err(DbError::Internal("memory lease acquire result taxonomy".to_string())),
        }
    }
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, ttl_ms, now_ms }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory lease renew result taxonomy".to_string())),
        }
    }
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory lease release result taxonomy".to_string())),
        }
    }
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match memory_execute(self.pool.as_ref(), DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
            DbIoResult::OptionalLease(value) => Ok(value),
            _ => Err(DbError::Internal("memory lease get result taxonomy".to_string())),
        }
    }
}
//#endregion 🔖️Memory

//#region 🔖️Fs
/// @emoji 📁️ The zero-touch default `DbStorage`: pure files under a root directory, no new C
/// dependency (see module doc for the wasm32/native-only gating rationale). Layout under `root`:
/// `wal/<doc>/segment-<index>.bin` (+ `.sealed` marker once sealed), `snapshot/<doc>/gen-<n>.pack`,
/// `payload/<hash[0..2]>/<hash>.bin` (blake3 CAS, two-hex-char sharding), `catalog/root.bin`
/// (8-byte LE epoch prefix + opaque bytes), `index/<doc>/run-<id>.bin`, `lease/<resource>.bin`.
/// Every multi-step write (`cas_root`, lease grants, snapshot/index/payload writes) goes through
/// `pack::write_atomic` (temp file + `fsync` + `rename`) so a reader never observes a torn file.
/// Every blocking filesystem step runs as one typed task grant on the process I/O lane.
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
mod fs_storage {
    use super::check_len;
    use super::{
        close_db_io_backend, register_db_io_backend, retire_db_io_backend, submit_db_io_task, ArtifactId, ByteRange, ContentHash, DbError, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoBackendKind, DbIoExecutionStep, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages,
        DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DurabilityClass, EpochFence, LeaseInfo, MAX_READ_BYTES,
    };
    use super::{CatalogStorage, IndexStorage, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
    use semio_framework_async::WorkerPool;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};

    /// @emoji 🚨️ Wraps a `std::io::Error` into `DbError::Io` — the only place `std::io::Error` is
    /// allowed to appear, per the contract's no-`std::io::Error`-in-public-signatures rule.
    #[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                             // 🚫️async: E4 fn-pointer slot
    fn io_err(err: std::io::Error) -> DbError {
        DbError::Io(err.to_string())
    }

    /// @emoji 🧭️ Maps a `std::io::Error` to `DbError::NotFound(missing())` when it's a missing-file
    /// error, or `DbError::Io` otherwise — used everywhere an open/read/stat is expected to find a
    /// caller-addressed blob that might legitimately not exist yet.
    // 🚫️async: E1 pure accessor called from sync `.map_err(|err| open_err(...))` closures — see R9
    fn open_err(err: std::io::Error, missing: impl FnOnce() -> String) -> DbError {
        if err.kind() == std::io::ErrorKind::NotFound {
            DbError::NotFound(missing())
        } else {
            io_err(err)
        }
    }

    /// @emoji 🛡️ Rejects a path component that could escape `root` (empty, `.`, `..`, or
    /// containing a path separator/NUL) — every document id, resource name, etc. that becomes a
    /// filesystem path component is validated through this before use.
    fn safe_component(raw: &str) -> Result<&str, DbError> {
        if raw.is_empty() || raw == "." || raw == ".." || raw.contains('/') || raw.contains('\\') || raw.contains('\0') {
            return Err(DbError::InvalidArgument(format!("unsafe storage path component: {raw:?}")));
        }
        Ok(raw)
    }

    fn wal_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("wal").join(safe_component(&document.0)?))
    }

    fn snapshot_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("snapshot").join(safe_component(&document.0)?))
    }

    fn index_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("index").join(safe_component(&document.0)?))
    }

    fn payload_path(root: &Path, hash: &ContentHash) -> PathBuf {
        let hex = hash.to_string();
        root.join("payload").join(&hex[0..2]).join(format!("{hex}.bin"))
    }

    fn catalog_path(root: &Path) -> PathBuf {
        root.join("catalog").join("root.bin")
    }

    fn lease_path(root: &Path, resource: &str) -> Result<PathBuf, DbError> {
        Ok(root.join("lease").join(format!("{}.bin", safe_component(resource)?)))
    }

    fn segment_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.bin"))
    }

    fn sealed_marker_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.sealed"))
    }

    fn generation_path(dir: &Path, generation: u64) -> PathBuf {
        dir.join(format!("gen-{generation:020}.pack"))
    }

    fn run_path(dir: &Path, run_id: u64) -> PathBuf {
        dir.join(format!("run-{run_id:020}.bin"))
    }

    fn read_lease_file(path: &Path) -> Result<Option<(EpochFence, u64, DbIoText)>, DbError> {
        if !path.exists() {
            return Ok(None);
        }
        let mut file = std::fs::File::open(path).map_err(io_err)?;
        let mut header = [0u8; 18];
        file.read_exact(&mut header).map_err(io_err)?;
        let mut epoch_bytes = [0u8; 8];
        epoch_bytes.copy_from_slice(&header[..8]);
        let mut expiry_bytes = [0u8; 8];
        expiry_bytes.copy_from_slice(&header[8..16]);
        let mut holder_len_bytes = [0u8; 2];
        holder_len_bytes.copy_from_slice(&header[16..18]);
        let epoch = u64::from_le_bytes(epoch_bytes);
        let expires_at_ms = u64::from_le_bytes(expiry_bytes);
        let holder_len = usize::from(u16::from_le_bytes(holder_len_bytes));
        if holder_len > super::DB_IO_TEXT_BYTES || file.metadata().map_err(io_err)?.len() != (18 + holder_len) as u64 {
            return Err(DbError::Corrupt("lease record exceeds its fixed authority".to_string()));
        }
        let mut holder = [0u8; super::DB_IO_TEXT_BYTES];
        file.read_exact(&mut holder[..holder_len]).map_err(io_err)?;
        let holder = std::str::from_utf8(&holder[..holder_len]).map_err(|_| DbError::Corrupt("lease holder is not valid UTF-8".to_string()))?;
        Ok(Some((EpochFence { epoch }, expires_at_ms, DbIoText::try_from_str(holder)?)))
    }

    fn write_lease_file(path: &Path, fence: EpochFence, expires_at_ms: u64, holder: &str) -> Result<(), DbError> {
        let holder = DbIoText::try_from_str(holder)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(io_err)?;
        }
        let mut record = [0u8; 18 + super::DB_IO_TEXT_BYTES];
        record[..8].copy_from_slice(&fence.epoch.to_le_bytes());
        record[8..16].copy_from_slice(&expires_at_ms.to_le_bytes());
        record[16..18].copy_from_slice(&(holder.as_str().len() as u16).to_le_bytes());
        record[18..18 + holder.as_str().len()].copy_from_slice(holder.as_str().as_bytes());
        pack::write_atomic(path, &record[..18 + holder.as_str().len()])?;
        Ok(())
    }

    struct FsDbIoExecutor {
        root: DbIoText,
        catalog_lock: Mutex<()>,
        lease_lock: Mutex<()>,
        payload_hashes: [Mutex<Option<(u64, semio_framework_hash::Hasher)>>; 64],
        readers: [Mutex<Option<FsReadState>>; 64],
        backend_close_cursor: std::sync::atomic::AtomicUsize,
        backend_terminal: std::sync::atomic::AtomicBool,
    }

    struct FsReadState {
        operation: u64,
        file: std::fs::File,
        offset: u64,
        total: usize,
        fence: Option<EpochFence>,
    }

    impl FsDbIoExecutor {
        fn new(root: DbIoText) -> Self {
            Self {
                root,
                catalog_lock: Mutex::new(()),
                lease_lock: Mutex::new(()),
                payload_hashes: [const { Mutex::new(None) }; 64],
                readers: [const { Mutex::new(None) }; 64],
                backend_close_cursor: std::sync::atomic::AtomicUsize::new(0),
                backend_terminal: std::sync::atomic::AtomicBool::new(false),
            }
        }

        fn root(&self) -> &Path {
            Path::new(self.root.as_str())
        }

        fn document_dir(&self, family: &str, document: &DbIoText) -> Result<PathBuf, DbError> {
            Ok(self.root().join(family).join(safe_component(document.as_str())?))
        }

        fn read_step(&self, operation: u64, path: &Path, offset: u64, exact_len: Option<u64>, catalog: bool, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<(DbIoPages, Option<EpochFence>)>), DbError> {
            let slot = operation as usize % self.readers.len();
            let mut owner = self.readers[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if owner.as_ref().is_some_and(|reader| reader.operation != operation) {
                return Err(DbError::Unavailable("filesystem read cursor capacity exhausted".to_string()));
            }
            if owner.is_none() {
                let mut file = std::fs::File::open(path).map_err(|error| open_err(error, || format!("database I/O object {} not found", path.display())))?;
                let available = file.metadata().map_err(io_err)?.len().checked_sub(offset).ok_or_else(|| DbError::InvalidArgument("DB I/O read offset exceeds object length".to_string()))?;
                let total = exact_len.unwrap_or(available);
                check_len(total, MAX_READ_BYTES, "db_io typed filesystem read")?;
                if total > available {
                    return Err(DbError::InvalidArgument("DB I/O read range exceeds object length".to_string()));
                }
                let fence = if catalog {
                    let mut header = [0u8; 8];
                    file.read_exact(&mut header).map_err(io_err)?;
                    Some(EpochFence { epoch: u64::from_le_bytes(header) })
                } else {
                    None
                };
                *owner = Some(FsReadState { operation, file, offset, total: total as usize, fence });
            }
            let Some(reader) = owner.as_mut() else {
                return Err(DbError::Internal("filesystem read cursor lost its retained owner".to_string()));
            };
            if output.len() == reader.total {
                let fence = reader.fence;
                return match output.seal_retained_step()? {
                    Some(pages) => {
                        owner.take();
                        Ok((DbIoExecutionStep::Complete, Some((pages, fence))))
                    }
                    None => Ok((DbIoExecutionStep::Yield, None)),
                };
            }
            let mut fragment = [0u8; super::DB_IO_PAGE_BYTES];
            let fragment_len = (reader.total - output.len()).min(fragment.len());
            reader.file.seek(SeekFrom::Start(reader.offset + output.len() as u64)).map_err(io_err)?;
            reader.file.read_exact(&mut fragment[..fragment_len]).map_err(io_err)?;
            if output.write_fragment(&fragment[..fragment_len])? != fragment_len {
                return Err(DbError::Internal("DB I/O output writer accepted a partial fixed fragment".to_string()));
            }
            Ok((DbIoExecutionStep::Yield, None))
        }

        fn read_pages_step(&self, operation: u64, path: &Path, offset: u64, exact_len: Option<u64>, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            let (step, result) = self.read_step(operation, path, offset, exact_len, false, output)?;
            Ok((step, result.map(|(pages, _)| DbIoResult::Pages(pages))))
        }

        fn replace_step(&self, path: &Path, operation: u64, input: &mut DbIoPages, header: &[u8]) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            let parent = path.parent().ok_or_else(|| DbError::InvalidArgument("DB I/O replacement path has no parent".to_string()))?;
            std::fs::create_dir_all(parent).map_err(io_err)?;
            let name = path.file_name().and_then(std::ffi::OsStr::to_str).ok_or_else(|| DbError::InvalidArgument("DB I/O replacement path is not UTF-8".to_string()))?;
            let temporary = parent.join(format!(".{name}.{operation:016x}.dbio"));
            if !temporary.exists() {
                let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary).map_err(io_err)?;
                if !header.is_empty() {
                    file.write_all(header).map_err(io_err)?;
                }
            }
            if let Some(fragment) = input.page(0) {
                let fragment_len = fragment.len();
                let mut file = std::fs::OpenOptions::new().append(true).open(&temporary).map_err(io_err)?;
                file.write_all(fragment).map_err(io_err)?;
                input.advance(fragment_len)?;
                if !input.is_empty() {
                    return Ok((DbIoExecutionStep::Yield, None));
                }
            }
            let file = std::fs::OpenOptions::new().write(true).open(&temporary).map_err(io_err)?;
            file.sync_all().map_err(io_err)?;
            std::fs::rename(temporary, path).map_err(io_err)?;
            Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
        }

        fn list_step(&self, dir: &Path, prefix: &str, suffix: &str, list: &mut DbIoU64List) -> Result<bool, DbError> {
            if !dir.exists() {
                return Ok(true);
            }
            let after = list.as_slice().last().copied();
            let mut next = None;
            for entry in std::fs::read_dir(dir).map_err(io_err)? {
                let name = entry.map_err(io_err)?.file_name();
                let name = name.to_string_lossy();
                let Some(number) = name.strip_prefix(prefix).and_then(|rest| rest.strip_suffix(suffix)) else { continue };
                let Ok(value) = number.parse::<u64>() else { continue };
                if after.is_none_or(|after| value > after) && next.is_none_or(|next| value < next) {
                    next = Some(value);
                }
            }
            if let Some(value) = next {
                list.push(value)?;
                Ok(false)
            } else {
                Ok(true)
            }
        }

        fn payload_hash_step(&self, operation: u64, input: &mut DbIoPages) -> Result<Option<ContentHash>, DbError> {
            let slot = operation as usize % self.payload_hashes.len();
            let mut state = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.as_ref().is_some_and(|(owner, _)| *owner != operation) {
                return Err(DbError::Unavailable("DB I/O filesystem hash cursor capacity exhausted".to_string()));
            }
            let (_, hasher) = state.get_or_insert_with(|| (operation, semio_framework_hash::Hasher::new()));
            if let Some(fragment) = input.page(0) {
                let len = fragment.len();
                hasher.update(fragment);
                input.advance(len)?;
            }
            if !input.is_empty() {
                return Ok(None);
            }
            let Some((_, hasher)) = state.take() else {
                return Err(DbError::Internal("DB I/O payload hash cursor lost its retained owner".to_string()));
            };
            Ok(Some(ContentHash(*hasher.finalize().as_bytes())))
        }
    }

    impl DbIoTaskExecutor for FsDbIoExecutor {
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }

        fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, Err(DbError::Internal("filesystem backend has no async-native driver".to_string())))
            })
        }

        fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            match task {
                DbIoTask::BackendOpen { path, .. } => {
                    if path.as_str() != self.root.as_str() {
                        return Err(DbError::InvalidArgument("DB I/O filesystem root authority mismatch".to_string()));
                    }
                    std::fs::create_dir_all(self.root()).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalCreate { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    std::fs::create_dir_all(&dir).map_err(io_err)?;
                    let path = segment_path(&dir, *index);
                    if path.exists() {
                        return Err(DbError::AlreadyExists(format!("wal segment {index} for {} already exists", document.as_str())));
                    }
                    std::fs::File::create(path).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalAppend { document, index, input, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    if sealed_marker_path(&dir, *index).exists() {
                        return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
                    }
                    let path = segment_path(&dir, *index);
                    if let Some(fragment) = input.page(0) {
                        let len = fragment.len();
                        let mut file = std::fs::OpenOptions::new().append(true).open(&path).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?;
                        file.write_all(fragment).map_err(io_err)?;
                        input.advance(len)?;
                        if !input.is_empty() {
                            return Ok((DbIoExecutionStep::Yield, None));
                        }
                    }
                    let len = std::fs::metadata(path).map_err(io_err)?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::WalSync { document, index, class, .. } => {
                    if matches!(class, DurabilityClass::Fsync | DurabilityClass::Quorum(_)) {
                        let path = segment_path(&self.document_dir("wal", document)?, *index);
                        std::fs::OpenOptions::new().write(true).open(path).map_err(io_err)?.sync_all().map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalSeal { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    if !segment_path(&dir, *index).exists() {
                        return Err(DbError::NotFound(format!("wal segment {index} not found")));
                    }
                    std::fs::File::create(sealed_marker_path(&dir, *index)).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalRead { document, index, range, output, .. } => {
                    let path = segment_path(&self.document_dir("wal", document)?, *index);
                    self.read_pages_step(operation, &path, range.offset, Some(range.len), output)
                }
                DbIoTask::WalLength { document, index, .. } => {
                    let path = segment_path(&self.document_dir("wal", document)?, *index);
                    let len = std::fs::metadata(path).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::WalList { document, output, .. } => {
                    if self.list_step(&self.document_dir("wal", document)?, "segment-", ".bin", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::WalTruncate { document, index, new_len, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    if sealed_marker_path(&dir, *index).exists() {
                        return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
                    }
                    let path = segment_path(&dir, *index);
                    let file = std::fs::OpenOptions::new().write(true).open(path).map_err(io_err)?;
                    if *new_len > file.metadata().map_err(io_err)?.len() {
                        return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
                    }
                    file.set_len(*new_len).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalDelete { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    for path in [segment_path(&dir, *index), sealed_marker_path(&dir, *index)] {
                        if path.exists() {
                            std::fs::remove_file(path).map_err(io_err)?;
                        }
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::SnapshotWrite { document, generation, input, .. } => self.replace_step(&generation_path(&self.document_dir("snapshot", document)?, *generation), operation, input, &[]),
                DbIoTask::SnapshotRead { document, generation, output, .. } => self.read_pages_step(operation, &generation_path(&self.document_dir("snapshot", document)?, *generation), 0, None, output),
                DbIoTask::SnapshotLatest { document, output, .. } => {
                    if self.list_step(&self.document_dir("snapshot", document)?, "gen-", ".pack", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLength(output.as_slice().last().copied()))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::SnapshotList { document, output, .. } => {
                    if self.list_step(&self.document_dir("snapshot", document)?, "gen-", ".pack", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::SnapshotDelete { document, generation, .. } => {
                    let path = generation_path(&self.document_dir("snapshot", document)?, *generation);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::PayloadPut { input, .. } => {
                    let temporary = self.root().join("payload").join(format!(".{operation:016x}.dbio"));
                    if !temporary.exists() {
                        if let Some(parent) = temporary.parent() {
                            std::fs::create_dir_all(parent).map_err(io_err)?;
                        }
                        std::fs::File::create(&temporary).map_err(io_err)?;
                    }
                    if let Some(fragment) = input.page(0) {
                        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(&temporary).map_err(io_err)?;
                        file.write_all(fragment).map_err(io_err)?;
                    }
                    let Some(hash) = self.payload_hash_step(operation, input)? else { return Ok((DbIoExecutionStep::Yield, None)) };
                    let path = payload_path(self.root(), &hash);
                    if temporary.exists() {
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent).map_err(io_err)?;
                        }
                        if !path.exists() {
                            std::fs::rename(temporary, path).map_err(io_err)?;
                        } else {
                            std::fs::remove_file(temporary).map_err(io_err)?;
                        }
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Hash(hash))))
                }
                DbIoTask::PayloadGet { hash, output, .. } => self.read_pages_step(operation, &payload_path(self.root(), hash), 0, None, output),
                DbIoTask::PayloadExists { hash, .. } => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Exists(payload_path(self.root(), hash).exists())))),
                DbIoTask::PayloadLength { hash, .. } => {
                    let len = std::fs::metadata(payload_path(self.root(), hash)).map_err(|error| open_err(error, || format!("payload {hash} not found")))?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::PayloadDelete { hash, .. } => {
                    let path = payload_path(self.root(), hash);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::CatalogRead { output, .. } => {
                    let path = catalog_path(self.root());
                    if !path.exists() {
                        return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalCatalog(None))));
                    }
                    let (step, result) = self.read_step(operation, &path, 8, None, true, output)?;
                    let result = match result {
                        Some((pages, Some(fence))) => Some(DbIoResult::OptionalCatalog(Some((pages, fence)))),
                        Some((pages, None)) => {
                            drop(pages);
                            return Err(DbError::Internal("catalog read cursor lost its retained fence".to_string()));
                        }
                        None => None,
                    };
                    Ok((step, result))
                }
                DbIoTask::CatalogCas { expected, input, .. } => {
                    let _guard = self.catalog_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let current = read_root_fence(self.root())?.unwrap_or(EpochFence::INITIAL);
                    expected.check(current)?;
                    let next = expected.next();
                    let (step, result) = self.replace_step(&catalog_path(self.root()), operation, input, &next.epoch.to_le_bytes())?;
                    Ok((step, result.map(|_| DbIoResult::Fence(next))))
                }
                DbIoTask::IndexWrite { document, run_id, input, .. } => self.replace_step(&run_path(&self.document_dir("index", document)?, *run_id), operation, input, &[]),
                DbIoTask::IndexRead { document, run_id, output, .. } => self.read_pages_step(operation, &run_path(&self.document_dir("index", document)?, *run_id), 0, None, output),
                DbIoTask::IndexList { document, output, .. } => {
                    if self.list_step(&self.document_dir("index", document)?, "run-", ".bin", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::IndexDelete { document, run_id, .. } => {
                    let path = run_path(&self.document_dir("index", document)?, *run_id);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseAcquire { document, holder, now_ms, ttl_ms, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let fence = match read_lease_file(&path)? {
                        Some((fence, expires, existing)) if *now_ms < expires && existing.as_str() != holder.as_str() => return Err(DbError::Conflict("resource is leased by another holder".to_string())),
                        Some((fence, expires, _)) if *now_ms < expires => fence,
                        Some((fence, _, _)) => fence.next(),
                        None => EpochFence::INITIAL,
                    };
                    let expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("filesystem lease expiry"))?;
                    write_lease_file(&path, fence, expires_at_ms, holder.as_str())?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Fence(fence))))
                }
                DbIoTask::LeaseRenew { document, holder, fence, now_ms, ttl_ms, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let (current, expires, existing) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                    if *now_ms >= expires || existing.as_str() != holder.as_str() {
                        return Err(DbError::Unauthorized("lease owner or expiry mismatch".to_string()));
                    }
                    fence.check(current)?;
                    let expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("filesystem lease expiry"))?;
                    write_lease_file(&path, *fence, expires_at_ms, holder.as_str())?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let (current, _, existing) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                    if existing.as_str() != holder.as_str() {
                        return Err(DbError::Unauthorized("lease owner mismatch".to_string()));
                    }
                    fence.check(current)?;
                    std::fs::remove_file(path).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseGet { document, now_ms, .. } => {
                    let lease = read_lease_file(&lease_path(self.root(), document.as_str())?)?;
                    let result = match lease {
                        Some((fence, expires_at_ms, holder)) if *now_ms < expires_at_ms => Some(super::DbIoLeaseResult::new(document.clone(), holder, fence, expires_at_ms)),
                        _ => None,
                    };
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLease(result))))
                }
                DbIoTask::BackendClose { .. } => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit))),
            }
        }

        fn close_operation_step(&self, operation: u64, task: &DbIoTask) -> Result<bool, DbError> {
            let read_slot = operation as usize % self.readers.len();
            let mut reader = self.readers[read_slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if reader.as_ref().is_some_and(|owner| owner.operation == operation) {
                reader.take();
                return Ok(false);
            }
            drop(reader);
            let temporary = match task {
                DbIoTask::SnapshotWrite { document, generation, .. } => {
                    let path = generation_path(&self.document_dir("snapshot", document)?, *generation);
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::IndexWrite { document, run_id, .. } => {
                    let path = run_path(&self.document_dir("index", document)?, *run_id);
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::CatalogCas { .. } => {
                    let path = catalog_path(self.root());
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::PayloadPut { .. } => Some(self.root().join("payload").join(format!(".{operation:016x}.dbio"))),
                _ => None,
            };
            if let Some(temporary) = temporary {
                if temporary.exists() {
                    std::fs::remove_file(temporary).map_err(io_err)?;
                    return Ok(false);
                }
            }
            let slot = operation as usize % self.payload_hashes.len();
            let mut state = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.as_ref().is_some_and(|(owner, _)| *owner == operation) {
                state.take();
                return Ok(false);
            }
            Ok(true)
        }

        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            let cursor = self.backend_close_cursor.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            if cursor < self.readers.len() {
                self.readers[cursor].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                return Ok(false);
            }
            let hash = cursor - self.readers.len();
            if hash < self.payload_hashes.len() {
                self.payload_hashes[hash].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                return Ok(false);
            }
            self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
            Ok(true)
        }

        fn backend_terminal_is_empty(&self) -> bool {
            self.backend_terminal.load(std::sync::atomic::Ordering::Acquire)
                && self.readers.iter().all(|owner| owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
                && self.payload_hashes.iter().all(|owner| owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
        }
    }

    async fn execute(pool: &WorkerPool, task: DbIoTask) -> Result<DbIoResult, DbError> {
        submit_db_io_task(pool, task).map_err(|(error, _)| error)?.finish().await
    }

    fn document_text(document: &ArtifactId) -> Result<DbIoText, DbError> {
        DbIoText::try_from_str(&document.0)
    }

    fn output_writer(bytes: u64) -> Result<DbIoPageWriter, DbError> {
        let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("db_io output writer bytes"))?.div_ceil(super::DB_IO_PAGE_BYTES);
        DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
    }

    fn typed_result_fault(expected: &'static str) -> DbError {
        DbError::Internal(format!("DB I/O filesystem executor did not return {expected}"))
    }

    fn unit(result: DbIoResult) -> Result<(), DbError> {
        match result {
            DbIoResult::Unit => Ok(()),
            _ => Err(typed_result_fault("unit")),
        }
    }

    fn length(result: DbIoResult) -> Result<u64, DbError> {
        match result {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(typed_result_fault("length")),
        }
    }

    fn optional_length(result: DbIoResult) -> Result<Option<u64>, DbError> {
        match result {
            DbIoResult::OptionalLength(value) => Ok(value),
            _ => Err(typed_result_fault("optional length")),
        }
    }

    fn pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
        match result {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(typed_result_fault("pages")),
        }
    }

    fn list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
        match result {
            DbIoResult::List(value) => Ok(value),
            _ => Err(typed_result_fault("list")),
        }
    }

    /// @emoji 📁️ The zero-touch default `DbStorage` backend — see module doc for the on-disk
    /// layout. `catalog_lock`/`lease_lock` serialize this-process's compare-and-swap operations;
    /// see `CatalogStorage`/`LeaseStorage` impls below for why a bare read-verify-write over
    /// `write_atomic` isn't itself enough across OS processes (documented extension seam). `pool`
    /// is what every trait method dispatches its blocking body through (see module doc); its
    /// `Arc` is cloned into each method's `'static` blocking closure.
    pub struct FsStorage {
        control: DbIoBackendControl,
        pool: Arc<WorkerPool>,
        closed: std::sync::atomic::AtomicBool,
    }

    impl FsStorage {
        /// @emoji 🚀️ Opens (creating if absent) a `FsStorage` rooted at `root`, dispatching every
        /// subsequent trait call's blocking body through the typed task owner onto `pool`'s
        /// `Lane::Io`. The constructor's directory creation uses that same retained authority;
        /// callers never prepare the root synchronously or through a pool-less fallback.
        pub async fn open(pool: Arc<WorkerPool>, root: &Path) -> Result<Self, DbError> {
            let root = root.to_str().ok_or_else(|| DbError::InvalidArgument("filesystem storage root is not UTF-8".to_string())).and_then(DbIoText::try_from_str)?;
            let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(FsDbIoExecutor::new(root.clone())), pool.clone())?;
            let task = DbIoTask::BackendOpen { backend: control, path: root };
            if let Err(error) = execute(pool.as_ref(), task).await {
                let _ = execute(pool.as_ref(), DbIoTask::BackendClose { backend: control }).await;
                return Err(error);
            }
            Ok(Self { control, pool, closed: std::sync::atomic::AtomicBool::new(false) })
        }

        pub async fn close(&self) -> Result<(), DbError> {
            let result = match execute(self.pool.as_ref(), DbIoTask::BackendClose { backend: self.control }).await? {
                DbIoResult::Unit => Ok(()),
                _ => Err(DbError::Internal("filesystem backend close returned the wrong typed result".to_string())),
            };
            if result.is_ok() {
                close_db_io_backend(self.control).await?;
                self.closed.store(true, std::sync::atomic::Ordering::Release);
            }
            result
        }

        /// @emoji 🎚️ Always durable, `fsync`-capable, CAS-capable — the on-disk default.
        pub async fn capabilities(&self) -> StorageCapabilities {
            StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
        }
    }

    impl Drop for FsStorage {
        fn drop(&mut self) {
            if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
                let _ = retire_db_io_backend(self.control);
            }
        }
    }

    impl WalStorage for FsStorage {
        async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalCreate { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
            check_len(bytes.len() as u64, MAX_READ_BYTES, "wal_storage::append")?;
            length(execute(self.pool.as_ref(), DbIoTask::WalAppend { backend: self.control, document: document_text(document)?, index, input: bytes }).await?)
        }

        async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalSync { backend: self.control, document: document_text(document)?, index, class }).await?)
        }

        async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalSeal { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
            if let Err(err) = check_len(range.len, MAX_READ_BYTES, "wal_storage::read") {
                return { Err(err) };
            }
            let output = output_writer(range.len)?;
            pages(execute(self.pool.as_ref(), DbIoTask::WalRead { backend: self.control, document: document_text(document)?, index, range, output }).await?)
        }

        async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
            length(execute(self.pool.as_ref(), DbIoTask::WalLength { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::WalList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalTruncate { backend: self.control, document: document_text(document)?, index, new_len }).await?)
        }

        async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalDelete { backend: self.control, document: document_text(document)?, index }).await?)
        }
    }

    impl SnapshotStorage for FsStorage {
        async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
            check_len(bytes.len() as u64, MAX_READ_BYTES, "snapshot_storage::write_generation")?;
            unit(execute(self.pool.as_ref(), DbIoTask::SnapshotWrite { backend: self.control, document: document_text(document)?, generation, input: bytes }).await?)
        }

        async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::SnapshotRead { backend: self.control, document: document_text(document)?, generation, output: output_writer(MAX_READ_BYTES)? }).await?)
        }

        async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
            optional_length(execute(self.pool.as_ref(), DbIoTask::SnapshotLatest { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::SnapshotList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::SnapshotDelete { backend: self.control, document: document_text(document)?, generation }).await?)
        }
    }

    impl PayloadStorage for FsStorage {
        async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
            if let Err(err) = check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put") {
                return { Err(err) };
            }
            match execute(self.pool.as_ref(), DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
                DbIoResult::Hash(hash) => Ok(hash),
                _ => Err(typed_result_fault("content hash")),
            }
        }

        async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: output_writer(MAX_READ_BYTES)? }).await?)
        }

        async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
                DbIoResult::Exists(exists) => Ok(exists),
                _ => Err(typed_result_fault("existence")),
            }
        }

        async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
        }

        async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
            length(execute(self.pool.as_ref(), DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await?)
        }
    }

    impl CatalogStorage for FsStorage {
        async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::CatalogRead { backend: self.control, output: output_writer(MAX_READ_BYTES)? }).await? {
                DbIoResult::OptionalCatalog(root) => Ok(root),
                _ => Err(typed_result_fault("optional catalog root")),
            }
        }

        async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
            if let Err(err) = check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root") {
                return { Err(err) };
            }
            match execute(self.pool.as_ref(), DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
                DbIoResult::Fence(fence) => Ok(fence),
                _ => Err(typed_result_fault("catalog fence")),
            }
        }
    }

    /// @emoji 📖️ The blocking body behind `CatalogStorage::read_root` — factored out so
    /// `cas_root` can reuse it under `catalog_lock` without recursive task submission.
    // 🚫️async: E1 pure-shaped typed-task accessor.
    fn read_root_fence(root: &Path) -> Result<Option<EpochFence>, DbError> {
        let path = catalog_path(root);
        if !path.exists() {
            return Ok(None);
        }
        let metadata = std::fs::metadata(&path).map_err(io_err)?;
        if metadata.len() < 8 {
            return Err(DbError::Corrupt("catalog root file is shorter than its 8-byte epoch header".to_string()));
        }
        let mut epoch_bytes = [0u8; 8];
        std::fs::File::open(&path).map_err(io_err)?.read_exact(&mut epoch_bytes).map_err(io_err)?;
        Ok(Some(EpochFence { epoch: u64::from_le_bytes(epoch_bytes) }))
    }

    impl IndexStorage for FsStorage {
        async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
            check_len(bytes.len() as u64, MAX_READ_BYTES, "index_storage::write_run")?;
            unit(execute(self.pool.as_ref(), DbIoTask::IndexWrite { backend: self.control, document: document_text(document)?, run_id, input: bytes }).await?)
        }

        async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::IndexRead { backend: self.control, document: document_text(document)?, run_id, output: output_writer(MAX_READ_BYTES)? }).await?)
        }

        async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::IndexList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::IndexDelete { backend: self.control, document: document_text(document)?, run_id }).await?)
        }
    }

    impl LeaseStorage for FsStorage {
        async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
                DbIoResult::Fence(fence) => Ok(fence),
                _ => Err(typed_result_fault("lease fence")),
            }
        }

        async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
        }

        async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
        }

        async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
                DbIoResult::OptionalLease(lease) => Ok(lease),
                _ => Err(typed_result_fault("optional lease")),
            }
        }
    }
}

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
pub use fs_storage::FsStorage;
//#endregion 🔖️Fs

#[cfg(test)]
mod db_io_retained_fixtures {
    use super::*;

    static FIXTURE_LOCK: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    struct FixtureSerial;

    impl Drop for FixtureSerial {
        fn drop(&mut self) {
            FIXTURE_LOCK.store(false, std::sync::atomic::Ordering::Release);
        }
    }

    fn fixture_serial() -> FixtureSerial {
        while FIXTURE_LOCK.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            std::thread::yield_now();
        }
        FixtureSerial
    }

    struct AsyncNativeLawExecutor {
        terminal: bool,
    }

    struct BlockingFaultTaxonomyLawExecutor {
        terminal: bool,
    }

    struct AsyncFaultTaxonomyLawExecutor {
        terminal: bool,
    }

    struct BlockingFaultLawExecutor {
        panic: bool,
        terminal: bool,
    }

    struct BlockingCompleteLawExecutor {
        terminal: bool,
    }

    struct BlockingOutputLifecycleLawExecutor {
        terminal: bool,
        success_steps: Arc<std::sync::atomic::AtomicUsize>,
        cancel_steps: Arc<std::sync::atomic::AtomicUsize>,
        abandon_steps: Arc<std::sync::atomic::AtomicUsize>,
    }

    #[derive(serde::Deserialize)]
    struct PageLifecycleFixture {
        pattern_modulo: usize,
        pattern_addend: usize,
        lengths: Vec<usize>,
        fault_categories: Vec<FaultCategoryFixture>,
    }

    #[derive(Clone, Copy, serde::Deserialize)]
    #[serde(tag = "category", rename_all = "snake_case")]
    enum FaultCategoryFixture {
        Io,
        NotFound,
        AlreadyExists,
        InvalidArgument,
        Conflict,
        Fenced { expected: u64, actual: u64 },
        StaleGeneration { expected: u64, actual: u64 },
        LimitExceeded,
        Unavailable,
        Timeout,
        Corrupt,
        Closed,
        Unauthorized,
        Unimplemented,
        Internal,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum FaultCategoryOracle {
        Io,
        NotFound,
        AlreadyExists,
        InvalidArgument,
        Conflict,
        Fenced { expected: u64, actual: u64 },
        StaleGeneration { expected: u64, actual: u64 },
        LimitExceeded,
        Unavailable,
        Timeout,
        Corrupt,
        Closed,
        Unauthorized,
        Unimplemented,
        Internal,
        Other,
    }

    struct AsyncLaneProbeExecutor {
        terminal: bool,
        caller: std::thread::ThreadId,
        polled_on_worker: Arc<std::sync::atomic::AtomicBool>,
        poll_thread: Arc<std::sync::Mutex<Option<std::thread::ThreadId>>>,
        worker_role: Arc<std::sync::atomic::AtomicBool>,
        close_thread: Arc<std::sync::Mutex<Option<std::thread::ThreadId>>>,
        close_worker_role: Arc<std::sync::atomic::AtomicBool>,
    }

    impl DbIoTaskExecutor for AsyncNativeLawExecutor {
        fn mode(&self) -> DbIoExecutorMode {
            DbIoExecutorMode::AsyncNative
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            Err(DbError::Internal("async-native law executor entered blocking step".to_string()))
        }
        fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, Ok(DbIoResult::Unit))
            })
        }
        fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
            Ok(true)
        }
        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            self.terminal = true;
            Ok(true)
        }
        fn backend_terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    fn fault_fixture_error(category: FaultCategoryFixture) -> DbError {
        match category {
            FaultCategoryFixture::Io => DbError::Io("fault taxonomy fixture io".to_string()),
            FaultCategoryFixture::NotFound => DbError::NotFound("fault taxonomy fixture missing".to_string()),
            FaultCategoryFixture::AlreadyExists => DbError::AlreadyExists("fault taxonomy fixture exists".to_string()),
            FaultCategoryFixture::InvalidArgument => DbError::InvalidArgument("fault taxonomy fixture invalid".to_string()),
            FaultCategoryFixture::Conflict => DbError::Conflict("fault taxonomy fixture conflict".to_string()),
            FaultCategoryFixture::Fenced { expected, actual } => DbError::Fenced { expected, actual },
            FaultCategoryFixture::StaleGeneration { expected, actual } => DbError::StaleGeneration { expected: crate::db_ids::GenerationId(expected), actual: crate::db_ids::GenerationId(actual) },
            FaultCategoryFixture::LimitExceeded => DbError::LimitExceeded("fault taxonomy fixture limit"),
            FaultCategoryFixture::Unavailable => DbError::Unavailable("fault taxonomy fixture unavailable".to_string()),
            FaultCategoryFixture::Timeout => DbError::Timeout("fault taxonomy fixture timeout".to_string()),
            FaultCategoryFixture::Corrupt => DbError::Corrupt("fault taxonomy fixture corrupt".to_string()),
            FaultCategoryFixture::Closed => DbError::Closed,
            FaultCategoryFixture::Unauthorized => DbError::Unauthorized("fault taxonomy fixture unauthorized".to_string()),
            FaultCategoryFixture::Unimplemented => DbError::Unimplemented("fault taxonomy fixture unimplemented"),
            FaultCategoryFixture::Internal => DbError::Internal("fault taxonomy fixture internal".to_string()),
        }
    }

    fn fault_category_oracle(error: &DbError) -> FaultCategoryOracle {
        match error {
            DbError::Io(_) => FaultCategoryOracle::Io,
            DbError::NotFound(_) => FaultCategoryOracle::NotFound,
            DbError::AlreadyExists(_) => FaultCategoryOracle::AlreadyExists,
            DbError::InvalidArgument(_) => FaultCategoryOracle::InvalidArgument,
            DbError::Conflict(_) => FaultCategoryOracle::Conflict,
            DbError::Fenced { expected, actual } => FaultCategoryOracle::Fenced { expected: *expected, actual: *actual },
            DbError::StaleGeneration { expected, actual } => FaultCategoryOracle::StaleGeneration { expected: expected.0, actual: actual.0 },
            DbError::LimitExceeded(_) => FaultCategoryOracle::LimitExceeded,
            DbError::Unavailable(_) => FaultCategoryOracle::Unavailable,
            DbError::Timeout(_) => FaultCategoryOracle::Timeout,
            DbError::Corrupt(_) => FaultCategoryOracle::Corrupt,
            DbError::Closed => FaultCategoryOracle::Closed,
            DbError::Unauthorized(_) => FaultCategoryOracle::Unauthorized,
            DbError::Unimplemented(_) => FaultCategoryOracle::Unimplemented,
            DbError::Internal(_) => FaultCategoryOracle::Internal,
            _ => FaultCategoryOracle::Other,
        }
    }

    fn fault_fixture_error_for_task(task: &DbIoTask) -> DbError {
        let DbIoTask::PayloadGet { hash, .. } = task else { return DbError::Internal("fault taxonomy fixture received the wrong task".to_string()) };
        let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("🧪️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
        fixture.fault_categories.get(usize::from(hash.0[0])).copied().map(fault_fixture_error).unwrap_or_else(|| DbError::Internal("fault taxonomy fixture discriminator is out of range".to_string()))
    }

    impl DbIoTaskExecutor for BlockingFaultTaxonomyLawExecutor {
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn execute_step(&self, _operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            let DbIoTask::PayloadGet { output, .. } = task else { return Err(DbError::Internal("blocking fault taxonomy fixture received the wrong task".to_string())) };
            if output.is_empty() {
                if output.write_fragment(&[0xa1])? != 1 {
                    return Err(DbError::Internal("blocking fault taxonomy fixture lost its retained page".to_string()));
                }
                return Ok((DbIoExecutionStep::Yield, None));
            }
            Err(fault_fixture_error_for_task(task))
        }
        fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, Err(DbError::Internal("blocking fault taxonomy fixture has no async driver".to_string())))
            })
        }
        fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
            Ok(true)
        }
        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            self.terminal = true;
            Ok(true)
        }
        fn backend_terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    impl DbIoTaskExecutor for AsyncFaultTaxonomyLawExecutor {
        fn mode(&self) -> DbIoExecutorMode {
            DbIoExecutorMode::AsyncNative
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            Err(DbError::Internal("async fault taxonomy fixture entered the blocking driver".to_string()))
        }
        fn drive_async(self: Box<Self>, _operation: u64, mut task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let terminal = match &mut task {
                    DbIoTask::PayloadGet { output, .. } => match output.write_fragment(&[0xa2]) {
                        Ok(1) => Err(fault_fixture_error_for_task(&task)),
                        Ok(_) => Err(DbError::Internal("async fault taxonomy fixture lost its retained page".to_string())),
                        Err(error) => Err(error),
                    },
                    _ => Err(DbError::Internal("async fault taxonomy fixture received the wrong task".to_string())),
                };
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, terminal)
            })
        }
        fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
            Ok(true)
        }
        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            self.terminal = true;
            Ok(true)
        }
        fn backend_terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    impl DbIoTaskExecutor for AsyncLaneProbeExecutor {
        fn mode(&self) -> DbIoExecutorMode {
            DbIoExecutorMode::AsyncNative
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            Err(DbError::Internal("async lane probe entered blocking step".to_string()))
        }
        fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let thread = std::thread::current().id();
                self.polled_on_worker.store(thread != self.caller, std::sync::atomic::Ordering::Release);
                self.worker_role.store(semio_framework_trace::is_worker_thread(), std::sync::atomic::Ordering::Release);
                *self.poll_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(thread);
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, Ok(DbIoResult::Unit))
            })
        }
        fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
            Ok(true)
        }
        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            self.terminal = true;
            Ok(true)
        }
        fn backend_terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    impl DbIoTaskExecutor for BlockingFaultLawExecutor {
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            if self.panic {
                panic!("hostile DB I/O fixture panic");
            }
            Err(DbError::Internal("hostile DB I/O fixture backend fault".to_string()))
        }
        fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, Err(DbError::Internal("blocking fault fixture has no async driver".to_string())))
            })
        }
        fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
            Ok(true)
        }
        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            self.terminal = true;
            Ok(true)
        }
        fn backend_terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    impl DbIoTaskExecutor for BlockingCompleteLawExecutor {
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
        }
        fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, Err(DbError::Internal("blocking completion fixture has no async driver".to_string())))
            })
        }
        fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
            Ok(true)
        }
        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            self.terminal = true;
            Ok(true)
        }
        fn backend_terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    impl DbIoTaskExecutor for BlockingOutputLifecycleLawExecutor {
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn execute_step(&self, _operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            let DbIoTask::PayloadGet { hash, output, .. } = task else {
                return Err(DbError::Internal("output lifecycle fixture received the wrong task".to_string()));
            };
            let counter = match hash.0[0] {
                0x71 => &self.success_steps,
                0x72 => &self.cancel_steps,
                0x73 => &self.abandon_steps,
                _ => return Err(DbError::Internal("output lifecycle fixture received an unknown scenario".to_string())),
            };
            counter.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            {
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::Executing) {
                    return Err(DbError::Internal("output lifecycle fixture entered a turn outside Executing".to_string()));
                }
            }
            let total = DB_IO_PAGE_BYTES + 1;
            if output.len() < total {
                let start = output.len();
                let end = start.saturating_add(DB_IO_PAGE_BYTES).min(total);
                let fragment: Vec<u8> = (start..end).map(|index| (index % 251) as u8).collect();
                if output.write_fragment(&fragment)? != fragment.len() {
                    return Err(DbError::Internal("output lifecycle fixture lost an admitted fragment".to_string()));
                }
                return Ok((DbIoExecutionStep::Yield, None));
            }
            if hash.0[0] != 0x71 {
                return Ok((DbIoExecutionStep::Yield, None));
            }
            match output.seal_retained_step()? {
                Some(pages) => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Pages(pages)))),
                None => Ok((DbIoExecutionStep::Yield, None)),
            }
        }
        fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, Err(DbError::Internal("blocking output lifecycle fixture has no async driver".to_string())))
            })
        }
        fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
            Ok(true)
        }
        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            self.terminal = true;
            Ok(true)
        }
        fn backend_terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    fn ledger_witness() -> (DbIoCredit, usize) {
        let ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        (ledger.totals, ledger.free_len)
    }

    fn exact_fixture_result(terminal: Result<DbIoResultLease, DbIoFault>, context: &str) -> DbIoResult {
        match terminal {
            Ok(lease) => match lease.into_result() {
                Ok(result) => result,
                Err(error) => panic!("{context} result handback failed: {error}"),
            },
            Err(mut fault) => {
                while fault.close_step() {}
                panic!("{context} task faulted")
            }
        }
    }

    fn pages(bytes: &[u8]) -> DbIoPages {
        let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("fixture pages admitted");
        for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
            assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
        }
        writer.finish().unwrap()
    }

    fn drain_pages(mut pages: DbIoPages) {
        while pages.close_step().unwrap().is_some() {}
        while db_io_page_maintenance_step().unwrap().is_some() {}
    }

    async fn drain_control_tasks(control: DbIoBackendControl) {
        while DB_IO_TASK_SLOTS.iter().any(|task| task.lock().unwrap_or_else(std::sync::PoisonError::into_inner).backend == Some(control)) {
            assert!(db_io_maintenance_step().unwrap());
            std::future::poll_fn(|context| {
                context.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
    }

    #[test]
    fn db_io_fixed_page_max_plus_one_and_zero_are_exact() {
        let _serial = fixture_serial();
        let empty = pages(&[]);
        assert!(empty.is_empty());
        drain_pages(empty);
        let max = vec![0x5a; DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES];
        let retained = pages(&max);
        assert_eq!(retained.len(), max.len());
        assert_eq!(usize::from(retained.page_count()), DB_IO_OPERATION_PAGES);
        assert!(DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES + 1).is_err());
        drain_pages(retained);
    }

    #[test]
    fn db_io_artifact_rejection_is_an_internal_executor_boundary_violation() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let fault = db_io_task_fault(
            DbIoFaultKind::Backend,
            &DbError::Rejected { policy: protocol::MergePolicy::Normal, worst: protocol::Severity::Error, messages: Vec::new() },
        );
        assert_eq!(fault.cause, DbIoFaultCause::Internal);
        assert_eq!(fault.into_db_error(), DbError::Internal("DB I/O backend returned an artifact-layer rejection".to_string()));
        while db_io_maintenance_step().unwrap() {}
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_blocking_fault_preserves_exact_category_scalars_and_retires() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("🧪️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
        let pool = db_io_test_pool();
        let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingFaultTaxonomyLawExecutor { terminal: false }), pool.clone()).unwrap();

        for (index, category) in fixture.fault_categories.into_iter().enumerate() {
            let expected_error = fault_fixture_error(category);
            let expected_category = fault_category_oracle(&expected_error);
            let output = DbIoPageWriter::try_reserve(1).unwrap();
            let operation = submit_db_io_task(pool.as_ref(), DbIoTask::PayloadGet { backend: control, hash: ContentHash([u8::try_from(index).unwrap(); 32]), output }).unwrap_or_else(|(error, _)| panic!("blocking fault taxonomy admission failed: {error}"));
            let handle = operation.handle;
            let fault = match operation.await {
                Err(fault) => fault,
                Ok(_) => panic!("blocking fault taxonomy fixture returned a result"),
            };
            assert_eq!(fault.kind, DbIoFaultKind::Backend);
            {
                let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() else { panic!("blocking fault taxonomy task lost its writer") };
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
            }
            let actual_error = fault.into_db_error();
            assert_eq!(actual_error, expected_error);
            assert_eq!(fault_category_oracle(&actual_error), expected_category);
            drain_control_tasks(control).await;
        }

        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_async_native_fault_preserves_exact_category_scalars_and_retires() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("🧪️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
        let pool = db_io_test_pool();
        let control = register_db_io_backend(DbIoBackendKind::Postgres, Box::new(AsyncFaultTaxonomyLawExecutor { terminal: false }), pool.clone()).unwrap();

        for (index, category) in fixture.fault_categories.into_iter().enumerate() {
            let expected_error = fault_fixture_error(category);
            let expected_category = fault_category_oracle(&expected_error);
            let output = DbIoPageWriter::try_reserve(1).unwrap();
            let mut operation = submit_db_io_task(pool.as_ref(), DbIoTask::PayloadGet { backend: control, hash: ContentHash([u8::try_from(index).unwrap(); 32]), output }).unwrap_or_else(|(error, _)| panic!("async fault taxonomy admission failed: {error}"));
            let handle = operation.handle;
            operation.start_async_native_on_lane_io().await.unwrap();
            let fault = match operation.await {
                Err(fault) => fault,
                Ok(_) => panic!("async fault taxonomy fixture returned a result"),
            };
            assert_eq!(fault.kind, DbIoFaultKind::Backend);
            {
                let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() else { panic!("async fault taxonomy task lost its writer") };
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
            }
            let actual_error = fault.into_db_error();
            assert_eq!(actual_error, expected_error);
            assert_eq!(fault_category_oracle(&actual_error), expected_category);
            drain_control_tasks(control).await;
        }

        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_executing_output_seal_keeps_every_page_executing_until_atomic_publication() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let bytes: Vec<u8> = (0..DB_IO_PAGE_BYTES + 1).map(|index| (index % 251) as u8).collect();
        let mut writer = DbIoPageWriter::try_reserve(2).unwrap();
        assert_eq!(writer.write_fragment(&bytes).unwrap(), DB_IO_PAGE_BYTES);
        assert_eq!(writer.write_fragment(&bytes[DB_IO_PAGE_BYTES..]).unwrap(), 1);
        writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap();
        writer.transition(DbIoPagePhase::Queued, DbIoPagePhase::Executing).unwrap();

        let mut yields = 0;
        let mut published = loop {
            match writer.seal_retained_step().unwrap() {
                Some(pages) => break pages,
                None => {
                    yields += 1;
                    let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    assert!(writer.pages.iter().take(writer.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::Executing));
                    drop(arena);
                    writer.transition(DbIoPagePhase::Executing, DbIoPagePhase::Queued).unwrap();
                    writer.transition(DbIoPagePhase::Queued, DbIoPagePhase::Executing).unwrap();
                }
            }
        };

        assert!(yields > 1);
        assert_eq!(published, bytes);
        {
            let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(published.pages.iter().take(published.retained as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
        }
        while published.close_step().unwrap().is_some() {}
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_page_identity_rejects_generation_operation_and_phase_mismatches_exactly() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
        let page = writer.pages[0].as_ref().unwrap();
        let (slot_index, generation, operation) = (page.slot as usize, page.generation, page.operation);

        {
            let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot_index].generation = generation + 1;
        }
        let generation_error = writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap_err();
        {
            let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot_index].generation = generation;
        }
        assert_eq!(generation_error, DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(generation + 1) });

        {
            let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot_index].operation = operation + 1;
        }
        let operation_error = writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap_err();
        {
            let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot_index].operation = operation;
        }
        assert!(matches!(operation_error, DbError::Internal(message) if message.contains("page operation mismatch") && message.contains(&operation.to_string())));

        let phase_error = writer.transition(DbIoPagePhase::Executing, DbIoPagePhase::Queued).unwrap_err();
        assert!(matches!(phase_error, DbError::Internal(message) if message.contains("page phase mismatch") && message.contains("CheckedOutWriter")));
        while writer.close_step().unwrap().is_some() {}
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_output_task_yield_cancel_abandon_and_close_retire_exactly_once() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let pool = db_io_test_pool();
        let success_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let cancel_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let abandon_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let control = register_db_io_backend(
            DbIoBackendKind::Memory,
            Box::new(BlockingOutputLifecycleLawExecutor { terminal: false, success_steps: success_steps.clone(), cancel_steps: cancel_steps.clone(), abandon_steps: abandon_steps.clone() }),
            pool.clone(),
        )
        .unwrap();
        let expected: Vec<u8> = (0..DB_IO_PAGE_BYTES + 1).map(|index| (index % 251) as u8).collect();

        let output = DbIoPageWriter::try_reserve(2).unwrap();
        let operation = submit_db_io_task(pool.as_ref(), DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x71; 32]), output }).unwrap_or_else(|(error, _)| panic!("output lifecycle task admission failed: {error}"));
        let mut result = match exact_fixture_result(operation.await, "output lifecycle") {
            DbIoResult::Pages(pages) => pages,
            _ => panic!("output lifecycle returned the wrong typed result"),
        };
        assert!(success_steps.load(std::sync::atomic::Ordering::Acquire) > 4);
        assert_eq!(result, expected);
        while result.close_step().unwrap().is_some() {}
        drain_control_tasks(control).await;

        let output = DbIoPageWriter::try_reserve(2).unwrap();
        let operation = submit_db_io_task(pool.as_ref(), DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x72; 32]), output }).unwrap_or_else(|(error, _)| panic!("cancel lifecycle task admission failed: {error}"));
        for _ in 0..1_000_000 {
            if cancel_steps.load(std::sync::atomic::Ordering::Acquire) >= 3 {
                break;
            }
            std::thread::yield_now();
        }
        assert!(cancel_steps.load(std::sync::atomic::Ordering::Acquire) >= 3);
        let handle = operation.handle;
        operation.cancel().unwrap();
        let mut fault = match operation.await {
            Err(fault) => fault,
            Ok(_) => panic!("cancel lifecycle task published a result"),
        };
        {
            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() else { panic!("cancel lifecycle task lost its writer") };
            let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
        }
        assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
        while fault.close_step() {}
        drain_control_tasks(control).await;

        let output = DbIoPageWriter::try_reserve(2).unwrap();
        let operation = submit_db_io_task(pool.as_ref(), DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x73; 32]), output }).unwrap_or_else(|(error, _)| panic!("abandon lifecycle task admission failed: {error}"));
        for _ in 0..1_000_000 {
            if abandon_steps.load(std::sync::atomic::Ordering::Acquire) >= 3 {
                break;
            }
            std::thread::yield_now();
        }
        assert!(abandon_steps.load(std::sync::atomic::Ordering::Acquire) >= 3);
        let handle = operation.handle;
        drop(operation);
        for _ in 0..1_000_000 {
            let phase = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).phase;
            if phase == DbIoTaskPhase::Cancelled {
                break;
            }
            std::thread::yield_now();
        }
        {
            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert_eq!(owner.phase, DbIoTaskPhase::Cancelled);
            let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() else { panic!("abandon lifecycle task lost its writer") };
            let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
        }
        drain_control_tasks(control).await;

        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        assert_eq!(ledger_witness(), before);
    }

    #[cfg(all(feature = "sqlite", not(target_arch = "wasm32")))]
    #[semio_framework_async_macros::async_test]
    async fn sqlite_payload_roundtrip_obeys_the_neutral_page_lifecycle_fixture() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("🧪️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
        let storage = crate::db_storage_sqlite::SqliteStorage::open_in_memory(db_io_test_pool()).await.unwrap();

        for length in fixture.lengths {
            let bytes: Vec<u8> = (0..length).map(|index| (index % fixture.pattern_modulo + fixture.pattern_addend) as u8).collect();
            let expected_hash = ContentHash(*semio_framework_hash::hash(&bytes).as_bytes());
            let hash = storage.put(pages(&bytes)).await.unwrap();
            assert_eq!(hash, expected_hash);
            assert!(storage.contains(&hash).await.unwrap());
            assert_eq!(storage.len(&hash).await.unwrap(), length as u64);
            let fetched = storage.get(&hash).await.unwrap();
            assert_eq!(fetched, bytes);
            drain_pages(fetched);
            storage.delete(&hash).await.unwrap();
            assert!(!storage.contains(&hash).await.unwrap());
        }

        let missing = ContentHash([0xff; 32]);
        assert!(!storage.contains(&missing).await.unwrap());
        assert!(matches!(storage.get(&missing).await, Err(DbError::NotFound(_))));
        storage.close().await.unwrap();
        while db_io_maintenance_step().unwrap() {}
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_page_writer_seal_memory_sqlite_neo_state_wal_index_max_cancel_fault_drop_is_one_opportunity() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let waker = std::task::Waker::noop();
        let context = &mut std::task::Context::from_waker(waker);

        let mut writer = DbIoPageWriter::try_reserve(2).unwrap();
        assert_eq!(writer.write_fragment(&vec![0x41; DB_IO_PAGE_BYTES + 1]).unwrap(), DB_IO_PAGE_BYTES);
        assert_eq!(writer.write_fragment(&[0x42]).unwrap(), 1);
        let mut seal = Box::pin(writer.seal_retained());
        assert!(matches!(Future::poll(seal.as_mut(), context), std::task::Poll::Pending));
        drop(seal);
        while db_io_lost_owner_close_step().unwrap() {}
        assert_eq!(ledger_witness(), before);

        let mut writer = DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES).unwrap();
        assert_eq!(writer.write_fragment(&[0x43]).unwrap(), 1);
        let mut seal = Box::pin(writer.seal_retained());
        let mut pending = 0usize;
        let pages = loop {
            match Future::poll(seal.as_mut(), context) {
                std::task::Poll::Pending => pending += 1,
                std::task::Poll::Ready(Ok(pages)) => break pages,
                std::task::Poll::Ready(Err(rejected)) => panic!("retained seal faulted: {}", rejected.error()),
            }
        };
        assert!(pending > DB_IO_OPERATION_PAGES);
        drain_pages(pages);
        assert!(DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES + 1).is_err());

        let writer = DbIoPageWriter::try_reserve(1).unwrap();
        writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap();
        let mut seal = Box::pin(writer.seal_retained());
        let rejected = match Future::poll(seal.as_mut(), context) {
            std::task::Poll::Ready(Err(rejected)) => rejected,
            _ => panic!("invalid-phase seal did not retain its typed fault owner"),
        };
        let mut writer = rejected.into_writer().unwrap();
        while writer.close_step().unwrap().is_some() {}
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_one_byte_high_capacity_candidate_is_rejected_with_exact_owner() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
        let mut reservation = DbIoDriverReservation::try_reserve(operation, DB_IO_OPERATION_BYTES as usize).unwrap();
        let mut candidate = Vec::with_capacity(DB_IO_OPERATION_BYTES as usize + 1);
        candidate.push(0x5a);
        let identity = candidate.as_ptr();
        let capacity = candidate.capacity();
        let error = reservation.observe_capacity(candidate.capacity()).unwrap_err();
        assert!(matches!(error, DbError::LimitExceeded("DB I/O external driver allocation capacity")));
        assert_eq!(candidate.as_ptr(), identity);
        assert_eq!(candidate.capacity(), capacity);
        assert_eq!(candidate, [0x5a]);
        drop(candidate);
        reservation.close_step().unwrap();
        db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_artifact_and_lease_result_owners_retain_exact_incremental_handback() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
        let text = DbIoText::try_from_str("post-admission-artifact").unwrap();
        let mut artifact = DbIoArtifactId::try_from_text(operation, &text).unwrap();
        assert_eq!(artifact.as_str(), "post-admission-artifact");
        assert_eq!(artifact.as_artifact().unwrap().0, "post-admission-artifact");
        assert!(artifact.close_step().unwrap());
        drop(artifact);
        while db_io_lost_owner_close_step().unwrap() {}
        db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();

        let mut lease = DbIoLeaseResult::new(DbIoText::try_from_str("resource").unwrap(), DbIoText::try_from_str("holder").unwrap(), EpochFence::INITIAL, 10);
        assert!(lease.close_step());
        assert!(!lease.terminal_is_empty());
        drop(lease);
        assert!(db_io_lost_owner_close_step().unwrap());
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_process_and_operation_ledger_return_to_exact_prior_witness() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let owner = pages(&[0x41; DB_IO_PAGE_BYTES + 1]);
        let during = ledger_witness();
        assert_eq!(during.0.pages, before.0.pages + 2);
        assert_eq!(during.0.bytes, before.0.bytes + (2 * DB_IO_PAGE_BYTES) as u64);
        assert_eq!(during.0.items, before.0.items + 3);
        assert_eq!(during.0.controls, before.0.controls + 1);
        drain_pages(owner);
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_range_moves_the_same_page_leases_without_suffix_copy() {
        let _serial = fixture_serial();
        let bytes = vec![0x33; DB_IO_PAGE_BYTES + 3];
        let owner = pages(&bytes);
        let operation = owner.operation();
        let suffix = owner.try_range(DB_IO_PAGE_BYTES).unwrap();
        assert_eq!(suffix.operation(), operation);
        assert_eq!(suffix, [0x33; 3]);
        drain_pages(suffix);
    }

    #[test]
    fn db_io_list_capacity_plus_one_does_not_mutate_the_fixed_owner() {
        let _serial = fixture_serial();
        let mut list = DbIoU64List::new();
        for value in 0..DB_IO_LIST_ITEMS as u64 {
            list.push(value).unwrap();
        }
        assert!(list.push(DB_IO_LIST_ITEMS as u64).is_err());
        assert_eq!(list.len(), DB_IO_LIST_ITEMS);
        assert_eq!(list.as_slice().last(), Some(&((DB_IO_LIST_ITEMS - 1) as u64)));
        while list.close_step() {}
        assert!(list.terminal_is_empty());
    }

    #[test]
    fn db_io_list_keeps_exact_capacity_off_worker_stacks_and_in_the_ledger() {
        let _serial = fixture_serial();
        let mut task = DbIoTask::WalList {
            backend: DbIoBackendControl::Memory { slot: 0, generation: 1 },
            document: DbIoText::try_from_str("budget-witness").unwrap(),
            output: DbIoU64List::new(),
        };
        assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.is_none()));
        let credit = task.aggregate_credit();
        assert_eq!(credit.bytes, DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES);
        assert_eq!(credit.items, DB_IO_LIST_ITEMS * 2 + 1);
        assert!(db_io_credit_within_limits(credit, false));
        assert!(db_io_credit_within_limits(
            DbIoCredit { pages: DB_IO_OPERATION_PAGES, bytes: DB_IO_OPERATION_BYTES, items: DB_IO_OPERATION_ITEM_CREDIT, controls: DB_IO_OPERATION_CONTROL_CREDIT },
            false,
        ));
        assert!(!db_io_credit_within_limits(
            DbIoCredit { pages: DB_IO_OPERATION_PAGES, bytes: DB_IO_OPERATION_BYTES + 1, items: DB_IO_OPERATION_ITEM_CREDIT, controls: DB_IO_OPERATION_CONTROL_CREDIT },
            false,
        ));
        let before = ledger_witness();
        let operation = db_io_operation_reserve(credit).unwrap();
        let during = ledger_witness();
        assert_eq!(during.0, before.0.checked_add(credit).unwrap());
        assert_eq!(during.1 + 1, before.1);
        task.admit_list_backing().unwrap();
        assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.as_deref().map(<[u64]>::len) == Some(DB_IO_LIST_ITEMS)));
        task.release_unstarted_list_backing();
        assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.is_none()));
        task.admit_list_backing().unwrap();
        let mut source = DbIoU64List::new();
        source.push(1).unwrap();
        assert_eq!(source.values.as_deref().map(<[u64]>::len), Some(DB_IO_LIST_ITEMS));
        assert_eq!(DB_IO_LIST_TRANSIENT_BYTES, 2 * DB_IO_LIST_ITEMS as u64 * std::mem::size_of::<u64>() as u64);
        while source.close_step() {}
        db_io_operation_return(operation, credit).unwrap();
        assert_eq!(ledger_witness(), before);
        assert!(std::mem::size_of::<DbIoU64List>() <= 64);
        assert!(std::mem::size_of::<DbIoTask>() <= 4 * 1024);
        assert!(std::mem::size_of::<DbIoResult>() <= 4 * 1024);
        assert!(std::mem::size_of::<DbIoTaskSlot>() <= 8 * 1024);
        assert!(std::mem::size_of::<DbIoLostOwner>() <= 4 * 1024);
    }

    #[test]
    fn db_io_process_page_max_plus_one_preflight_is_atomic() {
        let mut state = DbIoPageArenaState::new();
        for _ in 0..DB_IO_TOTAL_PAGES / DB_IO_OPERATION_PAGES {
            db_io_preflight_page_checkout(&state, DB_IO_OPERATION_PAGES).unwrap();
            state.free_len -= DB_IO_OPERATION_PAGES;
        }
        let before = (state.free_read, state.free_len, state.next_generation);
        assert!(db_io_preflight_page_checkout(&state, 1).is_err());
        assert_eq!((state.free_read, state.free_len, state.next_generation), before);
    }

    #[test]
    fn db_io_result_page_reservation_plus_one_returns_the_writer() {
        let _serial = fixture_serial();
        let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
        assert_eq!(writer.write_fragment(&[0x44; DB_IO_PAGE_BYTES]).unwrap(), DB_IO_PAGE_BYTES);
        assert!(matches!(writer.write_fragment(&[0x55]), Err(DbError::LimitExceeded(_))));
        assert_eq!(writer.len(), DB_IO_PAGE_BYTES);
        assert!(writer.close_step().unwrap().is_some());
        assert!(writer.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_platform_fixed_ring_max_plus_one_returns_exact_capacity() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let source = pages(&[0x39]);
        let mut owners: [Option<DbIoPlatformBuffer>; DB_IO_PLATFORM_BUFFERS] = std::array::from_fn(|_| None);
        for owner in &mut owners {
            let copy = match db_io_prepare_platform(&source) {
                Ok(copy) => copy,
                Err(error) => panic!("platform max fixture reservation failed: {error}"),
            };
            *owner = Some(match copy.await {
                Ok(owner) => owner,
                Err(error) => panic!("platform max fixture copy failed: {error}"),
            });
        }
        assert!(matches!(db_io_prepare_platform(&source), Err(DbError::Unavailable(_))));
        for owner in &mut owners {
            let Some(owner) = owner.take() else { panic!("platform max fixture lost an admitted owner") };
            if let Err(error) = db_io_close_platform(owner).await {
                panic!("platform max fixture close failed: {error}");
            }
        }
        drain_pages(source);
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate() {
        let _serial = fixture_serial();
        while db_io_lost_owner_close_step().unwrap() {}
        DB_IO_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
        for _ in 0..DB_IO_LOST_OWNER_SLOTS {
            let owner = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "retained-ring-owner"));
            assert!(db_io_try_park_lost_owner(owner).is_ok());
        }
        {
            let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in overflow.iter_mut() {
                *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "retained-overflow-owner")));
            }
        }
        let owner = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-plus-one-candidate"));
        assert!(db_io_park_lost_owner(owner).is_ok());
        let second = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-plus-two-candidate"));
        assert!(db_io_park_lost_owner(second).is_ok());
        assert!(DB_IO_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
        {
            let quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let exact = quarantine.iter().flatten().find_map(|owner| match owner {
                DbIoLostOwner::Fault(candidate) => Some(candidate.detail.as_str()),
                _ => None,
            });
            assert_eq!(exact, Some("exact-plus-one-candidate"));
            assert!(quarantine.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-two-candidate")));
        }
        for _ in 0..DB_IO_LOST_OWNER_SLOTS {
            assert!(db_io_lost_owner_close_step().unwrap());
        }
        for _ in 0..DB_IO_LOST_OWNER_OVERFLOW_SLOTS * 2 {
            assert!(db_io_lost_owner_close_step().unwrap());
        }
        assert!(db_io_lost_owner_close_step().unwrap());
        {
            let owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(owners.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-one-candidate")));
        }
        assert!(db_io_lost_owner_close_step().unwrap());
        assert!(db_io_lost_owner_close_step().unwrap());
        {
            let owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(owners.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-two-candidate")));
        }
        assert!(db_io_lost_owner_close_step().unwrap());
        assert!(!db_io_lost_owner_close_step().unwrap());

        for _ in 0..DB_IO_LOST_OWNER_SLOTS {
            assert!(db_io_try_park_lost_owner(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-primary-owner"))).is_ok());
        }
        {
            let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in overflow.iter_mut() {
                *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-overflow-owner")));
            }
        }
        {
            let mut quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in quarantine.iter_mut() {
                *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-quarantine-owner")));
            }
        }
        let refused = match db_io_park_lost_owner(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-all-tier-refusal"))) {
            Err(owner) => owner,
            Ok(()) => panic!("all-tier retirement saturation accepted an unreserved owner"),
        };
        assert!(matches!(&refused, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-all-tier-refusal"));
        assert!(db_io_lost_owner_close_step().unwrap());
        assert!(db_io_park_lost_owner(refused).is_ok());
        while db_io_lost_owner_close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_storage_ready_and_pending_close_interruption_recover_the_same_owner_and_ledger() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let source = pages(&[0x61; DB_IO_PAGE_BYTES + 1]);
        let platform = db_io_prepare_platform(&source).unwrap().await.unwrap();
        let mut close = db_io_close_platform(platform);
        let waker = std::task::Waker::noop();
        let context = &mut std::task::Context::from_waker(waker);
        assert!(matches!(Pin::new(&mut close).poll(context), std::task::Poll::Pending));
        drop(close);
        assert!(db_io_platform_maintenance_step().unwrap());
        while db_io_lost_owner_close_step().unwrap() {}
        drain_pages(source);

        let empty_source = pages(&[]);
        let empty_platform = db_io_prepare_platform(&empty_source).unwrap().await.unwrap();
        let mut ready_close = db_io_close_platform(empty_platform);
        assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Pending));
        assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Pending));
        assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Ready(Ok(()))));
        drop(ready_close);
        drain_pages(empty_source);

        let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
        let reservation = DbIoDriverReservation::try_reserve(operation, DB_IO_PAGE_BYTES).unwrap();
        let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
        let mut copy = db_io_write_observed_bytes(reservation, vec![0x62; DB_IO_PAGE_BYTES], &mut writer);
        assert!(matches!(Pin::new(&mut copy).poll(context), std::task::Poll::Pending));
        assert!(matches!(Pin::new(&mut copy).poll(context), std::task::Poll::Pending));
        drop(copy);
        while db_io_lost_owner_close_step().unwrap() {}
        while writer.close_step().unwrap().is_some() {}
        db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();

        let fault_operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
        let reservation = DbIoDriverReservation::try_reserve(fault_operation, 1).unwrap();
        let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
        let mut fault = db_io_write_observed_bytes(reservation, Vec::with_capacity(DB_IO_PAGE_BYTES), &mut writer);
        assert!(matches!(Pin::new(&mut fault).poll(context), std::task::Poll::Ready(Err(DbError::Unavailable(_)))));
        drop(fault);
        while db_io_lost_owner_close_step().unwrap() {}
        while writer.close_step().unwrap().is_some() {}
        db_io_operation_return(fault_operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_interrupted_close_retires_one_page_or_owner_per_grant() {
        let _serial = fixture_serial();
        let input = pages(&[0x66; DB_IO_PAGE_BYTES + 1]);
        let mut task = DbIoTask::WalAppend { backend: DbIoBackendControl::Memory { slot: 0, generation: 1 }, document: DbIoText::try_from_str("close-fixture").unwrap(), index: 0, input };
        assert_eq!(task.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
        assert!(!task.terminal_is_empty());
        assert_eq!(task.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
        assert!(!task.terminal_is_empty());
        assert_eq!(task.close_step().unwrap(), Some(0));
        assert!(!task.terminal_is_empty());
        assert_eq!(task.close_step().unwrap(), Some(0));
        assert!(task.terminal_is_empty());
        assert_eq!(task.close_step().unwrap(), None);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn db_io_real_queued_callback_rejects_a_reused_task_slot_aba() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        pool.try_submit(
            Lane::Io,
            Box::new(move || {
                let _ = started_tx.send(());
                let _ = release_rx.recv();
            }),
        )
        .ok()
        .expect("DB I/O ABA blocker admission");
        started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();

        let task = DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://aba-old").unwrap() };
        let old = db_io_allocate_task(pool.as_ref(), task).unwrap_or_else(|(error, _)| panic!("{error}"));
        let (callback_tx, callback_rx) = std::sync::mpsc::channel();
        pool.try_submit(
            Lane::Io,
            Box::new(move || {
                db_io_drive_one(old);
                let _ = callback_tx.send(());
            }),
        )
        .ok()
        .expect("DB I/O ABA callback admission");
        db_io_drive_one(old);
        let terminal = match (DbIoTaskOperation { handle: old, resolved: false }).await {
            Ok(lease) => match lease.into_result() {
                Ok(result) => result,
                Err(error) => panic!("old ABA result handback failed: {error}"),
            },
            Err(_) => panic!("old ABA task faulted"),
        };
        assert!(matches!(terminal, DbIoResult::Unit));
        drain_control_tasks(control).await;

        let reused = loop {
            let task = DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://aba-reused").unwrap() };
            let handle = db_io_allocate_task(pool.as_ref(), task).unwrap_or_else(|(error, _)| panic!("{error}"));
            if handle.slot == old.slot {
                break handle;
            }
            db_io_drive_one(handle);
            let terminal = match (DbIoTaskOperation { handle, resolved: false }).await {
                Ok(lease) => match lease.into_result() {
                    Ok(result) => result,
                    Err(error) => panic!("ABA cycle result handback failed: {error}"),
                },
                Err(_) => panic!("ABA cycle task faulted"),
            };
            assert!(matches!(terminal, DbIoResult::Unit));
            drain_control_tasks(control).await;
        };
        let before_callback = {
            let owner = DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            (owner.generation, owner.operation, owner.phase)
        };
        release_tx.send(()).unwrap();
        callback_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        let after_callback = {
            let owner = DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            (owner.generation, owner.operation, owner.phase)
        };
        assert_eq!(before_callback, after_callback);
        assert!(db_io_slot_matches(&DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner), reused));
        db_io_drive_one(reused);
        let terminal = match (DbIoTaskOperation { handle: reused, resolved: false }).await {
            Ok(lease) => match lease.into_result() {
                Ok(result) => result,
                Err(error) => panic!("reused ABA result handback failed: {error}"),
            },
            Err(_) => panic!("reused ABA task faulted"),
        };
        assert!(matches!(terminal, DbIoResult::Unit));
        drain_control_tasks(control).await;
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        pool.shutdown();
        loop {
            match db_io_maintenance_step() {
                Ok(true) => {}
                Ok(false) => break,
                Err(error) => panic!("ABA maintenance failed: {error}"),
            }
        }
        assert_eq!(ledger_witness(), before);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn db_io_saturated_task_retry_wakes_parked_caller_without_unrelated_ingress() {
        let _serial = fixture_serial();
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
        assert_eq!(u64::from(DB_IO_RETRY_LIMIT), fixture["retry"]["maximumAttempts"].as_u64().unwrap());
        assert_eq!(DB_IO_RETRY_DELAY_MS, fixture["retry"]["timerDelayMs"].as_u64().unwrap());
        let before = ledger_witness();
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        pool.try_submit(Lane::Io, Box::new(move || { started_tx.send(()).unwrap(); release_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap(); })).ok().expect("retry blocker admitted");
        started_rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE { pool.try_submit(Lane::Io, Box::new(|| {})).ok().expect("exact queue capacity"); }
        let mut operation = submit_db_io_task(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://parked-retry").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
        assert!(DB_IO_TASK_SLOTS[operation.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).retry_attempt.is_some());
        struct WakeSender(std::sync::mpsc::Sender<()>);
        impl std::task::Wake for WakeSender { fn wake(self: Arc<Self>) { let _ = self.0.send(()); } }
        let (wake_tx, wake_rx) = std::sync::mpsc::channel();
        let waker = std::task::Waker::from(Arc::new(WakeSender(wake_tx)));
        let mut context = std::task::Context::from_waker(&waker);
        DB_IO_RETRY_MAINTENANCE_CURSOR.store((operation.handle.slot as usize + 1) % DB_IO_OPERATION_ITEMS, std::sync::atomic::Ordering::Release);
        assert!(Pin::new(&mut operation).poll(&mut context).is_pending());
        release_tx.send(()).unwrap();
        assert_eq!(wake_rx.recv_timeout(std::time::Duration::from_secs(2)).is_ok(), fixture["retry"]["terminalAfterQueueRelease"].as_bool().unwrap(), "retry must wake the parked caller without another DB request or maintenance poll");
        assert!(matches!(exact_fixture_result(operation.await, "parked retry"), DbIoResult::Unit));
        drain_control_tasks(control).await;
        close_db_io_backend(control).await.unwrap();
        pool.shutdown();
        assert_eq!(ledger_witness(), before);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn db_io_retry_generation_max_publishes_a_lossless_terminal_fault() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        pool.try_submit(
            Lane::Io,
            Box::new(move || {
                let _ = started_tx.send(());
                let _ = release_rx.recv();
            }),
        )
        .ok()
        .expect("DB I/O retry blocker admission");
        started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
            if let Err(error) = pool.try_submit(Lane::Io, Box::new(|| {})) {
                panic!("retry max fixture exhausted early: {:?}", error.kind());
            }
        }
        let mut operation =
            submit_db_io_task(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://retry-generation-max").unwrap() }).unwrap_or_else(|(error, _)| panic!("retry max fixture allocation failed: {error}"));
        {
            let mut owner = DB_IO_TASK_SLOTS[operation.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(owner.retry_attempt.is_some());
            owner.retry_generation = u64::MAX;
        }
        db_io_retry(operation.handle, u64::MAX);
        let mut fault = match operation.take() {
            Ok(Some(Err(fault))) => fault,
            Ok(Some(Ok(_))) => panic!("retry generation exhaustion published a result"),
            Ok(None) => panic!("retry generation exhaustion did not publish its terminal fault"),
            Err(error) => panic!("retry generation exhaustion take failed: {error}"),
        };
        assert_eq!(fault.kind, DbIoFaultKind::Saturated);
        assert_eq!(fault.detail.as_str(), "DB I/O retry generation exhausted");
        while fault.close_step() {}
        release_tx.send(()).unwrap();
        drain_control_tasks(control).await;
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        pool.shutdown();
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_postgres_and_neo4j_mock_drivers_use_supplied_writer_and_observed_capacity() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let pool = db_io_test_pool();
        for kind in [DbIoBackendKind::Postgres, DbIoBackendKind::Neo4j] {
            let control = register_db_io_backend(kind, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();
            let output = DbIoPageWriter::try_reserve(1).unwrap();
            let task = DbIoTask::WalRead { backend: control, document: DbIoText::try_from_str("fixture-driver-result").unwrap(), index: 0, range: ByteRange { offset: 0, len: 1 }, output };
            let mut operation = submit_db_io_task(pool.as_ref(), task).unwrap_or_else(|(error, _)| panic!("mock driver task allocation failed: {error}"));
            let mut lease = match operation.take_async_native().await {
                Ok(lease) => lease,
                Err(error) => panic!("mock driver lease failed: {error}"),
            };
            lease.enter_lane_io_driver_turn().unwrap();
            let task_operation = lease.operation();
            let mut reservation = DbIoDriverReservation::try_reserve(task_operation, DB_IO_OPERATION_BYTES as usize).unwrap();
            let mut driver_result = Vec::with_capacity(DB_IO_OPERATION_BYTES as usize);
            driver_result.push(0x7c);
            reservation.observe_capacity(driver_result.capacity()).unwrap();
            let result = match lease.task_mut() {
                Ok(DbIoTask::WalRead { output, .. }) => {
                    assert_eq!(output.write_fragment(&driver_result).unwrap(), driver_result.len());
                    drop(driver_result);
                    reservation.close_step().unwrap();
                    DbIoResult::Pages(output.finish().unwrap())
                }
                Ok(_) => panic!("mock driver task taxonomy changed"),
                Err(error) => panic!("mock driver lost supplied writer: {error}"),
            };
            lease.leave_lane_io_driver_turn().unwrap();
            lease.complete(Ok(result)).unwrap();
            let mut pages = match exact_fixture_result(operation.await, "mock driver") {
                DbIoResult::Pages(pages) => pages,
                _ => panic!("mock driver returned the wrong typed result"),
            };
            while pages.close_step().unwrap().is_some() {}
            drain_control_tasks(control).await;
            retire_db_io_backend(control).unwrap();
            close_db_io_backend(control).await.unwrap();
        }
        assert_eq!(ledger_witness(), before);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let polled_on_worker = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let poll_thread = Arc::new(std::sync::Mutex::new(None));
        let worker_role = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let close_thread = Arc::new(std::sync::Mutex::new(None));
        let close_worker_role = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let caller = std::thread::current().id();
        let pool = db_io_test_pool();
        let control = register_db_io_backend(
            DbIoBackendKind::Postgres,
            Box::new(AsyncLaneProbeExecutor {
                terminal: false,
                caller,
                polled_on_worker: polled_on_worker.clone(),
                poll_thread: poll_thread.clone(),
                worker_role: worker_role.clone(),
                close_thread: close_thread.clone(),
                close_worker_role: close_worker_role.clone(),
            }),
            pool.clone(),
        )
        .unwrap();
        let mut operation = submit_db_io_task(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://actual-lane-io").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
        operation.start_async_native_on_lane_io().await.unwrap();
        let terminal = operation.await.unwrap().into_result().unwrap();
        assert!(matches!(terminal, DbIoResult::Unit));
        assert!(polled_on_worker.load(std::sync::atomic::Ordering::Acquire));
        assert!(worker_role.load(std::sync::atomic::Ordering::Acquire));
        assert_ne!(*poll_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(caller));
        drain_control_tasks(control).await;
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        assert!(close_worker_role.load(std::sync::atomic::Ordering::Acquire));
        assert_ne!(*close_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(caller));
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_all_five_backend_controls_require_explicit_terminal_close_witness() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        for kind in [DbIoBackendKind::Memory, DbIoBackendKind::Filesystem, DbIoBackendKind::Sqlite, DbIoBackendKind::Postgres, DbIoBackendKind::Neo4j] {
            let pool = db_io_test_pool();
            let control = register_db_io_backend(kind, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool).unwrap();
            retire_db_io_backend(control).unwrap();
            close_db_io_backend(control).await.unwrap();
            let waker = std::task::Waker::noop();
            let context = &mut std::task::Context::from_waker(waker);
            assert!(matches!(db_io_backend_close_lane_step(control, context), Err(DbError::StaleGeneration { .. })));
        }
        assert_eq!(ledger_witness(), before);
    }

    #[test]
    fn db_io_lost_page_handle_resumes_the_same_retirement_cursor() {
        let _serial = fixture_serial();
        let owner = pages(&[0x77; DB_IO_PAGE_BYTES + 1]);
        let operation = owner.operation();
        drop(owner);
        assert_eq!(db_io_page_maintenance_step().unwrap(), Some(DB_IO_PAGE_BYTES));
        assert_eq!(db_io_page_maintenance_step().unwrap(), Some(DB_IO_PAGE_BYTES));
        assert_eq!(db_io_page_maintenance_step().unwrap(), None);
        assert_ne!(operation, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return() {
        let _serial = fixture_serial();
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
        let inline = std::mem::size_of::<MemoryDbIoExecutor>() as u64;
        assert!(inline <= fixture["maximumInlineBytes"].as_u64().unwrap(), "fixed backend tables must not occupy the caller stack");
        let before = ledger_witness();
        let expected = DbIoCredit { pages: 0, bytes: MemoryDbIoExecutor::backing_bytes(), items: 1, controls: 1 };
        for case in fixture["admission"].as_array().unwrap() {
            let remaining = match case["remaining"].as_str().unwrap() {
                "exact" => expected.bytes,
                "one-short" => expected.bytes - 1,
                "zero" => 0,
                other => panic!("unknown memory admission vector {other}"),
            };
            let filler = DbIoCredit { bytes: DB_IO_PROCESS_BYTES - before.0.bytes - remaining, ..DbIoCredit::default() };
            let filler_operation = db_io_backend_owner_reserve(filler).unwrap();
            let admitted = db_io_backend_owner_reserve(expected);
            assert_eq!(admitted.is_ok(), case["accepted"].as_bool().unwrap());
            if let Ok(operation) = admitted {
                let ledger = lock(db_io_operation_ledger());
                assert_eq!(ledger.slots[db_io_operation_slot(&ledger, operation).unwrap()].live, expected);
                drop(ledger);
                db_io_operation_return(operation, expected).unwrap();
            }
            db_io_operation_return(filler_operation, filler).unwrap();
            assert_eq!(ledger_witness(), before);
        }
        let storage = MemoryStorage::new(db_io_test_pool()).await.unwrap();
        let (slot, generation) = db_io_backend_parts(storage.control);
        let owner_operation = {
            let mut registry = lock(db_io_backend_registry());
            let owner = &mut registry.slots[slot as usize];
            assert_eq!(owner.generation, generation);
            assert_eq!(owner.owner_credit, expected);
            let executor = owner.executor.as_mut().unwrap().as_any_mut().downcast_mut::<MemoryDbIoExecutor>().unwrap();
            fn table<T>(name: &'static str, cells: &std::sync::Mutex<Box<[Option<T>]>>) -> (&'static str, usize, usize) {
                let cells = lock(cells);
                (name, cells.len(), std::mem::size_of_val(cells.as_ref()))
            }
            let tables = [
                table("wal", &executor.wal), table("snapshots", &executor.snapshots), table("payloads", &executor.payloads),
                table("index-runs", &executor.index_runs), table("leases", &executor.leases), table("operations", &executor.operations),
                table("retired-pages", &executor.retired_pages), table("retired-wal", &executor.retired_wal),
            ];
            assert_eq!(fixture["tables"].as_array().unwrap().len(), tables.len());
            for ((name, slots, _), row) in tables.iter().zip(fixture["tables"].as_array().unwrap()) {
                assert_eq!(row["name"].as_str().unwrap(), *name);
                assert_eq!(row["slots"].as_u64().unwrap(), *slots as u64);
            }
            assert_eq!(inline + tables.iter().map(|(_, _, bytes)| *bytes as u64).sum::<u64>(), expected.bytes);
            assert_eq!(executor.owner_backing_bytes(), expected.bytes);
            owner.owner_operation
        };
        {
            let ledger = lock(db_io_operation_ledger());
            assert_eq!(ledger.slots[db_io_operation_slot(&ledger, owner_operation).unwrap()].live, expected);
        }
        let probe_credit = DbIoCredit { items: 1, ..DbIoCredit::default() };
        let probe = db_io_operation_reserve(probe_credit).unwrap();
        assert!(db_io_backend_admit_operation(storage.control, probe).unwrap());
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        assert_eq!(db_io_backend_close_lane_step(storage.control, &mut context).unwrap(), fixture["closeWhileAdmitted"].as_bool().unwrap());
        assert!(lock(db_io_backend_registry()).slots[slot as usize].executor.is_some());
        db_io_backend_return_operation(storage.control, probe).unwrap();
        db_io_operation_return(probe, probe_credit).unwrap();
        let document = ArtifactId("memory-retirement-frontier".into());
        storage.create_segment(&document, 0).await.unwrap();
        let mut retained = storage.list_segments(&document).await.unwrap();
        assert_eq!(retained.as_slice(), &[0]);
        for index in 0..fixture["sequentialTasks"].as_u64().unwrap() {
            assert_eq!(storage.segment_len(&document, 0).await.unwrap_or_else(|error| panic!("sequential task {index} lost capacity with one retained result: {error}")), 0);
            assert_eq!(retained.as_slice(), &[0], "task retirement must preserve the caller-owned list");
            let handle = retained.result_handback.unwrap();
            let ledger = lock(db_io_operation_ledger());
            assert!(ledger.slots[db_io_operation_slot(&ledger, handle.operation).unwrap()].live.bytes >= DB_IO_LIST_TRANSIENT_BYTES);
        }
        while retained.close_step() {}
        assert!(retained.values.is_none());
        storage.close().await.unwrap();
        close_db_io_backend(storage.control).await.unwrap();
        assert!(db_io_operation_slot(&lock(db_io_operation_ledger()), owner_operation).is_none());
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_memory_backend_uses_actual_typed_submit_take_result_and_terminal_close() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let pool = crate::db_storage::db_io_test_pool();
        let storage = MemoryStorage::new(pool.clone()).await.unwrap();
        let second = MemoryStorage::new(pool.clone()).await.unwrap();
        assert!(Arc::ptr_eq(&storage.pool, &pool));
        assert!(Arc::ptr_eq(&second.pool, &pool));
        let document = ArtifactId("typed-memory-fixture".to_string());
        storage.create_segment(&document, 1).await.unwrap();
        storage.append(&document, 1, pages(&[0x91; DB_IO_PAGE_BYTES + 1])).await.unwrap();
        let mut result = storage.read(&document, 1, ByteRange { offset: 0, len: (DB_IO_PAGE_BYTES + 1) as u64 }).await.unwrap();
        assert_eq!(result.page_count(), 2);
        while result.close_step().unwrap().is_some() {}
        storage.close().await.unwrap();
        second.close().await.unwrap();
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_async_native_lost_backend_uses_typed_lane_lease_and_mounted_terminal_witness() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let pool = db_io_test_pool();
        let control = register_db_io_backend(DbIoBackendKind::Postgres, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();
        let mut operation = submit_db_io_task(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://async-native").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
        let lease = operation.take_async_native().await.unwrap();
        lease.complete(Ok(DbIoResult::Unit)).unwrap();
        let terminal = match operation.await {
            Ok(terminal) => terminal,
            Err(_) => panic!("async-native law task faulted"),
        };
        assert!(matches!(terminal.into_result().unwrap(), DbIoResult::Unit));
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_cancellation_before_during_and_receiver_drop_retain_exact_terminal_owners() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let pool = db_io_test_pool();
        let control = register_db_io_backend(DbIoBackendKind::Neo4j, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();

        let mut before_execution = submit_db_io_task(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://cancel-before").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
        before_execution.cancel().unwrap();
        let mut fault = match before_execution.await {
            Err(fault) => fault,
            Ok(_) => panic!("cancel-before fixture returned a result"),
        };
        assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
        assert_eq!(fault.cause, DbIoFaultCause::Closed);
        while fault.close_step() {}
        drain_control_tasks(control).await;

        let mut during_execution = submit_db_io_task(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://cancel-during").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
        let lease = during_execution.take_async_native().await.unwrap();
        during_execution.cancel().unwrap();
        assert!(db_io_maintenance_step().unwrap());
        {
            let owner = DB_IO_TASK_SLOTS[during_execution.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(owner.async_detached);
            assert!(owner.task.is_none());
            assert!(owner.close_enqueued);
            assert!(owner.terminal.is_none());
        }
        let mut output = DbIoPageWriter::try_reserve_for_operation(lease.operation(), 1).unwrap();
        assert_eq!(output.write_fragment(&[0xA5]).unwrap(), 1);
        lease.complete(Ok(DbIoResult::Pages(output.finish().unwrap()))).unwrap();
        let mut fault = loop {
            match during_execution.take().unwrap() {
                Some(Err(fault)) => break fault,
                Some(Ok(_)) => panic!("cancel-during fixture published its retained result"),
                None => assert!(db_io_maintenance_step().unwrap()),
            }
        };
        assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
        assert_eq!(fault.cause, DbIoFaultCause::Closed);
        while fault.close_step() {}
        drain_control_tasks(control).await;

        let mut receiver_drop = submit_db_io_task(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://receiver-drop").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
        let lease = receiver_drop.take_async_native().await.unwrap();
        drop(receiver_drop);
        drop(lease);
        drain_control_tasks(control).await;
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        assert_eq!(ledger_witness(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn db_io_panic_backend_fault_and_shutdown_close_reach_exact_prior_witness() {
        let _serial = fixture_serial();
        let before = ledger_witness();
        let pool = db_io_test_pool();
        for (panics, expected) in [(false, DbIoFaultKind::Backend), (true, DbIoFaultKind::Panic)] {
            let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingFaultLawExecutor { panic: panics, terminal: false }), pool.clone()).unwrap();
            let operation = submit_db_io_task(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://hostile-blocking").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
            let mut fault = match operation.await {
                Err(fault) => fault,
                Ok(_) => panic!("hostile blocking fixture returned a result"),
            };
            assert_eq!(fault.kind, expected);
            assert_eq!(fault.cause, DbIoFaultCause::Internal);
            let expected_detail = if panics { "DB I/O backend panicked" } else { "hostile DB I/O fixture backend fault" };
            assert_eq!(fault.detail.as_str(), expected_detail);
            while fault.close_step() {}
            drain_control_tasks(control).await;
            retire_db_io_backend(control).unwrap();
            close_db_io_backend(control).await.unwrap();
        }
        assert_eq!(ledger_witness(), before);
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn pages(bytes: &[u8]) -> DbIoPages {
        let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("test storage pages admitted");
        for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
            assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
        }
        writer.finish().unwrap()
    }

    //#region 🔖️WalStorage
    async fn exercise_wal_storage(storage: &impl WalStorage) {
        let document: ArtifactId = "doc-wal".into();

        block_on_ready(storage.create_segment(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.create_segment(&document, 0)).await, Err(DbError::AlreadyExists(_))));

        let len_after_first = block_on_ready(storage.append(&document, 0, pages(b"hello "))).await.unwrap();
        assert_eq!(len_after_first, 6);
        let len_after_second = block_on_ready(storage.append(&document, 0, pages(b"world"))).await.unwrap();
        assert_eq!(len_after_second, 11);
        assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 11);

        let read_back = block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 5 })).await.unwrap();
        assert_eq!(read_back, b"world");
        assert!(matches!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 100 })).await, Err(DbError::InvalidArgument(_))));

        block_on_ready(storage.sync(&document, 0, DurabilityClass::Fsync)).await.unwrap();

        block_on_ready(storage.truncate_tail(&document, 0, 6)).await.unwrap();
        assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 6);
        assert_eq!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 0, len: 6 })).await.unwrap(), b"hello ");

        block_on_ready(storage.create_segment(&document, 1)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0, 1]);

        block_on_ready(storage.seal(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.append(&document, 0, pages(b"!"))).await, Err(DbError::InvalidArgument(_))));
        assert!(matches!(block_on_ready(storage.truncate_tail(&document, 0, 0)).await, Err(DbError::InvalidArgument(_))));

        block_on_ready(storage.delete_segment(&document, 1)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0]);

        assert!(matches!(block_on_ready(storage.append(&document, 99, pages(b"x"))).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_wal_storage_laws() {
        exercise_wal_storage(&MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_wal_storage_laws() {
        exercise_wal_storage(&fs_scratch("wal_laws").await).await;
    }
    //#endregion 🔖️WalStorage

    //#region 🔖️SnapshotStorage
    async fn exercise_snapshot_storage(storage: &impl SnapshotStorage) {
        let document: ArtifactId = "doc-snap".into();
        assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), None);

        block_on_ready(storage.write_generation(&document, 0, pages(b"gen-zero-bytes"))).await.unwrap();
        block_on_ready(storage.write_generation(&document, 1, pages(b"gen-one-bytes"))).await.unwrap();
        assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![0, 1]);
        assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), Some(1));
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-bytes");

        block_on_ready(storage.write_generation(&document, 0, pages(b"gen-zero-overwritten"))).await.unwrap();
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-overwritten");

        block_on_ready(storage.delete_generation(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.read_generation(&document, 0)).await, Err(DbError::NotFound(_))));
        assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_snapshot_storage_laws() {
        exercise_snapshot_storage(&MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_snapshot_storage_laws() {
        exercise_snapshot_storage(&fs_scratch("snapshot_laws").await).await;
    }
    //#endregion 🔖️SnapshotStorage

    //#region 🔖️PayloadStorage
    async fn exercise_payload_storage(storage: &impl PayloadStorage) {
        let bytes = b"a payload blob that gets content-addressed";
        let hash_a = block_on_ready(storage.put(pages(bytes))).await.unwrap();
        let hash_b = block_on_ready(storage.put(pages(bytes))).await.unwrap();
        assert_eq!(hash_a, hash_b, "put is idempotent under content equality");
        assert_eq!(hash_a, ContentHash(*semio_framework_hash::hash(bytes).as_bytes()));

        assert!(block_on_ready(storage.contains(&hash_a)).await.unwrap());
        assert_eq!(block_on_ready(storage.get(&hash_a)).await.unwrap(), bytes);
        assert_eq!(block_on_ready(storage.len(&hash_a)).await.unwrap(), bytes.len() as u64);

        let other_hash = ContentHash([0xAB; 32]);
        assert!(!block_on_ready(storage.contains(&other_hash)).await.unwrap());
        assert!(matches!(block_on_ready(storage.get(&other_hash)).await, Err(DbError::NotFound(_))));

        block_on_ready(storage.delete(&hash_a)).await.unwrap();
        assert!(!block_on_ready(storage.contains(&hash_a)).await.unwrap());
        assert!(matches!(block_on_ready(storage.get(&hash_a)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_payload_storage_laws() {
        exercise_payload_storage(&MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_payload_storage_laws() {
        exercise_payload_storage(&fs_scratch("payload_laws").await).await;
    }
    //#endregion 🔖️PayloadStorage

    //#region 🔖️CatalogStorage
    async fn exercise_catalog_storage(storage: &impl CatalogStorage) {
        assert_eq!(block_on_ready(storage.read_root()).await.unwrap(), None);

        let epoch_1 = block_on_ready(storage.cas_root(EpochFence::INITIAL, pages(b"root-v1"))).await.unwrap();
        assert_eq!(epoch_1, EpochFence::INITIAL.next());
        let (bytes, fence) = block_on_ready(storage.read_root()).await.unwrap().unwrap();
        assert_eq!(bytes, b"root-v1");
        assert_eq!(fence, epoch_1);

        // A stale `expected` (still `INITIAL`, but the root already moved to `epoch_1`) is fenced.
        assert!(matches!(block_on_ready(storage.cas_root(EpochFence::INITIAL, pages(b"root-stale"))).await, Err(DbError::Fenced { .. })));

        let epoch_2 = block_on_ready(storage.cas_root(epoch_1, pages(b"root-v2"))).await.unwrap();
        assert_eq!(epoch_2, epoch_1.next());
        assert_eq!(block_on_ready(storage.read_root()).await.unwrap().unwrap().0, b"root-v2");
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_catalog_storage_laws() {
        exercise_catalog_storage(&MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_catalog_storage_laws() {
        exercise_catalog_storage(&fs_scratch("catalog_laws").await).await;
    }
    //#endregion 🔖️CatalogStorage

    //#region 🔖️IndexStorage
    async fn exercise_index_storage(storage: &impl IndexStorage) {
        let document: ArtifactId = "doc-index".into();
        block_on_ready(storage.write_run(&document, 0, pages(b"run-zero"))).await.unwrap();
        block_on_ready(storage.write_run(&document, 1, pages(b"run-one"))).await.unwrap();
        assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![0, 1]);
        assert_eq!(block_on_ready(storage.read_run(&document, 1)).await.unwrap(), b"run-one");

        block_on_ready(storage.delete_run(&document, 0)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![1]);
        assert!(matches!(block_on_ready(storage.read_run(&document, 0)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_index_storage_laws() {
        exercise_index_storage(&MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_index_storage_laws() {
        exercise_index_storage(&fs_scratch("index_laws").await).await;
    }
    //#endregion 🔖️IndexStorage

    //#region 🔖️LeaseStorage
    async fn exercise_lease_storage(storage: &impl LeaseStorage) {
        let fence_1 = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 0)).await.unwrap();
        assert_eq!(fence_1, EpochFence::INITIAL);

        // Re-acquiring the same, unexpired lease by the same holder is idempotent (same fence).
        let fence_reacquire = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 100)).await.unwrap();
        assert_eq!(fence_reacquire, fence_1);

        // A different holder cannot acquire an unexpired lease.
        assert!(matches!(block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 100)).await, Err(DbError::Conflict(_))));

        block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 500)).await.unwrap();
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1.next(), 1_000, 500)).await, Err(DbError::Fenced { .. })));
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-b", fence_1, 1_000, 500)).await, Err(DbError::Unauthorized(_))));

        let mut current = block_on_ready(storage.current("shard-1", 600)).await.unwrap().unwrap();
        assert_eq!(current.holder.as_str(), "node-a");
        assert_eq!(current.fence, fence_1);
        current.close_step();

        // After expiry (renewed at 500 for 1_000ms => expires at 1_500), a different holder can
        // take over, bumping the fence — the fencing law a stale former holder is later rejected by.
        assert_eq!(block_on_ready(storage.current("shard-1", 2_000)).await.unwrap(), None);
        let fence_2 = block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 2_000)).await.unwrap();
        assert_eq!(fence_2, fence_1.next());

        // The old holder's stale fence is now rejected.
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 2_100)).await, Err(DbError::Unauthorized(_))));

        block_on_ready(storage.release("shard-1", "node-b", fence_2)).await.unwrap();
        assert_eq!(block_on_ready(storage.current("shard-1", 2_100)).await.unwrap(), None);
        assert!(matches!(block_on_ready(storage.release("shard-1", "node-b", fence_2)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_lease_storage_laws() {
        exercise_lease_storage(&MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_lease_storage_laws() {
        exercise_lease_storage(&fs_scratch("lease_laws").await).await;
    }
    //#endregion 🔖️LeaseStorage

    //#region 🔖️DbBackend
    #[semio_framework_async_macros::async_test]
    async fn memory_storage_db_backend_accessors_and_capabilities() {
        let storage: DbBackend = DbBackend::Memory(MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap());
        let document: ArtifactId = "doc-umbrella".into();
        block_on_ready(poll_once(storage.wal()).await.create_segment(&document, 0)).await.unwrap();
        block_on_ready(poll_once(storage.catalog()).await.cas_root(EpochFence::INITIAL, pages(b"root"))).await.unwrap();

        let capabilities = poll_once(storage.capabilities()).await;
        assert!(!capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Memory);
        assert!(capabilities.supports_cas);
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_db_backend_accessors_and_capabilities() {
        let storage: DbBackend = DbBackend::Fs(fs_scratch("umbrella").await);
        let document: ArtifactId = "doc-umbrella".into();
        block_on_ready(poll_once(storage.index()).await.write_run(&document, 0, pages(b"run"))).await.unwrap();
        assert_eq!(block_on_ready(poll_once(storage.index()).await.read_run(&document, 0)).await.unwrap(), b"run");

        let capabilities = poll_once(storage.capabilities()).await;
        assert!(capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Fsync);
        assert!(capabilities.supports_fsync);
    }
    //#endregion 🔖️DbBackend

    //#region 🔖️Fs
    #[cfg(feature = "fs")]
    static SCRATCH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    /// @emoji 🎲️ A fresh `FsStorage` rooted at a unique scratch directory under
    /// `std::env::temp_dir()` — no external `tempfile` crate dependency, mirroring `pack_io`'s own
    /// test helper convention. The test-owned process pool drives the same retained operation path.
    #[cfg(feature = "fs")]
    async fn fs_scratch(name: &str) -> FsStorage {
        let pid = std::process::id();
        let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("db_storage_test_{name}_{pid}_{counter}"));
        poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap()
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_rejects_unsafe_path_components() {
        let storage = fs_scratch("path_safety").await;
        let traversal_document: ArtifactId = "../escape".into();
        assert!(matches!(block_on_ready(storage.create_segment(&traversal_document, 0)).await, Err(DbError::InvalidArgument(_))));

        let separator_document: ArtifactId = "sub/dir".into();
        assert!(matches!(block_on_ready(storage.create_segment(&separator_document, 0)).await, Err(DbError::InvalidArgument(_))));

        let empty_document: ArtifactId = "".into();
        assert!(matches!(block_on_ready(storage.create_segment(&empty_document, 0)).await, Err(DbError::InvalidArgument(_))));
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_write_atomic_survives_reopen_across_instances() {
        let pid = std::process::id();
        let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("db_storage_test_reopen_{pid}_{counter}"));

        {
            let storage = poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap();
            let document: ArtifactId = "doc-reopen".into();
            block_on_ready(storage.write_generation(&document, 0, pages(b"persisted across reopen"))).await.unwrap();
        }
        {
            let storage = poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap();
            let document: ArtifactId = "doc-reopen".into();
            assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"persisted across reopen");
        }
    }
    //#endregion 🔖️Fs
}
//#endregion 🧪️Tests
