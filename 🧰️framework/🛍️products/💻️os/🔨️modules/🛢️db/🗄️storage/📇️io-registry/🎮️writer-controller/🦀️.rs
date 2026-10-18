//! 🎮️ Writer controller: the retry limits and the WAL writer controller. See [`super`].

use super::*;

//#region 🔖️RetainedDbIo
pub const DB_IO_RETRY_LIMIT: u8 = 8;

pub const DB_IO_RETRY_DELAY_MS: u64 = 1;
//#endregion 🔖️RetainedDbIo

pub(crate) fn notify_terminal(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    for cell in &WAL_WRITER_SIGNALS[usize::from(slot)] {
        let waiter = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner).notification.take();
        if let Some(waiter) = waiter {
            waiter.wake();
        }
    }
}

pub(crate) fn notify_faults(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    for cell in &WAL_WRITER_SIGNALS[usize::from(slot)] {
        let waiter = {
            let mut cell = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if cell.active.is_some_and(|key| key.backend == backend) && cell.fault.is_some() {
                cell.waiter.take()
            } else {
                None
            }
        };
        if let Some(waiter) = waiter {
            waiter.wake();
        }
    }
}

fn has_runnable_request(backend: DbIoBackendControl) -> bool {
    let (slot, _) = db_io_backend_parts(backend);
    WAL_WRITER_SIGNALS[usize::from(slot)].iter().any(|cell| {
        let cell = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        cell.active.is_some_and(|key| key.backend == backend) && cell.requested && cell.fault.is_none()
    })
}

struct WalWriterControllerWake {
    pub backend: DbIoBackendControl,
}

impl std::task::Wake for WalWriterControllerWake {
    fn wake(self: Arc<Self>) {
        request_controller(self.backend);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        request_controller(self.backend);
    }
}

pub const fn controller_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: (size_of::<WalWriterControllerWake>() + 2 * size_of::<usize>()) as u64, items: 1, controls: 1 }
}

pub(crate) fn install_controller(backend: DbIoBackendControl, pool: Arc<WorkerPool>) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(backend);
    let mut row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if row.is_some() {
        return Err(DbError::Conflict("WAL writer controller occupied".to_string()));
    }
    let deferred_wake_ticket = pool.install_deferred_wake_partition().map_err(|error| DbError::Unavailable(format!("WAL writer deferred-wake admission: {error:?}")))?;
    let ticket = match pool.install_maintenance_hook(Lane::Io, controller_step, [u64::from(slot), generation]) {
        Ok(ticket) => ticket,
        Err(error) => {
            assert!(pool.remove_deferred_wake_partition(deferred_wake_ticket).is_ok_and(|removed| removed));
            return Err(DbError::Unavailable(format!("WAL writer maintenance admission: {error:?}")));
        }
    };
    let waker = Waker::from(Arc::new(WalWriterControllerWake { backend }));
    *row = Some(WalWriterController { backend, pool, ticket: Some(ticket), deferred_wake_ticket: Some(deferred_wake_ticket), waker });
    Ok(())
}

pub fn defer_fault_notifications(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    let controller = {
        let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| row.backend == backend) else { return };
        let Some(ticket) = row.deferred_wake_ticket else { return };
        (row.pool.clone(), ticket)
    };
    defer_fault_waiters(backend, &controller.0, controller.1);
}

pub fn controller_step([slot, generation]: [u64; 2]) -> WorkerMaintenanceStep {
    let Some(controller) = WAL_WRITER_CONTROLLERS.get(slot as usize) else { return WorkerMaintenanceStep::Fault };
    let (backend, waker) = {
        let row = controller.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| db_io_backend_parts(row.backend) == (slot as u16, generation)) else { return WorkerMaintenanceStep::Idle };
        (row.backend, row.waker.clone())
    };
    let writer = if has_runnable_request(backend) {
        let result = crate::db_io_writer_release_lane_step(backend, &mut Context::from_waker(&waker));
        if let Err(error) = &result {
            fault_all_requested(backend, error);
        }
        notify_terminal(backend);
        notify_faults(backend);
        Some(result)
    } else {
        None
    };
    match crate::db_io_lost_owner_maintenance_batch() {
        Ok(true) => return WorkerMaintenanceStep::More,
        Err(_) => return WorkerMaintenanceStep::Fault,
        Ok(false) => {}
    }
    let tasks = crate::db_io_task_retirement_batch();
    let writer_more = matches!(writer, Some(Ok(DbIoWriterReleaseStep::More | DbIoWriterReleaseStep::Faulted)));
    match crate::db_io_backend_retirement_turn(backend, &mut Context::from_waker(&waker)) {
        DbIoBackendRetirementTurn::Retired => WorkerMaintenanceStep::Retire,
        DbIoBackendRetirementTurn::More => WorkerMaintenanceStep::More,
        DbIoBackendRetirementTurn::Idle if writer_more || tasks => WorkerMaintenanceStep::More,
        DbIoBackendRetirementTurn::Idle => WorkerMaintenanceStep::Idle,
    }
}

/// 📣️ Coalesces one retirement request into the backend's pre-admitted maintenance hook; it
/// never allocates, never queues a closure, and cannot be lost to lane contention.
pub(crate) fn request_retirement(backend: DbIoBackendControl) -> Result<bool, DbError> {
    let (slot, _) = db_io_backend_parts(backend);
    let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(row) = row.as_ref().filter(|row| row.backend == backend) else { return Ok(false) };
    let Some(ticket) = row.ticket else { return Ok(false) };
    row.pool.request_maintenance(ticket).map(|request| request == WorkerMaintenanceRequest::Requested).map_err(|error| DbError::Unavailable(format!("DB I/O backend retirement request refused: {error:?}")))
}

/// 🪦️ Runs inside the backend's own hook turn: the hook returns `Retire`, so its slot is released
/// by the pool without an external removal racing the running invocation.
pub(crate) fn retire_controller(backend: DbIoBackendControl) -> Result<bool, DbError> {
    let (slot, _) = db_io_backend_parts(backend);
    let mut row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(owner) = row.as_mut() else { return Ok(true) };
    if owner.backend != backend {
        return Err(DbError::Fenced { expected: db_io_backend_parts(owner.backend).1, actual: db_io_backend_parts(backend).1 });
    }
    if let Some(ticket) = owner.deferred_wake_ticket {
        if !owner.pool.remove_deferred_wake_partition(ticket).map_err(|error| DbError::Unavailable(format!("WAL writer deferred-wake retirement: {error:?}")))? {
            return Ok(false);
        }
        owner.deferred_wake_ticket = None;
    }
    *row = None;
    Ok(true)
}
