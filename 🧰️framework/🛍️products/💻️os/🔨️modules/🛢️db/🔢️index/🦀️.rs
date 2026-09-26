//! 🗄️ `db_index` — the `db` family's secondary-index engine: immutable sorted runs merged
//! LSM-lite (append a new sorted+checksummed run per write batch, fold old runs together as they
//! accumulate) underneath typed per-kind index builders for all ten kinds (command, actor-seq,
//! frontier, touched-region, inverse, commit, conflict, projection, full-text, preview — see
//! `IndexKind`'s doc). Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`) and Part 2 of the approved plan.
//!
//! 🎯️ Design choice: this crate has no opinion on what a key/value byte string *means* — that's
//! `db_artifact`'s job (it decides what to index and when). This crate only guarantees the LSM-lite
//! law: for a fixed `(document, kind)`, `get`/`scan_prefix` always resolve to the value written by
//! the most recent `put`/`delete`, regardless of how many runs that history is currently spread
//! across, and `compact`/the automatic merge policy never change what a reader observes — only how
//! many runs it's spread across (checksums via `pack::crc32c` catch on-disk corruption either
//! way). `db_storage::IndexStorage` stores opaque per-`(document, run_id)` byte blobs; this crate
//! owns everything about what's inside a run and how `run_id`s are namespaced per `IndexKind`.

use crate::db_durability::Frontier;
use crate::db_ids::{check_len, ActorId, ArtifactId, DbError};
use crate::*;
use db_storage::IndexStorage;
#[cfg(test)]
use pack::crc32c;
use pack::ByteWriter;

//#region 🔖️Limits
/// @emoji 🛡️ Ceiling on one entry's key, validated via `check_len` before the key's bytes
/// are read off storage (decode side) or written into a run (encode side).
const MAX_KEY_LEN: u64 = 64 * 1024;

/// @emoji 🛡️ Ceiling on one entry's value — generous enough for a serialized `Frontier`/postings
/// list/location pointer, small enough to refuse an obviously-corrupt on-disk length before
/// allocating it.
const MAX_VALUE_LEN: u64 = 16 * 1024 * 1024;

/// @emoji 🛡️ Ceiling on the number of entries a single run may hold, checked against the header's
/// `entry_count` field before admitting decoded fixed entry slots.
const MAX_RUN_ENTRIES: u64 = 64;

/// @emoji 🧺️ The entries one run holds at most — what an owner batching its own entries fills.
pub const RUN_ENTRIES_MAX: usize = MAX_RUN_ENTRIES as usize;
//#endregion 🔖️Limits

//#region 🔖️IndexKind
/// @emoji 🗂️ The ten index namespaces `db_artifact`/`db_conflict`/`db_projection`/`db_query` build
/// on top of this crate's sorted-run engine (per the contract's per-crate responsibility line for
/// `db_index`). Every kind shares the same generic `IndexHandle` mechanism (`put`/`get`/`delete`/
/// `scan_prefix`/`compact`/`stats` all work identically for any kind); the typed wrappers below
/// (`CommandIndex`, `ActorSeqIndex`, `FrontierIndex`, `TouchedRegionIndex`, `InverseIndex`,
/// `CommitIndex`, `ConflictIndex`, `ProjectionIndex`, `FullTextIndex`, `PreviewIndex`) give every
/// kind a key/value codec on top. Each typed wrapper's value shape is deliberately opaque bytes
/// (`db_conflict::ConflictRecord`, projection state, a preview payload, …) supplied by the caller —
/// this crate never depends on the crates that own those shapes (see the module doc's design note).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IndexKind {
    Command,
    ActorSeq,
    Frontier,
    TouchedRegion,
    Inverse,
    Commit,
    Conflict,
    Projection,
    FullText,
    Preview,
}

impl IndexKind {
    /// @emoji 📋️ Every kind, for tests and for callers that want to enumerate/verify a document's
    /// whole index (e.g. `db_cli verify`).
    pub const ALL: [IndexKind; 10] = [IndexKind::Command, IndexKind::ActorSeq, IndexKind::Frontier, IndexKind::TouchedRegion, IndexKind::Inverse, IndexKind::Commit, IndexKind::Conflict, IndexKind::Projection, IndexKind::FullText, IndexKind::Preview];

    /// @emoji 🏷️ The one-byte tag stamped in every run's header and packed into the high byte of
    /// its `run_id`s (see `make_run_id`) — this crate's own on-disk representation, not part of the
    /// frozen contract.
    fn tag(self) -> u8 {
        match self {
            IndexKind::Command => 1,
            IndexKind::ActorSeq => 2,
            IndexKind::Frontier => 3,
            IndexKind::TouchedRegion => 4,
            IndexKind::Inverse => 5,
            IndexKind::Commit => 6,
            IndexKind::Conflict => 7,
            IndexKind::Projection => 8,
            IndexKind::FullText => 9,
            IndexKind::Preview => 10,
        }
    }
}

/// @emoji 🔢️ A `run_id`'s layout, high to low: `[format:4][kind:4][sequence:50][entries-1:6]`.
/// `db_storage::IndexStorage` addresses runs by a single flat `u64` per document; the high byte
/// namespaces the runs of one kind (and this crate's run-id format) so ten kinds share one document's
/// storage without colliding, and the low bits carry the run's entry count, so a listing alone tells
/// the merge policy every run's size — no run is read just to learn how many entries it holds.
const RUN_ID_FORMAT: u64 = 1;
const RUN_NAMESPACE_SHIFT: u32 = 56;
const RUN_ENTRY_BITS: u32 = 6;
const RUN_ENTRY_MASK: u64 = (1u64 << RUN_ENTRY_BITS) - 1;
const SEQUENCE_BITS: u32 = RUN_NAMESPACE_SHIFT - RUN_ENTRY_BITS;
const SEQUENCE_MASK: u64 = (1u64 << SEQUENCE_BITS) - 1;
const _: () = assert!(MAX_RUN_ENTRIES == 1 << RUN_ENTRY_BITS);

/// @emoji 🏷️ The high byte every run of `kind` carries in its id.
fn run_namespace(kind: IndexKind) -> u8 {
    ((RUN_ID_FORMAT << 4) | u64::from(kind.tag())) as u8
}

/// @emoji 🧮️ Packs `kind`, `sequence` and the run's `entries` count into one `run_id`. Errors
/// `LimitExceeded` if `sequence` doesn't fit its 50 bits (2^50 runs of one kind for one document) or
/// `entries` is outside `1..=MAX_RUN_ENTRIES` (an empty run is never written).
fn make_run_id(kind: IndexKind, sequence: u64, entries: usize) -> Result<u64, DbError> {
    if sequence > SEQUENCE_MASK {
        return Err(DbError::LimitExceeded("db_index run sequence exceeds the 50-bit per-kind namespace"));
    }
    if entries == 0 || entries as u64 > MAX_RUN_ENTRIES {
        return Err(DbError::LimitExceeded("db_index run entry count"));
    }
    Ok((u64::from(run_namespace(kind)) << RUN_NAMESPACE_SHIFT) | (sequence << RUN_ENTRY_BITS) | (entries as u64 - 1))
}

fn namespace_of_run_id(run_id: u64) -> u8 {
    (run_id >> RUN_NAMESPACE_SHIFT) as u8
}

fn sequence_of_run_id(run_id: u64) -> u64 {
    (run_id >> RUN_ENTRY_BITS) & SEQUENCE_MASK
}

fn entries_of_run_id(run_id: u64) -> u64 {
    (run_id & RUN_ENTRY_MASK) + 1
}
//#endregion 🔖️IndexKind

//#region 🔖️SortedRun
/// @emoji 📇️ One entry's value in a sorted run: either a live payload or a tombstone recording that
/// a key was deleted (and must keep shadowing that key in any older, not-yet-merged run beneath).
pub struct IndexCursorControl {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline: std::time::Instant,
    fuel: usize,
    cooperative: Option<(std::time::Duration, usize)>,
}

