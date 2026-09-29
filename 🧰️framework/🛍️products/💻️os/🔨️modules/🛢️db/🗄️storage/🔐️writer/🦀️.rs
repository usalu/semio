//! 🔐️ Fixed document-writer capabilities and backend-owned exclusion guards. A writer is a backend's scarce, declared
//! resource ([`WAL_WRITER_CAPACITY`] per backend), so a document holds one only while it writes: its holder parks it at
//! rest ([`WalWriterPermit::park`]), the backend reclaims the least recently parked one when an acquisition finds every
//! slot taken, and the holder acquires a fresh writer before it writes again ([`WalWriterPermit::resume`]). An acquisition
//! that finds every writer writing waits in the backend's admission line, first come first served, and only a wait that
//! outlives the declared admission bound ([`WAL_WRITER_ADMISSION_WAIT_MS`]) is refused — typed and retryable
//! ([`WAL_WRITER_ADMISSION_REFUSAL`]), never a silent stall.
use super::{db_io_backend_parts, DbIoBackendControl, DbIoText, DB_IO_BACKEND_CONTROLS};
use crate::DbError;

#[path = "🔔️release/🦀️.rs"]
pub(crate) mod release;

pub(crate) const WAL_WRITER_CAPACITY: usize = 32;
const _: () = assert!(WAL_WRITER_CAPACITY <= u8::MAX as usize);
/// 👣️ Release steps a reclaim spends inside the acquiring turn: a local guard (file lock, none) reaches terminal within
/// two, a remote one hands its unlock to the driver runtime after one and finishes on the release controller.
const WAL_WRITER_RECLAIM_STEPS: usize = 4;

/// 🎟️ Non-cloneable writer ownership; only its issuing backend may validate or retire it.
pub struct WalWriterPermit {
    key: WalWriterKey,
    document: DbIoText,
    release: Option<release::WalWriterRelease>,
}

/// 🧷 Opaque task stamp; an old stamp never authorizes a recycled slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalWriterKey {
    backend: DbIoBackendControl,
    slot: u8,
    generation: u64,
}

impl WalWriterKey {
    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }
}

impl WalWriterPermit {
    pub(crate) fn key(&self) -> WalWriterKey {
        self.key
    }
    pub(crate) fn document(&self) -> &DbIoText {
        &self.document
    }

    pub fn release(mut self) -> release::WalWriterRelease {
        self.release.take().expect("only a backend-bound writer exposes retained release")
    }

    /// 🅿️ Its holder rests: from now until [`Self::resume`] the backend may reclaim this writer for another document.
    pub(crate) fn park(&self) {
        if self.release.is_some() {
            release::park(self.key);
        }
    }

    /// ▶️ Its holder writes again: `true` while this writer is still the holder's, `false` once its backend reclaimed it
    /// (or a release was requested) — the holder then releases this permit and acquires a fresh writer.
    pub(crate) fn resume(&self) -> bool {
        self.release.is_none() || release::resume(self.key)
    }

    fn request_release(&self) {
        if self.release.is_some() && release::request(self.key) {
            release::request_controller(self.key.backend);
        }
    }
}

impl Drop for WalWriterPermit {
    fn drop(&mut self) {
        self.request_release();
    }
}

struct WalWriterEntry<G> {
    document: DbIoText,
    generation: u64,
    active_operation: Option<u64>,
    releasing: bool,
    guard: G,
}

/// 🧹 A backend guard keeps failed or partial release ownership until its terminal witness.
pub(crate) trait WalWriterGuard {
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
pub(crate) struct WalWriterTable<G> {
    backend: Option<DbIoBackendControl>,
    signalled: bool,
    next_generation: u64,
    close_cursor: usize,
    entries: [Option<WalWriterEntry<G>>; WAL_WRITER_CAPACITY],
    #[cfg(test)]
    release_failures: usize,
}

impl<G: WalWriterGuard> WalWriterTable<G> {
    #[cfg(test)]
    pub(crate) fn new(backend: DbIoBackendControl) -> Self {
        Self { backend: Some(backend), signalled: false, next_generation: 1, close_cursor: 0, entries: std::array::from_fn(|_| None), release_failures: 0 }
    }

