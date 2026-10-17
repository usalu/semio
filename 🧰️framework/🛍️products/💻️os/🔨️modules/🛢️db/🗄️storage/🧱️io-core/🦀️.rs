//! 🧲️ Db I/O kernel core: credit ledger, page arena, task model and lost-owner parking.
#![allow(async_fn_in_trait)]

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, GenerationId};
use db_policy::AdmissionWaiters;
use pack::{ByteRange, ContentHash};
use semio_framework_async::{WorkerDeferredWakeTicket, WorkerMaintenanceTicket, WorkerPool, WorkerPoolUse, WorkerSubmitErrorKind};
use std::{future::Future, pin::Pin, sync::Arc, sync::Mutex, sync::atomic::AtomicU64, sync::atomic::Ordering, task::Context, task::Poll, task::Waker};

/// 📊️ One process-wide typed DB I/O task depth signal.
pub static BLOCKING_QUEUE: semio_framework_trace::QueueCounter = semio_framework_trace::QueueCounter::new();
//#region 🔖️Limits
/// 🛡️ Ceiling on any single blob this crate reads into memory in one call (one WAL read
/// range, one snapshot generation, one payload, one index run, one lease record) — validated via
/// `check_len` BEFORE the read buffer is allocated, mirroring `pack_core`'s stated
/// invariant. This crate's own choice (the contract doesn't fix a number): generous enough for a
/// snapshot generation or a large payload, small enough to refuse an obviously-corrupt on-disk
/// length before trying to allocate it.
pub const DB_IO_MAX_READ_BYTES: u64 = 496 * 1024;

pub const DB_IO_PAGE_BYTES: usize = 16 * 1024;

pub const DB_IO_OPERATION_PAGES: usize = 64;

pub const DB_IO_TOTAL_PAGES: usize = 1024;

pub const DB_IO_PAGE_RETIREMENT_SLOTS: usize = DB_IO_TOTAL_PAGES * 2;

pub const DB_IO_OPERATION_ITEMS: usize = 64;

pub const DB_IO_BACKEND_CONTROLS: usize = 64;

pub const DB_IO_LEDGER_ITEMS: usize = DB_IO_OPERATION_ITEMS + DB_IO_BACKEND_CONTROLS;

const DB_IO_TASK_SLOT_BYTES: u64 = size_of::<DbIoTaskSlot>() as u64;

pub const DB_IO_LIST_BACKING_BYTES: u64 = (DB_IO_LIST_ITEMS * size_of::<u64>()) as u64;

const DB_IO_LIST_TRANSIENT_BYTES: u64 = DB_IO_LIST_BACKING_BYTES * 2;

pub const DB_IO_OPERATION_BYTES: u64 = (DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES * 4) as u64 + DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES;

const DB_IO_OPERATION_ITEM_CREDIT: usize = DB_IO_LIST_ITEMS * 2 + DB_IO_OPERATION_PAGES + 32;

const DB_IO_OPERATION_CONTROL_CREDIT: usize = 16;

pub const DB_IO_PROCESS_BYTES: u64 = (DB_IO_PAGE_BYTES * DB_IO_TOTAL_PAGES * 2 + DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES * DB_IO_OPERATION_ITEMS * 2) as u64 + DB_IO_OPERATION_ITEMS as u64 * (DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES);

const DB_IO_PROCESS_ITEM_CREDIT: usize = DB_IO_OPERATION_ITEMS * DB_IO_OPERATION_ITEM_CREDIT;

const DB_IO_PROCESS_CONTROL_CREDIT: usize = DB_IO_OPERATION_ITEMS * DB_IO_OPERATION_CONTROL_CREDIT;

/// 🧱️ Fixed writer signals/controllers are process backing, separate from recyclable operation credit.
pub const DB_IO_WRITER_STATIC_BACKING_BYTES: u64 =
    (crate::WAL_WRITER_SIGNAL_BACKING_BYTES + crate::WAL_WRITER_PARK_CLOCK_BACKING_BYTES + crate::WAL_WRITER_CONTROLLER_BACKING_BYTES + crate::WAL_WRITER_ADMISSION_BACKING_BYTES) as u64;

pub const DB_IO_PROCESS_WITH_WRITER_BACKING_BYTES: u64 = DB_IO_PROCESS_BYTES + DB_IO_WRITER_STATIC_BACKING_BYTES;

const _: () = assert!(DB_IO_WRITER_STATIC_BACKING_BYTES <= 4 * 1024 * 1024);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DbIoCredit {
    pub pages: usize,
    pub bytes: u64,
    pub items: usize,
    pub controls: usize,
}

impl DbIoCredit {
    pub fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self { pages: self.pages.checked_add(other.pages)?, bytes: self.bytes.checked_add(other.bytes)?, items: self.items.checked_add(other.items)?, controls: self.controls.checked_add(other.controls)? })
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self { pages: self.pages.checked_sub(other.pages)?, bytes: self.bytes.checked_sub(other.bytes)?, items: self.items.checked_sub(other.items)?, controls: self.controls.checked_sub(other.controls)? })
    }

    fn is_zero(self) -> bool {
        self == Self::default()
    }
}

#[derive(Clone, Copy)]
struct DbIoOperationCreditSlot {
    pub generation: u64,
    pub operation: u64,
    pub live: DbIoCredit,
    pub result_leases: u16,
    task_attached: bool,
    backend_owner: bool,
    #[cfg(test)]
    pub owner: u64,
}

const EMPTY_DB_IO_OPERATION_CREDIT_SLOT: DbIoOperationCreditSlot = DbIoOperationCreditSlot { generation: 0, operation: 0, live: DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 0 }, result_leases: 0, task_attached: false, backend_owner: false, #[cfg(test)] owner: 0 };

#[cfg(any(test, feature = "testkit"))]
std::thread_local! {
    pub static DB_IO_LEDGER_OWNER: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(any(test, feature = "testkit"))]
pub static DB_IO_LEDGER_NEXT_OWNER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// 🏷️ Attributes every DB I/O operation and backend owner this thread reserves to one
/// census owner until the guard drops. The ledger admits against the process budget either way;
/// the owner only scopes [`db_io_ledger_census`], so concurrent owners in one process observe
/// their own credit instead of each other's.
#[cfg(any(test, feature = "testkit"))]
pub struct DbIoLedgerOwner {
    pub owner: u64,
    pub previous: u64,
}

#[cfg(any(test, feature = "testkit"))]
impl DbIoLedgerOwner {
    pub fn enter() -> Self {
        let owner = DB_IO_LEDGER_NEXT_OWNER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let previous = DB_IO_LEDGER_OWNER.with(|current| current.replace(owner));
        Self { owner, previous }
    }

    pub fn id(&self) -> u64 {
        self.owner
    }
}

#[cfg(any(test, feature = "testkit"))]
impl Drop for DbIoLedgerOwner {
    fn drop(&mut self) {
        DB_IO_LEDGER_OWNER.with(|current| current.set(self.previous));
    }
}

/// 🧮️ The live credit and live ledger slots attributed to one census owner.
#[cfg(any(test, feature = "testkit"))]
pub fn db_io_ledger_census(owner: u64) -> (DbIoCredit, usize) {
    let ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    ledger.slots.iter().filter(|slot| slot.generation != 0 && slot.owner == owner).fold((DbIoCredit::default(), 0), |(credit, slots), slot| (credit.checked_add(slot.live).expect("census credit is bounded by the process ledger"), slots + 1))
}

struct DbIoOperationLedger {
    pub slots: [DbIoOperationCreditSlot; DB_IO_LEDGER_ITEMS],
    pub free: [u16; DB_IO_LEDGER_ITEMS],
    pub free_read: usize,
    pub free_len: usize,
    totals: DbIoCredit,
    next_operation: u64,
    pub next_generation: u64,
}

impl DbIoOperationLedger {
    pub fn new() -> Self {
        Self { slots: [EMPTY_DB_IO_OPERATION_CREDIT_SLOT; DB_IO_LEDGER_ITEMS], free: std::array::from_fn(|index| index as u16), free_read: 0, free_len: DB_IO_LEDGER_ITEMS, totals: DbIoCredit::default(), next_operation: 1, next_generation: 1 }
    }
}

static DB_IO_OPERATION_LEDGER: std::sync::OnceLock<std::sync::Mutex<DbIoOperationLedger>> = std::sync::OnceLock::new();

pub fn db_io_operation_ledger() -> &'static std::sync::Mutex<DbIoOperationLedger> {
    DB_IO_OPERATION_LEDGER.get_or_init(|| std::sync::Mutex::new(DbIoOperationLedger::new()))
}

fn db_io_credit_within_limits(credit: DbIoCredit, process: bool) -> bool {
    if process {
        credit.pages <= DB_IO_TOTAL_PAGES && credit.bytes <= DB_IO_PROCESS_BYTES && credit.items <= DB_IO_PROCESS_ITEM_CREDIT && credit.controls <= DB_IO_PROCESS_CONTROL_CREDIT
    } else {
        credit.pages <= DB_IO_OPERATION_PAGES && credit.bytes <= DB_IO_OPERATION_BYTES && credit.items <= DB_IO_OPERATION_ITEM_CREDIT && credit.controls <= DB_IO_OPERATION_CONTROL_CREDIT
    }
}

pub fn db_io_operation_slot(ledger: &DbIoOperationLedger, operation: u64) -> Option<usize> {
    ledger.slots.iter().position(|slot| slot.operation == operation && slot.generation != 0)
}

/// 🎟️ The ledger slot a live operation occupies: unique among every operation alive at once,
/// so a backend keying per-operation cursor state by it can never hand one operation's cursor to
/// another (an `operation % capacity` key collided as soon as operations ran concurrently).
pub fn db_io_operation_owner_slot(operation: u64) -> Result<usize, DbError> {
    let ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O cursor owner has no live operation".to_string()))
}

