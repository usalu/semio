//! 🔓️ Open rejection and backend lifecycle: the storage open rejection and the backend registration, retirement and close. See [`super`].

use super::*;

//#region 🔖️Limits
/// 🧲️ Storage construction rejection retaining backend registration or close authority.
#[must_use = "storage-open rejection must reach its exact backend terminal acknowledgement"]
pub enum DbStorageOpenRejected {
    BeforeExecutor(DbError),
    Registration(DbIoBackendRegistrationRejected),
    Registered { cause: DbError, cleanup_error: Option<DbError>, control: DbIoBackendControl },
}

impl DbStorageOpenRejected {
    pub fn registered(cause: DbError, control: DbIoBackendControl) -> Self {
        Self::Registered { cause, cleanup_error: None, control }
    }

    pub fn error(&self) -> &DbError {
        match self {
            Self::BeforeExecutor(cause) | Self::Registered { cause, .. } => cause,
            Self::Registration(rejected) => rejected.error(),
        }
    }

    pub fn cleanup_error(&self) -> Option<&DbError> {
        match self {
            Self::BeforeExecutor(_) => None,
            Self::Registration(rejected) => rejected.cleanup_error(),
            Self::Registered { cleanup_error, .. } => cleanup_error.as_ref(),
        }
    }

    pub fn has_retained_backend(&self) -> bool {
        !matches!(self, Self::BeforeExecutor(_))
    }

    pub async fn retry_close(self) -> Result<DbError, Self> {
        match self {
            Self::BeforeExecutor(cause) => Ok(cause),
            Self::Registration(mut rejected) => match rejected.close_step() {
                Ok(true) => Ok(rejected.cause),
                Ok(false) | Err(_) => {
                    semio_framework_async::yield_once().await;
                    Err(Self::Registration(rejected))
                }
            },
            Self::Registered { cause, mut cleanup_error, control } => {
                if let Err(error) = db_io_request_backend_close(control).and_then(|_| db_io_maintenance_step().map(|_| ())) {
                    cleanup_error = Some(error);
                    return Err(Self::Registered { cause, cleanup_error, control });
                }
                let (slot, generation) = db_io_backend_parts(control);
                let terminal = {
                    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let owner = &registry.slots[slot as usize];
                    owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control
                };
                if terminal {
                    Ok(cause)
                } else {
                    semio_framework_async::yield_once().await;
                    Err(Self::Registered { cause, cleanup_error, control })
                }
            }
        }
    }
}

impl From<DbError> for DbStorageOpenRejected {
    fn from(cause: DbError) -> Self {
        Self::BeforeExecutor(cause)
    }
}

impl From<DbIoBackendRegistrationRejected> for DbStorageOpenRejected {
    fn from(rejected: DbIoBackendRegistrationRejected) -> Self {
        Self::Registration(rejected)
    }
}

impl std::fmt::Debug for DbStorageOpenRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbStorageOpenRejected").field("cause", self.error()).field("cleanup_error", &self.cleanup_error()).field("has_retained_backend", &self.has_retained_backend()).finish()
    }
}

pub fn register_db_io_backend_prepared_with_use(
    kind: DbIoBackendKind,
    executor: Box<dyn DbIoTaskExecutor>,
    pool: Arc<WorkerPool>,
    pool_use: Arc<WorkerPoolUse>,
    rollback: DbIoBackendRollbackReservation,
) -> Result<DbIoBackendControl, DbIoBackendRegistrationRejected> {
    let owner_credit = db_io_backend_credit(executor.as_ref());
    let owner_operation = match db_io_backend_owner_reserve(owner_credit) {
        Ok(operation) => operation,
        Err(cause) => {
            let retirement = rollback.commit(executor, 0, DbIoCredit::default(), pool, pool_use);
            return Err(DbIoBackendRegistrationRejected { cause, cleanup_error: None, owner: Some(DbIoBackendRegistrationRejectedOwner::Retiring(retirement)) });
        }
    };
    register_db_io_backend_reserved_with_use(kind, executor, pool, pool_use, owner_operation, owner_credit, rollback).map_err(|failure| DbIoBackendRegistrationRejected {
        cause: failure.cause,
        cleanup_error: None,
        owner: Some(DbIoBackendRegistrationRejectedOwner::Retiring(failure.retirement)),
    })
}

pub fn retire_db_io_backend(control: DbIoBackendControl) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
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
            (owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control, owner.close_fault.as_ref().map(|fault| fault.as_str().to_string()))
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
        if let Some(fault) = db_io_take_backend_cleanup_fault(control) {
            return std::task::Poll::Ready(Err(fault));
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    })
    .await
}

pub fn db_io_backend_admit_operation(control: DbIoBackendControl, operation: u64) -> Result<Arc<WorkerPool>, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    if owner.close_requested || owner.executor_retired {
        return Err(DbError::Closed);
    }
    if owner.mode != DbIoExecutorMode::BlockingLane && owner.admitted_operation != 0 {
        return Err(DbError::Unavailable(DB_IO_ASYNC_NATIVE_OPERATION_BUSY.to_string()));
    }
    let pool = owner.pool.clone().ok_or_else(|| DbError::Internal("DB I/O backend admission lost its registered WorkerPool".to_string()))?;
    let pending = owner.pending_operations.checked_add(1).filter(|count| *count <= DB_IO_OPERATION_ITEMS).ok_or(DbError::LimitExceeded(DB_IO_BACKEND_PENDING_OPERATIONS))?;
    if owner.mode != DbIoExecutorMode::BlockingLane {
        owner.admitted_operation = operation;
    }
    owner.pending_operations = pending;
    Ok(pool)
}

pub fn db_io_take_async_executor(control: DbIoBackendControl, operation: u64) -> Result<Box<dyn DbIoTaskExecutor>, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    if owner.leased_operation != 0 {
        return Err(DbError::Unavailable("DB I/O async-native backend already has an operation lease".to_string()));
    }
    let executor = owner.executor.take().ok_or(DbError::Closed)?;
    owner.leased_operation = operation;
    Ok(executor)
}

/// 🧯 Takes the operation-cleanup fault last recorded on `control`, if any, as the error its own waiter or close reports.
pub(super) fn db_io_take_backend_cleanup_fault(control: DbIoBackendControl) -> Option<DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let fault = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return None;
        }
        owner.cleanup_fault.take()
    };
    fault.map(DbIoFault::into_db_error)
}
//#endregion 🔖️Limits
