//! 🎫️ Wal writer permit and admission: the writer capacity, the permit and key and the admission census, lines and waiters. See [`super`].

use super::*;

pub const WAL_WRITER_CAPACITY: usize = 32;

const _: () = assert!(WAL_WRITER_CAPACITY <= u8::MAX as usize);

/// 🎟️ Non-cloneable writer ownership; only its issuing backend may validate or retire it.
pub struct WalWriterPermit {
    pub key: WalWriterKey,
    pub document: DbIoText,
    pub release: Option<crate::WalWriterRelease>,
}

/// 🧷 Opaque task stamp; an old stamp never authorizes a recycled slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalWriterKey {
    pub backend: DbIoBackendControl,
    pub slot: u8,
    pub generation: u64,
}

impl WalWriterKey {
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

impl WalWriterPermit {
    pub fn key(&self) -> WalWriterKey {
        self.key
    }
    pub fn document(&self) -> &DbIoText {
        &self.document
    }

    pub fn release(mut self) -> crate::WalWriterRelease {
        self.release.take().expect("only a backend-bound writer exposes retained release")
    }

    /// 🅿️ Its holder rests: from now until [`Self::resume`] the backend may reclaim this writer for another document.
    pub fn park(&self) {
        if self.release.is_some() {
            crate::park(self.key);
        }
    }

    /// ▶️ Its holder writes again: `true` while this writer is still the holder's, `false` once its backend reclaimed it
    /// (or a release was requested) — the holder then releases this permit and acquires a fresh writer.
    pub fn resume(&self) -> bool {
        self.release.is_none() || crate::resume(self.key)
    }

    pub(super) fn request_release(&self) {
        if self.release.is_some() && crate::request(self.key) {
            crate::request_controller(self.key.backend);
        }
    }
}

impl Drop for WalWriterPermit {
    fn drop(&mut self) {
        self.request_release();
    }
}

//#region 🚦️Admission
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
pub struct WalWriterAdmissionLine {
    pub backend: Option<DbIoBackendControl>,
    next_ticket: u64,
    pub capacity_epoch: u64,
    pub census: WalWriterAdmissionCensus,
    pub waiters: std::collections::VecDeque<(u64, Option<std::task::Waker>)>,
}

impl WalWriterAdmissionLine {
    pub const fn new() -> Self {
        Self {
            backend: None,
            next_ticket: 1,
            capacity_epoch: 0,
            census: WalWriterAdmissionCensus { waited: 0, refused: 0, waiting: 0 },
            waiters: std::collections::VecDeque::new(),
        }
    }

    pub fn rebind(&mut self, backend: DbIoBackendControl) -> Vec<std::task::Waker> {
        let stale = self.waiters.drain(..).filter_map(|(_, waker)| waker).collect();
        self.backend = Some(backend);
        self.next_ticket = 1;
        self.capacity_epoch = 0;
        self.census = WalWriterAdmissionCensus::default();
        stale
    }

    pub fn head(&self) -> Option<u64> {
        self.waiters.front().map(|(ticket, _)| *ticket)
    }

    pub fn join(&mut self) -> Result<u64, DbError> {
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

    pub fn leave(&mut self, ticket: u64) -> Option<std::task::Waker> {
        let was_head = self.head() == Some(ticket);
        self.waiters.retain(|(waiting, _)| *waiting != ticket);
        if was_head {
            self.take_head_waker()
        } else {
            None
        }
    }

    pub fn capacity_changed(&mut self) -> Option<std::task::Waker> {
        self.capacity_epoch = self.capacity_epoch.wrapping_add(1);
        self.take_head_waker()
    }

    fn take_head_waker(&mut self) -> Option<std::task::Waker> {
        self.waiters.front_mut().and_then(|(_, waker)| waker.take())
    }

    pub fn register(&mut self, ticket: u64, waker: &std::task::Waker) -> bool {
        let Some((_, slot)) = self.waiters.iter_mut().find(|(waiting, _)| *waiting == ticket) else { return false };
        if slot.as_ref().is_none_or(|parked| !parked.will_wake(waker)) {
            *slot = Some(waker.clone());
        }
        true
    }

    pub fn census(&self) -> WalWriterAdmissionCensus {
        WalWriterAdmissionCensus { waiting: self.waiters.len(), ..self.census }
    }
}

static WAL_WRITER_ADMISSIONS: [std::sync::Mutex<WalWriterAdmissionLine>; DB_IO_BACKEND_CONTROLS] = [const { std::sync::Mutex::new(WalWriterAdmissionLine::new()) }; DB_IO_BACKEND_CONTROLS];

pub(crate) const WAL_WRITER_ADMISSION_BACKING_BYTES: usize = size_of_val(&WAL_WRITER_ADMISSIONS);

/// 🔗️ `backend`'s line, bound to it on first use; `None` for a backend whose slot a newer generation already owns (it
/// retired). Binding a newer generation wakes the retired one's leftover waiters, which then answer `Closed`.
pub fn with_admission_line<T>(backend: DbIoBackendControl, visit: impl FnOnce(&mut WalWriterAdmissionLine) -> T) -> Option<T> {
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
pub fn capacity_changed(backend: DbIoBackendControl) {
    if let Some(Some(waker)) = with_admission_line(backend, WalWriterAdmissionLine::capacity_changed) {
        waker.wake();
    }
}
//#endregion 🚦️Admission