    pub(crate) fn for_backend(backend: DbIoBackendControl) -> Self {
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

    pub(crate) fn unbound() -> Self {
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

    pub(crate) fn bind(&mut self, backend: DbIoBackendControl) -> Result<(), DbError> {
        if self.backend.is_some() {
            return Err(DbError::Internal("WAL writer table bound twice".to_string()));
        }
        self.backend = Some(backend);
        Ok(())
    }

    fn backend(&self) -> DbIoBackendControl {
        self.backend.expect("writer admission requires a bound backend")
    }

    #[cfg(test)]
    pub(crate) fn fail_next_release(&mut self) {
        self.release_failures = self.release_failures.saturating_add(1);
    }

    pub(crate) fn acquire_with(&mut self, document: &DbIoText, guard: impl FnOnce() -> Result<G, DbError>) -> Result<WalWriterPermit, DbError> {
        let slot = self.ensure_capacity(document)?;
        let next = self.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("WAL writer generation"))?;
        let generation = self.next_generation;
        let key = WalWriterKey { backend: self.backend(), slot: slot as u8, generation };
        let signal = if self.signalled { Some(release::prepare(key)?) } else { None };
        let guard = guard()?;
        self.entries[slot] = Some(WalWriterEntry { document: document.clone(), generation, active_operation: None, releasing: false, guard });
        self.next_generation = next;
        Ok(WalWriterPermit { key, document: document.clone(), release: signal.map(release::WalWriterSignalReservation::commit) })
    }

    /// 🎫️ The slot the next writer for `document` takes: its dropped predecessor's once that is retired, else a free one,
    /// else the one a parked writer frees when the backend reclaims it (least recently parked first). `Conflict` while a
    /// live predecessor holds the document; `LimitExceeded("WAL writer capacity")` while every writer writes, or while a
    /// reclaimed remote guard still unlocks (its completion frees the slot and tells the admission line). Remote backends
    /// call this before they open a lock session, so a full table never costs one.
    pub(crate) fn ensure_capacity(&mut self, document: &DbIoText) -> Result<usize, DbError> {
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
            if let Some(since) = release::parked_since(WalWriterKey { backend, slot: slot as u8, generation: entry.generation }) {
                candidates[parked] = (since, slot);
                parked += 1;
            }
        }
        candidates[..parked].sort_unstable();
        for &(_, slot) in &candidates[..parked] {
            let entry = self.entries[slot].as_ref().expect("selected parked writer");
            let key = WalWriterKey { backend, slot: slot as u8, generation: entry.generation };
            if !release::reclaim(key) {
                continue;
            }
            let document = entry.document.clone();
            for _ in 0..WAL_WRITER_RECLAIM_STEPS {
                match self.release_step(key, backend, &document) {
                    Ok(false) => return Ok(Some(slot)),
                    Ok(true) if self.entries[slot].as_ref().is_some_and(|entry| entry.guard.awaiting_wake()) => break,
                    Ok(true) => {}
                    Err(error) => {
                        release::fault(key, &error);
                        break;
                    }
                }
            }
            release::request_controller(backend);
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
        let abandoned = entry.active_operation.is_none() && (entry.releasing || self.signalled && release::requested(key)) && !(self.signalled && release::faulted(key));
        if !abandoned {
            return Ok(false);
        }
        let document = entry.document.clone();
        Ok(!self.release_step(key, self.backend(), &document)?)
    }

    #[cfg(test)]
    pub(crate) fn acquire(&mut self, document: &DbIoText, guard: G) -> Result<WalWriterPermit, (DbError, G)> {
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

    pub(crate) fn validate(&self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText) -> Result<&G, DbError> {
        let entry = self.matching_entry(key, backend, document)?;
        if entry.releasing || self.signalled && release::requested(key) {
            return Err(DbError::Closed);
        }
        Ok(&entry.guard)
    }

    pub(crate) fn pin_operation(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<&G, DbError> {
        self.matching_entry(key, backend, document)?;
        if operation == 0 {
            return Err(DbError::InvalidArgument("zero WAL writer operation".to_string()));
        }
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        if (entry.releasing || self.signalled && release::requested(key)) && entry.active_operation != Some(operation) {
            return Err(DbError::Closed);
        }
        if entry.active_operation.is_some_and(|active| active != operation) {
            return Err(DbError::Conflict("WAL writer operation already admitted".to_string()));
        }
        entry.active_operation = Some(operation);
        Ok(&entry.guard)
    }

    /// 🔑 The exact pinned operation borrows its guard mutably, e.g. to drive writes through the lock session.
    pub(crate) fn pinned_guard_mut(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<&mut G, DbError> {
        self.matching_entry(key, backend, document)?;
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        if entry.active_operation != Some(operation) {
            return Err(DbError::Fenced { expected: entry.active_operation.unwrap_or(0), actual: operation });
        }
        Ok(&mut entry.guard)
    }

    pub(crate) fn register_wake(&self, waker: &std::task::Waker) {
        for entry in self.entries.iter().flatten() {
            entry.guard.register_wake(waker);
        }
    }

    pub(crate) fn finish_operation(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<(), DbError> {
        self.matching_entry(key, backend, document)?;
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        if entry.active_operation != Some(operation) {
            return Err(DbError::Fenced { expected: entry.active_operation.unwrap_or(0), actual: operation });
        }
        entry.active_operation = None;
        if self.signalled && release::requested(key) {
            release::request_controller(self.backend());
        }
        Ok(())
    }

    pub(crate) fn finish_operation_if_pinned(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<(), DbError> {
        if self.matching_entry(key, backend, document).is_ok_and(|entry| entry.active_operation == Some(operation)) {
            self.finish_operation(key, backend, document, operation)?;
        }
        Ok(())
    }

    pub(crate) fn release_step(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText) -> Result<bool, DbError> {
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
            release::finish(key);
            capacity_changed(key.backend);
        }
        Ok(false)
    }

    pub(crate) fn close_step(&mut self) -> Result<bool, DbError> {
        let Some(slot) = (0..WAL_WRITER_CAPACITY).map(|offset| (self.close_cursor + offset) % WAL_WRITER_CAPACITY).find(|slot| self.entries[*slot].is_some()) else { return Ok(false) };
        self.close_cursor = (slot + 1) % WAL_WRITER_CAPACITY;
        let entry = self.entries[slot].as_ref().expect("selected retained writer slot");
        let key = WalWriterKey { backend: self.backend(), slot: slot as u8, generation: entry.generation };
        let document = entry.document.clone();
        self.release_step(key, self.backend(), &document)?;
        Ok(true)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.entries.iter().all(Option::is_none)
    }

    pub(crate) fn release_requested_step(&mut self) -> Result<super::DbIoWriterReleaseStep, DbError> {
        let Some(slot) = (0..WAL_WRITER_CAPACITY).map(|offset| (self.close_cursor + offset) % WAL_WRITER_CAPACITY).find(|slot| {
            self.entries[*slot].as_ref().is_some_and(|entry| {
                entry.active_operation.is_none()
                    && !entry.guard.awaiting_wake()
                    && (entry.releasing || self.signalled && release::requested(WalWriterKey { backend: self.backend(), slot: *slot as u8, generation: entry.generation }))
                    && (!self.signalled || !release::faulted(WalWriterKey { backend: self.backend(), slot: *slot as u8, generation: entry.generation }))
            })
        }) else {
            return Ok(super::DbIoWriterReleaseStep::Idle);
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
                    release::fault(key, &error);
                    return Ok(super::DbIoWriterReleaseStep::Faulted);
                }
                return Err(error);
            }
        }
        Ok(super::DbIoWriterReleaseStep::More)
    }
}

/// 📁 Cross-process sidecar exclusion; called only from the backend's admitted I/O lane.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct WalFileWriterGuard {
    file: Option<std::fs::File>,
}

#[cfg(not(target_arch = "wasm32"))]
impl WalFileWriterGuard {
    pub(crate) fn try_acquire(path: &std::path::Path) -> Result<Self, DbError> {
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path).map_err(|error| DbError::Io(error.to_string()))?;
        file.try_lock().map_err(|error| match error {
            std::fs::TryLockError::WouldBlock => DbError::Conflict("WAL document already has a filesystem writer".to_string()),
            std::fs::TryLockError::Error(error) => DbError::Io(error.to_string()),
        })?;
        Ok(Self { file: Some(file) })
    }

    pub(crate) fn close_step(&mut self) -> Result<bool, DbError> {
        let Some(file) = self.file.as_ref() else { return Ok(false) };
        file.unlock().map_err(|error| DbError::Io(error.to_string()))?;
        self.file = None;
        Ok(true)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
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
/// 👥️ How many acquisitions may wait in one backend's line at once — declared (`admission.waiters`); one more is refused
/// at once. It bounds the line's memory at this many tickets and wakers per backend.
pub const WAL_WRITER_ADMISSION_WAITERS: usize = 4_096;
/// 🗣️ The developer detail of a refused admission: `DbError::Unavailable`, transient — the caller retries the write.
pub const WAL_WRITER_ADMISSION_REFUSAL: &str = "WAL writer admission refused: every writer of this database is writing";

/// 📊️ One backend's admission line so far: acquisitions that had to wait, waits that were refused, and those waiting now.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalWriterAdmissionCensus {
    pub waited: u64,
    pub refused: u64,
    pub waiting: usize,
}

/// 🚦️ One backend's waiting acquisitions in arrival order and its capacity epoch — bumped whenever a writer slot is freed
/// or a writer is parked. Only the first in line tries again, and only after the epoch moved past what it last saw, so a
/// newcomer never overtakes a waiter and no freed slot goes unnoticed.
struct WalWriterAdmissionLine {
    backend: Option<DbIoBackendControl>,
    next_ticket: u64,
    capacity_epoch: u64,
    census: WalWriterAdmissionCensus,
    waiters: std::collections::VecDeque<(u64, Option<std::task::Waker>)>,
}

impl WalWriterAdmissionLine {
    const fn new() -> Self {
        Self {
            backend: None,
            next_ticket: 1,
            capacity_epoch: 0,
            census: WalWriterAdmissionCensus { waited: 0, refused: 0, waiting: 0 },
            waiters: std::collections::VecDeque::new(),
        }
    }

    fn rebind(&mut self, backend: DbIoBackendControl) -> Vec<std::task::Waker> {
        let stale = self.waiters.drain(..).filter_map(|(_, waker)| waker).collect();
        self.backend = Some(backend);
        self.next_ticket = 1;
        self.capacity_epoch = 0;
        self.census = WalWriterAdmissionCensus::default();
        stale
    }

    fn head(&self) -> Option<u64> {
        self.waiters.front().map(|(ticket, _)| *ticket)
    }

    fn join(&mut self) -> Result<u64, DbError> {
        if self.waiters.len() >= WAL_WRITER_ADMISSION_WAITERS {
            self.census.refused = self.census.refused.saturating_add(1);
            return Err(DbError::Unavailable(WAL_WRITER_ADMISSION_REFUSAL.to_string()));
        }
        let ticket = self.next_ticket;
        self.next_ticket = self.next_ticket.checked_add(1).ok_or(DbError::LimitExceeded("WAL writer admission ticket"))?;
        self.waiters.push_back((ticket, None));
        self.census.waited = self.census.waited.saturating_add(1);
        Ok(ticket)
    }

    fn leave(&mut self, ticket: u64) -> Option<std::task::Waker> {
        let was_head = self.head() == Some(ticket);
        self.waiters.retain(|(waiting, _)| *waiting != ticket);
        if was_head {
            self.take_head_waker()
        } else {
            None
        }
    }

    fn capacity_changed(&mut self) -> Option<std::task::Waker> {
        self.capacity_epoch = self.capacity_epoch.wrapping_add(1);
        self.take_head_waker()
    }

    fn take_head_waker(&mut self) -> Option<std::task::Waker> {
        self.waiters.front_mut().and_then(|(_, waker)| waker.take())
    }

    fn register(&mut self, ticket: u64, waker: &std::task::Waker) -> bool {
        let Some((_, slot)) = self.waiters.iter_mut().find(|(waiting, _)| *waiting == ticket) else { return false };
        if slot.as_ref().is_none_or(|parked| !parked.will_wake(waker)) {
            *slot = Some(waker.clone());
        }
        true
    }

    fn census(&self) -> WalWriterAdmissionCensus {
        WalWriterAdmissionCensus { waiting: self.waiters.len(), ..self.census }
    }
}

static WAL_WRITER_ADMISSIONS: [std::sync::Mutex<WalWriterAdmissionLine>; DB_IO_BACKEND_CONTROLS] = [const { std::sync::Mutex::new(WalWriterAdmissionLine::new()) }; DB_IO_BACKEND_CONTROLS];
pub(crate) const WAL_WRITER_ADMISSION_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_ADMISSIONS);

/// 🔗️ `backend`'s line, bound to it on first use; `None` for a backend whose slot a newer generation already owns (it
/// retired). Binding a newer generation wakes the retired one's leftover waiters, which then answer `Closed`.
fn with_admission_line<T>(backend: DbIoBackendControl, visit: impl FnOnce(&mut WalWriterAdmissionLine) -> T) -> Option<T> {
    let (slot, generation) = db_io_backend_parts(backend);
    let mut line = WAL_WRITER_ADMISSIONS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let stale = match line.backend.map(db_io_backend_parts) {
        Some((_, bound)) if bound == generation => Vec::new(),
        Some((_, bound)) if bound > generation => return None,
        _ => line.rebind(backend),
    };
    let visited = visit(&mut line);
    drop(line);
    for waker in stale {
        waker.wake();
    }
    Some(visited)
}

/// 🔔️ A writer slot of `backend` was freed or a writer was parked: the acquisition first in line tries again.
pub(crate) fn capacity_changed(backend: DbIoBackendControl) {
    if let Some(Some(waker)) = with_admission_line(backend, WalWriterAdmissionLine::capacity_changed) {
        waker.wake();
    }
}

/// 📊️ `backend`'s admission census (all zero for a backend that never had to wait).
pub(crate) fn admission_census(backend: DbIoBackendControl) -> WalWriterAdmissionCensus {
    with_admission_line(backend, |line| line.census()).unwrap_or_default()
}

/// 🎟️ A place in the admission line; dropping it — admitted, refused or cancelled — leaves the line and hands the turn on.
struct WalWriterAdmissionPlace {
    backend: DbIoBackendControl,
    ticket: u64,
}

impl WalWriterAdmissionPlace {
    fn join(backend: DbIoBackendControl) -> Result<Self, DbError> {
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
pub(crate) async fn admitted_acquire<F, Fut>(backend: DbIoBackendControl, wait: std::time::Duration, mut attempt: F) -> Result<WalWriterPermit, DbError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<WalWriterPermit, DbError>>,
{
    let pool = release::controller_pool(backend).ok_or(DbError::Closed)?;
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
    backend: DbIoBackendControl,
    ticket: u64,
    observed: u64,
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(all(test, feature = "sqlite", not(target_arch = "wasm32")))]
#[path = "🧪️tests/🔬️fence-conformance/🦀️.rs"]
mod fence_conformance;
