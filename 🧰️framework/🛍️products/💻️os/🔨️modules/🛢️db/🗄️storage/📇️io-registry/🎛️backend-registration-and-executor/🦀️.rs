//! 🎛️ Backend registration and executor: backend registration, credit, control, execution, close steps and retirement turns. See [`super`].

use super::*;

//#region 🔖️Limits
pub fn register_db_io_backend(kind: DbIoBackendKind, executor: Box<dyn DbIoTaskExecutor>, pool: Arc<WorkerPool>) -> Result<DbIoBackendControl, DbIoBackendRegistrationRejected> {
    let rollback = match DbIoBackendRollbackReservation::try_reserve() {
        Ok(rollback) => rollback,
        Err(cause) => return Err(DbIoBackendRegistrationRejected { cause, cleanup_error: None, owner: Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool }) }),
    };
    let pool_use = match pool.acquire_use() {
        Ok(pool_use) => pool_use,
        Err(error) => {
            return Err(DbIoBackendRegistrationRejected {
                cause: DbError::Unavailable(format!("DB I/O backend WorkerPool use rejected: {error:?}")),
                cleanup_error: None,
                owner: Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool }),
            })
        }
    };
    let owner_credit = db_io_backend_credit(executor.as_ref());
    let owner_operation = match db_io_backend_owner_reserve(owner_credit) {
        Ok(operation) => operation,
        Err(error) => {
            let retirement = rollback.commit(executor, 0, DbIoCredit::default(), pool, pool_use);
            return Err(DbIoBackendRegistrationRejected { cause: error, cleanup_error: None, owner: Some(DbIoBackendRegistrationRejectedOwner::Retiring(retirement)) });
        }
    };
    register_db_io_backend_reserved_with_use(kind, executor, pool, pool_use, owner_operation, owner_credit, rollback).map_err(|failure| DbIoBackendRegistrationRejected {
        cause: failure.cause,
        cleanup_error: None,
        owner: Some(DbIoBackendRegistrationRejectedOwner::Retiring(failure.retirement)),
    })
}

pub fn db_io_backend_credit(executor: &dyn DbIoTaskExecutor) -> DbIoCredit {
    let base = DbIoCredit { pages: 0, bytes: executor.owner_backing_bytes(), items: 1, controls: 1 };
    base.checked_add(crate::controller_credit()).expect("fixed DB I/O maintenance controller credit")
}

pub fn register_db_io_backend_reserved_with_use(
    kind: DbIoBackendKind,
    mut executor: Box<dyn DbIoTaskExecutor>,
    pool: Arc<WorkerPool>,
    pool_use: Arc<WorkerPoolUse>,
    owner_operation: u64,
    owner_credit: DbIoCredit,
    rollback: DbIoBackendRollbackReservation,
) -> Result<DbIoBackendControl, DbIoBackendAdmittedFailure> {
    if db_io_backend_credit(executor.as_ref()) != owner_credit {
        let cause = DbError::Internal("DB I/O backend backing differs from reserved bytes".to_string());
        let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
        return Err(DbIoBackendAdmittedFailure { cause, retirement });
    }
    if let Err(error) = executor.bind_owner_operation(owner_operation) {
        let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
        return Err(DbIoBackendAdmittedFailure { cause: error, retirement });
    }
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if registry.free_len == 0 || registry.next_generation == u64::MAX {
        drop(registry);
        let cause = DbError::Unavailable(DB_IO_BACKEND_CONTROLS_EXHAUSTED.to_string());
        let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
        return Err(DbIoBackendAdmittedFailure { cause, retirement });
    }
    let slot = registry.free[registry.free_read];
    let generation = registry.next_generation;
    registry.next_generation += 1;
    let control = db_io_backend_control(kind, slot, generation);
    if executor.supports_writer_authority() {
        if let Err(error) = executor.bind_writer_control(control) {
            drop(registry);
            let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
            return Err(DbIoBackendAdmittedFailure { cause: error, retirement });
        }
    }
    if let Err(error) = crate::install_controller(control, pool.clone()) {
        drop(registry);
        let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
        return Err(DbIoBackendAdmittedFailure { cause: error, retirement });
    }
    drop(rollback);
    registry.free_read = (registry.free_read + 1) % DB_IO_BACKEND_CONTROLS;
    registry.free_len -= 1;
    let mode = executor.mode();
    registry.slots[slot as usize] = DbIoBackendRegistrySlot {
        generation,
        kind,
        executor: Some(executor),
        executor_retired: false,
        close_requested: false,
        leased_operation: 0,
        admitted_operation: 0,
        executing_steps: 0,
        pending_operations: 0,
        owner_operation,
        owner_credit,
        mode,
        pool: Some(pool),
        pool_use: Some(pool_use),
        close_fault: None,
        cleanup_fault: None,
    };
    Ok(control)
}