impl IndexCursorControl {
    pub fn new(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("index cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel, cooperative: None })
    }

    fn retained(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        let duration = deadline.saturating_duration_since(std::time::Instant::now());
        if fuel == 0 || duration.is_zero() {
            return Err(DbError::LimitExceeded("index cursor retained budget"));
        }
        Ok(Self { cancelled, deadline, fuel, cooperative: Some((duration, fuel)) })
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("index cursor fuel"));
        }
        self.deadline = deadline;
        self.fuel = fuel;
        Ok(())
    }

    pub fn grant(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("index cursor cancelled".to_string()));
        }
        let now = std::time::Instant::now();
        if (now >= self.deadline || self.fuel == 0) && self.cooperative.is_some() {
            let (duration, fuel) = self.cooperative.expect("checked retained index budget");
            std::thread::yield_now();
            self.deadline = std::time::Instant::now() + duration;
            self.fuel = fuel;
        } else if now >= self.deadline {
            return Err(DbError::Unavailable("index cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("index cursor fuel"))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct IndexBytes {
    pages: db_storage::DbIoPages,
}

impl PartialEq for IndexBytes {
    fn eq(&self, expected: &Self) -> bool {
        self.pages == expected.pages
    }
}

impl Eq for IndexBytes {}

#[derive(Debug)]
pub struct IndexBytesRejected {
    source: Option<Vec<u8>>,
    writer: Option<db_storage::DbIoPageWriter>,
    error: DbError,
}

impl IndexBytes {
    pub async fn try_admit(source: Vec<u8>, maximum: u64, control: &mut IndexCursorControl) -> Result<Self, IndexBytesRejected> {
        if source.capacity() as u64 > maximum {
            return Err(IndexBytesRejected { source: Some(source), writer: None, error: DbError::LimitExceeded("index source backing capacity") });
        }
        let pages = source.capacity().div_ceil(db_storage::DB_IO_PAGE_BYTES);
        let mut writer = match db_storage::DbIoPageWriter::try_reserve(pages) {
            Ok(writer) => writer,
            Err(error) => return Err(IndexBytesRejected { source: Some(source), writer: error.into_writer(), error: DbError::Unavailable("index page admission rejected".to_string()) }),
        };
        let mut reservation = match db_storage::DbIoDriverReservation::try_reserve(writer.operation(), source.capacity()) {
            Ok(reservation) => reservation,
            Err(error) => return Err(IndexBytesRejected { source: Some(source), writer: Some(writer), error }),
        };
        let mut source = db_storage::DbIoExternalBytes::new(source);
        if let Err(error) = source.capacity().and_then(|capacity| reservation.observe_capacity(capacity)) {
            return Err(IndexBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
        }
        let mut offset = 0;
        while offset < source.as_slice().map_err(|error| IndexBytesRejected { source: None, writer: None, error })?.len() {
            if let Err(error) = control.grant() {
                return Err(IndexBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
            }
            match source.as_slice().and_then(|source| writer.write_fragment(&source[offset..])) {
                Ok(written) => offset += written,
                Err(error) => return Err(IndexBytesRejected { source: source.into_value().ok(), writer: Some(writer), error }),
            }
            semio_framework_async::yield_once().await;
        }
        while !source.terminal_is_empty() {
            if let Err(error) = control.grant() {
                return Err(IndexBytesRejected { source: None, writer: Some(writer), error });
            }
            let _ = source.close_step();
            semio_framework_async::yield_once().await;
        }
        if let Err(error) = reservation.close_step() {
            return Err(IndexBytesRejected { source: None, writer: Some(writer), error });
        }
        writer.seal_retained().await.map(|pages| Self { pages }).map_err(|rejected| {
            let (error, writer) = rejected.into_parts();
            IndexBytesRejected { source: None, writer, error }
        })
    }

    pub fn operation(&self) -> u64 {
        self.pages.operation()
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    pub fn fragments(&self) -> db_storage::DbIoPageReader<'_> {
        self.pages.fragments()
    }

    #[cfg(test)]
    async fn prepare_platform(&self) -> Result<db_storage::DbIoPlatformBuffer, DbError> {
        db_storage::db_io_prepare_platform(&self.pages)?.await
    }

    pub async fn copy_for_operation(operation: u64, bytes: &[u8], control: &mut IndexCursorControl) -> Result<Self, DbError> {
        index_bytes_from_slice_for_operation(operation, bytes, control).await
    }

    pub fn read_fragment(&self, offset: usize, output: &mut [u8]) -> usize {
        if offset >= self.len() || output.is_empty() {
            return 0;
        }
        let page = (offset / db_storage::DB_IO_PAGE_BYTES) as u8;
        let page_offset = offset % db_storage::DB_IO_PAGE_BYTES;
        let Some(fragment) = self.pages.page(page) else { return 0 };
        let read = output.len().min(fragment.len().saturating_sub(page_offset));
        output[..read].copy_from_slice(&fragment[page_offset..page_offset + read]);
        read
    }

    pub fn starts_with(&self, prefix: &IndexBytes) -> bool {
        index_bytes_compare_prefix(self, prefix)
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        self.pages.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.terminal_is_empty()
    }
}

impl IndexBytesRejected {
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

impl Drop for IndexBytesRejected {
    fn drop(&mut self) {
        if let Some(source) = self.source.take() {
            drop(db_storage::DbIoExternalBytes::new(source));
        }
    }
}

async fn admit_generated_index_bytes(source: Vec<u8>, maximum: u64, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    match IndexBytes::try_admit(source, maximum, control).await {
        Ok(bytes) => Ok(bytes),
        Err(mut rejected) => {
            let error = std::mem::replace(&mut rejected.error, DbError::Closed);
            loop {
                control.grant()?;
                if !rejected.close_step()? {
                    break;
                }
                semio_framework_async::yield_once().await;
            }
            if let Some(source) = rejected.source.take() {
                let mut source = db_storage::DbIoExternalBytes::new(source);
                while !source.terminal_is_empty() {
                    control.grant()?;
                    let _ = source.close_step();
                    semio_framework_async::yield_once().await;
                }
            }
            Err(error)
        }
    }
}

async fn close_index_bytes(mut bytes: IndexBytes, control: &mut IndexCursorControl) -> Result<(), DbError> {
    control.grant()?;
    let _ = bytes.close_step()?;
    drop(bytes);
    Ok(())
}

fn index_bytes_cmp(left: &IndexBytes, right: &IndexBytes) -> std::cmp::Ordering {
    let mut left_fragments = left.fragments();
    let mut right_fragments = right.fragments();
    let mut left_fragment = left_fragments.next().unwrap_or_default();
    let mut right_fragment = right_fragments.next().unwrap_or_default();
    let (mut left_offset, mut right_offset) = (0, 0);
    loop {
        let compared = (left_fragment.len() - left_offset).min(right_fragment.len() - right_offset);
        let order = left_fragment[left_offset..left_offset + compared].cmp(&right_fragment[right_offset..right_offset + compared]);
        if order != std::cmp::Ordering::Equal {
            return order;
        }
        left_offset += compared;
        right_offset += compared;
        if left_offset == left_fragment.len() {
            match left_fragments.next() {
                Some(next) => {
                    left_fragment = next;
                    left_offset = 0;
                }
                None => return if right_offset == right_fragment.len() && right_fragments.next().is_none() { std::cmp::Ordering::Equal } else { std::cmp::Ordering::Less },
            }
        }
        if right_offset == right_fragment.len() {
            match right_fragments.next() {
                Some(next) => {
                    right_fragment = next;
                    right_offset = 0;
                }
                None => return std::cmp::Ordering::Greater,
            }
        }
    }
}

fn index_bytes_compare_prefix(value: &IndexBytes, prefix: &IndexBytes) -> bool {
    if prefix.len() > value.len() {
        return false;
    }
    let mut compared = 0;
    for fragment in value.fragments() {
        for byte in fragment {
            let Some(prefix_byte) = prefix.pages.page((compared / db_storage::DB_IO_PAGE_BYTES) as u8).and_then(|page| page.get(compared % db_storage::DB_IO_PAGE_BYTES)) else { return true };
            if byte != prefix_byte {
                return false;
            }
            compared += 1;
            if compared == prefix.len() {
                return true;
            }
        }
    }
    prefix.is_empty()
}

#[derive(Debug)]
pub enum RunValue {
    Put(IndexBytes),
    Tombstone,
}

/// @emoji 📌️ One `(key, value)` pair inside a sorted run. A well-formed run's entries are strictly
/// ascending and unique by `key` — both `encode_run` (on the way in) and `decode_run` (on the way
/// back out, defending against on-disk corruption) enforce this.
#[derive(Debug)]
pub struct RunEntry {
    pub key: IndexBytes,
    pub value: RunValue,
}

pub struct RunEntries {
    entries: Box<[Option<RunEntry>]>,
    len: u8,
}

impl RunEntries {
    pub fn new() -> Self {
        Self { entries: (0..MAX_RUN_ENTRIES).map(|_| None).collect(), len: 0 }
    }

    pub fn push(&mut self, entry: RunEntry) -> Result<(), RunEntry> {
        let Some(slot) = self.entries.get_mut(self.len as usize) else { return Err(entry) };
        *slot = Some(entry);
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&RunEntry> {
        self.entries.get(index)?.as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut RunEntry> {
        self.entries.get_mut(index)?.as_mut()
    }

    pub fn take(&mut self, index: usize) -> Option<RunEntry> {
        self.entries.get_mut(index)?.take()
    }

    pub fn pop(&mut self) -> Option<RunEntry> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.entries[self.len as usize].take()
    }

    pub fn sort_step(&mut self, left: usize, right: usize, control: &mut IndexCursorControl) -> Result<bool, DbError> {
        control.grant()?;
        let swap = match (self.get(left), self.get(right)) {
            (Some(left), Some(right)) => index_bytes_cmp(&left.key, &right.key) == std::cmp::Ordering::Greater,
            _ => false,
        };
        if swap {
            self.entries.swap(left, right);
        }
        Ok(swap)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len as usize - 1;
        let Some(entry) = self.entries[index].as_mut() else {
            self.len -= 1;
            return Ok(true);
        };
        if let RunValue::Put(value) = &mut entry.value {
            if value.close_step()?.is_some() {
                return Ok(true);
            }
        }
        if entry.key.close_step()?.is_some() {
            return Ok(true);
        }
        self.entries[index] = None;
        self.len -= 1;
        Ok(true)
    }
}

/// @emoji 🪧️ A run's 6-byte header: 4-byte magic, 1-byte format version, 1-byte `IndexKind` tag —
/// see `read_run_header`.
const RUN_MAGIC: [u8; 4] = *b"DBIR";
const RUN_VERSION: u8 = 1;

/// @emoji 📐️ A run header's parsed fields plus how many bytes of `body` it occupied, so the caller
/// knows where the entry stream starts.
struct RunHeader {
    entry_count: u64,
}

struct RunPageReader<'pages> {
    pages: &'pages db_storage::DbIoPages,
    position: usize,
    limit: usize,
}

impl<'pages> RunPageReader<'pages> {
    fn new(pages: &'pages db_storage::DbIoPages, limit: usize) -> Self {
        Self { pages, position: 0, limit }
    }

    fn fragment(&self) -> Result<&'pages [u8], DbError> {
        if self.position >= self.limit {
            return Err(DbError::Corrupt("index run ended mid-field".to_string()));
        }
        let mut base = 0usize;
        for fragment in self.pages.fragments() {
            let end = base + fragment.len();
            if self.position < end {
                return Ok(&fragment[self.position - base..fragment.len().min(self.limit - base)]);
            }
            base = end;
        }
        Err(DbError::Corrupt("index run retained page cursor lost its fragment".to_string()))
    }

    fn byte(&mut self) -> Result<u8, DbError> {
        let byte = self.fragment()?[0];
        self.position += 1;
        Ok(byte)
    }

    fn varint(&mut self) -> Result<u64, DbError> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.byte()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("index run varint exceeds u64".to_string()))
    }

    fn skip(&mut self, len: usize) -> Result<RunRange, DbError> {
        let end = self.position.checked_add(len).ok_or(DbError::LimitExceeded("index run field cursor"))?;
        if end > self.limit {
            return Err(DbError::Corrupt("index run field exceeds retained body".to_string()));
        }
        let range = RunRange { start: self.position, len };
        self.position = end;
        Ok(range)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], DbError> {
        let mut output = [0u8; N];
        let mut written = 0usize;
        while written < N {
            let fragment = self.fragment()?;
            let count = (N - written).min(fragment.len());
            output[written..written + count].copy_from_slice(&fragment[..count]);
            self.position += count;
            written += count;
        }
        Ok(output)
    }
}

async fn read_run_header(reader: &mut RunPageReader<'_>, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunHeader, DbError> {
    control.grant()?;
    if reader.array::<4>()? != RUN_MAGIC {
        return Err(DbError::Corrupt("index run has a bad magic".to_string()));
    }
    let version = reader.byte()?;
    if version != RUN_VERSION {
        return Err(DbError::Corrupt(format!("unsupported index run version {version}")));
    }
    let kind_tag = reader.byte()?;
    if kind_tag != expected_kind.tag() {
        return Err(DbError::Corrupt(format!("index run kind mismatch: expected {expected_kind:?} (tag {}), found tag {kind_tag}", expected_kind.tag())));
    }
    let entry_count = reader.varint()?;
    check_len(entry_count, MAX_RUN_ENTRIES, "db_index::entries")?;
    Ok(RunHeader { entry_count })
}

/// @emoji ✍️ Encodes a well-formed (strictly ascending, unique-by-key) entry list into one run's
/// bytes: `MAGIC(4) VERSION(1) KIND(1) entry_count(varint) entries... crc32c(4, LE)`. Each entry is
/// `key_len(varint) key value_tag(1: 0=tombstone,1=put) [value_len(varint) value]`. Errors
/// `InvalidArgument` if `entries` isn't strictly ascending — this fn never silently re-sorts, since
/// a caller with unsorted/duplicate entries must use the bounded incremental sorter first.
fn varint_len(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 0x80 {
        value >>= 7;
        len += 1;
    }
    len
}

fn encode_varint(mut value: u64, buffer: &mut [u8; 10]) -> &[u8] {
    let mut cursor = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buffer[cursor] = byte;
        cursor += 1;
        if value == 0 {
            return &buffer[..cursor];
        }
    }
}

async fn run_write(writer: &mut db_storage::DbIoPageWriter, checksum: &mut pack::codec::Crc32cCursor, bytes: &[u8]) -> Result<(), DbError> {
    let mut cursor = 0;
    while cursor < bytes.len() {
        let written = writer.write_fragment(&bytes[cursor..])?;
        checksum.update_page(&bytes[cursor..cursor + written]);
        cursor += written;
        semio_framework_async::yield_once().await;
    }
    Ok(())
}

async fn run_write_trailer(writer: &mut db_storage::DbIoPageWriter, bytes: &[u8]) -> Result<(), DbError> {
    let mut cursor = 0;
    while cursor < bytes.len() {
        cursor += writer.write_fragment(&bytes[cursor..])?;
        semio_framework_async::yield_once().await;
    }
    Ok(())
}

async fn run_write_pages(writer: &mut db_storage::DbIoPageWriter, checksum: &mut pack::codec::Crc32cCursor, bytes: &IndexBytes, control: &mut IndexCursorControl) -> Result<(), DbError> {
    for fragment in bytes.fragments() {
        control.grant()?;
        run_write(writer, checksum, fragment).await?;
    }
    Ok(())
}

async fn encode_run_pages(kind: IndexKind, entries: &RunEntries, control: &mut IndexCursorControl) -> Result<db_storage::DbIoPages, DbError> {
    check_len(entries.len() as u64, MAX_RUN_ENTRIES, "db_index::entries")?;
    let mut encoded_len = RUN_MAGIC.len() + 2 + varint_len(entries.len() as u64) + 4;
    let mut previous_key: Option<&IndexBytes> = None;
    for index in 0..entries.len() {
        control.grant()?;
        let entry = entries.get(index).ok_or_else(|| DbError::Internal("index run entry slot lost".to_string()))?;
        if let Some(previous) = previous_key {
            if index_bytes_cmp(&entry.key, previous) != std::cmp::Ordering::Greater {
                return Err(DbError::InvalidArgument("db_index run entries must be strictly ascending and unique by key".to_string()));
            }
        }
        previous_key = Some(&entry.key);
        check_len(entry.key.len() as u64, MAX_KEY_LEN, "db_index::key")?;
        encoded_len = encoded_len.checked_add(varint_len(entry.key.len() as u64)).and_then(|len| len.checked_add(entry.key.len() + 1)).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
        match &entry.value {
            RunValue::Tombstone => {}
            RunValue::Put(value) => {
                check_len(value.len() as u64, MAX_VALUE_LEN, "db_index::value")?;
                encoded_len = encoded_len.checked_add(varint_len(value.len() as u64)).and_then(|len| len.checked_add(value.len())).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
            }
        }
        semio_framework_async::yield_once().await;
    }
    let mut writer = db_storage::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    let mut checksum = pack::codec::Crc32cCursor::new();
    run_write(&mut writer, &mut checksum, &RUN_MAGIC).await?;
    run_write(&mut writer, &mut checksum, &[RUN_VERSION, kind.tag()]).await?;
    let mut varint = [0u8; 10];
    run_write(&mut writer, &mut checksum, encode_varint(entries.len() as u64, &mut varint)).await?;
    for index in 0..entries.len() {
        control.grant()?;
        let entry = entries.get(index).ok_or_else(|| DbError::Internal("index run entry slot lost".to_string()))?;
        run_write(&mut writer, &mut checksum, encode_varint(entry.key.len() as u64, &mut varint)).await?;
        run_write_pages(&mut writer, &mut checksum, &entry.key, control).await?;
        match &entry.value {
            RunValue::Tombstone => run_write(&mut writer, &mut checksum, &[0]).await?,
            RunValue::Put(value) => {
                run_write(&mut writer, &mut checksum, &[1]).await?;
                run_write(&mut writer, &mut checksum, encode_varint(value.len() as u64, &mut varint)).await?;
                run_write_pages(&mut writer, &mut checksum, value, control).await?;
            }
        }
    }
    run_write_trailer(&mut writer, &checksum.finish().to_le_bytes()).await?;
    writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
}

/// @emoji 📥️ Encodes strictly ascending, unique `(key, value)` puts into one run's bytes (the same
/// layout as `encode_run_pages`) straight from caller-owned slices, under one page writer.
async fn encode_sorted_run_pages(kind: IndexKind, entries: &[(&[u8], &[u8])], control: &mut IndexCursorControl) -> Result<db_storage::DbIoPages, DbError> {
    check_len(entries.len() as u64, MAX_RUN_ENTRIES, "db_index::entries")?;
    let mut encoded_len = RUN_MAGIC.len() + 2 + varint_len(entries.len() as u64) + 4;
    for (index, (key, value)) in entries.iter().enumerate() {
        control.grant()?;
        if index > 0 && entries[index - 1].0 >= *key {
            return Err(DbError::InvalidArgument("db_index sorted run entries must be strictly ascending and unique by key".to_string()));
        }
        check_len(key.len() as u64, MAX_KEY_LEN, "db_index::key")?;
        check_len(value.len() as u64, MAX_VALUE_LEN, "db_index::value")?;
        encoded_len = encoded_len
            .checked_add(varint_len(key.len() as u64) + key.len() + 1 + varint_len(value.len() as u64) + value.len())
            .ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
    }
    let mut writer = db_storage::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    let mut checksum = pack::codec::Crc32cCursor::new();
    let mut varint = [0u8; 10];
    let written = async {
        run_write(&mut writer, &mut checksum, &RUN_MAGIC).await?;
        run_write(&mut writer, &mut checksum, &[RUN_VERSION, kind.tag()]).await?;
        run_write(&mut writer, &mut checksum, encode_varint(entries.len() as u64, &mut varint)).await?;
        for (key, value) in entries {
            control.grant()?;
            run_write(&mut writer, &mut checksum, encode_varint(key.len() as u64, &mut varint)).await?;
            run_write(&mut writer, &mut checksum, key).await?;
            run_write(&mut writer, &mut checksum, &[1]).await?;
            run_write(&mut writer, &mut checksum, encode_varint(value.len() as u64, &mut varint)).await?;
            run_write(&mut writer, &mut checksum, value).await?;
        }
        Ok::<(), DbError>(())
    }
    .await;
    if let Err(error) = written {
        let _ = writer.seal_retained().await.map(close_run_pages);
        return Err(error);
    }
    run_write_trailer(&mut writer, &checksum.finish().to_le_bytes()).await?;
    writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
}

async fn index_bytes_from_reader(operation: u64, reader: &mut RunPageReader<'_>, len: usize, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(operation, len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    let end = reader.position.checked_add(len).ok_or(DbError::LimitExceeded("index run field cursor"))?;
    if end > reader.limit {
        return Err(DbError::Corrupt("index run field exceeds retained body".to_string()));
    }
    while reader.position < end {
        control.grant()?;
        let fragment = reader.fragment()?;
        let count = (end - reader.position).min(fragment.len());
        let written = writer.write_fragment(&fragment[..count])?;
        reader.position += written;
    }
    writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_storage::DbIoPageWriterRejected::into_error)
}

