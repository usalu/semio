//! 🗒️ Backend registry and rejections: the backend registry, the rejected backend registry, rollback reservation and rejected backend parking. See [`super`].

use super::*;

//#region 🔖️Limits
pub(super) struct DbIoBackendRegistrySlot {
    pub generation: u64,
    pub kind: DbIoBackendKind,
    pub executor: Option<Box<dyn DbIoTaskExecutor>>,
    pub executor_retired: bool,
    pub close_requested: bool,
    pub leased_operation: u64,
    pub admitted_operation: u64,
    pub(super) executing_steps: u32,
    pub pending_operations: usize,
    pub owner_operation: u64,
    pub owner_credit: DbIoCredit,
    pub mode: DbIoExecutorMode,
    pub pool: Option<Arc<WorkerPool>>,
    pub pool_use: Option<Arc<WorkerPoolUse>>,
    pub close_fault: Option<DbIoText>,
    pub cleanup_fault: Option<DbIoFault>,
}

struct DbIoBackendRegistry {
    pub slots: [DbIoBackendRegistrySlot; DB_IO_BACKEND_CONTROLS],
    pub free: [u16; DB_IO_BACKEND_CONTROLS],
    pub free_read: usize,
    pub free_len: usize,
    pub next_generation: u64,
}

impl DbIoBackendRegistry {
    pub fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| DbIoBackendRegistrySlot {
                generation: 0,
                kind: DbIoBackendKind::Memory,
                executor: None,
                executor_retired: false,
                close_requested: false,
                leased_operation: 0,
                admitted_operation: 0,
                executing_steps: 0,
                pending_operations: 0,
                owner_operation: 0,
                owner_credit: DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 0 },
                mode: DbIoExecutorMode::BlockingLane,
                pool: None,
                pool_use: None,
                close_fault: None,
                cleanup_fault: None,
            }),
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: DB_IO_BACKEND_CONTROLS,
            next_generation: 1,
        }
    }
}

static DB_IO_BACKEND_REGISTRY: std::sync::OnceLock<std::sync::Mutex<DbIoBackendRegistry>> = std::sync::OnceLock::new();

pub fn db_io_backend_registry() -> &'static std::sync::Mutex<DbIoBackendRegistry> {
    DB_IO_BACKEND_REGISTRY.get_or_init(|| std::sync::Mutex::new(DbIoBackendRegistry::new()))
}

pub struct DbIoRejectedBackendSlot {
    pub generation: u64,
    pub reserved: bool,
    pub executor: Option<Box<dyn DbIoTaskExecutor>>,
    pub operation: u64,
    pub credit: DbIoCredit,
    pub pool: Option<Arc<WorkerPool>>,
    pub pool_use: Option<Arc<WorkerPoolUse>>,
    pub executor_retired: bool,
    scheduled: bool,
    lane_turn: bool,
    wake_requested: bool,
    pub fault: Option<DbIoText>,
}

impl DbIoRejectedBackendSlot {
    pub fn empty() -> Self {
        Self { generation: 0, reserved: false, executor: None, operation: 0, credit: DbIoCredit::default(), pool: None, pool_use: None, executor_retired: false, scheduled: false, lane_turn: false, wake_requested: false, fault: None }
    }
}

struct DbIoRejectedBackendRegistry {
    pub slots: [DbIoRejectedBackendSlot; DB_IO_BACKEND_CONTROLS],
    pub next_generation: u64,
}

static DB_IO_REJECTED_BACKENDS: std::sync::OnceLock<std::sync::Mutex<DbIoRejectedBackendRegistry>> = std::sync::OnceLock::new();

pub fn db_io_rejected_backends() -> &'static std::sync::Mutex<DbIoRejectedBackendRegistry> {
    DB_IO_REJECTED_BACKENDS.get_or_init(|| std::sync::Mutex::new(DbIoRejectedBackendRegistry { slots: std::array::from_fn(|_| DbIoRejectedBackendSlot::empty()), next_generation: 1 }))
}

