//! 🔔️ Fixed writer release intent and monotonic completion witnesses survive backend-slot reuse.
use super::super::{db_io_backend_parts, DB_IO_BACKEND_CONTROLS};
use super::super::{DbIoBackendControl, DbIoCredit, DbIoText, DbIoWriterReleaseStep};
use super::{WalWriterKey, WAL_WRITER_CAPACITY};
use crate::DbError;
use semio_framework_async::{Lane, WorkerDeferredWakeTicket, WorkerMaintenanceStep, WorkerMaintenanceTicket, WorkerPool};
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

struct WalWriterSignalCell {
    active: Option<WalWriterKey>,
    requested: bool,
    terminal_epoch: u64,
    waiter: Option<Waker>,
    notification: Option<Waker>,
    fault: Option<DbIoText>,
    deferred_fault_waiter: bool,
}

impl WalWriterSignalCell {
    const fn new() -> Self {
        Self { active: None, requested: false, terminal_epoch: 0, waiter: None, notification: None, fault: None, deferred_fault_waiter: false }
    }

    fn prepare(&mut self, key: WalWriterKey) -> Result<u64, DbError> {
        if self.active.is_some() || self.notification.is_some() || self.deferred_fault_waiter {
            return Err(DbError::Conflict("WAL writer signal occupied".to_string()));
        }
        let required_epoch = self.terminal_epoch.checked_add(1).ok_or(DbError::LimitExceeded("WAL writer terminal epoch"))?;
        self.active = Some(key);
        self.requested = false;
        self.fault = None;
        self.deferred_fault_waiter = false;
        Ok(required_epoch)
    }

    fn request(&mut self, key: WalWriterKey) -> bool {
        if self.active != Some(key) {
            return false;
        }
        self.requested = true;
        true
    }

    fn requested(&self, key: WalWriterKey) -> bool {
        self.active == Some(key) && self.requested
    }

    fn cancel(&mut self, key: WalWriterKey) {
        assert_eq!(self.active, Some(key));
        assert!(self.waiter.is_none());
        assert!(!self.deferred_fault_waiter);
        self.active = None;
        self.requested = false;
    }

    fn finish(&mut self, key: WalWriterKey) -> Option<Waker> {
        assert_eq!(self.active, Some(key));
        self.terminal_epoch = self.terminal_epoch.checked_add(1).expect("writer acquisition reserved terminal epoch");
        self.active = None;
        self.requested = false;
        self.fault = None;
        self.deferred_fault_waiter = false;
        self.waiter.take()
    }

    fn poll(&mut self, key: WalWriterKey, required_epoch: u64, context: &mut Context<'_>) -> Poll<Result<(), DbError>> {
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

static WAL_WRITER_SIGNALS: [[Mutex<WalWriterSignalCell>; WAL_WRITER_CAPACITY]; DB_IO_BACKEND_CONTROLS] = [const { [const { Mutex::new(WalWriterSignalCell::new()) }; WAL_WRITER_CAPACITY] }; DB_IO_BACKEND_CONTROLS];
pub(crate) const WAL_WRITER_SIGNAL_BACKING_BYTES: usize = std::mem::size_of_val(&WAL_WRITER_SIGNALS);

fn cell(key: WalWriterKey) -> &'static Mutex<WalWriterSignalCell> {
    let (slot, _) = db_io_backend_parts(key.backend);
    &WAL_WRITER_SIGNALS[usize::from(slot)][usize::from(key.slot)]
}

/// 🧾️ A preflight reservation is cancelled without consuming a terminal epoch if guard construction fails.
pub(crate) struct WalWriterSignalReservation {
    key: WalWriterKey,
    required_epoch: u64,
    committed: bool,
}

impl WalWriterSignalReservation {
    pub(crate) fn commit(mut self) -> WalWriterRelease {
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
    key: WalWriterKey,
    required_epoch: u64,
    requested: bool,
    resolved: bool,
}

/// 🛟️ A failed close returns the same exact retained wait authority instead of discarding it.
pub struct WalWriterReleaseFailure {
    error: DbError,
    release: WalWriterRelease,
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

pub(crate) fn prepare(key: WalWriterKey) -> Result<WalWriterSignalReservation, DbError> {
    let (slot, _) = db_io_backend_parts(key.backend);
    if usize::from(slot) >= DB_IO_BACKEND_CONTROLS || usize::from(key.slot) >= WAL_WRITER_CAPACITY {
        return Err(DbError::LimitExceeded("WAL writer signal authority"));
    }
    let required_epoch = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).prepare(key)?;
    Ok(WalWriterSignalReservation { key, required_epoch, committed: false })
}

pub(crate) fn request(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).request(key)
}

pub(crate) fn requested(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).requested(key)
}

pub(crate) fn fault(key: WalWriterKey, error: &DbError) {
    let mut cell = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if cell.active == Some(key) && cell.fault.is_none() {
        cell.fault = Some(super::super::db_io_error_text(error));
    }
}

