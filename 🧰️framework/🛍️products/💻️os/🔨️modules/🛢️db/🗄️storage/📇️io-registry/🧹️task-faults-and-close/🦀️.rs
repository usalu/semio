//! 🧹️ Task faults and close: task faults, cleanup, lost owner close, the maintenance turn and the task close step. See [`super`].

use super::*;

//#region 🔖️Limits
pub fn db_io_task_fault(kind: DbIoFaultKind, error: &DbError) -> DbIoFault {
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

pub fn db_io_literal_fault(kind: DbIoFaultKind, cause: DbIoFaultCause, detail: &'static str) -> DbIoFault {
    DbIoFault { kind, cause, detail: db_io_text_literal(detail), result_handback: None }
}

/// 🧯 Records one operation-cleanup fault on the backend it belongs to — never on the caller that happened to drive the close
/// ring — so only that backend's own task waiters and its close report it; the task stays in the ring and is retried. A
/// successful cleanup of the backend (`None`) clears it.
fn db_io_note_backend_cleanup(control: DbIoBackendControl, outcome: Option<&DbError>) {
    let (slot, generation) = db_io_backend_parts(control);
    let stale = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return;
        }
        std::mem::replace(&mut owner.cleanup_fault, outcome.map(|error| db_io_task_fault(DbIoFaultKind::Backend, error)))
    };
    if let Some(mut stale) = stale {
        while stale.close_step() {}
    }
}

fn db_io_task_closable(owner: &DbIoTaskSlot) -> bool {
    let running = owner.async_driver.is_some() || owner.phase == DbIoTaskPhase::Executing || owner.phase == DbIoTaskPhase::Queued && owner.retry_attempt.is_none();
    let awaiting_owner = !owner.abandoned && (matches!(owner.terminal, Some(DbIoTerminal::Result(_) | DbIoTerminal::Fault(_))) || owner.terminal.is_none() && owner.phase != DbIoTaskPhase::Closing);
    !running && !awaiting_owner
}

/// 🧺️ One bounded batch of the task close ring, run by a backend's maintenance hook. Reports
/// whether a closable task remains; running or owner-held tasks resume the hook through their wakers.
pub(crate) fn db_io_task_retirement_batch() -> bool {
    for _ in 0..DB_IO_OPERATION_ITEMS {
        if !matches!(db_io_task_close_step(), Ok(Some(_))) {
            break;
        }
    }
    let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    (0..arena.closing_len).filter_map(|offset| arena.closing[(arena.closing_read + offset) % DB_IO_OPERATION_ITEMS]).any(|handle| {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        db_io_slot_matches(&owner, handle) && db_io_task_closable(&owner)
    })
}

pub fn db_io_wake(owner: &mut DbIoTaskSlot) -> Option<std::task::Waker> {
    owner.waker.take()
}

pub static DB_IO_RETRY_MAINTENANCE_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// ⏱️ How long one worker-pool turn keeps driving the same task's resumable steps (one page,
/// one list item, one seal opportunity each) before it yields the worker: every step used to be its
/// own pool job, so a single index-run read cost ~35 pool turns and a whole commit ~14 000.
pub const DB_IO_EXECUTOR_TURN_MICROS: u64 = 2_000;

/// 🧮️ The most resumable steps one worker-pool turn drives, whatever the clock says.
pub const DB_IO_EXECUTOR_TURN_STEPS: usize = 4 * DB_IO_OPERATION_PAGES;

pub fn db_io_cancel_requested(handle: DbIoTaskHandle) -> bool {
    let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    !db_io_slot_matches(&owner, handle) || owner.cancelled
}

