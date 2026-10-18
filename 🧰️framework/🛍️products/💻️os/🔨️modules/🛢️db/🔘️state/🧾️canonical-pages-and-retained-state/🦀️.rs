//! 🧾️ Canonical pages and retained state: canonical encoding, pages, state entries, the retained state map and the page store. See [`super`].

use super::*;

//#region 🔖️Pages
/// 📇️ A type that can serialize itself into a byte buffer in a canonical, deterministic
/// order — the basis every persistent structure's `content_hash` is built on. Implemented for the
/// handful of primitive value types the `db` family's overlays actually store; a higher crate
/// (`db_artifact`) that needs to content-address an application-level value implements this for
/// its own wrapper type rather than `db_state` growing a dependency on a serialization crate.
pub trait CanonicalEncode {
    fn encode_canonical(&self, out: &mut Vec<u8>);
}

impl CanonicalEncode for String {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.len() as u64).to_le_bytes());
        out.extend_from_slice(self.as_bytes());
    }
}

impl CanonicalEncode for Vec<u8> {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.len() as u64).to_le_bytes());
        out.extend_from_slice(self);
    }
}

impl CanonicalEncode for u64 {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}

impl CanonicalEncode for i64 {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}

impl CanonicalEncode for bool {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.push(*self as u8);
    }
}

impl CanonicalEncode for () {
    fn encode_canonical(&self, _out: &mut Vec<u8>) {}
}

/// 🔑️ Hashes `bytes` with blake3, the family's hashing algorithm throughout (matches
/// `pack`/`protocol`'s `ContentHash`).
pub(super) fn hash_bytes(bytes: &[u8]) -> pack::ContentHash {
    pack::ContentHash(*semio_framework_hash::hash(bytes).as_bytes())
}

/// 📦️ An immutable, content-addressed byte page: its `hash` is the blake3 digest of
/// `bytes`, computed once at construction. `db_snapshot` will use pages of this shape as the
/// unit written into `KIND_CHUNK` segments.
pub struct Page {
    pub hash: pack::ContentHash,
    pub pages: db_io_core::DbIoPages,
}

impl Page {
    pub async fn try_from_pages(pages: db_io_core::DbIoPages) -> Result<Page, DbError> {
        if pages.operation() == 0 {
            return Err(DbError::InvalidArgument("state page requires a nonzero DB I/O operation".to_string()));
        }
        let hash = db_io_registry::db_io_hash_pages(&pages).await;
        Ok(Page { hash, pages })
    }

    pub fn operation(&self) -> u64 {
        self.pages.operation()
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn fragments(&self) -> db_io_core::DbIoPageReader<'_> {
        self.pages.fragments()
    }

    pub fn pages(&self) -> &db_io_core::DbIoPages {
        &self.pages
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        self.pages.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.terminal_is_empty()
    }
}

pub(super) const RETAINED_STATE_ENTRIES: usize = 64;

pub struct StateCursorControl {
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub deadline: std::time::Instant,
    pub fuel: usize,
}

