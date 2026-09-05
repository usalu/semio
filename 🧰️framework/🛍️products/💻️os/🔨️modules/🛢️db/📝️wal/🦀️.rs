//! 🗄️ `db_wal` — the `db` crate family's write-ahead log: a per-document, per-segment `.spr`
//! container (reusing `protocol::{SprWriter, FrameCursor, ReverseFrameCursor}` and
//! `protocol::format::recover` directly — no new framing invented) built on top of
//! `db_storage::WalStorage`'s opaque byte segments. Owns the WAL's own record kinds (SPR
//! extension range `0x40..=0x4F`), real group-commit batching (bounded by delay/bytes/count),
//! segment rotation with a cross-segment hash chain, and crash recovery (torn-tail truncation,
//! multi-segment replay). Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice (dependency): the `protocol` facade re-exports `SprWriter`/`FrameCursor`/
//! `ReverseFrameCursor`/`RecoveryMode`/`RecoveryReport`/`ProtocolError`/`ProtocolLimits`, but not
//! the free functions `protocol::format::recover`/`parse_commit_payload`, the constants
//! `COMMIT_FRAME_LEN`/`HEADER_SIZE`, or `protocol::wire::REC_COMMIT`/`REQUIRED_HASH_CHAIN` — this
//! crate needs all of those (recovery; reading back a just-sealed segment's tip `chain_hash` to
//! seed the next segment's cross-segment chain; recognizing a `REC_COMMIT` frame explicitly while
//! decoding rather than assuming "not one of ours" means "must be a commit"; opening every segment
//! with hash-chaining required), so it takes direct `protocol_format`/`protocol_core` path
//! dependencies alongside `protocol` rather than duplicating their logic. Every other
//! protocol-family type this crate touches comes from the `protocol` facade, per the contract.
use crate::db_durability::Frontier;
use crate::*;
/// @emoji 📍️ First record marker in a fresh segment: document identity, segment index, and the
/// previous segment's final commit `chain_hash` (the WAL's cross-segment hash chain — protocol's
/// own commit chain resets to `chain_0 = blake3(header)` at every segment boundary, since
/// `SprWriter::begin` always starts fresh; this record is how a document's WAL stays one
/// verifiable chain across segment rotation).
pub const WAL_SEGMENT_HEADER: u8 = 0x40;
/// @emoji 🚪️ Opens a logical transaction: one group of records (typically one `submit()` batch)
/// that must be applied atomically on replay.
pub const WAL_TX_BEGIN: u8 = 0x41;
/// @emoji 🏁️ Closes a transaction successfully; carries the record count written since the
/// matching `WAL_TX_BEGIN` as a replay sanity check.
pub const WAL_TX_COMMIT: u8 = 0x42;
/// @emoji 🚫️ Closes a transaction as rolled back; replay must discard every record since the
/// matching `WAL_TX_BEGIN`.
pub const WAL_TX_ABORT: u8 = 0x43;
/// @emoji ✉️ A `protocol::MutationEnvelope`'s bytes, stored verbatim (zero-copy on the write
/// path — this crate never re-encodes what `db_artifact` hands it).
pub const WAL_COMMAND: u8 = 0x44;
/// @emoji 🫙️ A command payload, either inlined or referenced by CAS hash (see `WalPayloadRef`).
pub const WAL_PAYLOAD: u8 = 0x45;
/// @emoji 🔀️ An opaque `protocol::MutationDiff`-shaped byte blob (db crates below `db_artifact`
/// never interpret operation semantics, per the contract's hard dependency rule).
pub const WAL_DIFF: u8 = 0x46;
/// @emoji ⏪️ An opaque inverse/undo byte blob for `db_artifact`'s inverse-undo pipeline.
pub const WAL_INVERSE: u8 = 0x47;
/// @emoji 📣️ An opaque effect/notification byte blob.
pub const WAL_EVENT: u8 = 0x48;
/// @emoji 📤️ An opaque outgoing-effect outbox entry byte blob.
pub const WAL_OUTBOX: u8 = 0x49;
/// @emoji 🧭️ A structured `Frontier` snapshot, written periodically so recovery can
/// cross-check replay against a known-good checkpoint.
pub const WAL_FRONTIER: u8 = 0x4A;
/// @emoji 🌿️ An opaque vcs change/checkpoint id string, recorded alongside the WAL entry that
/// produced it (`db_engine`'s `VersionGraph` seam; this crate stores the id, never interprets it).
pub const WAL_VCS_REF: u8 = 0x4B;
/// @emoji 📸️ Marks that a snapshot generation was published, with the frontier it covers.
pub const WAL_SNAPSHOT_PUB: u8 = 0x4C;
/// @emoji 🔖️ Marks an index checkpoint: the set of `db_index` run ids current as of this point.
pub const WAL_INDEX_CKPT: u8 = 0x4D;
/// @emoji ⏳️ A lease grant/renewal/release record (`db_cluster`'s ownership-lease durability).
pub const WAL_LEASE: u8 = 0x4E;
/// @emoji 🚚️ An opaque schema/data migration descriptor byte blob.
pub const WAL_MIGRATION: u8 = 0x4F;

/// @emoji ✅️ True iff `kind` falls in this crate's SPR extension range — the test every reader
/// (`WalReplayCursor`) uses to decide whether a frame is one of ours (vs.
/// protocol's own `REC_COMMIT`, which every `.spr` stream also contains and this crate skips).
pub const fn is_wal_record_kind(kind: u8) -> bool {
    kind >= WAL_SEGMENT_HEADER && kind <= WAL_MIGRATION
}
//#endregion 🔖️RecordKinds

//#region 🔖️Records
/// @emoji 🛡️ Ceiling on any single length-prefixed scalar field (document id, resource name,
/// holder id, vcs ref) this crate decodes — validated BEFORE allocating, mirroring
/// `pack::PackLimits`'s stated invariant. Generous for any legitimate identifier, small
/// enough to reject an obviously-corrupt length up front.
const MAX_FIELD_BYTES: u64 = 1024 * 1024;
/// @emoji 🛡️ Ceiling on a `WAL_INDEX_CKPT` record's run-id count, validated before the `Vec` is
/// sized.
const MAX_RUN_IDS: u64 = 1_000_000;

/// @emoji ✍️ Writes a varint-length-prefixed byte field — this crate's one field encoding used by
/// every string/id-shaped record field.
#[cfg(test)]
async fn write_field(writer: &mut pack::ByteWriter, bytes: &[u8]) {
    writer.write_varint_u64(bytes.len() as u64);
    writer.write_bytes(bytes);
}

/// @emoji 📖️ Inverse of `write_field`.
#[cfg(test)]
async fn read_field_bytes(reader: &mut pack::ByteReader<'_>) -> Result<Vec<u8>, DbError> {
    let len = reader.read_varint_u64()?;
    check_len(len, MAX_FIELD_BYTES, "wal_record::field")?;
    Ok(reader.read_bytes(len as usize)?.to_vec())
}

/// @emoji 📖️ `read_field_bytes` plus a utf-8 validation, for text fields.
#[cfg(test)]
async fn read_field_string(reader: &mut pack::ByteReader<'_>) -> Result<String, DbError> {
    String::from_utf8(read_field_bytes(reader).await?).map_err(|_| DbError::Corrupt("wal record field is not valid utf-8".to_string()))
}

#[cfg(test)]
async fn encode_frontier(writer: &mut pack::ByteWriter, frontier: &Frontier) {
    write_field(writer, frontier.document.0.as_bytes()).await;
    writer.write_u64_le(frontier.head_seq);
    writer.write_u64_le(frontier.commit_seq);
    writer.write_bytes(&frontier.chain_hash);
    writer.write_u64_le(frontier.epoch);
}

fn wal_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
    let mut len = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output[len] = byte;
        len += 1;
        if value == 0 {
            return &output[..len];
        }
    }
}

fn wal_varint_len(value: u64) -> usize {
    let mut output = [0u8; 10];
    wal_varint(value, &mut output).len()
}

pub(crate) fn wal_read_canonical_varint(mut next: impl FnMut() -> Result<u8, DbError>) -> Result<u64, DbError> {
    let mut value = 0u64;
    for shift in (0..70).step_by(7) {
        let byte = next()?;
        if shift == 63 && byte > 1 { return Err(DbError::Corrupt("wal retained varint exceeds u64".to_string())); }
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            if shift != 0 && byte == 0 { return Err(DbError::Corrupt("wal retained varint is noncanonical".to_string())); }
            return Ok(value);
        }
    }
    Err(DbError::Corrupt("wal retained varint exceeds u64".to_string()))
}

fn wal_field_len(bytes: &[u8]) -> usize {
    wal_varint_len(bytes.len() as u64) + bytes.len()
}

fn wal_frontier_len(frontier: &Frontier) -> usize {
    wal_field_len(frontier.document.0.as_bytes()) + 8 + 8 + 32 + 8
}

async fn wal_record_write(record: &mut protocol::SprIdentityRecord<'_, SharedBuf>, bytes: &[u8]) -> Result<(), DbError> {
    record.write_fragment(bytes).await.map_err(protocol_err)
}

async fn wal_record_write_field(record: &mut protocol::SprIdentityRecord<'_, SharedBuf>, bytes: &[u8]) -> Result<(), DbError> {
    let mut length = [0u8; 10];
    wal_record_write(record, wal_varint(bytes.len() as u64, &mut length)).await?;
    wal_record_write(record, bytes).await
}

async fn wal_record_write_frontier(record: &mut protocol::SprIdentityRecord<'_, SharedBuf>, frontier: &Frontier) -> Result<(), DbError> {
    wal_record_write_field(record, frontier.document.0.as_bytes()).await?;
    wal_record_write(record, &frontier.head_seq.to_le_bytes()).await?;
    wal_record_write(record, &frontier.commit_seq.to_le_bytes()).await?;
    wal_record_write(record, &frontier.chain_hash).await?;
    wal_record_write(record, &frontier.epoch.to_le_bytes()).await
}

#[cfg(test)]
async fn decode_frontier(reader: &mut pack::ByteReader<'_>) -> Result<Frontier, DbError> {
    let document = ArtifactId(read_field_string(reader).await?);
    let head_seq = reader.read_u64_le()?;
    let commit_seq = reader.read_u64_le()?;
    let chain_hash = reader.read_array32()?;
    let epoch = reader.read_u64_le()?;
    Ok(Frontier { document, head_seq, commit_seq, chain_hash, epoch })
}

pub struct WalCursorControl {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline: std::time::Instant,
    fuel: usize,
}

impl WalCursorControl {
    pub fn new(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("wal cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel })
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("wal cursor fuel"));
        }
        self.deadline = deadline;
        self.fuel = fuel;
        Ok(())
    }

    fn check(&self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("wal cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("wal cursor deadline reached".to_string()));
        }
        Ok(())
    }

    pub fn grant(&mut self) -> Result<(), DbError> {
        self.check()?;
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("wal cursor fuel"))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct WalBytes {
    pages: db_storage::DbIoPages,
}

pub struct WalBytesCursor<'bytes> {
    bytes: &'bytes WalBytes,
    offset: usize,
}

#[derive(Debug)]
pub struct WalBytesRejected {
    source: Option<Vec<u8>>,
    writer: Option<db_storage::DbIoPageWriter>,
    error: DbError,
}

impl WalBytes {
    pub async fn try_admit(source: Vec<u8>, maximum: u64, control: &mut WalCursorControl) -> Result<Self, WalBytesRejected> {
        if source.capacity() as u64 > maximum {
            return Err(WalBytesRejected { source: Some(source), writer: None, error: DbError::LimitExceeded("wal source backing capacity") });
        }
        let mut writer = match db_storage::DbIoPageWriter::try_reserve(source.capacity().div_ceil(db_storage::DB_IO_PAGE_BYTES)) {
            Ok(writer) => writer,
            Err(rejected) => return Err(WalBytesRejected { source: Some(source), writer: rejected.into_writer(), error: DbError::Unavailable("wal page admission rejected".to_string()) }),
        };
        let mut reservation = match db_storage::DbIoDriverReservation::try_reserve(writer.operation(), source.capacity()) {
            Ok(reservation) => reservation,
            Err(error) => return Err(WalBytesRejected { source: Some(source), writer: Some(writer), error }),
        };
        let mut source = db_storage::DbIoExternalBytes::new(source);
        if let Err(error) = source.capacity().and_then(|capacity| reservation.observe_capacity(capacity)) {
            return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
        }
        let mut offset = 0;
        while offset < source.as_slice().map_err(|error| WalBytesRejected { source: None, writer: None, error })?.len() {
            if let Err(error) = control.grant() {
                return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
            }
            match source.as_slice().and_then(|source| writer.write_fragment(&source[offset..])) {
                Ok(written) => offset += written,
                Err(error) => return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error }),
            }
            semio_framework_async::yield_once().await;
        }
        while !source.terminal_is_empty() {
            if let Err(error) = control.grant() {
                return Err(WalBytesRejected { source: None, writer: Some(writer), error });
            }
            let _ = source.close_step();
            semio_framework_async::yield_once().await;
        }
        if let Err(error) = reservation.close_step() {
            return Err(WalBytesRejected { source: None, writer: Some(writer), error });
        }
        writer.seal_retained().await.map(|pages| Self { pages }).map_err(|rejected| {
            let (error, writer) = rejected.into_parts();
            WalBytesRejected { source: None, writer, error }
        })
    }

    async fn copy_for_operation(operation: u64, source: &[u8], control: &mut WalCursorControl) -> Result<Self, DbError> {
        let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(operation, source.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        let mut offset = 0;
        while offset < source.len() {
            control.grant()?;
            offset += writer.write_fragment(&source[offset..])?;
            semio_framework_async::yield_once().await;
        }
        writer.seal_retained().await.map(|pages| Self { pages }).map_err(db_storage::DbIoPageWriterRejected::into_error)
    }

    pub fn operation(&self) -> u64 {
        self.pages.operation()
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn fragments(&self) -> db_storage::DbIoPageReader<'_> {
        self.pages.fragments()
    }

    pub fn cursor(&self) -> WalBytesCursor<'_> {
        WalBytesCursor { bytes: self, offset: 0 }
    }

    #[cfg(test)]
    async fn prepare_platform(&self) -> Result<db_storage::DbIoPlatformBuffer, DbError> {
        db_storage::db_io_prepare_platform(&self.pages)?.await
    }

    pub async fn hash(&self) -> [u8; 32] {
        db_storage::db_io_hash_pages(&self.pages).await.0
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        self.pages.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.terminal_is_empty()
    }
}

impl<'bytes> WalBytesCursor<'bytes> {
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }

    pub fn byte(&mut self, control: &mut WalCursorControl) -> Result<u8, DbError> {
        control.grant()?;
        let page = (self.offset / db_storage::DB_IO_PAGE_BYTES) as u8;
        let page_offset = self.offset % db_storage::DB_IO_PAGE_BYTES;
        let byte = self.bytes.pages.page(page).and_then(|fragment| fragment.get(page_offset)).copied().ok_or_else(|| DbError::Corrupt("wal retained field ended early".to_string()))?;
        self.offset += 1;
        Ok(byte)
    }

    pub fn varint(&mut self, control: &mut WalCursorControl) -> Result<u64, DbError> {
        let start = self.offset;
        let result = wal_read_canonical_varint(|| self.byte(control));
        if result.is_err() { self.offset = start; }
        result
    }

    pub fn begin_field(&mut self, maximum: u64, control: &mut WalCursorControl) -> Result<usize, DbError> {
        let len = self.varint(control)?;
        check_len(len, maximum, "wal retained field")?;
        let len = len as usize;
        if len > self.remaining() {
            return Err(DbError::Corrupt("wal retained field length exceeds record".to_string()));
        }
        Ok(len)
    }

    pub fn read_field_fragment(&mut self, remaining: &mut usize, output: &mut [u8], control: &mut WalCursorControl) -> Result<usize, DbError> {
        if *remaining == 0 || output.is_empty() {
            return Ok(0);
        }
        control.grant()?;
        let page = (self.offset / db_storage::DB_IO_PAGE_BYTES) as u8;
        let page_offset = self.offset % db_storage::DB_IO_PAGE_BYTES;
        let fragment = self.bytes.pages.page(page).ok_or_else(|| DbError::Corrupt("wal retained field ended early".to_string()))?;
        let copied = (*remaining).min(output.len()).min(fragment.len().saturating_sub(page_offset));
        if copied == 0 {
            return Err(DbError::Corrupt("wal retained field cursor stalled".to_string()));
        }
        output[..copied].copy_from_slice(&fragment[page_offset..page_offset + copied]);
        self.offset += copied;
        *remaining -= copied;
        Ok(copied)
    }

    pub fn text(&mut self, maximum: u64, control: &mut WalCursorControl) -> Result<String, DbError> {
        let mut remaining = self.begin_field(maximum, control)?;
        let mut output = Vec::with_capacity(remaining);
        let mut fragment = [0u8; 1024];
        while remaining != 0 {
            let copied = self.read_field_fragment(&mut remaining, &mut fragment, control)?;
            output.extend_from_slice(&fragment[..copied]);
        }
        String::from_utf8(output).map_err(|_| DbError::Corrupt("wal retained text is not valid utf-8".to_string()))
    }
}

impl WalBytesRejected {
    pub fn source(&self) -> Option<&Vec<u8>> {
        self.source.as_ref()
    }

    pub fn into_source(mut self) -> Option<Vec<u8>> {
        self.source.take()
    }

    pub fn error(&self) -> &DbError {
        &self.error
    }

