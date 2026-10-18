//! 💳️ Credit limits and operation ledger: the blocking queue signal, the page, item, byte and control limits, the credit, the test ledger owner and the operation ledger with its reserve, add, task, result lease and release operations. See [`super`].

use super::*;

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

pub const DB_IO_TASK_SLOT_BYTES: u64 = size_of::<DbIoTaskSlot>() as u64;

pub const DB_IO_LIST_BACKING_BYTES: u64 = (DB_IO_LIST_ITEMS * size_of::<u64>()) as u64;

pub const DB_IO_LIST_TRANSIENT_BYTES: u64 = DB_IO_LIST_BACKING_BYTES * 2;

pub const DB_IO_OPERATION_BYTES: u64 = (DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES * 4) as u64 + DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES;

pub const DB_IO_OPERATION_ITEM_CREDIT: usize = DB_IO_LIST_ITEMS * 2 + DB_IO_OPERATION_PAGES + 32;

pub const DB_IO_OPERATION_CONTROL_CREDIT: usize = 16;

pub const DB_IO_PROCESS_BYTES: u64 = (DB_IO_PAGE_BYTES * DB_IO_TOTAL_PAGES * 2 + DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES * DB_IO_OPERATION_ITEMS * 2) as u64 + DB_IO_OPERATION_ITEMS as u64 * (DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES);

const DB_IO_PROCESS_ITEM_CREDIT: usize = DB_IO_OPERATION_ITEMS * DB_IO_OPERATION_ITEM_CREDIT;

pub(super) const DB_IO_PROCESS_CONTROL_CREDIT: usize = DB_IO_OPERATION_ITEMS * DB_IO_OPERATION_CONTROL_CREDIT;

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

pub fn db_io_credit_within_limits(credit: DbIoCredit, process: bool) -> bool {
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

pub(super) fn db_io_operation_transfer_to_backend(from: u64, to: u64, credit: DbIoCredit) -> Result<(), DbError> {
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
//#endregion 🔖️Limits