async fn index_bytes_from_slice_for_operation(operation: u64, bytes: &[u8], control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(operation, bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    let mut offset = 0usize;
    while offset < bytes.len() {
        control.grant()?;
        offset += writer.write_fragment(&bytes[offset..])?;
    }
    writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_storage::DbIoPageWriterRejected::into_error)
}

#[cfg(test)]
async fn decode_run_pages_inner(pages: &db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
    control.grant()?;
    let operation = pages.operation();
    if pages.len() < 4 {
        return Err(DbError::Corrupt("index run is shorter than its checksum trailer".to_string()));
    }
    let body_len = pages.len() - 4;
    let mut checksum = pack::codec::Crc32cCursor::new();
    let mut remaining = body_len;
    for fragment in pages.fragments() {
        control.grant()?;
        let count = remaining.min(fragment.len());
        checksum.update_page(&fragment[..count]);
        remaining -= count;
        if remaining == 0 {
            break;
        }
    }
    let mut trailer = RunPageReader::new(&pages, pages.len());
    trailer.position = body_len;
    if checksum.finish() != u32::from_le_bytes(trailer.array::<4>()?) {
        return Err(DbError::Corrupt("index run checksum mismatch".to_string()));
    }
    let mut reader = RunPageReader::new(&pages, body_len);
    let header = read_run_header(&mut reader, expected_kind, control).await?;
    let mut entries = RunEntries::new();
    for _ in 0..header.entry_count {
        control.grant()?;
        let key_len = reader.varint()?;
        check_len(key_len, MAX_KEY_LEN, "db_index::key")?;
        let key = index_bytes_from_reader(operation, &mut reader, key_len as usize, control).await?;
        if entries.get(entries.len().saturating_sub(1)).is_some_and(|previous| index_bytes_cmp(&key, &previous.key) != std::cmp::Ordering::Greater) {
            return Err(DbError::Corrupt("index run entries are not strictly ascending by key".to_string()));
        }
        let value = match reader.byte()? {
            0 => RunValue::Tombstone,
            1 => {
                let value_len = reader.varint()?;
                check_len(value_len, MAX_VALUE_LEN, "db_index::value")?;
                RunValue::Put(index_bytes_from_reader(operation, &mut reader, value_len as usize, control).await?)
            }
            other => return Err(DbError::Corrupt(format!("index run entry has unknown value tag {other}"))),
        };
        entries.push(RunEntry { key, value }).map_err(|_| DbError::LimitExceeded("index run fixed entry owner"))?;
    }
    if reader.position != body_len {
        return Err(DbError::Corrupt("index run has trailing bytes before checksum".to_string()));
    }
    Ok(entries)
}

#[cfg(test)]
async fn decode_run_pages(mut pages: db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
    let result = decode_run_pages_inner(&pages, expected_kind, control).await;
    let _ = pages.close_step()?;
    drop(pages);
    result
}
//#endregion 🔖️SortedRun

//#region 🔖️RunView
/// @emoji 📏️ One byte range inside a run's retained pages.
#[derive(Clone, Copy, Debug)]
struct RunRange {
    start: usize,
    len: usize,
}

/// @emoji 👁️ One entry of a run read in place: its key and, for a put, its value, as ranges of the
/// run's own pages.
#[derive(Clone, Copy, Debug)]
struct RunViewEntry {
    key: RunRange,
    value: Option<RunRange>,
}

/// @emoji 👁️ One run read in place. Loading, searching and merging a run costs exactly its retained
/// pages — never a page per entry — so a run of `MAX_RUN_ENTRIES` small entries stays inside one
/// operation's I/O credit, and an index never outgrows that credit as its document grows.
struct RunView {
    pages: db_storage::DbIoPages,
    entries: Box<[RunViewEntry]>,
}

/// @emoji 🧹️ Returns every page of a run read to its arena before the owner is dropped, so a read
/// never parks its pages as a lost owner for maintenance to reclaim later.
fn close_run_pages(mut pages: db_storage::DbIoPages) -> Result<(), DbError> {
    while pages.close_step()?.is_some() {}
    drop(pages);
    Ok(())
}

impl RunView {
    fn close(self) -> Result<(), DbError> {
        close_run_pages(self.pages)
    }

    fn key_cmp(&self, index: usize, key: &IndexBytes) -> Result<std::cmp::Ordering, DbError> {
        run_range_cmp(&self.pages, self.entries[index].key, &key.pages, RunRange { start: 0, len: key.len() })
    }

    fn key_starts_with(&self, index: usize, prefix: &IndexBytes) -> Result<bool, DbError> {
        let key = self.entries[index].key;
        if prefix.len() > key.len {
            return Ok(false);
        }
        Ok(run_range_cmp(&self.pages, RunRange { start: key.start, len: prefix.len() }, &prefix.pages, RunRange { start: 0, len: prefix.len() })? == std::cmp::Ordering::Equal)
    }

    /// @emoji 🔢️ Entry `index`'s key as a big-endian `u64` (the seq-keyed kinds' key shape).
    fn key_u64_be(&self, index: usize) -> Result<u64, DbError> {
        let key = self.entries[index].key;
        if key.len != 8 {
            return Err(DbError::Corrupt("index run key is not an 8-byte sequence".to_string()));
        }
        let mut reader = RunPageReader { pages: &self.pages, position: key.start, limit: key.start + key.len };
        Ok(u64::from_be_bytes(reader.array()?))
    }

    /// @emoji 🔢️ Entry `index`'s put value as a little-endian `u64` (the actor-seq kind's value shape).
    fn value_u64_le(&self, index: usize) -> Result<u64, DbError> {
        let value = self.entries[index].value.ok_or_else(|| DbError::Corrupt("index run entry has no value".to_string()))?;
        if value.len != 8 {
            return Err(DbError::Corrupt("index run value is not an 8-byte sequence".to_string()));
        }
        let mut reader = RunPageReader { pages: &self.pages, position: value.start, limit: value.start + value.len };
        Ok(u64::from_le_bytes(reader.array()?))
    }

    /// @emoji 🔎️ The entry holding exactly `key`, by binary search over the run's ascending keys.
    fn find(&self, key: &IndexBytes, control: &mut IndexCursorControl) -> Result<Option<usize>, DbError> {
        let (mut low, mut high) = (0usize, self.entries.len());
        while low < high {
            control.grant()?;
            let middle = low + (high - low) / 2;
            match self.key_cmp(middle, key)? {
                std::cmp::Ordering::Less => low = middle + 1,
                std::cmp::Ordering::Greater => high = middle,
                std::cmp::Ordering::Equal => return Ok(Some(middle)),
            }
        }
        Ok(None)
    }

    /// @emoji 📤️ Copies one of this run's ranges out as caller-owned bytes, charged to the run's own
    /// read operation.
    async fn materialize(&self, range: RunRange, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
        let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(self.pages.operation(), range.len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        let mut reader = RunPageReader { pages: &self.pages, position: range.start, limit: range.start + range.len };
        while reader.position < reader.limit {
            control.grant()?;
            let fragment = reader.fragment()?;
            let written = writer.write_fragment(fragment)?;
            reader.position += written;
        }
        writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_storage::DbIoPageWriterRejected::into_error)
    }
}

/// @emoji ⚖️ Lexicographic order of two ranges, each over its own pages.
fn run_range_cmp(left: &db_storage::DbIoPages, left_range: RunRange, right: &db_storage::DbIoPages, right_range: RunRange) -> Result<std::cmp::Ordering, DbError> {
    let mut left_reader = RunPageReader { pages: left, position: left_range.start, limit: left_range.start + left_range.len };
    let mut right_reader = RunPageReader { pages: right, position: right_range.start, limit: right_range.start + right_range.len };
    loop {
        match (left_reader.position < left_reader.limit, right_reader.position < right_reader.limit) {
            (false, false) => return Ok(std::cmp::Ordering::Equal),
            (false, true) => return Ok(std::cmp::Ordering::Less),
            (true, false) => return Ok(std::cmp::Ordering::Greater),
            (true, true) => {}
        }
        let left_fragment = left_reader.fragment()?;
        let right_fragment = right_reader.fragment()?;
        let count = left_fragment.len().min(right_fragment.len());
        match left_fragment[..count].cmp(&right_fragment[..count]) {
            std::cmp::Ordering::Equal => {
                left_reader.position += count;
                right_reader.position += count;
            }
            order => return Ok(order),
        }
    }
}

