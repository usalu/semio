//! 📡️ Wal writer signals, release and controllers: the writer lease and signal cells, the signal reservation, the release and its failure and the writer controllers with deferred fault waiters. See [`super`].

use super::*;

/// 🅿️ Who may use an acquired writer right now: its holder while the holder writes (`Held`), the backend's next
/// acquisition while the holder rests (`Parked` at `since` on its backend's park clock, least recent first), and nobody
/// once the backend reclaimed it for another document — the holder then acquires a fresh writer before it writes again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WalWriterLease {
    Held,
    Parked { since: u64 },
    Reclaimed,
}

pub struct WalWriterSignalCell {
    pub active: Option<WalWriterKey>,
    pub requested: bool,
    pub lease: WalWriterLease,
    pub terminal_epoch: u64,
    pub waiter: Option<Waker>,
    pub notification: Option<Waker>,
    pub fault: Option<DbIoText>,
    deferred_fault_waiter: bool,
}

impl WalWriterSignalCell {
    pub const fn new() -> Self {
        Self { active: None, requested: false, lease: WalWriterLease::Held, terminal_epoch: 0, waiter: None, notification: None, fault: None, deferred_fault_waiter: false }
    }

    pub fn park(&mut self, key: WalWriterKey, since: u64) -> bool {
        if self.active != Some(key) || self.requested || self.lease != WalWriterLease::Held {
            return false;
        }
        self.lease = WalWriterLease::Parked { since };
        true
    }

    pub fn resume(&mut self, key: WalWriterKey) -> bool {
        if self.active != Some(key) || self.requested {
            return false;
        }
        match self.lease {
            WalWriterLease::Held => true,
            WalWriterLease::Parked { .. } => {
                self.lease = WalWriterLease::Held;
                true
            }
            WalWriterLease::Reclaimed => false,
        }
    }

    pub fn parked_since(&self, key: WalWriterKey) -> Option<u64> {
        match self.lease {
            WalWriterLease::Parked { since } if self.active == Some(key) && !self.requested && self.fault.is_none() => Some(since),
            _ => None,
        }
    }

    pub fn reclaim(&mut self, key: WalWriterKey) -> bool {
        if self.parked_since(key).is_none() {
            return false;
        }
        self.lease = WalWriterLease::Reclaimed;
        self.requested = true;
        true
    }

    /// 🔁️ A finished predecessor's undelivered terminal notification is handed back to its caller to
    /// wake: the predecessor's epoch is already published, so the successor never waits on the
    /// controller's asynchronous `notify_terminal` delivery.
    pub fn prepare(&mut self, key: WalWriterKey) -> Result<(u64, Option<Waker>), DbError> {
        if self.active.is_some() || self.deferred_fault_waiter {
            return Err(DbError::Conflict("WAL writer signal occupied".to_string()));
        }
        let required_epoch = self.terminal_epoch.checked_add(1).ok_or(DbError::LimitExceeded("WAL writer terminal epoch"))?;
        self.active = Some(key);
        self.requested = false;
        self.lease = WalWriterLease::Held;
        self.fault = None;
        self.deferred_fault_waiter = false;
        Ok((required_epoch, self.notification.take()))
    }

    pub fn request(&mut self, key: WalWriterKey) -> bool {
        if self.active != Some(key) {
            return false;
        }
        self.requested = true;
        true
    }

    pub fn requested(&self, key: WalWriterKey) -> bool {
        self.active == Some(key) && self.requested
    }

    pub fn cancel(&mut self, key: WalWriterKey) {
        assert_eq!(self.active, Some(key));
        assert!(self.waiter.is_none());
        assert!(!self.deferred_fault_waiter);
        self.active = None;
        self.requested = false;
        self.lease = WalWriterLease::Held;
    }

    pub fn finish(&mut self, key: WalWriterKey) -> Option<Waker> {
        assert_eq!(self.active, Some(key));
        self.terminal_epoch = self.terminal_epoch.checked_add(1).expect("writer acquisition reserved terminal epoch");
        self.active = None;
        self.requested = false;
        self.lease = WalWriterLease::Held;
        self.fault = None;
        self.deferred_fault_waiter = false;
        self.waiter.take()
    }

    pub fn poll(&mut self, key: WalWriterKey, required_epoch: u64, context: &mut Context<'_>) -> Poll<Result<(), DbError>> {
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

pub static WAL_WRITER_SIGNALS: [[Mutex<WalWriterSignalCell>; WAL_WRITER_CAPACITY]; DB_IO_BACKEND_CONTROLS] = [const { [const { Mutex::new(WalWriterSignalCell::new()) }; WAL_WRITER_CAPACITY] }; DB_IO_BACKEND_CONTROLS];

pub const WAL_WRITER_SIGNAL_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_SIGNALS);

static WAL_WRITER_PARK_CLOCKS: [AtomicU64; DB_IO_BACKEND_CONTROLS] = [const { AtomicU64::new(1) }; DB_IO_BACKEND_CONTROLS];

pub(crate) const WAL_WRITER_PARK_CLOCK_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_PARK_CLOCKS);

pub fn cell(key: WalWriterKey) -> &'static Mutex<WalWriterSignalCell> {
    let (slot, _) = db_io_backend_parts(key.backend);
    &WAL_WRITER_SIGNALS[usize::from(slot)][usize::from(key.slot)]
}