    pub fn into_error(mut self) -> DbError {
        std::mem::replace(&mut self.error, DbError::Closed)
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

impl Drop for WalBytesRejected {
    fn drop(&mut self) {
        if let Some(source) = self.source.take() {
            drop(db_storage::DbIoExternalBytes::new(source));
        }
    }
}

/// @emoji 🫙️ `WAL_PAYLOAD`'s two shapes: small payloads inline, large ones by CAS reference into
/// `db_storage::PayloadStorage` — mirrors that trait's own blake3-CAS design.
pub enum WalPayloadRef {
    Inline(WalBytes),
    CasRef(ContentHash),
}

/// @emoji 📜️ One decoded WAL record — the typed shape every `WAL_*` kind decodes to/encodes from.
/// `Command`/`Diff`/`Inverse`/`Event`/`Outbox`/`Migration` carry opaque bytes verbatim (per the
/// contract, no db crate below `db_artifact` interprets operation semantics); the rest are
/// structured since this crate itself owns their meaning (transaction boundaries, segment
/// chaining, frontiers, leases).
pub enum WalRecord {
    SegmentHeader { document: ArtifactId, segment_index: u64, prev_chain_hash: Option<[u8; 32]> },
    TxBegin { tx_id: u64 },
    TxCommit { tx_id: u64, record_count: u32 },
    TxAbort { tx_id: u64 },
    Command(WalBytes),
    Payload(WalPayloadRef),
    Diff(WalBytes),
    Inverse(WalBytes),
    Event(WalBytes),
    Outbox(WalBytes),
    Frontier(Frontier),
    VcsRef(db_storage::DbIoText),
    SnapshotPub { generation: u64, frontier: Frontier },
    IndexCkpt { run_ids: db_storage::DbIoU64List },
    Lease { resource: db_storage::DbIoText, holder: db_storage::DbIoText, fence: u64, expires_at_ms: u64 },
    Migration(WalBytes),
}

pub struct WalRecordBatch {
    records: [Option<WalRecord>; 64],
    len: u8,
}

impl WalRecordBatch {
    pub fn new() -> Self {
        Self { records: std::array::from_fn(|_| None), len: 0 }
    }

    pub fn push(&mut self, record: WalRecord) -> Result<(), WalRecord> {
        let Some(slot) = self.records.get_mut(self.len as usize) else { return Err(record) };
        *slot = Some(record);
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &WalRecord> {
        self.records[..self.len as usize].iter().flatten()
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len as usize - 1;
        let record = self.records[index].as_mut().ok_or_else(|| DbError::Internal("WAL batch close lost retained record".to_string()))?;
        if record.close_step()? {
            return Ok(true);
        }
        self.records[index] = None;
        self.len -= 1;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.records.iter().all(Option::is_none)
    }
}

impl Default for WalRecordBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl WalRecord {
    /// @emoji 🔢️ The transaction id carried by a `TxBegin`/`TxCommit`/`TxAbort` record, or `None`
    /// for every other kind — used by `ArtifactWal::open` to resume `next_tx_id` past whatever was
    /// already durable.
    fn tx_id(&self) -> Option<u64> {
        match self {
            WalRecord::TxBegin { tx_id } | WalRecord::TxCommit { tx_id, .. } | WalRecord::TxAbort { tx_id } => Some(*tx_id),
            _ => None,
        }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        match self {
            Self::Command(bytes) | Self::Diff(bytes) | Self::Inverse(bytes) | Self::Event(bytes) | Self::Outbox(bytes) | Self::Migration(bytes) | Self::Payload(WalPayloadRef::Inline(bytes)) => Ok(bytes.close_step()?.is_some()),
            Self::VcsRef(text) => Ok(text.close_step()),
            Self::IndexCkpt { run_ids } => Ok(run_ids.close_step()),
            Self::Lease { resource, holder, .. } => {
                if resource.close_step() {
                    return Ok(true);
                }
                Ok(holder.close_step())
            }
            _ => Ok(false),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Command(bytes) | Self::Diff(bytes) | Self::Inverse(bytes) | Self::Event(bytes) | Self::Outbox(bytes) | Self::Migration(bytes) | Self::Payload(WalPayloadRef::Inline(bytes)) => bytes.terminal_is_empty(),
            Self::VcsRef(text) => text.terminal_is_empty(),
            Self::IndexCkpt { run_ids } => run_ids.terminal_is_empty(),
            Self::Lease { resource, holder, .. } => resource.terminal_is_empty() && holder.terminal_is_empty(),
            _ => true,
        }
    }

    /// @emoji ✍️ Encodes `self` to its on-disk `(kind, critical, payload)` triple, ready for
    /// `protocol::SprWriter::write_record`. Every kind is critical: unlike protocol's own
    /// history-log records (where e.g. a dictionary delta can plausibly be "skippable" to some
    /// future reader), every `WAL_*` record is load-bearing for correct replay — there is no
    /// optional WAL record in this crate's design.
    #[cfg(test)]
    pub async fn encode(&self) -> (u8, bool, Vec<u8>) {
        let mut writer = pack::ByteWriter::new();
        let kind = match self {
            WalRecord::SegmentHeader { document, segment_index, prev_chain_hash } => {
                write_field(&mut writer, document.0.as_bytes()).await;
                writer.write_u64_le(*segment_index);
                match prev_chain_hash {
                    Some(hash) => {
                        writer.write_u8(1);
                        writer.write_bytes(hash);
                    }
                    None => writer.write_u8(0),
                }
                WAL_SEGMENT_HEADER
            }
            WalRecord::TxBegin { tx_id } => {
                writer.write_u64_le(*tx_id);
                WAL_TX_BEGIN
            }
            WalRecord::TxCommit { tx_id, record_count } => {
                writer.write_u64_le(*tx_id);
                writer.write_u32_le(*record_count);
                WAL_TX_COMMIT
            }
            WalRecord::TxAbort { tx_id } => {
                writer.write_u64_le(*tx_id);
                WAL_TX_ABORT
            }
            WalRecord::Command(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_COMMAND
            }
            WalRecord::Payload(WalPayloadRef::Inline(bytes)) => {
                writer.write_u8(0);
                writer.write_varint_u64(bytes.len() as u64);
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_PAYLOAD
            }
            WalRecord::Payload(WalPayloadRef::CasRef(hash)) => {
                writer.write_u8(1);
                writer.write_bytes(&hash.0);
                WAL_PAYLOAD
            }
            WalRecord::Diff(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_DIFF
            }
            WalRecord::Inverse(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_INVERSE
            }
            WalRecord::Event(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_EVENT
            }
            WalRecord::Outbox(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_OUTBOX
            }
            WalRecord::Frontier(frontier) => {
                encode_frontier(&mut writer, frontier).await;
                WAL_FRONTIER
            }
            WalRecord::VcsRef(id) => {
                write_field(&mut writer, id.as_str().as_bytes()).await;
                WAL_VCS_REF
            }
            WalRecord::SnapshotPub { generation, frontier } => {
                writer.write_u64_le(*generation);
                encode_frontier(&mut writer, frontier).await;
                WAL_SNAPSHOT_PUB
            }
            WalRecord::IndexCkpt { run_ids } => {
                writer.write_varint_u64(run_ids.len() as u64);
                for run_id in run_ids.as_slice() {
                    writer.write_u64_le(*run_id);
                }
                WAL_INDEX_CKPT
            }
            WalRecord::Lease { resource, holder, fence, expires_at_ms } => {
                write_field(&mut writer, resource.as_str().as_bytes()).await;
                write_field(&mut writer, holder.as_str().as_bytes()).await;
                writer.write_u64_le(*fence);
                writer.write_u64_le(*expires_at_ms);
                WAL_LEASE
            }
            WalRecord::Migration(bytes) => {
                for fragment in bytes.fragments() {
                    writer.write_bytes(fragment);
                }
                WAL_MIGRATION
            }
        };
        (kind, true, writer.into_bytes())
    }

    fn retained_shape(&self) -> (u8, usize) {
        match self {
            Self::SegmentHeader { document, prev_chain_hash, .. } => (WAL_SEGMENT_HEADER, wal_field_len(document.0.as_bytes()) + 8 + 1 + prev_chain_hash.map_or(0, |_| 32)),
            Self::TxBegin { .. } => (WAL_TX_BEGIN, 8),
            Self::TxCommit { .. } => (WAL_TX_COMMIT, 12),
            Self::TxAbort { .. } => (WAL_TX_ABORT, 8),
            Self::Command(bytes) => (WAL_COMMAND, bytes.len()),
            Self::Payload(WalPayloadRef::Inline(bytes)) => (WAL_PAYLOAD, 1 + wal_varint_len(bytes.len() as u64) + bytes.len()),
            Self::Payload(WalPayloadRef::CasRef(_)) => (WAL_PAYLOAD, 33),
            Self::Diff(bytes) => (WAL_DIFF, bytes.len()),
            Self::Inverse(bytes) => (WAL_INVERSE, bytes.len()),
            Self::Event(bytes) => (WAL_EVENT, bytes.len()),
            Self::Outbox(bytes) => (WAL_OUTBOX, bytes.len()),
            Self::Frontier(frontier) => (WAL_FRONTIER, wal_frontier_len(frontier)),
            Self::VcsRef(id) => (WAL_VCS_REF, wal_field_len(id.as_str().as_bytes())),
            Self::SnapshotPub { frontier, .. } => (WAL_SNAPSHOT_PUB, 8 + wal_frontier_len(frontier)),
            Self::IndexCkpt { run_ids } => (WAL_INDEX_CKPT, wal_varint_len(run_ids.len() as u64) + run_ids.len() * 8),
            Self::Lease { resource, holder, .. } => (WAL_LEASE, wal_field_len(resource.as_str().as_bytes()) + wal_field_len(holder.as_str().as_bytes()) + 16),
            Self::Migration(bytes) => (WAL_MIGRATION, bytes.len()),
        }
    }

    async fn write_retained(&self, writer: &mut protocol::SprWriter<SharedBuf>) -> Result<u64, DbError> {
        let (kind, payload_len) = self.retained_shape();
        let mut record = writer.begin_identity_record(kind, true, payload_len).await.map_err(protocol_err)?;
        match self {
            Self::SegmentHeader { document, segment_index, prev_chain_hash } => {
                wal_record_write_field(&mut record, document.0.as_bytes()).await?;
                wal_record_write(&mut record, &segment_index.to_le_bytes()).await?;
                wal_record_write(&mut record, &[u8::from(prev_chain_hash.is_some())]).await?;
                if let Some(hash) = prev_chain_hash {
                    wal_record_write(&mut record, hash).await?;
                }
            }
            Self::TxBegin { tx_id } | Self::TxAbort { tx_id } => wal_record_write(&mut record, &tx_id.to_le_bytes()).await?,
            Self::TxCommit { tx_id, record_count } => {
                wal_record_write(&mut record, &tx_id.to_le_bytes()).await?;
                wal_record_write(&mut record, &record_count.to_le_bytes()).await?;
            }
            Self::Command(bytes) | Self::Diff(bytes) | Self::Inverse(bytes) | Self::Event(bytes) | Self::Outbox(bytes) | Self::Migration(bytes) => {
                for fragment in bytes.fragments() {
                    wal_record_write(&mut record, fragment).await?;
                }
            }
            Self::Payload(WalPayloadRef::Inline(bytes)) => {
                wal_record_write(&mut record, &[0]).await?;
                let mut length = [0u8; 10];
                wal_record_write(&mut record, wal_varint(bytes.len() as u64, &mut length)).await?;
                for fragment in bytes.fragments() {
                    wal_record_write(&mut record, fragment).await?;
                }
            }
            Self::Payload(WalPayloadRef::CasRef(hash)) => {
                wal_record_write(&mut record, &[1]).await?;
                wal_record_write(&mut record, &hash.0).await?;
            }
            Self::Frontier(frontier) => wal_record_write_frontier(&mut record, frontier).await?,
            Self::VcsRef(id) => wal_record_write_field(&mut record, id.as_str().as_bytes()).await?,
            Self::SnapshotPub { generation, frontier } => {
                wal_record_write(&mut record, &generation.to_le_bytes()).await?;
                wal_record_write_frontier(&mut record, frontier).await?;
            }
            Self::IndexCkpt { run_ids } => {
                let mut count = [0u8; 10];
                wal_record_write(&mut record, wal_varint(run_ids.len() as u64, &mut count)).await?;
                for run_id in run_ids.as_slice() {
                    wal_record_write(&mut record, &run_id.to_le_bytes()).await?;
                }
            }
            Self::Lease { resource, holder, fence, expires_at_ms } => {
                wal_record_write_field(&mut record, resource.as_str().as_bytes()).await?;
                wal_record_write_field(&mut record, holder.as_str().as_bytes()).await?;
                wal_record_write(&mut record, &fence.to_le_bytes()).await?;
                wal_record_write(&mut record, &expires_at_ms.to_le_bytes()).await?;
            }
        }
        record.finish().await.map_err(protocol_err)
    }

    /// @emoji 📖️ Inverse of `encode`. Errors `DbError::Corrupt` on an unrecognized `kind` (a
    /// genuinely corrupt or future-version record) rather than silently dropping it — every
    /// `WAL_*` kind is critical (see `encode`'s doc).
    #[cfg(test)]
    async fn decode_retained(operation: u64, kind: u8, payload: &[u8], control: &mut WalCursorControl) -> Result<WalRecord, DbError> {
        control.grant()?;
        let mut reader = pack::ByteReader::new(payload);
        let record = match kind {
            WAL_SEGMENT_HEADER => {
                let document = ArtifactId(read_field_string(&mut reader).await?);
                let segment_index = reader.read_u64_le()?;
                let prev_chain_hash = if reader.read_u8()? == 1 { Some(reader.read_array32()?) } else { None };
                WalRecord::SegmentHeader { document, segment_index, prev_chain_hash }
            }
            WAL_TX_BEGIN => WalRecord::TxBegin { tx_id: reader.read_u64_le()? },
            WAL_TX_COMMIT => WalRecord::TxCommit { tx_id: reader.read_u64_le()?, record_count: reader.read_u32_le()? },
            WAL_TX_ABORT => WalRecord::TxAbort { tx_id: reader.read_u64_le()? },
            WAL_COMMAND => WalRecord::Command(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_PAYLOAD => match reader.read_u8()? {
                0 => {
                    let len = reader.read_varint_u64()?;
                    check_len(len, MAX_FIELD_BYTES, "wal_record::payload")?;
                    WalRecord::Payload(WalPayloadRef::Inline(WalBytes::copy_for_operation(operation, reader.read_bytes(len as usize)?, control).await?))
                }
                1 => WalRecord::Payload(WalPayloadRef::CasRef(ContentHash(reader.read_array32()?))),
                other => return Err(DbError::Corrupt(format!("unknown wal payload tag {other}"))),
            },
            WAL_DIFF => WalRecord::Diff(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_INVERSE => WalRecord::Inverse(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_EVENT => WalRecord::Event(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_OUTBOX => WalRecord::Outbox(WalBytes::copy_for_operation(operation, payload, control).await?),
            WAL_FRONTIER => WalRecord::Frontier(decode_frontier(&mut reader).await?),
            WAL_VCS_REF => WalRecord::VcsRef(db_storage::DbIoText::try_from_str(&read_field_string(&mut reader).await?)?),
            WAL_SNAPSHOT_PUB => {
                let generation = reader.read_u64_le()?;
                WalRecord::SnapshotPub { generation, frontier: decode_frontier(&mut reader).await? }
            }
            WAL_INDEX_CKPT => {
                let count = reader.read_varint_u64()?;
                check_len(count, MAX_RUN_IDS, "wal_record::index_ckpt run_ids")?;
                let mut run_ids = db_storage::DbIoU64List::new();
                for _ in 0..count {
                    control.grant()?;
                    run_ids.push(reader.read_u64_le()?)?;
                }
                WalRecord::IndexCkpt { run_ids }
            }
            WAL_LEASE => {
                let resource = db_storage::DbIoText::try_from_str(&read_field_string(&mut reader).await?)?;
                let holder = db_storage::DbIoText::try_from_str(&read_field_string(&mut reader).await?)?;
                let fence = reader.read_u64_le()?;
                let expires_at_ms = reader.read_u64_le()?;
                WalRecord::Lease { resource, holder, fence, expires_at_ms }
            }
            WAL_MIGRATION => WalRecord::Migration(WalBytes::copy_for_operation(operation, payload, control).await?),
            other => return Err(DbError::Corrupt(format!("unknown wal record kind {other:#x}"))),
        };
        Ok(record)
    }
}
//#endregion 🔖️Records

//#region 🔖️PayloadTransform
/// @emoji 🔐️ The encryption hook for `WAL_PAYLOAD` bytes: a caller building a `WalPayloadRef`
/// applies `encrypt` to the plaintext BEFORE wrapping it as `WalPayloadRef::Inline` or handing it to
/// `db_storage::PayloadStorage::put` for a `WalPayloadRef::CasRef`, and applies `decrypt` after
/// reading either form back — this crate itself never calls `PayloadStorage` (see `WalPayloadRef`'s
/// doc: the inline-vs-CAS choice is deliberately the caller's, keeping `db_wal` decoupled from that
/// trait), so it cannot invoke the hook itself either; it only defines the seam. Per the contract:
/// a trait only, no real implementation here — external crypto libraries stay behind it (the
/// family's "external libs behind an interface" rule).
pub trait PayloadTransform: Send + Sync {
    /// @emoji 🔒️ Transforms `plaintext` before it is embedded inline or stored via `PayloadStorage`.
    async fn encrypt(&self, plaintext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError>;
    /// @emoji 🔓️ Inverts `encrypt` — must exactly reconstruct the original bytes.
    async fn decrypt(&self, ciphertext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError>;
}

/// @emoji 🪟️ A `PayloadTransform` that passes bytes through unchanged — the default for a
/// deployment with no encryption configured.
#[derive(Clone, Copy, Default, Debug)]
pub struct IdentityPayloadTransform;

impl PayloadTransform for IdentityPayloadTransform {
    async fn encrypt(&self, plaintext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
        control.grant()?;
        Ok(plaintext)
    }

    async fn decrypt(&self, ciphertext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
        control.grant()?;
        Ok(ciphertext)
    }
}
//#endregion 🔖️PayloadTransform

//#region 🔖️GroupCommit
/// @emoji ⏱️ Bounds a segment's group-commit batching: whichever of delay/bytes/records is hit
/// first triggers the next physical `SprWriter::commit()` + `WalStorage::sync`. This crate's own
/// choice of defaults (the contract fixes the mechanism, not the numbers) — 20ms/256KiB/256
/// records amortizes fsync cost under load while keeping worst-case latency low for a single
/// isolated command.
#[derive(Clone, Copy, Debug)]
pub struct GroupCommitPolicy {
    pub max_delay_ms: u64,
    pub max_bytes: u64,
    pub max_records: u32,
}

impl Default for GroupCommitPolicy {
    fn default() -> Self {
        Self { max_delay_ms: 20, max_bytes: 256 * 1024, max_records: 256 }
    }
}

impl GroupCommitPolicy {
    /// @emoji ⏰️ True iff any bound is currently exceeded — `ArtifactWal::submit` also always
    /// commits immediately regardless of this policy when the caller requests `Fsync`/`Quorum`
    /// durability (a durability request stronger than what's already durable can never be
    /// satisfied by deferring the commit).
    fn is_due(&self, pending_bytes: u64, pending_records: u32, oldest_pending_at_ms: Option<u64>, now_ms: u64) -> bool {
        pending_bytes >= self.max_bytes || pending_records >= self.max_records || oldest_pending_at_ms.is_some_and(|started| now_ms.saturating_sub(started) >= self.max_delay_ms)
    }
}
//#endregion 🔖️GroupCommit

//#region 🔖️Sink
/// @emoji 🪞️ A `pack::PackSink` over shared fixed DB I/O pages. `protocol::SprWriter`
/// owns one retained-writer control; `SegmentWriter` retains another to flush the unflushed suffix to
/// `db_storage::WalStorage` — `SprWriter` has no public accessor for its private `sink` field, and
/// holding the buffer open for a segment's lifetime also lets recovery retain the exact verified
/// prefix while `SprWriter::resume_verified` continues its original commit chain.
#[derive(Clone)]
struct SharedBuf(std::sync::Arc<std::sync::Mutex<db_storage::DbIoPageWriter>>);

/// @emoji 🩹️ Recovers a poisoned lock instead of panicking — one panicking document actor must
/// never turn every other document's WAL access into a cascading panic (mirrors `db_storage`'s
/// own `MemoryStorage` convention).
// 🚫️async: E1 pure accessor (no suspension: `Mutex::lock` on a never-genuinely-contended
// in-process lock), consumed synchronously alongside `.await`ed uses elsewhere — see R9
fn lock<T>(mutex: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl SharedBuf {
    // 🚫️async: E1 pure constructor — see `lock`
    fn try_new() -> Result<Self, DbError> {
        let writer = db_storage::DbIoPageWriter::try_reserve(db_storage::DB_IO_OPERATION_PAGES).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        Ok(Self(std::sync::Arc::new(std::sync::Mutex::new(writer))))
    }

    // 🚫️async: E1 pure accessor — see `lock`
    fn len(&self) -> u64 {
        lock(&self.0).len() as u64
    }

    fn close_step(&mut self) -> Result<bool, DbError> {
        if std::sync::Arc::strong_count(&self.0) != 1 {
            return Err(DbError::Internal("WAL retained buffer still has a writer clone".to_string()));
        }
        Ok(lock(&self.0).close_step()?.is_some())
    }

    fn terminal_is_empty(&self) -> bool {
        std::sync::Arc::strong_count(&self.0) == 1 && lock(&self.0).terminal_is_empty()
    }

    async fn copy_range(&self, offset: usize, len: usize) -> Result<db_storage::DbIoPages, DbError> {
        let mut output = db_storage::DbIoPageWriter::try_reserve(len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        let mut copied = 0;
        let mut fragment = [0u8; db_storage::DB_IO_PAGE_BYTES];
        while copied < len {
            let requested = (len - copied).min(fragment.len());
            let read = lock(&self.0).read_fragment(offset + copied, &mut fragment[..requested])?;
            if read == 0 {
                return Err(DbError::Corrupt("WAL retained page range ended early".to_string()));
            }
            let mut written = 0;
            while written < read {
                let count = output.write_fragment(&fragment[written..read])?;
                if count == 0 { return Err(DbError::LimitExceeded("WAL retained suffix writer")); }
                written += count;
                semio_framework_async::yield_once().await;
            }
            copied += read;
            semio_framework_async::yield_once().await;
        }
        output.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
    }

    async fn read_exact(&self, offset: usize, output: &mut [u8]) -> Result<(), DbError> {
        let mut copied = 0;
        while copied < output.len() {
            let read = lock(&self.0).read_fragment(offset + copied, &mut output[copied..])?;
            if read == 0 {
                return Err(DbError::Corrupt("WAL retained page read ended early".to_string()));
            }
            copied += read;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }
}

impl pack::PackSink for SharedBuf {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), pack::PackError> {
        let mut cursor = 0;
        while cursor < bytes.len() {
            cursor += lock(&self.0).write_fragment(&bytes[cursor..]).map_err(|error| pack::PackError::Io(error.to_string()))?;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }

    async fn position(&self) -> u64 {
        self.len()
    }
}

/// @emoji ⛓️ Every `db_wal` segment is opened requiring `protocol::wire::REQUIRED_HASH_CHAIN` —
/// `SprWriter` always computes the commit chain regardless of this flag, but stamping it into the
/// header makes the requirement an explicit, reader-enforced part of the file's contract rather
/// than an implicit convention only this crate happens to uphold.
// 🚫️async: E1 pure constructor, always used by reference inline (`&segment_write_options()`) — see R9
fn segment_write_options() -> protocol::format::WriteOptions {
    protocol::format::WriteOptions { required_flags: protocol::wire::REQUIRED_HASH_CHAIN, optional_flags: 0 }
}
//#endregion 🔖️Sink

//#region 🔖️Recovery
/// @emoji 🚨️ Maps `protocol::ProtocolError` (this crate never leaks it in a public signature, per
/// the family's `DbError`-only rule) onto the closest `DbError` variant.
// 🚫️async: E4 fn-pointer slot
fn protocol_err(err: protocol::ProtocolError) -> DbError {
    match err {
        protocol::ProtocolError::Pack(pack_error) => DbError::from(pack_error),
        protocol::ProtocolError::Io(message) => DbError::Io(message),
        protocol::ProtocolError::LimitExceeded(what) => DbError::LimitExceeded(what),
        other => DbError::Corrupt(other.to_string()),
    }
}

struct WalPageSource<'pages>(&'pages db_storage::DbIoPages);

impl protocol::PackSource for WalPageSource<'_> {
    async fn len(&self) -> u64 {
        self.0.len() as u64
    }

    async fn read_at(&self, offset: u64, output: &mut [u8]) -> Result<usize, protocol::codec::PackError> {
        let offset = usize::try_from(offset).map_err(|_| protocol::codec::PackError::Truncated(offset))?;
        if offset > self.0.len() {
            return Err(protocol::codec::PackError::Truncated(offset as u64));
        }
        let mut base = 0usize;
        let mut written = 0usize;
        for fragment in self.0.fragments() {
            let end = base + fragment.len();
            if end <= offset {
                base = end;
                continue;
            }
            let start = offset.saturating_sub(base);
            let count = (output.len() - written).min(fragment.len() - start);
            output[written..written + count].copy_from_slice(&fragment[start..start + count]);
            written += count;
            base = end;
            if written == output.len() {
                break;
            }
        }
        Ok(written)
    }
}

pub(crate) trait WalImmutableByteSource: Sync {
    fn byte_len(&self) -> usize;
    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError>;
}

impl<S: WalImmutableByteSource + ?Sized> WalImmutableByteSource for &S {
    fn byte_len(&self) -> usize { (**self).byte_len() }
    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> { (**self).fragment_at(offset, limit) }
}

impl WalImmutableByteSource for db_storage::DbIoPages {
    fn byte_len(&self) -> usize { self.len() }

    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
        if offset >= limit || limit > self.len() { return Err(DbError::Corrupt("wal immutable source range".to_string())); }
        let first = self.page(0).ok_or_else(|| DbError::Corrupt("wal immutable source lost first page".to_string()))?.len();
        let (page, local) = if offset < first { (0, offset) } else { let rest = offset - first; (1 + rest / db_storage::DB_IO_PAGE_BYTES, rest % db_storage::DB_IO_PAGE_BYTES) };
        let bytes = self.page(u8::try_from(page).map_err(|_| DbError::LimitExceeded("wal immutable source page"))?).ok_or_else(|| DbError::Corrupt("wal immutable source lost page".to_string()))?;
        let remaining = bytes.len().checked_sub(local).ok_or_else(|| DbError::Corrupt("wal immutable source offset".to_string()))?;
        let count = remaining.min(limit - offset);
        if count == 0 { return Err(DbError::Corrupt("wal immutable source empty fragment".to_string())); }
        Ok(&bytes[local..local + count])
    }
}

struct WalPageReader<'pages> {
    pages: &'pages dyn WalImmutableByteSource,
    position: usize,
    limit: usize,
}

impl<'pages> WalPageReader<'pages> {
    fn new(pages: &'pages dyn WalImmutableByteSource, position: usize, limit: usize) -> Result<Self, DbError> {
        if position > limit || limit > pages.byte_len() {
            return Err(DbError::Corrupt("wal retained reader authority".to_string()));
        }
        Ok(Self { pages, position, limit })
    }

    fn fragment(&self) -> Result<&'pages [u8], DbError> {
        self.pages.fragment_at(self.position, self.limit)
    }

    fn byte(&mut self) -> Result<u8, DbError> {
        let byte = self.fragment()?[0];
        self.position += 1;
        Ok(byte)
    }

    fn varint(&mut self) -> Result<u64, DbError> {
        let start = self.position;
        let result = wal_read_canonical_varint(|| self.byte());
        if result.is_err() { self.position = start; }
        result
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

    fn text(&mut self) -> Result<db_storage::DbIoText, DbError> {
        let len = self.varint()?;
        check_len(len, db_storage::DbIoText::maximum_capacity() as u64, "wal retained text")?;
        let mut bytes = [0u8; 1024];
        let mut written = 0usize;
        while written < len as usize {
            let fragment = self.fragment()?;
            let count = (len as usize - written).min(fragment.len());
            bytes[written..written + count].copy_from_slice(&fragment[..count]);
            self.position += count;
            written += count;
        }
        let value = std::str::from_utf8(&bytes[..written]).map_err(|_| DbError::Corrupt("wal retained text is not utf-8".to_string()))?;
        db_storage::DbIoText::try_from_str(value)
    }

    fn string(&mut self) -> Result<String, DbError> {
        let mut text = self.text()?;
        let output = text.as_str().to_string();
        text.close_step();
        Ok(output)
    }

}

fn wal_crc_range(pages: &db_storage::DbIoPages, start: usize, len: usize, control: &mut WalCursorControl) -> Result<u32, DbError> {
    let end = start.checked_add(len).ok_or(DbError::LimitExceeded("wal frame crc range"))?;
    if end > pages.len() {
        return Err(DbError::Corrupt("wal frame crc exceeds segment".to_string()));
    }
    let mut crc = protocol::codec::Crc32cCursor::new();
    let mut base = 0usize;
    for fragment in pages.fragments() {
        let fragment_end = base + fragment.len();
        if fragment_end <= start {
            base = fragment_end;
            continue;
        }
        if base >= end {
            break;
        }
        control.grant()?;
        let local_start = start.saturating_sub(base);
        let local_end = fragment.len().min(end - base);
        crc.update_page(&fragment[local_start..local_end]);
        base = fragment_end;
    }
    Ok(crc.finish())
}

fn wal_decode_frontier(reader: &mut WalPageReader<'_>) -> Result<Frontier, DbError> {
    let document = ArtifactId(reader.string()?);
    let head_seq = u64::from_le_bytes(reader.array()?);
    let commit_seq = u64::from_le_bytes(reader.array()?);
    let chain_hash = reader.array()?;
    let epoch = u64::from_le_bytes(reader.array()?);
    Ok(Frontier { document, head_seq, commit_seq, chain_hash, epoch })
}

fn wal_decode_scalar_record(kind: u8, reader: &mut WalPageReader<'_>) -> Result<WalRecord, DbError> {
    let record = match kind {
        WAL_SEGMENT_HEADER => {
            let document = ArtifactId(reader.string()?);
            let segment_index = u64::from_le_bytes(reader.array()?);
            let prev_chain_hash = match reader.byte()? {
                0 => None,
                1 => Some(reader.array()?),
                _ => return Err(DbError::Corrupt("wal segment hash tag".to_string())),
            };
            WalRecord::SegmentHeader { document, segment_index, prev_chain_hash }
        }
        WAL_TX_BEGIN => WalRecord::TxBegin { tx_id: u64::from_le_bytes(reader.array()?) },
        WAL_TX_COMMIT => WalRecord::TxCommit { tx_id: u64::from_le_bytes(reader.array()?), record_count: u32::from_le_bytes(reader.array()?) },
        WAL_TX_ABORT => WalRecord::TxAbort { tx_id: u64::from_le_bytes(reader.array()?) },
        WAL_PAYLOAD => match reader.byte()? {
            1 => WalRecord::Payload(WalPayloadRef::CasRef(ContentHash(reader.array()?))),
            _ => return Err(DbError::Corrupt("wal payload tag".to_string())),
        },
        WAL_FRONTIER => WalRecord::Frontier(wal_decode_frontier(reader)?),
        WAL_VCS_REF => WalRecord::VcsRef(reader.text()?),
        WAL_SNAPSHOT_PUB => WalRecord::SnapshotPub { generation: u64::from_le_bytes(reader.array()?), frontier: wal_decode_frontier(reader)? },
        WAL_LEASE => WalRecord::Lease { resource: reader.text()?, holder: reader.text()?, fence: u64::from_le_bytes(reader.array()?), expires_at_ms: u64::from_le_bytes(reader.array()?) },
        _ => return Err(DbError::Corrupt(format!("unknown wal record kind {kind:#x}"))),
    };
    if reader.position != reader.limit {
        return Err(DbError::Corrupt("wal record retained decoder left trailing payload".to_string()));
    }
    Ok(record)
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct WalRecordFrame {
    pub(crate) kind: u8,
    pub(crate) payload_start: usize,
    pub(crate) payload_end: usize,
    pub(crate) frame_end: usize,
}

enum WalRetainedDecodeState {
    Start,
    Bytes { position: usize, end: usize },
    Index { position: usize, remaining: usize },
    Done,
}

struct WalRetainedRecordDecoder {
    frame: WalRecordFrame,
    state: WalRetainedDecodeState,
    writer: Option<db_storage::DbIoPageWriter>,
    run_ids: Option<db_storage::DbIoU64List>,
}

impl WalRetainedRecordDecoder {
    fn new(frame: WalRecordFrame) -> Self {
        Self { frame, state: WalRetainedDecodeState::Start, writer: None, run_ids: None }
    }

    fn step(&mut self, pages: &db_storage::DbIoPages, control: &mut WalCursorControl) -> Result<Option<WalRecord>, DbError> {
        control.grant()?;
        match &mut self.state {
            WalRetainedDecodeState::Start => {
                let mut reader = WalPageReader::new(pages, self.frame.payload_start, self.frame.payload_end)?;
                if self.frame.kind == WAL_INDEX_CKPT {
                    let count = reader.varint()?;
                    check_len(count, 4_096, "wal retained checkpoint")?;
                    if reader.position.checked_add(count as usize * 8) != Some(reader.limit) { return Err(DbError::Corrupt("wal checkpoint payload length differs".to_string())); }
                    self.run_ids = Some(db_storage::DbIoU64List::new());
                    self.state = WalRetainedDecodeState::Index { position: reader.position, remaining: count as usize };
                    return Ok(None);
                }
                let opaque = matches!(self.frame.kind, WAL_COMMAND | WAL_DIFF | WAL_INVERSE | WAL_EVENT | WAL_OUTBOX | WAL_MIGRATION);
                let inline = if self.frame.kind == WAL_PAYLOAD {
                    match reader.byte()? {
                        0 => {
                            let len = reader.varint()?;
                            check_len(len, MAX_FIELD_BYTES, "wal retained payload")?;
                            if reader.position.checked_add(len as usize) != Some(reader.limit) { return Err(DbError::Corrupt("wal inline payload length differs".to_string())); }
                            true
                        }
                        1 => false,
                        _ => return Err(DbError::Corrupt("wal payload tag".to_string())),
                    }
                } else { false };
                if opaque || inline {
                    self.state = WalRetainedDecodeState::Bytes { position: reader.position, end: reader.limit };
                    match db_storage::DbIoPageWriter::try_reserve_for_operation(pages.operation(), (reader.limit - reader.position).div_ceil(db_storage::DB_IO_PAGE_BYTES)) {
                        Ok(writer) => self.writer = Some(writer),
                        Err(rejected) => { let (error, writer) = rejected.into_parts(); self.writer = writer; return Err(error); }
                    }
                    return Ok(None);
                }
                let mut reader = WalPageReader::new(pages, self.frame.payload_start, self.frame.payload_end)?;
                let record = wal_decode_scalar_record(self.frame.kind, &mut reader)?;
                self.state = WalRetainedDecodeState::Done;
                Ok(Some(record))
            }
            WalRetainedDecodeState::Bytes { position, end } => {
                let writer = self.writer.as_mut().ok_or_else(|| DbError::Internal("wal decoder lost byte writer".to_string()))?;
                if *position < *end {
                    let reader = WalPageReader::new(pages, *position, *end)?;
                    let fragment = reader.fragment()?;
                    let written = writer.write_fragment(fragment)?;
                    if written == 0 { return Err(DbError::LimitExceeded("wal retained decoder made no progress")); }
                    *position += written;
                    return Ok(None);
                }
                let Some(pages) = writer.seal_retained_step()? else { return Ok(None) };
                self.writer = None;
                self.state = WalRetainedDecodeState::Done;
                let bytes = WalBytes { pages };
                Ok(Some(match self.frame.kind {
                    WAL_COMMAND => WalRecord::Command(bytes), WAL_PAYLOAD => WalRecord::Payload(WalPayloadRef::Inline(bytes)),
                    WAL_DIFF => WalRecord::Diff(bytes), WAL_INVERSE => WalRecord::Inverse(bytes), WAL_EVENT => WalRecord::Event(bytes),
                    WAL_OUTBOX => WalRecord::Outbox(bytes), WAL_MIGRATION => WalRecord::Migration(bytes), _ => unreachable!(),
                }))
            }
            WalRetainedDecodeState::Index { position, remaining } => {
                if *remaining == 0 {
                    let run_ids = self.run_ids.take().ok_or_else(|| DbError::Internal("wal decoder lost checkpoint list".to_string()))?;
                    self.state = WalRetainedDecodeState::Done;
                    return Ok(Some(WalRecord::IndexCkpt { run_ids }));
                }
                let mut reader = WalPageReader::new(pages, *position, self.frame.payload_end)?;
                let value = u64::from_le_bytes(reader.array()?);
                self.run_ids.as_mut().ok_or_else(|| DbError::Internal("wal decoder lost checkpoint list".to_string()))?.push(value)?;
                *position = reader.position;
                *remaining -= 1;
                Ok(None)
            }
            WalRetainedDecodeState::Done => Err(DbError::Closed),
        }
    }

    fn close_owner_step(&mut self) -> Result<bool, DbError> {
        if let Some(writer) = self.writer.as_mut() { if writer.close_step()?.is_some() { return Ok(true); } }
        self.writer = None;
        if let Some(run_ids) = self.run_ids.as_mut() { if run_ids.close_step() { return Ok(true); } }
        self.run_ids = None;
        self.state = WalRetainedDecodeState::Done;
        Ok(false)
    }

    fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.run_ids.is_none() && matches!(self.state, WalRetainedDecodeState::Done)
    }
}

enum WalVerifiedFrameStep { Frame(WalRecordFrame), PhysicalCommit, Done }

/// 🧷️ Reads one span only after `WalSegmentChain` verified these exact immutable pages in full.
fn wal_next_verified_page_frame(pages: &dyn WalImmutableByteSource, offset: &mut usize, trusted_len: usize, control: &mut WalCursorControl) -> Result<WalVerifiedFrameStep, DbError> {
        control.grant()?;
        if *offset == trusted_len {
            return Ok(WalVerifiedFrameStep::Done);
        }
        let frame_start = *offset;
        let mut reader = WalPageReader::new(pages, frame_start, trusted_len)?;
        let body_len = reader.varint()?;
        check_len(body_len, protocol::ProtocolLimits::default().max_frame_len, "wal retained frame")?;
        let body_start = reader.position;
        let body_end = body_start.checked_add(body_len as usize).ok_or(DbError::LimitExceeded("wal frame body"))?;
        let frame_end = body_end.checked_add(8).ok_or(DbError::LimitExceeded("wal frame trailer"))?;
        if body_len < 2 || frame_end > trusted_len {
            return Err(DbError::Corrupt("wal frame exceeds trusted segment".to_string()));
        }
        let kind = reader.byte()?;
        let flags = reader.byte()?;
        if flags != protocol::wire::FRAME_FLAG_CRITICAL {
            return Err(DbError::Corrupt("db_wal requires exact critical frame flags".to_string()));
        }
        let payload_start = reader.position;
        let mut trailer = WalPageReader::new(pages, body_end, frame_end)?;
        let _verified_crc = trailer.array::<4>()?;
        let back_len = u32::from_le_bytes(trailer.array()?) as usize;
        if back_len != frame_end - frame_start {
            return Err(DbError::Corrupt("wal frame retained back length mismatch".to_string()));
        }
        if is_wal_record_kind(kind) {
            return Ok(WalVerifiedFrameStep::Frame(WalRecordFrame { kind, payload_start, payload_end: body_end, frame_end }));
        }
        if kind != protocol::wire::REC_COMMIT {
            return Err(DbError::Corrupt(format!("unexpected non-wal, non-commit frame kind {kind:#x} in a db_wal segment")));
        }
        *offset = frame_end;
        Ok(WalVerifiedFrameStep::PhysicalCommit)
}

pub(crate) struct WalTransactionGate {
    transaction_id: Option<u64>,
    ready: Option<u64>,
    frames: [Option<WalRecordFrame>; 64],
    frames_len: u8,
    next_tx_id: u64,
    header_seen: bool,
    transaction_seen: bool,
}

impl WalTransactionGate {
    pub(crate) fn new() -> Self {
        Self { transaction_id: None, ready: None, frames: [None; 64], frames_len: 0, next_tx_id: 1, header_seen: false, transaction_seen: false }
    }

    fn clear_frames(&mut self) {
        self.frames[..self.frames_len as usize].fill(None);
        self.frames_len = 0;
    }

    fn committed_frame(&self, index: usize) -> Option<WalRecordFrame> {
        self.ready?;
        self.frames.get(index).copied().flatten()
    }

    fn push(&mut self, pages: &dyn WalImmutableByteSource, frame: WalRecordFrame) -> Result<bool, DbError> {
        if self.ready.is_some() { return Err(DbError::Corrupt("wal committed transaction remains borrowed".to_string())); }
        if frame.kind == WAL_SEGMENT_HEADER {
            if self.header_seen || self.transaction_id.is_some() { return Err(DbError::Corrupt("wal logical segment header repeated".to_string())); }
            self.header_seen = true;
            return Ok(false);
        }
        if !self.header_seen { return Err(DbError::Corrupt("wal logical segment header missing".to_string())); }
        let mut reader = WalPageReader::new(pages, frame.payload_start, frame.payload_end)?;
        match frame.kind {
            WAL_TX_BEGIN => {
                let tx_id = u64::from_le_bytes(reader.array()?);
                if reader.position != reader.limit || self.transaction_id.is_some() || tx_id < self.next_tx_id || (self.transaction_seen && tx_id != self.next_tx_id) { return Err(DbError::Corrupt("wal transaction begin is nested or out of sequence".to_string())); }
                self.next_tx_id = tx_id.checked_add(1).ok_or(DbError::LimitExceeded("wal transaction sequence"))?;
                self.transaction_id = Some(tx_id);
                self.transaction_seen = true;
                Ok(false)
            }
            WAL_TX_COMMIT | WAL_TX_ABORT => {
                let tx_id = u64::from_le_bytes(reader.array()?);
                let count = if frame.kind == WAL_TX_COMMIT { Some(u32::from_le_bytes(reader.array()?)) } else { None };
                if reader.position != reader.limit || self.transaction_id != Some(tx_id) || count.is_some_and(|count| count != u32::from(self.frames_len)) {
                    return Err(DbError::Corrupt("wal logical terminal id or count differs".to_string()));
                }
                self.transaction_id = None;
                if count.is_some() { self.ready = Some(tx_id); Ok(true) } else { self.clear_frames(); Ok(false) }
            }
            WAL_COMMAND..=WAL_MIGRATION => {
                if self.transaction_id.is_none() { return Err(DbError::Corrupt("wal body outside a transaction".to_string())); }
                let slot = self.frames.get_mut(self.frames_len as usize).ok_or(DbError::LimitExceeded("wal transaction records"))?;
                *slot = Some(frame);
                self.frames_len += 1;
                Ok(false)
            }
            _ => Err(DbError::Corrupt("unknown wal logical record kind".to_string())),
        }
    }

    fn release(&mut self) -> Result<(), DbError> {
        self.ready.take().ok_or_else(|| DbError::Corrupt("wal has no committed transaction to finish".to_string()))?;
        self.clear_frames();
        Ok(())
    }

    fn finish_segment(&self, writable_highest: bool) -> Result<Option<u64>, DbError> {
        if !self.header_seen || self.ready.is_some() { return Err(DbError::Corrupt("wal logical segment is not drained".to_string())); }
        if self.transaction_id.is_some() && !writable_highest { return Err(DbError::Corrupt("wal logical transaction crosses a sealed boundary".to_string())); }
        Ok(self.transaction_id)
    }

    fn advance_segment(&mut self) -> Result<(), DbError> {
        self.finish_segment(false)?;
        self.header_seen = false;
        Ok(())
    }
}

/// @emoji 📋️ What `ArtifactWal::open` found while recovering a document's WAL.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalRecoveryReport {
    pub segments_seen: u64,
    pub records_replayed: u64,
    /// @emoji ✂️ Bytes discarded off the active segment's tail because they came after the last
    /// trusted commit (a torn write from a crash mid-append) — `0` on a clean recovery.
    pub torn_tail_bytes: u64,
    /// 🧯️ Incomplete active transaction durably aborted before this opener returned.
    pub recovered_abort_tx_id: Option<u64>,
}

struct WalChainFrame {
    start: usize,
    body: usize,
    end: usize,
    next: usize,
    position: usize,
    kind: u8,
    digest: semio_framework_hash::Hasher,
    crc: protocol::codec::Crc32cCursor,
}

struct WalSegmentChain {
    offset: usize,
    frame: Option<WalChainFrame>,
    pending: semio_framework_hash::Hasher,
    records_len: u64,
    record_count: u32,
    next_commit: u64,
    last_commit_offset: u64,
    tip: Option<[u8; 32]>,
    previous_tip: WalPriorChainTip,
    segment: u64,
    header_seen: bool,
}

#[derive(Clone, Copy)]
pub(crate) enum WalPriorChainTip { Genesis, RetainedBoundary, Verified([u8; 32]) }

impl WalSegmentChain {
    fn new(pages: &dyn WalImmutableByteSource, segment: u64, previous_tip: WalPriorChainTip) -> Result<Self, DbError> {
        let bytes = WalPageReader::new(pages, 0, protocol::format::HEADER_SIZE)?.array::<{ protocol::format::HEADER_SIZE }>()?;
        let header = protocol::format::parse_header_bytes(&bytes).map_err(protocol_err)?;
        if header.required_flags != protocol::wire::REQUIRED_HASH_CHAIN || header.optional_flags != 0 || header.version_minor != protocol::format::FORMAT_VERSION_MINOR {
            return Err(DbError::Corrupt("wal requires its exact hash-chain format".to_string()));
        }
        let mut pending = semio_framework_hash::Hasher::new();
        pending.update(semio_framework_hash::hash(&bytes).as_bytes());
        Ok(Self { offset: protocol::format::HEADER_SIZE, frame: None, pending, records_len: 0, record_count: 0, next_commit: 1, last_commit_offset: 0, tip: None, previous_tip, segment, header_seen: false })
    }

    fn check_segment_header(&mut self, pages: &dyn WalImmutableByteSource, frame: &WalChainFrame, document: &ArtifactId) -> Result<(), DbError> {
        if frame.kind != WAL_SEGMENT_HEADER || self.header_seen || frame.start != protocol::format::HEADER_SIZE {
            return Err(DbError::Corrupt("wal segment header is missing or repeated".to_string()));
        }
        let mut reader = WalPageReader::new(pages, frame.body + 2, frame.end)?;
        if reader.varint()? != document.0.len() as u64 { return Err(DbError::Corrupt("wal segment document differs".to_string())); }
        for byte in document.0.as_bytes() {
            if reader.byte()? != *byte { return Err(DbError::Corrupt("wal segment document differs".to_string())); }
        }
        if u64::from_le_bytes(reader.array()?) != self.segment { return Err(DbError::Corrupt("wal segment index differs".to_string())); }
        let previous = match reader.byte()? { 0 => None, 1 => Some(reader.array::<32>()?), _ => return Err(DbError::Corrupt("wal previous tip marker differs".to_string())) };
        let matches = match self.previous_tip {
            WalPriorChainTip::Genesis => previous.is_none(),
            WalPriorChainTip::RetainedBoundary => previous.is_some(),
            WalPriorChainTip::Verified(tip) => previous == Some(tip),
        };
        if !matches || reader.position != frame.end { return Err(DbError::Corrupt("wal previous committed chain tip differs".to_string())); }
        self.header_seen = true;
        Ok(())
    }

    fn step(&mut self, pages: &dyn WalImmutableByteSource, trusted_len: usize, document: &ArtifactId, control: &mut WalCursorControl) -> Result<bool, DbError> {
        control.grant()?;
        if self.frame.is_none() {
            if self.offset == trusted_len {
                if !self.header_seen || self.tip.is_none() || self.record_count != 0 || self.records_len != 0
                    || self.last_commit_offset.checked_add(protocol::format::COMMIT_FRAME_LEN) != Some(trusted_len as u64) {
                    return Err(DbError::Corrupt("wal segment ends outside a verified commit".to_string()));
                }
                return Ok(true);
            }
            let mut reader = WalPageReader::new(pages, self.offset, trusted_len)?;
            let length = reader.varint()?;
            check_len(length, protocol::ProtocolLimits::default().max_frame_len, "wal chain frame")?;
            let body = reader.position;
            let end = body.checked_add(usize::try_from(length).map_err(|_| DbError::LimitExceeded("wal chain frame length"))?).ok_or(DbError::LimitExceeded("wal chain frame end"))?;
            let next = end.checked_add(8).ok_or(DbError::LimitExceeded("wal chain trailer"))?;
            if length < 2 || next > trusted_len { return Err(DbError::Corrupt("wal chain frame exceeds committed bytes".to_string())); }
            let kind = reader.byte()?;
            let flags = reader.byte()?;
            if flags != protocol::wire::FRAME_FLAG_CRITICAL { return Err(DbError::Corrupt("wal chain frame flags differ".to_string())); }
            self.frame = Some(WalChainFrame { start: self.offset, body, end, next, position: self.offset, kind, digest: semio_framework_hash::Hasher::new(), crc: protocol::codec::Crc32cCursor::new() });
        }
        let frame = self.frame.as_mut().ok_or_else(|| DbError::Internal("wal chain frame disappeared".to_string()))?;
        let fragment = pages.fragment_at(frame.position, frame.next)?;
        let count = fragment.len().min(db_storage::DB_IO_PAGE_BYTES);
        if count == 0 { return Err(DbError::Corrupt("wal chain made no byte progress".to_string())); }
        let bytes = &fragment[..count];
        frame.digest.update(bytes);
        let crc_start = frame.position.max(frame.body);
        let crc_end = (frame.position + count).min(frame.end);
        if crc_start < crc_end { frame.crc.update_page(&bytes[crc_start - frame.position..crc_end - frame.position]); }
        frame.position += count;
        if frame.position != frame.next { return Ok(false); }
        let frame = self.frame.take().ok_or_else(|| DbError::Internal("wal chain frame disappeared".to_string()))?;
        let mut trailer = WalPageReader::new(pages, frame.end, frame.next)?;
        if u32::from_le_bytes(trailer.array()?) != frame.crc.finish() || u32::from_le_bytes(trailer.array()?) as usize != frame.next - frame.start {
            return Err(DbError::Corrupt("wal chain crc or frame length differs".to_string()));
        }
        if frame.kind == protocol::wire::REC_COMMIT {
            if frame.end - frame.body - 2 != protocol::format::COMMIT_PAYLOAD_LEN || frame.next - frame.start != protocol::format::COMMIT_FRAME_LEN as usize {
                return Err(DbError::Corrupt("wal chain commit framing differs".to_string()));
            }
            let payload = WalPageReader::new(pages, frame.body + 2, frame.end)?.array::<{ protocol::format::COMMIT_PAYLOAD_LEN }>()?;
            let commit = protocol::format::parse_commit_payload(&payload).map_err(protocol_err)?;
            if commit.commit_seq != self.next_commit || commit.prev_commit_offset != self.last_commit_offset || commit.record_count != self.record_count
                || commit.records_len != self.records_len || payload[28..32] != [0; 4] || commit.chain_hash != *self.pending.finalize().as_bytes() {
                return Err(DbError::Corrupt("wal committed hash chain differs".to_string()));
            }
            self.tip = Some(commit.chain_hash);
            self.pending = semio_framework_hash::Hasher::new(); self.pending.update(&commit.chain_hash);
            self.record_count = 0; self.records_len = 0;
            self.last_commit_offset = frame.start as u64;
            self.next_commit = self.next_commit.checked_add(1).ok_or(DbError::LimitExceeded("wal commit sequence"))?;
        } else {
            if !is_wal_record_kind(frame.kind) { return Err(DbError::Corrupt("wal chain contains an unknown record".to_string())); }
            if !self.header_seen || frame.kind == WAL_SEGMENT_HEADER { self.check_segment_header(pages, &frame, document)?; }
            self.pending.update(frame.digest.finalize().as_bytes());
            self.record_count = self.record_count.checked_add(1).ok_or(DbError::LimitExceeded("wal commit record count"))?;
            self.records_len = self.records_len.checked_add((frame.next - frame.start) as u64).ok_or(DbError::LimitExceeded("wal commit record bytes"))?;
        }
        self.offset = frame.next;
        Ok(false)
    }
}

pub(crate) enum WalAuthenticatedStep { Yield, Committed, Done }

/// 🔐️ Owns the one immutable source shared by physical authentication and logical admission.
pub(crate) struct WalAuthenticatedSource<S> {
    source: S,
    gate: WalTransactionGate,
    chain: Option<WalSegmentChain>,
    segment: u64,
    previous: WalPriorChainTip,
    offset: usize,
    verified: bool,
    done: bool,
}

impl<S: WalImmutableByteSource> WalAuthenticatedSource<S> {
    pub(crate) fn new(source: S, gate: WalTransactionGate, segment: u64, previous: WalPriorChainTip) -> Self {
        Self { source, gate, chain: None, segment, previous, offset: protocol::format::HEADER_SIZE, verified: false, done: false }
    }

    pub(crate) fn source(&self) -> &S { &self.source }

    pub(crate) fn verify_step(&mut self, document: &ArtifactId, control: &mut WalCursorControl) -> Result<bool, DbError> {
        control.check()?;
        if self.verified { return Ok(true); }
        if self.chain.is_none() {
            control.grant()?;
            self.chain = Some(WalSegmentChain::new(&self.source, self.segment, self.previous)?);
            return Ok(false);
        }
        self.verified = self.chain.as_mut().ok_or(DbError::Closed)?.step(&self.source, self.source.byte_len(), document, control)?;
        Ok(self.verified)
    }

    pub(crate) fn next_step(&mut self, control: &mut WalCursorControl) -> Result<WalAuthenticatedStep, DbError> {
        control.check()?;
        if !self.verified { return Err(DbError::Corrupt("wal source has not completed authentication".to_string())); }
        if self.done { return Ok(WalAuthenticatedStep::Done); }
        if self.gate.ready.is_some() { return Err(DbError::Corrupt("wal authenticated transaction remains unfinished".to_string())); }
        match wal_next_verified_page_frame(&self.source, &mut self.offset, self.source.byte_len(), control)? {
            WalVerifiedFrameStep::Frame(frame) => {
                let committed = self.gate.push(&self.source, frame)?;
                self.offset = frame.frame_end;
                Ok(if committed { WalAuthenticatedStep::Committed } else { WalAuthenticatedStep::Yield })
            }
            WalVerifiedFrameStep::PhysicalCommit => Ok(WalAuthenticatedStep::Yield),
            WalVerifiedFrameStep::Done => {
                self.gate.advance_segment()?;
                self.done = true;
                Ok(WalAuthenticatedStep::Done)
            }
        }
    }

    pub(crate) fn committed_frame(&self, index: usize) -> Option<WalRecordFrame> { self.gate.committed_frame(index) }

    pub(crate) fn finish_transaction(&mut self) -> Result<(), DbError> { self.gate.release() }

    pub(crate) fn finish(self) -> Result<(S, WalTransactionGate, [u8; 32]), Self> {
        match (self.done, self.chain.as_ref().and_then(|chain| chain.tip)) {
            (true, Some(tip)) => Ok((self.source, self.gate, tip)),
            _ => Err(self),
        }
    }

    /// 🛑️ Destroys verification and transaction continuation, returning only raw ownership for retirement.
    pub(crate) fn abort_into_source(self) -> S { self.source }
}

/// @emoji 🔁️ Decodes every `WAL_*` record across a document's ENTIRE WAL (every sealed segment in
/// full, plus the active segment), in segment then on-disk order. Each segment's complete retained
/// bytes must end in a verified commit; torn tails are rejected. Page-bounded verification checks
/// every frame and commit before releasing that segment's records, carrying the validated chain
/// tip into the next exact document/index header. Recovery/truncation belongs to the WAL opener.
/// A compacted suffix has no authenticated preceding tip; use `open_genesis` for a genesis proof.
pub struct WalReplayCursor<'storage, S: db_storage::WalStorage> {
    storage: &'storage S,
    document: ArtifactId,
    segments: db_storage::DbIoU64List,
    segment: usize,
    pages: Option<db_storage::DbIoPages>,
    offset: usize,
    trusted_len: usize,
    validation: Option<WalSegmentChain>,
    decoder: Option<WalRetainedRecordDecoder>,
    previous_tip: Option<[u8; 32]>,
    genesis_required: bool,
    failed: bool,
    control: WalCursorControl,
    closed: bool,
}

pub enum WalReplayStep {
    Record(WalRecord),
    Yield,
    Done,
}

impl<'storage, S: db_storage::WalStorage> WalReplayCursor<'storage, S> {
    /// 🔋️ Replenishes one bounded caller-owned replay opportunity without replacing the cursor.
    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        self.control.replenish(deadline, fuel)
    }

    pub async fn open(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<Self, DbError> {
        let segments = storage.list_segments(document).await?;
        Ok(Self { storage, document: document.clone(), segments, segment: 0, pages: None, offset: protocol::format::HEADER_SIZE, trusted_len: 0, validation: None, decoder: None, previous_tip: None, genesis_required: false, failed: false, control, closed: false })
    }

    /// 🌱️ Requires the retained chain to start at genesis rather than trusting a compacted boundary.
    pub async fn open_genesis(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<Self, DbError> {
        let mut cursor = Self::open(storage, document, control).await?;
        cursor.genesis_required = true;
        Ok(cursor)
    }

    async fn open_segment(&mut self) -> Result<bool, DbError> {
        let Some(index) = self.segments.as_slice().get(self.segment).copied() else { return Ok(false) };
        self.control.grant()?;
        let first = self.segments.as_slice()[0];
        if (self.genesis_required && first != 0) || first.checked_add(self.segment as u64) != Some(index) {
            return Err(DbError::Corrupt("wal segment sequence differs from its required boundary".to_string()));
        }
        let len = self.storage.segment_len(&self.document, index).await?;
        self.pages = Some(self.storage.read(&self.document, index, pack::ByteRange { offset: 0, len }).await?);
        let pages = self.pages.as_ref().ok_or_else(|| DbError::Internal("wal replay lost admitted pages".to_string()))?;
        self.offset = protocol::format::HEADER_SIZE;
        self.trusted_len = pages.len();
        let previous_tip = match self.previous_tip {
            Some(tip) => WalPriorChainTip::Verified(tip),
            None if index == 0 => WalPriorChainTip::Genesis,
            None => WalPriorChainTip::RetainedBoundary,
        };
        self.validation = Some(WalSegmentChain::new(pages, index, previous_tip)?);
        Ok(true)
    }

    async fn close_segment_step(&mut self) -> Result<bool, DbError> {
        self.control.grant()?;
        if let Some(pages) = self.pages.as_mut() {
            if pages.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.pages = None;
        self.trusted_len = 0;
        Ok(false)
    }

    pub async fn next_step(&mut self) -> Result<WalReplayStep, DbError> {
        if self.failed { return Err(DbError::Corrupt("wal replay remains failed until closed".to_string())); }
        let result = self.next_validated_step().await;
        let interrupted = matches!(&result, Err(DbError::LimitExceeded("wal cursor fuel")))
            || matches!(&result, Err(DbError::Unavailable(message)) if message == "wal cursor cancelled" || message == "wal cursor deadline reached");
        if result.is_err() && !interrupted { self.failed = true; }
        result
    }

    async fn next_validated_step(&mut self) -> Result<WalReplayStep, DbError> {
            self.control.check()?;
            if self.closed {
                return Ok(WalReplayStep::Done);
            }
            if self.pages.is_none() {
                return Ok(if self.open_segment().await? { WalReplayStep::Yield } else { WalReplayStep::Done });
            }
            if let Some(validation) = self.validation.as_mut() {
                let pages = self.pages.as_ref().ok_or_else(|| DbError::Internal("wal chain lost segment pages".to_string()))?;
                if validation.step(pages, self.trusted_len, &self.document, &mut self.control)? {
                    self.previous_tip = validation.tip;
                    self.validation = None;
                }
                return Ok(WalReplayStep::Yield);
            }
            if self.offset == self.trusted_len {
                if self.close_segment_step().await? {
                    return Ok(WalReplayStep::Yield);
                }
                self.segment += 1;
                return Ok(WalReplayStep::Yield);
            }
            let pages = self.pages.as_ref().ok_or_else(|| DbError::Internal("wal replay lost segment pages".to_string()))?;
            if self.decoder.is_none() {
                if let WalVerifiedFrameStep::Frame(frame) = wal_next_verified_page_frame(pages, &mut self.offset, self.trusted_len, &mut self.control)? {
                    self.decoder = Some(WalRetainedRecordDecoder::new(frame));
                }
                return Ok(WalReplayStep::Yield);
            }
            let decoder = self.decoder.as_mut().ok_or_else(|| DbError::Internal("wal replay lost retained decoder".to_string()))?;
            if let Some(record) = decoder.step(pages, &mut self.control)? {
                self.offset = decoder.frame.frame_end;
                self.decoder = None;
                return Ok(WalReplayStep::Record(record));
            }
            Ok(WalReplayStep::Yield)
    }

    #[cfg(test)]
    pub async fn next(&mut self) -> Result<Option<WalRecord>, DbError> {
        loop {
            match self.next_step().await? {
                WalReplayStep::Record(record) => return Ok(Some(record)),
                WalReplayStep::Yield => {}
                WalReplayStep::Done => return Ok(None),
            }
        }
    }

    pub fn close_owner_step(&mut self) -> Result<bool, DbError> {
        if let Some(decoder) = self.decoder.as_mut() { if decoder.close_owner_step()? { return Ok(true); } }
        self.decoder = None;
        if let Some(pages) = self.pages.as_mut() {
            if pages.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.pages = None;
        self.validation = None;
        self.previous_tip = None;
        self.trusted_len = 0;
        if self.segments.close_step() {
            return Ok(true);
        }
        if self.closed {
            return Ok(false);
        }
        self.closed = true;
        Ok(true)
    }

    pub async fn close_step(&mut self) -> Result<bool, DbError> {
        if self.close_owner_step()? {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.pages.is_none() && self.validation.is_none() && self.decoder.is_none() && self.segments.terminal_is_empty()
    }
}

pub async fn replay_document<'storage, S: db_storage::WalStorage>(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<WalReplayCursor<'storage, S>, DbError> {
    WalReplayCursor::open(storage, document, control).await
}

/// 🛂️ Releases bounded transaction bodies only after their exact logical commit is verified.
pub struct WalCommittedCursor<'storage, S: db_storage::WalStorage> {
    raw: WalReplayCursor<'storage, S>,
    gate: WalTransactionGate,
    record: Option<WalRecord>,
    record_index: usize,
}

pub enum WalCommittedStep<'cursor, 'storage, S: db_storage::WalStorage> {
    Transaction(WalCommittedTransaction<'cursor, 'storage, S>),
    Yield,
    Done,
}

/// 🤝️ Keeps the source segment and current decoded body borrowed until explicit retirement.
pub struct WalCommittedTransaction<'cursor, 'storage, S: db_storage::WalStorage> {
    cursor: &'cursor mut WalCommittedCursor<'storage, S>,
    finished: bool,
}

pub enum WalCommittedRecordStep<'record> {
    Record(&'record WalRecord),
    Yield,
    Done,
}

impl<'storage, S: db_storage::WalStorage> WalCommittedCursor<'storage, S> {
    /// 🗂️ Lists retained segments, including header-only segments; admission completes during replay.
    pub fn segment_indices(&self) -> &[u64] { self.raw.segments.as_slice() }

    pub async fn open(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<Self, DbError> {
        Ok(Self { raw: WalReplayCursor::open(storage, document, control).await?, gate: WalTransactionGate::new(), record: None, record_index: 0 })
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        self.raw.replenish(deadline, fuel)
    }

    async fn prepare_transaction_step(&mut self) -> Result<Option<bool>, DbError> {
        self.raw.control.check()?;
        if self.raw.closed { return Ok(None); }
        if self.gate.ready.is_some() { return Err(DbError::Corrupt("wal committed transaction was not finished".to_string())); }
        if self.raw.pages.is_none() { return Ok(self.raw.open_segment().await?.then_some(false)); }
        if let Some(validation) = self.raw.validation.as_mut() {
            let pages = self.raw.pages.as_ref().ok_or_else(|| DbError::Internal("wal committed replay lost validation pages".to_string()))?;
            if validation.step(pages, self.raw.trusted_len, &self.raw.document, &mut self.raw.control)? {
                self.raw.previous_tip = validation.tip;
                self.raw.validation = None;
            }
            return Ok(Some(false));
        }
        if self.raw.offset == self.raw.trusted_len {
            self.gate.finish_segment(false)?;
            if self.raw.close_segment_step().await? { return Ok(Some(false)); }
            self.gate.advance_segment()?;
            self.raw.segment += 1;
            return Ok(Some(false));
        }
        let pages = self.raw.pages.as_ref().ok_or_else(|| DbError::Internal("wal committed replay lost source pages".to_string()))?;
        let WalVerifiedFrameStep::Frame(frame) = wal_next_verified_page_frame(pages, &mut self.raw.offset, self.raw.trusted_len, &mut self.raw.control)? else { return Ok(Some(false)) };
        let ready = self.gate.push(pages, frame)?;
        self.raw.offset = frame.frame_end;
        Ok(Some(ready))
    }

    pub async fn next_transaction_step(&mut self) -> Result<WalCommittedStep<'_, 'storage, S>, DbError> {
        if self.raw.failed { return Err(DbError::Corrupt("wal committed replay remains failed until closed".to_string())); }
        match self.prepare_transaction_step().await {
            Ok(Some(true)) => Ok(WalCommittedStep::Transaction(WalCommittedTransaction { cursor: self, finished: false })),
            Ok(Some(false)) => Ok(WalCommittedStep::Yield),
            Ok(None) => Ok(WalCommittedStep::Done),
            Err(error) => {
                if !wal_cursor_interrupted(&error) { self.raw.failed = true; }
                Err(error)
            }
        }
    }

    pub fn close_owner_step(&mut self) -> Result<bool, DbError> {
        if let Some(record) = self.record.as_mut() { if record.close_step()? { return Ok(true); } }
        self.record = None;
        if self.raw.close_owner_step()? { return Ok(true); }
        self.gate.clear_frames();
        self.gate.ready = None;
        self.gate.transaction_id = None;
        self.record_index = 0;
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.raw.terminal_is_empty() && self.record.is_none() && self.gate.frames_len == 0 && self.gate.ready.is_none() && self.gate.transaction_id.is_none()
    }
}

fn wal_cursor_interrupted(error: &DbError) -> bool {
    matches!(error, DbError::LimitExceeded("wal cursor fuel"))
        || matches!(error, DbError::Unavailable(message) if message == "wal cursor cancelled" || message == "wal cursor deadline reached")
}

impl<'cursor, 'storage, S: db_storage::WalStorage> WalCommittedTransaction<'cursor, 'storage, S> {
    pub fn transaction_id(&self) -> u64 { self.cursor.gate.ready.expect("committed transaction owns its ready gate") }

    pub fn segment_index(&self) -> u64 { self.cursor.raw.segments.as_slice()[self.cursor.raw.segment] }

    pub fn record_count(&self) -> usize { self.cursor.gate.frames_len as usize }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> { self.cursor.replenish(deadline, fuel) }

    fn decode_record_step(&mut self) -> Result<bool, DbError> {
        if self.cursor.record.is_some() { return Err(DbError::Corrupt("wal committed body must be closed before advancing".to_string())); }
        let frame = self.cursor.gate.frames[self.cursor.record_index].ok_or_else(|| DbError::Internal("wal committed replay lost body span".to_string()))?;
        if self.cursor.raw.decoder.is_none() { self.cursor.raw.decoder = Some(WalRetainedRecordDecoder::new(frame)); }
        let pages = self.cursor.raw.pages.as_ref().ok_or_else(|| DbError::Internal("wal committed replay lost body source".to_string()))?;
        let decoder = self.cursor.raw.decoder.as_mut().ok_or_else(|| DbError::Internal("wal committed replay lost body decoder".to_string()))?;
        if let Some(record) = decoder.step(pages, &mut self.cursor.raw.control)? {
            self.cursor.record = Some(record);
            self.cursor.raw.decoder = None;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn next_record_step(&mut self) -> Result<WalCommittedRecordStep<'_>, DbError> {
        if self.cursor.raw.failed { return Err(DbError::Corrupt("wal committed replay remains failed until closed".to_string())); }
        if self.cursor.record_index == self.record_count() { return Ok(WalCommittedRecordStep::Done); }
        match self.decode_record_step() {
            Ok(true) => Ok(WalCommittedRecordStep::Record(self.cursor.record.as_ref().expect("decoded body remains cursor-owned"))),
            Ok(false) => Ok(WalCommittedRecordStep::Yield),
            Err(error) => {
                if !wal_cursor_interrupted(&error) { self.cursor.raw.failed = true; }
                Err(error)
            }
        }
    }

    pub fn close_record_step(&mut self) -> Result<bool, DbError> {
        let record = self.cursor.record.as_mut().ok_or_else(|| DbError::Corrupt("wal committed body is not borrowed".to_string()))?;
        if record.close_step()? { return Ok(true); }
        self.cursor.record = None;
        self.cursor.record_index += 1;
        Ok(false)
    }

    pub fn finish(mut self) -> Result<(), DbError> {
        if self.cursor.raw.failed || self.cursor.record.is_some() || self.cursor.raw.decoder.is_some() || self.cursor.record_index != self.record_count() {
            return Err(DbError::Corrupt("wal committed transaction has unretired bodies".to_string()));
        }
        self.cursor.gate.release()?;
        self.cursor.record_index = 0;
        self.finished = true;
        Ok(())
    }
}

impl<S: db_storage::WalStorage> Drop for WalCommittedTransaction<'_, '_, S> {
    fn drop(&mut self) { if !self.finished { self.cursor.raw.failed = true; } }
}

pub async fn replay_committed_document<'storage, S: db_storage::WalStorage>(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<WalCommittedCursor<'storage, S>, DbError> {
    WalCommittedCursor::open(storage, document, control).await
}

async fn scan_retained_pages(pages: &db_storage::DbIoPages, control: &mut WalCursorControl) -> Result<protocol::format::retained::VerifiedSprSpan, DbError> {
    use protocol::format::retained::{RetainedSprDiagnostic, RetainedSprLimits, RetainedSprVerification};
    let map_error = |error| match error {
        RetainedSprDiagnostic::Capacity => DbError::LimitExceeded("wal retained verification"),
        RetainedSprDiagnostic::Cancelled => DbError::Unavailable("wal retained verification cancelled".to_string()),
        _ => DbError::Corrupt("wal retained framing or commit verification failed".to_string()),
    };
    let capacity = db_storage::DB_IO_MAX_READ_BYTES;
    let mut scan = RetainedSprVerification::new(pages.len() as u64, RetainedSprLimits { file_bytes: capacity, frame_body_bytes: capacity, records: capacity / 11 }).map_err(map_error)?;
    for fragment in pages.fragments() {
        control.grant()?;
        let mut fuel = fragment.len();
        if scan.push(fragment, &mut fuel).map_err(map_error)? != fragment.len() { return Err(DbError::Corrupt("wal retained scan made partial page progress".to_string())); }
        semio_framework_async::yield_once().await;
    }
    let header = protocol::format::read_header(&WalPageSource(pages)).await.map_err(protocol_err)?;
    if header.optional_flags != 0 { return Err(DbError::Corrupt("wal optional header flags differ".to_string())); }
    scan.finish().map_err(map_error)
}

struct WalValidatedPrefix {
    records: u64,
    next_tx_id: u64,
    incomplete_active_tx: Option<u64>,
}

async fn validate_wal_prefix(pages: &db_storage::DbIoPages, span: &protocol::format::retained::VerifiedSprSpan, index: u64, prior: WalPriorChainTip, document: &ArtifactId, gate: &mut WalTransactionGate, writable_highest: bool, control: &mut WalCursorControl) -> Result<WalValidatedPrefix, DbError> {
    let end = usize::try_from(span.end()).map_err(|_| DbError::LimitExceeded("wal verified end"))?;
    let mut chain = WalSegmentChain::new(pages, index, prior)?;
    while !chain.step(pages, end, document, control)? { semio_framework_async::yield_once().await; }
    if chain.tip != Some(*span.chain()) { return Err(DbError::Corrupt("wal verifiers disagree on committed chain".to_string())); }
    let mut offset = protocol::format::HEADER_SIZE;
    let mut records = 0u64;
    loop {
        let frame = match wal_next_verified_page_frame(pages, &mut offset, end, control)? {
            WalVerifiedFrameStep::Frame(frame) => frame,
            WalVerifiedFrameStep::PhysicalCommit => { semio_framework_async::yield_once().await; continue; }
            WalVerifiedFrameStep::Done => break,
        };
        if gate.push(pages, frame)? { gate.release()?; }
        offset = frame.frame_end;
        records = records.checked_add(1).ok_or(DbError::LimitExceeded("wal recovered records"))?;
        semio_framework_async::yield_once().await;
    }
    let incomplete_active_tx = gate.finish_segment(writable_highest)?;
    if incomplete_active_tx.is_none() { gate.advance_segment()?; }
    Ok(WalValidatedPrefix { records, next_tx_id: gate.next_tx_id, incomplete_active_tx })
}

async fn copy_verified_prefix(pages: &db_storage::DbIoPages, end: u64, control: &mut WalCursorControl) -> Result<SharedBuf, DbError> {
    let buf = SharedBuf::try_new()?;
    let result = async {
        let mut remaining = usize::try_from(end).map_err(|_| DbError::LimitExceeded("wal prefix end"))?;
        for fragment in pages.fragments() {
            if remaining == 0 { break; }
            control.grant()?;
            let count = remaining.min(fragment.len());
            let mut written = 0;
            while written < count {
                let progress = lock(&buf.0).write_fragment(&fragment[written..count])?;
                if progress == 0 { return Err(DbError::LimitExceeded("wal retained prefix writer")); }
                written += progress;
                semio_framework_async::yield_once().await;
            }
            remaining -= count;
            semio_framework_async::yield_once().await;
        }
        if remaining != 0 { return Err(DbError::Corrupt("wal verified prefix ended early".to_string())); }
        Ok(())
    }.await;
    if let Err(error) = result {
        while lock(&buf.0).close_step()?.is_some() { semio_framework_async::yield_once().await; }
        return Err(error);
    }
    Ok(buf)
}

fn exact_prior_tip(prior: WalPriorChainTip) -> Result<Option<[u8; 32]>, DbError> {
    match prior {
        WalPriorChainTip::Genesis => Ok(None),
        WalPriorChainTip::Verified(tip) => Ok(Some(tip)),
        WalPriorChainTip::RetainedBoundary => Err(DbError::Corrupt("wal cannot initialize an unanchored compacted boundary".to_string())),
    }
}
//#endregion 🔖️Recovery

//#region 🔖️Segment
/// @emoji 📦️ The live write path for exactly one WAL segment: an in-memory `protocol::SprWriter`
/// over a `SharedBuf`, plus how much of that buffer has actually been flushed (durably appended)
/// to `db_storage::WalStorage` so far. Held open for the segment's entire lifetime (see
/// `SharedBuf`'s doc for why) — sealed and replaced by a fresh one on rotation.
struct SegmentWriter {
    document: ArtifactId,
    index: u64,
    buf: Option<SharedBuf>,
    writer: Option<protocol::SprWriter<SharedBuf>>,
    flushed_len: u64,
    pending_records: u32,
    oldest_pending_at_ms: Option<u64>,
}

impl SegmentWriter {
    fn writer_mut(&mut self) -> Result<&mut protocol::SprWriter<SharedBuf>, DbError> {
        self.writer.as_mut().ok_or(DbError::Closed)
    }

    fn buf(&self) -> Result<&SharedBuf, DbError> {
        self.buf.as_ref().ok_or(DbError::Closed)
    }

    fn ensure_open(&self) -> Result<(), DbError> {
        if self.writer.is_some() && self.buf.is_some() { Ok(()) } else { Err(DbError::Closed) }
    }

    fn poison(&mut self) {
        self.writer = None;
    }

    async fn retire_poisoned(&mut self) -> Result<(), DbError> {
        self.poison();
        while self.close_step()? { semio_framework_async::yield_once().await; }
        Ok(())
    }

    async fn resume_existing_verified(document: ArtifactId, index: u64, buf: SharedBuf, span: protocol::format::retained::VerifiedSprSpan) -> Result<Self, DbError> {
        let flushed_len = span.end();
        let writer = match protocol::SprWriter::resume_verified(buf.clone(), span).await.map_err(protocol_err) {
            Ok(writer) => writer,
            Err(error) => {
                let mut buf = buf;
                while buf.close_step()? { semio_framework_async::yield_once().await; }
                return Err(error);
            }
        };
        Ok(Self { document, index, buf: Some(buf), writer: Some(writer), flushed_len, pending_records: 0, oldest_pending_at_ms: None })
    }

    async fn initialize_existing_empty(storage: &impl db_storage::WalStorage, document: ArtifactId, index: u64, prev_chain_hash: Option<[u8; 32]>, partial: &db_storage::DbIoPages, now_ms: u64, control: &mut WalCursorControl) -> Result<Self, DbError> {
        let mut buf = SharedBuf::try_new()?;
        let writer = match protocol::SprWriter::begin(buf.clone(), &segment_write_options()).await.map_err(protocol_err) {
            Ok(writer) => writer,
            Err(error) => {
                while buf.close_step()? { semio_framework_async::yield_once().await; }
                return Err(error);
            }
        };
        let mut segment = Self { document: document.clone(), index, buf: Some(buf), writer: Some(writer), flushed_len: 0, pending_records: 0, oldest_pending_at_ms: None };
        let result = async {
            let mut expected = [0u8; protocol::format::HEADER_SIZE];
            segment.buf()?.read_exact(0, &mut expected).await?;
            let mut actual = WalPageReader::new(partial, 0, partial.len())?;
            for byte in &expected[..partial.len()] {
                if actual.byte()? != *byte { return Err(DbError::Corrupt("wal partial header differs from its exact profile".to_string())); }
            }
            control.grant()?;
            if !partial.is_empty() {
                storage.truncate_tail(&document, index, 0).await?;
                storage.sync(&document, index, DurabilityClass::Fsync).await?;
            }
            segment.append_record(&WalRecord::SegmentHeader { document: document.clone(), segment_index: index, prev_chain_hash }, now_ms).await?;
            segment.commit_and_flush(storage, DurabilityClass::Fsync).await?;
            Ok(())
        }.await;
        if let Err(error) = result {
            segment.retire_poisoned().await?;
            return Err(error);
        }
        Ok(segment)
    }

    /// @emoji 🆕️ Creates segment `index` in `storage`, writes its `WAL_SEGMENT_HEADER` record, and
    /// commits+flushes immediately (a segment's own identity/chain-link should never be lost to a
    /// crash before the segment records anything else).
    async fn begin(storage: &impl db_storage::WalStorage, document: ArtifactId, index: u64, prev_chain_hash: Option<[u8; 32]>, now_ms: u64) -> Result<Self, DbError> {
        storage.create_segment(&document, index).await?;
        let mut buf = SharedBuf::try_new()?;
        let writer = match protocol::SprWriter::begin(buf.clone(), &segment_write_options()).await.map_err(protocol_err) {
            Ok(writer) => writer,
            Err(error) => {
                while buf.close_step()? { semio_framework_async::yield_once().await; }
                return Err(error);
            }
        };
        let mut segment = Self { document: document.clone(), index, buf: Some(buf), writer: Some(writer), flushed_len: 0, pending_records: 0, oldest_pending_at_ms: None };
        let result = async {
            segment.append_record(&WalRecord::SegmentHeader { document, segment_index: index, prev_chain_hash }, now_ms).await?;
            segment.commit_and_flush(storage, DurabilityClass::Fsync).await?;
            Ok(())
        }.await;
        if let Err(error) = result {
            segment.poison();
            while segment.close_step()? { semio_framework_async::yield_once().await; }
            return Err(error);
        }
        Ok(segment)
    }

    async fn append_record(&mut self, record: &WalRecord, now_ms: u64) -> Result<u64, DbError> {
        let offset = record.write_retained(self.writer_mut()?).await?;
        if self.pending_records == 0 {
            self.oldest_pending_at_ms = Some(now_ms);
        }
        self.pending_records += 1;
        Ok(offset)
    }

    /// @emoji 📏️ Bytes written since the last flush — not yet visible to `WalStorage`.
    fn pending_bytes(&self) -> Result<u64, DbError> {
        self.buf()?.len().checked_sub(self.flushed_len).ok_or_else(|| DbError::Corrupt("WAL flushed length exceeds its retained writer".to_string()))
    }

    /// @emoji 📏️ The segment's total logical length so far, flushed or not — what
    /// `ArtifactWal::submit` compares against its segment-rotation threshold.
    fn total_len(&self) -> Result<u64, DbError> {
        Ok(self.buf()?.len())
    }

    /// @emoji ⛓️ Physically commits (`SprWriter::commit`, hash-chaining everything pending) and
    /// flushes the newly-committed suffix to `WalStorage::append` + `sync(class)` — the group-
    /// commit primitive `ArtifactWal::submit`/`force_flush`/`rotate` all funnel through. A no-op
    /// (`Ok(None)`) if nothing is pending.
    async fn commit_and_flush(&mut self, storage: &impl db_storage::WalStorage, class: DurabilityClass) -> Result<Option<u64>, DbError> {
        self.ensure_open()?;
        if self.pending_records == 0 {
            return Ok(None);
        }
        let commit = self.writer_mut()?.commit().await.map_err(protocol_err);
        let commit_offset = match commit {
            Ok(offset) => offset,
            Err(error) => {
                self.poison();
                return Err(error);
            }
        };
        let flush = async {
            let expected_len = self.buf()?.len();
            let suffix_len = usize::try_from(expected_len.checked_sub(self.flushed_len).ok_or_else(|| DbError::Corrupt("WAL flushed length exceeds its retained writer".to_string()))?).map_err(|_| DbError::LimitExceeded("WAL retained suffix length"))?;
            let pages = self.buf()?.copy_range(self.flushed_len as usize, suffix_len).await?;
            let new_len = storage.append(&self.document, self.index, pages).await?;
            if new_len != expected_len {
                return Err(DbError::Corrupt(format!("WAL append returned segment length {new_len}, expected {expected_len}")));
            }
            self.flushed_len = new_len;
            storage.sync(&self.document, self.index, class).await?;
            Ok(())
        }.await;
        match flush {
            Ok(()) => {
                self.pending_records = 0;
                self.oldest_pending_at_ms = None;
                Ok(Some(commit_offset))
            }
            Err(error) => {
                self.poison();
                Err(error)
            }
        }
    }

    /// @emoji ⛓️ The chain_hash of this segment's last commit (falling back to `blake3(header)` if
    /// nothing has committed beyond the segment's own header write, which `begin` always performs,
    /// so this should never actually hit that branch in practice — handled honestly rather than
    /// assumed away). Used by `ArtifactWal::rotate` to seed the next segment's
    /// `WAL_SEGMENT_HEADER.prev_chain_hash`.
    async fn tip_chain_hash(&self) -> Result<[u8; 32], DbError> {
        self.ensure_open()?;
        let commit_len = protocol::format::COMMIT_FRAME_LEN as usize;
        if self.buf()?.len() < commit_len as u64 {
            return Err(DbError::Corrupt("WAL retained pages contain no commit frame".to_string()));
        }
        let commit_offset = self.buf()?.len() as usize - commit_len;
        let mut frame_bytes = [0u8; protocol::format::COMMIT_FRAME_LEN as usize];
        self.buf()?.read_exact(commit_offset, &mut frame_bytes).await?;
        let mut cursor = protocol::FrameCursor::new(&frame_bytes, 0).await;
        let frame = cursor.next_frame().await.map_err(protocol_err)?.ok_or_else(|| DbError::Corrupt("expected a commit frame while sealing wal segment".to_string()))?;
        if frame.kind != protocol::wire::REC_COMMIT {
            return Err(DbError::Corrupt(format!("expected REC_COMMIT at the recovered commit offset, found kind {:#x}", frame.kind)));
        }
        Ok(protocol::format::parse_commit_payload(frame.payload().await).map_err(protocol_err)?.chain_hash)
    }

    fn close_step(&mut self) -> Result<bool, DbError> {
        if self.pending_records != 0 && self.writer.is_some() {
            return Err(DbError::InvalidArgument("WAL has pending records; force_flush is required before close".to_string()));
        }
        if self.writer.take().is_some() {
            return Ok(true);
        }
        let Some(buf) = self.buf.as_mut() else { return Ok(false) };
        if buf.close_step()? {
            return Ok(true);
        }
        if !buf.terminal_is_empty() {
            return Err(DbError::Internal("WAL retained buffer did not reach terminal ownership".to_string()));
        }
        self.buf = None;
        Ok(true)
    }

    fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.buf.is_none()
    }
}
//#endregion 🔖️Segment

//#region 🔖️ArtifactWal
/// @emoji 📏️ Default segment-rotation threshold (this crate's own choice — the contract fixes
/// "per-document segment files", not an exact size): large enough that rotation stays rare under
/// ordinary load, small enough that a single segment's crash-recovery replay stays bounded.
const DEFAULT_MAX_SEGMENT_BYTES: u64 = db_storage::DB_IO_MAX_READ_BYTES;

fn wal_frame_bytes(payload: usize) -> Result<u64, DbError> {
    let body = (payload as u64).checked_add(2).ok_or(DbError::LimitExceeded("wal frame bytes"))?;
    body.checked_add(wal_varint_len(body) as u64 + 8).ok_or(DbError::LimitExceeded("wal frame bytes"))
}

fn wal_transaction_frame_bytes(records: &WalRecordBatch) -> Result<u64, DbError> {
    let mut bytes = wal_frame_bytes(8)?.checked_add(wal_frame_bytes(12)?).ok_or(DbError::LimitExceeded("wal transaction bytes"))?;
    for record in records.iter() {
        bytes = bytes.checked_add(wal_frame_bytes(record.retained_shape().1)?).ok_or(DbError::LimitExceeded("wal transaction bytes"))?;
    }
    Ok(bytes)
}

/// @emoji 🧾️ What `ArtifactWal::submit` did with one transaction's records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalAppendReceipt {
    pub segment_index: u64,
    pub tx_id: u64,
    /// @emoji ✅️ True iff this call caused (or piggybacked on) a physical `commit()`+`sync` —
    /// false means the transaction is durable only up to `DurabilityClass::Memory` so far, still
    /// batched behind `GroupCommitPolicy`.
    pub committed: bool,
}

/// @emoji 📼️ One document's write-ahead log: an ordered chain of per-segment `.spr` files over
/// `db_storage::WalStorage`, with group-commit batching and crash recovery. This is the type
/// `db_artifact`'s authority actor owns one of per open document.
pub struct ArtifactWal {
    document: ArtifactId,
    policy: GroupCommitPolicy,
    max_segment_bytes: u64,
    next_segment_index: u64,
    active: SegmentWriter,
    next_tx_id: u64,
}

impl ArtifactWal {
    /// @emoji 🌱️ Creates a brand new WAL for `document` (segment 0, genesis — no prior segment to
    /// chain from). Errors `AlreadyExists` if `document` already has WAL segments in `storage`.
    pub async fn create(storage: &impl db_storage::WalStorage, document: ArtifactId, policy: GroupCommitPolicy, now_ms: u64) -> Result<Self, DbError> {
        let active = SegmentWriter::begin(storage, document.clone(), 0, None, now_ms).await?;
        Ok(Self { document, policy, max_segment_bytes: DEFAULT_MAX_SEGMENT_BYTES, next_segment_index: 1, active, next_tx_id: 1 })
    }

    /// 🚑️ Verifies the retained chain, durably aborts incomplete active transactions, repairs the
    /// uncommitted tail, and resumes the physical sequence under the caller's exclusive write authority.
    pub async fn open(storage: &impl db_storage::WalStorage, document: ArtifactId, policy: GroupCommitPolicy, now_ms: u64) -> Result<(Self, WalRecoveryReport), DbError> {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = WalCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
        Self::open_with_control(storage, document, policy, now_ms, &mut control).await
    }

    /// 🚦️ Caller-controlled bounded recovery; cleanup is never gated by cancellation.
    pub async fn open_with_control(storage: &impl db_storage::WalStorage, document: ArtifactId, policy: GroupCommitPolicy, now_ms: u64, control: &mut WalCursorControl) -> Result<(Self, WalRecoveryReport), DbError> {
        control.grant()?;
        let mut indices = storage.list_segments(&document).await?;
        let result = async {
            if indices.is_empty() {
                control.grant()?;
                return Ok((Self::create(storage, document.clone(), policy, now_ms).await?, WalRecoveryReport::default()));
            }
            let first = indices.as_slice()[0];
            let last = *indices.last().ok_or_else(|| DbError::Internal("wal segment list lost its highest index".to_string()))?;
            let mut next_segment_index = last.checked_add(1).ok_or(DbError::LimitExceeded("wal segment sequence"))?;
            for (ordinal, index) in indices.as_slice().iter().enumerate() {
                if first.checked_add(ordinal as u64) != Some(*index) { return Err(DbError::Corrupt("wal retained segment sequence is not dense".to_string())); }
            }
            let mut prior = if first == 0 { WalPriorChainTip::Genesis } else { WalPriorChainTip::RetainedBoundary };
            let mut report = WalRecoveryReport { segments_seen: indices.len() as u64, ..WalRecoveryReport::default() };
            let mut next_tx_id = 1;
            let mut logical = WalTransactionGate::new();
            let mut active = None;
            for &index in indices.as_slice() {
                control.grant()?;
                let state = storage.segment_state(&document, index).await?;
                let writable = index == last && state == db_storage::WalSegmentState::Active;
                if index != last && state != db_storage::WalSegmentState::Sealed { return Err(DbError::Corrupt("wal has an active segment before its highest index".to_string())); }
                if index == last && !writable { next_segment_index = next_segment_index.checked_add(1).ok_or(DbError::LimitExceeded("wal successor sequence"))?; }
                control.grant()?;
                let len = storage.segment_len(&document, index).await?;
                control.grant()?;
                let mut pages = storage.read(&document, index, pack::ByteRange { offset: 0, len }).await?;
                let outcome = async {
                    if pages.len() as u64 != len { return Err(DbError::Corrupt("wal retained read length differs".to_string())); }
                    if len < protocol::format::HEADER_SIZE as u64 {
                        if !writable { return Err(DbError::Corrupt("sealed wal segment has no committed header".to_string())); }
                        let predecessor = exact_prior_tip(prior)?;
                        let segment = SegmentWriter::initialize_existing_empty(storage, document.clone(), index, predecessor, &pages, now_ms, control).await?;
                        return Ok((Some(segment), None, 0, 1, len, None));
                    }
                    let span = scan_retained_pages(&pages, control).await?;
                    if !writable && (span.tail() != 0 || span.sequence() == 0) { return Err(DbError::Corrupt("sealed wal segment has uncommitted bytes".to_string())); }
                    let genesis = span.sequence() == 0;
                    let predecessor = if genesis { exact_prior_tip(prior)? } else { None };
                    let validated = if genesis { WalValidatedPrefix { records: 0, next_tx_id: logical.next_tx_id, incomplete_active_tx: None } } else { validate_wal_prefix(&pages, &span, index, prior, &document, &mut logical, writable, control).await? };
                    let tip = *span.chain();
                    let tail = span.tail();
                    let end = span.end();
                    if !writable { return Ok((None, Some(tip), validated.records, validated.next_tx_id, 0, None)); }
                    if validated.incomplete_active_tx.is_some() {
                        let repaired_end = end.checked_add(wal_frame_bytes(8)?).and_then(|bytes| bytes.checked_add(protocol::format::COMMIT_FRAME_LEN));
                        if repaired_end.is_none_or(|bytes| bytes > db_storage::DB_IO_MAX_READ_BYTES) { return Err(DbError::LimitExceeded("wal recovery abort exceeds retained segment budget")); }
                    }
                    let buf = copy_verified_prefix(&pages, end, control).await?;
                    let mut segment = SegmentWriter::resume_existing_verified(document.clone(), index, buf, span).await?;
                    let repair = async {
                        control.grant()?;
                        if tail != 0 {
                            storage.truncate_tail(&document, index, end).await?;
                            storage.sync(&document, index, DurabilityClass::Fsync).await?;
                        }
                        if genesis {
                            segment.append_record(&WalRecord::SegmentHeader { document: document.clone(), segment_index: index, prev_chain_hash: predecessor }, now_ms).await?;
                            segment.commit_and_flush(storage, DurabilityClass::Fsync).await?;
                        }
                        if let Some(tx_id) = validated.incomplete_active_tx {
                            segment.append_record(&WalRecord::TxAbort { tx_id }, now_ms).await?;
                            segment.commit_and_flush(storage, DurabilityClass::Fsync).await?;
                        }
                        segment.tip_chain_hash().await
                    }.await;
                    let tip = match repair {
                        Ok(tip) => tip,
                        Err(error) => {
                            segment.retire_poisoned().await?;
                            return Err(error);
                        }
                    };
                    Ok((Some(segment), Some(tip), validated.records, validated.next_tx_id, tail, validated.incomplete_active_tx))
                }.await;
                while pages.close_step()?.is_some() { semio_framework_async::yield_once().await; }
                let (segment, tip, records, next_tx, tail, recovered_abort) = outcome?;
                report.records_replayed = report.records_replayed.checked_add(records).ok_or(DbError::LimitExceeded("wal recovered records"))?;
                report.torn_tail_bytes = tail;
                report.recovered_abort_tx_id = recovered_abort;
                next_tx_id = next_tx_id.max(next_tx);
                if let Some(tip) = tip { prior = WalPriorChainTip::Verified(tip); }
                if let Some(segment) = segment { active = Some(segment); }
            }
            let active = match active {
                Some(active) => active,
                None => {
                    let predecessor = exact_prior_tip(prior)?;
                    control.grant()?;
                    SegmentWriter::begin(storage, document.clone(), last + 1, predecessor, now_ms).await?
                }
            };
            Ok((Self { document: document.clone(), policy, max_segment_bytes: DEFAULT_MAX_SEGMENT_BYTES, next_segment_index, active, next_tx_id }, report))
        }.await;
        while indices.close_step() { semio_framework_async::yield_once().await; }
        result
    }

    pub async fn document(&self) -> &ArtifactId {
        &self.document
    }

    pub async fn active_segment_index(&self) -> u64 {
        self.active.index
    }

    /// @emoji ✍️ Appends `records` as one transaction (`WAL_TX_BEGIN` .. `WAL_TX_COMMIT`), then
    /// group-commits per `GroupCommitPolicy` — except `durability >= Fsync` always forces an
    /// immediate commit, since deferring one can never satisfy a durability request stronger than
    /// what's already flushed. Rotates to a new segment (sealing this one first, which forces a
    /// commit if anything is still pending) once the active segment crosses `max_segment_bytes`.
    pub async fn submit(&mut self, storage: &impl db_storage::WalStorage, records: &WalRecordBatch, durability: DurabilityClass, now_ms: u64) -> Result<WalAppendReceipt, DbError> {
        self.active.ensure_open()?;
        let reservation = wal_transaction_frame_bytes(records)?.checked_add(protocol::format::COMMIT_FRAME_LEN).ok_or(DbError::LimitExceeded("wal transaction reservation"))?;
        let next_tx_id = self.next_tx_id.checked_add(1).ok_or(DbError::LimitExceeded("wal transaction sequence"))?;
        if self.active.total_len()?.checked_add(reservation).ok_or(DbError::LimitExceeded("wal segment reservation"))? > db_storage::DB_IO_MAX_READ_BYTES {
            let header_payload = wal_field_len(self.document.0.as_bytes()).checked_add(41).ok_or(DbError::LimitExceeded("wal successor header"))?;
            let header_bytes = wal_frame_bytes(header_payload)?.checked_add(protocol::format::HEADER_SIZE as u64 + protocol::format::COMMIT_FRAME_LEN).ok_or(DbError::LimitExceeded("wal successor header"))?;
            if header_bytes.checked_add(reservation).ok_or(DbError::LimitExceeded("wal transaction reservation"))? > db_storage::DB_IO_MAX_READ_BYTES {
                return Err(DbError::LimitExceeded("wal transaction exceeds readable segment"));
            }
            self.rotate(storage, now_ms).await?;
        }
        let tx_id = self.next_tx_id;
        self.next_tx_id = next_tx_id;
        let segment_index = self.active.index;

        self.active.append_record(&WalRecord::TxBegin { tx_id }, now_ms).await?;
        for record in records.iter() {
            self.active.append_record(record, now_ms).await?;
        }
        self.active.append_record(&WalRecord::TxCommit { tx_id, record_count: records.len() as u32 }, now_ms).await?;

        let forced = matches!(durability, DurabilityClass::Fsync | DurabilityClass::Quorum(_));
        let due = forced || self.policy.is_due(self.active.pending_bytes()?, self.active.pending_records, self.active.oldest_pending_at_ms, now_ms);
        let mut committed = false;
        if due {
            committed = self.active.commit_and_flush(storage, durability).await?.is_some();
        }
        if self.active.total_len()? >= self.max_segment_bytes {
            self.rotate(storage, now_ms).await?;
            committed = true;
        }
        Ok(WalAppendReceipt { segment_index, tx_id, committed })
    }

    /// @emoji 🚿️ Forces a commit+flush of whatever is currently pending, regardless of policy —
    /// the primitive a timer-driven group-commit loop or a clean-shutdown drain calls. Returns
    /// `true` iff there was anything to flush.
    pub async fn force_flush(&mut self, storage: &impl db_storage::WalStorage) -> Result<bool, DbError> {
        Ok(self.active.commit_and_flush(storage, DurabilityClass::Fsync).await?.is_some())
    }

    /// @emoji 🔄️ Seals the active segment (after a final commit+flush) and begins a fresh one,
    /// carrying the sealed segment's tip `chain_hash` forward as the new segment's
    /// `WAL_SEGMENT_HEADER.prev_chain_hash` — the cross-segment hash-chain link.
    async fn rotate(&mut self, storage: &impl db_storage::WalStorage, now_ms: u64) -> Result<(), DbError> {
        self.active.ensure_open()?;
        let following_index = self.next_segment_index.checked_add(1).ok_or(DbError::LimitExceeded("wal segment sequence"))?;
        self.active.commit_and_flush(storage, DurabilityClass::Fsync).await?;
        let chain_hash = self.active.tip_chain_hash().await?;
        let sealed_index = self.active.index;
        let sealed = storage.seal(&self.document, sealed_index).await;
        self.active.poison();
        sealed?;
        let new_index = self.next_segment_index;
        while self.active.close_step()? { semio_framework_async::yield_once().await; }
        let active = SegmentWriter::begin(storage, self.document.clone(), new_index, Some(chain_hash), now_ms).await?;
        self.active = active;
        self.next_segment_index = following_index;
        Ok(())
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        self.active.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.active.terminal_is_empty()
    }
}
//#endregion 🔖️ArtifactWal

//#region 🧪️Tests
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use db_storage::{MemoryStorage, WalStorage};
    use {ArtifactId, DurabilityClass, Frontier};

    fn pages(bytes: &[u8]) -> db_storage::DbIoPages {
        let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).expect("test WAL writer admitted");
        for fragment in bytes.chunks(db_storage::DB_IO_PAGE_BYTES) {
            assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
        }
        writer.finish().unwrap()
    }

    async fn doc(id: &str) -> ArtifactId {
        ArtifactId::from(id)
    }

    async fn sample_frontier(document: &ArtifactId) -> Frontier {
        Frontier { document: document.clone(), head_seq: 7, commit_seq: 3, chain_hash: [9u8; 32], epoch: 1 }
    }

    fn control() -> WalCursorControl {
        WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap()
    }

    async fn retained(source: &[u8]) -> WalBytes {
        let mut control = control();
        WalBytes::try_admit(source.to_vec(), MAX_FIELD_BYTES, &mut control).await.unwrap()
    }

    async fn read_retained(bytes: &WalBytes) -> Vec<u8> {
        let mut output = Vec::with_capacity(bytes.len());
        for fragment in bytes.fragments() {
            output.extend_from_slice(fragment);
            semio_framework_async::yield_once().await;
        }
        assert_eq!(output.len(), bytes.len());
        output
    }

    async fn decode(kind: u8, payload: &[u8]) -> Result<WalRecord, DbError> {
        let carrier = retained(payload).await;
        let operation = carrier.operation();
        let mut control = control();
        let decoded = WalRecord::decode_retained(operation, kind, payload, &mut control).await;
        let mut carrier = carrier;
        while carrier.close_step()?.is_some() {}
        decoded
    }

    fn run_ids(values: &[u64]) -> db_storage::DbIoU64List {
        let mut list = db_storage::DbIoU64List::new();
        for value in values {
            list.push(*value).unwrap();
        }
        list
    }

    async fn submit_one(storage: &impl WalStorage, wal: &mut ArtifactWal, record: WalRecord, durability: DurabilityClass, now_ms: u64) -> WalAppendReceipt {
        let mut records = WalRecordBatch::new();
        assert!(records.push(record).is_ok());
        let receipt = wal.submit(storage, &records, durability, now_ms).await.unwrap();
        while records.close_step().unwrap() {}
        receipt
    }

    #[derive(Debug, PartialEq, Eq)]
    enum ReplaySummary {
        Segment(u64, Option<[u8; 32]>),
        Begin(u64),
        Command(Vec<u8>),
        Commit(u64, u32),
        Abort(u64),
        Other(u8),
    }

    async fn replay_summaries(storage: &impl WalStorage, document: &ArtifactId) -> Vec<ReplaySummary> {
        let mut replay = replay_document(storage, document, control()).await.unwrap();
        let mut summaries = Vec::new();
        while let Some(mut record) = replay.next().await.unwrap() {
            let summary = match &record {
                WalRecord::SegmentHeader { segment_index, prev_chain_hash, .. } => ReplaySummary::Segment(*segment_index, *prev_chain_hash),
                WalRecord::TxBegin { tx_id } => ReplaySummary::Begin(*tx_id),
                WalRecord::Command(bytes) => ReplaySummary::Command(read_retained(bytes).await),
                WalRecord::TxCommit { tx_id, record_count } => ReplaySummary::Commit(*tx_id, *record_count),
                WalRecord::TxAbort { tx_id } => ReplaySummary::Abort(*tx_id),
                _ => ReplaySummary::Other(record.retained_shape().0),
            };
            summaries.push(summary);
            while record.close_step().unwrap() {}
        }
        while replay.close_step().await.unwrap() {}
        summaries
    }

    async fn segment_bytes(storage: &impl WalStorage, document: &ArtifactId, index: u64) -> Vec<u8> {
        let len = storage.segment_len(document, index).await.unwrap();
        let mut pages = storage.read(document, index, pack::ByteRange { offset: 0, len }).await.unwrap();
        let mut prepared = db_storage::db_io_prepare_platform(&pages).unwrap().await.unwrap();
        let output = prepared.as_slice().to_vec();
        while prepared.close_step().unwrap() {}
        while pages.close_step().unwrap().is_some() {}
        output
    }

    fn recovery_fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️fixtures/🚑️recovery/🔣️.json")).unwrap()
    }

    pub(crate) async fn committed_fixture_storage(row: &serde_json::Value, document: &ArtifactId) -> MemoryStorage {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        write_committed_fixture(&storage, row, document).await;
        storage
    }

    async fn write_committed_fixture(storage: &impl WalStorage, row: &serde_json::Value, document: &ArtifactId) {
        let mut previous = None;
        for (index, segment) in row["segments"].as_array().unwrap().iter().enumerate() {
            let mut writer = SegmentWriter::begin(storage, document.clone(), index as u64, previous, 0).await.unwrap();
            for (ordinal, frame) in segment["frames"].as_array().unwrap().iter().enumerate().skip(1) {
                let id = || frame["id"].as_str().unwrap().parse::<u64>().unwrap();
                let mut record = match frame["kind"].as_str().unwrap() {
                    "header" => WalRecord::SegmentHeader { document: document.clone(), segment_index: index as u64, prev_chain_hash: previous },
                    "begin" => WalRecord::TxBegin { tx_id: id() },
                    "commit" => WalRecord::TxCommit { tx_id: id(), record_count: frame["count"].as_u64().unwrap() as u32 },
                    "abort" => WalRecord::TxAbort { tx_id: id() },
                    "command" => WalRecord::Command(retained(&[ordinal as u8]).await),
                    "frontier" => WalRecord::Frontier(sample_frontier(document).await),
                    "snapshot" => WalRecord::SnapshotPub { generation: 1, frontier: sample_frontier(document).await },
                    "cas" => WalRecord::Payload(WalPayloadRef::CasRef(ContentHash([7; 32]))),
                    other => panic!("unknown committed fixture kind {other}"),
                };
                writer.append_record(&record, 0).await.unwrap();
                while record.close_step().unwrap() {}
                if segment["physicalCommitsAfter"].as_array().unwrap().iter().any(|value| value.as_u64() == Some(ordinal as u64)) {
                    writer.commit_and_flush(storage, DurabilityClass::Fsync).await.unwrap();
                }
            }
            previous = Some(writer.tip_chain_hash().await.unwrap());
            while writer.close_step().unwrap() {}
            if segment["state"] == "sealed" { storage.seal(document, index as u64).await.unwrap(); }
        }
    }

    async fn assert_no_committed_transaction(storage: &impl WalStorage, document: &ArtifactId) {
        let mut cursor = replay_committed_document(storage, document, control()).await.unwrap();
        let found = loop {
            match cursor.next_transaction_step().await.unwrap() {
                WalCommittedStep::Yield => {}
                WalCommittedStep::Done => break false,
                WalCommittedStep::Transaction(transaction) => { drop(transaction); break true; }
            }
        };
        while cursor.close_owner_step().unwrap() { semio_framework_async::yield_once().await; }
        assert!(cursor.terminal_is_empty());
        assert!(!found, "recovery made an incomplete transaction visible");
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_recovery_aborts_only_incomplete_active_transactions_idempotently() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        for name in ["active-incomplete-needs-durable-abort", "active-empty-begin-needs-durable-abort", "sealed-incomplete-is-corrupt", "cross-segment-open-transaction-is-corrupt"] {
            let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == name).unwrap();
            let document = ArtifactId::from("abort-recovery");
            let storage = committed_fixture_storage(row, &document).await;
            let mut before = Vec::new();
            for index in 0..row["segments"].as_array().unwrap().len() { before.push((segment_bytes(&storage, &document, index as u64).await, storage.segment_state(&document, index as u64).await.unwrap())); }
            let opened = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 1).await;
            if row["expected"]["accepted"] == false {
                let rejected = match opened {
                    Err(DbError::Corrupt(_)) => true,
                    Ok((mut wal, _)) => { while wal.close_step().unwrap() {} false },
                    Err(_) => false,
                };
                assert!(rejected, "{name}");
                for (index, (bytes, state)) in before.iter().enumerate() {
                    assert_eq!(segment_bytes(&storage, &document, index as u64).await, *bytes, "{name}");
                    assert_eq!(storage.segment_state(&document, index as u64).await.unwrap(), *state, "{name}");
                }
                continue;
            }
            let expected_abort: u64 = row["expected"]["recoverAbort"].as_str().unwrap().parse().unwrap();
            let next: u64 = row["expected"]["nextTxId"].as_str().unwrap().parse().unwrap();
            let (mut wal, report) = opened.unwrap();
            assert_eq!(report.recovered_abort_tx_id, Some(expected_abort));
            assert_eq!(wal.next_tx_id, next);
            assert_eq!(wal.active.index, 0);
            assert_eq!(report.torn_tail_bytes, 0);
            let repaired = segment_bytes(&storage, &document, 0).await;
            assert!(repaired.starts_with(&before[0].0));
            assert_eq!(repaired.len() as u64, before[0].0.len() as u64 + wal_frame_bytes(8).unwrap() + protocol::format::COMMIT_FRAME_LEN);
            assert_eq!(replay_summaries(&storage, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(expected_abort)).count(), 1);
            assert_no_committed_transaction(&storage, &document).await;
            while wal.close_step().unwrap() {}
            let (mut reopened, report) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 2).await.unwrap();
            assert_eq!(report.recovered_abort_tx_id, None);
            assert_eq!(segment_bytes(&storage, &document, 0).await, repaired);
            let receipt = submit_one(&storage, &mut reopened, WalRecord::Command(retained(b"after-recovery").await), DurabilityClass::Fsync, 3).await;
            assert_eq!(receipt.tx_id, next);
            while reopened.close_step().unwrap() {}
            eprintln!("[DEBUG] active WAL recovery appended exactly one durable abort and preserved byte identity on reopen: {name}");
        }
    }

    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    #[semio_framework_async_macros::async_test]
    async fn wal_recovery_abort_fsync_survives_two_independent_filesystem_reopens() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        for (ordinal, name) in ["active-incomplete-needs-durable-abort", "active-empty-begin-needs-durable-abort"].iter().enumerate() {
            let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == *name).unwrap();
            let document = ArtifactId::from("abort-filesystem");
            let expected_abort = row["expected"]["recoverAbort"].as_str().unwrap().parse::<u64>().unwrap();
            let next = row["expected"]["nextTxId"].as_str().unwrap().parse::<u64>().unwrap();
            let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
            let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            let root = base.join(format!("wal-active-abort-{}-{nonce}-{ordinal}", std::process::id()));
            {
                let filesystem = db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &root).await.unwrap();
                write_committed_fixture(&filesystem, row, &document).await;
                assert_eq!(filesystem.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Active);
                filesystem.close().await.unwrap();
            }
            let repaired = {
                let filesystem = db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &root).await.unwrap();
                let (mut wal, report) = ArtifactWal::open(&filesystem, document.clone(), GroupCommitPolicy::default(), 1).await.unwrap();
                assert_eq!(report.recovered_abort_tx_id, Some(expected_abort));
                assert_eq!(report.torn_tail_bytes, 0);
                assert_eq!(wal.next_tx_id, next);
                assert_eq!(replay_summaries(&filesystem, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(expected_abort)).count(), 1);
                assert_no_committed_transaction(&filesystem, &document).await;
                let bytes = segment_bytes(&filesystem, &document, 0).await;
                while wal.close_step().unwrap() {}
                filesystem.close().await.unwrap();
                bytes
            };
            {
                let filesystem = db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &root).await.unwrap();
                let (mut wal, report) = ArtifactWal::open(&filesystem, document.clone(), GroupCommitPolicy::default(), 2).await.unwrap();
                assert_eq!(report.recovered_abort_tx_id, None);
                assert_eq!(report.torn_tail_bytes, 0);
                assert_eq!(segment_bytes(&filesystem, &document, 0).await, repaired);
                assert_eq!(filesystem.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Active);
                assert_eq!(replay_summaries(&filesystem, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(expected_abort)).count(), 1);
                assert_no_committed_transaction(&filesystem, &document).await;
                let receipt = submit_one(&filesystem, &mut wal, WalRecord::Command(retained(b"after-filesystem-recovery").await), DurabilityClass::Fsync, 3).await;
                assert_eq!(receipt.tx_id, next);
                while wal.close_step().unwrap() {}
                filesystem.close().await.unwrap();
            }
            eprintln!("[DEBUG] one durable WAL abort survived independent filesystem retirement and reopen, preserving the next transaction id: {name}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_recovery_abort_faults_retry_without_duplicate_abort() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "active-incomplete-needs-durable-abort").unwrap();
        let faults: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🛑️fail-stop/🔣️.json")).unwrap();
        for case in faults["cases"].as_array().unwrap().iter().filter(|case| case["fault"] != "successorAppendError") {
            for (tail, fail_tail_sync) in [(false, false), (true, false), (true, true)] {
            if fail_tail_sync && case["fault"] != "syncError" { continue; }
            let document = ArtifactId::from("abort-fault");
            let inner = std::sync::Arc::new(db_storage::DbBackend::Memory(committed_fixture_storage(row, &document).await));
            let storage = crate::db_testkit::FaultStorage::new(inner.clone()).await;
            let baseline = segment_bytes(&storage, &document, 0).await;
            if tail { storage.append(&document, 0, pages(b"uncommitted-tail")).await.unwrap(); }
            let append_boundary = storage.append_calls().await + 1;
            let sync_boundary = storage.sync_calls().await + if tail && !fail_tail_sync { 2 } else { 1 };
            let mut script = crate::db_testkit::FaultScript::default();
            match case["fault"].as_str().unwrap() {
                "shortAppend" => script.torn_write_at = Some((append_boundary, case["keepBytes"].as_u64().unwrap())),
                "appendError" => script.fail_nth_write = Some(append_boundary),
                "syncError" => script.fail_nth_sync = Some(sync_boundary),
                _ => unreachable!(),
            }
            storage.set_script(script).await;
            let error = match ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 1).await {
                Err(error) => error,
                Ok((mut wal, _)) => { while wal.close_step().unwrap() {} panic!("abort recovery ignored injected fault"); }
            };
            match case["expectedError"].as_str().unwrap() { "Corrupt" => assert!(matches!(error, DbError::Corrupt(_))), "Io" => assert!(matches!(error, DbError::Io(_))), _ => unreachable!() }
            let failed = segment_bytes(&storage, &document, 0).await;
            let suffix = if fail_tail_sync { "absent" } else { case["expectedPhysicalSuffix"].as_str().unwrap() };
            match suffix {
                "absent" => assert_eq!(failed, baseline),
                "torn" => { assert!(failed.starts_with(&baseline)); assert_eq!(failed.len(), baseline.len() + case["keepBytes"].as_u64().unwrap() as usize); }
                "complete" => assert!(failed.len() > baseline.len() && failed.starts_with(&baseline)),
                _ => unreachable!(),
            }
            let facet = inner.wal().await;
            let (mut wal, report) = ArtifactWal::open(&facet, document.clone(), GroupCommitPolicy::default(), 2).await.unwrap();
            let repaired = segment_bytes(&facet, &document, 0).await;
            if suffix == "complete" { assert_eq!(repaired, failed); assert_eq!(report.recovered_abort_tx_id, None); }
            else { assert_eq!(report.recovered_abort_tx_id, Some(7)); }
            assert_no_committed_transaction(&facet, &document).await;
            assert_eq!(replay_summaries(&facet, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(7)).count(), 1);
            while wal.close_step().unwrap() {}
            let (mut wal, report) = ArtifactWal::open(&facet, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
            assert_eq!(report.recovered_abort_tx_id, None);
            assert_eq!(segment_bytes(&facet, &document, 0).await, repaired);
            while wal.close_step().unwrap() {}
            eprintln!("[DEBUG] WAL abort fault retired owners and reopened idempotently: {}, tail={tail}, fail_tail_sync={fail_tail_sync}", case["name"]);
            }
        }
    }