pub struct DbIoBackendRollbackReservation {
    pub index: usize,
    pub generation: u64,
    pub active: bool,
}

impl DbIoBackendRollbackReservation {
    pub fn try_reserve() -> Result<Self, DbError> {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let index = registry.slots.iter().position(|slot| slot.generation == 0).ok_or_else(|| DbError::Unavailable(DB_IO_BACKEND_ROLLBACK_EXHAUSTED.to_string()))?;
        let generation = registry.next_generation;
        registry.next_generation = generation.checked_add(1).filter(|generation| *generation != 0).ok_or(DbError::LimitExceeded("DB I/O backend rollback generation"))?;
        registry.slots[index] = DbIoRejectedBackendSlot { generation, reserved: true, ..DbIoRejectedBackendSlot::empty() };
        Ok(Self { index, generation, active: true })
    }

    pub fn commit(mut self, executor: Box<dyn DbIoTaskExecutor>, operation: u64, credit: DbIoCredit, pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>) -> DbIoBackendRegistrationRetirement {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[self.index];
        assert!(owner.generation == self.generation && owner.reserved && owner.executor.is_none());
        *owner = DbIoRejectedBackendSlot {
            generation: self.generation,
            reserved: false,
            executor: Some(executor),
            operation,
            credit,
            pool: Some(pool),
            pool_use: Some(pool_use),
            executor_retired: false,
            scheduled: false,
            lane_turn: false,
            wake_requested: false,
            fault: None,
        };
        self.active = false;
        drop(registry);
        let _ = db_io_request_rejected_backend_close(self.index, self.generation);
        DbIoBackendRegistrationRetirement { index: self.index, generation: self.generation }
    }
}

impl Drop for DbIoBackendRollbackReservation {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.slots[self.index].generation == self.generation && registry.slots[self.index].reserved {
            registry.slots[self.index] = DbIoRejectedBackendSlot::empty();
        }
        drop(registry);
        db_io_admission_released();
    }
}

struct DbIoBackendRegistrationRetirement {
    pub index: usize,
    pub generation: u64,
}

pub(super) struct DbIoBackendAdmittedFailure {
    pub cause: DbError,
    pub retirement: DbIoBackendRegistrationRetirement,
}

impl DbIoBackendAdmittedFailure {
    fn into_cause(self) -> DbError {
        self.cause
    }
}

pub enum DbIoBackendRegistrationRejectedOwner {
    Incoming { kind: DbIoBackendKind, executor: Box<dyn DbIoTaskExecutor>, pool: Arc<WorkerPool> },
    Retiring(DbIoBackendRegistrationRetirement),
}

#[must_use = "backend registration rejection retains an executor or exact rollback retirement"]
pub struct DbIoBackendRegistrationRejected {
    pub cause: DbError,
    pub cleanup_error: Option<DbError>,
    pub owner: Option<DbIoBackendRegistrationRejectedOwner>,
}

impl std::fmt::Debug for DbIoBackendRegistrationRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DbIoBackendRegistrationRejected")
            .field("cause", &self.cause)
            .field("cleanup_error", &self.cleanup_error)
            .field("retained_executor", &matches!(self.owner, Some(DbIoBackendRegistrationRejectedOwner::Incoming { .. })))
            .field("retained_retirement", &matches!(self.owner, Some(DbIoBackendRegistrationRejectedOwner::Retiring(_))))
            .finish()
    }
}

impl DbIoBackendRegistrationRejected {
    pub fn error(&self) -> &DbError {
        &self.cause
    }

    pub fn has_retained_executor(&self) -> bool {
        matches!(self.owner, Some(DbIoBackendRegistrationRejectedOwner::Incoming { .. }))
    }

    pub fn cleanup_error(&self) -> Option<&DbError> {
        self.cleanup_error.as_ref()
    }