/// @emoji 👁️ Verifies one run's checksum and structure and reads its entries in place.
async fn view_run_pages(pages: db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunView, DbError> {
    match view_run_entries(&pages, expected_kind, control).await {
        Ok(entries) => Ok(RunView { pages, entries }),
        Err(error) => {
            close_run_pages(pages)?;
            Err(error)
        }
    }
}

async fn view_run_entries(pages: &db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<Box<[RunViewEntry]>, DbError> {
    control.grant()?;
    if pages.len() < 4 {
        return Err(DbError::Corrupt("index run is shorter than its checksum trailer".to_string()));
    }
    let body_len = pages.len() - 4;
    let mut checksum = pack::codec::Crc32cCursor::new();
    let mut remaining = body_len;
    for fragment in pages.fragments() {
        control.grant()?;
        let count = remaining.min(fragment.len());
        checksum.update_page(&fragment[..count]);
        remaining -= count;
        if remaining == 0 {
            break;
        }
    }
    let mut trailer = RunPageReader::new(pages, pages.len());
    trailer.position = body_len;
    if checksum.finish() != u32::from_le_bytes(trailer.array::<4>()?) {
        return Err(DbError::Corrupt("index run checksum mismatch".to_string()));
    }
    let mut reader = RunPageReader::new(pages, body_len);
    let header = read_run_header(&mut reader, expected_kind, control).await?;
    let mut entries: Vec<RunViewEntry> = Vec::with_capacity(header.entry_count as usize);
    for _ in 0..header.entry_count {
        control.grant()?;
        let key_len = reader.varint()?;
        check_len(key_len, MAX_KEY_LEN, "db_index::key")?;
        let key = reader.skip(key_len as usize)?;
        if let Some(previous) = entries.last() {
            if run_range_cmp(pages, previous.key, pages, key)? != std::cmp::Ordering::Less {
                return Err(DbError::Corrupt("index run entries are not strictly ascending by key".to_string()));
            }
        }
        let value = match reader.byte()? {
            0 => None,
            1 => {
                let value_len = reader.varint()?;
                check_len(value_len, MAX_VALUE_LEN, "db_index::value")?;
                Some(reader.skip(value_len as usize)?)
            }
            other => return Err(DbError::Corrupt(format!("index run entry has unknown value tag {other}"))),
        };
        entries.push(RunViewEntry { key, value });
    }
    if reader.position != body_len {
        return Err(DbError::Corrupt("index run has trailing bytes before checksum".to_string()));
    }
    Ok(entries.into_boxed_slice())
}

/// @emoji 📌️ One output entry of a merge: which input view, which of its entries.
#[derive(Clone, Copy)]
struct RunPick {
    view: usize,
    entry: usize,
}

/// @emoji 🔀️ The ascending merge of two adjacent runs, the newer winning on an equal key and
/// tombstones dropped only when nothing older remains beneath.
fn merge_run_views(older: &RunView, newer: &RunView, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<Vec<RunPick>, DbError> {
    let (mut old_index, mut new_index) = (0usize, 0usize);
    let mut picks = Vec::with_capacity(older.entries.len() + newer.entries.len());
    while old_index < older.entries.len() || new_index < newer.entries.len() {
        control.grant()?;
        let order = if old_index == older.entries.len() {
            std::cmp::Ordering::Greater
        } else if new_index == newer.entries.len() {
            std::cmp::Ordering::Less
        } else {
            run_range_cmp(&older.pages, older.entries[old_index].key, &newer.pages, newer.entries[new_index].key)?
        };
        let pick = match order {
            std::cmp::Ordering::Less => {
                old_index += 1;
                RunPick { view: 0, entry: old_index - 1 }
            }
            std::cmp::Ordering::Greater => {
                new_index += 1;
                RunPick { view: 1, entry: new_index - 1 }
            }
            std::cmp::Ordering::Equal => {
                old_index += 1;
                new_index += 1;
                RunPick { view: 1, entry: new_index - 1 }
            }
        };
        let view = if pick.view == 0 { older } else { newer };
        if !(drop_tombstones && view.entries[pick.entry].value.is_none()) {
            picks.push(pick);
        }
    }
    Ok(picks)
}

async fn run_write_range(writer: &mut db_storage::DbIoPageWriter, checksum: &mut pack::codec::Crc32cCursor, pages: &db_storage::DbIoPages, range: RunRange, control: &mut IndexCursorControl) -> Result<(), DbError> {
    let mut reader = RunPageReader { pages, position: range.start, limit: range.start + range.len };
    while reader.position < reader.limit {
        control.grant()?;
        let fragment = reader.fragment()?;
        run_write(writer, checksum, fragment).await?;
        reader.position += fragment.len();
    }
    Ok(())
}

/// @emoji ✍️ Encodes picked entries of in-place runs as one run, copying key and value bytes straight
/// from their source pages — the same wire as `encode_run_pages`.
async fn encode_run_from_views(kind: IndexKind, views: [&RunView; 2], picks: &[RunPick], control: &mut IndexCursorControl) -> Result<db_storage::DbIoPages, DbError> {
    check_len(picks.len() as u64, MAX_RUN_ENTRIES, "db_index::entries")?;
    let mut encoded_len = RUN_MAGIC.len() + 2 + varint_len(picks.len() as u64) + 4;
    for pick in picks {
        control.grant()?;
        let entry = views[pick.view].entries[pick.entry];
        encoded_len = encoded_len.checked_add(varint_len(entry.key.len as u64) + entry.key.len + 1).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
        if let Some(value) = entry.value {
            encoded_len = encoded_len.checked_add(varint_len(value.len as u64) + value.len).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
        }
    }
    let mut writer = db_storage::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    let mut checksum = pack::codec::Crc32cCursor::new();
    run_write(&mut writer, &mut checksum, &RUN_MAGIC).await?;
    run_write(&mut writer, &mut checksum, &[RUN_VERSION, kind.tag()]).await?;
    let mut varint = [0u8; 10];
    run_write(&mut writer, &mut checksum, encode_varint(picks.len() as u64, &mut varint)).await?;
    for pick in picks {
        let view = views[pick.view];
        let entry = view.entries[pick.entry];
        run_write(&mut writer, &mut checksum, encode_varint(entry.key.len as u64, &mut varint)).await?;
        run_write_range(&mut writer, &mut checksum, &view.pages, entry.key, control).await?;
        match entry.value {
            None => run_write(&mut writer, &mut checksum, &[0]).await?,
            Some(value) => {
                run_write(&mut writer, &mut checksum, &[1]).await?;
                run_write(&mut writer, &mut checksum, encode_varint(value.len as u64, &mut varint)).await?;
                run_write_range(&mut writer, &mut checksum, &view.pages, value, control).await?;
            }
        }
    }
    run_write_trailer(&mut writer, &checksum.finish().to_le_bytes()).await?;
    writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
}
//#endregion 🔖️RunView

//#region 🔖️Merge
async fn close_run_entry(mut entry: RunEntry, control: &mut IndexCursorControl) -> Result<(), DbError> {
    control.grant()?;
    if let RunValue::Put(value) = &mut entry.value {
        let _ = value.close_step()?;
    } else {
        let _ = entry.key.close_step()?;
    }
    drop(entry);
    Ok(())
}

#[cfg(test)]
async fn merge_run_entries(mut older: RunEntries, mut newer: RunEntries, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
    let (mut old_index, mut new_index) = (0, 0);
    let mut output = RunEntries::new();
    while old_index < older.len() || new_index < newer.len() {
        control.grant()?;
        let order = match (older.get(old_index), newer.get(new_index)) {
            (Some(old), Some(new)) => index_bytes_cmp(&old.key, &new.key),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => break,
        };
        let entry = match order {
            std::cmp::Ordering::Less => {
                let entry = older.take(old_index).ok_or_else(|| DbError::Internal("index older merge owner lost".to_string()))?;
                old_index += 1;
                entry
            }
            std::cmp::Ordering::Greater => {
                let entry = newer.take(new_index).ok_or_else(|| DbError::Internal("index newer merge owner lost".to_string()))?;
                new_index += 1;
                entry
            }
            std::cmp::Ordering::Equal => {
                let shadowed = older.take(old_index).ok_or_else(|| DbError::Internal("index shadowed merge owner lost".to_string()))?;
                close_run_entry(shadowed, control).await?;
                old_index += 1;
                let entry = newer.take(new_index).ok_or_else(|| DbError::Internal("index winning merge owner lost".to_string()))?;
                new_index += 1;
                entry
            }
        };
        if drop_tombstones && matches!(entry.value, RunValue::Tombstone) {
            close_run_entry(entry, control).await?;
        } else if let Err(entry) = output.push(entry) {
            close_run_entry(entry, control).await?;
            return Err(DbError::LimitExceeded("index merged fixed entry owner"));
        }
        semio_framework_async::yield_once().await;
    }
    Ok(output)
}
//#endregion 🔖️Merge

//#region 🔖️MergePolicy
/// @emoji ⚖️ When an append (`IndexHandle::put_batch`/`put_sorted_run`) folds runs together. This
/// crate's own choice (the contract fixes the LSM-lite shape, not the trigger threshold): after every
/// append, if a kind has more than `max_runs_before_merge` runs, the oldest adjacent pair among its
/// newest `max_runs_before_merge + 1` runs whose entries fit one run (`MAX_RUN_ENTRIES`) is merged
/// into one — at most ONE merge per append, sized from the run ids alone (see
/// `IndexHandle::merge_one_within_policy`). A run never grows past `MAX_RUN_ENTRIES`, so every run —
/// and every merge of two — stays inside one operation's I/O credit however large its document grows;
/// full runs simply accumulate behind the newest window and are never read again by an append.
#[derive(Clone, Copy, Debug)]
pub struct MergePolicy {
    pub max_runs_before_merge: usize,
}

impl Default for MergePolicy {
    fn default() -> Self {
        Self { max_runs_before_merge: 4 }
    }
}
//#endregion 🔖️MergePolicy

//#region 🔖️Stats
/// @emoji 📊️ A kind's current shape: how many runs it's spread across, how many live entries (each
/// counted once even if shadowed copies exist in older runs — `entry_count` sums each run's raw
/// header count, so a key overwritten `N` times across `N` runs is NOT deduplicated here; `compact`
/// first is the way to get an exact live-key count) and how many bytes on `IndexStorage`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IndexStats {
    pub run_count: usize,
    pub entry_count: u64,
    pub total_bytes: u64,
}
//#endregion 🔖️Stats

/// @emoji 🧭️ What one entry of a newest-first range walk means for the rest of its run.
enum RangeStep {
    Skip,
    Below,
}

//#region 🔖️IndexHandle
/// @emoji 🔍️ One `(document, kind)`'s view onto its sorted runs — every typed wrapper below
/// (`CommandIndex`, `FrontierIndex`, ...) is a thin codec layered on top of one of these. Never
/// interprets key/value bytes itself; that's the typed layer's job.
pub struct IndexHandle<'a, S: IndexStorage> {
    storage: &'a S,
    document: ArtifactId,
    kind: IndexKind,
    policy: MergePolicy,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl<'a, S: IndexStorage> IndexHandle<'a, S> {
    /// @emoji 🚀️ Opens a handle with the default `MergePolicy`.
    pub async fn new(storage: &'a S, document: ArtifactId, kind: IndexKind) -> Self {
        Self::with_policy(storage, document, kind, MergePolicy::default()).await
    }

    /// @emoji 🚀️ Opens a handle with an explicit `MergePolicy` (e.g. a tighter threshold for a
    /// hot, frequently-scanned kind, or a looser one for a write-heavy, rarely-read kind).
    pub async fn with_policy(storage: &'a S, document: ArtifactId, kind: IndexKind, policy: MergePolicy) -> Self {
        Self { storage, document, kind, policy, cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)) }
    }

    pub fn operation_control(&self, fuel: usize) -> Result<IndexCursorControl, DbError> {
        IndexCursorControl::new(self.cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), fuel)
    }

    /// 🧵️ Mounts a parent job's exact cancellation/deadline authority for retained index work.
    pub fn retained_operation_control(&self, cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<IndexCursorControl, DbError> {
        IndexCursorControl::retained(cancelled, deadline, fuel)
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    /// @emoji 📋️ This handle's live run ids, ascending (oldest sequence first) — one storage listing;
    /// every id of another kind or run-id format for the same document is filtered out.
    async fn kind_run_ids(&self, control: &mut IndexCursorControl) -> Result<Vec<u64>, DbError> {
        control.grant()?;
        let mut source = self.storage.list_runs(&self.document).await?;
        let namespace = run_namespace(self.kind);
        let mut output = Vec::new();
        for id in source.as_slice() {
            control.grant()?;
            if namespace_of_run_id(*id) == namespace {
                output.push(*id);
            }
        }
        while source.close_step() {}
        Ok(output)
    }

    async fn view_run(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<RunView, DbError> {
        let pages = self.storage.read_run(&self.document, run_id).await?;
        view_run_pages(pages, self.kind, control).await
    }

    /// @emoji 🔀️ Folds two adjacent runs (`older` directly beneath `newer`) into one run under
    /// `older`'s sequence and deletes both inputs: written before deleted, so a crash in between
    /// leaves copies whose values agree, and the newer copy of every key still wins. The caller only
    /// picks a pair whose entries fit one run.
    async fn merge_adjacent(&self, older_id: u64, newer_id: u64, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let older = self.view_run(older_id, control).await?;
        let newer = match self.view_run(newer_id, control).await {
            Ok(newer) => newer,
            Err(error) => {
                older.close()?;
                return Err(error);
            }
        };
        let encoded = match merge_run_views(&older, &newer, drop_tombstones, control) {
            Ok(picks) if picks.is_empty() => Ok(None),
            Ok(picks) => match make_run_id(self.kind, sequence_of_run_id(older_id), picks.len()) {
                Ok(merged_id) => encode_run_from_views(self.kind, [&older, &newer], &picks, control).await.map(|pages| Some((merged_id, pages))),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        };
        older.close()?;
        newer.close()?;
        if let Some((merged_id, pages)) = encoded? {
            self.storage.write_run(&self.document, merged_id, pages).await?;
            if merged_id != older_id {
                self.storage.delete_run(&self.document, older_id).await?;
            }
        } else {
            self.storage.delete_run(&self.document, older_id).await?;
        }
        self.storage.delete_run(&self.document, newer_id).await
    }

    /// @emoji 🥇️ The greatest live entry whose key starts with `prefix` and is at most `upper`, walking
    /// runs newest-first and keeping only the best candidate plus the tombstoned keys above it —
    /// never every entry of the kind. The newest occurrence of a key decides whether it is live.
    pub async fn last_live_in_range(&self, prefix: &IndexBytes, upper: Option<&IndexBytes>, control: &mut IndexCursorControl) -> Result<Option<(IndexBytes, IndexBytes)>, DbError> {
        const DEAD_KEYS_MAX: usize = MAX_RUN_ENTRIES as usize;
        let ids = self.kind_run_ids(control).await?;
        let mut best: Option<(IndexBytes, IndexBytes)> = None;
        let mut dead: Vec<IndexBytes> = Vec::new();
        let mut failure = None;
        'runs: for position in (0..ids.len()).rev() {
            let view = match self.view_run(ids[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    failure = Some(error);
                    break;
                }
            };
            for index in (0..view.entries.len()).rev() {
                let step = async {
                    control.grant()?;
                    if !view.key_starts_with(index, prefix)? {
                        return Ok::<_, DbError>(if view.key_cmp(index, prefix)? == std::cmp::Ordering::Less { RangeStep::Below } else { RangeStep::Skip });
                    }
                    if let Some(upper) = upper {
                        if view.key_cmp(index, upper)? == std::cmp::Ordering::Greater {
                            return Ok(RangeStep::Skip);
                        }
                    }
                    if let Some((best_key, _)) = best.as_ref() {
                        if view.key_cmp(index, best_key)? != std::cmp::Ordering::Greater {
                            return Ok(RangeStep::Below);
                        }
                    }
                    for dead_key in &dead {
                        if view.key_cmp(index, dead_key)? == std::cmp::Ordering::Equal {
                            return Ok(RangeStep::Skip);
                        }
                    }
                    let entry = view.entries[index];
                    match entry.value {
                        None => {
                            if dead.len() == DEAD_KEYS_MAX {
                                return Err(DbError::LimitExceeded("index range scan tombstone window"));
                            }
                            dead.push(view.materialize(entry.key, control).await?);
                            Ok(RangeStep::Skip)
                        }
                        Some(value) => {
                            let key = view.materialize(entry.key, control).await?;
                            let value = view.materialize(value, control).await?;
                            if let Some((previous_key, previous_value)) = best.replace((key, value)) {
                                close_index_bytes(previous_key, control).await?;
                                close_index_bytes(previous_value, control).await?;
                            }
                            Ok(RangeStep::Below)
                        }
                    }
                }
                .await;
                match step {
                    Ok(RangeStep::Skip) => {}
                    Ok(RangeStep::Below) => break,
                    Err(error) => {
                        failure = Some(error);
                        view.close()?;
                        break 'runs;
                    }
                }
            }
            view.close()?;
        }
        for key in dead {
            close_index_bytes(key, control).await?;
        }
        if let Some(error) = failure {
            if let Some((key, value)) = best {
                close_index_bytes(key, control).await?;
                close_index_bytes(value, control).await?;
            }
            return Err(error);
        }
        Ok(best)
    }

    /// @emoji ✍️ Durably appends `entries` as one new, newest retained run,
    /// then applies `MergePolicy`. A no-op (no run written) if `entries` is empty.
    pub async fn put_batch(&self, mut entries: RunEntries, control: &mut IndexCursorControl) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        for pass in 0..entries.len() {
            for index in 0..entries.len().saturating_sub(pass + 1) {
                entries.sort_step(index, index + 1, control)?;
            }
        }
        let mut unique = RunEntries::new();
        for index in 0..entries.len() {
            let entry = entries.take(index).ok_or_else(|| DbError::Internal("index sorted entry owner lost".to_string()))?;
            if unique.get(unique.len().saturating_sub(1)).is_some_and(|previous| index_bytes_cmp(&previous.key, &entry.key) == std::cmp::Ordering::Equal) {
                close_run_entry(unique.pop().ok_or_else(|| DbError::Internal("index duplicate owner lost".to_string()))?, control).await?;
            }
            if let Err(entry) = unique.push(entry) {
                close_run_entry(entry, control).await?;
                return Err(DbError::LimitExceeded("index unique fixed entry owner"));
            }
        }
        let count = unique.len();
        let pages = encode_run_pages(self.kind, &unique, control).await?;
        control.grant()?;
        let _ = unique.close_step()?;
        drop(unique);
        self.append_run(pages, count, control).await
    }

    pub async fn put(&self, key: IndexBytes, value: IndexBytes, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let mut entries = RunEntries::new();
        entries.push(RunEntry { key, value: RunValue::Put(value) }).map_err(|_| DbError::LimitExceeded("index entry owner"))?;
        self.put_batch(entries, control).await
    }

    pub async fn delete(&self, key: IndexBytes, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let mut entries = RunEntries::new();
        entries.push(RunEntry { key, value: RunValue::Tombstone }).map_err(|_| DbError::LimitExceeded("index entry owner"))?;
        self.put_batch(entries, control).await
    }

    /// @emoji 📥️ Durably appends one run of already strictly ascending, unique `(key, value)` puts —
    /// the bulk path of an owner that generates its own keys (the artifact engine's command, inverse,
    /// actor-seq and frontier entries): the run is encoded straight from the caller's bytes into one
    /// write, with no per-entry byte owner. Errors `InvalidArgument` on unordered or duplicate keys.
    pub async fn put_sorted_run(&self, entries: &[(&[u8], &[u8])], control: &mut IndexCursorControl) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        let pages = encode_sorted_run_pages(self.kind, entries, control).await?;
        self.append_run(pages, entries.len(), control).await
    }

    /// @emoji ➕️ Writes `pages` (holding `count` entries) as the kind's newest run from ONE listing
    /// of its runs, then lets `MergePolicy` fold at most one adjacent pair — sizes come from the run
    /// ids, so an append never reads a run it does not merge.
    async fn append_run(&self, pages: db_storage::DbIoPages, count: usize, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let mut ids = match self.kind_run_ids(control).await {
            Ok(ids) => ids,
            Err(error) => {
                close_run_pages(pages)?;
                return Err(error);
            }
        };
        let next = ids.last().map_or(0, |id| sequence_of_run_id(*id) + 1);
        let run_id = match make_run_id(self.kind, next, count) {
            Ok(run_id) => run_id,
            Err(error) => {
                close_run_pages(pages)?;
                return Err(error);
            }
        };
        self.storage.write_run(&self.document, run_id, pages).await?;
        ids.push(run_id);
        self.merge_one_within_policy(&ids, control).await
    }

    /// @emoji 🔎️ Resolves `key` by searching runs newest-to-oldest in place and returning the first match —
    /// `Ok(None)` if the first match is a tombstone, or if no run has ever held `key`.
    pub async fn get(&self, key: &IndexBytes, control: &mut IndexCursorControl) -> Result<Option<IndexBytes>, DbError> {
        let ids = self.kind_run_ids(control).await?;
        let mut result = Ok(None);
        for position in (0..ids.len()).rev() {
            control.grant()?;
            let view = match self.view_run(ids[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    result = Err(error);
                    break;
                }
            };
            let found = match view.find(key, control) {
                Ok(Some(index)) => match view.entries[index].value {
                    Some(value) => Some(view.materialize(value, control).await.map(Some)),
                    None => Some(Ok(None)),
                },
                Ok(None) => None,
                Err(error) => Some(Err(error)),
            };
            view.close()?;
            if let Some(found) = found {
                result = found;
                break;
            }
        }
        result
    }

    /// @emoji 🔝️ The kind's newest run read in place, or `None` when the kind holds no run — the
    /// one read an append-only owner needs to learn how far its entries reach.
    async fn newest_run(&self, control: &mut IndexCursorControl) -> Result<Option<RunView>, DbError> {
        let ids = self.kind_run_ids(control).await?;
        match ids.last() {
            Some(newest) => self.view_run(*newest, control).await.map(Some),
            None => Ok(None),
        }
    }

    /// @emoji 📜️ Every live (non-tombstoned) `(key, value)` whose key starts with `prefix`, ascending by
    /// key — runs searched in place newest-first, each key decided by its newest occurrence, only the
    /// live matches materialized (at most `MAX_RUN_ENTRIES` of them).
    pub async fn scan_prefix(&self, prefix: &IndexBytes, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
        let run_ids = self.kind_run_ids(control).await?;
        let mut output = RunEntries::new();
        let mut dead: Vec<IndexBytes> = Vec::new();
        let mut failure = None;
        'runs: for position in (0..run_ids.len()).rev() {
            let view = match self.view_run(run_ids[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    failure = Some(error);
                    break;
                }
            };
            for index in 0..view.entries.len() {
                let step = async {
                    control.grant()?;
                    if !view.key_starts_with(index, prefix)? {
                        return Ok::<_, DbError>(());
                    }
                    for seen in (0..output.len()).filter_map(|slot| output.get(slot)).map(|entry| &entry.key).chain(dead.iter()) {
                        if view.key_cmp(index, seen)? == std::cmp::Ordering::Equal {
                            return Ok(());
                        }
                    }
                    let entry = view.entries[index];
                    let key = view.materialize(entry.key, control).await?;
                    match entry.value {
                        None => {
                            if dead.len() == MAX_RUN_ENTRIES as usize {
                                close_index_bytes(key, control).await?;
                                return Err(DbError::LimitExceeded("index scan tombstone window"));
                            }
                            dead.push(key);
                        }
                        Some(value) => {
                            let value = view.materialize(value, control).await?;
                            if let Err(entry) = output.push(RunEntry { key, value: RunValue::Put(value) }) {
                                close_run_entry(entry, control).await?;
                                return Err(DbError::LimitExceeded("index scan result owner"));
                            }
                        }
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = step {
                    failure = Some(error);
                    view.close()?;
                    break 'runs;
                }
            }
            view.close()?;
        }
        for key in dead {
            close_index_bytes(key, control).await?;
        }
        if let Some(error) = failure {
            while output.close_step()? {}
            return Err(error);
        }
        for pass in 0..output.len() {
            for index in 0..output.len().saturating_sub(pass + 1) {
                output.sort_step(index, index + 1, control)?;
            }
        }
        Ok(output)
    }

    /// @emoji 🌀️ `MergePolicy`'s enforcement after one append, bounded to ONE merge: while this kind
    /// has more runs than `policy.max_runs_before_merge`, the oldest adjacent pair among its newest
    /// `max_runs_before_merge + 1` runs whose entries fit one run is merged. Sizes come from the run
    /// ids. Tombstones are dropped only when the pair holds the kind's oldest run, since nothing older
    /// can still need shadowing. Full runs never pair, so an owner that appends full runs never merges.
    async fn merge_one_within_policy(&self, ids: &[u64], control: &mut IndexCursorControl) -> Result<(), DbError> {
        if ids.len() <= self.policy.max_runs_before_merge {
            return Ok(());
        }
        let window = ids.len() - (self.policy.max_runs_before_merge + 1);
        let candidates = &ids[window..];
        let Some(pair) = (0..candidates.len() - 1).find(|pair| entries_of_run_id(candidates[*pair]) + entries_of_run_id(candidates[*pair + 1]) <= MAX_RUN_ENTRIES) else { return Ok(()) };
        control.grant()?;
        self.merge_adjacent(candidates[pair], candidates[pair + 1], window + pair == 0, control).await
    }

    /// @emoji 🧹️ Folds this kind's runs into the fewest runs `MAX_RUN_ENTRIES` allows: from the oldest,
    /// every adjacent pair that fits one run is merged (tombstones dropped wherever the pair holds the
    /// oldest run, and from a lone oldest run). Returns the post-compaction `stats()`.
    pub async fn compact(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let mut position = 0usize;
        loop {
            let run_ids = self.kind_run_ids(control).await?;
            let pair = (position + 1 < run_ids.len()).then(|| (run_ids[position], run_ids[position + 1]));
            let oldest = run_ids.first().copied();
            let Some((older, newer)) = pair else {
                if let Some(oldest) = oldest {
                    self.drop_run_tombstones(oldest, control).await?;
                }
                break;
            };
            if entries_of_run_id(older) + entries_of_run_id(newer) <= MAX_RUN_ENTRIES {
                self.merge_adjacent(older, newer, position == 0, control).await?;
            } else {
                if position == 0 {
                    self.drop_run_tombstones(older, control).await?;
                }
                position += 1;
            }
        }
        self.stats(control).await
    }

    /// @emoji 🧹️ Rewrites the kind's oldest run without its tombstones (nothing older remains for
    /// them to shadow), or deletes it when nothing else is left in it.
    async fn drop_run_tombstones(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let view = self.view_run(run_id, control).await?;
        if view.entries.iter().all(|entry| entry.value.is_some()) {
            return view.close();
        }
        let picks: Vec<RunPick> = (0..view.entries.len()).filter(|index| view.entries[*index].value.is_some()).map(|entry| RunPick { view: 0, entry }).collect();
        let encoded = if picks.is_empty() {
            Ok(None)
        } else {
            match make_run_id(self.kind, sequence_of_run_id(run_id), picks.len()) {
                Ok(rewritten) => encode_run_from_views(self.kind, [&view, &view], &picks, control).await.map(|pages| Some((rewritten, pages))),
                Err(error) => Err(error),
            }
        };
        view.close()?;
        match encoded? {
            Some((rewritten, pages)) => {
                self.storage.write_run(&self.document, rewritten, pages).await?;
                if rewritten != run_id {
                    self.storage.delete_run(&self.document, run_id).await?;
                }
                Ok(())
            }
            None => self.storage.delete_run(&self.document, run_id).await,
        }
    }

    /// @emoji 📊️ Current shape of this kind's runs — see `IndexStats`'s doc for what `entry_count`
    /// does and doesn't count. Run and entry counts come from the listing; bytes from each run.
    pub async fn stats(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let run_ids = self.kind_run_ids(control).await?;
        let mut total_bytes = 0u64;
        for run_id in &run_ids {
            control.grant()?;
            let bytes = self.storage.read_run(&self.document, *run_id).await?;
            total_bytes += bytes.len() as u64;
            close_run_pages(bytes)?;
        }
        Ok(IndexStats { run_count: run_ids.len(), entry_count: run_ids.iter().map(|id| entries_of_run_id(*id)).sum(), total_bytes })
    }

    /// @emoji ✅️ Fully decodes (checksum + structural validation) every live run for this kind,
    /// surfacing the first `DbError::Corrupt` found rather than any value — `db_cli verify`'s hook.
    /// A run whose entry count differs from the one its id declares is corrupt too.
    pub async fn verify(&self, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let ids = self.kind_run_ids(control).await?;
        for id in ids {
            let view = self.view_run(id, control).await?;
            let declared = entries_of_run_id(id);
            let held = view.entries.len() as u64;
            view.close()?;
            if declared != held {
                return Err(DbError::Corrupt(format!("index run {id:#x} declares {declared} entries and holds {held}")));
            }
        }
        Ok(())
    }
}
//#endregion 🔖️IndexHandle

//#region 🔖️RecordLocation
/// @emoji 📍️ A pointer into a document's WAL: which segment, what byte offset, how many bytes.
/// `CommandIndex`/`InverseIndex`'s value shape — deliberately NOT the WAL record itself (this crate
/// never depends on `db_wal`/`protocol`; a location is exactly enough for a caller who DOES depend
/// on those to seek and re-read the actual record).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RecordLocation {
    pub segment: u64,
    pub offset: u64,
    pub len: u64,
}