pub fn db_io_operation_reserve(initial: DbIoCredit) -> Result<u64, DbError> {
    if !db_io_credit_within_limits(initial, false) {
        return Err(DbError::LimitExceeded("db_io operation aggregate credit"));
    }
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let totals = ledger.totals.checked_add(initial).ok_or(DbError::LimitExceeded("db_io process aggregate credit"))?;
    if ledger.free_len == 0 || ledger.next_operation == u64::MAX || ledger.next_generation == u64::MAX || !db_io_credit_within_limits(totals, true) {
        return Err(DbError::Unavailable(DB_IO_PROCESS_CREDIT_EXHAUSTED.to_string()));
    }
    let slot = ledger.free[ledger.free_read];
    ledger.free_read = (ledger.free_read + 1) % DB_IO_LEDGER_ITEMS;
    ledger.free_len -= 1;
    let operation = ledger.next_operation;
    ledger.next_operation += 1;
    let generation = ledger.next_generation;
    ledger.next_generation += 1;
    ledger.slots[slot as usize] = DbIoOperationCreditSlot { generation, operation, live: initial, result_leases: 0, task_attached: false, backend_owner: false, #[cfg(test)] owner: DB_IO_LEDGER_OWNER.with(std::cell::Cell::get) };
    ledger.totals = totals;
    Ok(operation)
}

/// ⚖️ The process-budget admission predicate of one backend owner against `occupied` credit.
pub fn db_io_backend_owner_admits(occupied: DbIoCredit, initial: DbIoCredit) -> bool {
    occupied.checked_add(initial).is_some_and(|totals| db_io_credit_within_limits(totals, true))
}

pub fn db_io_backend_owner_reserve(initial: DbIoCredit) -> Result<u64, DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let totals = ledger.totals.checked_add(initial).ok_or(DbError::LimitExceeded("DB I/O backend process credit"))?;
    if ledger.free_len == 0 || ledger.next_operation == u64::MAX || ledger.next_generation == u64::MAX || !db_io_backend_owner_admits(ledger.totals, initial) {
        return Err(DbError::Unavailable(DB_IO_BACKEND_CREDIT_EXHAUSTED.to_string()));
    }
    let slot = ledger.free[ledger.free_read];
    ledger.free_read = (ledger.free_read + 1) % DB_IO_LEDGER_ITEMS;
    ledger.free_len -= 1;
    let operation = ledger.next_operation;
    ledger.next_operation += 1;
    let generation = ledger.next_generation;
    ledger.next_generation += 1;
    ledger.slots[slot as usize] = DbIoOperationCreditSlot { generation, operation, live: initial, result_leases: 0, task_attached: false, backend_owner: true, #[cfg(test)] owner: DB_IO_LEDGER_OWNER.with(std::cell::Cell::get) };
    ledger.totals = totals;
    Ok(operation)
}

pub fn db_io_operation_add(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
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

/// 🧱️ Grows a backend owner's own heap credit; only the process budget bounds a backend,
/// never the per-operation budget of a single task.
pub fn db_io_backend_owner_add(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O backend owner is not live".to_string()))?;
    if !ledger.slots[index].backend_owner {
        return Err(DbError::Internal("DB I/O backend credit target is not a backend owner".to_string()));
    }
    let owner_total = ledger.slots[index].live.checked_add(credit).ok_or(DbError::LimitExceeded("DB I/O backend owner credit"))?;
    let process_total = ledger.totals.checked_add(credit).ok_or(DbError::LimitExceeded("DB I/O backend process credit"))?;
    if !db_io_credit_within_limits(process_total, true) {
        return Err(DbError::Unavailable(DB_IO_BACKEND_CREDIT_EXHAUSTED.to_string()));
    }
    ledger.slots[index].live = owner_total;
    ledger.totals = process_total;
    Ok(())
}

pub fn db_io_operation_attach_task(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O task lost aggregate operation".to_string()))?;
    if ledger.slots[index].task_attached {
        return Err(DbError::Internal("DB I/O aggregate operation already has a task".to_string()));
    }
    let operation_total = ledger.slots[index].live.checked_add(credit).ok_or(DbError::LimitExceeded("db_io operation aggregate credit"))?;
    let process_total = ledger.totals.checked_add(credit).ok_or(DbError::LimitExceeded("db_io process aggregate credit"))?;
    if !db_io_credit_within_limits(operation_total, false) || !db_io_credit_within_limits(process_total, true) {
        return Err(DbError::Unavailable(DB_IO_TASK_ADMISSION_EXHAUSTED.to_string()));
    }
    ledger.slots[index].live = operation_total;
    ledger.slots[index].task_attached = true;
    ledger.totals = process_total;
    Ok(())
}

pub fn db_io_operation_mark_task(operation: u64) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O task lost aggregate operation".to_string()))?;
    if ledger.slots[index].task_attached {
        return Err(DbError::Internal("DB I/O aggregate operation already has a task".to_string()));
    }
    ledger.slots[index].task_attached = true;
    Ok(())
}

pub fn db_io_operation_detach_task(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O aggregate task return lost operation".to_string()))?;
    if !ledger.slots[index].task_attached {
        return Err(DbError::Internal("DB I/O aggregate task returned twice".to_string()));
    }
    ledger.slots[index].live = ledger.slots[index].live.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O aggregate task credit returned twice".to_string()))?;
    ledger.slots[index].task_attached = false;
    ledger.totals = ledger.totals.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O process task credit returned twice".to_string()))?;
    let released = db_io_operation_try_release_locked(&mut ledger, index);
    drop(ledger);
    db_io_admission_released();
    released
}

pub fn db_io_operation_add_result_lease(operation: u64) -> Result<(), DbError> {
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

pub fn db_io_operation_return_result_lease(operation: u64, retained_credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O result handback lost aggregate operation".to_string()))?;
    ledger.slots[index].result_leases = ledger.slots[index].result_leases.checked_sub(1).ok_or_else(|| DbError::Internal("DB I/O result lease returned twice".to_string()))?;
    let credit = db_io_result_lease_credit().checked_add(retained_credit).ok_or(DbError::LimitExceeded("DB I/O retained result handback credit"))?;
    ledger.slots[index].live = ledger.slots[index].live.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O result lease credit returned twice".to_string()))?;
    ledger.totals = ledger.totals.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O process result lease credit returned twice".to_string()))?;
    db_io_operation_try_release_locked(&mut ledger, index)
}

pub fn db_io_operation_return(operation: u64, credit: DbIoCredit) -> Result<(), DbError> {
    let mut ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = db_io_operation_slot(&ledger, operation).ok_or_else(|| DbError::Internal("DB I/O aggregate return lost operation".to_string()))?;
    ledger.slots[index].live = ledger.slots[index].live.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O aggregate credit returned twice".to_string()))?;
    ledger.totals = ledger.totals.checked_sub(credit).ok_or_else(|| DbError::Internal("DB I/O process credit returned twice".to_string()))?;
    let released = db_io_operation_try_release_locked(&mut ledger, index);
    drop(ledger);
    db_io_admission_released();
    released
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
pub enum DbIoPagePhase {
    Free,
    CheckedOutInput,
    CheckedOutWriter,
    Queued,
    Executing,
    TerminalResult,
    Closing,
}

struct DbIoPageBacking(std::cell::UnsafeCell<[u8; DB_IO_PAGE_BYTES]>);

impl DbIoPageBacking {
    pub const fn new() -> Self {
        Self(std::cell::UnsafeCell::new([0; DB_IO_PAGE_BYTES]))
    }
}

unsafe impl Sync for DbIoPageBacking {}

static DB_IO_PAGE_BACKINGS: [DbIoPageBacking; DB_IO_TOTAL_PAGES] = [const { DbIoPageBacking::new() }; DB_IO_TOTAL_PAGES];

#[derive(Clone, Copy)]
struct DbIoPageSlot {
    pub generation: u64,
    pub operation: u64,
    pub phase: DbIoPagePhase,
}

pub const EMPTY_DB_IO_PAGE_SLOT: DbIoPageSlot = DbIoPageSlot { generation: 0, operation: 0, phase: DbIoPagePhase::Free };

pub struct DbIoPageArenaState {
    pub slots: [DbIoPageSlot; DB_IO_TOTAL_PAGES],
    pub free: [u16; DB_IO_TOTAL_PAGES],
    pub free_read: usize,
    pub free_len: usize,
    pub retired: [Option<(u16, u64)>; DB_IO_PAGE_RETIREMENT_SLOTS],
    pub retired_read: usize,
    pub retired_len: usize,
    pub next_generation: u64,
}

impl DbIoPageArenaState {
    pub fn new() -> Self {
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

pub fn db_io_page_arena() -> &'static std::sync::Mutex<DbIoPageArenaState> {
    DB_IO_PAGE_ARENA.get_or_init(|| std::sync::Mutex::new(DbIoPageArenaState::new()))
}

pub fn db_io_page_credit(count: usize) -> DbIoCredit {
    DbIoCredit { pages: count, bytes: (count * DB_IO_PAGE_BYTES) as u64, items: count, controls: 0 }
}

fn db_io_page_shell_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: 0, items: 1, controls: 1 }
}

#[derive(Debug)]
struct DbIoPageLease {
    pub slot: u16,
    pub generation: u64,
    pub operation: u64,
    used: u16,
    pub returned: bool,
}

impl DbIoPageLease {
    fn validate_identity(&self, slot: DbIoPageSlot) -> Result<(), DbError> {
        if slot.generation != self.generation {
            return Err(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(slot.generation) });
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

    pub fn phase(&self) -> Result<DbIoPagePhase, DbError> {
        let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = state.slots[self.slot as usize];
        self.validate_identity(slot)?;
        Ok(slot.phase)
    }

    pub fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        self.validate_phase(*slot, expected)?;
        slot.phase = next;
        Ok(())
    }

    pub fn bytes(&self) -> &[u8] {
        let used = usize::from(self.used);
        unsafe { &(&*DB_IO_PAGE_BACKINGS[self.slot as usize].0.get())[..used] }
    }

    pub fn write(&mut self, offset: usize, source: &[u8]) -> Result<(), DbError> {
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

pub fn db_io_preflight_page_checkout(state: &DbIoPageArenaState, count: usize) -> Result<(), DbError> {
    if count > state.free_len || state.next_generation.checked_add(count as u64).is_none() {
        return Err(DbError::Unavailable(DB_IO_PROCESS_PAGES_EXHAUSTED.to_string()));
    }
    Ok(())
}

/// ✍️ Retained writer backed only by exact fixed arena page leases.
pub struct DbIoPageWriter {
    pub operation: u64,
    pub pages: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES],
    pub reserved: u8,
    pub cursor: u8,
    pub total_len: usize,
    shell_returned: bool,
    seal_phase: u8,
    seal_page: u8,
    seal_visible: u8,
    seal_current: Option<DbIoPagePhase>,
}

#[derive(Debug)]
pub struct DbIoPageWriterRejected {
    pub error: DbError,
    pub writer: Option<DbIoPageWriter>,
}

/// 🧵 One retained unused-page retirement opportunity per poll before writer publication.
pub struct DbIoPageWriterSeal {
    pub writer: Option<DbIoPageWriter>,
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

    #[cfg(any(test, feature = "testkit"))]
    pub fn seal(mut self) -> Result<DbIoPages, DbIoPageWriterRejected> {
        match self.finish() {
            Ok(pages) => Ok(pages),
            Err(error) => Err(DbIoPageWriterRejected { error, writer: Some(self) }),
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn finish(&mut self) -> Result<DbIoPages, DbError> {
        loop {
            if let Some(owner) = self.seal_retained_step()? {
                return Ok(owner);
            }
        }
    }

    /// 🪡 One retained validation, unused-page retirement, or atomic transition-and-publication opportunity.
    pub fn seal_retained_step(&mut self) -> Result<Option<DbIoPages>, DbError> {
        match self.seal_phase {
            0 if self.reserved > self.seal_visible => {
                self.seal_visible = if self.total_len == 0 { 0 } else { self.total_len.div_ceil(DB_IO_PAGE_BYTES) as u8 };
                if self.reserved <= self.seal_visible {
                    return Ok(None);
                }
                let index = usize::from(self.reserved - 1);
                let page = self.pages[index].as_ref().ok_or_else(|| DbError::Internal("DB I/O retained writer lost an unused page".to_string()))?;
                let phase = page.phase()?;
                if !matches!(phase, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
                    return Err(DbError::Internal(format!("DB I/O unused writer page phase mismatch: expected CheckedOutWriter or Executing, got {phase:?} for operation {} generation {}", page.operation, page.generation)));
                }
                page.transition(phase, DbIoPagePhase::Closing)?;
                let page = self.pages[index].take().ok_or_else(|| DbError::Internal("DB I/O retained writer lost an unused page".to_string()))?;
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

    pub fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
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

/// 🎟️ Pre-admitted capacity for one external-driver allocation owned by an aggregate operation.
pub struct DbIoDriverReservation {
    pub operation: u64,
    pub credit: DbIoCredit,
    pub returned: bool,
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

/// 🏷️ Exact fixed post-admission artifact identity retained without a heap string.
pub struct DbIoArtifactId {
    pub value: DbIoText,
    driver: Option<ArtifactId>,
    pub external: Option<DbIoExternalBytes>,
    pub reservation: Option<DbIoDriverReservation>,
    pub phase: u8,
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

/// 🧳 Retained external allocation with one page-content or backing-release close opportunity.
pub struct DbIoExternalBytes {
    pub value: Option<Vec<u8>>,
    pub phase: u8,
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

/// 📄 Exact ordered fixed-page leases with a zero-copy movable range cursor.
#[derive(Debug)]
pub struct DbIoPages {
    pub operation: u64,
    pub pages: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES],
    pub retained: u8,
    visible: u8,
    first_offset: usize,
    pub total_len: usize,
    shell_returned: bool,
    pub result_handback: Option<DbIoResultHandback>,
}

impl DbIoPages {
    pub fn operation(&self) -> u64 {
        self.operation
    }

    pub fn take_for_async_driver(&mut self) -> Self {
        let operation = self.operation;
        std::mem::replace(self, Self { operation, pages: std::array::from_fn(|_| None), retained: 0, visible: 0, first_offset: 0, total_len: 0, shell_returned: true, result_handback: None })
    }

    pub fn transfer_to_backend(&mut self, backend_operation: u64) -> Result<(), DbError> {
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
                return Err(DbError::StaleGeneration { expected: GenerationId(page.generation), actual: GenerationId(owner.generation) });
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

    pub fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
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

    pub fn admit(&self) -> Result<(), DbError> {
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
            if let Some(handback) = self.result_handback {
                db_io_result_handback(handback)?;
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
    pub pages: &'a DbIoPages,
    pub cursor: u8,
}

impl<'a> Iterator for DbIoPageReader<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let fragment = self.pages.page(self.cursor)?;
        self.cursor += 1;
        Some(fragment)
    }
}

pub const DB_IO_TEXT_BYTES: usize = 1024;

/// 📋️ The exact capacity of one [`DbIoU64List`]: a backend list longer than this is refused.
pub const DB_IO_LIST_ITEMS: usize = 4096;

/// 🔤 Fixed repository-owned path, document, key or fault text.
#[derive(Clone, PartialEq, Eq)]
pub struct DbIoText {
    pub bytes: [u8; DB_IO_TEXT_BYTES],
    pub len: u16,
}

impl DbIoText {
    pub fn new() -> Self {
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

/// 🔢 Bounded heap-backed typed list result with incremental close ownership.
pub struct DbIoU64List {
    pub values: Option<Box<[u64]>>,
    pub len: u16,
    pub result_handback: Option<DbIoResultHandback>,
}

impl DbIoU64List {
    pub fn retained_bytes(&self) -> u64 {
        (size_of::<Self>() + self.values.as_ref().map_or(0, |values| size_of_val(values.as_ref()))) as u64
    }

    pub fn new() -> Self {
        Self { values: None, len: 0, result_handback: None }
    }

    pub fn take_for_result(&mut self) -> Self {
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

    /// 🔢️ Orders the listed values ascending in place, for a backend whose enumeration has no order.
    pub fn sort_ascending(&mut self) {
        let len = usize::from(self.len);
        if let Some(values) = self.values.as_deref_mut() {
            values[..len].sort_unstable();
        }
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
            if let Some(handback) = self.result_handback {
                if db_io_result_handback(handback).is_ok() {
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

/// 🧭 Repository-owned backend identity; external driver values never cross this boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoBackendControl {
    Memory { slot: u16, generation: u64 },
    Filesystem { slot: u16, generation: u64 },
    Sqlite { slot: u16, generation: u64 },
    Postgres { slot: u16, generation: u64 },
    Neo4j { slot: u16, generation: u64 },
}

/// 🚦️ Persisted lifecycle state of one WAL segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalSegmentState {
    Active,
    Sealed,
}

/// 🗂️ Schema-first database I/O task owner.
pub enum DbIoTask {
    BackendOpen { backend: DbIoBackendControl, path: DbIoText },
    WalWriterAcquire { backend: DbIoBackendControl, document: DbIoText },
    WalCreate { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64 },
    WalAppend { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64, input: DbIoPages },
    WalSync { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64, class: DurabilityClass },
    WalSeal { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64 },
    WalRead { backend: DbIoBackendControl, document: DbIoText, index: u64, range: ByteRange, output: DbIoPageWriter },
    WalLength { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalState { backend: DbIoBackendControl, document: DbIoText, index: u64 },
    WalList { backend: DbIoBackendControl, document: DbIoText, output: DbIoU64List },
    WalTruncate { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64, new_len: u64 },
    WalDelete { backend: DbIoBackendControl, document: DbIoText, writer: WalWriterKey, index: u64 },
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

/// 📬 Exact typed database I/O terminal result.
pub enum DbIoResult {
    Unit,
    WalWriter(WalWriterPermit),
    Length(u64),
    WalSegmentState(WalSegmentState),
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
    pub result_handback: Option<DbIoResultHandback>,
}

impl DbIoLeaseResult {
    pub fn new(resource: DbIoText, holder: DbIoText, fence: EpochFence, expires_at_ms: u64) -> Self {
        Self { resource, holder, fence, expires_at_ms, result_handback: None }
    }

    pub fn handback_step(&mut self) -> bool {
        let Some(handback) = self.result_handback else { return false };
        if db_io_result_handback(handback).is_err() {
            return false;
        }
        self.result_handback = None;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
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
//#region 🔖️DbIoCensus
/// 🏷️ One [`DbIoTask`] variant, as the process-wide [`db_io_census`] counts it: the cost model
/// of every storage call in typed tasks and in executor steps (one worker-pool turn each).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DbIoTaskKind {
    BackendOpen,
    WalWriterAcquire,
    WalCreate,
    WalAppend,
    WalSync,
    WalSeal,
    WalRead,
    WalLength,
    WalState,
    WalList,
    WalTruncate,
    WalDelete,
    SnapshotWrite,
    SnapshotRead,
    SnapshotLatest,
    SnapshotList,
    SnapshotDelete,
    PayloadPut,
    PayloadGet,
    PayloadExists,
    PayloadLength,
    PayloadDelete,
    CatalogRead,
    CatalogCas,
    IndexWrite,
    IndexRead,
    IndexList,
    IndexDelete,
    LeaseAcquire,
    LeaseRenew,
    LeaseRelease,
    LeaseGet,
    BackendClose,
}

impl DbIoTaskKind {
    pub const ALL: [DbIoTaskKind; 33] = [
        Self::BackendOpen,
        Self::WalWriterAcquire,
        Self::WalCreate,
        Self::WalAppend,
        Self::WalSync,
        Self::WalSeal,
        Self::WalRead,
        Self::WalLength,
        Self::WalState,
        Self::WalList,
        Self::WalTruncate,
        Self::WalDelete,
        Self::SnapshotWrite,
        Self::SnapshotRead,
        Self::SnapshotLatest,
        Self::SnapshotList,
        Self::SnapshotDelete,
        Self::PayloadPut,
        Self::PayloadGet,
        Self::PayloadExists,
        Self::PayloadLength,
        Self::PayloadDelete,
        Self::CatalogRead,
        Self::CatalogCas,
        Self::IndexWrite,
        Self::IndexRead,
        Self::IndexList,
        Self::IndexDelete,
        Self::LeaseAcquire,
        Self::LeaseRenew,
        Self::LeaseRelease,
        Self::LeaseGet,
        Self::BackendClose,
    ];
}
//#endregion 🔖️DbIoCensus

impl DbIoTask {
    pub fn kind(&self) -> DbIoTaskKind {
        match self {
            Self::BackendOpen { .. } => DbIoTaskKind::BackendOpen,
            Self::WalWriterAcquire { .. } => DbIoTaskKind::WalWriterAcquire,
            Self::WalCreate { .. } => DbIoTaskKind::WalCreate,
            Self::WalAppend { .. } => DbIoTaskKind::WalAppend,
            Self::WalSync { .. } => DbIoTaskKind::WalSync,
            Self::WalSeal { .. } => DbIoTaskKind::WalSeal,
            Self::WalRead { .. } => DbIoTaskKind::WalRead,
            Self::WalLength { .. } => DbIoTaskKind::WalLength,
            Self::WalState { .. } => DbIoTaskKind::WalState,
            Self::WalList { .. } => DbIoTaskKind::WalList,
            Self::WalTruncate { .. } => DbIoTaskKind::WalTruncate,
            Self::WalDelete { .. } => DbIoTaskKind::WalDelete,
            Self::SnapshotWrite { .. } => DbIoTaskKind::SnapshotWrite,
            Self::SnapshotRead { .. } => DbIoTaskKind::SnapshotRead,
            Self::SnapshotLatest { .. } => DbIoTaskKind::SnapshotLatest,
            Self::SnapshotList { .. } => DbIoTaskKind::SnapshotList,
            Self::SnapshotDelete { .. } => DbIoTaskKind::SnapshotDelete,
            Self::PayloadPut { .. } => DbIoTaskKind::PayloadPut,
            Self::PayloadGet { .. } => DbIoTaskKind::PayloadGet,
            Self::PayloadExists { .. } => DbIoTaskKind::PayloadExists,
            Self::PayloadLength { .. } => DbIoTaskKind::PayloadLength,
            Self::PayloadDelete { .. } => DbIoTaskKind::PayloadDelete,
            Self::CatalogRead { .. } => DbIoTaskKind::CatalogRead,
            Self::CatalogCas { .. } => DbIoTaskKind::CatalogCas,
            Self::IndexWrite { .. } => DbIoTaskKind::IndexWrite,
            Self::IndexRead { .. } => DbIoTaskKind::IndexRead,
            Self::IndexList { .. } => DbIoTaskKind::IndexList,
            Self::IndexDelete { .. } => DbIoTaskKind::IndexDelete,
            Self::LeaseAcquire { .. } => DbIoTaskKind::LeaseAcquire,
            Self::LeaseRenew { .. } => DbIoTaskKind::LeaseRenew,
            Self::LeaseRelease { .. } => DbIoTaskKind::LeaseRelease,
            Self::LeaseGet { .. } => DbIoTaskKind::LeaseGet,
            Self::BackendClose { .. } => DbIoTaskKind::BackendClose,
        }
    }


    pub fn writer_stamp(&self) -> Option<(WalWriterKey, DbIoBackendControl, &DbIoText)> {
        match self {
            Self::WalCreate { writer, backend, document, .. }
            | Self::WalAppend { writer, backend, document, .. }
            | Self::WalSync { writer, backend, document, .. }
            | Self::WalSeal { writer, backend, document, .. }
            | Self::WalTruncate { writer, backend, document, .. }
            | Self::WalDelete { writer, backend, document, .. } => Some((*writer, *backend, document)),
            _ => None,
        }
    }

    pub fn backend(&self) -> DbIoBackendControl {
        match self {
            Self::BackendOpen { backend, .. }
            | Self::WalWriterAcquire { backend, .. }
            | Self::WalCreate { backend, .. }
            | Self::WalAppend { backend, .. }
            | Self::WalSync { backend, .. }
            | Self::WalSeal { backend, .. }
            | Self::WalRead { backend, .. }
            | Self::WalLength { backend, .. }
            | Self::WalState { backend, .. }
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

    pub fn operation(&self) -> Option<u64> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => Some(input.operation()),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => Some(output.operation()),
            _ => None,
        }
    }

    pub fn aggregate_credit(&self) -> DbIoCredit {
        let (bytes, items) = match self {
            Self::WalList { .. } | Self::SnapshotLatest { .. } | Self::SnapshotList { .. } | Self::IndexList { .. } => (DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES, DB_IO_LIST_ITEMS * 2 + 1),
            _ => (DB_IO_TASK_SLOT_BYTES, 1),
        };
        DbIoCredit { pages: 0, bytes, items, controls: 1 }
    }

    pub fn admit_list_backing(&mut self) -> Result<(), DbError> {
        match self {
            Self::WalList { output, .. } | Self::SnapshotLatest { output, .. } | Self::SnapshotList { output, .. } | Self::IndexList { output, .. } if output.is_empty() => output.ensure_backing(),
            Self::WalList { .. } | Self::SnapshotLatest { .. } | Self::SnapshotList { .. } | Self::IndexList { .. } => Err(DbError::Internal("DB I/O list output was not empty at admission".to_string())),
            _ => Ok(()),
        }
    }

    pub fn release_unstarted_list_backing(&mut self) {
        match self {
            Self::WalList { output, .. } | Self::SnapshotLatest { output, .. } | Self::SnapshotList { output, .. } | Self::IndexList { output, .. } if output.is_empty() => output.values = None,
            _ => {}
        }
    }

    pub fn admit_pages(&self) -> Result<(), DbError> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => input.admit(),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => {
                output.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued)
            }
            _ => Ok(()),
        }
    }

    pub fn transition_pages(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        match self {
            Self::WalAppend { input, .. } | Self::SnapshotWrite { input, .. } | Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } | Self::IndexWrite { input, .. } => input.transition(expected, next),
            Self::WalRead { output, .. } | Self::SnapshotRead { output, .. } | Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } | Self::IndexRead { output, .. } => output.transition(expected, next),
            _ => Ok(()),
        }
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
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
            Self::WalWriterAcquire { document, .. }
            | Self::WalCreate { document, .. }
            | Self::WalSync { document, .. }
            | Self::WalSeal { document, .. }
            | Self::WalLength { document, .. }
            | Self::WalState { document, .. }
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

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::WalAppend { document, input, .. } | Self::SnapshotWrite { document, input, .. } | Self::IndexWrite { document, input, .. } => document.terminal_is_empty() && input.terminal_is_empty(),
            Self::PayloadPut { input, .. } | Self::CatalogCas { input, .. } => input.terminal_is_empty(),
            Self::WalRead { document, output, .. } | Self::SnapshotRead { document, output, .. } | Self::IndexRead { document, output, .. } => document.terminal_is_empty() && output.terminal_is_empty(),
            Self::PayloadGet { output, .. } | Self::CatalogRead { output, .. } => output.terminal_is_empty(),
            Self::BackendOpen { path, .. } => path.terminal_is_empty(),
            Self::WalList { document, output, .. } | Self::SnapshotLatest { document, output, .. } | Self::SnapshotList { document, output, .. } | Self::IndexList { document, output, .. } => document.terminal_is_empty() && output.terminal_is_empty(),
            Self::WalWriterAcquire { document, .. }
            | Self::WalCreate { document, .. }
            | Self::WalSync { document, .. }
            | Self::WalSeal { document, .. }
            | Self::WalLength { document, .. }
            | Self::WalState { document, .. }
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
    pub fn retained_credit(&self) -> DbIoCredit {
        match self {
            Self::List(list) if list.values.is_some() => DbIoCredit { pages: 0, bytes: DB_IO_LIST_BACKING_BYTES, items: DB_IO_LIST_ITEMS, controls: 0 },
            _ => DbIoCredit::default(),
        }
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        match self {
            Self::WalWriter(_) => {
                drop(std::mem::replace(self, Self::Unit));
                Ok(Some(0))
            }
            Self::Pages(pages) => pages.close_step(),
            Self::OptionalCatalog(Some((pages, _))) => pages.close_step(),
            Self::List(list) => Ok(list.close_step().then_some(0)),
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => {
                if lease.holder.close_step() || lease.resource.close_step() || lease.handback_step() {
                    return Ok(Some(0));
                }
                Ok(None)
            }
            Self::Unit | Self::Length(_) | Self::WalSegmentState(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => Ok(None),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::WalWriter(_) => false,
            Self::Pages(pages) => pages.terminal_is_empty(),
            Self::OptionalCatalog(Some((pages, _))) => pages.terminal_is_empty(),
            Self::List(list) => list.terminal_is_empty(),
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => lease.terminal_is_empty(),
            Self::Unit | Self::Length(_) | Self::WalSegmentState(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => true,
        }
    }

    fn attach_result_handback(&mut self, handback: DbIoResultHandback) -> bool {
        match self {
            Self::WalWriter(_) => false,
            Self::Pages(pages) => {
                pages.result_handback = Some(handback);
                true
            }
            Self::OptionalCatalog(Some((pages, _))) => {
                pages.result_handback = Some(handback);
                true
            }
            Self::List(list) => {
                list.result_handback = Some(handback);
                true
            }
            Self::Lease(lease) | Self::OptionalLease(Some(lease)) => {
                lease.result_handback = Some(handback);
                true
            }
            Self::Unit | Self::Length(_) | Self::WalSegmentState(_) | Self::OptionalLength(_) | Self::Exists(_) | Self::Hash(_) | Self::Fence(_) | Self::OptionalCatalog(None) | Self::OptionalLease(None) => false,
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

/// 🎁️ What an async-native driver hands back when its task reaches a terminal: the executor the
/// backend registry lends out for the turn, the typed task owner, and the terminal itself.
pub type DbIoAsyncDriverOutput = (Box<dyn DbIoTaskExecutor>, DbIoTask, Result<DbIoResult, DbError>);

pub type DbIoAsyncDriverFuture = Pin<Box<dyn Future<Output = DbIoAsyncDriverOutput> + Send + 'static>>;

/// 🧵️ The one seam where a driver bound to a foreign executor may be polled.
///
/// `Lane::Io` of the process [`WorkerPool`] is plain `std::thread`s with no foreign runtime context
/// of any kind — that is the whole point of `semio_framework_async` ("No `tokio` in this crate").
/// A driver like `sqlx` or `neo4rs` demands ITS runtime's thread-local context at **poll** time, not
/// merely at construction time, so polling such a future on a pool worker aborts the process
/// (`this functionality requires a Tokio context`). A backend whose driver has that requirement
/// therefore owns a bounded runtime of its own and returns it here; [`DbIoAsyncTaskLease::start_on_lane_io`]
/// then never puts the driver's future in the slot at all. What the slot gets instead is the
/// **bridge** this returns: a repo-owned rendezvous (`semio_framework_async::oneshot`) that is safe
/// to poll anywhere, because the only thing it does is read a value and register a waker.
///
/// This is an interface, not an implementation: nothing in `db_storage` names a concrete runtime.
pub trait DbIoAsyncDriverRuntime: Send + Sync {
    /// 🚚 Take ownership of `future`, drive it to completion on a thread this runtime owns, and
    /// return the `Lane::Io`-safe bridge future that resolves to the same output.
    fn detach(&self, future: DbIoAsyncDriverFuture) -> DbIoAsyncDriverFuture;
}

/// 👣️ One writer-guard release opportunity either progressed or waits for an exact future wake.
pub enum DbIoWriterReleaseStep {
    More,
    Faulted,
    Idle,
}

/// 🔌 Platform drivers implement one typed, resumable task step behind repository owners.
pub trait DbIoTaskExecutor: Send + Sync {
    fn supports_writer_authority(&self) -> bool {
        false
    }

    fn bind_writer_control(&mut self, _control: DbIoBackendControl) -> Result<(), DbError> {
        Ok(())
    }

    fn writer_release_step(&self, _context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
        Ok(DbIoWriterReleaseStep::Idle)
    }

    fn pin_writer_operation(&self, _operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        if task.writer_stamp().is_some() {
            Err(DbError::Unavailable("backend has no WAL writer fence".to_string()))
        } else {
            Ok(())
        }
    }

    fn finish_writer_operation(&self, _operation: u64, _task: &DbIoTask) -> Result<(), DbError> {
        Ok(())
    }

    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::BlockingLane
    }

    /// 🧵️ The runtime this executor's driver must be polled on, when its driver is bound to one.
    /// `None` — the default, and the answer for every backend in this crate — means the future
    /// [`Self::drive_async`] returns is safe to poll on `Lane::Io` directly.
    fn driver_runtime(&self) -> Option<&'static dyn DbIoAsyncDriverRuntime> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn bind_owner_operation(&mut self, _operation: u64) -> Result<(), DbError> {
        Ok(())
    }

    fn owner_backing_bytes(&self) -> u64 {
        size_of_val(self) as u64
    }

    fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError>;

    fn drive_async(self: Box<Self>, operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture;

    fn close_operation_step(&self, operation: u64, task: &DbIoTask) -> Result<bool, DbError>;

    fn close_backend_step(&mut self, context: &mut std::task::Context<'_>) -> Result<bool, DbError>;

    fn backend_terminal_is_empty(&self) -> bool;
}

pub fn db_io_backend_parts(control: DbIoBackendControl) -> (u16, u64) {
    match control {
        DbIoBackendControl::Memory { slot, generation }
        | DbIoBackendControl::Filesystem { slot, generation }
        | DbIoBackendControl::Sqlite { slot, generation }
        | DbIoBackendControl::Postgres { slot, generation }
        | DbIoBackendControl::Neo4j { slot, generation } => (slot, generation),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoFaultKind {
    Backend,
    Cancelled,
    Panic,
    Saturated,
    Stale,
}

/// 🧩 Bounded storage-error taxonomy retained independently from runner provenance.
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
    pub fn from_db_error(error: &DbError) -> Self {
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
    pub result_handback: Option<DbIoResultHandback>,
}

impl std::fmt::Debug for DbIoFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbIoFault").field("kind", &self.kind).field("cause", &self.cause).field("detail", &self.detail.as_str()).field("has_result_handback", &self.result_handback.is_some()).finish()
    }
}

impl DbIoFault {
    pub fn into_db_error(mut self) -> DbError {
        let detail = self.detail.as_str().to_string();
        let error = match self.cause {
            DbIoFaultCause::Io => DbError::Io(detail),
            DbIoFaultCause::NotFound => DbError::NotFound(detail),
            DbIoFaultCause::AlreadyExists => DbError::AlreadyExists(detail),
            DbIoFaultCause::InvalidArgument => DbError::InvalidArgument(detail),
            DbIoFaultCause::LimitExceeded(detail) => DbError::LimitExceeded(detail),
            DbIoFaultCause::Conflict => DbError::Conflict(detail),
            DbIoFaultCause::Fenced { expected, actual } => DbError::Fenced { expected, actual },
            DbIoFaultCause::StaleGeneration { expected, actual } => DbError::StaleGeneration { expected: GenerationId(expected), actual: GenerationId(actual) },
            DbIoFaultCause::Unavailable => DbError::Unavailable(detail),
            DbIoFaultCause::Timeout => DbError::Timeout(detail),
            DbIoFaultCause::Corrupt => DbError::Corrupt(detail),
            DbIoFaultCause::Closed => DbError::Closed,
            DbIoFaultCause::Unauthorized => DbError::Unauthorized(detail),
            DbIoFaultCause::Unimplemented(detail) => DbError::Unimplemented(detail),
            DbIoFaultCause::Internal => DbError::Internal(detail),
        };
        while self.close_step() {}
        error
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

pub enum DbIoTerminal {
    Result(DbIoResult),
    Fault(DbIoFault),
    Cancelled(Option<DbIoResult>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DbIoTaskHandle {
    pub slot: u16,
    pub generation: u64,
    pub operation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DbIoResultHandback {
    pub handle: DbIoTaskHandle,
    pub retained_credit: DbIoCredit,
}

pub struct DbIoTaskSlot {
    pub generation: u64,
    pub operation: u64,
    pub phase: DbIoTaskPhase,
    pub task: Option<DbIoTask>,
    pub terminal: Option<DbIoTerminal>,
    pub pool: Option<Arc<WorkerPool>>,
    pub retry_attempt: Option<u8>,
    pub terminal_submit_kind: Option<WorkerSubmitErrorKind>,
    pub waker: Option<std::task::Waker>,
    pub retry_generation: u64,
    pub cancelled: bool,
    pub abandoned: bool,
    pub close_enqueued: bool,
    pub backend_cleanup_done: bool,
    pub backend_to_close: Option<DbIoBackendControl>,
    pub backend: Option<DbIoBackendControl>,
    pub counted: bool,
    pub aggregate_credit: DbIoCredit,
    pub result_retained_credit: DbIoCredit,
    pub aggregate_returned: bool,
    pub async_ready: bool,
    pub async_detached: bool,
    pub async_lane_turn: bool,
    pub backend_admitted: bool,
    pub async_driver: Option<DbIoAsyncDriverFuture>,
    pub async_driver_scheduled: bool,
    pub async_driver_wake_requested: bool,
}

impl DbIoTaskSlot {
    pub const fn empty() -> Self {
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
            result_retained_credit: DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 0 },
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

pub static DB_IO_TASK_SLOTS: [std::sync::Mutex<DbIoTaskSlot>; DB_IO_OPERATION_ITEMS] = [const { std::sync::Mutex::new(DbIoTaskSlot::empty()) }; DB_IO_OPERATION_ITEMS];

struct DbIoTaskArena {
    pub free: [u16; DB_IO_OPERATION_ITEMS],
    pub free_read: usize,
    pub free_len: usize,
    pub closing: [Option<DbIoTaskHandle>; DB_IO_OPERATION_ITEMS],
    pub closing_read: usize,
    pub closing_len: usize,
    pub next_generation: u64,
}

impl DbIoTaskArena {
    pub fn new() -> Self {
        Self { free: std::array::from_fn(|index| index as u16), free_read: 0, free_len: DB_IO_OPERATION_ITEMS, closing: [None; DB_IO_OPERATION_ITEMS], closing_read: 0, closing_len: 0, next_generation: 1 }
    }
}

static DB_IO_TASK_ARENA: std::sync::OnceLock<std::sync::Mutex<DbIoTaskArena>> = std::sync::OnceLock::new();

pub fn db_io_task_arena() -> &'static std::sync::Mutex<DbIoTaskArena> {
    DB_IO_TASK_ARENA.get_or_init(|| std::sync::Mutex::new(DbIoTaskArena::new()))
}

pub fn db_io_text_literal(value: &'static str) -> DbIoText {
    match DbIoText::try_from_str(value) {
        Ok(text) => text,
        Err(_) => DbIoText::new(),
    }
}

pub fn db_io_error_text(error: &DbError) -> DbIoText {
    DbIoText::try_from_str(&error.to_string()).unwrap_or_else(|_| db_io_text_literal("DB I/O error detail exceeded fixed authority"))
}

pub fn db_io_slot_matches(slot: &DbIoTaskSlot, handle: DbIoTaskHandle) -> bool {
    slot.generation == handle.generation && slot.operation == handle.operation && handle.generation != 0 && handle.operation != 0
}

pub fn db_io_enqueue_close(handle: DbIoTaskHandle) -> Result<(), DbError> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !db_io_slot_matches(&owner, handle) {
        return Ok(());
    }
    if owner.close_enqueued {
        let backend = owner.backend;
        drop(owner);
        drop(arena);
        if let Some(backend) = backend {
            crate::request_controller(backend);
        }
        return Ok(());
    }
    if arena.closing_len == DB_IO_OPERATION_ITEMS {
        return Err(DbError::Unavailable("DB I/O task close arena retained every admitted task".to_string()));
    }
    owner.close_enqueued = true;
    let backend = owner.backend.or_else(|| owner.task.as_ref().map(DbIoTask::backend));
    drop(owner);
    let write = (arena.closing_read + arena.closing_len) % DB_IO_OPERATION_ITEMS;
    arena.closing[write] = Some(handle);
    arena.closing_len += 1;
    drop(arena);
    if let Some(backend) = backend {
        crate::request_controller(backend);
    }
    Ok(())
}

/// 🎟️ Generation-qualified terminal lease retained by its aggregate operation.
pub struct DbIoResultLease {
    pub handle: DbIoTaskHandle,
    pub retained_credit: DbIoCredit,
    pub result: Option<DbIoResult>,
    pub transferred: bool,
}

pub fn db_io_result_lease_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: size_of::<DbIoResultLease>() as u64, items: 1, controls: 1 }
}

pub const DB_IO_LOST_OWNER_SLOTS: usize = DB_IO_TOTAL_PAGES + DB_IO_PROCESS_CONTROL_CREDIT;

pub const DB_IO_LOST_OWNER_OVERFLOW_SLOTS: usize = DB_IO_OPERATION_ITEMS;

pub enum DbIoLostOwner {
    PageWriter(DbIoPageWriter),
    Pages(DbIoPages),
    List(DbIoU64List),
    Lease(DbIoLeaseResult),
    Fault(DbIoFault),
    DriverReservation(DbIoDriverReservation),
    ExternalBytes(DbIoExternalBytes),
    ArtifactId(DbIoArtifactId),
    Backend { owner: Option<Box<dyn DbIoTaskExecutor>>, operation: u64, credit: DbIoCredit, pool: Option<Arc<WorkerPool>>, pool_use: Option<Arc<WorkerPoolUse>> },
    ResultLease { handback: DbIoResultHandback, result: Option<DbIoResult> },
}

pub static DB_IO_LOST_OWNERS: std::sync::Mutex<[Option<DbIoLostOwner>; DB_IO_LOST_OWNER_SLOTS]> = std::sync::Mutex::new([const { None }; DB_IO_LOST_OWNER_SLOTS]);

pub static DB_IO_LOST_OWNER_OVERFLOW: std::sync::Mutex<[Option<DbIoLostOwner>; DB_IO_LOST_OWNER_OVERFLOW_SLOTS]> = std::sync::Mutex::new([const { None }; DB_IO_LOST_OWNER_OVERFLOW_SLOTS]);

pub static DB_IO_LOST_OWNER_QUARANTINE: std::sync::Mutex<[Option<DbIoLostOwner>; DB_IO_LOST_OWNER_OVERFLOW_SLOTS]> = std::sync::Mutex::new([const { None }; DB_IO_LOST_OWNER_OVERFLOW_SLOTS]);

pub static DB_IO_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub static DB_IO_LOST_OWNER_WAKE_REQUESTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn db_io_request_lost_owner_maintenance() {
    if !DB_IO_LOST_OWNER_WAKE_REQUESTED.swap(true, std::sync::atomic::Ordering::AcqRel) {
        crate::request_any_controller();
    }
}

pub fn db_io_try_park_lost_owner(owner: DbIoLostOwner) -> Result<(), DbIoLostOwner> {
    let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(slot) = owners.iter_mut().find(|slot| slot.is_none()) {
        *slot = Some(owner);
        return Ok(());
    }
    Err(owner)
}

pub fn db_io_park_lost_owner(owner: DbIoLostOwner) -> Result<(), DbIoLostOwner> {
    let owner = match db_io_try_park_lost_owner(owner) {
        Ok(()) => {
            db_io_request_lost_owner_maintenance();
            return Ok(());
        }
        Err(owner) => owner,
    };
    DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
    let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(slot) = overflow.iter_mut().find(|slot| slot.is_none()) {
        *slot = Some(owner);
        drop(overflow);
        db_io_request_lost_owner_maintenance();
        return Ok(());
    }
    drop(overflow);
    let mut quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = quarantine.iter_mut().find(|slot| slot.is_none()) else { return Err(owner) };
    *slot = Some(owner);
    drop(quarantine);
    db_io_request_lost_owner_maintenance();
    Ok(())
}

impl DbIoResultLease {
    pub fn into_result(mut self) -> Result<DbIoResult, DbError> {
        let mut result = self.result.take().ok_or_else(|| DbError::Internal("DB I/O result lease consumed twice".to_string()))?;
        let handback = DbIoResultHandback { handle: self.handle, retained_credit: self.retained_credit };
        if !result.attach_result_handback(handback) {
            db_io_result_handback(handback)?;
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
            let handback = DbIoResultHandback { handle: self.handle, retained_credit: self.retained_credit };
            if let Err(DbIoLostOwner::ResultLease { handback, result }) = db_io_park_lost_owner(DbIoLostOwner::ResultLease { handback, result: self.result.take() }) {
                self.handle = handback.handle;
                self.retained_credit = handback.retained_credit;
                self.result = result;
            }
        }
        let _ = db_io_enqueue_close(self.handle);
    }
}

pub fn db_io_result_handback(handback: DbIoResultHandback) -> Result<(), DbError> {
    db_io_operation_return_result_lease(handback.handle.operation, handback.retained_credit)?;
    db_io_enqueue_close(handback.handle)
}

pub const DB_IO_PROCESS_CREDIT_EXHAUSTED: &str = "DB I/O process aggregate credit exhausted";

pub const DB_IO_TASK_ADMISSION_EXHAUSTED: &str = "DB I/O aggregate task admission exhausted";

pub const DB_IO_PROCESS_PAGES_EXHAUSTED: &str = "db I/O process page capacity exhausted";

pub const DB_IO_BACKEND_CREDIT_EXHAUSTED: &str = "DB I/O backend process credit exhausted";

const DB_IO_ADMISSION_WAITERS: usize = 256;

pub static DB_IO_ADMISSION_EPOCH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub static DB_IO_ADMISSION_WAITING: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub static DB_IO_ADMISSION_WAITER_TABLE: std::sync::Mutex<AdmissionWaiters<DB_IO_ADMISSION_WAITERS>> = std::sync::Mutex::new(AdmissionWaiters::new());

/// 📣️ Records that DB I/O capacity was released and wakes every task waiting for admission (none: two atomics).
pub fn db_io_admission_released() {
    DB_IO_ADMISSION_EPOCH.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    if DB_IO_ADMISSION_WAITING.load(std::sync::atomic::Ordering::Acquire) == 0 {
        return;
    }
    let wakers = DB_IO_ADMISSION_WAITER_TABLE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).wakers();
    wakers.into_iter().flatten().for_each(std::task::Waker::wake);
}
//#endregion 🔖️Limits
//#region 🔖️Capabilities
/// 🎚️ What a concrete `DbStorage` backend actually supports — negotiated once at
/// `Database::open` and folded into `DbCapabilities` alongside enabled Cargo features.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StorageCapabilities {
    /// 💾️ True iff data written through this backend survives a process restart.
    pub durable: bool,
    /// 🥇️ The strongest `DurabilityClass` this backend can actually deliver on `sync`.
    pub max_durability: DurabilityClass,
    /// 🔒️ True iff `WalStorage::sync`/equivalent can force data to physical storage.
    pub supports_fsync: bool,
    /// ✅️ True iff `CatalogStorage::cas_root`/`LeaseStorage` provide real compare-and-swap
    /// fencing (as opposed to a backend that could only ever serve a single writer).
    pub supports_cas: bool,
}
//#endregion 🔖️Capabilities
//#region 🔖️WalStorage
/// 📜️ Raw, per-document, per-segment append-only byte storage — `db_wal` frames its own
/// `.spr` records on top of what this trait stores; this trait never interprets a byte written
/// through it. A document's WAL is a sequence of segments identified by a dense `u64` index.
/// At most one segment is active; a transient all-sealed sequence is valid while its owner is
/// rotating to a new segment.
pub trait WalStorage: Send + Sync {
    /// 🔐️ Acquires exclusive document ownership before inventory, recovery, or mutation.
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError>;
    /// 🆕️ Creates a new, empty, unsealed segment `index` for `document`. Errors
    /// `AlreadyExists` if `index` already exists for `document`.
    async fn create_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;

    /// ➕️ Appends `bytes` to the active segment `index`, returning the segment's new total
    /// length. Errors `NotFound` if the segment doesn't exist, `InvalidArgument` if it is sealed.
    async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError>;

    /// 🔒️ Forces everything appended to segment `index` so far to the durability level
    /// implied by `class` — a no-op for `Memory`/`Os` (per `DurabilityClass`'s own
    /// doc: `Os` only promises "handed to the OS", not `fsync`ed), a real flush-to-disk for
    /// `Fsync`/`Quorum` (replication itself is `db_cluster`'s concern, not this trait's).
    async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError>;

    /// 🏁️ Marks segment `index` sealed: no further `append`/`truncate_tail` may target it.
    /// Errors `NotFound` if the segment doesn't exist. Idempotent if already sealed.
    async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;

    /// 📖️ Reads `range` of segment `index`'s bytes. Errors `NotFound` if the segment
    /// doesn't exist, `InvalidArgument` if `range` extends past the segment's current length.
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError>;

    /// 📏️ The current length in bytes of segment `index`.
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError>;

    /// 🚦️ Observes whether segment `index` accepts writes, without mutating storage.
    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError>;

    /// 📋️ Every segment index that exists for `document`, ascending. Empty (not an error)
    /// if `document` has no WAL yet.
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// ✂️ Truncates the ACTIVE (unsealed) segment `index` down to `new_len` bytes — the
    /// crash-recovery primitive for discarding a torn/uncommitted tail write. Errors
    /// `InvalidArgument` if the segment is sealed or if `new_len` exceeds its current length
    /// (this trait never extends a segment via truncation).
    async fn truncate_tail(&self, writer: &WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError>;

    /// 🗑️ Deletes segment `index` entirely (both its bytes and seal marker), e.g. after
    /// `db_compact` has folded it into a later generation. Idempotent if already absent.
    async fn delete_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;
}
//#endregion 🔖️WalStorage
//#region 🔖️SnapshotStorage
/// 📸️ Storage for whole snapshot generations — `db_snapshot` hands this trait complete
/// `.spk` pack-file bytes (pages in `KIND_CHUNK`, descriptor in `KIND_SNAPSHOT`) per generation;
/// this trait never parses them, it only persists and retrieves them by `(document, generation)`.
pub trait SnapshotStorage: Send + Sync {
    /// 🪪️ Identity of the concrete storage that owns these generations: two facades over the
    /// same backend report the same scope, so publication exclusion is per backend and document,
    /// never process-wide per document name.
    fn publication_scope(&self) -> usize;

    /// ✍️ Durably writes `bytes` as generation `generation` of `document`'s snapshot
    /// history. Overwrites if the same `(document, generation)` is written twice (the caller's
    /// responsibility to pick a fresh generation number per the contract's
    /// `Footer.prev_footer_offset` incremental-generation chain).
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError>;

    /// 📖️ Reads generation `generation`'s complete bytes. Errors `NotFound` if absent.
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError>;

    /// 🥇️ The highest generation number stored for `document`, or `None` if it has no
    /// snapshot yet.
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError>;

    /// 📋️ Every generation number stored for `document`, ascending.
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// 🗑️ Deletes generation `generation`, e.g. once `db_compact`'s retention policy
    /// supersedes it. Idempotent if already absent.
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError>;
}
//#endregion 🔖️SnapshotStorage
//#region 🔖️PayloadStorage
/// 🫙️ Content-addressed blob storage (blake3 CAS), shared across every document — large
/// command/payload bytes referenced from a WAL record or a snapshot page are stored once here and
/// referenced by `ContentHash` everywhere else, so identical payloads never duplicate on disk.
pub trait PayloadStorage: Send + Sync {
    /// ➕️ Stores `bytes` (if not already present — `put` is idempotent under content
    /// equality) and returns its `blake3` `ContentHash`.
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError>;

    /// 📖️ Reads the payload stored under `hash`. Errors `NotFound` if absent.
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError>;

    /// ❓️ True iff a payload is stored under `hash`, without reading it.
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError>;

    /// 🗑️ Deletes the payload stored under `hash` — `db_compact`'s ref-traced payload GC.
    /// Idempotent if already absent.
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError>;

    /// 📏️ The byte length of the payload stored under `hash`, without reading it. Errors
    /// `NotFound` if absent.
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError>;
}
//#endregion 🔖️PayloadStorage
//#region 🔖️CatalogStorage
/// 🗂️ The single catalog root blob (the family's document directory: names, ids,
/// metadata — opaque to this crate) with compare-and-swap-by-epoch writes: the split-brain gate
/// per `EpochFence`'s doc. Exactly one root exists per `DbStorage` instance.
pub trait CatalogStorage: Send + Sync {
    /// 📖️ The current root bytes and the `EpochFence` they were written under, or `None`
    /// if `cas_root` has never succeeded yet (a fresh, empty `DbStorage`).
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError>;

    /// ✅️ Compare-and-swap: succeeds only if `expected` matches the epoch of the currently
    /// stored root (or `EpochFence::INITIAL` if no root has ever been written), in which case the
    /// root becomes `new_bytes` under the next epoch (`expected.next()`), which is returned.
    /// Fails `DbError::Fenced` on any mismatch — a writer that lost leadership never silently
    /// overwrites a newer root.
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError>;
}
//#endregion 🔖️CatalogStorage
//#region 🔖️IndexStorage
/// 🔍️ Storage for `db_index`'s immutable sorted runs (LSM-lite) — opaque per-document,
/// per-run byte blobs; this trait has no opinion on what's inside a run.
pub trait IndexStorage: Send + Sync {
    /// ✍️ Durably writes `bytes` as run `run_id` of `document`'s index. Overwrites if the
    /// same `(document, run_id)` is written twice.
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError>;

    /// 📖️ Reads run `run_id`'s complete bytes. Errors `NotFound` if absent.
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError>;

    /// 📋️ Every run id stored for `document`, ascending.
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError>;

    /// 🗑️ Deletes run `run_id`, e.g. after `db_index`'s merge policy folds it into a
    /// larger run. Idempotent if already absent.
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError>;
}
//#endregion 🔖️IndexStorage
//#region 🔖️LeaseStorage
/// ⏳️ Named, TTL'd, fenced ownership leases — `db_cluster`'s shard-ownership + epoch
/// failover primitive. A lease is keyed by an opaque `resource` string (e.g. a shard id); at most
/// one `holder` may hold a given resource's lease at a time, and every successful hand-off (a
/// fresh `acquire` after the previous holder's lease expired) bumps the resource's `EpochFence` so
/// a stale former holder's writes are fenced out by `CatalogStorage`-style checks downstream.
pub trait LeaseStorage: Send + Sync {
    /// 🤝️ Acquires (or idempotently re-acquires, if `holder` already holds an unexpired
    /// lease on `resource`) `resource` for `holder`, valid until `now_ms + ttl_ms`. Returns the
    /// resource's current `EpochFence` — unchanged on re-acquire by the same holder, bumped
    /// (`.next()`) on a genuine hand-off from an expired or absent lease. Errors `Conflict` if
    /// another holder's lease on `resource` has not yet expired.
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError>;

    /// ♻️ Extends `holder`'s existing, unexpired lease on `resource` to `now_ms + ttl_ms`.
    /// `fence` must match the lease's current `EpochFence` (`DbError::Fenced` otherwise) and
    /// `holder` must match the current holder (`DbError::Unauthorized` otherwise). Errors
    /// `NotFound`/`Unavailable` if no lease (or an already-expired one) exists on `resource`.
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError>;

    /// 🕊️ Voluntarily releases `holder`'s lease on `resource` early (`fence` and `holder`
    /// must match, same rules as `renew`), immediately freeing `resource` for another `acquire`
    /// (which will still bump the epoch, since a hand-off occurred).
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError>;

    /// 👀️ The current unexpired lease on `resource` as of `now_ms`, or `None` if unheld or
    /// expired (an expired lease is reported as absent, never as stale data).
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError>;
}
//#endregion 🔖️LeaseStorage

#[cfg(all(any(test, feature = "claimed-backend-laws"), not(target_arch = "wasm32")))]
#[path = "../🧪️tests/🔬️claimed-backend/🦀️.rs"]
pub mod claimed_backend;

pub const WAL_WRITER_CAPACITY: usize = 32;

const _: () = assert!(WAL_WRITER_CAPACITY <= u8::MAX as usize);

/// 🎟️ Non-cloneable writer ownership; only its issuing backend may validate or retire it.
pub struct WalWriterPermit {
    pub key: WalWriterKey,
    pub document: DbIoText,
    pub release: Option<crate::WalWriterRelease>,
}

/// 🧷 Opaque task stamp; an old stamp never authorizes a recycled slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalWriterKey {
    pub backend: DbIoBackendControl,
    pub slot: u8,
    pub generation: u64,
}

impl WalWriterKey {
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

impl WalWriterPermit {
    pub fn key(&self) -> WalWriterKey {
        self.key
    }
    pub fn document(&self) -> &DbIoText {
        &self.document
    }

    pub fn release(mut self) -> crate::WalWriterRelease {
        self.release.take().expect("only a backend-bound writer exposes retained release")
    }

    /// 🅿️ Its holder rests: from now until [`Self::resume`] the backend may reclaim this writer for another document.
    pub fn park(&self) {
        if self.release.is_some() {
            crate::park(self.key);
        }
    }

    /// ▶️ Its holder writes again: `true` while this writer is still the holder's, `false` once its backend reclaimed it
    /// (or a release was requested) — the holder then releases this permit and acquires a fresh writer.
    pub fn resume(&self) -> bool {
        self.release.is_none() || crate::resume(self.key)
    }

    fn request_release(&self) {
        if self.release.is_some() && crate::request(self.key) {
            crate::request_controller(self.key.backend);
        }
    }
}

impl Drop for WalWriterPermit {
    fn drop(&mut self) {
        self.request_release();
    }
}
//#region 🚦️Admission
/// 👥️ How many acquisitions may wait in one backend's line at once — declared (`admission.waiters`); one more is refused
/// at once. It bounds the line's memory at this many tickets and wakers per backend.
pub const WAL_WRITER_ADMISSION_WAITERS: usize = 4_096;

/// 🗣️ The developer detail of a refused admission: `DbError::Unavailable`, transient — the caller retries the write.
pub const WAL_WRITER_ADMISSION_REFUSAL: &str = "WAL writer admission refused: every writer of this database is writing";

/// 📊️ One backend's admission line so far: acquisitions that had to wait, waits that were refused, and those waiting now.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalWriterAdmissionCensus {
    pub waited: u64,
    pub refused: u64,
    pub waiting: usize,
}

/// 🚦️ One backend's waiting acquisitions in arrival order and its capacity epoch — bumped whenever a writer slot is freed
/// or a writer is parked. Only the first in line tries again, and only after the epoch moved past what it last saw, so a
/// newcomer never overtakes a waiter and no freed slot goes unnoticed.
pub struct WalWriterAdmissionLine {
    pub backend: Option<DbIoBackendControl>,
    next_ticket: u64,
    pub capacity_epoch: u64,
    pub census: WalWriterAdmissionCensus,
    pub waiters: std::collections::VecDeque<(u64, Option<std::task::Waker>)>,
}

impl WalWriterAdmissionLine {
    pub const fn new() -> Self {
        Self {
            backend: None,
            next_ticket: 1,
            capacity_epoch: 0,
            census: WalWriterAdmissionCensus { waited: 0, refused: 0, waiting: 0 },
            waiters: std::collections::VecDeque::new(),
        }
    }

    pub fn rebind(&mut self, backend: DbIoBackendControl) -> Vec<std::task::Waker> {
        let stale = self.waiters.drain(..).filter_map(|(_, waker)| waker).collect();
        self.backend = Some(backend);
        self.next_ticket = 1;
        self.capacity_epoch = 0;
        self.census = WalWriterAdmissionCensus::default();
        stale
    }

    pub fn head(&self) -> Option<u64> {
        self.waiters.front().map(|(ticket, _)| *ticket)
    }

    pub fn join(&mut self) -> Result<u64, DbError> {
        if self.waiters.len() >= WAL_WRITER_ADMISSION_WAITERS {
            self.census.refused = self.census.refused.saturating_add(1);
            return Err(DbError::Unavailable(WAL_WRITER_ADMISSION_REFUSAL.to_string()));
        }
        let ticket = self.next_ticket;
        self.next_ticket = self.next_ticket.checked_add(1).ok_or(DbError::LimitExceeded("WAL writer admission ticket"))?;
        self.waiters.push_back((ticket, None));
        self.census.waited = self.census.waited.saturating_add(1);
        Ok(ticket)
    }

    pub fn leave(&mut self, ticket: u64) -> Option<std::task::Waker> {
        let was_head = self.head() == Some(ticket);
        self.waiters.retain(|(waiting, _)| *waiting != ticket);
        if was_head {
            self.take_head_waker()
        } else {
            None
        }
    }

    pub fn capacity_changed(&mut self) -> Option<std::task::Waker> {
        self.capacity_epoch = self.capacity_epoch.wrapping_add(1);
        self.take_head_waker()
    }

    fn take_head_waker(&mut self) -> Option<std::task::Waker> {
        self.waiters.front_mut().and_then(|(_, waker)| waker.take())
    }

    pub fn register(&mut self, ticket: u64, waker: &std::task::Waker) -> bool {
        let Some((_, slot)) = self.waiters.iter_mut().find(|(waiting, _)| *waiting == ticket) else { return false };
        if slot.as_ref().is_none_or(|parked| !parked.will_wake(waker)) {
            *slot = Some(waker.clone());
        }
        true
    }

    pub fn census(&self) -> WalWriterAdmissionCensus {
        WalWriterAdmissionCensus { waiting: self.waiters.len(), ..self.census }
    }
}

static WAL_WRITER_ADMISSIONS: [std::sync::Mutex<WalWriterAdmissionLine>; DB_IO_BACKEND_CONTROLS] = [const { std::sync::Mutex::new(WalWriterAdmissionLine::new()) }; DB_IO_BACKEND_CONTROLS];

pub(crate) const WAL_WRITER_ADMISSION_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_ADMISSIONS);

/// 🔗️ `backend`'s line, bound to it on first use; `None` for a backend whose slot a newer generation already owns (it
/// retired). Binding a newer generation wakes the retired one's leftover waiters, which then answer `Closed`.
pub fn with_admission_line<T>(backend: DbIoBackendControl, visit: impl FnOnce(&mut WalWriterAdmissionLine) -> T) -> Option<T> {
    let (slot, generation) = db_io_backend_parts(backend);
    let mut line = WAL_WRITER_ADMISSIONS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let stale = match line.backend.map(db_io_backend_parts) {
        Some((_, bound)) if bound == generation => Vec::new(),
        Some((_, bound)) if bound > generation => return None,
        _ => line.rebind(backend),
    };
    let visited = visit(&mut line);
    drop(line);
    for waker in stale {
        waker.wake();
    }
    Some(visited)
}

/// 🔔️ A writer slot of `backend` was freed or a writer was parked: the acquisition first in line tries again.
pub fn capacity_changed(backend: DbIoBackendControl) {
    if let Some(Some(waker)) = with_admission_line(backend, WalWriterAdmissionLine::capacity_changed) {
        waker.wake();
    }
}
//#endregion 🚦️Admission

/// 🅿️ Who may use an acquired writer right now: its holder while the holder writes (`Held`), the backend's next
/// acquisition while the holder rests (`Parked` at `since` on its backend's park clock, least recent first), and nobody
/// once the backend reclaimed it for another document — the holder then acquires a fresh writer before it writes again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WalWriterLease {
    Held,
    Parked { since: u64 },
    Reclaimed,
}

struct WalWriterSignalCell {
    pub active: Option<WalWriterKey>,
    pub requested: bool,
    pub lease: WalWriterLease,
    terminal_epoch: u64,
    pub waiter: Option<Waker>,
    pub notification: Option<Waker>,
    pub fault: Option<DbIoText>,
    deferred_fault_waiter: bool,
}

impl WalWriterSignalCell {
    pub const fn new() -> Self {
        Self { active: None, requested: false, lease: WalWriterLease::Held, terminal_epoch: 0, waiter: None, notification: None, fault: None, deferred_fault_waiter: false }
    }

    pub fn park(&mut self, key: WalWriterKey, since: u64) -> bool {
        if self.active != Some(key) || self.requested || self.lease != WalWriterLease::Held {
            return false;
        }
        self.lease = WalWriterLease::Parked { since };
        true
    }

    pub fn resume(&mut self, key: WalWriterKey) -> bool {
        if self.active != Some(key) || self.requested {
            return false;
        }
        match self.lease {
            WalWriterLease::Held => true,
            WalWriterLease::Parked { .. } => {
                self.lease = WalWriterLease::Held;
                true
            }
            WalWriterLease::Reclaimed => false,
        }
    }

    pub fn parked_since(&self, key: WalWriterKey) -> Option<u64> {
        match self.lease {
            WalWriterLease::Parked { since } if self.active == Some(key) && !self.requested && self.fault.is_none() => Some(since),
            _ => None,
        }
    }

    pub fn reclaim(&mut self, key: WalWriterKey) -> bool {
        if self.parked_since(key).is_none() {
            return false;
        }
        self.lease = WalWriterLease::Reclaimed;
        self.requested = true;
        true
    }

    /// 🔁️ A finished predecessor's undelivered terminal notification is handed back to its caller to
    /// wake: the predecessor's epoch is already published, so the successor never waits on the
    /// controller's asynchronous `notify_terminal` delivery.
    pub fn prepare(&mut self, key: WalWriterKey) -> Result<(u64, Option<Waker>), DbError> {
        if self.active.is_some() || self.deferred_fault_waiter {
            return Err(DbError::Conflict("WAL writer signal occupied".to_string()));
        }
        let required_epoch = self.terminal_epoch.checked_add(1).ok_or(DbError::LimitExceeded("WAL writer terminal epoch"))?;
        self.active = Some(key);
        self.requested = false;
        self.lease = WalWriterLease::Held;
        self.fault = None;
        self.deferred_fault_waiter = false;
        Ok((required_epoch, self.notification.take()))
    }

    pub fn request(&mut self, key: WalWriterKey) -> bool {
        if self.active != Some(key) {
            return false;
        }
        self.requested = true;
        true
    }

    pub fn requested(&self, key: WalWriterKey) -> bool {
        self.active == Some(key) && self.requested
    }

    pub fn cancel(&mut self, key: WalWriterKey) {
        assert_eq!(self.active, Some(key));
        assert!(self.waiter.is_none());
        assert!(!self.deferred_fault_waiter);
        self.active = None;
        self.requested = false;
        self.lease = WalWriterLease::Held;
    }

    pub fn finish(&mut self, key: WalWriterKey) -> Option<Waker> {
        assert_eq!(self.active, Some(key));
        self.terminal_epoch = self.terminal_epoch.checked_add(1).expect("writer acquisition reserved terminal epoch");
        self.active = None;
        self.requested = false;
        self.lease = WalWriterLease::Held;
        self.fault = None;
        self.deferred_fault_waiter = false;
        self.waiter.take()
    }

    pub fn poll(&mut self, key: WalWriterKey, required_epoch: u64, context: &mut Context<'_>) -> Poll<Result<(), DbError>> {
        if self.terminal_epoch >= required_epoch {
            return Poll::Ready(Ok(()));
        }
        if self.active != Some(key) {
            return Poll::Ready(Err(DbError::Fenced { expected: self.active.map_or(0, |key| key.generation), actual: key.generation }));
        }
        if let Some(fault) = &self.fault {
            if self.deferred_fault_waiter {
                return Poll::Pending;
            }
            return Poll::Ready(Err(DbError::Unavailable(fault.as_str().to_string())));
        }
        if self.waiter.as_ref().is_none_or(|waiter| !waiter.will_wake(context.waker())) {
            self.waiter = Some(context.waker().clone());
        }
        Poll::Pending
    }
}

pub static WAL_WRITER_SIGNALS: [[Mutex<WalWriterSignalCell>; WAL_WRITER_CAPACITY]; DB_IO_BACKEND_CONTROLS] = [const { [const { Mutex::new(WalWriterSignalCell::new()) }; WAL_WRITER_CAPACITY] }; DB_IO_BACKEND_CONTROLS];

pub(crate) const WAL_WRITER_SIGNAL_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_SIGNALS);

static WAL_WRITER_PARK_CLOCKS: [AtomicU64; DB_IO_BACKEND_CONTROLS] = [const { AtomicU64::new(1) }; DB_IO_BACKEND_CONTROLS];

pub(crate) const WAL_WRITER_PARK_CLOCK_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_PARK_CLOCKS);

pub fn cell(key: WalWriterKey) -> &'static Mutex<WalWriterSignalCell> {
    let (slot, _) = db_io_backend_parts(key.backend);
    &WAL_WRITER_SIGNALS[usize::from(slot)][usize::from(key.slot)]
}

/// 🧾️ A preflight reservation is cancelled without consuming a terminal epoch if guard construction fails.
pub struct WalWriterSignalReservation {
    pub key: WalWriterKey,
    required_epoch: u64,
    committed: bool,
}

impl WalWriterSignalReservation {
    pub fn commit(mut self) -> WalWriterRelease {
        self.committed = true;
        WalWriterRelease { key: self.key, required_epoch: self.required_epoch, requested: false, resolved: false }
    }
}

impl Drop for WalWriterSignalReservation {
    fn drop(&mut self) {
        if !self.committed {
            cell(self.key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).cancel(self.key);
        }
    }
}

/// ⏳️ One retained waiter observes its immutable terminal epoch even after a newer writer reuses the cell.
pub struct WalWriterRelease {
    pub key: WalWriterKey,
    required_epoch: u64,
    pub requested: bool,
    pub resolved: bool,
}

/// 🛟️ A failed close returns the same exact retained wait authority instead of discarding it.
pub struct WalWriterReleaseFailure {
    pub error: DbError,
    pub release: WalWriterRelease,
}

impl WalWriterReleaseFailure {
    pub fn error(&self) -> &DbError {
        &self.error
    }
    pub fn into_parts(self) -> (DbError, WalWriterRelease) {
        (self.error, self.release)
    }
}

impl std::fmt::Debug for WalWriterReleaseFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("WalWriterReleaseFailure").field("error", &self.error).field("key", &self.release.key).finish()
    }
}

impl WalWriterRelease {
    fn request_release(&mut self) {
        if !self.requested {
            self.requested = true;
            if request(self.key) {
                request_controller(self.key.backend);
            }
        }
    }

    pub fn retry(mut self) -> Self {
        let mut cell = cell(self.key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if cell.active == Some(self.key) {
            assert!(!cell.deferred_fault_waiter, "writer retry cannot overtake its exact deferred fault waiter");
            cell.fault = None;
            cell.waiter = None;
        }
        drop(cell);
        self.requested = true;
        request_controller(self.key.backend);
        self
    }
}

impl Drop for WalWriterRelease {
    fn drop(&mut self) {
        if !self.resolved {
            self.request_release();
        }
    }
}

impl std::future::Future for WalWriterRelease {
    type Output = Result<(), WalWriterReleaseFailure>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(!self.resolved, "writer close polled after terminal transfer");
        self.request_release();
        let result = {
            let mut cell = cell(self.key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if cell.deferred_fault_waiter && !deferred_wake_pending(self.key) {
                cell.deferred_fault_waiter = false;
            }
            cell.poll(self.key, self.required_epoch, context)
        };
        match result {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                self.resolved = true;
                Poll::Ready(result.map_err(|error| WalWriterReleaseFailure { error, release: Self { key: self.key, required_epoch: self.required_epoch, requested: true, resolved: false } }))
            }
        }
    }
}

pub fn prepare(key: WalWriterKey) -> Result<WalWriterSignalReservation, DbError> {
    let (slot, _) = db_io_backend_parts(key.backend);
    if usize::from(slot) >= DB_IO_BACKEND_CONTROLS || usize::from(key.slot) >= WAL_WRITER_CAPACITY {
        return Err(DbError::LimitExceeded("WAL writer signal authority"));
    }
    let (required_epoch, predecessor) = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).prepare(key)?;
    if let Some(waiter) = predecessor {
        waiter.wake();
    }
    Ok(WalWriterSignalReservation { key, required_epoch, committed: false })
}

pub fn request(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).request(key)
}

pub fn requested(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).requested(key)
}

pub fn fault(key: WalWriterKey, error: &DbError) {
    let mut cell = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if cell.active == Some(key) && cell.fault.is_none() {
        cell.fault = Some(crate::db_io_error_text(error));
    }
}

pub fn finish(key: WalWriterKey) {
    let mut cell = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(cell.notification.is_none());
    cell.notification = cell.finish(key);
}

/// 🅿️ The holder rests: until it resumes, its backend may reclaim this writer for another document, least recently parked
/// first, and the acquisition first in the backend's admission line is told a writer became reclaimable.
pub(crate) fn park(key: WalWriterKey) {
    let (slot, _) = db_io_backend_parts(key.backend);
    let since = WAL_WRITER_PARK_CLOCKS[usize::from(slot)].fetch_add(1, Ordering::Relaxed);
    if cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).park(key, since) {
        crate::capacity_changed(key.backend);
    }
}

/// ▶️ The holder writes again: `true` while its writer is still its own (held or parked), `false` once the backend
/// reclaimed it or a release was requested — the holder must acquire a fresh writer first.
pub(crate) fn resume(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).resume(key)
}

/// ⏱️ The park clock reading of a writer its holder parked and nobody reclaimed or asked to release yet.
pub fn parked_since(key: WalWriterKey) -> Option<u64> {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).parked_since(key)
}

/// 🪝️ Takes a parked writer back for the backend: exactly one of the holder's resume and this reclaim wins, so a holder
/// never writes with a writer its backend is releasing. The reclaimed writer is released like a requested one.
pub fn reclaim(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).reclaim(key)
}

pub fn fault_all_requested(backend: DbIoBackendControl, error: &DbError) {
    let (slot, _) = db_io_backend_parts(backend);
    for cell in &WAL_WRITER_SIGNALS[usize::from(slot)] {
        let mut cell = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if cell.active.is_some_and(|key| key.backend == backend) && cell.requested && cell.fault.is_none() {
            cell.fault = Some(crate::db_io_error_text(error));
        }
    }
}

pub struct WalWriterController {
    pub backend: DbIoBackendControl,
    pub pool: Arc<WorkerPool>,
    pub ticket: Option<WorkerMaintenanceTicket>,
    pub deferred_wake_ticket: Option<WorkerDeferredWakeTicket>,
    pub waker: Waker,
}

pub static WAL_WRITER_CONTROLLERS: [Mutex<Option<WalWriterController>>; DB_IO_BACKEND_CONTROLS] = [const { Mutex::new(None) }; DB_IO_BACKEND_CONTROLS];

pub(crate) const WAL_WRITER_CONTROLLER_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_CONTROLLERS);

pub fn request_controller(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    let controller = {
        let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| row.backend == backend) else { return };
        let (Some(ticket), Some(deferred_wake_ticket)) = (row.ticket, row.deferred_wake_ticket) else { return };
        if row.pool.request_maintenance(ticket).is_ok() {
            return;
        }
        (row.pool.clone(), deferred_wake_ticket)
    };
    fault_all_requested(backend, &DbError::Unavailable("WAL writer maintenance request refused; exact guard remains retained".to_string()));
    defer_fault_waiters(backend, &controller.0, controller.1);
}

pub(crate) fn request_any_controller() {
    for row in &WAL_WRITER_CONTROLLERS {
        let controller = {
            let row = row.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            row.as_ref().and_then(|row| Some((row.pool.clone(), row.ticket?)))
        };
        if controller.is_some_and(|(pool, ticket)| pool.request_maintenance(ticket).is_ok()) {
            return;
        }
    }
}

pub fn defer_fault_waiters(backend: DbIoBackendControl, pool: &WorkerPool, ticket: WorkerDeferredWakeTicket) {
    let (slot, _) = db_io_backend_parts(backend);
    for (writer_slot, cell) in WAL_WRITER_SIGNALS[usize::from(slot)].iter().enumerate() {
        let mut cell = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if cell.active.is_some_and(|key| key.backend == backend) && cell.fault.is_some() && !cell.deferred_fault_waiter {
            if let Some(waiter) = cell.waiter.take() {
                match pool.defer_wake(ticket, writer_slot, waiter) {
                    Ok(()) => cell.deferred_fault_waiter = true,
                    Err(rejected) => restore_fault_waiter(&mut cell, rejected.into_waker()),
                }
            }
        }
    }
}

fn restore_fault_waiter(cell: &mut WalWriterSignalCell, waiter: Waker) {
    assert!(cell.waiter.is_none() && !cell.deferred_fault_waiter, "rejected deferred wake must return to its exact empty signal slot");
    cell.waiter = Some(waiter);
}

pub fn deferred_wake_pending(key: WalWriterKey) -> bool {
    let (slot, _) = db_io_backend_parts(key.backend);
    let controller = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(controller) = controller.as_ref().filter(|controller| controller.backend == key.backend) else { return false };
    let Some(ticket) = controller.deferred_wake_ticket else { return false };
    controller.pool.deferred_wake_pending(ticket, usize::from(key.slot)).unwrap_or(false)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"]
mod db_io_retained_fixtures;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
