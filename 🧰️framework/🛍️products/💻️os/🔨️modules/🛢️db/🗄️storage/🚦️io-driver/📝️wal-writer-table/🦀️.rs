//! 📝️ WAL writer table: the writer guard, the writer table and the file writer guard. See [`super`].

use super::*;

/// 👣️ Release steps a reclaim spends inside the acquiring turn: a local guard (file lock, none) reaches terminal within
/// two, a remote one hands its unlock to the driver runtime after one and finishes on the release controller.
const WAL_WRITER_RECLAIM_STEPS: usize = 4;

struct WalWriterEntry<G> {
    pub document: DbIoText,
    pub generation: u64,
    active_operation: Option<u64>,
    pub(super) releasing: bool,
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
