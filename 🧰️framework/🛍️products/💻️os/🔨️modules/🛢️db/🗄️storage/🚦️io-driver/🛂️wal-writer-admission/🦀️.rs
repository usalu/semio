//! 🛂️ WAL writer admission: the writer admission place, wait and the controller helpers. See [`super`].

use super::*;

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
    pub(super) deadline_ms: u64,
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
