//! 🚦️ Db I/O kernel driver: task submission, drive loop, WAL writer table and the storage traits.
#![allow(async_fn_in_trait)]

use db_ids::{DbError, GenerationId};
use db_io_core::{BLOCKING_QUEUE, DB_IO_ADMISSION_EPOCH, DB_IO_ADMISSION_WAITER_TABLE, DB_IO_ADMISSION_WAITING, DB_IO_BACKEND_CREDIT_EXHAUSTED, DB_IO_OPERATION_ITEMS, DB_IO_PROCESS_CREDIT_EXHAUSTED, DB_IO_PROCESS_PAGES_EXHAUSTED, DB_IO_RETIREMENT_PRESSURE_FAULT, DB_IO_TASK_ADMISSION_EXHAUSTED, DB_IO_TASK_SLOTS, DbIoAsyncDriverRuntime as _, DbIoBackendControl, DbIoCredit, DbIoExecutionStep, DbIoExecutorMode, DbIoFault, DbIoFaultCause, DbIoFaultKind, DbIoPagePhase, DbIoResult, DbIoResultHandback, DbIoResultLease, DbIoTask, DbIoTaskExecutor, DbIoTaskHandle, DbIoTaskKind, DbIoTaskPhase, DbIoTaskSlot, DbIoTerminal, DbIoText, DbIoWriterReleaseStep, PayloadStorage as _, WAL_WRITER_ADMISSION_REFUSAL, WAL_WRITER_CAPACITY, WAL_WRITER_CONTROLLERS, WalStorage as _, WalWriterAdmissionCensus, WalWriterAdmissionLine, WalWriterKey, WalWriterPermit, capacity_changed, cell, db_io_backend_owner_reserve, db_io_backend_parts, db_io_enqueue_close, db_io_operation_add, db_io_operation_add_result_lease, db_io_operation_attach_task, db_io_operation_detach_task, db_io_operation_mark_task, db_io_operation_reserve, db_io_operation_return, db_io_operation_return_result_lease, db_io_slot_matches, db_io_task_arena, with_admission_line};
use db_io_registry::{DB_IO_BACKEND_CONTROLS_EXHAUSTED, DB_IO_BACKEND_ROLLBACK_EXHAUSTED, DB_IO_CENSUS_STEPS, DB_IO_CENSUS_TASKS, DB_IO_CENSUS_TURNS, DB_IO_EXECUTOR_TURN_MICROS, DB_IO_EXECUTOR_TURN_STEPS, DB_IO_MAINTENANCE_CURSOR, DB_IO_RETRY_DELAY_MS, DB_IO_RETRY_LIMIT, DB_IO_RETRY_MAINTENANCE_CURSOR, DbIoBackendKind, DbIoBackendRegistrationRejected, DbIoBackendRegistrationRejectedOwner, DbIoBackendRollbackReservation, controller_step, db_io_backend_control, db_io_backend_credit, db_io_backend_registry, db_io_backend_return_operation, db_io_cancel_requested, db_io_census_admission_wait, db_io_executor_execute, db_io_executor_mode, db_io_literal_fault, db_io_lost_owner_close_step, db_io_maintenance_turn, db_io_page_maintenance_step, db_io_platform_maintenance_step, db_io_rejected_backend_maintenance_step, db_io_request_backend_close, db_io_return_async_executor, db_io_task_close_step, db_io_task_fault, db_io_wake, register_db_io_backend_reserved_with_use};
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Job, Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, pin::Pin, sync::Arc, sync::atomic::Ordering, task::Context, task::Poll, task::Waker};
#[cfg(any(test, feature = "testkit"))]
use db_durability::EpochFence;
#[cfg(any(test, feature = "testkit"))]
use db_io_core::{DB_IO_BACKEND_CONTROLS, DB_IO_LEDGER_OWNER, DB_IO_LOST_OWNERS, DB_IO_LOST_OWNER_OVERFLOW, DB_IO_LOST_OWNER_OVERFLOW_SLOTS, DB_IO_LOST_OWNER_QUARANTINE, DB_IO_LOST_OWNER_SLOTS, DB_IO_OPERATION_BYTES, DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DB_IO_TOTAL_PAGES, DbIoArtifactId, DbIoAsyncDriverFuture, DbIoDriverReservation, DbIoLeaseResult, DbIoLedgerOwner, DbIoLostOwner, DbIoPageArenaState, DbIoPageWriter, DbIoPages, LeaseStorage as _, WAL_WRITER_ADMISSION_WAITERS, db_io_ledger_census, db_io_page_arena, db_io_park_lost_owner, db_io_preflight_page_checkout, db_io_try_park_lost_owner, deferred_wake_pending};
#[cfg(any(test, feature = "testkit"))]
use db_io_registry::{DB_IO_PLATFORM_BUFFERS, DbIoPlatformBuffer, DbIoRejectedBackendSlot, db_io_backend_close_step, db_io_close_platform, db_io_lost_owner_close_opportunity, db_io_prepare_platform, db_io_rejected_backends, db_io_write_observed_bytes, register_db_io_backend};
#[cfg(any(test, feature = "testkit"))]
use pack::{ByteRange, ContentHash};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_async::WorkerSubmitErrorKind;
#[cfg(any(test, feature = "testkit"))]
use std::{process::Command, sync::Mutex};

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
fn db_io_take_backend_cleanup_fault(control: DbIoBackendControl) -> Option<DbError> {
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

fn db_io_allocate_task(mut task: DbIoTask) -> Result<DbIoTaskHandle, (DbError, DbIoTask)> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if arena.free_len == 0 || arena.next_generation == u64::MAX {
        return Err((DbError::Unavailable(DB_IO_TASK_CAPACITY_EXHAUSTED.to_string()), task));
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
    let pool = match db_io_backend_admit_operation(task.backend(), operation) {
        Ok(pool) => pool,
        Err(error) => {
            task.release_unstarted_list_backing();
            let _ = db_io_operation_detach_task(operation, aggregate_credit);
            return Err((error, task));
        }
    };
    if let Err(error) = task.admit_pages() {
        let _ = db_io_backend_return_operation(task.backend(), operation);
        task.release_unstarted_list_backing();
        let _ = db_io_operation_detach_task(operation, aggregate_credit);
        drop(arena);
        db_io_registry::defer_fault_notifications(task.backend());
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
        pool: Some(pool),
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
        result_retained_credit: DbIoCredit::default(),
        aggregate_returned: false,
        async_ready: false,
        async_detached: false,
        async_lane_turn: false,
        backend_admitted: true,
        async_driver: None,
        async_driver_scheduled: false,
        async_driver_wake_requested: false,
    };
    let _ = attached;
    BLOCKING_QUEUE.enqueued(aggregate_credit.bytes);
    Ok(handle)
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
        Err(error) => match worker_submit_retry_attempt(error.kind(), attempt, DB_IO_RETRY_LIMIT) {
            Some(next) => {
                drop(error.into_job());
                let generation = {
                    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if !db_io_slot_matches(&owner, handle) {
                        return;
                    }
                    owner.retry_attempt = Some(next);
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
            None => {
                let kind = error.kind();
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
    let kind = task_owner.kind() as usize;
    DB_IO_CENSUS_TURNS[kind].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let turn_ends = std::time::Instant::now() + std::time::Duration::from_micros(DB_IO_EXECUTOR_TURN_MICROS);
    let execution = task_owner.transition_pages(DbIoPagePhase::Queued, DbIoPagePhase::Executing).and_then(|()| {
        let mut steps = 0usize;
        loop {
            DB_IO_CENSUS_STEPS[kind].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            steps += 1;
            let step = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| db_io_executor_execute(task_owner.backend(), handle.operation, &mut task_owner))) {
                Ok(result) => result,
                Err(_) => {
                    panicked = true;
                    Err(DbError::Internal("DB I/O backend panicked".to_string()))
                }
            };
            match step {
                Ok((DbIoExecutionStep::Yield, None)) if steps < DB_IO_EXECUTOR_TURN_STEPS && std::time::Instant::now() < turn_ends && !db_io_cancel_requested(handle) => {}
                terminal => break terminal,
            }
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
    pub handle: DbIoTaskHandle,
}

impl std::task::Wake for DbIoAsyncDriverWake {
    fn wake(self: Arc<Self>) {
        db_io_request_async_driver_poll(self.handle);
    }

    fn wake_by_ref(self: &Arc<Self>) {
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
        return Err(DbError::StaleGeneration { expected: GenerationId(handle.generation), actual: GenerationId(owner.generation) });
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
    let waker = std::task::Waker::from(Arc::new(DbIoAsyncDriverWake { handle }));
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
    pub handle: DbIoTaskHandle,
    pub resolved: bool,
}

/// 🌐️ Exact task/backend lease driven by an async-native platform executor after Lane::Io admission.
pub struct DbIoAsyncTaskLease {
    pub handle: DbIoTaskHandle,
    pub backend: DbIoBackendControl,
    pub task: Option<DbIoTask>,
    pub executor: Option<Box<dyn DbIoTaskExecutor>>,
    pub completed: bool,
    credit_returned: bool,
}

fn db_io_async_lease_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: size_of::<DbIoAsyncTaskLease>() as u64, items: 1, controls: 1 }
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
            return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
        }
        owner.async_lane_turn = true;
        Ok(())
    }

    pub fn leave_lane_io_driver_turn(&self) -> Result<(), DbError> {
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) || !owner.async_detached || !owner.async_lane_turn {
            return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
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
            return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
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
        let runtime = executor.driver_runtime();
        let future = executor.drive_async(self.handle.operation, task);
        let future = match runtime {
            Some(runtime) => runtime.detach(future),
            None => future,
        };
        {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) || !owner.async_detached || owner.async_driver.is_some() {
                return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
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

const DB_IO_TASK_CAPACITY_EXHAUSTED: &str = "db I/O task capacity exhausted";

const DB_IO_ASYNC_NATIVE_OPERATION_BUSY: &str = "DB I/O async-native backend operation capacity exhausted";

const DB_IO_BACKEND_PENDING_OPERATIONS: &str = "DB I/O backend pending operations";

/// ⏳️ How long [`submit_db_io_task_admitted`] waits for DB I/O admission before it answers the refusal: long
/// enough to ride out a burst (an operation holds its capacity for milliseconds to seconds), and well inside the hub's
/// 30 s document-socket frame deadline, so a capacity that never frees (resident values) still reaches its writer as
/// a refusal instead of a closed socket.
pub const DB_IO_ADMISSION_WAIT_MS: u64 = 10_000;

/// 🚦️ Whether `error` only says a fixed DB I/O capacity is momentarily taken — the task arena, the process
/// credit ledger, the page arena, a backend's pending operations, or an async-native backend's one in-flight
/// operation — so the same task is admitted once something is released.
fn db_io_admission_saturated(error: &DbError) -> bool {
    match error {
        DbError::Unavailable(message) => [DB_IO_TASK_CAPACITY_EXHAUSTED, DB_IO_PROCESS_CREDIT_EXHAUSTED, DB_IO_TASK_ADMISSION_EXHAUSTED, DB_IO_ASYNC_NATIVE_OPERATION_BUSY, DB_IO_PROCESS_PAGES_EXHAUSTED].contains(&message.as_str()),
        DbError::LimitExceeded(label) => *label == DB_IO_BACKEND_PENDING_OPERATIONS,
        _ => false,
    }
}

/// 🚦️ Whether registering a backend was refused only because a fixed backend capacity is momentarily taken
/// (the control registry, the rollback registry, the backend process credit) — so a fresh registration succeeds
/// once a retiring backend releases its share.
pub fn db_io_backend_admission_saturated(error: &DbError) -> bool {
    matches!(error, DbError::Unavailable(message) if [DB_IO_BACKEND_CONTROLS_EXHAUSTED, DB_IO_BACKEND_ROLLBACK_EXHAUSTED, DB_IO_BACKEND_CREDIT_EXHAUSTED].contains(&message.as_str()))
}

/// ⏳️ Resolves once DB I/O capacity has been released since `epoch` (registered before it re-checks, so no
/// release between a refused submission and this wait is lost), or at `deadline_ms` on `pool`'s clock — the wait is
/// bounded by the pool's timer, not by the next release, so a capacity that never frees still ends in its refusal.
struct DbIoAdmissionReleased {
    pub epoch: u64,
    pub waiter: Option<u64>,
    pub pool: Arc<WorkerPool>,
    deadline_ms: u64,
    deadline_armed: bool,
}

impl DbIoAdmissionReleased {
    pub fn new(epoch: u64, pool: Arc<WorkerPool>, deadline_ms: u64) -> Self {
        Self { epoch, waiter: None, pool, deadline_ms, deadline_armed: false }
    }
}

impl Future for DbIoAdmissionReleased {
    type Output = Result<(), DbError>;

    fn poll(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        if DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire) != this.epoch || this.pool.now_ms() >= this.deadline_ms {
            return std::task::Poll::Ready(Ok(()));
        }
        let mut table = DB_IO_ADMISSION_WAITER_TABLE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let fresh = this.waiter.is_none();
        if !table.register(&mut this.waiter, context.waker()) {
            return std::task::Poll::Ready(Err(DbError::Unavailable("DB I/O admission waiters saturated".to_string())));
        }
        if fresh {
            DB_IO_ADMISSION_WAITING.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
        drop(table);
        if DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire) != this.epoch {
            return std::task::Poll::Ready(Ok(()));
        }
        if !this.deadline_armed {
            this.deadline_armed = true;
            let waker = context.waker().clone();
            this.pool.callback_at(this.deadline_ms, move || waker.wake());
        }
        std::task::Poll::Pending
    }
}

impl Drop for DbIoAdmissionReleased {
    fn drop(&mut self) {
        if let Some(waiter) = self.waiter.take() {
            DB_IO_ADMISSION_WAITER_TABLE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(waiter);
            DB_IO_ADMISSION_WAITING.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}

/// 🚦️ Submits `task`, waiting (at most [`DB_IO_ADMISSION_WAIT_MS`]) while a fixed DB I/O capacity is taken
/// instead of answering the refusal: a postgres or neo4j backend runs one async-native operation at a time, so the
/// second concurrent write of a busy hub used to be refused (`async-native backend operation capacity exhausted`, the
/// growth e2e's 24 documents on postgres, g18), and the process ledger refused writes while other documents held
/// credit. Any other refusal is answered at once; dropping the future cancels the wait.
pub async fn submit_db_io_task_admitted(mut task: DbIoTask) -> Result<DbIoTaskOperation, DbError> {
    let kind = task.kind();
    let mut clock: Option<(Arc<WorkerPool>, u64)> = None;
    let mut waiting: Option<std::time::Instant> = None;
    let outcome = loop {
        let epoch = DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire);
        match submit_db_io_task(task) {
            Ok(operation) => break Ok(operation),
            Err((error, returned)) if db_io_admission_saturated(&error) => {
                if clock.is_none() {
                    clock = db_io_backend_pool(returned.backend()).map(|pool| {
                        let deadline_ms = pool.now_ms().saturating_add(DB_IO_ADMISSION_WAIT_MS);
                        (pool, deadline_ms)
                    });
                }
                let Some((pool, deadline_ms)) = clock.clone().filter(|(pool, deadline_ms)| pool.now_ms() < *deadline_ms) else { break Err(error) };
                task = returned;
                waiting.get_or_insert_with(std::time::Instant::now);
                if let Err(error) = DbIoAdmissionReleased::new(epoch, pool, deadline_ms).await {
                    break Err(error);
                }
            }
            Err((error, _)) => break Err(error),
        }
    };
    if let Some(started) = waiting {
        db_io_census_admission_wait(kind, started.elapsed(), outcome.is_err());
    }
    outcome
}

/// 🕰️ The worker pool (and so the clock) serving `control`'s backend while it is registered.
fn db_io_backend_pool(control: DbIoBackendControl) -> Option<Arc<WorkerPool>> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    registry.slots.get(usize::from(slot)).filter(|owner| owner.generation == generation).and_then(|owner| owner.pool.clone())
}

/// 🚪️ Opens one storage backend with `open`, waiting (at most [`DB_IO_ADMISSION_WAIT_MS`] on `pool`'s clock) while
/// the process's fixed backend capacity — the control registry, the rollback registry, the backend process credit — is
/// taken by backends still retiring, instead of refusing. Every storage facade opens through it: in a shared test process
/// dozens of laws open and retire backends at once, and a refused open failed an unrelated law ("db I/O backend control
/// capacity exhausted", the in-process `--lib` flake). Any other refusal is answered at once; dropping the future
/// cancels the wait.
pub async fn open_db_io_backend_admitted<T, O>(pool: &Arc<WorkerPool>, mut open: impl FnMut() -> O) -> Result<T, DbStorageOpenRejected>
where
    O: Future<Output = Result<T, DbStorageOpenRejected>>,
{
    let deadline_ms = pool.now_ms().saturating_add(DB_IO_ADMISSION_WAIT_MS);
    let mut waiting: Option<std::time::Instant> = None;
    let outcome = loop {
        let epoch = DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire);
        match open().await {
            Err(rejected) if db_io_backend_admission_saturated(rejected.error()) && pool.now_ms() < deadline_ms => {
                drop(rejected);
                waiting.get_or_insert_with(std::time::Instant::now);
                if let Err(error) = DbIoAdmissionReleased::new(epoch, pool.clone(), deadline_ms).await {
                    break Err(error.into());
                }
            }
            outcome => break outcome,
        }
    };
    if let Some(started) = waiting {
        db_io_census_admission_wait(DbIoTaskKind::BackendOpen, started.elapsed(), outcome.is_err());
    }
    outcome
}

pub fn submit_db_io_task(task: DbIoTask) -> Result<DbIoTaskOperation, (DbError, DbIoTask)> {
    let _ = db_io_maintenance_step();
    let kind = task.kind();
    let handle = db_io_allocate_task(task)?;
    DB_IO_CENSUS_TASKS[kind as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    db_io_submit_job(handle, Box::new(move || db_io_drive_one(handle)), 0);
    Ok(DbIoTaskOperation { handle, resolved: false })
}

impl DbIoTaskOperation {
    /// 🧹 Finishes one retained cleanup opportunity per poll before storage-result handoff.
    pub async fn finish(self) -> Result<DbIoResult, DbError> {
        let handle = self.handle;
        match self.await {
            Ok(lease) => {
                let result = lease.into_result()?;
                db_io_wait_task_retirement(handle).await?;
                Ok(result)
            }
            Err(fault) => {
                let error = fault.into_db_error();
                db_io_wait_task_retirement(handle).await?;
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
            return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
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
                    return std::task::Poll::Ready(Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) }));
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
                Ok(executor) => match executor.pin_writer_operation(self.handle.operation, &task).and_then(|()| db_io_operation_add(self.handle.operation, db_io_async_lease_credit())) {
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
                return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
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
                return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
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
                db_io_operation_return_result_lease(self.handle.operation, DbIoCredit::default())?;
                let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
            }
            let Some(terminal) = owner.terminal.take() else {
                drop(owner);
                db_io_operation_return_result_lease(self.handle.operation, DbIoCredit::default())?;
                return Err(DbError::Internal("DB I/O terminal owner changed during exact take".to_string()));
            };
            let retained_credit = match &terminal {
                DbIoTerminal::Result(result) => result.retained_credit(),
                _ => DbIoCredit::default(),
            };
            owner.result_retained_credit = retained_credit;
            owner.phase = DbIoTaskPhase::Closing;
            (terminal, retained_credit)
        };
        self.resolved = true;
        let (terminal, retained_credit) = terminal;
        Ok(Some(match terminal {
            DbIoTerminal::Result(result) => Ok(DbIoResultLease { handle: self.handle, retained_credit, result: Some(result), transferred: false }),
            DbIoTerminal::Fault(mut fault) => {
                fault.result_handback = Some(DbIoResultHandback { handle: self.handle, retained_credit });
                Err(fault)
            }
            DbIoTerminal::Cancelled(None) => {
                let mut fault = db_io_literal_fault(DbIoFaultKind::Cancelled, DbIoFaultCause::Closed, "DB I/O task cancelled");
                fault.result_handback = Some(DbIoResultHandback { handle: self.handle, retained_credit });
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
            if owner.terminal.is_some() && !matches!(owner.terminal, Some(DbIoTerminal::Cancelled(Some(_)))) {
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

async fn db_io_wait_task_retirement(handle: DbIoTaskHandle) -> Result<(), DbError> {
    std::future::poll_fn(move |context| {
        db_io_maintenance_step()?;
        let backend = {
            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, handle) {
                return std::task::Poll::Ready(Ok(()));
            }
            owner.backend
        };
        if let Some(fault) = backend.and_then(db_io_take_backend_cleanup_fault) {
            return std::task::Poll::Ready(Err(fault));
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    })
    .await
}

/// 🧹 One fixed mounted DB I/O retry, page, platform or terminal-close opportunity.
pub fn db_io_maintenance_step() -> Result<bool, DbError> {
    db_io_maintenance_turn(&DB_IO_MAINTENANCE_CURSOR, |class| match class {
        0 => db_io_lost_owner_close_step(),
        1 => Ok(db_io_page_maintenance_step()?.is_some()),
        2 => db_io_platform_maintenance_step(),
        3 => Ok(db_io_retry_maintenance_step()),
        4 => db_io_rejected_backend_maintenance_step(),
        5 => Ok(db_io_task_close_step()?.is_some()),
        _ => unreachable!(),
    })
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
            owner.waker = owner.backend.and_then(crate::retirement_waker);
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
//#endregion 🔖️Limits
//#region 🔖️RetainedDbIo
/// 🏝️ A law that saturates or asserts a process-global capacity (a fixed admission table, a
/// retirement ring, the page arena, the backend registry) owns a whole process: inside a shared
/// test process it re-runs itself as the only law of a child process and reports that child's
/// verdict; a runner that already gives every law its own process (nextest) runs it in place.
/// Returns `true` where the law body must run.
#[cfg(any(test, feature = "testkit"))]
pub fn process_isolated_law(law: &str) -> bool {
    const ISOLATED_LAW: &str = "SEMIO_DB_ISOLATED_LAW";
    if std::env::var_os("NEXTEST").is_some() || std::env::var(ISOLATED_LAW).is_ok_and(|isolated| isolated == law) {
        return true;
    }
    const ISOLATED_LAW_WATCHDOG: std::time::Duration = std::time::Duration::from_secs(300);
    let executable = std::env::current_exe().expect("process-isolated law resolves its test executable");
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let log_path = std::env::temp_dir().join(format!("semio-db-isolated-law-{}-{nonce}.log", std::process::id()));
    let log = std::fs::File::create(&log_path).expect("process-isolated law creates its child log");
    let mut child = std::process::Command::new(executable)
        .args(["--exact", law, "--test-threads=1"])
        .env(ISOLATED_LAW, law)
        .stdout(log.try_clone().expect("process-isolated law shares its child log"))
        .stderr(log)
        .spawn()
        .expect("process-isolated law spawns its child process");
    let deadline = std::time::Instant::now() + ISOLATED_LAW_WATCHDOG;
    let status = loop {
        if let Some(status) = child.try_wait().expect("process-isolated law observes its child") {
            break Some(status);
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    let output = std::fs::read_to_string(&log_path).unwrap_or_default();
    let _ = std::fs::remove_file(&log_path);
    assert!(status.is_some_and(|status| status.success()) && output.contains("test result: ok. 1 passed"), "process-isolated law {law} did not pass as the only law of its own process ({status:?}):\n{output}");
    false
}

#[doc(hidden)]
pub fn db_io_test_pool() -> Arc<WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)))).clone()
}

/// ✅️ Test-only sync/async bridge. 🚫️async: E5 executor bridge — poll-once: every future
/// this crate's own backends (`MemoryStorage`, and `FsStorage`/`DbBackend` driven by
/// `semio_framework_async::testkit::ManualRuntime`, whose `run_blocking` executes synchronously)
/// hand back is already `Ready` the instant it's first polled — there is no real async wait
/// anywhere in a unit test. This drives one to completion without needing a real executor,
/// mirroring `semio_framework_async`'s own `ManualRuntime` test helper. The crate's single such
/// bridge (R2 E5: "at most one per crate"); [`block_on_ready`] is a thin `Result`-shaped wrapper
/// over it, not a second one.
#[cfg(any(test, feature = "testkit"))]
pub async fn poll_once<T>(fut: impl Future<Output = T>) -> T {
    fut.await
}

#[cfg(any(test, feature = "testkit"))]
pub async fn block_on_ready<T>(fut: impl Future<Output = Result<T, DbError>>) -> Result<T, DbError> {
    poll_once(fut).await
}
//#endregion 🔖️RetainedDbIo

/// 👣️ Release steps a reclaim spends inside the acquiring turn: a local guard (file lock, none) reaches terminal within
/// two, a remote one hands its unlock to the driver runtime after one and finishes on the release controller.
const WAL_WRITER_RECLAIM_STEPS: usize = 4;

struct WalWriterEntry<G> {
    pub document: DbIoText,
    pub generation: u64,
    active_operation: Option<u64>,
    releasing: bool,
    guard: G,
}

/// 🧹 A backend guard keeps failed or partial release ownership until its terminal witness.
pub trait WalWriterGuard {
    fn close_step(&mut self) -> Result<bool, DbError>;
    fn terminal_is_empty(&self) -> bool;

    /// ⏸️ A remote unlock in flight re-requests its controller itself; until then the slot is not runnable.
    fn awaiting_wake(&self) -> bool {
        false
    }

    /// 🔔 Backend close parks here while a remote unlock is in flight.
    fn register_wake(&self, _waker: &std::task::Waker) {}
}

impl WalWriterGuard for () {
    fn close_step(&mut self) -> Result<bool, DbError> {
        Ok(false)
    }
    fn terminal_is_empty(&self) -> bool {
        true
    }
}

/// 🗃️ Executor-owned bounded slots; a guard remains retained until explicit release.
pub struct WalWriterTable<G> {
    pub backend: Option<DbIoBackendControl>,
    signalled: bool,
    pub next_generation: u64,
    close_cursor: usize,
    pub entries: [Option<WalWriterEntry<G>>; WAL_WRITER_CAPACITY],
    #[cfg(test)]
    release_failures: usize,
}

impl<G: WalWriterGuard> WalWriterTable<G> {
    #[cfg(test)]
    pub fn new(backend: DbIoBackendControl) -> Self {
        Self { backend: Some(backend), signalled: false, next_generation: 1, close_cursor: 0, entries: std::array::from_fn(|_| None), release_failures: 0 }
    }

    pub fn for_backend(backend: DbIoBackendControl) -> Self {
        Self {
            backend: Some(backend),
            signalled: true,
            next_generation: 1,
            close_cursor: 0,
            entries: std::array::from_fn(|_| None),
            #[cfg(test)]
            release_failures: 0,
        }
    }

    pub fn unbound() -> Self {
        Self {
            backend: None,
            signalled: true,
            next_generation: 1,
            close_cursor: 0,
            entries: std::array::from_fn(|_| None),
            #[cfg(test)]
            release_failures: 0,
        }
    }

    pub fn bind(&mut self, backend: DbIoBackendControl) -> Result<(), DbError> {
        if self.backend.is_some() {
            return Err(DbError::Internal("WAL writer table bound twice".to_string()));
        }
        self.backend = Some(backend);
        Ok(())
    }

    pub fn backend(&self) -> DbIoBackendControl {
        self.backend.expect("writer admission requires a bound backend")
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn fail_next_release(&mut self) {
        self.release_failures = self.release_failures.saturating_add(1);
    }

    pub fn acquire_with(&mut self, document: &DbIoText, guard: impl FnOnce() -> Result<G, DbError>) -> Result<WalWriterPermit, DbError> {
        let slot = self.ensure_capacity(document)?;
        let next = self.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("WAL writer generation"))?;
        let generation = self.next_generation;
        let key = WalWriterKey { backend: self.backend(), slot: slot as u8, generation };
        let signal = if self.signalled { Some(db_io_core::prepare(key)?) } else { None };
        let guard = guard()?;
        self.entries[slot] = Some(WalWriterEntry { document: document.clone(), generation, active_operation: None, releasing: false, guard });
        self.next_generation = next;
        Ok(WalWriterPermit { key, document: document.clone(), release: signal.map(db_io_core::WalWriterSignalReservation::commit) })
    }

    /// 🎫️ The slot the next writer for `document` takes: its dropped predecessor's once that is retired, else a free one,
    /// else the one a parked writer frees when the backend reclaims it (least recently parked first). `Conflict` while a
    /// live predecessor holds the document; `LimitExceeded("WAL writer capacity")` while every writer writes, or while a
    /// reclaimed remote guard still unlocks (its completion frees the slot and tells the admission line). Remote backends
    /// call this before they open a lock session, so a full table never costs one.
    pub fn ensure_capacity(&mut self, document: &DbIoText) -> Result<usize, DbError> {
        if document.as_str().is_empty() {
            return Err(DbError::InvalidArgument("empty WAL writer document".to_string()));
        }
        if !self.retire_abandoned_predecessor(document)? {
            return Err(DbError::Conflict("WAL document already has a writer".to_string()));
        }
        if let Some(slot) = self.entries.iter().position(Option::is_none) {
            return Ok(slot);
        }
        self.reclaim_parked()?.ok_or(DbError::LimitExceeded("WAL writer capacity"))
    }

    /// 🪝️ Reclaims the least recently parked writer that no operation pins and nobody releases yet: `Some(slot)` once its
    /// guard reached terminal inside this turn (local guards), `None` when nothing is parked or its remote guard still
    /// unlocks — the backend's release controller then finishes it. A guard that fails to unlock stays retained, faulted,
    /// exactly like a requested release that failed.
    fn reclaim_parked(&mut self) -> Result<Option<usize>, DbError> {
        if !self.signalled {
            return Ok(None);
        }
        let backend = self.backend();
        let mut candidates = [(u64::MAX, 0usize); WAL_WRITER_CAPACITY];
        let mut parked = 0;
        for (slot, entry) in self.entries.iter().enumerate() {
            let Some(entry) = entry.as_ref().filter(|entry| entry.active_operation.is_none() && !entry.releasing) else { continue };
            if let Some(since) = db_io_core::parked_since(WalWriterKey { backend, slot: slot as u8, generation: entry.generation }) {
                candidates[parked] = (since, slot);
                parked += 1;
            }
        }
        candidates[..parked].sort_unstable();
        for &(_, slot) in &candidates[..parked] {
            let entry = self.entries[slot].as_ref().expect("selected parked writer");
            let key = WalWriterKey { backend, slot: slot as u8, generation: entry.generation };
            if !db_io_core::reclaim(key) {
                continue;
            }
            let document = entry.document.clone();
            for _ in 0..WAL_WRITER_RECLAIM_STEPS {
                match self.release_step(key, backend, &document) {
                    Ok(false) => return Ok(Some(slot)),
                    Ok(true) if self.entries[slot].as_ref().is_some_and(|entry| entry.guard.awaiting_wake()) => break,
                    Ok(true) => {}
                    Err(error) => {
                        db_io_core::fault(key, &error);
                        break;
                    }
                }
            }
            db_io_core::request_controller(backend);
            return Ok(None);
        }
        Ok(None)
    }

    /// ⏭️ A successor for the same document completes the release its dropped predecessor
    /// already requested, in the same table turn, instead of racing the backend's retirement hook:
    /// a dropped engine's writer is therefore retired deterministically before any reopen. Returns
    /// `false` while a live, pinned, faulted or multi-step predecessor still holds the document.
    fn retire_abandoned_predecessor(&mut self, document: &DbIoText) -> Result<bool, DbError> {
        let Some(slot) = self.entries.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.document == *document)) else { return Ok(true) };
        let entry = self.entries[slot].as_ref().expect("selected predecessor writer");
        let key = WalWriterKey { backend: self.backend(), slot: slot as u8, generation: entry.generation };
        let abandoned = entry.active_operation.is_none() && (entry.releasing || self.signalled && db_io_core::requested(key)) && !(self.signalled && crate::faulted(key));
        if !abandoned {
            return Ok(false);
        }
        let document = entry.document.clone();
        Ok(!self.release_step(key, self.backend(), &document)?)
    }

    #[cfg(test)]
    pub fn acquire(&mut self, document: &DbIoText, guard: G) -> Result<WalWriterPermit, (DbError, G)> {
        if document.as_str().is_empty() {
            return Err((DbError::InvalidArgument("empty WAL writer document".to_string()), guard));
        }
        if self.entries.iter().flatten().any(|entry| entry.document == *document) {
            return Err((DbError::Conflict("WAL document already has a writer".to_string()), guard));
        }
        let Some(slot) = self.entries.iter().position(Option::is_none) else {
            return Err((DbError::LimitExceeded("WAL writer capacity"), guard));
        };
        let Some(next) = self.next_generation.checked_add(1) else {
            return Err((DbError::LimitExceeded("WAL writer generation"), guard));
        };
        let generation = self.next_generation;
        self.entries[slot] = Some(WalWriterEntry { document: document.clone(), generation, active_operation: None, releasing: false, guard });
        self.next_generation = next;
        Ok(WalWriterPermit { key: WalWriterKey { backend: self.backend(), slot: slot as u8, generation }, document: document.clone(), release: None })
    }

    fn matching_entry(&self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText) -> Result<&WalWriterEntry<G>, DbError> {
        let entry = self.entries.get(usize::from(key.slot)).and_then(Option::as_ref);
        if self.backend != Some(backend) || key.backend != backend || entry.is_none_or(|entry| entry.generation != key.generation || entry.document != *document) {
            return Err(DbError::Fenced { expected: entry.map_or(0, |entry| entry.generation), actual: key.generation });
        }
        Ok(entry.expect("validated writer slot"))
    }

    pub fn validate(&self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText) -> Result<&G, DbError> {
        let entry = self.matching_entry(key, backend, document)?;
        if entry.releasing || self.signalled && db_io_core::requested(key) {
            return Err(DbError::Closed);
        }
        Ok(&entry.guard)
    }

    pub fn pin_operation(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<&G, DbError> {
        self.matching_entry(key, backend, document)?;
        if operation == 0 {
            return Err(DbError::InvalidArgument("zero WAL writer operation".to_string()));
        }
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        if (entry.releasing || self.signalled && db_io_core::requested(key)) && entry.active_operation != Some(operation) {
            return Err(DbError::Closed);
        }
        if entry.active_operation.is_some_and(|active| active != operation) {
            return Err(DbError::Conflict("WAL writer operation already admitted".to_string()));
        }
        entry.active_operation = Some(operation);
        Ok(&entry.guard)
    }

    /// 🔑 The exact pinned operation borrows its guard mutably, e.g. to drive writes through the lock session.
    pub fn pinned_guard_mut(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<&mut G, DbError> {
        self.matching_entry(key, backend, document)?;
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        if entry.active_operation != Some(operation) {
            return Err(DbError::Fenced { expected: entry.active_operation.unwrap_or(0), actual: operation });
        }
        Ok(&mut entry.guard)
    }

    pub fn register_wake(&self, waker: &std::task::Waker) {
        for entry in self.entries.iter().flatten() {
            entry.guard.register_wake(waker);
        }
    }

    pub fn finish_operation(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<(), DbError> {
        self.matching_entry(key, backend, document)?;
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        if entry.active_operation != Some(operation) {
            return Err(DbError::Fenced { expected: entry.active_operation.unwrap_or(0), actual: operation });
        }
        entry.active_operation = None;
        if self.signalled && db_io_core::requested(key) {
            db_io_core::request_controller(self.backend());
        }
        Ok(())
    }

    pub fn finish_operation_if_pinned(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<(), DbError> {
        if self.matching_entry(key, backend, document).is_ok_and(|entry| entry.active_operation == Some(operation)) {
            self.finish_operation(key, backend, document, operation)?;
        }
        Ok(())
    }

    pub fn release_step(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText) -> Result<bool, DbError> {
        self.matching_entry(key, backend, document)?;
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        entry.releasing = true;
        #[cfg(test)]
        if self.release_failures != 0 {
            self.release_failures -= 1;
            return Err(DbError::Io("injected WAL writer unlock failure".to_string()));
        }
        if entry.active_operation.is_some() || entry.guard.close_step()? {
            return Ok(true);
        }
        if !entry.guard.terminal_is_empty() {
            return Err(DbError::Internal("WAL writer guard returned a false terminal witness".to_string()));
        }
        self.entries[usize::from(key.slot)] = None;
        if self.signalled {
            db_io_core::finish(key);
            capacity_changed(key.backend);
        }
        Ok(false)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        let Some(slot) = (0..WAL_WRITER_CAPACITY).map(|offset| (self.close_cursor + offset) % WAL_WRITER_CAPACITY).find(|slot| self.entries[*slot].is_some()) else { return Ok(false) };
        self.close_cursor = (slot + 1) % WAL_WRITER_CAPACITY;
        let entry = self.entries[slot].as_ref().expect("selected retained writer slot");
        let key = WalWriterKey { backend: self.backend(), slot: slot as u8, generation: entry.generation };
        let document = entry.document.clone();
        self.release_step(key, self.backend(), &document)?;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.entries.iter().all(Option::is_none)
    }

    pub fn release_requested_step(&mut self) -> Result<db_io_core::DbIoWriterReleaseStep, DbError> {
        let Some(slot) = (0..WAL_WRITER_CAPACITY).map(|offset| (self.close_cursor + offset) % WAL_WRITER_CAPACITY).find(|slot| {
            self.entries[*slot].as_ref().is_some_and(|entry| {
                entry.active_operation.is_none()
                    && !entry.guard.awaiting_wake()
                    && (entry.releasing || self.signalled && db_io_core::requested(WalWriterKey { backend: self.backend(), slot: *slot as u8, generation: entry.generation }))
                    && (!self.signalled || !crate::faulted(WalWriterKey { backend: self.backend(), slot: *slot as u8, generation: entry.generation }))
            })
        }) else {
            return Ok(db_io_core::DbIoWriterReleaseStep::Idle);
        };
        self.close_cursor = (slot + 1) % WAL_WRITER_CAPACITY;
        let entry = self.entries[slot].as_ref().expect("selected exact requested writer");
        let key = WalWriterKey { backend: self.backend(), slot: slot as u8, generation: entry.generation };
        let document = entry.document.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.release_step(key, self.backend(), &document))) {
            Ok(Ok(_)) => {}
            result => {
                let error = match result {
                    Ok(Err(error)) => error,
                    _ => DbError::Internal("WAL writer guard release panicked; exact guard remains retained".to_string()),
                };
                if self.signalled {
                    db_io_core::fault(key, &error);
                    return Ok(db_io_core::DbIoWriterReleaseStep::Faulted);
                }
                return Err(error);
            }
        }
        Ok(db_io_core::DbIoWriterReleaseStep::More)
    }
}

/// 📁 Cross-process sidecar exclusion; called only from the backend's admitted I/O lane.
#[cfg(not(target_arch = "wasm32"))]
pub struct WalFileWriterGuard {
    pub file: Option<std::fs::File>,
}

#[cfg(not(target_arch = "wasm32"))]
impl WalFileWriterGuard {
    pub fn try_acquire(path: &std::path::Path) -> Result<Self, DbError> {
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path).map_err(|error| DbError::Io(error.to_string()))?;
        file.try_lock().map_err(|error| match error {
            std::fs::TryLockError::WouldBlock => DbError::Conflict("WAL document already has a filesystem writer".to_string()),
            std::fs::TryLockError::Error(error) => DbError::Io(error.to_string()),
        })?;
        Ok(Self { file: Some(file) })
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        let Some(file) = self.file.as_ref() else { return Ok(false) };
        file.unlock().map_err(|error| DbError::Io(error.to_string()))?;
        self.file = None;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.file.is_none()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl WalWriterGuard for WalFileWriterGuard {
    fn close_step(&mut self) -> Result<bool, DbError> {
        WalFileWriterGuard::close_step(self)
    }
    fn terminal_is_empty(&self) -> bool {
        WalFileWriterGuard::terminal_is_empty(self)
    }
}
//#region 🚦️Admission
/// ⏳️ How long one acquisition waits in its backend's admission line while every writer writes — declared
/// (`🧫️fixtures` `admission.waitMs`); a wait that outlives it is refused with [`WAL_WRITER_ADMISSION_REFUSAL`].
pub const WAL_WRITER_ADMISSION_WAIT_MS: u64 = 10_000;

/// 📊️ `backend`'s admission census (all zero for a backend that never had to wait).
pub fn admission_census(backend: DbIoBackendControl) -> WalWriterAdmissionCensus {
    with_admission_line(backend, |line| line.census()).unwrap_or_default()
}

/// 🎟️ A place in the admission line; dropping it — admitted, refused or cancelled — leaves the line and hands the turn on.
struct WalWriterAdmissionPlace {
    pub backend: DbIoBackendControl,
    pub ticket: u64,
}

impl WalWriterAdmissionPlace {
    pub fn join(backend: DbIoBackendControl) -> Result<Self, DbError> {
        let ticket = with_admission_line(backend, WalWriterAdmissionLine::join).ok_or(DbError::Closed)??;
        Ok(Self { backend, ticket })
    }
}

impl Drop for WalWriterAdmissionPlace {
    fn drop(&mut self) {
        if let Some(Some(waker)) = with_admission_line(self.backend, |line| line.leave(self.ticket)) {
            waker.wake();
        }
    }
}

/// 🎫️ Acquires one writer through `attempt` in `backend`'s admission order: at once while nobody waits and a slot is free
/// or reclaimable; otherwise in line, trying again each time a slot is freed or a writer parked while it is first. A wait
/// that outlives `wait` (the declared [`WAL_WRITER_ADMISSION_WAIT_MS`] for every storage) is refused with
/// [`WAL_WRITER_ADMISSION_REFUSAL`] as `DbError::Unavailable`; a retired backend answers `Closed`. Every other outcome of
/// `attempt` is returned as it is, and dropping the future leaves the line.
pub async fn admitted_acquire<F, Fut>(backend: DbIoBackendControl, wait: std::time::Duration, mut attempt: F) -> Result<WalWriterPermit, DbError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<WalWriterPermit, DbError>>,
{
    let pool = crate::controller_pool(backend).ok_or(DbError::Closed)?;
    let deadline_ms = pool.now_ms().saturating_add(u64::try_from(wait.as_millis()).unwrap_or(u64::MAX));
    let mut place: Option<WalWriterAdmissionPlace> = None;
    loop {
        let (turn, observed) = with_admission_line(backend, |line| {
            let turn = match place.as_ref() {
                Some(place) => line.head() == Some(place.ticket),
                None => line.waiters.is_empty(),
            };
            (turn, line.capacity_epoch)
        })
        .ok_or(DbError::Closed)?;
        if turn {
            match attempt().await {
                Err(DbError::LimitExceeded("WAL writer capacity")) => {}
                outcome => return outcome,
            }
        }
        if place.is_none() {
            place = Some(WalWriterAdmissionPlace::join(backend)?);
        }
        let ticket = place.as_ref().expect("a waiting acquisition holds its place").ticket;
        match (WalWriterAdmissionWait { backend, ticket, observed, pool: pool.clone(), deadline_ms, armed: false }).await {
            WalWriterAdmissionTurn::Ready => {}
            WalWriterAdmissionTurn::Elapsed => {
                with_admission_line(backend, |line| line.census.refused = line.census.refused.saturating_add(1));
                return Err(DbError::Unavailable(WAL_WRITER_ADMISSION_REFUSAL.to_string()));
            }
            WalWriterAdmissionTurn::Retired => return Err(DbError::Closed),
        }
    }
}

/// 🚥️ How one admission wait ended.
enum WalWriterAdmissionTurn {
    Ready,
    Elapsed,
    Retired,
}

/// ⏳️ Resolves once its waiter is first in line and the capacity epoch moved past `observed`, or at `deadline_ms` —
/// measured and armed on the backend's own pool clock, so the one wake it arms can never arrive before the deadline it
/// checks — or at once when the backend retired.
struct WalWriterAdmissionWait {
    pub backend: DbIoBackendControl,
    pub ticket: u64,
    observed: u64,
    pub pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    deadline_ms: u64,
    armed: bool,
}

impl std::future::Future for WalWriterAdmissionWait {
    type Output = WalWriterAdmissionTurn;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<WalWriterAdmissionTurn> {
        let (ticket, observed, deadline_ms) = (self.ticket, self.observed, self.deadline_ms);
        let now_ms = self.pool.now_ms();
        let turn = with_admission_line(self.backend, |line| {
            if line.head() == Some(ticket) && line.capacity_epoch != observed {
                Some(WalWriterAdmissionTurn::Ready)
            } else if now_ms >= deadline_ms {
                Some(WalWriterAdmissionTurn::Elapsed)
            } else if !line.register(ticket, context.waker()) {
                Some(WalWriterAdmissionTurn::Retired)
            } else {
                None
            }
        });
        match turn {
            None => return std::task::Poll::Ready(WalWriterAdmissionTurn::Retired),
            Some(Some(turn)) => return std::task::Poll::Ready(turn),
            Some(None) => {}
        }
        if !self.armed {
            self.armed = true;
            let waker = context.waker().clone();
            self.pool.callback_at(deadline_ms, move || waker.wake());
        }
        std::task::Poll::Pending
    }
}
//#endregion 🚦️Admission

pub(crate) fn faulted(key: WalWriterKey) -> bool {
    let cell = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    cell.active == Some(key) && cell.fault.is_some()
}

/// 🕰️ The worker pool (and so the clock and timer) of `backend`'s controller — the one clock a writer admission wait
/// measures its deadline on and arms its wake with; `None` once the backend retired.
pub(crate) fn controller_pool(backend: DbIoBackendControl) -> Option<Arc<WorkerPool>> {
    let (slot, _) = db_io_backend_parts(backend);
    WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().filter(|row| row.backend == backend).map(|row| row.pool.clone())
}

/// 🔔️ The backend hook's own waker, handed to owners whose terminal event must resume retirement.
pub(crate) fn retirement_waker(backend: DbIoBackendControl) -> Option<Waker> {
    let (slot, _) = db_io_backend_parts(backend);
    let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    row.as_ref().filter(|row| row.backend == backend).map(|row| row.waker.clone())
}

#[cfg(any(test, feature = "testkit"))]
pub fn suspend_controller_for_refusal(backend: DbIoBackendControl) -> Result<(), DbError> {
    let (slot, _) = db_io_backend_parts(backend);
    let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = row.as_ref().filter(|owner| owner.backend == backend).ok_or_else(|| DbError::Closed)?;
    let ticket = owner.ticket.ok_or(DbError::Closed)?;
    if !owner.pool.remove_maintenance_hook(ticket).map_err(|error| DbError::Unavailable(format!("WAL writer refusal fixture retirement: {error:?}")))? {
        return Err(DbError::Conflict("WAL writer controller is already running".to_string()));
    }
    Ok(())
}

#[cfg(any(test, feature = "testkit"))]
pub fn restore_controller_after_refusal(backend: DbIoBackendControl) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(backend);
    let mut row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = row.as_mut().filter(|owner| owner.backend == backend).ok_or_else(|| DbError::Closed)?;
    let ticket = owner.pool.install_maintenance_hook(Lane::Io, controller_step, [u64::from(slot), generation]).map_err(|error| DbError::Unavailable(format!("WAL writer refusal fixture restore: {error:?}")))?;
    owner.ticket = Some(ticket);
    Ok(())
}

#[cfg(any(test, feature = "testkit"))]
pub fn deferred_wake_pending_for_test(key: WalWriterKey) -> bool {
    deferred_wake_pending(key)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"]
mod db_io_retained_fixtures;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