/// 🧾️ A preflight reservation is cancelled without consuming a terminal epoch if guard construction fails.
pub struct WalWriterSignalReservation {
    pub key: WalWriterKey,
    required_epoch: u64,
    committed: bool,
}

impl WalWriterSignalReservation {
    pub fn commit(mut self) -> WalWriterRelease {
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
    pub key: WalWriterKey,
    required_epoch: u64,
    pub requested: bool,
    pub resolved: bool,
}

/// 🛟️ A failed close returns the same exact retained wait authority instead of discarding it.
pub struct WalWriterReleaseFailure {
    pub error: DbError,
    pub release: WalWriterRelease,
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
    pub(super) fn request_release(&mut self) {
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

pub fn prepare(key: WalWriterKey) -> Result<WalWriterSignalReservation, DbError> {
    let (slot, _) = db_io_backend_parts(key.backend);
    if usize::from(slot) >= DB_IO_BACKEND_CONTROLS || usize::from(key.slot) >= WAL_WRITER_CAPACITY {
        return Err(DbError::LimitExceeded("WAL writer signal authority"));
    }
    let (required_epoch, predecessor) = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).prepare(key)?;
    if let Some(waiter) = predecessor {
        waiter.wake();
    }
    Ok(WalWriterSignalReservation { key, required_epoch, committed: false })
}

pub fn request(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).request(key)
}

pub fn requested(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).requested(key)
}

pub fn fault(key: WalWriterKey, error: &DbError) {
    let mut cell = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if cell.active == Some(key) && cell.fault.is_none() {
        cell.fault = Some(crate::db_io_error_text(error));
    }
}

pub fn finish(key: WalWriterKey) {
    let mut cell = cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(cell.notification.is_none());
    cell.notification = cell.finish(key);
}

/// 🅿️ The holder rests: until it resumes, its backend may reclaim this writer for another document, least recently parked
/// first, and the acquisition first in the backend's admission line is told a writer became reclaimable.
pub(crate) fn park(key: WalWriterKey) {
    let (slot, _) = db_io_backend_parts(key.backend);
    let since = WAL_WRITER_PARK_CLOCKS[usize::from(slot)].fetch_add(1, Ordering::Relaxed);
    if cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).park(key, since) {
        crate::capacity_changed(key.backend);
    }
}

/// ▶️ The holder writes again: `true` while its writer is still its own (held or parked), `false` once the backend
/// reclaimed it or a release was requested — the holder must acquire a fresh writer first.
pub(crate) fn resume(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).resume(key)
}

/// ⏱️ The park clock reading of a writer its holder parked and nobody reclaimed or asked to release yet.
pub fn parked_since(key: WalWriterKey) -> Option<u64> {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).parked_since(key)
}

/// 🪝️ Takes a parked writer back for the backend: exactly one of the holder's resume and this reclaim wins, so a holder
/// never writes with a writer its backend is releasing. The reclaimed writer is released like a requested one.
pub fn reclaim(key: WalWriterKey) -> bool {
    cell(key).lock().unwrap_or_else(std::sync::PoisonError::into_inner).reclaim(key)
}

pub fn fault_all_requested(backend: DbIoBackendControl, error: &DbError) {
    let (slot, _) = db_io_backend_parts(backend);
    for cell in &WAL_WRITER_SIGNALS[usize::from(slot)] {
        let mut cell = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if cell.active.is_some_and(|key| key.backend == backend) && cell.requested && cell.fault.is_none() {
            cell.fault = Some(crate::db_io_error_text(error));
        }
    }
}

pub struct WalWriterController {
    pub backend: DbIoBackendControl,
    pub pool: Arc<WorkerPool>,
    pub ticket: Option<WorkerMaintenanceTicket>,
    pub deferred_wake_ticket: Option<WorkerDeferredWakeTicket>,
    pub waker: Waker,
}

pub static WAL_WRITER_CONTROLLERS: [Mutex<Option<WalWriterController>>; DB_IO_BACKEND_CONTROLS] = [const { Mutex::new(None) }; DB_IO_BACKEND_CONTROLS];

pub(crate) const WAL_WRITER_CONTROLLER_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_CONTROLLERS);

pub fn request_controller(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    let controller = {
        let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| row.backend == backend) else { return };
        let (Some(ticket), Some(deferred_wake_ticket)) = (row.ticket, row.deferred_wake_ticket) else { return };
        if row.pool.request_maintenance(ticket).is_ok() {
            return;
        }
        (row.pool.clone(), deferred_wake_ticket)
    };
    fault_all_requested(backend, &DbError::Unavailable("WAL writer maintenance request refused; exact guard remains retained".to_string()));
    defer_fault_waiters(backend, &controller.0, controller.1);
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

pub fn defer_fault_waiters(backend: DbIoBackendControl, pool: &WorkerPool, ticket: WorkerDeferredWakeTicket) {
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

pub fn deferred_wake_pending(key: WalWriterKey) -> bool {
    let (slot, _) = db_io_backend_parts(key.backend);
    let controller = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(controller) = controller.as_ref().filter(|controller| controller.backend == key.backend) else { return false };
    let Some(ticket) = controller.deferred_wake_ticket else { return false };
    controller.pool.deferred_wake_pending(ticket, usize::from(key.slot)).unwrap_or(false)
}