async fn encode_location(location: RecordLocation) -> Vec<u8> {
    let mut writer = ByteWriter::new();
    writer.write_varint_u64(location.segment);
    writer.write_varint_u64(location.offset);
    writer.write_varint_u64(location.len);
    writer.into_bytes()
}

fn decode_location(reader: &mut RunPageReader<'_>) -> Result<RecordLocation, DbError> {
    let segment = reader.varint()?;
    let offset = reader.varint()?;
    let len = reader.varint()?;
    Ok(RecordLocation { segment, offset, len })
}

async fn decode_index_bytes<T>(mut bytes: IndexBytes, control: &mut IndexCursorControl, decode: impl FnOnce(&mut RunPageReader<'_>) -> Result<T, DbError>) -> Result<T, DbError> {
    control.grant()?;
    let result = (|| {
        let mut reader = RunPageReader::new(&bytes.pages, bytes.len());
        let decoded = decode(&mut reader)?;
        if reader.position != reader.limit {
            return Err(DbError::Corrupt("typed index value has trailing bytes".to_string()));
        }
        Ok(decoded)
    })();
    control.grant()?;
    let _ = bytes.close_step()?;
    drop(bytes);
    result
}

/// @emoji 🔢️ `u64 -> RecordLocation`, keyed big-endian so byte order matches numeric order — the
/// shared shape behind both `CommandIndex` (keyed by command seq) and `InverseIndex` (keyed by the
/// same command seq, pointing at its inverse's location instead).
struct SeqLocationIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> SeqLocationIndex<'a, S> {
    async fn new(storage: &'a S, document: ArtifactId, kind: IndexKind) -> Self {
        Self { handle: IndexHandle::new(storage, document, kind).await }
    }

    async fn record(&self, seq: u64, location: RecordLocation) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let value = admit_generated_index_bytes(encode_location(location).await, MAX_VALUE_LEN, &mut control).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), &seq.to_be_bytes(), &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    async fn lookup(&self, seq: u64) -> Result<Option<RecordLocation>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(seq.to_be_bytes().to_vec(), MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        match result {
            Some(bytes) => Ok(Some(decode_index_bytes(bytes, &mut control, decode_location).await?)),
            None => Ok(None),
        }
    }

    async fn remove(&self, seq: u64) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(seq.to_be_bytes().to_vec(), MAX_KEY_LEN, &mut control).await?;
        self.handle.delete(key, &mut control).await
    }

    async fn record_run(&self, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(entries.len());
        for (seq, location) in entries {
            encoded.push((seq.to_be_bytes(), encode_location(*location).await));
        }
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.put_sorted_run(&slices, &mut control).await
    }

    async fn indexed_through(&self) -> Result<u64, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let Some(view) = self.handle.newest_run(&mut control).await? else { return Ok(0) };
        let highest = view.entries.len().checked_sub(1).map_or(Ok(0), |last| view.key_u64_be(last));
        view.close()?;
        highest
    }
}
//#endregion 🔖️RecordLocation

