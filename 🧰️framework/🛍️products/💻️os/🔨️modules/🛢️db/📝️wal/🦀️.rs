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
pub async fn is_wal_record_kind(kind: u8) -> bool {
    (WAL_SEGMENT_HEADER..=WAL_MIGRATION).contains(&kind)
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

    pub fn grant(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("wal cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("wal cursor deadline reached".to_string()));
        }
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
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.byte(control)?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("wal retained varint overflow".to_string()))
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
/// (per this crate's module doc) no resume-mid-stream constructor either, so holding the buffer
/// open for a segment's whole lifetime via a second handle is the only way to both keep writing
/// AND read back what's been written so far without prematurely consuming the writer.
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
            if output.write_fragment(&fragment[..read])? != read {
                return Err(DbError::LimitExceeded("WAL retained suffix writer"));
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

struct WalPageReader<'pages> {
    pages: &'pages db_storage::DbIoPages,
    position: usize,
    limit: usize,
}

impl<'pages> WalPageReader<'pages> {
    fn new(pages: &'pages db_storage::DbIoPages, position: usize, limit: usize) -> Result<Self, DbError> {
        if position > limit || limit > pages.len() {
            return Err(DbError::Corrupt("wal retained reader authority".to_string()));
        }
        Ok(Self { pages, position, limit })
    }

    fn fragment(&self) -> Result<&'pages [u8], DbError> {
        if self.position >= self.limit {
            return Err(DbError::Corrupt("wal retained field ended early".to_string()));
        }
        let mut base = 0usize;
        for fragment in self.pages.fragments() {
            let end = base + fragment.len();
            if self.position < end {
                return Ok(&fragment[self.position - base..fragment.len().min(self.limit - base)]);
            }
            base = end;
        }
        Err(DbError::Corrupt("wal retained reader lost its fragment".to_string()))
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
        Err(DbError::Corrupt("wal retained varint exceeds u64".to_string()))
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

    async fn bytes(&mut self, operation: u64, len: usize, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
        let end = self.position.checked_add(len).ok_or(DbError::LimitExceeded("wal retained byte field"))?;
        if end > self.limit {
            return Err(DbError::Corrupt("wal retained byte field exceeds frame".to_string()));
        }
        let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(operation, len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        while self.position < end {
            control.grant()?;
            let fragment = self.fragment()?;
            let count = (end - self.position).min(fragment.len());
            let written = writer.write_fragment(&fragment[..count])?;
            self.position += written;
        }
        writer.seal_retained().await.map(|pages| WalBytes { pages }).map_err(db_storage::DbIoPageWriterRejected::into_error)
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

async fn wal_decode_page_record(operation: u64, kind: u8, reader: &mut WalPageReader<'_>, control: &mut WalCursorControl) -> Result<WalRecord, DbError> {
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
        WAL_COMMAND => WalRecord::Command(reader.bytes(operation, reader.limit - reader.position, control).await?),
        WAL_PAYLOAD => match reader.byte()? {
            0 => {
                let len = reader.varint()?;
                check_len(len, MAX_FIELD_BYTES, "wal retained payload")?;
                WalRecord::Payload(WalPayloadRef::Inline(reader.bytes(operation, len as usize, control).await?))
            }
            1 => WalRecord::Payload(WalPayloadRef::CasRef(ContentHash(reader.array()?))),
            _ => return Err(DbError::Corrupt("wal payload tag".to_string())),
        },
        WAL_DIFF => WalRecord::Diff(reader.bytes(operation, reader.limit - reader.position, control).await?),
        WAL_INVERSE => WalRecord::Inverse(reader.bytes(operation, reader.limit - reader.position, control).await?),
        WAL_EVENT => WalRecord::Event(reader.bytes(operation, reader.limit - reader.position, control).await?),
        WAL_OUTBOX => WalRecord::Outbox(reader.bytes(operation, reader.limit - reader.position, control).await?),
        WAL_FRONTIER => WalRecord::Frontier(wal_decode_frontier(reader)?),
        WAL_VCS_REF => WalRecord::VcsRef(reader.text()?),
        WAL_SNAPSHOT_PUB => WalRecord::SnapshotPub { generation: u64::from_le_bytes(reader.array()?), frontier: wal_decode_frontier(reader)? },
        WAL_INDEX_CKPT => {
            let count = reader.varint()?;
            check_len(count, 4_096, "wal retained checkpoint")?;
            let mut run_ids = db_storage::DbIoU64List::new();
            for _ in 0..count {
                control.grant()?;
                run_ids.push(u64::from_le_bytes(reader.array()?))?;
            }
            WalRecord::IndexCkpt { run_ids }
        }
        WAL_LEASE => WalRecord::Lease { resource: reader.text()?, holder: reader.text()?, fence: u64::from_le_bytes(reader.array()?), expires_at_ms: u64::from_le_bytes(reader.array()?) },
        WAL_MIGRATION => WalRecord::Migration(reader.bytes(operation, reader.limit - reader.position, control).await?),
        _ => return Err(DbError::Corrupt(format!("unknown wal record kind {kind:#x}"))),
    };
    if reader.position != reader.limit {
        return Err(DbError::Corrupt("wal record retained decoder left trailing payload".to_string()));
    }
    Ok(record)
}

async fn wal_next_page_record(pages: &db_storage::DbIoPages, offset: &mut usize, trusted_len: usize, control: &mut WalCursorControl) -> Result<Option<WalRecord>, DbError> {
    loop {
        control.grant()?;
        if *offset == trusted_len {
            return Ok(None);
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
        if flags & protocol::wire::FRAME_FLAG_COMPRESSED != 0 {
            return Err(DbError::Corrupt("db_wal does not admit compressed retained records".to_string()));
        }
        let payload_start = reader.position;
        let mut trailer = WalPageReader::new(pages, body_end, frame_end)?;
        let stored_crc = u32::from_le_bytes(trailer.array()?);
        let back_len = u32::from_le_bytes(trailer.array()?) as usize;
        if back_len != frame_end - frame_start || wal_crc_range(pages, body_start, body_len as usize, control)? != stored_crc {
            return Err(DbError::Corrupt("wal frame retained crc or back length mismatch".to_string()));
        }
        *offset = frame_end;
        if is_wal_record_kind(kind).await {
            let mut payload = WalPageReader::new(pages, payload_start, body_end)?;
            return wal_decode_page_record(pages.operation(), kind, &mut payload, control).await.map(Some);
        }
        if kind != protocol::wire::REC_COMMIT {
            return Err(DbError::Corrupt(format!("unexpected non-wal, non-commit frame kind {kind:#x} in a db_wal segment")));
        }
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
}

/// @emoji 🔁️ Decodes every `WAL_*` record across a document's ENTIRE WAL (every sealed segment in
/// full, plus the active segment's currently-trusted prefix), in segment then on-disk order — the
/// primitive `db_artifact`'s materialize-from-WAL-suffix step builds on. Every sealed segment is
/// expected fully trusted (a torn sealed segment is `DbError::Corrupt`, since `truncate_tail` only
/// targets an unsealed segment); the last (possibly active, possibly unsealed) segment is
/// recovered via `protocol::format::recover` first.
pub struct WalReplayCursor<'storage, S: db_storage::WalStorage> {
    storage: &'storage S,
    document: ArtifactId,
    segments: db_storage::DbIoU64List,
    segment: usize,
    pages: Option<db_storage::DbIoPages>,
    offset: usize,
    trusted_len: usize,
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
        Ok(Self { storage, document: document.clone(), segments, segment: 0, pages: None, offset: protocol::format::HEADER_SIZE, trusted_len: 0, control, closed: false })
    }

    async fn open_segment(&mut self) -> Result<bool, DbError> {
        let Some(index) = self.segments.as_slice().get(self.segment).copied() else { return Ok(false) };
        self.control.grant()?;
        let len = self.storage.segment_len(&self.document, index).await?;
        let pages = self.storage.read(&self.document, index, pack::ByteRange { offset: 0, len }).await?;
        let report = protocol::format::recover(&WalPageSource(&pages), &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.map_err(protocol_err)?;
        if report.bytes_recovered != pages.len() as u64 {
            return Err(DbError::Corrupt(format!("wal segment {index} for {} has a torn tail ({} of {} bytes trusted)", self.document, report.bytes_recovered, pages.len())));
        }
        self.offset = protocol::format::HEADER_SIZE;
        self.trusted_len = report.bytes_recovered as usize;
        self.pages = Some(pages);
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
        loop {
            self.control.grant()?;
            if self.closed {
                return Ok(WalReplayStep::Done);
            }
            if self.pages.is_none() && !self.open_segment().await? {
                return Ok(WalReplayStep::Done);
            }
            if self.offset == self.trusted_len {
                if self.close_segment_step().await? {
                    return Ok(WalReplayStep::Yield);
                }
                self.segment += 1;
                return Ok(WalReplayStep::Yield);
            }
            let pages = self.pages.as_ref().ok_or_else(|| DbError::Internal("wal replay lost segment pages".to_string()))?;
            if let Some(record) = wal_next_page_record(pages, &mut self.offset, self.trusted_len, &mut self.control).await? {
                return Ok(WalReplayStep::Record(record));
            }
        }
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
        self.control.grant()?;
        if let Some(pages) = self.pages.as_mut() {
            if pages.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.pages = None;
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
        self.closed && self.pages.is_none() && self.segments.terminal_is_empty()
    }
}

pub async fn replay_document<'storage, S: db_storage::WalStorage>(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<WalReplayCursor<'storage, S>, DbError> {
    WalReplayCursor::open(storage, document, control).await
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
    buf: SharedBuf,
    writer: protocol::SprWriter<SharedBuf>,
    flushed_len: u64,
    pending_records: u32,
    oldest_pending_at_ms: Option<u64>,
}

impl SegmentWriter {
    /// @emoji 🆕️ Creates segment `index` in `storage`, writes its `WAL_SEGMENT_HEADER` record, and
    /// commits+flushes immediately (a segment's own identity/chain-link should never be lost to a
    /// crash before the segment records anything else).
    async fn begin(storage: &impl db_storage::WalStorage, document: ArtifactId, index: u64, prev_chain_hash: Option<[u8; 32]>, now_ms: u64) -> Result<Self, DbError> {
        storage.create_segment(&document, index).await?;
        let buf = SharedBuf::try_new()?;
        let writer = protocol::SprWriter::begin(buf.clone(), &segment_write_options()).await.map_err(protocol_err)?;
        let mut segment = Self { document: document.clone(), index, buf, writer, flushed_len: 0, pending_records: 0, oldest_pending_at_ms: None };
        segment.append_record(&WalRecord::SegmentHeader { document, segment_index: index, prev_chain_hash }, now_ms).await?;
        segment.commit_and_flush(storage, DurabilityClass::Fsync).await?;
        Ok(segment)
    }

    async fn append_record(&mut self, record: &WalRecord, now_ms: u64) -> Result<u64, DbError> {
        let offset = record.write_retained(&mut self.writer).await?;
        if self.pending_records == 0 {
            self.oldest_pending_at_ms = Some(now_ms);
        }
        self.pending_records += 1;
        Ok(offset)
    }

    /// @emoji 📏️ Bytes written since the last flush — not yet visible to `WalStorage`.
    fn pending_bytes(&self) -> u64 {
        self.buf.len() - self.flushed_len
    }

    /// @emoji 📏️ The segment's total logical length so far, flushed or not — what
    /// `ArtifactWal::submit` compares against its segment-rotation threshold.
    fn total_len(&self) -> u64 {
        self.buf.len()
    }

    /// @emoji ⛓️ Physically commits (`SprWriter::commit`, hash-chaining everything pending) and
    /// flushes the newly-committed suffix to `WalStorage::append` + `sync(class)` — the group-
    /// commit primitive `ArtifactWal::submit`/`force_flush`/`rotate` all funnel through. A no-op
    /// (`Ok(None)`) if nothing is pending.
    async fn commit_and_flush(&mut self, storage: &impl db_storage::WalStorage, class: DurabilityClass) -> Result<Option<u64>, DbError> {
        if self.pending_records == 0 {
            return Ok(None);
        }
        let commit_offset = self.writer.commit().await.map_err(protocol_err)?;
        let suffix_len = usize::try_from(self.buf.len().saturating_sub(self.flushed_len)).map_err(|_| DbError::LimitExceeded("WAL retained suffix length"))?;
        let pages = self.buf.copy_range(self.flushed_len as usize, suffix_len).await?;
        let new_len = storage.append(&self.document, self.index, pages).await?;
        storage.sync(&self.document, self.index, class).await?;
        self.flushed_len = new_len;
        self.pending_records = 0;
        self.oldest_pending_at_ms = None;
        Ok(Some(commit_offset))
    }

    /// @emoji ⛓️ The chain_hash of this segment's last commit (falling back to `blake3(header)` if
    /// nothing has committed beyond the segment's own header write, which `begin` always performs,
    /// so this should never actually hit that branch in practice — handled honestly rather than
    /// assumed away). Used by `ArtifactWal::rotate` to seed the next segment's
    /// `WAL_SEGMENT_HEADER.prev_chain_hash`.
    async fn tip_chain_hash(&self) -> Result<[u8; 32], DbError> {
        let commit_len = protocol::format::COMMIT_FRAME_LEN as usize;
        if self.buf.len() < commit_len as u64 {
            return Err(DbError::Corrupt("WAL retained pages contain no commit frame".to_string()));
        }
        let commit_offset = self.buf.len() as usize - commit_len;
        let mut frame_bytes = [0u8; protocol::format::COMMIT_FRAME_LEN as usize];
        self.buf.read_exact(commit_offset, &mut frame_bytes).await?;
        let mut cursor = protocol::FrameCursor::new(&frame_bytes, 0).await;
        let frame = cursor.next_frame().await.map_err(protocol_err)?.ok_or_else(|| DbError::Corrupt("expected a commit frame while sealing wal segment".to_string()))?;
        if frame.kind != protocol::wire::REC_COMMIT {
            return Err(DbError::Corrupt(format!("expected REC_COMMIT at the recovered commit offset, found kind {:#x}", frame.kind)));
        }
        Ok(protocol::format::parse_commit_payload(frame.payload().await).await.map_err(protocol_err)?.chain_hash)
    }
}
//#endregion 🔖️Segment

//#region 🔖️ArtifactWal
/// @emoji 📏️ Default segment-rotation threshold (this crate's own choice — the contract fixes
/// "per-document segment files", not an exact size): large enough that rotation stays rare under
/// ordinary load, small enough that a single segment's crash-recovery replay stays bounded.
const DEFAULT_MAX_SEGMENT_BYTES: u64 = (db_storage::DB_IO_OPERATION_PAGES * db_storage::DB_IO_PAGE_BYTES / 2) as u64;

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

    /// @emoji 🚑️ Opens `document`'s existing WAL, recovering it: every sealed segment (all but the
    /// highest index) must be fully trusted end-to-end (`DbError::Corrupt` otherwise — a torn
    /// sealed segment is unrecoverable damage, not a normal crash artifact); the highest-indexed
    /// segment is treated as possibly-active and recovered via `protocol::format::recover`,
    /// discarding any torn tail. Creates a fresh WAL (equivalent to `create`) if `document` has no
    /// segments yet.
    pub async fn open(storage: &impl db_storage::WalStorage, document: ArtifactId, policy: GroupCommitPolicy, now_ms: u64) -> Result<(Self, WalRecoveryReport), DbError> {
        let mut indices = storage.list_segments(&document).await?;
        if indices.is_empty() {
            let _ = indices.close_step();
            drop(indices);
            return Ok((Self::create(storage, document, policy, now_ms).await?, WalRecoveryReport::default()));
        }
        let last_index = *indices.last().ok_or_else(|| DbError::Internal("WAL segment list changed after non-empty witness".to_string()))?;

        for &index in &indices[..indices.len() - 1] {
            let len = storage.segment_len(&document, index).await?;
            let mut bytes = storage.read(&document, index, pack::ByteRange { offset: 0, len }).await?;
            let report = protocol::format::recover(&WalPageSource(&bytes), &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.map_err(protocol_err)?;
            if report.bytes_recovered != bytes.len() as u64 {
                return Err(DbError::Corrupt(format!("wal segment {index} for {document} has a torn tail ({} of {} bytes trusted) but is not the active segment", report.bytes_recovered, bytes.len())));
            }
            let _ = bytes.close_step()?;
            drop(bytes);
        }

        let len = storage.segment_len(&document, last_index).await?;
        let mut bytes = storage.read(&document, last_index, pack::ByteRange { offset: 0, len }).await?;
        let report = protocol::format::recover(&WalPageSource(&bytes), &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.map_err(protocol_err)?;
        let torn_tail_bytes = bytes.len() as u64 - report.bytes_recovered;

        // 🎯️ Design choice (forced by `protocol::SprWriter`'s API — the same constraint
        // `protocol_io::HistoryFile::open_append` documents and works around identically): there
        // is no public constructor that resumes a writer mid-stream with its running chain hash
        // intact. The only correctness-preserving resume is O(segment size): discard the physical
        // segment and replay every trusted `WAL_*` record through a freshly-begun writer, which
        // reproduces the exact same logical WAL content (fewer, coalesced commit frames is fine —
        // recovery only has to restore the committed record set, not the original commit
        // boundaries).
        storage.delete_segment(&document, last_index).await?;
        storage.create_segment(&document, last_index).await?;
        let buf = SharedBuf::try_new()?;
        let mut writer = protocol::SprWriter::begin(buf.clone(), &segment_write_options()).await.map_err(protocol_err)?;
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = WalCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
        let mut offset = protocol::format::HEADER_SIZE;
        let mut records_replayed = 0u64;
        let mut next_tx_id = 1u64;
        while let Some(mut record) = wal_next_page_record(&bytes, &mut offset, report.bytes_recovered as usize, &mut control).await? {
            record.write_retained(&mut writer).await?;
            if let Some(tx_id) = record.tx_id() {
                next_tx_id = next_tx_id.max(tx_id.saturating_add(1));
            }
            control.grant()?;
            let _ = record.close_step()?;
            drop(record);
            records_replayed += 1;
        }
        control.grant()?;
        let _ = bytes.close_step()?;
        drop(bytes);
        let pending_records = u32::try_from(records_replayed).map_err(|_| DbError::LimitExceeded("wal recovered record count"))?;
        let mut active = SegmentWriter { document: document.clone(), index: last_index, buf, writer, flushed_len: 0, pending_records, oldest_pending_at_ms: if records_replayed == 0 { None } else { Some(now_ms) } };
        active.commit_and_flush(storage, DurabilityClass::Fsync).await?;

        let recovery = WalRecoveryReport { segments_seen: indices.len() as u64, records_replayed, torn_tail_bytes };
        let _ = indices.close_step();
        drop(indices);
        Ok((Self { document, policy, max_segment_bytes: DEFAULT_MAX_SEGMENT_BYTES, next_segment_index: last_index + 1, active, next_tx_id }, recovery))
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
        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;
        let segment_index = self.active.index;

        self.active.append_record(&WalRecord::TxBegin { tx_id }, now_ms).await?;
        for record in records.iter() {
            self.active.append_record(record, now_ms).await?;
        }
        self.active.append_record(&WalRecord::TxCommit { tx_id, record_count: records.len() as u32 }, now_ms).await?;

        let forced = matches!(durability, DurabilityClass::Fsync | DurabilityClass::Quorum(_));
        let due = forced || self.policy.is_due(self.active.pending_bytes(), self.active.pending_records, self.active.oldest_pending_at_ms, now_ms);
        let mut committed = false;
        if due {
            committed = self.active.commit_and_flush(storage, durability).await?.is_some();
        }
        if self.active.total_len() >= self.max_segment_bytes {
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
        self.active.commit_and_flush(storage, DurabilityClass::Fsync).await?;
        let chain_hash = self.active.tip_chain_hash().await?;
        let sealed_index = self.active.index;
        storage.seal(&self.document, sealed_index).await?;
        let new_index = self.next_segment_index;
        self.next_segment_index += 1;
        self.active = SegmentWriter::begin(storage, self.document.clone(), new_index, Some(chain_hash), now_ms).await?;
        Ok(())
    }
}
//#endregion 🔖️ArtifactWal

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
        let mut prepared = bytes.prepare_platform().await.unwrap();
        let output = prepared.as_slice().to_vec();
        while prepared.close_step().unwrap() {}
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

    async fn submit_one(storage: &MemoryStorage, wal: &mut ArtifactWal, record: WalRecord, durability: DurabilityClass, now_ms: u64) -> WalAppendReceipt {
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
        Other(u8),
    }

    async fn replay_summaries(storage: &MemoryStorage, document: &ArtifactId) -> Vec<ReplaySummary> {
        let mut replay = replay_document(storage, document, control()).await.unwrap();
        let mut summaries = Vec::new();
        while let Some(mut record) = replay.next().await.unwrap() {
            let summary = match &record {
                WalRecord::SegmentHeader { segment_index, prev_chain_hash, .. } => ReplaySummary::Segment(*segment_index, *prev_chain_hash),
                WalRecord::TxBegin { tx_id } => ReplaySummary::Begin(*tx_id),
                WalRecord::Command(bytes) => ReplaySummary::Command(read_retained(bytes).await),
                WalRecord::TxCommit { tx_id, record_count } => ReplaySummary::Commit(*tx_id, *record_count),
                _ => ReplaySummary::Other(record.retained_shape().0),
            };
            summaries.push(summary);
            while record.close_step().unwrap() {}
        }
        while replay.close_step().await.unwrap() {}
        summaries
    }

    async fn segment_bytes(storage: &MemoryStorage, document: &ArtifactId, index: u64) -> Vec<u8> {
        let len = storage.segment_len(document, index).await.unwrap();
        let mut pages = storage.read(document, index, pack::ByteRange { offset: 0, len }).await.unwrap();
        let mut prepared = db_storage::db_io_prepare_platform(&pages).unwrap().await.unwrap();
        let output = prepared.as_slice().to_vec();
        while prepared.close_step().unwrap() {}
        while pages.close_step().unwrap().is_some() {}
        output
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
            assert!(is_wal_record_kind(kind).await);
        }
        // 🧪️ `0x0C` (protocol::wire::REC_COMMIT) hard-coded rather than depending on protocol_core
        // directly just for this one assertion — this crate's extension range never overlaps it.
        assert!(!is_wal_record_kind(0x0C).await);
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
    async fn torn_tail_is_recovered_by_truncating_and_replaying_trusted_prefix() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        {
            let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
            submit_one(&storage, &mut wal, WalRecord::Command(retained(b"trusted").await), DurabilityClass::Fsync, 1).await;
        }

        // Simulate a crash mid-append: bytes physically present past the last trusted commit,
        // written directly to storage (bypassing SprWriter, exactly like a torn OS-level write).
        db_actor::block_on(storage.append(&document, 0, pages(b"\x0Fgarbage-not-a-valid-frame-tail"))).unwrap();

        let (wal, report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
        assert!(report.torn_tail_bytes > 0);
        assert_eq!(report.segments_seen, 1);
        drop(wal);

        assert_eq!(replay_summaries(&storage, &document).await, vec![ReplaySummary::Segment(0, None), ReplaySummary::Begin(1), ReplaySummary::Command(b"trusted".to_vec()), ReplaySummary::Commit(1, 1)]);

        let bytes = segment_bytes(&storage, &document, 0).await;
        let post_recovery = protocol::format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
        assert_eq!(post_recovery.bytes_recovered, bytes.len() as u64, "the rebuilt segment must itself be torn-tail-free");
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
        let expected_chain_hash = protocol::format::parse_commit_payload(commit_frame.payload().await).await.unwrap().chain_hash;

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
                WalReplayStep::Yield => boundary_yields += 1,
                WalReplayStep::Done => panic!("retained replay closed without resumable segment retirement"),
            }
        }
        assert!(seen >= 1);
        while replay.close_step().await.unwrap() {}
        assert!(replay.terminal_is_empty());
    }
}
//#endregion 🧪️RetainedTests