pub(crate) fn faulted(key: WalWriterKey) -> bool {
    let cell = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    cell.active == Some(key) && cell.fault.is_some()
}

pub(crate) fn finish(key: WalWriterKey) {
    let mut cell = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(cell.notification.is_none());
    cell.notification = cell.finish(key);
}

pub(crate) fn notify_terminal(backend: super::super::DbIoBackendControl) {
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

fn fault_all_requested(backend: DbIoBackendControl, error: &DbError) {
    let (slot, _) = db_io_backend_parts(backend);
    for cell in &WAL_WRITER_SIGNALS[usize::from(slot)] {
        let mut cell = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if cell.active.is_some_and(|key| key.backend == backend) && cell.requested && cell.fault.is_none() {
            cell.fault = Some(super::super::db_io_error_text(error));
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

struct WalWriterController {
    backend: DbIoBackendControl,
    pool: Arc<WorkerPool>,
    ticket: Option<WorkerMaintenanceTicket>,
    deferred_wake_ticket: Option<WorkerDeferredWakeTicket>,
    waker: Waker,
}

struct WalWriterControllerWake {
    backend: DbIoBackendControl,
}

impl std::task::Wake for WalWriterControllerWake {
    fn wake(self: Arc<Self>) {
        request_controller(self.backend);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        request_controller(self.backend);
    }
}

static WAL_WRITER_CONTROLLERS: [Mutex<Option<WalWriterController>>; DB_IO_BACKEND_CONTROLS] = [const { Mutex::new(None) }; DB_IO_BACKEND_CONTROLS];
pub(crate) const WAL_WRITER_CONTROLLER_BACKING_BYTES: usize = std::mem::size_of_val(&WAL_WRITER_CONTROLLERS);

pub(crate) const fn controller_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: (std::mem::size_of::<WalWriterControllerWake>() + 2 * std::mem::size_of::<usize>()) as u64, items: 1, controls: 1 }
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

pub(crate) fn request_controller(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    let controller = {
        let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| row.backend == backend) else { return };
        let (Some(ticket), Some(deferred_wake_ticket)) = (row.ticket, row.deferred_wake_ticket) else { return };
        (row.pool.clone(), ticket, deferred_wake_ticket)
    };
    if controller.0.request_maintenance(controller.1).is_err() {
        fault_all_requested(backend, &DbError::Unavailable("WAL writer maintenance request refused; exact guard remains retained".to_string()));
        defer_fault_waiters(backend, &controller.0, controller.2);
    }
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

pub(crate) fn defer_fault_notifications(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    let controller = {
        let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| row.backend == backend) else { return };
        let Some(ticket) = row.deferred_wake_ticket else { return };
        (row.pool.clone(), ticket)
    };
    defer_fault_waiters(backend, &controller.0, controller.1);
}

fn defer_fault_waiters(backend: DbIoBackendControl, pool: &WorkerPool, ticket: WorkerDeferredWakeTicket) {
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

fn deferred_wake_pending(key: WalWriterKey) -> bool {
    let (slot, _) = db_io_backend_parts(key.backend);
    let controller = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(controller) = controller.as_ref().filter(|controller| controller.backend == key.backend) else { return false };
    let Some(ticket) = controller.deferred_wake_ticket else { return false };
    controller.pool.deferred_wake_pending(ticket, usize::from(key.slot)).unwrap_or(false)
}

fn controller_step([slot, generation]: [u64; 2]) -> WorkerMaintenanceStep {
    let Some(controller) = WAL_WRITER_CONTROLLERS.get(slot as usize) else { return WorkerMaintenanceStep::Fault };
    let (backend, waker) = {
        let row = controller.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| db_io_backend_parts(row.backend) == (slot as u16, generation)) else { return WorkerMaintenanceStep::Idle };
        (row.backend, row.waker.clone())
    };
    let writer = if has_runnable_request(backend) {
        let result = super::super::db_io_writer_release_lane_step(backend, &mut Context::from_waker(&waker));
        if let Err(error) = &result {
            fault_all_requested(backend, error);
        }
        notify_terminal(backend);
        notify_faults(backend);
        Some(result)
    } else {
        None
    };
    match super::super::db_io_lost_owner_maintenance_batch() {
        Ok(true) => return WorkerMaintenanceStep::More,
        Err(_) => return WorkerMaintenanceStep::Fault,
        Ok(false) => {}
    }
    match writer {
        Some(Ok(DbIoWriterReleaseStep::More | DbIoWriterReleaseStep::Faulted)) => WorkerMaintenanceStep::More,
        Some(Ok(DbIoWriterReleaseStep::Idle)) | None => WorkerMaintenanceStep::Idle,
        Some(Err(_)) => WorkerMaintenanceStep::Idle,
    }
}

pub(crate) fn close_controller(backend: DbIoBackendControl) -> Result<bool, DbError> {
    let (slot, _) = db_io_backend_parts(backend);
    let mut row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(owner) = row.as_mut() else { return Ok(true) };
    if owner.backend != backend {
        return Err(DbError::Fenced { expected: db_io_backend_parts(owner.backend).1, actual: db_io_backend_parts(backend).1 });
    }
    if let Some(ticket) = owner.ticket {
        if !owner.pool.remove_maintenance_hook(ticket).map_err(|error| DbError::Unavailable(format!("WAL writer maintenance retirement: {error:?}")))? {
            return Ok(false);
        }
        owner.ticket = None;
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

#[cfg(test)]
pub(crate) fn suspend_controller_for_refusal(backend: DbIoBackendControl) -> Result<(), DbError> {
    let (slot, _) = db_io_backend_parts(backend);
    let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = row.as_ref().filter(|owner| owner.backend == backend).ok_or_else(|| DbError::Closed)?;
    let ticket = owner.ticket.ok_or(DbError::Closed)?;
    if !owner.pool.remove_maintenance_hook(ticket).map_err(|error| DbError::Unavailable(format!("WAL writer refusal fixture retirement: {error:?}")))? {
        return Err(DbError::Conflict("WAL writer controller is already running".to_string()));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn restore_controller_after_refusal(backend: DbIoBackendControl) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(backend);
    let mut row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = row.as_mut().filter(|owner| owner.backend == backend).ok_or_else(|| DbError::Closed)?;
    let ticket = owner.pool.install_maintenance_hook(Lane::Io, controller_step, [u64::from(slot), generation]).map_err(|error| DbError::Unavailable(format!("WAL writer refusal fixture restore: {error:?}")))?;
    owner.ticket = Some(ticket);
    Ok(())
}

#[cfg(test)]
pub(crate) fn deferred_wake_pending_for_test(key: WalWriterKey) -> bool {
    deferred_wake_pending(key)
}

#[cfg(test)]
mod tests {
    use super::super::super::DbIoBackendControl;
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    struct Counter(AtomicUsize);
    impl std::task::Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn wal_writer_release_signal_preserves_exact_waits_across_writer_and_backend_reuse() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🔣️.json")).unwrap();
        let expected = &fixture["releaseSignal"];
        let counter = Arc::new(Counter(AtomicUsize::new(0)));
        let waker = Waker::from(counter.clone());
        let context = &mut Context::from_waker(&waker);
        let mut cell = WalWriterSignalCell::new();
        assert_eq!(cell.terminal_epoch.to_string(), expected["firstEpoch"]);
        let mut waits = Vec::new();
        for row in expected["writers"].as_array().unwrap() {
            let key = WalWriterKey { backend: DbIoBackendControl::Memory { slot: 0, generation: row["backendGeneration"].as_str().unwrap().parse().unwrap() }, slot: 0, generation: row["writerGeneration"].as_str().unwrap().parse().unwrap() };
            let required = cell.prepare(key).unwrap();
            assert!(matches!(cell.prepare(key), Err(DbError::Conflict(_))));
            for (old, epoch) in &waits {
                assert_eq!(cell.request(*old), expected["staleRequestAccepted"]);
                assert!(!cell.requested(*old));
                assert!(matches!(cell.poll(*old, *epoch, context), Poll::Ready(Ok(()))));
            }
            assert!(cell.request(key));
            assert!(cell.request(key));
            assert!(cell.requested(key));
            assert_eq!(cell.poll(key, required, context).is_ready(), expected["requestCompletesRelease"]);
            assert_eq!(cell.poll(key, required, context).is_ready(), expected["requestCompletesRelease"]);
            assert_eq!(counter.0.load(Ordering::SeqCst), waits.len());
            cell.finish(key).unwrap().wake();
            assert_eq!(cell.terminal_epoch.to_string(), row["terminalEpoch"]);
            assert_eq!(counter.0.load(Ordering::SeqCst), waits.len() + 1);
            waits.push((key, required));
            assert_eq!(waits.iter().all(|(old, epoch)| matches!(cell.poll(*old, *epoch, context), Poll::Ready(Ok(())))), expected["oldWaitSurvivesReuse"]);
        }
        let key = waits.last().unwrap().0;
        let before = cell.terminal_epoch;
        cell.prepare(key).unwrap();
        cell.cancel(key);
        assert_eq!(cell.terminal_epoch, before);
        cell.terminal_epoch = expected["maximumEpoch"].as_str().unwrap().parse().unwrap();
        assert!(matches!(cell.prepare(key), Err(DbError::LimitExceeded("WAL writer terminal epoch"))));
        assert!(cell.active.is_none());
        assert!(cell.waiter.is_none());
        assert_eq!(WAL_WRITER_SIGNAL_BACKING_BYTES, DB_IO_BACKEND_CONTROLS * WAL_WRITER_CAPACITY * std::mem::size_of::<Mutex<WalWriterSignalCell>>());
        eprintln!("[DEBUG] WAL release cells retained requests, woke once at terminal, rejected stale keys and overflow, and preserved all completion epochs through backend reuse");
    }
}