//#region 🔖️CommandIndex
/// @emoji 🗃️ `command_seq -> RecordLocation` — `db_artifact`'s primary lookup for "where in the
/// WAL is command N", the backbone of replay-from-a-point and `Consistency::Exact`/`AtLeast` query
/// resolution.
pub struct CommandIndex<'a, S: IndexStorage>(SeqLocationIndex<'a, S>);

impl<'a, S: IndexStorage> CommandIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self(SeqLocationIndex::new(storage, document, IndexKind::Command).await)
    }

    pub async fn record(&self, command_seq: u64, location: RecordLocation) -> Result<(), DbError> {
        self.0.record(command_seq, location).await
    }

    /// @emoji 📥️ Records ascending `(command_seq, location)` pairs as ONE run (at most `MAX_RUN_ENTRIES`).
    pub async fn record_run(&self, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {
        self.0.record_run(entries).await
    }

    /// @emoji 🔝️ The highest command seq recorded, `0` when none — read from the newest run alone,
    /// which holds it for an owner that records in ascending order.
    pub async fn indexed_through(&self) -> Result<u64, DbError> {
        self.0.indexed_through().await
    }

    pub async fn lookup(&self, command_seq: u64) -> Result<Option<RecordLocation>, DbError> {
        self.0.lookup(command_seq).await
    }

    pub async fn remove(&self, command_seq: u64) -> Result<(), DbError> {
        self.0.remove(command_seq).await
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.0.handle.operation_control(8_192)?;
        self.0.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.0.handle.operation_control(65_536)?;
        self.0.handle.compact(&mut control).await
    }
}
//#endregion 🔖️CommandIndex

//#region 🔖️InverseIndex
/// @emoji ↩️ `command_seq -> RecordLocation` of that command's inverse operation payload —
/// `db_artifact`'s undo machinery's lookup.
pub struct InverseIndex<'a, S: IndexStorage>(SeqLocationIndex<'a, S>);

impl<'a, S: IndexStorage> InverseIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self(SeqLocationIndex::new(storage, document, IndexKind::Inverse).await)
    }

    pub async fn record(&self, command_seq: u64, location: RecordLocation) -> Result<(), DbError> {
        self.0.record(command_seq, location).await
    }

    /// @emoji 📥️ Records ascending `(command_seq, location)` pairs as ONE run (at most `MAX_RUN_ENTRIES`).
    pub async fn record_run(&self, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {
        self.0.record_run(entries).await
    }

    /// @emoji 🔝️ The highest command seq recorded, `0` when none — read from the newest run alone,
    /// which holds it for an owner that records in ascending order.
    pub async fn indexed_through(&self) -> Result<u64, DbError> {
        self.0.indexed_through().await
    }

    pub async fn lookup(&self, command_seq: u64) -> Result<Option<RecordLocation>, DbError> {
        self.0.lookup(command_seq).await
    }

    pub async fn remove(&self, command_seq: u64) -> Result<(), DbError> {
        self.0.remove(command_seq).await
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.0.handle.operation_control(8_192)?;
        self.0.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.0.handle.operation_control(65_536)?;
        self.0.handle.compact(&mut control).await
    }
}
//#endregion 🔖️InverseIndex

//#region 🔖️ActorSeqIndex
/// @emoji 👤️ `(actor, actor_seq) -> command_seq` — resolves an actor's own local operation sequence
/// number (idempotency / causal-order checks at admission) to the document's global command
/// sequence. Keys are `actor_bytes || 0x00 || actor_seq(8, BE)`; `actor`'s id must not itself
/// contain a NUL byte (validated) so the `0x00` separator stays unambiguous and prefix scans by
/// actor (`latest_for_actor`) can't spill into a neighboring actor's entries.
pub struct ActorSeqIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn validate_actor_key_safe(actor: &ActorId) -> Result<(), DbError> {
    if actor.0.as_bytes().contains(&0u8) {
        return Err(DbError::InvalidArgument("actor id must not contain a NUL byte to be index-key safe".to_string()));
    }
    Ok(())
}

async fn actor_seq_key(actor: &ActorId, actor_seq: u64) -> Result<Vec<u8>, DbError> {
    validate_actor_key_safe(actor).await?;
    let mut key = Vec::with_capacity(actor.0.len() + 1 + 8);
    key.extend_from_slice(actor.0.as_bytes());
    key.push(0u8);
    key.extend_from_slice(&actor_seq.to_be_bytes());
    Ok(key)
}

// 🚫️async: E1 pure accessor consumed by sync Option::map/closures — see R9
fn decode_u64_le(reader: &mut RunPageReader<'_>) -> Result<u64, DbError> {
    Ok(u64::from_le_bytes(reader.array()?))
}