    struct AbortCancellationStorage<'a> {
        inner: &'a MemoryStorage,
        cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
        cancel_on_truncate: bool,
        synced: std::sync::atomic::AtomicUsize,
    }

    impl WalStorage for AbortCancellationStorage<'_> {
        async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> { self.inner.create_segment(document, index).await }
        async fn append(&self, document: &ArtifactId, index: u64, bytes: db_storage::DbIoPages) -> Result<u64, DbError> {
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.inner.append(document, index, bytes).await
        }
        async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
            let result = self.inner.sync(document, index, class).await;
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) && class == DurabilityClass::Fsync && result.is_ok() { self.synced.fetch_add(1, std::sync::atomic::Ordering::AcqRel); }
            result
        }
        async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> { self.inner.seal(document, index).await }
        async fn read(&self, document: &ArtifactId, index: u64, range: pack::ByteRange) -> Result<db_storage::DbIoPages, DbError> { self.inner.read(document, index, range).await }
        async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> { self.inner.segment_len(document, index).await }
        async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<db_storage::WalSegmentState, DbError> { self.inner.segment_state(document, index).await }
        async fn list_segments(&self, document: &ArtifactId) -> Result<db_storage::DbIoU64List, DbError> { self.inner.list_segments(document).await }
        async fn truncate_tail(&self, document: &ArtifactId, index: u64, len: u64) -> Result<(), DbError> {
            if self.cancel_on_truncate { self.cancelled.store(true, std::sync::atomic::Ordering::Release); }
            self.inner.truncate_tail(document, index, len).await
        }
        async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> { self.inner.delete_segment(document, index).await }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_recovery_abort_cancellation_has_one_durable_boundary() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "active-incomplete-needs-durable-abort").unwrap();
        let document = ArtifactId::from("abort-cancel");
        let storage = committed_fixture_storage(row, &document).await;
        storage.append(&document, 0, pages(b"uncommitted-tail")).await.unwrap();
        let before = segment_bytes(&storage, &document, 0).await;
        for mode in ["cancelled", "expired", "fuel"] {
            let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(mode == "cancelled"));
            let deadline = if mode == "expired" { std::time::Instant::now() } else { std::time::Instant::now() + std::time::Duration::from_secs(30) };
            let mut control = WalCursorControl::new(cancelled, deadline, if mode == "fuel" { 1 } else { 1_000_000 }).unwrap();
            let rejected = match ArtifactWal::open_with_control(&storage, document.clone(), GroupCommitPolicy::default(), 1, &mut control).await {
                Err(DbError::Unavailable(_)) | Err(DbError::LimitExceeded("wal cursor fuel")) => true,
                Ok((mut wal, _)) => { while wal.close_step().unwrap() {} false },
                Err(_) => false,
            };
            assert!(rejected, "{mode}");
            assert_eq!(segment_bytes(&storage, &document, 0).await, before);
        }
        for cancel_on_truncate in [false, true] {
        let storage = committed_fixture_storage(row, &document).await;
        storage.append(&document, 0, pages(b"uncommitted-tail")).await.unwrap();
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let boundary = AbortCancellationStorage { inner: &storage, cancelled: cancelled.clone(), cancel_on_truncate, synced: std::sync::atomic::AtomicUsize::new(0) };
        let mut control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
        let (mut wal, report) = ArtifactWal::open_with_control(&boundary, document.clone(), GroupCommitPolicy::default(), 2, &mut control).await.unwrap();
        assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
        assert_eq!(boundary.synced.load(std::sync::atomic::Ordering::Acquire), if cancel_on_truncate { 2 } else { 1 });
        assert_eq!(report.recovered_abort_tx_id, Some(7));
        assert_eq!(report.torn_tail_bytes, b"uncommitted-tail".len() as u64);
        let repaired = segment_bytes(&storage, &document, 0).await;
        while wal.close_step().unwrap() {}
        let (mut reopened, report) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
        assert_eq!(report.recovered_abort_tx_id, None);
        assert_eq!(segment_bytes(&storage, &document, 0).await, repaired);
        while reopened.close_step().unwrap() {}
        assert_no_committed_transaction(&storage, &document).await;
        eprintln!("[DEBUG] WAL abort recovery rejected pre-boundary cancellation without writes and completed admitted Fsync, cancel_on_truncate={cancel_on_truncate}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_recovery_abort_capacity_exact_and_plus_one_preserves_source() {
        let suffix = wal_frame_bytes(8).unwrap() + protocol::format::COMMIT_FRAME_LEN;
        for extra in [0, 1] {
            let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
            let document = ArtifactId::from("abort-capacity");
            let mut writer = SegmentWriter::begin(&storage, document.clone(), 0, None, 0).await.unwrap();
            writer.append_record(&WalRecord::TxBegin { tx_id: 7 }, 0).await.unwrap();
            let end = db_storage::DB_IO_MAX_READ_BYTES - suffix + extra;
            let available = end - writer.total_len().unwrap() - protocol::format::COMMIT_FRAME_LEN;
            let payload = (0..32).map(|overhead| available as usize - overhead).find(|bytes| wal_frame_bytes(*bytes).unwrap() == available).unwrap();
            let mut command = WalRecord::Command(retained(&vec![0xa5; payload]).await);
            writer.append_record(&command, 0).await.unwrap();
            while command.close_step().unwrap() {}
            writer.commit_and_flush(&storage, DurabilityClass::Fsync).await.unwrap();
            while writer.close_step().unwrap() {}
            let before = segment_bytes(&storage, &document, 0).await;
            assert_eq!(before.len() as u64, end);
            let result = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 1).await;
            if extra == 0 {
                let (mut wal, report) = result.unwrap();
                assert_eq!(report.recovered_abort_tx_id, Some(7));
                assert_eq!(storage.segment_len(&document, 0).await.unwrap(), db_storage::DB_IO_MAX_READ_BYTES);
                assert_no_committed_transaction(&storage, &document).await;
                let tip = wal.active.tip_chain_hash().await.unwrap();
                let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"successor").await), DurabilityClass::Fsync, 2).await;
                assert_eq!((receipt.segment_index, receipt.tx_id), (1, 8));
                assert_eq!(storage.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Sealed);
                while wal.close_step().unwrap() {}
                let (mut reopened, report) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
                assert_eq!(report.recovered_abort_tx_id, None);
                let replay = replay_summaries(&storage, &document).await;
                assert!(replay.contains(&ReplaySummary::Segment(1, Some(tip))));
                assert_eq!(replay.iter().filter(|record| **record == ReplaySummary::Abort(7)).count(), 1);
                assert!(replay.contains(&ReplaySummary::Command(b"successor".to_vec())));
                while reopened.close_step().unwrap() {}
            } else {
                let rejected = match result {
                    Err(DbError::LimitExceeded("wal recovery abort exceeds retained segment budget")) => true,
                    Ok((mut wal, _)) => { while wal.close_step().unwrap() {} false },
                    Err(_) => false,
                };
                assert!(rejected);
                assert_eq!(segment_bytes(&storage, &document, 0).await, before);
                assert_eq!(storage.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Active);
            }
            eprintln!("[DEBUG] WAL abort recovery exact retained capacity plus {extra} had the expected byte-preserving disposition");
        }
    }

    fn committed_fixture_kind(kind: u8) -> &'static str {
        match kind { WAL_COMMAND => "command", WAL_FRONTIER => "frontier", WAL_SNAPSHOT_PUB => "snapshot", WAL_PAYLOAD => "cas", _ => panic!("unexpected fixture body kind") }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_transaction_gate_matches_neutral_committed_spans() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let document = ArtifactId::from("committed-fixture");
            let storage = committed_fixture_storage(row, &document).await;
            let mut gate = WalTransactionGate::new();
            let mut transactions = Vec::new();
            let mut recovery_abort = None;
            let mut failure = None;
            for (index, segment) in row["segments"].as_array().unwrap().iter().enumerate() {
                let bytes = segment_bytes(&storage, &document, index as u64).await;
                let mut source = pages(&bytes);
                let mut offset = protocol::format::HEADER_SIZE;
                let result: Result<(), DbError> = async {
                    let prior = if index == 0 { WalPriorChainTip::Genesis } else { WalPriorChainTip::RetainedBoundary };
                    let mut chain = WalSegmentChain::new(&source, index as u64, prior)?;
                    while !chain.step(&source, bytes.len(), &document, &mut control())? {}
                    loop {
                        let frame = match wal_next_verified_page_frame(&source, &mut offset, bytes.len(), &mut control())? {
                            WalVerifiedFrameStep::Frame(frame) => frame,
                            WalVerifiedFrameStep::PhysicalCommit => continue,
                            WalVerifiedFrameStep::Done => break,
                        };
                        if gate.push(&source, frame)? {
                            let kinds: Vec<_> = gate.frames[..gate.frames_len as usize].iter().map(|frame| committed_fixture_kind(frame.unwrap().kind)).collect();
                            transactions.push(serde_json::json!({ "id": gate.ready.unwrap().to_string(), "kinds": kinds }));
                            gate.release()?;
                        }
                        offset = frame.frame_end;
                    }
                    recovery_abort = gate.finish_segment(segment["state"] == "active" && index + 1 == row["segments"].as_array().unwrap().len())?;
                    if index + 1 != row["segments"].as_array().unwrap().len() { gate.advance_segment()?; }
                    Ok(())
                }.await;
                while source.close_step().unwrap().is_some() {}
                if let Err(error) = result {
                    failure = Some(match error { DbError::LimitExceeded("wal transaction records") => "capacity", DbError::LimitExceeded("wal transaction sequence") => "sequence", DbError::Corrupt(_) => "corrupt", other => panic!("unexpected fixture error: {other:?}") });
                    break;
                }
            }
            let actual = if let Some(error) = failure {
                serde_json::json!({ "accepted": false, "transactions": [], "nextTxId": null, "recoverAbort": null, "error": error })
            } else {
                serde_json::json!({ "accepted": true, "transactions": transactions, "nextTxId": gate.next_tx_id.to_string(), "recoverAbort": recovery_abort.map(|id| id.to_string()), "error": null })
            };
            assert_eq!(actual, row["expected"], "{}", row["name"]);
            if failure.is_none() && recovery_abort.is_none() {
                let mut cursor = replay_committed_document(&storage, &document, control()).await.unwrap();
                let mut replayed = Vec::new();
                loop {
                    match cursor.next_transaction_step().await.unwrap() {
                        WalCommittedStep::Transaction(mut transaction) => {
                            let id = transaction.transaction_id().to_string();
                            let mut kinds = Vec::new();
                            loop {
                                match transaction.next_record_step().unwrap() {
                                    WalCommittedRecordStep::Record(record) => { kinds.push(committed_fixture_kind(record.retained_shape().0)); }
                                    WalCommittedRecordStep::Yield => continue,
                                    WalCommittedRecordStep::Done => break,
                                }
                                while transaction.close_record_step().unwrap() {}
                            }
                            transaction.finish().unwrap();
                            replayed.push(serde_json::json!({ "id": id, "kinds": kinds }));
                        }
                        WalCommittedStep::Yield => {}
                        WalCommittedStep::Done => break,
                    }
                }
                while cursor.close_owner_step().unwrap() {}
                assert!(cursor.terminal_is_empty());
                assert_eq!(serde_json::json!(replayed), row["expected"]["transactions"], "{}", row["name"]);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_immutable_source_fragmentation_matches_neutral_transactions() {
        struct Fragments<'a> { bytes: &'a [u8], chunk: usize }
        impl WalImmutableByteSource for Fragments<'_> {
            fn byte_len(&self) -> usize { self.bytes.len() }
            fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
                if offset >= limit || limit > self.bytes.len() { return Err(DbError::Corrupt("fixture immutable range".to_string())); }
                Ok(&self.bytes[offset..limit.min((offset / self.chunk + 1) * self.chunk)])
            }
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["expected"]["accepted"] == true && row["expected"]["recoverAbort"].is_null()) {
            let document = ArtifactId::from("committed-fragmented-source");
            let storage = committed_fixture_storage(row, &document).await;
            for chunk in [1, 7, 4_093, 4_096, 0] {
                let mut gate = WalTransactionGate::new();
                let mut previous = WalPriorChainTip::Genesis;
                let mut output = Vec::new();
                for index in 0..row["segments"].as_array().unwrap().len() {
                    let bytes = segment_bytes(&storage, &document, index as u64).await;
                    let fragments = Fragments { bytes: &bytes, chunk: chunk.max(1) };
                    let mut ranged = if chunk == 0 {
                        let mut padded = vec![0xad; 4_093];
                        padded.extend_from_slice(&bytes);
                        padded.extend_from_slice(&[0xda; 19]);
                        let mut writer = db_storage::DbIoPageWriter::try_reserve(padded.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).unwrap();
                        for fragment in padded.chunks(db_storage::DB_IO_PAGE_BYTES) { assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len()); }
                        let pages = writer.finish().unwrap();
                        let pages = pages.try_range(4_093).unwrap_or_else(|_| panic!("fixture range must retain its exact WAL"));
                        Some(pages.try_prefix(bytes.len()).unwrap_or_else(|_| panic!("fixture prefix must exclude trailing padding")))
                    } else { None };
                    let source: &dyn WalImmutableByteSource = match ranged.as_ref() { Some(pages) => pages, None => &fragments };
                    assert_eq!(source.byte_len(), bytes.len());
                    assert!(source.fragment_at(bytes.len(), bytes.len()).is_err());
                    assert!(source.fragment_at(0, bytes.len() + 1).is_err());
                    let owner = WalAuthenticatedSource::new(source, gate, index as u64, previous);
                    let mut owner = match owner.finish() { Err(owner) => owner, Ok(_) => panic!("unverified source must retain its ownership") };
                    let mut opportunity = control();
                    assert!(matches!(owner.next_step(&mut opportunity), Err(DbError::Corrupt(_))));
                    let mut turns = 0;
                    loop {
                        turns += 1;
                        assert!(turns <= bytes.len() + 2);
                        opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                        if owner.verify_step(&document, &mut opportunity).unwrap() { break; }
                    }
                    loop {
                        opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                        match owner.next_step(&mut opportunity).unwrap() {
                            WalAuthenticatedStep::Committed => {
                                let mut kinds = Vec::new();
                                for body in 0..64 {
                                    let Some(frame) = owner.committed_frame(body) else { break; };
                                    kinds.push(committed_fixture_kind(frame.kind));
                                }
                                output.push(serde_json::json!({ "id": owner.gate.ready.unwrap().to_string(), "kinds": kinds }));
                                assert!(matches!(owner.next_step(&mut opportunity), Err(DbError::Corrupt(_))));
                                owner.finish_transaction().unwrap();
                            }
                            WalAuthenticatedStep::Yield => {}
                            WalAuthenticatedStep::Done => break,
                        }
                    }
                    let (_, next_gate, tip) = owner.finish().unwrap_or_else(|_| panic!("drained authenticated source must transfer ownership"));
                    gate = next_gate;
                    previous = WalPriorChainTip::Verified(tip);
                    if let Some(pages) = ranged.as_mut() {
                        while !pages.terminal_is_empty() { pages.close_step().unwrap(); }
                        assert!(pages.terminal_is_empty());
                    }
                }
                assert_eq!(serde_json::json!(output), row["expected"]["transactions"], "{}: chunk={chunk}", row["name"]);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_committed_cursor_single_fuel_and_expired_turns_match_neutral_transactions() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["expected"]["accepted"] == true && row["expected"]["recoverAbort"].is_null()) {
            let document = ArtifactId::from("committed-one-fuel");
            let storage = committed_fixture_storage(row, &document).await;
            let mut cursor = replay_committed_document(&storage, &document, control()).await.unwrap();
            let mut output = Vec::new();
            let mut turns = 0;
            loop {
                turns += 1;
                assert!(turns < 4_096, "single-fuel replay stopped progressing: {}", row["name"]);
                cursor.replenish(std::time::Instant::now(), 1).unwrap();
                assert!(matches!(cursor.next_transaction_step().await, Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached"));
                cursor.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                match cursor.next_transaction_step().await.unwrap() {
                    WalCommittedStep::Transaction(mut transaction) => {
                        let id = transaction.transaction_id().to_string();
                        let mut kinds = Vec::new();
                        loop {
                            transaction.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                            match transaction.next_record_step().unwrap() {
                                WalCommittedRecordStep::Record(record) => kinds.push(committed_fixture_kind(record.retained_shape().0)),
                                WalCommittedRecordStep::Yield => continue,
                                WalCommittedRecordStep::Done => break,
                            }
                            while transaction.close_record_step().unwrap() {}
                        }
                        transaction.finish().unwrap();
                        output.push(serde_json::json!({ "id": id, "kinds": kinds }));
                    }
                    WalCommittedStep::Yield => {}
                    WalCommittedStep::Done => break,
                }
            }
            while cursor.close_owner_step().unwrap() {}
            assert!(cursor.terminal_is_empty());
            assert_eq!(serde_json::json!(output), row["expected"]["transactions"], "{}", row["name"]);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_retained_varints_match_neutral_exact_u64_and_atomic_interruption() {
        let _pool = crate::db_storage::db_io_test_pool();
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/📖️retained-decoder/🔣️.json")).unwrap();
        for row in fixture["varints"].as_array().unwrap() {
            let hex = row["hex"].as_str().unwrap();
            let input: Vec<_> = (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()).collect();
            let mut bytes = WalBytes { pages: pages(&input) };
            let expected = row["value"].as_str().map(|value| value.parse::<u64>().unwrap());
            let mut reader = WalPageReader::new(&bytes.pages, 0, input.len()).unwrap();
            let parsed = reader.varint();
            assert_eq!(parsed.as_ref().ok().copied(), expected, "{}", row["name"]);
            assert_eq!(reader.position, row["consumed"].as_u64().unwrap_or(0) as usize, "{}", row["name"]);
            let mut cursor = bytes.cursor();
            let parsed = cursor.varint(&mut control());
            assert_eq!(parsed.as_ref().ok().copied(), expected, "{}", row["name"]);
            assert_eq!(cursor.offset, row["consumed"].as_u64().unwrap_or(0) as usize, "{}", row["name"]);
            if row["consumed"].as_u64().is_some_and(|count| count > 1) {
                let mut cursor = bytes.cursor();
                let mut opportunity = control();
                opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                assert!(matches!(cursor.varint(&mut opportunity), Err(DbError::LimitExceeded("wal cursor fuel"))));
                assert_eq!(cursor.offset, 0);
                opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 16).unwrap();
                assert_eq!(cursor.varint(&mut opportunity).unwrap(), expected.unwrap());
            }
            while bytes.close_step().unwrap().is_some() {}
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_retained_decoder_fuel_resumes_exact_fragmented_bytes() {
        let _pool = crate::db_storage::db_io_test_pool();
        let expected: Vec<_> = (0..db_storage::DB_IO_PAGE_BYTES * 3 + 17).map(|index| (index % 251) as u8).collect();
        let mut source = pages(&expected);
        let frame = WalRecordFrame { kind: WAL_COMMAND, payload_start: 0, payload_end: expected.len(), frame_end: expected.len() };
        let mut decoder = WalRetainedRecordDecoder::new(frame);
        let mut opportunity = control();
        let mut attempts = 0;
        let mut record = loop {
            opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
            match decoder.step(&source, &mut opportunity).unwrap() {
                Some(record) => break record,
                None => assert!(matches!(decoder.step(&source, &mut opportunity), Err(DbError::LimitExceeded("wal cursor fuel")))),
            }
            attempts += 1;
            assert!(attempts < 1_024);
        };
        assert!(attempts > 3);
        let WalRecord::Command(bytes) = &record else { panic!("expected command") };
        assert_eq!(read_retained(bytes).await, expected);
        while record.close_step().unwrap() {}
        while decoder.close_owner_step().unwrap() {}
        assert!(decoder.terminal_is_empty());
        while source.close_step().unwrap().is_some() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_retained_decoder_cancel_close_preserves_source_and_returns_owner() {
        let _pool = crate::db_storage::db_io_test_pool();
        let expected = vec![0x57; db_storage::DB_IO_PAGE_BYTES + 7];
        let mut source = pages(&expected);
        for cut in 1..=4 {
            let mut decoder = WalRetainedRecordDecoder::new(WalRecordFrame { kind: WAL_COMMAND, payload_start: 0, payload_end: expected.len(), frame_end: expected.len() });
            let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let mut opportunity = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
            for _ in 0..cut { assert!(decoder.step(&source, &mut opportunity).unwrap().is_none()); }
            cancelled.store(true, std::sync::atomic::Ordering::Release);
            assert!(matches!(decoder.step(&source, &mut opportunity), Err(DbError::Unavailable(message)) if message == "wal cursor cancelled"));
            while decoder.close_owner_step().unwrap() { assert!(cancelled.load(std::sync::atomic::Ordering::Acquire)); }
            assert!(decoder.terminal_is_empty());
            assert_eq!(source.len(), expected.len());
        }
        while source.close_step().unwrap().is_some() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_committed_cursor_cancel_resume_keeps_transaction_position() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "two-commands-only-after-logical-commit").unwrap();
        let document = ArtifactId::from("committed-cancel");
        let storage = committed_fixture_storage(row, &document).await;
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let opportunity = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut cursor = replay_committed_document(&storage, &document, opportunity).await.unwrap();
        loop {
            match cursor.next_transaction_step().await.unwrap() {
                WalCommittedStep::Transaction(mut transaction) => {
                    assert_eq!(transaction.record_count(), 2);
                    assert!(matches!(transaction.next_record_step().unwrap(), WalCommittedRecordStep::Yield));
                    let offset = transaction.cursor.raw.offset;
                    cancelled.store(true, std::sync::atomic::Ordering::Release);
                    assert!(matches!(transaction.next_record_step(), Err(DbError::Unavailable(message)) if message == "wal cursor cancelled"));
                    assert_eq!(transaction.cursor.raw.offset, offset);
                    assert_eq!(transaction.cursor.record_index, 0);
                    cancelled.store(false, std::sync::atomic::Ordering::Release);
                    let mut output = Vec::new();
                    loop {
                        match transaction.next_record_step().unwrap() {
                            WalCommittedRecordStep::Record(WalRecord::Command(bytes)) => output.push(read_retained(bytes).await),
                            WalCommittedRecordStep::Record(_) => panic!("expected command"),
                            WalCommittedRecordStep::Yield => continue,
                            WalCommittedRecordStep::Done => break,
                        }
                        while transaction.close_record_step().unwrap() {}
                    }
                    assert_eq!(output, vec![vec![2], vec![3]]);
                    transaction.finish().unwrap();
                    break;
                }
                WalCommittedStep::Yield => {}
                WalCommittedStep::Done => panic!("committed transaction was skipped"),
            }
        }
        while cursor.close_owner_step().unwrap() {}
        assert!(cursor.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_committed_cursor_unfinished_borrow_poison_and_cancelled_close() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "two-commands-only-after-logical-commit").unwrap();
        let document = ArtifactId::from("committed-drop");
        let storage = committed_fixture_storage(row, &document).await;
        for decoded in [false, true] {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let opportunity = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut cursor = replay_committed_document(&storage, &document, opportunity).await.unwrap();
        loop {
            match cursor.next_transaction_step().await.unwrap() {
                WalCommittedStep::Transaction(mut transaction) => {
                    assert!(matches!(transaction.next_record_step().unwrap(), WalCommittedRecordStep::Yield));
                    if decoded {
                        loop {
                            match transaction.next_record_step().unwrap() {
                                WalCommittedRecordStep::Record(WalRecord::Command(_)) => break,
                                WalCommittedRecordStep::Yield => {}
                                _ => panic!("unfinished-record fixture lost its first command"),
                            }
                        }
                    }
                    drop(transaction);
                    break;
                }
                WalCommittedStep::Yield => {}
                WalCommittedStep::Done => panic!("committed transaction was skipped"),
            }
        }
        assert!(matches!(cursor.next_transaction_step().await, Err(DbError::Corrupt(_))));
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        while cursor.close_owner_step().unwrap() { assert!(cancelled.load(std::sync::atomic::Ordering::Acquire)); }
        assert!(cursor.terminal_is_empty());
        }
    }

    async fn capacity_submission(storage: &impl WalStorage, wal: &mut ArtifactWal, length: usize, durability: DurabilityClass) -> Result<WalAppendReceipt, DbError> {
        let mut batch = WalRecordBatch::new();
        assert!(batch.push(WalRecord::Command(retained(&vec![b'a'; length]).await)).is_ok());
        let result = wal.submit(storage, &batch, durability, 0).await;
        while batch.close_step().unwrap() { semio_framework_async::yield_once().await; }
        result
    }

    async fn capacity_backend(storage: &impl WalStorage, fixture: &serde_json::Value, case: &serde_json::Value) {
        let document = doc("d").await;
        let policy = GroupCommitPolicy { max_delay_ms: u64::MAX, max_bytes: u64::MAX, max_records: u32::MAX };
        let mut wal = ArtifactWal::create(storage, document.clone(), policy, 0).await.unwrap();
        let genesis = segment_bytes(storage, &document, 0).await;
        let rejected = capacity_submission(storage, &mut wal, fixture["oversizedPayloadBytes"].as_u64().unwrap() as usize, DurabilityClass::Fsync).await;
        assert!(matches!(rejected, Err(DbError::LimitExceeded(_))));
        assert_eq!(wal.next_tx_id, 1);
        assert_eq!(wal.active.pending_records, 0);
        assert_eq!(segment_bytes(storage, &document, 0).await, genesis);
        let durability = if case["durability"] == "fsync" { DurabilityClass::Fsync } else { DurabilityClass::Memory };
        for (ordinal, expected) in case["segments"].as_array().unwrap().iter().enumerate() {
            let receipt = capacity_submission(storage, &mut wal, fixture["payloadBytes"].as_u64().unwrap() as usize, durability).await.unwrap();
            assert_eq!(receipt.segment_index, expected.as_u64().unwrap());
            assert_eq!(receipt.tx_id, ordinal as u64 + 1);
        }
        wal.force_flush(storage).await.unwrap();
        let before = segment_bytes(storage, &document, 0).await;
        for (index, length) in case["lengths"].as_array().unwrap().iter().enumerate() {
            assert_eq!(storage.segment_len(&document, index as u64).await.unwrap(), length.as_u64().unwrap());
        }
        while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
        let (mut reopened, _) = ArtifactWal::open(storage, document.clone(), policy, 1).await.unwrap();
        assert_eq!(reopened.next_tx_id, 4);
        assert_eq!(segment_bytes(storage, &document, 0).await, before);
        assert_eq!(replay_summaries(storage, &document).await.iter().filter(|record| matches!(record, ReplaySummary::Command(_))).count(), 3);
        while reopened.close_step().unwrap() { semio_framework_async::yield_once().await; }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/📏️capacity/🔣️.json")).unwrap();
        for (ordinal, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
            let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
            capacity_backend(&storage, &fixture, case).await;
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            {
                let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
                let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
                let root = base.join(format!("wal-capacity-{}-{nonce}-{ordinal}", std::process::id()));
                let filesystem = db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &root).await.unwrap();
                capacity_backend(&filesystem, &fixture, case).await;
                filesystem.close().await.unwrap();
            }
        }
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("d").await;
        let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
        capacity_submission(&storage, &mut wal, fixture["exactPayloadBytes"].as_u64().unwrap() as usize, DurabilityClass::Fsync).await.unwrap();
        assert_eq!(storage.segment_len(&document, 0).await.unwrap(), fixture["maxSegmentBytes"].as_u64().unwrap());
        while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
        let (mut wal, _) = ArtifactWal::open(&storage, document, GroupCommitPolicy::default(), 0).await.unwrap();
        while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
        println!("[DEBUG] WAL capacity: Memory and filesystem Fsync/grouped submissions rotate before overflow, reject one-over without effects and reopen the exact maximum");
    }

    async fn recovery_seed(storage: &MemoryStorage, document: &ArtifactId) -> ArtifactWal {
        let mut wal = ArtifactWal::create(storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
        for (index, command) in recovery_fixture()["commands"].as_array().unwrap().iter().enumerate() {
            submit_one(storage, &mut wal, WalRecord::Command(retained(command.as_str().unwrap().as_bytes()).await), DurabilityClass::Fsync, index as u64 + 1).await;
        }
        wal
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_recovery_preserves_neutral_committed_prefixes() {
        let fixture = recovery_fixture();
        let input: Vec<u8> = (0..49_152).map(|index| ((index * 17 + 3) % 251) as u8).collect();
        let mut source = SharedBuf::try_new().unwrap();
        pack::PackSink::write_all(&mut source, &input).await.unwrap();
        for row in fixture["fragmentCopies"].as_array().unwrap() {
            let offset = row["offset"].as_u64().unwrap() as usize;
            let length = row["length"].as_u64().unwrap() as usize;
            let mut copied = source.copy_range(offset, length).await.unwrap();
            assert_eq!(wal_crc_range(&copied, 0, length, &mut control()).unwrap(), row["crc32c"].as_u64().unwrap() as u32);
            let prefix = copy_verified_prefix(&copied, length as u64, &mut control()).await.unwrap();
            let mut actual = vec![0; length];
            prefix.read_exact(0, &mut actual).await.unwrap();
            assert_eq!(actual, input[offset..offset + length]);
            while lock(&prefix.0).close_step().unwrap().is_some() { semio_framework_async::yield_once().await; }
            while copied.close_step().unwrap().is_some() { semio_framework_async::yield_once().await; }
        }
        while lock(&source.0).close_step().unwrap().is_some() { semio_framework_async::yield_once().await; }
        let document = doc(fixture["document"].as_str().unwrap()).await;
        let seed = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        drop(recovery_seed(&seed, &document).await);
        let full = segment_bytes(&seed, &document, 0).await;
        assert_eq!(full.len() as u64, fixture["commitEnds"][2].as_u64().unwrap());
        for row in fixture["cuts"].as_array().unwrap() {
            let cut = row["cut"].as_u64().unwrap() as usize;
            let trusted = row["trustedEnd"].as_u64().unwrap() as usize;
            let recovered = row["recoveredEnd"].as_u64().unwrap() as usize;
            let next_tx = row["nextTxId"].as_u64().unwrap();
            let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
            storage.create_segment(&document, 0).await.unwrap();
            if cut != 0 { storage.append(&document, 0, pages(&full[..cut])).await.unwrap(); }
            let (mut wal, report) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 9).await.unwrap_or_else(|error| panic!("cut {cut}: {error}"));
            assert_eq!(report.torn_tail_bytes, (cut - trusted) as u64, "cut {cut}");
            assert_eq!(report.segments_seen, 1);
            assert_eq!(wal.next_tx_id, next_tx, "cut {cut}");
            assert_eq!(segment_bytes(&storage, &document, 0).await, full[..recovered], "cut {cut}: exact retained bytes and original commit boundaries");
            assert!(!wal.force_flush(&storage).await.unwrap());
            drop(wal);
            let (mut wal, clean) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 10).await.unwrap();
            assert_eq!(clean.torn_tail_bytes, 0);
            assert_eq!(segment_bytes(&storage, &document, 0).await, full[..recovered]);
            let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"c").await), DurabilityClass::Fsync, 11).await;
            assert_eq!(receipt.tx_id, next_tx);
            let appended = segment_bytes(&storage, &document, 0).await;
            assert_eq!(&appended[..recovered], &full[..recovered]);
            assert!(replay_summaries(&storage, &document).await.contains(&ReplaySummary::Commit(next_tx, 1)));
        }
        println!("[DEBUG] WAL recovery: 4 independent CRC page-alignment copies and 18 neutral cuts preserve exact bytes, reopen idempotence, sequence and subsequent transaction ids");
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_recovery_matches_neutral_lifecycle_without_prefix_replacement() {
        let fixture = recovery_fixture();
        let document = doc(fixture["document"].as_str().unwrap()).await;
        for row in fixture["lifecycle"].as_array().unwrap() {
            let name = row["name"].as_str().unwrap();
            let accepted = row["accepted"].as_bool().unwrap();
            let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
            if name == "exhausted-segment" {
                drop(SegmentWriter::begin(&storage, document.clone(), u64::MAX, Some([7; 32]), 0).await.unwrap());
            } else if name != "missing" {
                let mut wal = recovery_seed(&storage, &document).await;
                if matches!(name, "successor-empty" | "successor-partial" | "successor-header" | "compacted-clean" | "compacted-empty" | "compacted-header" | "earlier-active" | "wrong-chain") {
                    wal.rotate(&storage, 3).await.unwrap();
                }
                if name == "compacted-clean" { submit_one(&storage, &mut wal, WalRecord::Command(retained(b"c").await), DurabilityClass::Fsync, 4).await; }
                if name == "exhausted-tx" {
                    wal.active.append_record(&WalRecord::TxBegin { tx_id: u64::MAX }, 4).await.unwrap();
                    wal.active.append_record(&WalRecord::TxCommit { tx_id: u64::MAX, record_count: 0 }, 4).await.unwrap();
                    wal.force_flush(&storage).await.unwrap();
                }
                let tip = wal.active.tip_chain_hash().await.unwrap();
                drop(wal);
                match name {
                    "highest-sealed" => storage.seal(&document, 0).await.unwrap(),
                    "successor-empty" | "compacted-empty" => storage.truncate_tail(&document, 1, 0).await.unwrap(),
                    "successor-partial" => storage.truncate_tail(&document, 1, 15).await.unwrap(),
                    "successor-header" | "compacted-header" => storage.truncate_tail(&document, 1, 32).await.unwrap(),
                    "sealed-torn" | "sealed-empty" => {
                        storage.truncate_tail(&document, 0, if name == "sealed-empty" { 0 } else { 386 }).await.unwrap();
                        storage.seal(&document, 0).await.unwrap();
                    }
                    "earlier-active" | "corrupt-crc" | "partial-header-mismatch" => {
                        let mut bytes = segment_bytes(&storage, &document, 0).await;
                        if name == "corrupt-crc" { bytes[100] ^= 1; }
                        if name == "partial-header-mismatch" { bytes.truncate(15); bytes[0] ^= 1; }
                        storage.delete_segment(&document, 0).await.unwrap();
                        storage.create_segment(&document, 0).await.unwrap();
                        storage.append(&document, 0, pages(&bytes)).await.unwrap();
                    }
                    "wrong-document" => {
                        let other = doc("e").await;
                        drop(SegmentWriter::begin(&storage, other.clone(), 0, None, 0).await.unwrap());
                        let bytes = segment_bytes(&storage, &other, 0).await;
                        storage.delete_segment(&document, 0).await.unwrap();
                        storage.create_segment(&document, 0).await.unwrap();
                        storage.append(&document, 0, pages(&bytes)).await.unwrap();
                    }
                    "wrong-chain" => {
                        storage.delete_segment(&document, 1).await.unwrap();
                        drop(SegmentWriter::begin(&storage, document.clone(), 1, Some([7; 32]), 0).await.unwrap());
                    }
                    "index-gap" => {
                        storage.seal(&document, 0).await.unwrap();
                        drop(SegmentWriter::begin(&storage, document.clone(), 2, Some(tip), 0).await.unwrap());
                    }
                    _ => {}
                }
                if name.starts_with("compacted-") { storage.delete_segment(&document, 0).await.unwrap(); }
            }
            let mut indices = storage.list_segments(&document).await.unwrap();
            let mut before = Vec::new();
            for &index in indices.as_slice() { before.push((index, segment_bytes(&storage, &document, index).await, storage.segment_state(&document, index).await.unwrap())); }
            while indices.close_step() {}
            let result = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 5).await;
            assert_eq!(result.is_ok(), accepted, "{name}: {:?}", result.as_ref().err());
            for (index, bytes, state) in &before {
                if !accepted || *state == db_storage::WalSegmentState::Sealed || name == "compacted-clean" {
                    assert_eq!(segment_bytes(&storage, &document, *index).await, *bytes, "{name}: retained prefix replaced");
                    assert_eq!(storage.segment_state(&document, *index).await.unwrap(), *state, "{name}: lifecycle changed");
                }
            }
            if let Ok((mut wal, _)) = result {
                let expected_tx = if name == "missing" { 1 } else if name == "compacted-clean" { 4 } else { 3 };
                assert_eq!(wal.next_tx_id, expected_tx, "{name}");
                let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"z").await), DurabilityClass::Fsync, 6).await;
                assert_eq!(receipt.tx_id, expected_tx);
                assert!(replay_summaries(&storage, &document).await.contains(&ReplaySummary::Commit(expected_tx, 1)), "{name}");
            }
        }
        println!("[DEBUG] WAL recovery: 18 neutral lifecycle rows cover rotation gaps, compaction boundaries, invalid partial headers, sealed damage, identity/chain forgery and exhausted ids without replacement");
    }

    //#region 🔖️RecordKinds
    #[semio_framework_async_macros::async_test]
    async fn record_kinds_fill_the_extension_range_uniquely() {
        let kinds = [WAL_SEGMENT_HEADER, WAL_TX_BEGIN, WAL_TX_COMMIT, WAL_TX_ABORT, WAL_COMMAND, WAL_PAYLOAD, WAL_DIFF, WAL_INVERSE, WAL_EVENT, WAL_OUTBOX, WAL_FRONTIER, WAL_VCS_REF, WAL_SNAPSHOT_PUB, WAL_INDEX_CKPT, WAL_LEASE, WAL_MIGRATION];
        assert_eq!(kinds.len(), 16);
        let mut sorted = kinds.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), kinds.len(), "wal record kinds must be pairwise distinct");
        assert_eq!(*sorted.first().unwrap(), 0x40);
        assert_eq!(*sorted.last().unwrap(), 0x4F);
        for kind in kinds {
            assert!(is_wal_record_kind(kind));
        }
        // 🧪️ `0x0C` (protocol::wire::REC_COMMIT) hard-coded rather than depending on protocol_core
        // directly just for this one assertion — this crate's extension range never overlaps it.
        assert!(!is_wal_record_kind(0x0C));
    }
    //#endregion 🔖️RecordKinds

    //#region 🔖️Records
    #[semio_framework_async_macros::async_test]
    async fn wal_record_round_trips_every_kind_through_encode_decode() {
        let document = doc("doc-1").await;
        let mut samples = WalRecordBatch::new();
        for sample in [
            WalRecord::SegmentHeader { document: document.clone(), segment_index: 0, prev_chain_hash: None },
            WalRecord::SegmentHeader { document: document.clone(), segment_index: 1, prev_chain_hash: Some([3u8; 32]) },
            WalRecord::TxBegin { tx_id: 42 },
            WalRecord::TxCommit { tx_id: 42, record_count: 5 },
            WalRecord::TxAbort { tx_id: 7 },
            WalRecord::Command(retained(b"envelope-bytes").await),
            WalRecord::Payload(WalPayloadRef::Inline(retained(b"small-payload").await)),
            WalRecord::Payload(WalPayloadRef::CasRef(pack::ContentHash([5u8; 32]))),
            WalRecord::Diff(retained(b"diff-bytes").await),
            WalRecord::Inverse(retained(b"inverse-bytes").await),
            WalRecord::Event(retained(b"event-bytes").await),
            WalRecord::Outbox(retained(b"outbox-bytes").await),
            WalRecord::Frontier(sample_frontier(&document).await),
            WalRecord::VcsRef(db_storage::DbIoText::try_from_str("ck-abc123").unwrap()),
            WalRecord::SnapshotPub { generation: 4, frontier: sample_frontier(&document).await },
            WalRecord::IndexCkpt { run_ids: run_ids(&[1, 2, 3, 100]) },
            WalRecord::Lease { resource: db_storage::DbIoText::try_from_str("shard-0").unwrap(), holder: db_storage::DbIoText::try_from_str("node-a").unwrap(), fence: 9, expires_at_ms: 12345 },
            WalRecord::Migration(retained(b"migration-bytes").await),
        ] {
            assert!(samples.push(sample).is_ok());
        }
        for sample in samples.iter() {
            let (kind, critical, payload) = sample.encode().await;
            assert!(critical, "every wal record is critical by design");
            let mut decoded = decode(kind, &payload).await.unwrap();
            let (_, _, round_trip) = decoded.encode().await;
            assert_eq!(round_trip, payload);
            while decoded.close_step().unwrap() {}
        }
        while samples.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_rejects_unknown_kind_and_malformed_payload() {
        assert!(matches!(decode(0x7E, b"").await, Err(DbError::Corrupt(_))));
        assert!(decode(WAL_TX_BEGIN, b"").await.is_err());
    }
    //#endregion 🔖️Records

    //#region 🔖️PayloadTransform
    #[semio_framework_async_macros::async_test]
    async fn identity_payload_transform_round_trips_without_changing_bytes() {
        let transform = IdentityPayloadTransform;
        let plaintext = b"hello wal";
        let mut control = control();
        let encrypted = transform.encrypt(retained(plaintext).await, &mut control).await.unwrap();
        assert_eq!(read_retained(&encrypted).await, plaintext);
        let mut decrypted = transform.decrypt(encrypted, &mut control).await.unwrap();
        assert_eq!(read_retained(&decrypted).await, plaintext);
        while decrypted.close_step().unwrap().is_some() {}
    }

    /// @emoji 🔐️ A reversing "cipher" — enough to prove a caller can thread a non-identity
    /// `PayloadTransform` through `WalPayloadRef::Inline` end-to-end via this crate's own
    /// encode/decode, without `db_wal` itself needing to know encryption happened.
    struct ReversingTransform;
    impl PayloadTransform for ReversingTransform {
        async fn encrypt(&self, mut plaintext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
            let mut prepared = plaintext.prepare_platform().await?;
            let reversed: Vec<u8> = prepared.as_slice().iter().rev().copied().collect();
            while prepared.close_step()? {}
            while plaintext.close_step()?.is_some() {}
            match WalBytes::try_admit(reversed, MAX_FIELD_BYTES, control).await {
                Ok(bytes) => Ok(bytes),
                Err(mut rejected) => {
                    while rejected.close_step()? {
                        control.grant()?;
                    }
                    Err(rejected.into_error())
                }
            }
        }
        async fn decrypt(&self, ciphertext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
            self.encrypt(ciphertext, control).await
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn non_identity_payload_transform_round_trips_through_an_inline_wal_payload_record() {
        let transform = ReversingTransform;
        let plaintext = b"round trip me through the wal".to_vec();
        let mut control = control();
        let ciphertext = transform.encrypt(retained(&plaintext).await, &mut control).await.unwrap();
        assert_ne!(read_retained(&ciphertext).await, plaintext, "the transform must actually have changed the bytes");

        let record = WalRecord::Payload(WalPayloadRef::Inline(ciphertext));
        let (kind, _critical, payload) = record.encode().await;
        let decoded = decode(kind, &payload).await.unwrap();
        let WalRecord::Payload(WalPayloadRef::Inline(stored_ciphertext)) = decoded else {
            panic!("expected an inline payload record");
        };

        let mut recovered = transform.decrypt(stored_ciphertext, &mut control).await.unwrap();
        assert_eq!(read_retained(&recovered).await, plaintext);
        while recovered.close_step().unwrap().is_some() {}
    }
    //#endregion 🔖️PayloadTransform

    //#region 🔖️GroupCommit
    #[semio_framework_async_macros::async_test]
    async fn group_commit_policy_is_due_on_any_threshold() {
        let policy = GroupCommitPolicy { max_delay_ms: 100, max_bytes: 1000, max_records: 10 };
        assert!(!policy.is_due(0, 0, None, 0));
        assert!(policy.is_due(1000, 0, None, 0));
        assert!(policy.is_due(0, 10, None, 0));
        assert!(policy.is_due(0, 0, Some(0), 100));
        assert!(!policy.is_due(0, 0, Some(50), 100));
    }
    //#endregion 🔖️GroupCommit

    //#region 🔖️Segment + ArtifactWal
    #[semio_framework_async_macros::async_test]
    async fn single_segment_write_commit_flush_recovers_cleanly() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();

        let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"cmd-1").await), DurabilityClass::Fsync, 1).await;
        assert!(receipt.committed, "Fsync durability must force an immediate commit");
        assert_eq!(receipt.tx_id, 1);

        let bytes = segment_bytes(&storage, &document, 0).await;
        let report = protocol::format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
        assert_eq!(report.bytes_recovered, bytes.len() as u64);
        assert_eq!(report.torn_tail_bytes, 0);

        assert_eq!(replay_summaries(&storage, &document).await, vec![ReplaySummary::Segment(0, None), ReplaySummary::Begin(1), ReplaySummary::Command(b"cmd-1".to_vec()), ReplaySummary::Commit(1, 1)]);
    }

    #[semio_framework_async_macros::async_test]
    async fn group_commit_batches_until_policy_threshold_then_commits() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let policy = GroupCommitPolicy { max_delay_ms: 1_000_000, max_bytes: u64::MAX, max_records: 5 };
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), policy, 0)).unwrap();

        // Each submit writes 3 records (begin/command/commit); Memory durability never forces a
        // commit, so nothing should be flushed to storage until pending_records >= 5.
        let receipt_1 = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"a").await), DurabilityClass::Memory, 10).await;
        assert!(!receipt_1.committed);
        assert_eq!(db_actor::block_on(storage.segment_len(&document, 0)).unwrap(), wal.active.flushed_len, "nothing new should have flushed yet");

        // Second submit pushes pending_records to 6 (>= max_records 5), which must commit.
        let receipt_2 = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"b").await), DurabilityClass::Memory, 11).await;
        assert!(receipt_2.committed);
        assert_eq!(wal.active.pending_records, 0);
        assert!(db_actor::block_on(storage.segment_len(&document, 0)).unwrap() > 32, "flush must have appended past the bare header");
    }

    #[semio_framework_async_macros::async_test]
    async fn fsync_durability_forces_immediate_commit_regardless_of_policy() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let policy = GroupCommitPolicy { max_delay_ms: u64::MAX, max_bytes: u64::MAX, max_records: u32::MAX };
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document, policy, 0)).unwrap();
        let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"a").await), DurabilityClass::Fsync, 0).await;
        assert!(receipt.committed);
    }

    #[semio_framework_async_macros::async_test]
    async fn torn_tail_is_recovered_by_truncating_only_the_uncommitted_suffix() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        {
            let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
            submit_one(&storage, &mut wal, WalRecord::Command(retained(b"trusted").await), DurabilityClass::Fsync, 1).await;
        }

        // Simulate a crash mid-append: bytes physically present past the last trusted commit,
        // written directly to storage (bypassing SprWriter, exactly like a torn OS-level write).
        let committed = segment_bytes(&storage, &document, 0).await;
        db_actor::block_on(storage.append(&document, 0, pages(b"\x0Fgarbage"))).unwrap();

        let (wal, report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
        assert!(report.torn_tail_bytes > 0);
        assert_eq!(report.segments_seen, 1);
        drop(wal);

        assert_eq!(replay_summaries(&storage, &document).await, vec![ReplaySummary::Segment(0, None), ReplaySummary::Begin(1), ReplaySummary::Command(b"trusted".to_vec()), ReplaySummary::Commit(1, 1)]);

        let bytes = segment_bytes(&storage, &document, 0).await;
        let post_recovery = protocol::format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
        assert_eq!(post_recovery.bytes_recovered, bytes.len() as u64, "the truncated segment must itself be torn-tail-free");
        assert_eq!(bytes, committed, "all original commits are byte-preserved");
    }

    #[semio_framework_async_macros::async_test]
    async fn recovery_resumes_next_tx_id_and_accepts_further_submits() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        {
            let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
            submit_one(&storage, &mut wal, WalRecord::Command(retained(b"one").await), DurabilityClass::Fsync, 1).await;
            submit_one(&storage, &mut wal, WalRecord::Command(retained(b"two").await), DurabilityClass::Fsync, 2).await;
        }

        let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3)).unwrap();
        let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"three").await), DurabilityClass::Fsync, 4).await;
        assert_eq!(receipt.tx_id, 3, "recovery must resume tx ids strictly past whatever was already durable");

        let commands: Vec<_> = replay_summaries(&storage, &document)
            .await
            .into_iter()
            .filter_map(|record| match record {
                ReplaySummary::Command(bytes) => Some(bytes),
                _ => None,
            })
            .collect();
        assert_eq!(commands, vec![b"one".to_vec(), b"two".to_vec(), b"three".to_vec()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn multi_segment_rotation_chains_prev_hash_and_replay_spans_segments() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        wal.max_segment_bytes = 200; // force rotation quickly for this test

        for i in 0..20u32 {
            submit_one(&storage, &mut wal, WalRecord::Command(retained(format!("cmd-{i}").as_bytes()).await), DurabilityClass::Fsync, u64::from(i)).await;
        }

        let segments = db_actor::block_on(storage.list_segments(&document)).unwrap();
        assert!(segments.len() >= 2, "the byte threshold must have forced at least one rotation");

        // Cross-check segment 1's WAL_SEGMENT_HEADER.prev_chain_hash against segment 0's
        // independently-recomputed tip chain_hash.
        let seg0_bytes = segment_bytes(&storage, &document, 0).await;
        let seg0_report = protocol::format::recover(&seg0_bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
        let commit_frame_end = (seg0_report.last_commit_offset + protocol::format::COMMIT_FRAME_LEN) as usize;
        let mut cursor = protocol::FrameCursor::new(&seg0_bytes[seg0_report.last_commit_offset as usize..commit_frame_end], 0).await;
        let commit_frame = cursor.next_frame().await.unwrap().unwrap();
        let expected_chain_hash = protocol::format::parse_commit_payload(commit_frame.payload().await).unwrap().chain_hash;

        let full_replay = replay_summaries(&storage, &document).await;
        assert!(full_replay.contains(&ReplaySummary::Segment(0, None)));
        assert!(full_replay.contains(&ReplaySummary::Segment(1, Some(expected_chain_hash))));
        let commands_in_order: Vec<String> = full_replay
            .into_iter()
            .filter_map(|record| match record {
                ReplaySummary::Command(bytes) => Some(String::from_utf8(bytes).unwrap()),
                _ => None,
            })
            .collect();
        let expected: Vec<String> = (0..20u32).map(|i| format!("cmd-{i}")).collect();
        assert_eq!(commands_in_order, expected, "replay must span every segment in rotation order");
    }

    #[semio_framework_async_macros::async_test]
    async fn recovery_rejects_a_torn_non_active_sealed_segment() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        wal.max_segment_bytes = 1; // rotate on the very next submit
        submit_one(&storage, &mut wal, WalRecord::Command(retained(b"forces-rotation").await), DurabilityClass::Fsync, 0).await;
        assert!(db_actor::block_on(storage.list_segments(&document)).unwrap().len() >= 2);

        // Corrupt the now-sealed segment 0 by truncating a byte off its tail directly in storage
        // — WalStorage::truncate_tail refuses a sealed segment, so simulate on-disk bit rot
        // instead via delete+recreate+append of a shortened copy.
        let seg0_bytes = segment_bytes(&storage, &document, 0).await;
        db_actor::block_on(storage.delete_segment(&document, 0)).unwrap();
        db_actor::block_on(storage.create_segment(&document, 0)).unwrap();
        db_actor::block_on(storage.append(&document, 0, pages(&seg0_bytes[..seg0_bytes.len() - 1]))).unwrap();
        db_actor::block_on(storage.seal(&document, 0)).unwrap();

        let result = db_actor::block_on(ArtifactWal::open(&storage, document, GroupCommitPolicy::default(), 100));
        assert!(matches!(result, Err(DbError::Corrupt(_))), "a torn sealed (non-active) segment must be a hard recovery error");
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_document_open_creates_a_fresh_wal() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let (wal, report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        assert_eq!(report, WalRecoveryReport::default());
        assert_eq!(wal.active_segment_index().await, 0);
        assert_eq!(db_actor::block_on(storage.list_segments(&document)).unwrap(), vec![0]);
    }
    //#endregion 🔖️Segment + ArtifactWal
}
//#endregion 🧪️Tests
//#region 🧪️RetainedTests
#[cfg(test)]
mod retained_tests {
    use super::*;
    use crate::db_storage::WalStorage;

    fn fail_stop_fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️fixtures/🛑️fail-stop/🔣️.json")).unwrap()
    }

    async fn fail_stop_segment_bytes(storage: &impl WalStorage, document: &ArtifactId) -> Vec<u8> {
        let len = storage.segment_len(document, 0).await.unwrap();
        let mut pages = storage.read(document, 0, pack::ByteRange { offset: 0, len }).await.unwrap();
        let mut bytes = Vec::with_capacity(len as usize);
        for fragment in pages.fragments() { bytes.extend_from_slice(fragment); }
        while pages.close_step().unwrap().is_some() { semio_framework_async::yield_once().await; }
        bytes
    }

    async fn assert_artifact_wal_fail_stop_case(name: &str) {
        let fixture = fail_stop_fixture();
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == name).unwrap();
        let inner = std::sync::Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let storage = crate::db_testkit::FaultStorage::new(inner.clone()).await;
        let document = ArtifactId::from(format!("retained-artifact-wal-fail-stop-{name}"));
        let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
        let baseline = fail_stop_segment_bytes(&storage, &document).await;
        let append_boundary = storage.append_calls().await + 1;
        let sync_boundary = storage.sync_calls().await + 1;
        let mut script = crate::db_testkit::FaultScript::default();
        match case["fault"].as_str().unwrap() {
            "shortAppend" => script.torn_write_at = Some((append_boundary, case["keepBytes"].as_u64().unwrap())),
            "appendError" => script.fail_nth_write = Some(append_boundary),
            "syncError" => script.fail_nth_sync = Some(sync_boundary),
            fault => panic!("unknown WAL fail-stop fixture fault {fault}"),
        }
        storage.set_script(script).await;

        let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
        let command = WalBytes::try_admit(format!("fault-{name}").into_bytes(), 64, &mut admission).await.unwrap();
        let mut batch = WalRecordBatch::new();
        assert!(batch.push(WalRecord::Command(command)).is_ok());
        let error = wal.submit(&storage, &batch, DurabilityClass::Fsync, 1).await.unwrap_err();
        match case["expectedError"].as_str().unwrap() {
            "Corrupt" => assert!(matches!(error, DbError::Corrupt(message) if message.contains("append returned segment length"))),
            "Io" => assert!(matches!(error, DbError::Io(_))),
            expected => panic!("unknown WAL fail-stop fixture error {expected}"),
        }
        let append_calls = storage.append_calls().await;
        let sync_calls = storage.sync_calls().await;
        assert!(matches!(wal.submit(&storage, &batch, DurabilityClass::Fsync, 2).await, Err(DbError::Closed)));
        assert!(matches!(wal.force_flush(&storage).await, Err(DbError::Closed)));
        assert!(matches!(wal.rotate(&storage, 2).await, Err(DbError::Closed)));
        assert_eq!(storage.append_calls().await, append_calls, "a poisoned writer must never retry its uncertain suffix");
        assert_eq!(storage.sync_calls().await, sync_calls, "a poisoned writer must never retry sync");
        while batch.close_step().unwrap() {}

        let after_failure = fail_stop_segment_bytes(&storage, &document).await;
        match case["expectedPhysicalSuffix"].as_str().unwrap() {
            "absent" => assert_eq!(after_failure, baseline),
            "torn" => {
                assert!(after_failure.starts_with(&baseline));
                assert_eq!(after_failure.len(), baseline.len() + case["keepBytes"].as_u64().unwrap() as usize);
            }
            "complete" => {
                assert!(after_failure.starts_with(&baseline));
                assert!(after_failure.len() > baseline.len());
            }
            suffix => panic!("unknown WAL fail-stop fixture suffix {suffix}"),
        }
        while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
        assert!(wal.terminal_is_empty());

        let inner_wal = inner.wal().await;
        let (mut reopened, report) = ArtifactWal::open(&inner_wal, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
        let recovered = fail_stop_segment_bytes(&inner_wal, &document).await;
        if case["expectedPhysicalSuffix"] == "complete" {
            assert_eq!(recovered, after_failure, "a sync error must not duplicate its already-appended complete commit");
            assert_eq!(report.torn_tail_bytes, 0);
        } else {
            assert_eq!(recovered, baseline, "reopen must retain exactly the last complete prefix");
            assert_eq!(report.torn_tail_bytes, after_failure.len() as u64 - baseline.len() as u64);
        }
        while reopened.close_step().unwrap() { semio_framework_async::yield_once().await; }
        assert!(reopened.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_bytes_exact_backing_handback_cancel_and_close_are_one_owner() {
        let _pool = crate::db_storage::db_io_test_pool();
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 64).unwrap();
        let mut source = Vec::with_capacity(32);
        source.push(0xA5);
        let mut retained = WalBytes::try_admit(source, 32, &mut control).await.unwrap();
        assert_eq!(retained.len(), 1);
        while retained.close_step().unwrap().is_some() {}
        assert!(retained.terminal_is_empty());

        let mut source = Vec::with_capacity(33);
        source.push(0x5A);
        let pointer = source.as_ptr();
        let rejected = WalBytes::try_admit(source, 32, &mut control).await.unwrap_err();
        let returned = rejected.into_source().unwrap();
        assert_eq!(returned.as_ptr(), pointer);
        assert_eq!(returned.capacity(), 33);

        cancelled.store(true, std::sync::atomic::Ordering::Release);
        let mut source = Vec::with_capacity(8);
        source.push(1);
        let pointer = source.as_ptr();
        let mut rejected = WalBytes::try_admit(source, 8, &mut control).await.unwrap_err();
        while rejected.close_step().unwrap() {}
        let returned = rejected.into_source().unwrap();
        assert_eq!(returned.as_ptr(), pointer);

        cancelled.store(false, std::sync::atomic::Ordering::Release);
        let mut deadline_control = WalCursorControl::new(cancelled, std::time::Instant::now(), 16).unwrap();
        let mut source = Vec::with_capacity(2);
        source.push(0x33);
        let pointer = source.as_ptr();
        let mut rejected = WalBytes::try_admit(source, 2, &mut deadline_control).await.unwrap_err();
        assert!(matches!(rejected.error(), DbError::Unavailable(message) if message == "wal cursor deadline reached"));
        while rejected.close_step().unwrap() {}
        assert_eq!(rejected.into_source().unwrap().as_ptr(), pointer);
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_replay_cancel_resume_close_and_fragment_crc_are_deterministic() {
        let storage = db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId::from("retained-replay");
        let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
        let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_024).unwrap();
        let bytes = WalBytes::try_admit(vec![0xA5; db_storage::DB_IO_PAGE_BYTES + 1], (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut admission).await.unwrap();
        let mut batch = WalRecordBatch::new();
        assert!(batch.push(WalRecord::Command(bytes)).is_ok());
        wal.submit(&storage, &batch, DurabilityClass::Fsync, 0).await.unwrap();
        while batch.close_step().unwrap() {}

        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut replay = replay_document(&storage, &document, control).await.unwrap();
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert!(matches!(replay.next().await, Err(DbError::Unavailable(_))));
        cancelled.store(false, std::sync::atomic::Ordering::Release);
        let mut seen = 0usize;
        let mut boundary_yields = 0usize;
        while boundary_yields < 2 {
            match replay.next_step().await.unwrap() {
                WalReplayStep::Record(mut record) => {
                    seen += 1;
                    while record.close_step().unwrap() {}
                }
                WalReplayStep::Yield => { if seen != 0 { boundary_yields += 1; } },
                WalReplayStep::Done => panic!("retained replay closed without resumable segment retirement"),
            }
        }
        assert!(seen >= 1);
        while replay.close_step().await.unwrap() {}
        assert!(replay.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty() {
        let storage = db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId::from("retained-replay-cancel-close");
        let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
        let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_024).unwrap();
        let bytes = WalBytes::try_admit(vec![0x5A; db_storage::DB_IO_PAGE_BYTES + 1], (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut admission).await.unwrap();
        let mut batch = WalRecordBatch::new();
        assert!(batch.push(WalRecord::Command(bytes)).is_ok());
        wal.submit(&storage, &batch, DurabilityClass::Fsync, 0).await.unwrap();
        while batch.close_step().unwrap() {}

        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut replay = replay_document(&storage, &document, control).await.unwrap();
        assert!(matches!(replay.next_step().await.unwrap(), WalReplayStep::Yield));
        assert!(replay.pages.is_some());
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert!(matches!(replay.next_step().await, Err(DbError::Unavailable(message)) if message == "wal cursor cancelled"));
        while replay.close_step().await.unwrap() {
            assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
        }
        assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
        assert!(replay.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_wal_repeated_open_close_is_page_budget_neutral() {
        let storage = db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId::from("retained-artifact-wal-close-budget");
        let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
        while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
        assert!(wal.terminal_is_empty());

        for turn in 0..18 {
            let (mut wal, _) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), turn as u64 + 1).await.unwrap();
            while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
            assert!(wal.terminal_is_empty());
        }

        let (mut wal, _) = ArtifactWal::open(&storage, document, GroupCommitPolicy::default(), 100).await.unwrap();
        let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
        let command = WalBytes::try_admit(b"after-repeated-close".to_vec(), 64, &mut admission).await.unwrap();
        let mut batch = WalRecordBatch::new();
        assert!(batch.push(WalRecord::Command(command)).is_ok());
        assert!(wal.submit(&storage, &batch, DurabilityClass::Fsync, 101).await.unwrap().committed);
        while batch.close_step().unwrap() {}
        while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
        assert!(wal.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_wal_close_rejects_pending_records_and_closed_writes() {
        let storage = db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId::from("retained-artifact-wal-pending-close");
        let policy = GroupCommitPolicy { max_delay_ms: u64::MAX, max_bytes: u64::MAX, max_records: u32::MAX };
        let mut wal = ArtifactWal::create(&storage, document, policy, 0).await.unwrap();
        let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
        let command = WalBytes::try_admit(b"pending".to_vec(), 64, &mut admission).await.unwrap();
        let mut batch = WalRecordBatch::new();
        assert!(batch.push(WalRecord::Command(command)).is_ok());
        assert!(!wal.submit(&storage, &batch, DurabilityClass::Memory, 1).await.unwrap().committed);
        while batch.close_step().unwrap() {}
        assert!(matches!(wal.close_step(), Err(DbError::InvalidArgument(message)) if message.contains("force_flush")));
        assert!(!wal.terminal_is_empty());
        assert!(wal.force_flush(&storage).await.unwrap());
        while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
        assert!(wal.terminal_is_empty());

        let empty = WalRecordBatch::new();
        assert!(matches!(wal.submit(&storage, &empty, DurabilityClass::Memory, 2).await, Err(DbError::Closed)));
        assert!(matches!(wal.force_flush(&storage).await, Err(DbError::Closed)));
        assert!(matches!(wal.rotate(&storage, 2).await, Err(DbError::Closed)));
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_wal_short_append_is_fail_stop_until_reopen() {
        assert_artifact_wal_fail_stop_case("short-append").await;
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_wal_append_error_is_fail_stop_until_reopen() {
        assert_artifact_wal_fail_stop_case("append-error").await;
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_wal_sync_error_is_fail_stop_until_reopen() {
        assert_artifact_wal_fail_stop_case("sync-error").await;
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_wal_successor_failure_after_seal_is_fail_stop_until_reopen() {
        assert!(fail_stop_fixture()["cases"].as_array().unwrap().iter().any(|case| case["fault"] == "successorAppendError"));
        let inner = std::sync::Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
        let storage = crate::db_testkit::FaultStorage::new(inner.clone()).await;
        let document = ArtifactId::from("retained-artifact-wal-successor-fail-stop");
        let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
        wal.max_segment_bytes = 1;
        storage.set_script(crate::db_testkit::FaultScript { fail_nth_write: Some(storage.append_calls().await + 2), ..crate::db_testkit::FaultScript::default() }).await;
        let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
        let command = WalBytes::try_admit(b"committed-before-successor-failure".to_vec(), 64, &mut admission).await.unwrap();
        let mut batch = WalRecordBatch::new();
        assert!(batch.push(WalRecord::Command(command)).is_ok());
        assert!(matches!(wal.submit(&storage, &batch, DurabilityClass::Fsync, 1).await, Err(DbError::Io(_))));
        while batch.close_step().unwrap() {}
        assert_eq!(storage.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Sealed);
        let sealed = fail_stop_segment_bytes(&storage, &document).await;
        let append_calls = storage.append_calls().await;
        assert!(matches!(wal.force_flush(&storage).await, Err(DbError::Closed)));
        assert!(matches!(wal.rotate(&storage, 2).await, Err(DbError::Closed)));
        assert_eq!(storage.append_calls().await, append_calls);
        while wal.close_step().unwrap() { semio_framework_async::yield_once().await; }
        assert!(wal.terminal_is_empty());

        let inner_wal = inner.wal().await;
        let (mut reopened, _) = ArtifactWal::open(&inner_wal, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
        assert_eq!(reopened.active_segment_index().await, 1);
        assert_eq!(fail_stop_segment_bytes(&inner_wal, &document).await, sealed);
        let control = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut replay = replay_document(&inner_wal, &document, control).await.unwrap();
        let mut commands = 0;
        while let Some(mut record) = replay.next().await.unwrap() {
            if matches!(&record, WalRecord::Command(_)) { commands += 1; }
            while record.close_step().unwrap() { semio_framework_async::yield_once().await; }
        }
        assert_eq!(commands, 1, "reopen must expose the pre-seal transaction exactly once");
        while replay.close_step().await.unwrap() {}
        while reopened.close_step().unwrap() { semio_framework_async::yield_once().await; }
        assert!(reopened.terminal_is_empty());
    }
}
//#endregion 🧪️RetainedTests