impl StateCursorControl {
    pub fn new(cancelled: Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("state cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel })
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("state cursor fuel"));
        }
        self.deadline = deadline;
        self.fuel = fuel;
        Ok(())
    }

    pub fn grant(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("state cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("state cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("state cursor fuel"))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct StateEntry {
    pub key: db_io_core::DbIoText,
    pub value: db_io_core::DbIoPages,
}

#[derive(Debug)]
pub struct StateEntryRejected {
    pub source: Option<Vec<u8>>,
    pub writer: Option<db_io_core::DbIoPageWriter>,
    pub error: DbError,
}

impl StateEntry {
    pub fn try_new(key: &str, value: db_io_core::DbIoPages) -> Result<Self, (DbError, db_io_core::DbIoPages)> {
        match db_io_core::DbIoText::try_from_str(key) {
            Ok(key) => Ok(Self { key, value }),
            Err(error) => Err((error, value)),
        }
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn value(&self) -> &db_io_core::DbIoPages {
        &self.value
    }

    pub fn operation(&self) -> u64 {
        self.value.operation()
    }

    pub fn into_parts(self) -> (db_io_core::DbIoText, db_io_core::DbIoPages) {
        (self.key, self.value)
    }

    pub async fn try_admit(key: &str, source: Vec<u8>, maximum: u64, control: &mut StateCursorControl) -> Result<Self, StateEntryRejected> {
        let key = match db_io_core::DbIoText::try_from_str(key) {
            Ok(key) => key,
            Err(error) => return Err(StateEntryRejected { source: Some(source), writer: None, error }),
        };
        if source.capacity() as u64 > maximum {
            return Err(StateEntryRejected { source: Some(source), writer: None, error: DbError::LimitExceeded("state source backing capacity") });
        }
        let mut writer = match db_io_core::DbIoPageWriter::try_reserve(source.capacity().div_ceil(db_io_core::DB_IO_PAGE_BYTES)) {
            Ok(writer) => writer,
            Err(rejected) => return Err(StateEntryRejected { source: Some(source), writer: rejected.into_writer(), error: DbError::Unavailable("state page admission rejected".to_string()) }),
        };
        let mut reservation = match db_io_core::DbIoDriverReservation::try_reserve(writer.operation(), source.capacity()) {
            Ok(reservation) => reservation,
            Err(error) => return Err(StateEntryRejected { source: Some(source), writer: Some(writer), error }),
        };
        let mut source = db_io_core::DbIoExternalBytes::new(source);
        if let Err(error) = source.capacity().and_then(|capacity| reservation.observe_capacity(capacity)) {
            return Err(StateEntryRejected { source: source.into_value().ok(), writer: Some(writer), error });
        }
        let mut offset = 0;
        while offset < source.as_slice().map_err(|error| StateEntryRejected { source: None, writer: None, error })?.len() {
            if let Err(error) = control.grant() {
                return Err(StateEntryRejected { source: source.into_value().ok(), writer: Some(writer), error });
            }
            match source.as_slice().and_then(|source| writer.write_fragment(&source[offset..])) {
                Ok(written) => offset += written,
                Err(error) => return Err(StateEntryRejected { source: source.into_value().ok(), writer: Some(writer), error }),
            }
            semio_framework_async::yield_once().await;
        }
        while !source.terminal_is_empty() {
            if let Err(error) = control.grant() {
                return Err(StateEntryRejected { source: None, writer: Some(writer), error });
            }
            let _ = source.close_step();
            semio_framework_async::yield_once().await;
        }
        if let Err(error) = reservation.close_step() {
            return Err(StateEntryRejected { source: None, writer: Some(writer), error });
        }
        writer.seal_retained().await.map(|value| Self { key, value }).map_err(|rejected| {
            let (error, writer) = rejected.into_parts();
            StateEntryRejected { source: None, writer, error }
        })
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.value.close_step()?.is_some() {
            return Ok(true);
        }
        Ok(self.key.close_step())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.key.terminal_is_empty() && self.value.terminal_is_empty()
    }
}

impl StateEntryRejected {
    pub fn source(&self) -> Option<&Vec<u8>> {
        self.source.as_ref()
    }

    pub fn into_source(mut self) -> Option<Vec<u8>> {
        self.source.take()
    }

    pub fn error(&self) -> &DbError {
        &self.error
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if let Some(writer) = self.writer.as_mut() {
            if writer.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.writer = None;
        Ok(false)
    }
}

impl Drop for StateEntryRejected {
    fn drop(&mut self) {
        if let Some(source) = self.source.take() {
            drop(db_io_core::DbIoExternalBytes::new(source));
        }
    }
}

pub struct RetainedStateMap {
    pub entries: [Option<StateEntry>; RETAINED_STATE_ENTRIES],
    pub len: u8,
}

impl RetainedStateMap {
    pub fn new() -> Self {
        Self { entries: std::array::from_fn(|_| None), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn position(&self, key: &str) -> Result<usize, usize> {
        self.entries[..self.len()].binary_search_by(|entry| entry.as_ref().map_or(std::cmp::Ordering::Less, |entry| entry.key().cmp(key)))
    }

    pub fn get(&self, key: &str) -> Option<&db_io_core::DbIoPages> {
        self.position(key).ok().and_then(|position| self.entries[position].as_ref().map(StateEntry::value))
    }

    pub fn insert(&mut self, entry: StateEntry) -> Result<Option<StateEntry>, StateEntry> {
        match self.position(entry.key()) {
            Ok(position) => Ok(self.entries[position].replace(entry)),
            Err(position) => {
                if self.len() == RETAINED_STATE_ENTRIES {
                    return Err(entry);
                }
                for index in (position..self.len()).rev() {
                    self.entries[index + 1] = self.entries[index].take();
                }
                self.entries[position] = Some(entry);
                self.len += 1;
                Ok(None)
            }
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<StateEntry> {
        let position = self.position(key).ok()?;
        let removed = self.entries[position].take();
        for index in position + 1..self.len() {
            self.entries[index - 1] = self.entries[index].take();
        }
        self.len -= 1;
        removed
    }

    pub fn iter(&self) -> impl Iterator<Item = &StateEntry> {
        self.entries[..self.len()].iter().flatten()
    }

    pub async fn content_hash(&self, control: &mut StateCursorControl) -> Result<pack::ContentHash, DbError> {
        let mut hash = semio_framework_hash::Hasher::new();
        for entry in self.iter() {
            control.grant()?;
            hash.update(&(entry.key().len() as u64).to_le_bytes());
            hash.update(entry.key().as_bytes());
            hash.update(&(entry.value().len() as u64).to_le_bytes());
            for fragment in entry.value().fragments() {
                control.grant()?;
                hash.update(fragment);
                semio_framework_async::yield_once().await;
            }
        }
        Ok(pack::ContentHash(*hash.finalize().as_bytes()))
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len() - 1;
        let entry = self.entries[index].as_mut().ok_or_else(|| DbError::Internal("retained state close lost entry".to_string()))?;
        if entry.close_step()? {
            return Ok(true);
        }
        self.entries[index] = None;
        self.len -= 1;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.entries.iter().all(Option::is_none)
    }
}

impl Default for RetainedStateMap {
    fn default() -> Self {
        Self::new()
    }
}

/// 🗃️ An in-memory content-addressed page cache: interning identical byte content twice
/// returns the same hash and shares the one underlying allocation — the structural-sharing/dedup
/// mechanism `db_snapshot`'s incremental generations build on.
#[derive(Default)]
#[cfg(test)]
pub struct PageStore {
    pages: std::collections::HashMap<pack::ContentHash, Rc<[u8]>>,
}

#[cfg(test)]
impl PageStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Interns `bytes`, returning its content hash. A byte-identical page already
    /// present is reused (no duplicate allocation, no duplicate `PageStore` entry).
    pub fn intern(&mut self, bytes: Vec<u8>) -> pack::ContentHash {
        let hash = hash_bytes(&bytes);
        self.pages.entry(hash).or_insert_with(|| Rc::from(bytes));
        hash
    }

    pub fn get(&self, hash: &pack::ContentHash) -> Option<Rc<[u8]>> {
        self.pages.get(hash).cloned()
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }
}
//#endregion 🔖️Pages
