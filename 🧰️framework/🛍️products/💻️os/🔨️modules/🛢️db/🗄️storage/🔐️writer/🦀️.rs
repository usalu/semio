//! 🔐️ Fixed document-writer capabilities and backend-owned exclusion guards.
use super::{DbIoBackendControl, DbIoText};
use crate::DbError;

#[path = "🔔️release/🦀️.rs"]
pub(crate) mod release;

pub(crate) const WAL_WRITER_CAPACITY: usize = 32;
const _: () = assert!(WAL_WRITER_CAPACITY <= u8::MAX as usize);

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
        if document.as_str().is_empty() {
            return Err(DbError::InvalidArgument("empty WAL writer document".to_string()));
        }
        if self.entries.iter().flatten().any(|entry| entry.document == *document) {
            return Err(DbError::Conflict("WAL document already has a writer".to_string()));
        }
        let slot = self.entries.iter().position(Option::is_none).ok_or(DbError::LimitExceeded("WAL writer capacity"))?;
        let next = self.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("WAL writer generation"))?;
        let generation = self.next_generation;
        let key = WalWriterKey { backend: self.backend(), slot: slot as u8, generation };
        let signal = if self.signalled { Some(release::prepare(key)?) } else { None };
        let guard = guard()?;
        self.entries[slot] = Some(WalWriterEntry { document: document.clone(), generation, active_operation: None, releasing: false, guard });
        self.next_generation = next;
        Ok(WalWriterPermit { key, document: document.clone(), release: signal.map(release::WalWriterSignalReservation::commit) })
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
        if self.signalled {
            release::finish(key);
        }
        self.entries[usize::from(key.slot)] = None;
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