impl<'a, S: IndexStorage> ActorSeqIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::ActorSeq).await }
    }

    pub async fn record(&self, actor: &ActorId, actor_seq: u64, command_seq: u64) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let value = admit_generated_index_bytes(command_seq.to_le_bytes().to_vec(), MAX_VALUE_LEN, &mut control).await?;
        let key_bytes = actor_seq_key(actor, actor_seq).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), &key_bytes, &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub async fn lookup(&self, actor: &ActorId, actor_seq: u64) -> Result<Option<u64>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(actor_seq_key(actor, actor_seq).await?, MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        match result {
            Some(bytes) => Ok(Some(decode_index_bytes(bytes, &mut control, decode_u64_le).await?)),
            None => Ok(None),
        }
    }

    /// @emoji 📥️ Records `(actor, actor_seq, command_seq)` triples as ONE run (at most
    /// `MAX_RUN_ENTRIES`), keyed and ordered as `record` keys them.
    pub async fn record_run(&self, entries: &[(ActorId, u64, u64)]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(entries.len());
        for (actor, actor_seq, command_seq) in entries {
            encoded.push((actor_seq_key(actor, *actor_seq).await?, command_seq.to_le_bytes()));
        }
        encoded.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.put_sorted_run(&slices, &mut control).await
    }

    /// @emoji 🔝️ The highest command seq any entry of the newest run points at, `0` when none —
    /// for an owner that records runs in ascending command order.
    pub async fn indexed_through(&self) -> Result<u64, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let Some(view) = self.handle.newest_run(&mut control).await? else { return Ok(0) };
        let mut highest = Ok(0u64);
        for index in 0..view.entries.len() {
            match view.value_u64_le(index) {
                Ok(command_seq) => highest = highest.map(|highest| highest.max(command_seq)),
                Err(error) => {
                    highest = Err(error);
                    break;
                }
            }
        }
        view.close()?;
        highest
    }

    /// @emoji 🥇️ The highest `(actor_seq, command_seq)` pair recorded for `actor`, or `None` if
    /// `actor` has never been recorded.
    pub async fn latest_for_actor(&self, actor: &ActorId) -> Result<Option<(u64, u64)>, DbError> {
        validate_actor_key_safe(actor).await?;
        let mut prefix = actor.0.as_bytes().to_vec();
        prefix.push(0u8);
        let mut control = self.handle.operation_control(16_384)?;
        let prefix_owner = admit_generated_index_bytes(prefix, MAX_KEY_LEN, &mut control).await?;
        let latest = self.handle.last_live_in_range(&prefix_owner, None, &mut control).await;
        close_index_bytes(prefix_owner, &mut control).await?;
        let Some((key, value)) = latest? else { return Ok(None) };
        let mut suffix = [0u8; 8];
        let suffix_read = key.read_fragment(key.len().saturating_sub(8), &mut suffix);
        close_index_bytes(key, &mut control).await?;
        if suffix_read != 8 {
            close_index_bytes(value, &mut control).await?;
            return Err(DbError::Corrupt("actor-seq index key has a malformed suffix".to_string()));
        }
        let command = decode_index_bytes(value, &mut control, decode_u64_le).await?;
        Ok(Some((u64::from_be_bytes(suffix), command)))
    }
}
//#endregion 🔖️ActorSeqIndex

//#region 🔖️FrontierIndex
/// @emoji 🧭️ `commit_seq -> Frontier` — a per-commit snapshot of `Frontier`, letting
/// `Consistency::Historical`/replica resume resolve "what did the frontier look like at commit N"
/// without replaying.
pub struct FrontierIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn encode_frontier(frontier: &Frontier) -> Vec<u8> {
    let mut writer = ByteWriter::new();
    let document_bytes = frontier.document.0.as_bytes();
    writer.write_varint_u64(document_bytes.len() as u64);
    writer.write_bytes(document_bytes);
    writer.write_varint_u64(frontier.head_seq);
    writer.write_varint_u64(frontier.commit_seq);
    writer.write_bytes(&frontier.chain_hash);
    writer.write_varint_u64(frontier.epoch);
    writer.into_bytes()
}

fn decode_frontier(reader: &mut RunPageReader<'_>) -> Result<Frontier, DbError> {
    let document_len = reader.varint()?;
    check_len(document_len, MAX_KEY_LEN, "db_index::frontier_document")?;
    let mut document_bytes = vec![0u8; document_len as usize];
    let mut written = 0usize;
    while written < document_bytes.len() {
        let fragment = reader.fragment()?;
        let count = (document_bytes.len() - written).min(fragment.len());
        document_bytes[written..written + count].copy_from_slice(&fragment[..count]);
        reader.position += count;
        written += count;
    }
    let document = ArtifactId(String::from_utf8(document_bytes).map_err(|_| DbError::Corrupt("frontier document id is not valid utf-8".to_string()))?);
    let head_seq = reader.varint()?;
    let commit_seq = reader.varint()?;
    let chain_hash = reader.array()?;
    let epoch = reader.varint()?;
    Ok(Frontier { document, head_seq, commit_seq, chain_hash, epoch })
}

impl<'a, S: IndexStorage> FrontierIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Frontier).await }
    }

    pub async fn record(&self, frontier: &Frontier) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let value = admit_generated_index_bytes(encode_frontier(frontier).await, MAX_VALUE_LEN, &mut control).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), &frontier.commit_seq.to_be_bytes(), &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub async fn lookup(&self, commit_seq: u64) -> Result<Option<Frontier>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(commit_seq.to_be_bytes().to_vec(), MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        match result {
            Some(bytes) => Ok(Some(decode_index_bytes(bytes, &mut control, decode_frontier).await?)),
            None => Ok(None),
        }
    }

    /// @emoji 📥️ Records ascending frontiers (by `commit_seq`) as ONE run (at most `MAX_RUN_ENTRIES`).
    pub async fn record_run(&self, frontiers: &[Frontier]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(frontiers.len());
        for frontier in frontiers {
            encoded.push((frontier.commit_seq.to_be_bytes(), encode_frontier(frontier).await));
        }
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.put_sorted_run(&slices, &mut control).await
    }

    /// @emoji 🔝️ The highest `commit_seq` recorded, `0` when none — from the newest run alone.
    pub async fn indexed_through(&self) -> Result<u64, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let Some(view) = self.handle.newest_run(&mut control).await? else { return Ok(0) };
        let highest = view.entries.len().checked_sub(1).map_or(Ok(0), |last| view.key_u64_be(last));
        view.close()?;
        highest
    }

    /// @emoji 🥇️ The frontier recorded under the highest `commit_seq`, or `None` if none recorded.
    pub async fn latest(&self) -> Result<Option<Frontier>, DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        let prefix = admit_generated_index_bytes(Vec::new(), MAX_KEY_LEN, &mut control).await?;
        let latest = self.handle.last_live_in_range(&prefix, None, &mut control).await;
        close_index_bytes(prefix, &mut control).await?;
        let Some((key, value)) = latest? else { return Ok(None) };
        close_index_bytes(key, &mut control).await?;
        Ok(Some(decode_index_bytes(value, &mut control, decode_frontier).await?))
    }
}
//#endregion 🔖️FrontierIndex

//#region 🔖️TouchedRegionIndex
/// @emoji 🎯️ `region -> [command_seq]` (ascending, deduplicated) — `db_conflict`'s reverse index:
/// given a region a new command is about to touch, which prior commands also touched it (the
/// candidate set for touched-region-intersection conflict checks).
pub struct TouchedRegionIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn encode_postings(postings: &db_storage::DbIoU64List) -> Vec<u8> {
    let mut writer = ByteWriter::new();
    writer.write_varint_u64(postings.len() as u64);
    for posting in postings.as_slice() {
        writer.write_varint_u64(*posting);
    }
    writer.into_bytes()
}

fn decode_postings(reader: &mut RunPageReader<'_>) -> Result<db_storage::DbIoU64List, DbError> {
    let count = reader.varint()?;
    check_len(count, MAX_RUN_ENTRIES, "db_index::postings")?;
    let mut postings = db_storage::DbIoU64List::new();
    for _ in 0..count {
        postings.push(reader.varint()?)?;
    }
    Ok(postings)
}

impl<'a, S: IndexStorage> TouchedRegionIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::TouchedRegion).await }
    }

    /// @emoji ➕️ Records that `command_seq` touched `region` — read-modify-write over the region's
    /// current posting list, kept sorted and deduplicated.
    pub async fn record_touch(&self, region: &[u8], command_seq: u64) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        let mut postings = self.touching_with_control(region, &mut control).await?;
        let mut updated = db_storage::DbIoU64List::new();
        let mut inserted = false;
        for posting in postings.as_slice() {
            if !inserted && command_seq < *posting {
                updated.push(command_seq)?;
                inserted = true;
            }
            if *posting == command_seq {
                inserted = true;
            }
            updated.push(*posting)?;
        }
        if !inserted {
            updated.push(command_seq)?;
        }
        control.grant()?;
        let _ = postings.close_step();
        drop(postings);
        let value = admit_generated_index_bytes(encode_postings(&updated).await, MAX_VALUE_LEN, &mut control).await?;
        control.grant()?;
        let _ = updated.close_step();
        drop(updated);
        let key = IndexBytes::copy_for_operation(value.operation(), region, &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    async fn touching_with_control(&self, region: &[u8], control: &mut IndexCursorControl) -> Result<db_storage::DbIoU64List, DbError> {
        let key = admit_generated_index_bytes(region.to_vec(), MAX_KEY_LEN, control).await?;
        let result = self.handle.get(&key, control).await?;
        close_index_bytes(key, control).await?;
        match result {
            Some(bytes) => decode_index_bytes(bytes, control, decode_postings).await,
            None => Ok(db_storage::DbIoU64List::new()),
        }
    }

    pub async fn touching(&self, region: &[u8]) -> Result<db_storage::DbIoU64List, DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        self.touching_with_control(region, &mut control).await
    }
}
//#endregion 🔖️TouchedRegionIndex

//#region 🔖️CommitIndex
/// @emoji 🏁️ `commit_id -> command_seq` — resolves a VCS-facing commit id (`vcs::Checkpoint.id`,
/// per the contract's content-addressed `ck-<hex16>` scheme) to the command sequence it was cut at,
/// for `Consistency::Historical(commit_id)` query resolution.
pub struct CommitIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> CommitIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Commit).await }
    }

    pub async fn record(&self, commit_id: &str, command_seq: u64) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let value = admit_generated_index_bytes(command_seq.to_le_bytes().to_vec(), MAX_VALUE_LEN, &mut control).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), commit_id.as_bytes(), &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub async fn lookup(&self, commit_id: &str) -> Result<Option<u64>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(commit_id.as_bytes().to_vec(), MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        match result {
            Some(bytes) => Ok(Some(decode_index_bytes(bytes, &mut control, decode_u64_le).await?)),
            None => Ok(None),
        }
    }
}
//#endregion 🔖️CommitIndex

//#region 🔖️FullTextIndex
/// @emoji 🔤️ `term -> [doc_ref]` — a minimal inverted index: `index_document` tokenizes text into
/// lowercase alphanumeric-run terms and records `doc_ref` (an opaque caller-chosen id, typically a
/// field/command location) against each; `search` resolves one term to its posting list. No
/// ranking/stemming/stopwords — `db_query`'s full-text query planner is expected to layer that on
/// top of this crate's exact-term postings.
pub struct FullTextIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> FullTextIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::FullText).await }
    }

    async fn postings(&self, term_key: &[u8], control: &mut IndexCursorControl) -> Result<db_storage::DbIoU64List, DbError> {
        let key = admit_generated_index_bytes(term_key.to_vec(), MAX_KEY_LEN, control).await?;
        let result = self.handle.get(&key, control).await?;
        close_index_bytes(key, control).await?;
        match result {
            Some(bytes) => decode_index_bytes(bytes, control, decode_postings).await,
            None => Ok(db_storage::DbIoU64List::new()),
        }
    }

    /// @emoji ➕️ Tokenizes `text` and records `doc_ref` against every distinct term it contains.
    pub async fn index_document(&self, doc_ref: u64, text: &str) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(65_536)?;
        for term in text.split(|character: char| !character.is_alphanumeric()).filter(|term| !term.is_empty()) {
            control.grant()?;
            let term = term.to_lowercase();
            let mut postings = self.postings(term.as_bytes(), &mut control).await?;
            if postings.as_slice().binary_search(&doc_ref).is_err() {
                let mut updated = db_storage::DbIoU64List::new();
                let mut inserted = false;
                for posting in postings.as_slice() {
                    if !inserted && doc_ref < *posting {
                        updated.push(doc_ref)?;
                        inserted = true;
                    }
                    updated.push(*posting)?;
                }
                if !inserted {
                    updated.push(doc_ref)?;
                }
                control.grant()?;
                let _ = postings.close_step();
                drop(postings);
                postings = updated;
            }
            let value = admit_generated_index_bytes(encode_postings(&postings).await, MAX_VALUE_LEN, &mut control).await?;
            control.grant()?;
            let _ = postings.close_step();
            drop(postings);
            let key = IndexBytes::copy_for_operation(value.operation(), term.as_bytes(), &mut control).await?;
            self.handle.put(key, value, &mut control).await?;
        }
        Ok(())
    }

    /// @emoji 🔎️ The posting list for `term` (case-folded to match `index_document`'s tokenizer),
    /// or an empty list if the term has never been indexed.
    pub async fn search(&self, term: &str) -> Result<db_storage::DbIoU64List, DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        self.postings(term.to_lowercase().as_bytes(), &mut control).await
    }
}
//#endregion 🔖️FullTextIndex

