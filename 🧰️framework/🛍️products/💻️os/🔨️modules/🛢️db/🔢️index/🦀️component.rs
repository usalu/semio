//! 🗄️ `db_index` — the `db` family's secondary-index engine: immutable sorted runs merged
//! LSM-lite (append a new sorted+checksummed run per write batch, fold old runs together as they
//! accumulate) underneath typed per-kind index builders for all ten kinds (command, actor-seq,
//! frontier, touched-region, inverse, commit, conflict, projection, full-text, preview — see
//! `IndexKind`'s doc). Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
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
use pack::{crc32c, ByteReader, ByteWriter};

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

/// @emoji 🔢️ How many low bits of a `run_id` are the within-kind sequence — the remaining high
/// bits are `IndexKind::tag()`. `db_storage::IndexStorage` addresses runs by a single flat `u64`
/// per document; this crate carves that space into one namespace per kind so ten kinds can share
/// one document's `IndexStorage` without colliding.
const SEQUENCE_BITS: u32 = 56;
const SEQUENCE_MASK: u64 = (1u64 << SEQUENCE_BITS) - 1;

/// @emoji 🧮️ Packs `kind` and `sequence` into one `run_id`. Errors `LimitExceeded` if `sequence`
/// doesn't fit the 56-bit namespace (never happens in practice — that's 2^56 runs of one kind for
/// one document before overflow, and the merge policy keeps live run counts tiny).
fn make_run_id(kind: IndexKind, sequence: u64) -> Result<u64, DbError> {
    if sequence > SEQUENCE_MASK {
        return Err(DbError::LimitExceeded("db_index run sequence exceeds the 56-bit per-kind namespace"));
    }
    Ok(((kind.tag() as u64) << SEQUENCE_BITS) | sequence)
}

fn kind_tag_of_run_id(run_id: u64) -> u8 {
    (run_id >> SEQUENCE_BITS) as u8
}

fn sequence_of_run_id(run_id: u64) -> u64 {
    run_id & SEQUENCE_MASK
}
//#endregion 🔖️IndexKind

//#region 🔖️SortedRun
/// @emoji 📇️ One entry's value in a sorted run: either a live payload or a tombstone recording that
/// a key was deleted (and must keep shadowing that key in any older, not-yet-merged run beneath).
pub struct IndexCursorControl {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline: std::time::Instant,
    fuel: usize,
}

