//! 🧺️ Task slots, lost owners and admission: the task handle and slots, the task arena, the close queue, the result lease, the lost owner parking and the admission waiters. See [`super`].

use super::*;

//#region 🔖️Limits
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
        pub fault_custody: Option<DbIoFaultCustody>,
    pub poll_custody: Option<DbIoFaultCustody>,
    pub cleanup_custody: Option<DbIoFaultCustody>,
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
                    fault_custody: None,
            poll_custody: None,
            cleanup_custody: None,
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

pub fn db_io_request_lost_owner_maintenance() {
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