//#region 🔖️BlobList
/// @emoji 📦️ Encodes a list of opaque byte blobs (`ConflictIndex`'s per-command conflict records)
/// as `count(varint) [len(varint) bytes]...` — the same read-modify-write accumulation shape
/// `TouchedRegionIndex`/`FullTextIndex` use for their posting lists, generalized to arbitrary-size
/// values instead of `u64` postings.
pub struct IndexBlobList {
    blobs: [Option<IndexBytes>; MAX_RUN_ENTRIES as usize],
    len: u8,
}

impl IndexBlobList {
    pub fn new() -> Self {
        Self { blobs: std::array::from_fn(|_| None), len: 0 }
    }

    pub fn push(&mut self, bytes: IndexBytes) -> Result<(), IndexBytes> {
        let Some(slot) = self.blobs.get_mut(self.len as usize) else { return Err(bytes) };
        *slot = Some(bytes);
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn get(&self, index: usize) -> Option<&IndexBytes> {
        self.blobs.get(index)?.as_ref()
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len as usize - 1;
        let blob = self.blobs[index].as_mut().ok_or_else(|| DbError::Internal("index blob close lost retained owner".to_string()))?;
        if blob.close_step()?.is_some() {
            return Ok(true);
        }
        self.blobs[index] = None;
        self.len -= 1;
        Ok(true)
    }
}

async fn encode_blob_list(blobs: &IndexBlobList, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    let mut total = varint_len(blobs.len() as u64);
    for index in 0..blobs.len() {
        let blob = blobs.get(index).unwrap();
        total = total.checked_add(varint_len(blob.len() as u64) + blob.len()).ok_or(DbError::LimitExceeded("index blob list bytes"))?;
    }
    let mut writer = db_storage::DbIoPageWriter::try_reserve(total.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    let mut varint = [0u8; 10];
    index_write_unchecked(encode_varint(blobs.len() as u64, &mut varint), &mut writer, control).await?;
    for index in 0..blobs.len() {
        let blob = blobs.get(index).unwrap();
        index_write_unchecked(encode_varint(blob.len() as u64, &mut varint), &mut writer, control).await?;
        for fragment in blob.fragments() {
            index_write_unchecked(fragment, &mut writer, control).await?;
        }
    }
    writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_storage::DbIoPageWriterRejected::into_error)
}

async fn index_write_unchecked(bytes: &[u8], writer: &mut db_storage::DbIoPageWriter, control: &mut IndexCursorControl) -> Result<(), DbError> {
    let mut offset = 0;
    while offset < bytes.len() {
        control.grant()?;
        offset += writer.write_fragment(&bytes[offset..])?;
    }
    Ok(())
}

async fn decode_blob_list_inner(bytes: &IndexBytes, control: &mut IndexCursorControl) -> Result<IndexBlobList, DbError> {
    let operation = bytes.operation();
    let mut reader = RunPageReader::new(&bytes.pages, bytes.len());
    let count = reader.varint()?;
    check_len(count, MAX_RUN_ENTRIES, "db_index::blob_list")?;
    let mut blobs = IndexBlobList::new();
    for _ in 0..count {
        control.grant()?;
        let len = reader.varint()?;
        check_len(len, MAX_VALUE_LEN, "db_index::blob_list_entry")?;
        let blob = index_bytes_from_reader(operation, &mut reader, len as usize, control).await?;
        blobs.push(blob).map_err(|_| DbError::LimitExceeded("index blob list owner"))?;
    }
    if reader.position != reader.limit {
        return Err(DbError::Corrupt("index blob list has trailing bytes".to_string()));
    }
    Ok(blobs)
}

async fn decode_blob_list(bytes: IndexBytes, control: &mut IndexCursorControl) -> Result<IndexBlobList, DbError> {
    let result = decode_blob_list_inner(&bytes, control).await;
    close_index_bytes(bytes, control).await?;
    result
}
//#endregion 🔖️BlobList

//#region 🔖️ConflictIndex
/// @emoji ⚔️ `command_seq -> [ConflictRecord bytes]` — a command may surface more than one
/// conflict (touched-region collision, constraint violation, …), so this accumulates a list per
/// `command_seq` the same way `TouchedRegionIndex` accumulates a posting list: read the current
/// list, append, write back. Record shapes are `db_conflict`'s concern; this index only stores and
/// returns the opaque bytes it's handed.
pub struct ConflictIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> ConflictIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Conflict).await }
    }

    /// @emoji ➕️ Appends `record` to `command_seq`'s conflict list.
    pub async fn record_conflict(&self, command_seq: u64, record: IndexBytes) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(32_768)?;
        let mut records = self.conflicts_for_with_control(command_seq, &mut control).await?;
        records.push(record).map_err(|_| DbError::LimitExceeded("index conflict list owner"))?;
        let value = encode_blob_list(&records, &mut control).await?;
        control.grant()?;
        let _ = records.close_step()?;
        drop(records);
        let key = IndexBytes::copy_for_operation(value.operation(), &command_seq.to_be_bytes(), &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    /// @emoji 📋️ Every conflict record recorded for `command_seq`, in the order they were
    /// recorded, or empty if none.
    async fn conflicts_for_with_control(&self, command_seq: u64, control: &mut IndexCursorControl) -> Result<IndexBlobList, DbError> {
        let key = admit_generated_index_bytes(command_seq.to_be_bytes().to_vec(), MAX_KEY_LEN, control).await?;
        let result = self.handle.get(&key, control).await?;
        close_index_bytes(key, control).await?;
        match result {
            Some(bytes) => decode_blob_list(bytes, control).await,
            None => Ok(IndexBlobList::new()),
        }
    }

    pub async fn conflicts_for(&self, command_seq: u64) -> Result<IndexBlobList, DbError> {
        let mut control = self.handle.operation_control(32_768)?;
        self.conflicts_for_with_control(command_seq, &mut control).await
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.compact(&mut control).await
    }
}
//#endregion 🔖️ConflictIndex

//#region 🔖️ProjectionIndex
/// @emoji 📽️ `(projection_id, frontier_seq) -> opaque projection state bytes`, floor-queryable per
/// projection id — `db_projection`'s "this projection's state as of at or before frontier X"
/// lookup. Keys are `projection_id_bytes || 0x00 || frontier_seq(8, BE)`, the same NUL-separated
/// composite shape `ActorSeqIndex` uses (`projection_id` must not itself contain a NUL byte,
/// validated) so a prefix scan by projection id can't spill into a lexicographically-neighboring
/// projection's entries.
pub struct ProjectionIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn validate_projection_id_key_safe(projection_id: &str) -> Result<(), DbError> {
    if projection_id.as_bytes().contains(&0u8) {
        return Err(DbError::InvalidArgument("projection id must not contain a NUL byte to be index-key safe".to_string()));
    }
    Ok(())
}

async fn projection_key(projection_id: &str, frontier_seq: u64) -> Result<Vec<u8>, DbError> {
    validate_projection_id_key_safe(projection_id).await?;
    let mut key = Vec::with_capacity(projection_id.len() + 1 + 8);
    key.extend_from_slice(projection_id.as_bytes());
    key.push(0u8);
    key.extend_from_slice(&frontier_seq.to_be_bytes());
    Ok(key)
}

impl<'a, S: IndexStorage> ProjectionIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Projection).await }
    }

    pub async fn record(&self, projection_id: &str, frontier_seq: u64, state: IndexBytes) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key_bytes = projection_key(projection_id, frontier_seq).await?;
        let key = IndexBytes::copy_for_operation(state.operation(), &key_bytes, &mut control).await?;
        self.handle.put(key, state, &mut control).await
    }

    /// @emoji 🎯️ The exact state recorded for `projection_id` at `frontier_seq`, or `None` if
    /// nothing was recorded at that exact sequence.
    pub async fn at(&self, projection_id: &str, frontier_seq: u64) -> Result<Option<IndexBytes>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(projection_key(projection_id, frontier_seq).await?, MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        Ok(result)
    }

    /// @emoji 🏔️ The state recorded at the greatest `frontier_seq' <= frontier_seq` for
    /// `projection_id` specifically — scoped to `projection_id`'s own key range (via the NUL
    /// separator) before scanning, so a projection with no entry at or before `frontier_seq` never
    /// wrongly surfaces a different, lexicographically-earlier projection's entry.
    pub async fn latest_at_or_before(&self, projection_id: &str, frontier_seq: u64) -> Result<Option<(u64, IndexBytes)>, DbError> {
        validate_projection_id_key_safe(projection_id).await?;
        let mut prefix = projection_id.as_bytes().to_vec();
        prefix.push(0u8);
        let mut upper = prefix.clone();
        upper.extend_from_slice(&frontier_seq.to_be_bytes());
        let mut control = self.handle.operation_control(32_768)?;
        let prefix_owner = admit_generated_index_bytes(prefix, MAX_KEY_LEN, &mut control).await?;
        let upper_owner = admit_generated_index_bytes(upper, MAX_KEY_LEN, &mut control).await?;
        let latest = self.handle.last_live_in_range(&prefix_owner, Some(&upper_owner), &mut control).await;
        close_index_bytes(prefix_owner, &mut control).await?;
        close_index_bytes(upper_owner, &mut control).await?;
        let Some((key, value)) = latest? else { return Ok(None) };
        let mut seq_bytes = [0u8; 8];
        let suffix_read = key.read_fragment(key.len().saturating_sub(8), &mut seq_bytes);
        close_index_bytes(key, &mut control).await?;
        if suffix_read != 8 {
            close_index_bytes(value, &mut control).await?;
            return Err(DbError::Corrupt("projection index key has a malformed suffix".to_string()));
        }
        Ok(Some((u64::from_be_bytes(seq_bytes), value)))
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.compact(&mut control).await
    }
}
//#endregion 🔖️ProjectionIndex

//#region 🔖️PreviewIndex
/// @emoji 🌫️ `(actor, preview_key) -> opaque latest preview bytes` — `publish`/`withdraw` are
/// plain `put`/`delete`, so `latest` naturally coalesces to the most recently published-or-
/// withdrawn value per `(actor, preview_key)`, matching the contract's "coalescing
/// latest-per-(actor,key)" preview law. Keys are `actor_bytes || 0x00 || preview_key_bytes`
/// (`actor`'s id must not contain a NUL byte, validated; `preview_key` is the final component so
/// it needs no such restriction). Never durable per that same law — `db_preview` is responsible for
/// never routing this index's writes through a durable `DurabilityClass`.
pub struct PreviewIndex<'a, S: IndexStorage> {
    handle: IndexHandle<'a, S>,
}

async fn encode_preview_key(actor: &ActorId, preview_key: &str) -> Result<Vec<u8>, DbError> {
    validate_actor_key_safe(actor).await?;
    let mut key = Vec::with_capacity(actor.0.len() + 1 + preview_key.len());
    key.extend_from_slice(actor.0.as_bytes());
    key.push(0u8);
    key.extend_from_slice(preview_key.as_bytes());
    Ok(key)
}

impl<'a, S: IndexStorage> PreviewIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Preview).await }
    }

    pub async fn publish(&self, actor: &ActorId, preview_key: &str, value: IndexBytes) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key_bytes = encode_preview_key(actor, preview_key).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), &key_bytes, &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub async fn withdraw(&self, actor: &ActorId, preview_key: &str) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(encode_preview_key(actor, preview_key).await?, MAX_KEY_LEN, &mut control).await?;
        self.handle.delete(key, &mut control).await
    }

    pub async fn latest(&self, actor: &ActorId, preview_key: &str) -> Result<Option<IndexBytes>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(encode_preview_key(actor, preview_key).await?, MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        Ok(result)
    }

    /// @emoji 📋️ Every currently-live `(preview_key, value)` published by `actor`.
    pub async fn for_actor(&self, actor: &ActorId) -> Result<RunEntries, DbError> {
        validate_actor_key_safe(actor).await?;
        let mut prefix = actor.0.as_bytes().to_vec();
        prefix.push(0u8);
        let mut control = self.handle.operation_control(16_384)?;
        let prefix = admit_generated_index_bytes(prefix, MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.scan_prefix(&prefix, &mut control).await?;
        close_index_bytes(prefix, &mut control).await?;
        Ok(result)
    }

    pub async fn stats(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.stats(&mut control).await
    }

    pub async fn compact(&self) -> Result<IndexStats, DbError> {
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.compact(&mut control).await
    }
}
//#endregion 🔖️PreviewIndex

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🧪️RetainedTests
#[cfg(test)]
#[path = "🧪️tests/🔬️retained/🦀️.rs"]
mod retained_tests;
//#endregion 🧪️RetainedTests
