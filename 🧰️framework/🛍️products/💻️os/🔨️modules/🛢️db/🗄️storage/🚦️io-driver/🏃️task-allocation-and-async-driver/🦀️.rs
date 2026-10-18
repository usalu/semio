//! 🏃️ Task allocation and async driver: task allocation, job submission, retries, one driver step and the async driver poll. See [`super`].

use super::*;

//#region 🔖️Limits
pub(super) fn db_io_allocate_task(mut task: DbIoTask) -> Result<DbIoTaskHandle, (DbError, DbIoTask)> {
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

pub(super) fn db_io_submit_job(handle: DbIoTaskHandle, job: Job, attempt: u8) {
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

pub(super) fn db_io_retry(handle: DbIoTaskHandle, retry_generation: u64) {
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

pub(super) fn db_io_retry_maintenance_step() -> bool {
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

pub(super) fn db_io_drive_one(handle: DbIoTaskHandle) {
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

pub(super) fn db_io_request_async_driver_poll(handle: DbIoTaskHandle) {
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
//#endregion 🔖️Limits