impl IndexCursorControl {
    pub fn new(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("index cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel })
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
        if std::time::Instant::now() >= self.deadline {
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
        if let Err(error) = reservation.observe_capacity(source.capacity()) {
            return Err(IndexBytesRejected { source: Some(source), writer: Some(writer), error });
        }
        let mut offset = 0;
        while offset < source.len() {
            if let Err(error) = control.grant() {
                return Err(IndexBytesRejected { source: Some(source), writer: Some(writer), error });
            }
            match writer.write_fragment(&source[offset..]) {
                Ok(written) => offset += written,
                Err(error) => return Err(IndexBytesRejected { source: Some(source), writer: Some(writer), error }),
            }
            std::future::poll_fn(|context| {
                context.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
        drop(source);
        if let Err(error) = reservation.close_step() {
            return Err(IndexBytesRejected { source: None, writer: Some(writer), error });
        }
        writer.finish().map(|pages| Self { pages }).map_err(|error| IndexBytesRejected { source: None, writer: Some(writer), error })
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

    pub fn into_source(self) -> Option<Vec<u8>> {
        self.source
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

async fn admit_generated_index_bytes(source: Vec<u8>, maximum: u64, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    match IndexBytes::try_admit(source, maximum, control).await {
        Ok(bytes) => Ok(bytes),
        Err(mut rejected) => {
            control.grant()?;
            let _ = rejected.close_step()?;
            Err(rejected.error)
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
    entries: [Option<RunEntry>; MAX_RUN_ENTRIES as usize],
    len: u8,
}

impl RunEntries {
    pub fn new() -> Self {
        Self { entries: std::array::from_fn(|_| None), len: 0 }
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
        let entry = self.entries[index].as_mut().ok_or_else(|| DbError::Internal("index close lost retained entry".to_string()))?;
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

async fn peek_entry_count(pages: &db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<u64, DbError> {
    if pages.len() < 4 {
        return Err(DbError::Corrupt("index run is shorter than its checksum trailer".to_string()));
    }
    let mut reader = RunPageReader::new(pages, pages.len() - 4);
    Ok(read_run_header(&mut reader, expected_kind, control).await?.entry_count)
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
        std::future::poll_fn(|context| {
            context.waker().wake_by_ref();
            std::task::Poll::Ready(())
        })
        .await;
    }
    Ok(())
}

async fn run_write_trailer(writer: &mut db_storage::DbIoPageWriter, bytes: &[u8]) -> Result<(), DbError> {
    let mut cursor = 0;
    while cursor < bytes.len() {
        cursor += writer.write_fragment(&bytes[cursor..])?;
        std::future::poll_fn(|context| {
            context.waker().wake_by_ref();
            std::task::Poll::Ready(())
        })
        .await;
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
        std::future::poll_fn(|context| {
            context.waker().wake_by_ref();
            std::task::Poll::Ready(())
        })
        .await;
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
    writer.finish().map(|pages| IndexBytes { pages })
}

async fn index_bytes_from_slice_for_operation(operation: u64, bytes: &[u8], control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
    let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(operation, bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    let mut offset = 0usize;
    while offset < bytes.len() {
        control.grant()?;
        offset += writer.write_fragment(&bytes[offset..])?;
    }
    writer.finish().map(|pages| IndexBytes { pages })
}

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

async fn decode_run_pages(mut pages: db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
    let result = decode_run_pages_inner(&pages, expected_kind, control).await;
    let _ = pages.close_step()?;
    drop(pages);
    result
}
//#endregion 🔖️SortedRun

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
        std::future::poll_fn(|context| {
            context.waker().wake_by_ref();
            std::task::Poll::Ready(())
        })
        .await;
    }
    Ok(output)
}
//#endregion 🔖️Merge

//#region 🔖️MergePolicy
/// @emoji ⚖️ When `IndexHandle::put_batch` should automatically fold old runs together. This
/// crate's own choice (the contract fixes the LSM-lite shape, not the trigger threshold): after
/// every write, while a kind's live run count exceeds `max_runs_before_merge`, the two OLDEST runs
/// are merged into one (see `IndexHandle::maybe_auto_merge`) — a bounded, incremental amount of
/// merge work per write rather than a large stop-the-world compaction.
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

    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    /// @emoji 📋️ This handle's live run ids, ascending by sequence (oldest first) — every other id
    /// belonging to a different kind for the same document is filtered out.
    async fn kind_run_ids(&self, control: &mut IndexCursorControl) -> Result<db_storage::DbIoU64List, DbError> {
        let mut source = self.storage.list_runs(&self.document).await?;
        let mut output = db_storage::DbIoU64List::new();
        for id in source.as_slice() {
            control.grant()?;
            if kind_tag_of_run_id(*id) == self.kind.tag() {
                output.push(*id)?;
            }
        }
        control.grant()?;
        let _ = source.close_step();
        drop(source);
        Ok(output)
    }

    /// @emoji ⏭️ The sequence the next `put_batch` should claim: one past the newest live run's
    /// sequence, or `0` if this kind has no runs yet.
    async fn next_sequence(&self, control: &mut IndexCursorControl) -> Result<u64, DbError> {
        let mut ids = self.kind_run_ids(control).await?;
        let next = ids.as_slice().last().map_or(0, |id| sequence_of_run_id(*id) + 1);
        control.grant()?;
        let _ = ids.close_step();
        drop(ids);
        Ok(next)
    }

    async fn load_run(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
        let pages = self.storage.read_run(&self.document, run_id).await?;
        decode_run_pages(pages, self.kind, control).await
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
        let pages = encode_run_pages(self.kind, &unique, control).await?;
        control.grant()?;
        let _ = unique.close_step()?;
        drop(unique);
        let run_id = make_run_id(self.kind, self.next_sequence(control).await?)?;
        self.storage.write_run(&self.document, run_id, pages).await?;
        self.maybe_auto_merge(control).await
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

    /// @emoji 🔎️ Resolves `key` by scanning runs newest-to-oldest and returning the first match —
    /// `Ok(None)` if the first match is a tombstone, or if no run has ever held `key`.
    pub async fn get(&self, key: &IndexBytes, control: &mut IndexCursorControl) -> Result<Option<IndexBytes>, DbError> {
        let mut ids = self.kind_run_ids(control).await?;
        for position in (0..ids.len()).rev() {
            control.grant()?;
            let mut entries = self.load_run(ids.as_slice()[position], control).await?;
            for index in 0..entries.len() {
                match index_bytes_cmp(&entries.get(index).unwrap().key, key) {
                    std::cmp::Ordering::Less => continue,
                    std::cmp::Ordering::Greater => break,
                    std::cmp::Ordering::Equal => {
                        let entry = entries.take(index).unwrap();
                        let result = match entry.value {
                            RunValue::Put(value) => Some(value),
                            RunValue::Tombstone => None,
                        };
                        let mut key = entry.key;
                        control.grant()?;
                        let _ = key.close_step()?;
                        drop(key);
                        let _ = entries.close_step()?;
                        drop(entries);
                        let _ = ids.close_step();
                        drop(ids);
                        return Ok(result);
                    }
                }
            }
            control.grant()?;
            let _ = entries.close_step()?;
            drop(entries);
        }
        control.grant()?;
        let _ = ids.close_step();
        drop(ids);
        Ok(None)
    }

    /// @emoji 📜️ Every live (non-tombstoned) `(key, value)` whose key starts with `prefix`,
    /// ascending by key — merges every run (newest wins on collision, tombstones dropped since this
    /// is a complete view across the whole kind) then filters.
    pub async fn scan_prefix(&self, prefix: &IndexBytes, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
        let mut run_ids = self.kind_run_ids(control).await?;
        let mut merged = RunEntries::new();
        for index in 0..run_ids.len() {
            let next = self.load_run(run_ids.as_slice()[index], control).await?;
            merged = merge_run_entries(merged, next, true, control).await?;
        }
        control.grant()?;
        let _ = run_ids.close_step();
        drop(run_ids);
        let mut output = RunEntries::new();
        for index in 0..merged.len() {
            let entry = merged.take(index).unwrap();
            if entry.key.starts_with(prefix) && matches!(entry.value, RunValue::Put(_)) {
                output.push(entry).map_err(|_| DbError::LimitExceeded("index scan result owner"))?;
            } else {
                close_run_entry(entry, control).await?;
            }
        }
        Ok(output)
    }

    /// @emoji 🌀️ `MergePolicy`'s enforcement: while this kind has more live runs than
    /// `policy.max_runs_before_merge`, merges the two oldest into one (written back under the
    /// older's `run_id`, preserving the oldest-first ordering invariant `kind_run_ids` relies on;
    /// the younger's `run_id` is then deleted). Tombstones are preserved (`drop_tombstones: false`)
    /// since runs even older than these two may still exist.
    async fn maybe_auto_merge(&self, control: &mut IndexCursorControl) -> Result<(), DbError> {
        loop {
            let mut run_ids = self.kind_run_ids(control).await?;
            if run_ids.len() <= self.policy.max_runs_before_merge {
                control.grant()?;
                let _ = run_ids.close_step();
                drop(run_ids);
                return Ok(());
            }
            let (oldest, second_oldest) = (run_ids.as_slice()[0], run_ids.as_slice()[1]);
            let older = self.load_run(oldest, control).await?;
            let newer = self.load_run(second_oldest, control).await?;
            let mut merged = merge_run_entries(older, newer, false, control).await?;
            let pages = encode_run_pages(self.kind, &merged, control).await?;
            control.grant()?;
            let _ = merged.close_step()?;
            drop(merged);
            let _ = run_ids.close_step();
            drop(run_ids);
            self.storage.write_run(&self.document, oldest, pages).await?;
            self.storage.delete_run(&self.document, second_oldest).await?;
        }
    }

    /// @emoji 🧹️ Merges EVERY live run for this kind into exactly one (dropping tombstones, since
    /// nothing older remains beneath a complete merge), written back under the oldest run's
    /// `run_id`. A no-op if already at zero or one runs. Returns the post-compaction `stats()`.
    pub async fn compact(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let mut run_ids = self.kind_run_ids(control).await?;
        if run_ids.len() > 1 {
            let mut merged = RunEntries::new();
            for index in 0..run_ids.len() {
                let next = self.load_run(run_ids.as_slice()[index], control).await?;
                merged = merge_run_entries(merged, next, index + 1 == run_ids.len(), control).await?;
            }
            let pages = encode_run_pages(self.kind, &merged, control).await?;
            control.grant()?;
            let _ = merged.close_step()?;
            drop(merged);
            self.storage.write_run(&self.document, run_ids.as_slice()[0], pages).await?;
            for index in 1..run_ids.len() {
                self.storage.delete_run(&self.document, run_ids.as_slice()[index]).await?;
            }
        }
        control.grant()?;
        let _ = run_ids.close_step();
        drop(run_ids);
        self.stats(control).await
    }

    /// @emoji 📊️ Current shape of this kind's runs — see `IndexStats`'s doc for what `entry_count`
    /// does and doesn't count. Cheap: reads every run's bytes but only parses each one's header.
    pub async fn stats(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let mut run_ids = self.kind_run_ids(control).await?;
        let mut entry_count = 0u64;
        let mut total_bytes = 0u64;
        for run_id in run_ids.as_slice() {
            control.grant()?;
            let mut bytes = self.storage.read_run(&self.document, *run_id).await?;
            total_bytes += bytes.len() as u64;
            entry_count += peek_entry_count(&bytes, self.kind, control).await?;
            control.grant()?;
            let _ = bytes.close_step()?;
            drop(bytes);
        }
        let run_count = run_ids.len();
        control.grant()?;
        let _ = run_ids.close_step();
        drop(run_ids);
        Ok(IndexStats { run_count, entry_count, total_bytes })
    }

    /// @emoji ✅️ Fully decodes (checksum + structural validation) every live run for this kind,
    /// surfacing the first `DbError::Corrupt` found rather than any value — `db_cli verify`'s hook.
    pub async fn verify(&self, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let mut ids = self.kind_run_ids(control).await?;
        for index in 0..ids.len() {
            let mut entries = self.load_run(ids.as_slice()[index], control).await?;
            control.grant()?;
            let _ = entries.close_step()?;
            drop(entries);
        }
        control.grant()?;
        let _ = ids.close_step();
        drop(ids);
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

    /// @emoji 🥇️ The highest `(actor_seq, command_seq)` pair recorded for `actor`, or `None` if
    /// `actor` has never been recorded.
    pub async fn latest_for_actor(&self, actor: &ActorId) -> Result<Option<(u64, u64)>, DbError> {
        validate_actor_key_safe(actor).await?;
        let mut prefix = actor.0.as_bytes().to_vec();
        prefix.push(0u8);
        let mut control = self.handle.operation_control(16_384)?;
        let prefix_owner = admit_generated_index_bytes(prefix, MAX_KEY_LEN, &mut control).await?;
        let mut entries = self.handle.scan_prefix(&prefix_owner, &mut control).await?;
        close_index_bytes(prefix_owner, &mut control).await?;
        let result = if entries.is_empty() {
            None
        } else {
            let entry = entries.take(entries.len() - 1).unwrap();
            let mut suffix = [0u8; 8];
            if entry.key.read_fragment(entry.key.len().saturating_sub(8), &mut suffix) != 8 {
                return Err(DbError::Corrupt("actor-seq index key has a malformed suffix".to_string()));
            }
            let command = match entry.value {
                RunValue::Put(value) => decode_index_bytes(value, &mut control, decode_u64_le).await?,
                RunValue::Tombstone => return Err(DbError::Corrupt("live index scan returned tombstone".to_string())),
            };
            close_index_bytes(entry.key, &mut control).await?;
            Some((u64::from_be_bytes(suffix), command))
        };
        control.grant()?;
        let _ = entries.close_step()?;
        drop(entries);
        Ok(result)
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

    /// @emoji 🥇️ The frontier recorded under the highest `commit_seq`, or `None` if none recorded.
    pub async fn latest(&self) -> Result<Option<Frontier>, DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        let prefix = admit_generated_index_bytes(Vec::new(), MAX_KEY_LEN, &mut control).await?;
        let mut entries = self.handle.scan_prefix(&prefix, &mut control).await?;
        close_index_bytes(prefix, &mut control).await?;
        let result = if entries.is_empty() {
            None
        } else {
            let entry = entries.take(entries.len() - 1).unwrap();
            close_index_bytes(entry.key, &mut control).await?;
            match entry.value {
                RunValue::Put(value) => Some(decode_index_bytes(value, &mut control, decode_frontier).await?),
                RunValue::Tombstone => None,
            }
        };
        control.grant()?;
        let _ = entries.close_step()?;
        drop(entries);
        Ok(result)
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
    writer.finish().map(|pages| IndexBytes { pages })
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
        let mut control = self.handle.operation_control(32_768)?;
        let prefix_owner = admit_generated_index_bytes(prefix, MAX_KEY_LEN, &mut control).await?;
        let mut entries = self.handle.scan_prefix(&prefix_owner, &mut control).await?;
        let mut result = None;
        for index in 0..entries.len() {
            let entry = entries.take(index).unwrap();
            let mut seq_bytes = [0u8; 8];
            if entry.key.read_fragment(entry.key.len().saturating_sub(8), &mut seq_bytes) != 8 {
                return Err(DbError::Corrupt("projection index key has a malformed suffix".to_string()));
            }
            let seq = u64::from_be_bytes(seq_bytes);
            if seq > frontier_seq {
                close_run_entry(entry, &mut control).await?;
                break;
            }
            if let Some((_, previous)) = result.take() {
                close_index_bytes(previous, &mut control).await?;
            }
            close_index_bytes(entry.key, &mut control).await?;
            result = match entry.value {
                RunValue::Put(value) => Some((seq, value)),
                RunValue::Tombstone => None,
            };
        }
        control.grant()?;
        let _ = entries.close_step()?;
        drop(entries);
        close_index_bytes(prefix_owner, &mut control).await?;
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
mod tests {
    use super::*;
    use db_storage::MemoryStorage;

    fn control() -> IndexCursorControl {
        IndexCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap()
    }

    async fn retained(source: &[u8]) -> IndexBytes {
        let mut control = control();
        IndexBytes::try_admit(source.to_vec(), MAX_VALUE_LEN, &mut control).await.unwrap()
    }

    async fn retained_vec(source: Vec<u8>) -> IndexBytes {
        let mut control = control();
        IndexBytes::try_admit(source, MAX_VALUE_LEN, &mut control).await.unwrap()
    }

    async fn read_retained(bytes: &IndexBytes) -> Vec<u8> {
        let mut prepared = bytes.prepare_platform().await.unwrap();
        let output = prepared.as_slice().to_vec();
        while prepared.close_step().unwrap() {}
        output
    }

    async fn entry(key: &[u8], value: &[u8]) -> RunEntry {
        RunEntry { key: retained(key).await, value: RunValue::Put(retained(value).await) }
    }

    async fn tombstone(key: &[u8]) -> RunEntry {
        RunEntry { key: retained(key).await, value: RunValue::Tombstone }
    }

    async fn run(entries: impl IntoIterator<Item = RunEntry>) -> RunEntries {
        let mut run = RunEntries::new();
        for entry in entries {
            assert!(run.push(entry).is_ok());
        }
        run
    }

    async fn assert_entry(entry: &RunEntry, key: &[u8], value: Option<&[u8]>) {
        assert_eq!(read_retained(&entry.key).await, key);
        match (&entry.value, value) {
            (RunValue::Put(actual), Some(expected)) => assert_eq!(read_retained(actual).await, expected),
            (RunValue::Tombstone, None) => {}
            _ => panic!("retained run value shape mismatch"),
        }
    }

    async fn put_bytes<S: IndexStorage>(handle: &IndexHandle<'_, S>, key: &[u8], value: &[u8]) {
        let mut control = control();
        handle.put(retained(key).await, retained(value).await, &mut control).await.unwrap();
    }

    async fn delete_bytes<S: IndexStorage>(handle: &IndexHandle<'_, S>, key: &[u8]) {
        let mut control = control();
        handle.delete(retained(key).await, &mut control).await.unwrap();
    }

    async fn get_bytes<S: IndexStorage>(handle: &IndexHandle<'_, S>, key: &[u8]) -> Option<Vec<u8>> {
        let mut control = control();
        let mut key = retained(key).await;
        let result = handle.get(&key, &mut control).await.unwrap();
        while key.close_step().unwrap().is_some() {}
        match result {
            Some(mut bytes) => {
                let output = read_retained(&bytes).await;
                while bytes.close_step().unwrap().is_some() {}
                Some(output)
            }
            None => None,
        }
    }

    async fn stats<S: IndexStorage>(handle: &IndexHandle<'_, S>) -> IndexStats {
        let mut control = control();
        handle.stats(&mut control).await.unwrap()
    }

    //#region 🔖️SortedRun
    #[semio_framework_async_macros::async_test]
    async fn run_round_trips_through_encode_and_decode() {
        let mut entries = run([entry(b"a", b"1").await, entry(b"b", b"2").await, tombstone(b"c").await]).await;
        let mut control = control();
        let encoded = encode_run_pages(IndexKind::Command, &entries, &mut control).await.unwrap();
        while entries.close_step().unwrap() {}
        let mut decoded = decode_run_pages(encoded, IndexKind::Command, &mut control).await.unwrap();
        assert_entry(decoded.get(0).unwrap(), b"a", Some(b"1")).await;
        assert_entry(decoded.get(1).unwrap(), b"b", Some(b"2")).await;
        assert_entry(decoded.get(2).unwrap(), b"c", None).await;
        while decoded.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_run_detects_corruption_via_checksum() {
        let mut entries = run([entry(b"a", b"1").await]).await;
        let mut control = control();
        let encoded = encode_run_pages(IndexKind::Command, &entries, &mut control).await.unwrap();
        while entries.close_step().unwrap() {}
        let mut prepared = db_storage::db_io_prepare_platform(&encoded).unwrap().await.unwrap();
        let mut corrupt = prepared.as_slice().to_vec();
        let last = corrupt.len() - 1;
        corrupt[last] ^= 0xFF;
        while prepared.close_step().unwrap() {}
        let mut encoded = encoded;
        while encoded.close_step().unwrap().is_some() {}
        let corrupt = retained_vec(corrupt).await;
        assert!(matches!(decode_run_pages(corrupt.pages, IndexKind::Command, &mut control).await, Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_run_rejects_kind_mismatch() {
        let mut entries = run([entry(b"a", b"1").await]).await;
        let mut control = control();
        let encoded = encode_run_pages(IndexKind::Command, &entries, &mut control).await.unwrap();
        while entries.close_step().unwrap() {}
        assert!(matches!(decode_run_pages(encoded, IndexKind::Commit, &mut control).await, Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_run_rejects_non_ascending_entries() {
        // Hand-build a malformed body bypassing `encode_run`'s own ordering check, to exercise
        // `decode_run`'s independent defensive re-validation.
        let mut writer = ByteWriter::new();
        writer.write_bytes(&RUN_MAGIC);
        writer.write_u8(RUN_VERSION);
        writer.write_u8(IndexKind::Command.tag());
        writer.write_varint_u64(2);
        for key in [b"b".as_slice(), b"a".as_slice()] {
            writer.write_varint_u64(key.len() as u64);
            writer.write_bytes(key);
            writer.write_u8(1);
            writer.write_varint_u64(1);
            writer.write_bytes(b"x");
        }
        let mut bytes = writer.into_bytes();
        let checksum = crc32c(&bytes);
        bytes.extend_from_slice(&checksum.to_le_bytes());
        let mut control = control();
        let bytes = retained_vec(bytes).await;
        assert!(matches!(decode_run_pages(bytes.pages, IndexKind::Command, &mut control).await, Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_run_sorts_and_last_write_wins_on_duplicate_keys() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let handle = IndexHandle::new(&storage, ArtifactId::from("sort"), IndexKind::Command).await;
        let entries = run([entry(b"b", b"1").await, entry(b"a", b"2").await, entry(b"b", b"3").await]).await;
        let mut control = control();
        handle.put_batch(entries, &mut control).await.unwrap();
        assert_eq!(get_bytes(&handle, b"a").await, Some(b"2".to_vec()));
        assert_eq!(get_bytes(&handle, b"b").await, Some(b"3".to_vec()));
    }
    //#endregion 🔖️SortedRun

    //#region 🔖️Merge
    #[semio_framework_async_macros::async_test]
    async fn merge_runs_prefers_newest_and_respects_drop_tombstones() {
        let older = run([entry(b"a", b"old-a").await, entry(b"b", b"old-b").await]).await;
        let newer = run([tombstone(b"b").await, entry(b"c", b"new-c").await]).await;
        let mut control = control();
        let mut merged = merge_run_entries(older, newer, false, &mut control).await.unwrap();
        assert_entry(merged.get(0).unwrap(), b"a", Some(b"old-a")).await;
        assert_entry(merged.get(1).unwrap(), b"b", None).await;
        assert_entry(merged.get(2).unwrap(), b"c", Some(b"new-c")).await;
        while merged.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_runs_of_zero_runs_is_empty() {
        let mut control = control();
        assert!(merge_run_entries(RunEntries::new(), RunEntries::new(), true, &mut control).await.unwrap().is_empty());
    }
    //#endregion 🔖️Merge

    //#region 🔖️IndexKind
    #[semio_framework_async_macros::async_test]
    async fn run_id_round_trips_kind_and_sequence_for_every_kind() {
        for kind in IndexKind::ALL {
            for sequence in [0u64, 1, SEQUENCE_MASK] {
                let run_id = make_run_id(kind, sequence).expect("make_run_id");
                assert_eq!(kind_tag_of_run_id(run_id), kind.tag());
                assert_eq!(sequence_of_run_id(run_id), sequence);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn run_id_rejects_sequence_overflowing_the_namespace() {
        assert!(matches!(make_run_id(IndexKind::Command, SEQUENCE_MASK + 1), Err(DbError::LimitExceeded(_))));
    }
    //#endregion 🔖️IndexKind

    //#region 🔖️IndexHandle
    #[semio_framework_async_macros::async_test]
    async fn index_handle_put_get_delete_round_trips() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        put_bytes(&handle, b"k1", b"v1").await;
        put_bytes(&handle, b"k2", b"v2").await;
        assert_eq!(get_bytes(&handle, b"k1").await, Some(b"v1".to_vec()));
        assert_eq!(get_bytes(&handle, b"k2").await, Some(b"v2".to_vec()));
        assert_eq!(get_bytes(&handle, b"missing").await, None);
        delete_bytes(&handle, b"k1").await;
        assert_eq!(get_bytes(&handle, b"k1").await, None);
        assert_eq!(get_bytes(&handle, b"k2").await, Some(b"v2".to_vec()));
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_put_overwrites_earlier_value_for_same_key() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        put_bytes(&handle, b"k", b"first").await;
        put_bytes(&handle, b"k", b"second").await;
        assert_eq!(get_bytes(&handle, b"k").await, Some(b"second".to_vec()));
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_scan_prefix_returns_sorted_live_entries_only() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        put_bytes(&handle, b"a/1", b"1").await;
        put_bytes(&handle, b"a/2", b"2").await;
        put_bytes(&handle, b"b/1", b"3").await;
        delete_bytes(&handle, b"a/2").await;
        let mut control = control();
        let mut prefix = retained(b"a/").await;
        let mut scanned = handle.scan_prefix(&prefix, &mut control).await.unwrap();
        assert_eq!(scanned.len(), 1);
        assert_entry(scanned.get(0).unwrap(), b"a/1", Some(b"1")).await;
        while scanned.close_step().unwrap() {}
        while prefix.close_step().unwrap().is_some() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_auto_merges_to_stay_within_policy() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let policy = MergePolicy { max_runs_before_merge: 2 };
        let handle = IndexHandle::with_policy(&storage, ArtifactId::from("doc-1"), IndexKind::Command, policy).await;
        for i in 0..6u64 {
            put_bytes(&handle, format!("k{i:03}").as_bytes(), &i.to_le_bytes()).await;
        }
        let shape = stats(&handle).await;
        assert!(shape.run_count <= 2, "run_count {} should respect the merge policy", shape.run_count);
        for i in 0..6u64 {
            let value = get_bytes(&handle, format!("k{i:03}").as_bytes()).await.unwrap();
            assert_eq!(u64::from_le_bytes(value.try_into().expect("8 bytes")), i);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_compact_collapses_to_one_run_and_drops_tombstones() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        put_bytes(&handle, b"a", b"1").await;
        put_bytes(&handle, b"b", b"2").await;
        delete_bytes(&handle, b"a").await;
        let mut control = control();
        let shape = handle.compact(&mut control).await.unwrap();
        assert_eq!(shape.run_count, 1);
        assert_eq!(shape.entry_count, 1);
        assert_eq!(get_bytes(&handle, b"a").await, None);
        assert_eq!(get_bytes(&handle, b"b").await, Some(b"2".to_vec()));
        handle.verify(&mut control).await.unwrap();
    }

    #[semio_framework_async_macros::async_test]
    async fn index_handle_compact_of_one_run_is_a_no_op() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
        put_bytes(&handle, b"a", b"1").await;
        let before = stats(&handle).await;
        let mut control = control();
        let after = handle.compact(&mut control).await.unwrap();
        assert_eq!(before, after);
    }

    #[semio_framework_async_macros::async_test]
    async fn different_kinds_do_not_collide_for_the_same_document() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId::from("doc-1");
        let commands = IndexHandle::new(&storage, document.clone(), IndexKind::Command).await;
        let regions = IndexHandle::new(&storage, document, IndexKind::TouchedRegion).await;

        put_bytes(&commands, b"shared-key", b"command-value").await;
        put_bytes(&regions, b"shared-key", b"region-value").await;
        assert_eq!(get_bytes(&commands, b"shared-key").await, Some(b"command-value".to_vec()));
        assert_eq!(get_bytes(&regions, b"shared-key").await, Some(b"region-value".to_vec()));
        assert_eq!(stats(&commands).await.run_count, 1);
        assert_eq!(stats(&regions).await.run_count, 1);
    }
    //#endregion 🔖️IndexHandle

    //#region 🔖️TypedIndexes
    #[semio_framework_async_macros::async_test]
    async fn command_index_records_and_looks_up_locations() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = CommandIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let location = RecordLocation { segment: 3, offset: 128, len: 64 };
        db_actor::block_on(index.record(42, location)).expect("record");
        assert_eq!(db_actor::block_on(index.lookup(42)).expect("lookup"), Some(location));
        assert_eq!(db_actor::block_on(index.lookup(43)).expect("lookup"), None);
        db_actor::block_on(index.remove(42)).expect("remove");
        assert_eq!(db_actor::block_on(index.lookup(42)).expect("lookup"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_index_records_and_looks_up_locations() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = InverseIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let location = RecordLocation { segment: 1, offset: 0, len: 16 };
        db_actor::block_on(index.record(7, location)).expect("record");
        assert_eq!(db_actor::block_on(index.lookup(7)).expect("lookup"), Some(location));
    }

    #[semio_framework_async_macros::async_test]
    async fn actor_seq_index_resolves_and_tracks_latest_per_actor() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = ActorSeqIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let alice = ActorId::from("alice");
        let bob = ActorId::from("bob");
        db_actor::block_on(index.record(&alice, 1, 100)).expect("record");
        db_actor::block_on(index.record(&alice, 2, 101)).expect("record");
        db_actor::block_on(index.record(&bob, 1, 200)).expect("record");

        assert_eq!(db_actor::block_on(index.lookup(&alice, 1)).expect("lookup"), Some(100));
        assert_eq!(db_actor::block_on(index.lookup(&alice, 2)).expect("lookup"), Some(101));
        assert_eq!(db_actor::block_on(index.latest_for_actor(&alice)).expect("latest"), Some((2, 101)));
        assert_eq!(db_actor::block_on(index.latest_for_actor(&bob)).expect("latest"), Some((1, 200)));
        assert_eq!(db_actor::block_on(index.latest_for_actor(&ActorId::from("carol"))).expect("latest"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn actor_seq_index_rejects_actor_id_with_embedded_nul() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = ActorSeqIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let unsafe_actor = ActorId::from("bad\u{0}actor");
        assert!(matches!(db_actor::block_on(index.record(&unsafe_actor, 1, 1)), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn frontier_index_round_trips_and_tracks_latest() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = FrontierIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let first = Frontier { document: ArtifactId::from("doc-1"), head_seq: 1, commit_seq: 1, chain_hash: [1u8; 32], epoch: 0 };
        let second = Frontier { document: ArtifactId::from("doc-1"), head_seq: 5, commit_seq: 2, chain_hash: [2u8; 32], epoch: 1 };
        db_actor::block_on(index.record(&first)).expect("record");
        db_actor::block_on(index.record(&second)).expect("record");

        assert_eq!(db_actor::block_on(index.lookup(1)).expect("lookup"), Some(first));
        assert_eq!(db_actor::block_on(index.latest()).expect("latest"), Some(second));
    }

    #[semio_framework_async_macros::async_test]
    async fn touched_region_index_accumulates_sorted_unique_seqs() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = TouchedRegionIndex::new(&storage, ArtifactId::from("doc-1")).await;
        db_actor::block_on(index.record_touch(b"region-a", 5)).expect("record_touch");
        db_actor::block_on(index.record_touch(b"region-a", 2)).expect("record_touch");
        db_actor::block_on(index.record_touch(b"region-a", 5)).expect("record_touch");
        assert_eq!(db_actor::block_on(index.touching(b"region-a")).expect("touching"), vec![2, 5]);
        assert_eq!(db_actor::block_on(index.touching(b"region-b")).expect("touching"), Vec::<u64>::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn commit_index_round_trips() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = CommitIndex::new(&storage, ArtifactId::from("doc-1")).await;
        db_actor::block_on(index.record("ck-abc123", 9)).expect("record");
        assert_eq!(db_actor::block_on(index.lookup("ck-abc123")).expect("lookup"), Some(9));
        assert_eq!(db_actor::block_on(index.lookup("ck-missing")).expect("lookup"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn full_text_index_search_finds_indexed_documents() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = FullTextIndex::new(&storage, ArtifactId::from("doc-1")).await;
        db_actor::block_on(index.index_document(1, "The Quick Brown Fox")).expect("index");
        db_actor::block_on(index.index_document(2, "quick jumps")).expect("index");

        assert_eq!(db_actor::block_on(index.search("quick")).expect("search"), vec![1, 2]);
        assert_eq!(db_actor::block_on(index.search("QUICK")).expect("search"), vec![1, 2]);
        assert_eq!(db_actor::block_on(index.search("fox")).expect("search"), vec![1]);
        assert_eq!(db_actor::block_on(index.search("absent")).expect("search"), Vec::<u64>::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn conflict_index_accumulates_multiple_records_per_command() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = ConflictIndex::new(&storage, ArtifactId::from("doc-1")).await;
        index.record_conflict(5, retained(b"region-collision").await).await.unwrap();
        index.record_conflict(5, retained(b"constraint-violation").await).await.unwrap();
        index.record_conflict(6, retained(b"other").await).await.unwrap();
        let mut records = index.conflicts_for(5).await.unwrap();
        assert_eq!(read_retained(records.get(0).unwrap()).await, b"region-collision");
        assert_eq!(read_retained(records.get(1).unwrap()).await, b"constraint-violation");
        while records.close_step().unwrap() {}
        let mut records = index.conflicts_for(6).await.unwrap();
        assert_eq!(read_retained(records.get(0).unwrap()).await, b"other");
        while records.close_step().unwrap() {}
        let mut records = index.conflicts_for(7).await.unwrap();
        assert_eq!(records.len(), 0);
        while records.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_index_resolves_exact_and_floor_lookups_scoped_to_projection_id() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = ProjectionIndex::new(&storage, ArtifactId::from("doc-1")).await;
        index.record("by-author", 10, retained(b"state-10").await).await.unwrap();
        index.record("by-author", 20, retained(b"state-20").await).await.unwrap();
        let mut exact = index.at("by-author", 10).await.unwrap().unwrap();
        assert_eq!(read_retained(&exact).await, b"state-10");
        while exact.close_step().unwrap().is_some() {}
        assert!(index.at("by-author", 15).await.unwrap().is_none());
        let (sequence, mut floor) = index.latest_at_or_before("by-author", 15).await.unwrap().unwrap();
        assert_eq!(sequence, 10);
        assert_eq!(read_retained(&floor).await, b"state-10");
        while floor.close_step().unwrap().is_some() {}
        let (sequence, mut floor) = index.latest_at_or_before("by-author", 20).await.unwrap().unwrap();
        assert_eq!(sequence, 20);
        assert_eq!(read_retained(&floor).await, b"state-20");
        while floor.close_step().unwrap().is_some() {}
        assert_eq!(db_actor::block_on(index.latest_at_or_before("by-author", 5)).expect("latest_at_or_before"), None);
        // 🎯️ "by-color" sorts after "by-author" but has no entries at all — must not fall back to
        // a lexicographically-earlier projection's entry.
        assert_eq!(db_actor::block_on(index.latest_at_or_before("by-color", 100)).expect("latest_at_or_before"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_index_rejects_projection_id_with_embedded_nul() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = ProjectionIndex::new(&storage, ArtifactId::from("doc-1")).await;
        assert!(matches!(index.record("bad\u{0}id", 1, retained(&[1]).await).await, Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_index_coalesces_latest_publish_or_withdraw_per_actor_and_key() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = PreviewIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let alice = ActorId::from("alice");

        index.publish(&alice, "drag-ghost", retained(&[1]).await).await.unwrap();
        let mut latest = index.latest(&alice, "drag-ghost").await.unwrap().unwrap();
        assert_eq!(read_retained(&latest).await, [1]);
        while latest.close_step().unwrap().is_some() {}
        index.publish(&alice, "drag-ghost", retained(&[2]).await).await.unwrap();
        index.publish(&alice, "cursor", retained(&[9]).await).await.unwrap();
        let mut for_alice = index.for_actor(&alice).await.unwrap();
        assert_eq!(for_alice.len(), 2);
        while for_alice.close_step().unwrap() {}

        db_actor::block_on(index.withdraw(&alice, "drag-ghost")).expect("withdraw");
        assert!(index.latest(&alice, "drag-ghost").await.unwrap().is_none());
        let mut for_alice = index.for_actor(&alice).await.unwrap();
        assert_eq!(for_alice.len(), 1);
        while for_alice.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_index_rejects_actor_id_with_embedded_nul() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let index = PreviewIndex::new(&storage, ArtifactId::from("doc-1")).await;
        let unsafe_actor = ActorId::from("bad\u{0}actor");
        assert!(matches!(index.publish(&unsafe_actor, "k", retained(&[1]).await).await, Err(DbError::InvalidArgument(_))));
    }
    //#endregion 🔖️TypedIndexes
}
//#endregion 🧪️Tests
//#region 🧪️RetainedTests
#[cfg(test)]
mod retained_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn exact_backing_handback_cancel_close_and_fragment_order_are_deterministic() {
        let _pool = crate::db_storage::db_io_test_pool();
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = IndexCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
        let mut one = Vec::with_capacity(db_storage::DB_IO_PAGE_BYTES + 1);
        one.push(b'a');
        let mut retained = IndexBytes::try_admit(one, (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut control).await.unwrap();
        let mut second_source = Vec::with_capacity(db_storage::DB_IO_PAGE_BYTES + 1);
        second_source.push(b'b');
        let mut second = IndexBytes::try_admit(second_source, (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut control).await.unwrap();
        assert_eq!(index_bytes_cmp(&retained, &second), std::cmp::Ordering::Less);
        while retained.close_step().unwrap().is_some() {}
        while second.close_step().unwrap().is_some() {}
        assert!(retained.terminal_is_empty());
        assert!(second.terminal_is_empty());

        let mut source = Vec::with_capacity(65);
        source.push(1);
        let pointer = source.as_ptr();
        let rejected = IndexBytes::try_admit(source, 64, &mut control).await.unwrap_err();
        let returned = rejected.into_source().unwrap();
        assert_eq!(returned.as_ptr(), pointer);
        assert_eq!(returned.capacity(), 65);

        cancelled.store(true, std::sync::atomic::Ordering::Release);
        let mut source = Vec::with_capacity(8);
        source.push(1);
        let pointer = source.as_ptr();
        let mut rejected = IndexBytes::try_admit(source, 8, &mut control).await.unwrap_err();
        while rejected.close_step().unwrap() {}
        assert_eq!(rejected.into_source().unwrap().as_ptr(), pointer);
    }
}
//#endregion 🧪️RetainedTests