pub(crate) fn db_io_lost_owner_maintenance_batch() -> Result<bool, DbError> {
    DB_IO_LOST_OWNER_WAKE_REQUESTED.store(false, std::sync::atomic::Ordering::Release);
    for _ in 0..DB_IO_LOST_OWNER_SLOTS {
        if !db_io_lost_owner_close_step()? {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn db_io_lost_owner_close_step() -> Result<bool, DbError> {
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

pub fn db_io_lost_owner_close_opportunity(owner: &mut DbIoLostOwner) -> Result<bool, DbError> {
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
        DbIoLostOwner::Backend { owner, operation, credit, pool, pool_use } => {
            let backend = owner.take().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its exact executor".to_string()))?;
            let worker_pool = pool.take().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its shared WorkerPool".to_string()))?;
            let worker_pool_use = pool_use.take().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its WorkerPool use".to_string()))?;
            match db_io_park_rejected_backend(backend, *operation, *credit, worker_pool, worker_pool_use) {
                Ok(()) => {
                    *operation = 0;
                    *credit = DbIoCredit::default();
                    true
                }
                Err((backend, worker_pool, worker_pool_use)) => {
                    *owner = Some(backend);
                    *pool = Some(worker_pool);
                    *pool_use = Some(worker_pool_use);
                    DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                    return Ok(false);
                }
            }
        }
        DbIoLostOwner::ResultLease { handback, result } => {
            if let Some(owner) = result.as_mut() {
                if owner.close_step()?.is_some() {
                    return Ok(false);
                }
                *result = None;
                return Ok(false);
            }
            db_io_result_handback(*handback)?;
            true
        }
    };
    Ok(terminal)
}

pub const DB_IO_BACKEND_CONTROLS_EXHAUSTED: &str = "db I/O backend control capacity exhausted";

pub const DB_IO_BACKEND_ROLLBACK_EXHAUSTED: &str = "DB I/O backend rollback capacity exhausted";

pub(super) const DB_IO_MAINTENANCE_CLASSES: usize = 6;

pub static DB_IO_MAINTENANCE_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn db_io_maintenance_turn(cursor: &std::sync::atomic::AtomicUsize, mut opportunity: impl FnMut(usize) -> Result<bool, DbError>) -> Result<bool, DbError> {
    let start = cursor.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % DB_IO_MAINTENANCE_CLASSES;
    for offset in 0..DB_IO_MAINTENANCE_CLASSES {
        if opportunity((start + offset) % DB_IO_MAINTENANCE_CLASSES)? {
            return Ok(true);
        }
    }
    Ok(false)
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
    if owner.cancelled && owner.phase == DbIoTaskPhase::Queued && owner.retry_attempt.is_some() {
        if let Some(task) = owner.task.as_ref() {
            task.transition_pages(DbIoPagePhase::Queued, DbIoPagePhase::TerminalResult)?;
        }
        owner.retry_attempt = None;
        owner.phase = DbIoTaskPhase::Cancelled;
        if owner.terminal.is_none() {
            owner.terminal = Some(DbIoTerminal::Cancelled(None));
        }
        if let Some(waker) = db_io_wake(&mut owner) {
            drop(owner);
            waker.wake();
        }
        return Ok(Some(0));
    }
    if !db_io_task_closable(&owner) {
        drop(owner);
        db_io_rotate_close_head(handle)?;
        return Ok(Some(0));
    }
    if !owner.backend_cleanup_done {
        let task = owner.task.as_ref().ok_or_else(|| DbError::Internal("DB I/O cleanup lost typed task owner".to_string()))?;
        let backend = task.backend();
        let result = db_io_executor_close_operation(backend, handle.operation, task);
        if matches!(result, Ok(true)) {
            owner.backend_cleanup_done = true;
        }
        drop(owner);
        let rotated = if matches!(result, Ok(true)) { Ok(()) } else { db_io_rotate_close_head(handle) };
        drop(_turn);
        crate::defer_fault_notifications(backend);
        match &result {
            Ok(true) => db_io_note_backend_cleanup(backend, None),
            Ok(false) => {}
            Err(error) => db_io_note_backend_cleanup(backend, Some(error)),
        }
        rotated?;
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
        let result = db_io_backend_return_operation(backend, handle.operation);
        if result.is_ok() {
            owner.backend_admitted = false;
        }
        drop(owner);
        let rotated = if result.is_ok() { Ok(()) } else { db_io_rotate_close_head(handle) };
        drop(_turn);
        crate::defer_fault_notifications(backend);
        db_io_note_backend_cleanup(backend, result.as_ref().err());
        rotated?;
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
        let task_credit = owner.aggregate_credit.checked_sub(owner.result_retained_credit).ok_or_else(|| DbError::Internal("DB I/O retained result credit exceeded its task aggregate".to_string()))?;
        db_io_operation_detach_task(handle.operation, task_credit)?;
        owner.aggregate_returned = true;
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
    db_io_admission_released();
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