    pub fn retry(mut self) -> Result<DbIoBackendControl, Self> {
        let Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool }) = self.owner.take() else { return Err(self) };
        match register_db_io_backend(kind, executor, pool) {
            Ok(control) => Ok(control),
            Err(rejected) => Err(rejected),
        }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        let Some(owner) = self.owner.take() else { return Ok(true) };
        let retirement = match owner {
            DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool } => {
                let reservation = match DbIoBackendRollbackReservation::try_reserve() {
                    Ok(reservation) => reservation,
                    Err(_) => {
                        self.owner = Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool });
                        return Ok(false);
                    }
                };
                let pool_use = match pool.acquire_use() {
                    Ok(pool_use) => pool_use,
                    Err(_) => {
                        self.owner = Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool });
                        return Ok(false);
                    }
                };
                reservation.commit(executor, 0, DbIoCredit::default(), pool, pool_use)
            }
            DbIoBackendRegistrationRejectedOwner::Retiring(retirement) => retirement,
        };
        {
            let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[retirement.index];
            if owner.generation != retirement.generation {
                return Ok(true);
            }
            owner.fault = None;
        }
        if let Err(error) = db_io_request_rejected_backend_close(retirement.index, retirement.generation) {
            self.cleanup_error = Some(error);
            self.owner = Some(DbIoBackendRegistrationRejectedOwner::Retiring(retirement));
            return Ok(false);
        }
        let terminal = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[retirement.index].generation != retirement.generation;
        if !terminal {
            self.owner = Some(DbIoBackendRegistrationRejectedOwner::Retiring(retirement));
        }
        Ok(terminal)
    }
}

pub(super) fn db_io_park_rejected_backend(executor: Box<dyn DbIoTaskExecutor>, operation: u64, credit: DbIoCredit, pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>) -> Result<(), (Box<dyn DbIoTaskExecutor>, Arc<WorkerPool>, Arc<WorkerPoolUse>)> {
    let reservation = match DbIoBackendRollbackReservation::try_reserve() {
        Ok(reservation) => reservation,
        Err(_) => return Err((executor, pool, pool_use)),
    };
    drop(reservation.commit(executor, operation, credit, pool, pool_use));
    Ok(())
}

struct DbIoRejectedBackendWake {
    pub index: usize,
    pub generation: u64,
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
        let pool_use = if registry.slots[index].generation == generation {
            registry.slots[index].scheduled = true;
            registry.slots[index].pool_use.take()
        } else {
            None
        };
        drop(registry);
        drop(pool_use);
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.slots[index].generation == generation && registry.slots[index].scheduled {
            registry.slots[index] = DbIoRejectedBackendSlot::empty();
        }
        drop(registry);
        db_io_admission_released();
    } else {
        let _ = db_io_request_rejected_backend_close(index, generation);
    }
}

fn db_io_request_rejected_backend_close(index: usize, generation: u64) -> Result<bool, DbError> {
    let pool = {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation || owner.reserved || owner.fault.is_some() {
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
        Err(error) if matches!(error.kind(), WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated) => {
            pool.submit_at(pool.now_ms(), Lane::Io, error.into_job());
            Ok(true)
        }
        Err(error) => {
            let kind = error.kind();
            drop(error.into_job());
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[index];
            if owner.generation == generation {
                owner.scheduled = false;
                owner.fault = Some(db_io_error_text(&DbError::Unavailable(format!("DB I/O rejected backend retirement submission failed closed: {kind:?}"))));
            }
            Ok(false)
        }
    }
}

pub fn db_io_rejected_backend_maintenance_step() -> Result<bool, DbError> {
    let candidate = {
        let registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry.slots.iter().enumerate().find(|(_, owner)| owner.generation != 0 && !owner.reserved && !owner.scheduled && !owner.lane_turn && owner.fault.is_none()).map(|(index, owner)| (index, owner.generation))
    };
    let Some((index, generation)) = candidate else { return Ok(false) };
    db_io_request_rejected_backend_close(index, generation)
}
//#endregion 🔖️Limits