pub fn db_io_writer_release_lane_step(control: DbIoBackendControl, context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[usize::from(slot)];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control || owner.executor_retired || owner.admitted_operation != 0 || owner.leased_operation != 0 {
        return Ok(DbIoWriterReleaseStep::Idle);
    }
    let Some(executor) = owner.executor.as_ref() else { return Ok(DbIoWriterReleaseStep::Idle) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| executor.writer_release_step(context))).unwrap_or_else(|_| Err(DbError::Internal("WAL writer guard release panicked; exact executor remains retained".to_string())))
}

pub fn db_io_backend_control(kind: DbIoBackendKind, slot: u16, generation: u64) -> DbIoBackendControl {
    match kind {
        DbIoBackendKind::Memory => DbIoBackendControl::Memory { slot, generation },
        DbIoBackendKind::Filesystem => DbIoBackendControl::Filesystem { slot, generation },
        DbIoBackendKind::Sqlite => DbIoBackendControl::Sqlite { slot, generation },
        DbIoBackendKind::Postgres => DbIoBackendControl::Postgres { slot, generation },
        DbIoBackendKind::Neo4j => DbIoBackendControl::Neo4j { slot, generation },
    }
}

pub fn db_io_executor_execute(control: DbIoBackendControl, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let executor: *const dyn DbIoTaskExecutor = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation {
            return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
        }
        if owner.mode != DbIoExecutorMode::BlockingLane || owner.pending_operations == 0 {
            return Err(DbError::Unavailable("DB I/O blocking backend step has no admitted operation".to_string()));
        }
        let executor = owner.executor.as_deref().ok_or(DbError::Closed)?;
        owner.executing_steps = owner.executing_steps.checked_add(1).ok_or(DbError::LimitExceeded("DB I/O executing backend steps"))?;
        executor as *const dyn DbIoTaskExecutor
    };
    // SAFETY: the executor stays boxed in its registry slot while `executing_steps` counts this step and
    // this operation is admitted (`pending_operations` > 0): every path that moves, drops or mutably
    // borrows it (backend close, retirement, the async-native lease) first requires both to be zero, so
    // concurrent steps of different operations only share `&self` (`DbIoTaskExecutor: Sync`).
    let execution = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let executor = unsafe { &*executor };
        executor.pin_writer_operation(operation, task)?;
        executor.execute_step(operation, task)
    }));
    {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        owner.executing_steps = owner.executing_steps.saturating_sub(1);
    }
    match execution {
        Ok(result) => result,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

pub fn db_io_executor_mode(control: DbIoBackendControl) -> Result<DbIoExecutorMode, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    Ok(owner.mode)
}

pub fn db_io_backend_return_operation(control: DbIoBackendControl, operation: u64) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation
        || db_io_backend_control(owner.kind, slot, generation) != control
        || owner.pending_operations == 0
        || owner.mode != DbIoExecutorMode::BlockingLane && (owner.admitted_operation != operation || owner.leased_operation != 0)
    {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    if owner.mode != DbIoExecutorMode::BlockingLane {
        owner.admitted_operation = 0;
    }
    owner.pending_operations -= 1;
    drop(registry);
    db_io_core::request_controller(control);
    db_io_admission_released();
    Ok(())
}

pub fn db_io_return_async_executor(control: DbIoBackendControl, operation: u64, executor: Box<dyn DbIoTaskExecutor>) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control || owner.leased_operation != operation || owner.executor.is_some() {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    owner.executor = Some(executor);
    owner.leased_operation = 0;
    drop(registry);
    db_io_core::request_controller(control);
    crate::defer_fault_notifications(control);
    Ok(())
}

pub(super) fn db_io_executor_close_operation(control: DbIoBackendControl, operation: u64, task: &DbIoTask) -> Result<bool, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    let executor = owner.executor.as_ref().ok_or(DbError::Closed)?;
    let terminal = executor.close_operation_step(operation, task)?;
    if terminal {
        executor.finish_writer_operation(operation, task)?;
    }
    Ok(terminal)
}

pub fn db_io_backend_close_step(control: DbIoBackendControl, context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let executor = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
        }
        if owner.pending_operations != 0 || owner.admitted_operation != 0 || owner.leased_operation != 0 || owner.executing_steps != 0 {
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
        crate::notify_terminal(control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
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
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    if !crate::retire_controller(control)? {
        return Ok(false);
    }
    if owner.owner_operation != 0 {
        db_io_operation_return(owner.owner_operation, owner.owner_credit)?;
        owner.owner_operation = 0;
        owner.owner_credit = DbIoCredit::default();
    }
    let pool_use = owner.pool_use.take().ok_or_else(|| DbError::Internal("DB I/O backend close lost its WorkerPool use".to_string()))?;
    registry.slots[slot as usize] = DbIoBackendRegistrySlot {
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
        owner_credit: DbIoCredit::default(),
        mode: DbIoExecutorMode::BlockingLane,
        pool: None,
        pool_use: None,
        close_fault: None,
        cleanup_fault: None,
    };
    let write = (registry.free_read + registry.free_len) % DB_IO_BACKEND_CONTROLS;
    registry.free[write] = slot;
    registry.free_len += 1;
    drop(registry);
    drop(pool_use);
    db_io_admission_released();
    Ok(true)
}

/// 🪦️ One retirement turn of a close-requested backend, run only inside its own pre-admitted
/// maintenance hook, so a dropped or closed backend can never lose its retirement request.
pub(crate) fn db_io_backend_retirement_turn(control: DbIoBackendControl, context: &mut std::task::Context<'_>) -> DbIoBackendRetirementTurn {
    let (slot, generation) = db_io_backend_parts(control);
    {
        let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control || !owner.close_requested || owner.close_fault.is_some() {
            return DbIoBackendRetirementTurn::Idle;
        }
        if owner.pending_operations != 0 || owner.admitted_operation != 0 || owner.leased_operation != 0 || owner.executing_steps != 0 {
            return DbIoBackendRetirementTurn::Idle;
        }
    }
    match db_io_backend_close_step(control, context) {
        Ok(true) => DbIoBackendRetirementTurn::Retired,
        Ok(false) => DbIoBackendRetirementTurn::More,
        Err(error) => {
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[slot as usize];
            if owner.generation == generation && db_io_backend_control(owner.kind, slot, generation) == control {
                owner.close_fault = Some(db_io_error_text(&error));
            }
            DbIoBackendRetirementTurn::Idle
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DbIoBackendRetirementTurn {
    Idle,
    More,
    Retired,
}

pub fn db_io_request_backend_close(control: DbIoBackendControl) -> Result<bool, DbError> {
    {
        let (slot, generation) = db_io_backend_parts(control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return Ok(false);
        }
        if let Some(fault) = owner.close_fault.as_ref() {
            return Err(DbError::Internal(fault.as_str().to_string()));
        }
        owner.close_requested = true;
    }
    crate::request_retirement(control)
}
//#endregion 🔖️Limits
