//! 🗄️ `db_wal` — the `db` crate family's write-ahead log: a per-document, per-segment `.spr`
//! container (reusing `protocol::{SprWriter, FrameCursor, ReverseFrameCursor}` and
//! `protocol::format::recover` directly — no new framing invented) built on top of
//! `db_storage::WalStorage`'s opaque byte segments. Owns the WAL's own record kinds (SPR
//! extension range `0x40..=0x4F`), real group-commit batching (bounded by delay/bytes/count),
//! segment rotation with a cross-segment hash chain, and crash recovery (torn-tail truncation,
//! multi-segment replay). Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
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
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::Frontier;
use db_ids::{ArtifactId, DbError, check_len};
use pack::ContentHash;

/// 📍️ First record marker in a fresh segment: document identity, segment index, and the
/// previous segment's final commit `chain_hash` (the WAL's cross-segment hash chain — protocol's
/// own commit chain resets to `chain_0 = blake3(header)` at every segment boundary, since
/// `SprWriter::begin` always starts fresh; this record is how a document's WAL stays one
/// verifiable chain across segment rotation).
pub const WAL_SEGMENT_HEADER: u8 = 0x40;

/// 🚪️ Opens a logical transaction: one group of records (typically one `submit()` batch)
/// that must be applied atomically on replay.
pub const WAL_TX_BEGIN: u8 = 0x41;

/// 🏁️ Closes a transaction successfully; carries the record count written since the
/// matching `WAL_TX_BEGIN` as a replay sanity check.
pub const WAL_TX_COMMIT: u8 = 0x42;

/// 🚫️ Closes a transaction as rolled back; replay must discard every record since the
/// matching `WAL_TX_BEGIN`.
pub const WAL_TX_ABORT: u8 = 0x43;

/// ✉️ A `protocol::MutationEnvelope`'s bytes, stored verbatim (zero-copy on the write
/// path — this crate never re-encodes what `db_artifact` hands it).
pub const WAL_COMMAND: u8 = 0x44;

/// 🫙️ A command payload, either inlined or referenced by CAS hash (see `WalPayloadRef`).
pub const WAL_PAYLOAD: u8 = 0x45;

/// 🔀️ An opaque `protocol::MutationDiff`-shaped byte blob (db crates below `db_artifact`
/// never interpret operation semantics, per the contract's hard dependency rule).
pub const WAL_DIFF: u8 = 0x46;

/// ⏪️ An opaque inverse/undo byte blob for `db_artifact`'s inverse-undo pipeline.
pub const WAL_INVERSE: u8 = 0x47;

/// 📣️ An opaque effect/notification byte blob.
pub const WAL_EVENT: u8 = 0x48;

/// 📤️ An opaque outgoing-effect outbox entry byte blob.
pub const WAL_OUTBOX: u8 = 0x49;

/// 🧭️ A structured `Frontier` snapshot, written periodically so recovery can
/// cross-check replay against a known-good checkpoint.
pub const WAL_FRONTIER: u8 = 0x4A;

/// 📸️ Marks that a snapshot generation was published, with the frontier it covers.
pub const WAL_SNAPSHOT_PUB: u8 = 0x4C;

/// 🔖️ Marks an index checkpoint: the set of `db_index` run ids current as of this point.
pub const WAL_INDEX_CKPT: u8 = 0x4D;

/// ⏳️ A lease grant/renewal/release record (`db_cluster`'s ownership-lease durability).
pub const WAL_LEASE: u8 = 0x4E;

/// 🚚️ An opaque schema/data migration descriptor byte blob.
pub const WAL_MIGRATION: u8 = 0x4F;

/// ✅️ True iff `kind` falls in this crate's SPR extension range — the test every reader
/// (`WalReplayCursor`) uses to decide whether a frame is one of ours (vs.
/// protocol's own `REC_COMMIT`, which every `.spr` stream also contains and this crate skips).
pub const fn is_wal_record_kind(kind: u8) -> bool {
    kind >= WAL_SEGMENT_HEADER && kind <= WAL_MIGRATION
}
//#region 🔖️Records
/// 🛡️ Ceiling on any single length-prefixed scalar field (document id, resource name,
/// holder id) this crate decodes — validated BEFORE allocating, mirroring
/// `pack::PackLimits`'s stated invariant. Generous for any legitimate identifier, small
/// enough to reject an obviously-corrupt length up front.
pub const MAX_FIELD_BYTES: u64 = 1024 * 1024;

/// 🛡️ Ceiling on a `WAL_INDEX_CKPT` record's run-id count, validated before the `Vec` is
/// sized.
#[cfg(test)]
const MAX_RUN_IDS: u64 = 1_000_000;

/// ✍️ Writes a varint-length-prefixed byte field — this crate's one field encoding used by
/// every string/id-shaped record field.
#[cfg(any(test, feature = "testkit"))]
async fn write_field(writer: &mut pack::ByteWriter, bytes: &[u8]) {
    writer.write_varint_u64(bytes.len() as u64);
    writer.write_bytes(bytes);
}

/// 📖️ Inverse of `write_field`.
#[cfg(test)]
async fn read_field_bytes(reader: &mut pack::ByteReader<'_>) -> Result<Vec<u8>, DbError> {
    let len = reader.read_varint_u64()?;
    check_len(len, MAX_FIELD_BYTES, "wal_record::field")?;
    Ok(reader.read_bytes(len as usize)?.to_vec())
}

/// 📖️ `read_field_bytes` plus a utf-8 validation, for text fields.
#[cfg(test)]
async fn read_field_string(reader: &mut pack::ByteReader<'_>) -> Result<String, DbError> {
    String::from_utf8(read_field_bytes(reader).await?).map_err(|_| DbError::Corrupt("wal record field is not valid utf-8".to_string()))
}

#[cfg(any(test, feature = "testkit"))]
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

pub fn wal_varint_len(value: u64) -> usize {
    let mut output = [0u8; 10];
    wal_varint(value, &mut output).len()
}

pub fn wal_read_canonical_varint(mut next: impl FnMut() -> Result<u8, DbError>) -> Result<u64, DbError> {
    let mut value = 0u64;
    for shift in (0..70).step_by(7) {
        let byte = next()?;
        if shift == 63 && byte > 1 {
            return Err(DbError::Corrupt("wal retained varint exceeds u64".to_string()));
        }
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            if shift != 0 && byte == 0 {
                return Err(DbError::Corrupt("wal retained varint is noncanonical".to_string()));
            }
            return Ok(value);
        }
    }
    Err(DbError::Corrupt("wal retained varint exceeds u64".to_string()))
}

pub fn wal_field_len(bytes: &[u8]) -> usize {
    wal_varint_len(bytes.len() as u64) + bytes.len()
}

fn wal_frontier_len(frontier: &Frontier) -> usize {
    wal_field_len(frontier.document.0.as_bytes()) + 8 + 8 + 32 + 8
}

pub async fn wal_record_write(record: &mut protocol::SprIdentityRecord<'_, SharedBuf>, bytes: &[u8]) -> Result<(), DbError> {
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
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub deadline: std::time::Instant,
    pub fuel: usize,
    pub step: Option<(std::time::Duration, usize)>,
}

impl WalCursorControl {
    pub fn new(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("wal cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel, step: None })
    }

    /// ⏱️ A control whose bound holds for each step of a whole-document scan rather than for the
    /// scan: every `renew_step` grants `stall` and `fuel` afresh, so the scan's total length follows
    /// its document's size and only a step that makes no progress within its bound fails.
    pub fn stall_bounded(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, stall: std::time::Duration, fuel: usize) -> Result<Self, DbError> {
        let mut control = Self::new(cancelled, std::time::Instant::now() + stall, fuel)?;
        control.step = Some((stall, fuel));
        Ok(control)
    }

    /// ⏱️ Renews a stall-bounded control for its next step; a control built with one fixed deadline
    /// keeps that one budget for the whole scan.
    pub fn renew_step(&mut self) -> Result<(), DbError> {
        match self.step {
            Some((stall, fuel)) => self.replenish(std::time::Instant::now() + stall, fuel),
            None => Ok(()),
        }
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("wal cursor fuel"));
        }
        self.deadline = deadline;
        self.fuel = fuel;
        Ok(())
    }

    pub fn check(&self) -> Result<(), DbError> {
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
    pub pages: db_io_core::DbIoPages,
}

pub struct WalBytesCursor<'bytes> {
    pub bytes: &'bytes WalBytes,
    pub offset: usize,
}

#[derive(Debug)]
pub struct WalBytesRejected {
    pub source: Option<Vec<u8>>,
    pub writer: Option<db_io_core::DbIoPageWriter>,
    pub error: DbError,
}

impl WalBytes {
    pub async fn try_admit(source: Vec<u8>, maximum: u64, control: &mut WalCursorControl) -> Result<Self, WalBytesRejected> {
        if source.capacity() as u64 > maximum {
            return Err(WalBytesRejected { source: Some(source), writer: None, error: DbError::LimitExceeded("wal source backing capacity") });
        }
        let mut writer = match db_io_core::DbIoPageWriter::try_reserve(source.capacity().div_ceil(db_io_core::DB_IO_PAGE_BYTES)) {
            Ok(writer) => writer,
            Err(rejected) => return Err(WalBytesRejected { source: Some(source), writer: rejected.into_writer(), error: DbError::Unavailable("wal page admission rejected".to_string()) }),
        };
        let mut reservation = match db_io_core::DbIoDriverReservation::try_reserve(writer.operation(), source.capacity()) {
            Ok(reservation) => reservation,
            Err(error) => return Err(WalBytesRejected { source: Some(source), writer: Some(writer), error }),
        };
        let mut source = db_io_core::DbIoExternalBytes::new(source);
        if let Err(error) = source.capacity().and_then(|capacity| reservation.observe_capacity(capacity)) {
            return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
        }
        let mut offset = 0;
        let mut turn = WalTurn::start();
        while offset < source.as_slice().map_err(|error| WalBytesRejected { source: None, writer: None, error })?.len() {
            if let Err(error) = control.grant() {
                return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error });
            }
            match source.as_slice().and_then(|source| writer.write_fragment(&source[offset..])) {
                Ok(written) => offset += written,
                Err(error) => return Err(WalBytesRejected { source: source.into_value().ok(), writer: Some(writer), error }),
            }
            turn.step().await;
        }
        while !source.terminal_is_empty() {
            if let Err(error) = control.grant() {
                return Err(WalBytesRejected { source: None, writer: Some(writer), error });
            }
            let _ = source.close_step();
            turn.step().await;
        }
        if let Err(error) = reservation.close_step() {
            return Err(WalBytesRejected { source: None, writer: Some(writer), error });
        }
        writer.seal_retained().await.map(|pages| Self { pages }).map_err(|rejected| {
            let (error, writer) = rejected.into_parts();
            WalBytesRejected { source: None, writer, error }
        })
    }

    /// 🧺️ Stages `source` under an operation that already holds DB I/O credit, so every record
    /// of one transaction shares one ledger slot instead of each claiming its own.
    pub async fn copy_for_operation(operation: u64, source: &[u8], control: &mut WalCursorControl) -> Result<Self, DbError> {
        let mut writer = db_io_core::DbIoPageWriter::try_reserve_for_operation(operation, source.len().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        let mut offset = 0;
        let mut turn = WalTurn::start();
        while offset < source.len() {
            control.grant()?;
            offset += writer.write_fragment(&source[offset..])?;
            turn.step().await;
        }
        writer.seal_retained().await.map(|pages| Self { pages }).map_err(db_io_core::DbIoPageWriterRejected::into_error)
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

    pub fn cursor(&self) -> WalBytesCursor<'_> {
        WalBytesCursor { bytes: self, offset: 0 }
    }

    #[cfg(test)]
    async fn prepare_platform(&self) -> Result<db_io_registry::DbIoPlatformBuffer, DbError> {
        db_io_registry::db_io_prepare_platform(&self.pages)?.await
    }

    /// 🔏️ The record's content hash in one bounded step (a record spans at most one operation's pages).
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = semio_framework_hash::Hasher::new();
        for fragment in self.fragments() {
            hasher.update(fragment);
        }
        *hasher.finalize().as_bytes()
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
        let page = (self.offset / db_io_core::DB_IO_PAGE_BYTES) as u8;
        let page_offset = self.offset % db_io_core::DB_IO_PAGE_BYTES;
        let byte = self.bytes.pages.page(page).and_then(|fragment| fragment.get(page_offset)).copied().ok_or_else(|| DbError::Corrupt("wal retained field ended early".to_string()))?;
        self.offset += 1;
        Ok(byte)
    }

    pub fn varint(&mut self, control: &mut WalCursorControl) -> Result<u64, DbError> {
        let start = self.offset;
        let result = wal_read_canonical_varint(|| self.byte(control));
        if result.is_err() {
            self.offset = start;
        }
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
        let page = (self.offset / db_io_core::DB_IO_PAGE_BYTES) as u8;
        let page_offset = self.offset % db_io_core::DB_IO_PAGE_BYTES;
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

    /// 📦️ One length-prefixed field copied out page by page; its length is checked against
    /// `maximum` and against what the record still holds before anything is allocated.
    pub fn field(&mut self, maximum: u64, control: &mut WalCursorControl) -> Result<Vec<u8>, DbError> {
        let mut remaining = self.begin_field(maximum, control)?;
        let mut output = Vec::with_capacity(remaining);
        while remaining != 0 {
            control.grant()?;
            let page = (self.offset / db_io_core::DB_IO_PAGE_BYTES) as u8;
            let page_offset = self.offset % db_io_core::DB_IO_PAGE_BYTES;
            let fragment = self.bytes.pages.page(page).ok_or_else(|| DbError::Corrupt("wal retained field ended early".to_string()))?;
            let copied = remaining.min(fragment.len().saturating_sub(page_offset));
            if copied == 0 {
                return Err(DbError::Corrupt("wal retained field cursor stalled".to_string()));
            }
            output.extend_from_slice(&fragment[page_offset..page_offset + copied]);
            self.offset += copied;
            remaining -= copied;
        }
        Ok(output)
    }

    pub fn text(&mut self, maximum: u64, control: &mut WalCursorControl) -> Result<String, DbError> {
        String::from_utf8(self.field(maximum, control)?).map_err(|_| DbError::Corrupt("wal retained text is not valid utf-8".to_string()))
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
            drop(db_io_core::DbIoExternalBytes::new(source));
        }
    }
}

/// 🫙️ `WAL_PAYLOAD`'s two shapes: small payloads inline, large ones by CAS reference into
/// `db_storage::PayloadStorage` — mirrors that trait's own blake3-CAS design.
pub enum WalPayloadRef {
    Inline(WalBytes),
    CasRef(ContentHash),
}

/// 📜️ One decoded WAL record — the typed shape every `WAL_*` kind decodes to/encodes from.
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
    SnapshotPub { generation: u64, frontier: Frontier },
    IndexCkpt { run_ids: db_io_core::DbIoU64List },
    Lease { resource: db_io_core::DbIoText, holder: db_io_core::DbIoText, fence: u64, expires_at_ms: u64 },
    Migration(WalBytes),
}

const WAL_RECORD_BATCH_CAPACITY: usize = 64;

pub struct WalRecordBatch {
    pub records: Box<[Option<WalRecord>]>,
    pub len: u8,
}

impl WalRecordBatch {
    pub fn new() -> Self {
        Self { records: (0..WAL_RECORD_BATCH_CAPACITY).map(|_| None).collect(), len: 0 }
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
    /// 🔢️ The transaction id carried by a `TxBegin`/`TxCommit`/`TxAbort` record, or `None`
    /// for every other kind — used by `ArtifactWal::open` to resume `next_tx_id` past whatever was
    /// already durable.
    pub fn tx_id(&self) -> Option<u64> {
        match self {
            WalRecord::TxBegin { tx_id } | WalRecord::TxCommit { tx_id, .. } | WalRecord::TxAbort { tx_id } => Some(*tx_id),
            _ => None,
        }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        match self {
            Self::Command(bytes) | Self::Diff(bytes) | Self::Inverse(bytes) | Self::Event(bytes) | Self::Outbox(bytes) | Self::Migration(bytes) | Self::Payload(WalPayloadRef::Inline(bytes)) => Ok(bytes.close_step()?.is_some()),
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
            Self::IndexCkpt { run_ids } => run_ids.terminal_is_empty(),
            Self::Lease { resource, holder, .. } => resource.terminal_is_empty() && holder.terminal_is_empty(),
            _ => true,
        }
    }

    /// ✍️ Encodes `self` to its on-disk `(kind, critical, payload)` triple, ready for
    /// `protocol::SprWriter::write_record`. Every kind is critical: unlike protocol's own
    /// history-log records (where e.g. a dictionary delta can plausibly be "skippable" to some
    /// future reader), every `WAL_*` record is load-bearing for correct replay — there is no
    /// optional WAL record in this crate's design.
    #[cfg(any(test, feature = "testkit"))]
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

    pub fn retained_shape(&self) -> (u8, usize) {
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
            Self::SnapshotPub { frontier, .. } => (WAL_SNAPSHOT_PUB, 8 + wal_frontier_len(frontier)),
            Self::IndexCkpt { run_ids } => (WAL_INDEX_CKPT, wal_varint_len(run_ids.len() as u64) + run_ids.len() * 8),
            Self::Lease { resource, holder, .. } => (WAL_LEASE, wal_field_len(resource.as_str().as_bytes()) + wal_field_len(holder.as_str().as_bytes()) + 16),
            Self::Migration(bytes) => (WAL_MIGRATION, bytes.len()),
        }
    }

    pub async fn write_retained(&self, writer: &mut protocol::SprWriter<SharedBuf>) -> Result<u64, DbError> {
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

    /// 📖️ Inverse of `encode`. Errors `DbError::Corrupt` on an unrecognized `kind` (a
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
            WAL_SNAPSHOT_PUB => {
                let generation = reader.read_u64_le()?;
                WalRecord::SnapshotPub { generation, frontier: decode_frontier(&mut reader).await? }
            }
            WAL_INDEX_CKPT => {
                let count = reader.read_varint_u64()?;
                check_len(count, MAX_RUN_IDS, "wal_record::index_ckpt run_ids")?;
                let mut run_ids = db_io_core::DbIoU64List::new();
                for _ in 0..count {
                    control.grant()?;
                    run_ids.push(reader.read_u64_le()?)?;
                }
                WalRecord::IndexCkpt { run_ids }
            }
            WAL_LEASE => {
                let resource = db_io_core::DbIoText::try_from_str(&read_field_string(&mut reader).await?)?;
                let holder = db_io_core::DbIoText::try_from_str(&read_field_string(&mut reader).await?)?;
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
/// 🔐️ The encryption hook for `WAL_PAYLOAD` bytes: a caller building a `WalPayloadRef`
/// applies `encrypt` to the plaintext BEFORE wrapping it as `WalPayloadRef::Inline` or handing it to
/// `db_storage::PayloadStorage::put` for a `WalPayloadRef::CasRef`, and applies `decrypt` after
/// reading either form back — this crate itself never calls `PayloadStorage` (see `WalPayloadRef`'s
/// doc: the inline-vs-CAS choice is deliberately the caller's, keeping `db_wal` decoupled from that
/// trait), so it cannot invoke the hook itself either; it only defines the seam. Per the contract:
/// a trait only, no real implementation here — external crypto libraries stay behind it (the
/// family's "external libs behind an interface" rule).
pub trait PayloadTransform: Send + Sync {
    /// 🔒️ Transforms `plaintext` before it is embedded inline or stored via `PayloadStorage`.
    async fn encrypt(&self, plaintext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError>;
    /// 🔓️ Inverts `encrypt` — must exactly reconstruct the original bytes.
    async fn decrypt(&self, ciphertext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError>;
}

/// 🪟️ A `PayloadTransform` that passes bytes through unchanged — the default for a
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
/// ⏱️ Bounds a segment's group-commit batching: whichever of delay/bytes/records is hit
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
    /// ⏰️ True iff any bound is currently exceeded — `ArtifactWal::submit` also always
    /// commits immediately regardless of this policy when the caller requests `Fsync`/`Quorum`
    /// durability (a durability request stronger than what's already durable can never be
    /// satisfied by deferring the commit).
    pub fn is_due(&self, pending_bytes: u64, pending_records: u32, oldest_pending_at_ms: Option<u64>, now_ms: u64) -> bool {
        pending_bytes >= self.max_bytes || pending_records >= self.max_records || oldest_pending_at_ms.is_some_and(|started| now_ms.saturating_sub(started) >= self.max_delay_ms)
    }
}
//#endregion 🔖️GroupCommit
//#region 🔖️Sink
/// 📐️ Pages one active segment's tail window may ever hold: the window never outgrows the
/// readable segment it belongs to.
const WAL_TAIL_WINDOW_PAGES: usize = (db_io_core::DB_IO_MAX_READ_BYTES as usize).div_ceil(db_io_core::DB_IO_PAGE_BYTES);

/// 🔢️ Writers one tail window may chain under its one I/O operation. Each new writer is at
/// least as large as the whole window before it, so this many always reach `WAL_TAIL_WINDOW_PAGES`.
const WAL_TAIL_WINDOW_WRITERS: usize = 6;

const _: () = assert!(1 << (WAL_TAIL_WINDOW_WRITERS - 1) >= WAL_TAIL_WINDOW_PAGES);

/// 🪟️ The unflushed tail of one active WAL segment, held in fixed DB I/O pages only while
/// records wait for their commit and flush. `base` is the absolute segment offset of the window's
/// first byte: everything before it is durable in `WalStorage` and no longer in memory, so an idle
/// document holds no I/O pages at all. Holding the whole segment image instead charged every mounted
/// document a full operation's pages for as long as it stayed mounted, and a process could not mount
/// its sixteenth document.
struct WalTailWindow {
    pub base: u64,
    writers: [Option<db_io_core::DbIoPageWriter>; WAL_TAIL_WINDOW_WRITERS],
    capacities: [usize; WAL_TAIL_WINDOW_WRITERS],
}

impl WalTailWindow {
    pub fn at(base: u64) -> Self {
        Self { base, writers: std::array::from_fn(|_| None), capacities: [0; WAL_TAIL_WINDOW_WRITERS] }
    }

    fn written(&self) -> usize {
        self.writers.iter().flatten().map(db_io_core::DbIoPageWriter::len).sum()
    }

    pub fn len(&self) -> u64 {
        self.base + self.written() as u64
    }

    pub fn admit(&mut self, bytes: u64) -> Result<(), DbError> {
        let bytes = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("wal tail window admission"))?;
        let reserved = self.capacities.iter().sum::<usize>();
        let free = (reserved * db_io_core::DB_IO_PAGE_BYTES).checked_sub(self.written()).ok_or_else(|| DbError::Internal("WAL tail window wrote past its pages".to_string()))?;
        if bytes <= free {
            return Ok(());
        }
        let needed = (bytes - free).div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        let pages = needed.max(reserved).min(WAL_TAIL_WINDOW_PAGES.saturating_sub(reserved));
        let slot = self.writers.iter().position(Option::is_none);
        let Some(slot) = slot.filter(|_| pages >= needed) else { return Err(DbError::LimitExceeded("wal tail window")) };
        let writer = match self.writers.iter().flatten().next() {
            Some(first) => db_io_core::DbIoPageWriter::try_reserve_for_operation(first.operation(), pages),
            None => db_io_core::DbIoPageWriter::try_reserve(pages),
        }
        .map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        self.writers[slot] = Some(writer);
        self.capacities[slot] = pages;
        Ok(())
    }

    fn write_step(&mut self, bytes: &[u8]) -> Result<usize, DbError> {
        for (writer, capacity) in self.writers.iter_mut().zip(self.capacities) {
            let Some(writer) = writer.as_mut() else { break };
            if writer.len() < capacity * db_io_core::DB_IO_PAGE_BYTES {
                return writer.write_fragment(bytes);
            }
        }
        Err(DbError::LimitExceeded("wal tail window admission"))
    }

    fn read_step(&self, offset: u64, output: &mut [u8]) -> Result<usize, DbError> {
        let relative = offset.checked_sub(self.base).ok_or_else(|| DbError::Internal("WAL tail window no longer holds a flushed offset".to_string()))?;
        let mut relative = usize::try_from(relative).map_err(|_| DbError::LimitExceeded("wal tail window offset"))?;
        for writer in self.writers.iter().flatten() {
            if relative < writer.len() {
                return writer.read_fragment(relative, output);
            }
            relative -= writer.len();
        }
        Ok(0)
    }

    pub fn release_step(&mut self) -> Result<bool, DbError> {
        for writer in self.writers.iter_mut().rev().flatten() {
            if writer.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.base = self.len();
        self.writers = std::array::from_fn(|_| None);
        self.capacities = [0; WAL_TAIL_WINDOW_WRITERS];
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.writers.iter().flatten().all(db_io_core::DbIoPageWriter::terminal_is_empty)
    }
}

/// 🪞️ A `semio_framework_pack_source::PackSink` over one segment's `WalTailWindow`. `protocol::SprWriter` owns one
/// clone and `SegmentWriter` another to flush the unflushed suffix to `db_storage::WalStorage` —
/// `SprWriter` has no public accessor for its private `sink` field. Every write is admitted first
/// (`admit`), so a full process budget refuses a transaction before any of its bytes exist.
#[derive(Clone)]
pub struct SharedBuf(pub std::sync::Arc<std::sync::Mutex<WalTailWindow>>);

/// 🩹️ Recovers a poisoned lock instead of panicking — one panicking document actor must
/// never turn every other document's WAL access into a cascading panic (mirrors `db_storage`'s
/// own `MemoryStorage` convention).
// 🚫️async: E1 pure accessor (no suspension: `Mutex::lock` on a never-genuinely-contended
// in-process lock), consumed synchronously alongside `.await`ed uses elsewhere — see R9
fn lock<T>(mutex: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl SharedBuf {
    /// 📍️ An empty window whose next byte lands at absolute segment offset `base`.
    pub fn at(base: u64) -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(WalTailWindow::at(base))))
    }

    pub fn len(&self) -> u64 {
        lock(&self.0).len()
    }

    /// 🎟️ Ensures the next `bytes` written fit the window's pages, growing it by one writer
    /// under the window's own I/O operation when they do not.
    pub fn admit(&self, bytes: u64) -> Result<(), DbError> {
        lock(&self.0).admit(bytes)
    }

    /// 🧺️ Returns one flushed page (or writer shell) to the process budget; `false` once the
    /// window is empty again and rebased past everything it held.
    pub fn release_step(&self) -> Result<bool, DbError> {
        lock(&self.0).release_step()
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if std::sync::Arc::strong_count(&self.0) != 1 {
            return Err(DbError::Internal("WAL retained buffer still has a writer clone".to_string()));
        }
        self.release_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        std::sync::Arc::strong_count(&self.0) == 1 && lock(&self.0).terminal_is_empty()
    }

    pub async fn copy_range(&self, offset: usize, len: usize) -> Result<db_io_core::DbIoPages, DbError> {
        let mut output = db_io_core::DbIoPageWriter::try_reserve(len.div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        let mut copied = 0;
        let mut fragment = [0u8; db_io_core::DB_IO_PAGE_BYTES];
        while copied < len {
            let requested = (len - copied).min(fragment.len());
            let read = lock(&self.0).read_step((offset + copied) as u64, &mut fragment[..requested])?;
            if read == 0 {
                return Err(DbError::Corrupt("WAL retained page range ended early".to_string()));
            }
            let mut written = 0;
            while written < read {
                let count = output.write_fragment(&fragment[written..read])?;
                if count == 0 {
                    return Err(DbError::LimitExceeded("WAL retained suffix writer"));
                }
                written += count;
                semio_framework_async::yield_once().await;
            }
            copied += read;
            semio_framework_async::yield_once().await;
        }
        output.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }

    pub async fn read_exact(&self, offset: usize, output: &mut [u8]) -> Result<(), DbError> {
        let mut copied = 0;
        while copied < output.len() {
            let read = lock(&self.0).read_step((offset + copied) as u64, &mut output[copied..])?;
            if read == 0 {
                return Err(DbError::Corrupt("WAL retained page read ended early".to_string()));
            }
            copied += read;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }
}

impl semio_framework_pack_source::PackSink for SharedBuf {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), pack::PackError> {
        let mut cursor = 0;
        while cursor < bytes.len() {
            let written = lock(&self.0).write_step(&bytes[cursor..]).map_err(|error| pack::PackError::Io(error.to_string()))?;
            if written == 0 {
                return Err(pack::PackError::Io("WAL tail window accepted no bytes".to_string()));
            }
            cursor += written;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }

    async fn position(&self) -> u64 {
        self.len()
    }
}

/// ⛓️ Every `db_wal` segment is opened requiring `protocol::wire::REQUIRED_HASH_CHAIN` —
/// `SprWriter` always computes the commit chain regardless of this flag, but stamping it into the
/// header makes the requirement an explicit, reader-enforced part of the file's contract rather
/// than an implicit convention only this crate happens to uphold.
// 🚫️async: E1 pure constructor, always used by reference inline (`&segment_write_options()`) — see R9
pub fn segment_write_options() -> protocol::format::WriteOptions {
    protocol::format::WriteOptions { required_flags: protocol::wire::REQUIRED_HASH_CHAIN, optional_flags: 0 }
}
//#endregion 🔖️Sink
//#region 🔖️Recovery
/// 🚨️ Maps `protocol::ProtocolError` (this crate never leaks it in a public signature, per
/// the family's `DbError`-only rule) onto the closest `DbError` variant.
// 🚫️async: E4 fn-pointer slot
pub fn protocol_err(err: protocol::ProtocolError) -> DbError {
    match err {
        protocol::ProtocolError::Pack(pack_error) => DbError::from(pack_error),
        protocol::ProtocolError::Io(message) => DbError::Io(message),
        protocol::ProtocolError::LimitExceeded(what) => DbError::LimitExceeded(what),
        other => DbError::Corrupt(other.to_string()),
    }
}

struct WalPageSource<'pages>(&'pages db_io_core::DbIoPages);

impl semio_framework_pack_source::PackSource for WalPageSource<'_> {
    async fn len(&self) -> u64 {
        self.0.len() as u64
    }

    async fn read_at(&self, offset: u64, output: &mut [u8]) -> Result<usize, protocol::codec::PackError> {
        let offset = usize::try_from(offset).map_err(|_| protocol::codec::PackError::Refusal(protocol::codec::PackRefusal::Truncated(offset)))?;
        if offset > self.0.len() {
            return Err(protocol::codec::PackError::Refusal(protocol::codec::PackRefusal::Truncated(offset as u64)));
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

pub trait WalImmutableByteSource: Sync {
    fn byte_len(&self) -> usize;
    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError>;
}

impl<S: WalImmutableByteSource + ?Sized> WalImmutableByteSource for &S {
    fn byte_len(&self) -> usize {
        (**self).byte_len()
    }
    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
        (**self).fragment_at(offset, limit)
    }
}

impl WalImmutableByteSource for db_io_core::DbIoPages {
    fn byte_len(&self) -> usize {
        self.len()
    }

    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
        if offset >= limit || limit > self.len() {
            return Err(DbError::Corrupt("wal immutable source range".to_string()));
        }
        let first = self.page(0).ok_or_else(|| DbError::Corrupt("wal immutable source lost first page".to_string()))?.len();
        let (page, local) = if offset < first {
            (0, offset)
        } else {
            let rest = offset - first;
            (1 + rest / db_io_core::DB_IO_PAGE_BYTES, rest % db_io_core::DB_IO_PAGE_BYTES)
        };
        let bytes = self.page(u8::try_from(page).map_err(|_| DbError::LimitExceeded("wal immutable source page"))?).ok_or_else(|| DbError::Corrupt("wal immutable source lost page".to_string()))?;
        let remaining = bytes.len().checked_sub(local).ok_or_else(|| DbError::Corrupt("wal immutable source offset".to_string()))?;
        let count = remaining.min(limit - offset);
        if count == 0 {
            return Err(DbError::Corrupt("wal immutable source empty fragment".to_string()));
        }
        Ok(&bytes[local..local + count])
    }
}

pub struct WalPageReader<'pages> {
    pub pages: &'pages dyn WalImmutableByteSource,
    pub position: usize,
    pub limit: usize,
}

impl<'pages> WalPageReader<'pages> {
    pub fn new(pages: &'pages dyn WalImmutableByteSource, position: usize, limit: usize) -> Result<Self, DbError> {
        if position > limit || limit > pages.byte_len() {
            return Err(DbError::Corrupt("wal retained reader authority".to_string()));
        }
        Ok(Self { pages, position, limit })
    }

    pub fn fragment(&self) -> Result<&'pages [u8], DbError> {
        self.pages.fragment_at(self.position, self.limit)
    }

    pub fn byte(&mut self) -> Result<u8, DbError> {
        let byte = self.fragment()?[0];
        self.position += 1;
        Ok(byte)
    }

    fn varint(&mut self) -> Result<u64, DbError> {
        let start = self.position;
        let result = wal_read_canonical_varint(|| self.byte());
        if result.is_err() {
            self.position = start;
        }
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

    fn text(&mut self) -> Result<db_io_core::DbIoText, DbError> {
        let len = self.varint()?;
        check_len(len, db_io_core::DbIoText::maximum_capacity() as u64, "wal retained text")?;
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
        db_io_core::DbIoText::try_from_str(value)
    }

    fn string(&mut self) -> Result<String, DbError> {
        let mut text = self.text()?;
        let output = text.as_str().to_string();
        text.close_step();
        Ok(output)
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn wal_crc_range(pages: &db_io_core::DbIoPages, start: usize, len: usize, control: &mut WalCursorControl) -> Result<u32, DbError> {
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
    pub kind: u8,
    pub payload_start: usize,
    pub payload_end: usize,
    pub frame_end: usize,
}

enum WalRetainedDecodeState {
    Start,
    Bytes { position: usize, end: usize },
    Index { position: usize, remaining: usize },
    Done,
}

struct WalRetainedRecordDecoder {
    pub frame: WalRecordFrame,
    pub state: WalRetainedDecodeState,
    pub writer: Option<db_io_core::DbIoPageWriter>,
    run_ids: Option<db_io_core::DbIoU64List>,
}

impl WalRetainedRecordDecoder {
    pub fn new(frame: WalRecordFrame) -> Self {
        Self { frame, state: WalRetainedDecodeState::Start, writer: None, run_ids: None }
    }

    pub fn step(&mut self, pages: &db_io_core::DbIoPages, control: &mut WalCursorControl) -> Result<Option<WalRecord>, DbError> {
        control.grant()?;
        match &mut self.state {
            WalRetainedDecodeState::Start => {
                let mut reader = WalPageReader::new(pages, self.frame.payload_start, self.frame.payload_end)?;
                if self.frame.kind == WAL_INDEX_CKPT {
                    let count = reader.varint()?;
                    check_len(count, 4_096, "wal retained checkpoint")?;
                    if reader.position.checked_add(count as usize * 8) != Some(reader.limit) {
                        return Err(DbError::Corrupt("wal checkpoint payload length differs".to_string()));
                    }
                    self.run_ids = Some(db_io_core::DbIoU64List::new());
                    self.state = WalRetainedDecodeState::Index { position: reader.position, remaining: count as usize };
                    return Ok(None);
                }
                let opaque = matches!(self.frame.kind, WAL_COMMAND | WAL_DIFF | WAL_INVERSE | WAL_EVENT | WAL_OUTBOX | WAL_MIGRATION);
                let inline = if self.frame.kind == WAL_PAYLOAD {
                    match reader.byte()? {
                        0 => {
                            let len = reader.varint()?;
                            check_len(len, MAX_FIELD_BYTES, "wal retained payload")?;
                            if reader.position.checked_add(len as usize) != Some(reader.limit) {
                                return Err(DbError::Corrupt("wal inline payload length differs".to_string()));
                            }
                            true
                        }
                        1 => false,
                        _ => return Err(DbError::Corrupt("wal payload tag".to_string())),
                    }
                } else {
                    false
                };
                if opaque || inline {
                    self.state = WalRetainedDecodeState::Bytes { position: reader.position, end: reader.limit };
                    match db_io_core::DbIoPageWriter::try_reserve_for_operation(pages.operation(), (reader.limit - reader.position).div_ceil(db_io_core::DB_IO_PAGE_BYTES)) {
                        Ok(writer) => self.writer = Some(writer),
                        Err(rejected) => {
                            let (error, writer) = rejected.into_parts();
                            self.writer = writer;
                            return Err(error);
                        }
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
                    if written == 0 {
                        return Err(DbError::LimitExceeded("wal retained decoder made no progress"));
                    }
                    *position += written;
                    return Ok(None);
                }
                let Some(pages) = writer.seal_retained_step()? else { return Ok(None) };
                self.writer = None;
                self.state = WalRetainedDecodeState::Done;
                let bytes = WalBytes { pages };
                Ok(Some(match self.frame.kind {
                    WAL_COMMAND => WalRecord::Command(bytes),
                    WAL_PAYLOAD => WalRecord::Payload(WalPayloadRef::Inline(bytes)),
                    WAL_DIFF => WalRecord::Diff(bytes),
                    WAL_INVERSE => WalRecord::Inverse(bytes),
                    WAL_EVENT => WalRecord::Event(bytes),
                    WAL_OUTBOX => WalRecord::Outbox(bytes),
                    WAL_MIGRATION => WalRecord::Migration(bytes),
                    _ => unreachable!(),
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

    pub fn close_owner_step(&mut self) -> Result<bool, DbError> {
        if let Some(writer) = self.writer.as_mut() {
            if writer.close_step()?.is_some() {
                return Ok(true);
            }
        }
        self.writer = None;
        if let Some(run_ids) = self.run_ids.as_mut() {
            if run_ids.close_step() {
                return Ok(true);
            }
        }
        self.run_ids = None;
        self.state = WalRetainedDecodeState::Done;
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.run_ids.is_none() && matches!(self.state, WalRetainedDecodeState::Done)
    }
}

pub enum WalVerifiedFrameStep {
    Frame(WalRecordFrame),
    PhysicalCommit,
    Done,
}

/// 🧷️ Reads one span only after `WalSegmentChain` verified these exact immutable pages in full.
pub fn wal_next_verified_page_frame(pages: &dyn WalImmutableByteSource, offset: &mut usize, trusted_len: usize, control: &mut WalCursorControl) -> Result<WalVerifiedFrameStep, DbError> {
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

/// 🧮️ Most body records one WAL transaction holds: a declared-legal document batch's commands
/// (`protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES`) plus its frontier. `ArtifactWal::preflight_submit`
/// refuses a larger transaction before any I/O and `WalTransactionGate` holds up to exactly this many frames, so
/// every transaction the writer admits is one recovery and replay read back (it held 64 while the writer bounded a
/// transaction only by the readable segment: a committed batch of 64 or more envelopes left its document unopenable).
pub const WAL_TRANSACTION_RECORDS_MAX: usize = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES + 1;

pub struct WalTransactionGate {
    pub transaction_id: Option<u64>,
    pub ready: Option<u64>,
    pub frames: Vec<WalRecordFrame>,
    pub next_tx_id: u64,
    header_seen: bool,
    transaction_seen: bool,
}

impl WalTransactionGate {
    pub fn new() -> Self {
        Self { transaction_id: None, ready: None, frames: Vec::new(), next_tx_id: 1, header_seen: false, transaction_seen: false }
    }

    fn clear_frames(&mut self) {
        self.frames.clear();
    }

    pub fn committed_frame(&self, index: usize) -> Option<WalRecordFrame> {
        self.ready?;
        self.frames.get(index).copied()
    }

    pub fn push(&mut self, pages: &dyn WalImmutableByteSource, frame: WalRecordFrame) -> Result<bool, DbError> {
        if self.ready.is_some() {
            return Err(DbError::Corrupt("wal committed transaction remains borrowed".to_string()));
        }
        if frame.kind == WAL_SEGMENT_HEADER {
            if self.header_seen || self.transaction_id.is_some() {
                return Err(DbError::Corrupt("wal logical segment header repeated".to_string()));
            }
            self.header_seen = true;
            return Ok(false);
        }
        if !self.header_seen {
            return Err(DbError::Corrupt("wal logical segment header missing".to_string()));
        }
        let mut reader = WalPageReader::new(pages, frame.payload_start, frame.payload_end)?;
        match frame.kind {
            WAL_TX_BEGIN => {
                let tx_id = u64::from_le_bytes(reader.array()?);
                if reader.position != reader.limit || self.transaction_id.is_some() || tx_id < self.next_tx_id || (self.transaction_seen && tx_id != self.next_tx_id) {
                    return Err(DbError::Corrupt("wal transaction begin is nested or out of sequence".to_string()));
                }
                self.next_tx_id = tx_id.checked_add(1).ok_or(DbError::LimitExceeded("wal transaction sequence"))?;
                self.transaction_id = Some(tx_id);
                self.transaction_seen = true;
                Ok(false)
            }
            WAL_TX_COMMIT | WAL_TX_ABORT => {
                let tx_id = u64::from_le_bytes(reader.array()?);
                let count = if frame.kind == WAL_TX_COMMIT { Some(u32::from_le_bytes(reader.array()?)) } else { None };
                if reader.position != reader.limit || self.transaction_id != Some(tx_id) || count.is_some_and(|count| usize::try_from(count).ok() != Some(self.frames.len())) {
                    return Err(DbError::Corrupt("wal logical terminal id or count differs".to_string()));
                }
                self.transaction_id = None;
                if count.is_some() {
                    self.ready = Some(tx_id);
                    Ok(true)
                } else {
                    self.clear_frames();
                    Ok(false)
                }
            }
            WAL_COMMAND..=WAL_MIGRATION => {
                if self.transaction_id.is_none() {
                    return Err(DbError::Corrupt("wal body outside a transaction".to_string()));
                }
                if self.frames.len() >= WAL_TRANSACTION_RECORDS_MAX {
                    return Err(DbError::LimitExceeded("wal transaction records"));
                }
                self.frames.try_reserve(1).map_err(|_| DbError::LimitExceeded("wal transaction records"))?;
                self.frames.push(frame);
                Ok(false)
            }
            _ => Err(DbError::Corrupt("unknown wal logical record kind".to_string())),
        }
    }

    pub fn release(&mut self) -> Result<(), DbError> {
        self.ready.take().ok_or_else(|| DbError::Corrupt("wal has no committed transaction to finish".to_string()))?;
        self.clear_frames();
        Ok(())
    }

    pub fn finish_segment(&self, writable_highest: bool) -> Result<Option<u64>, DbError> {
        if !self.header_seen || self.ready.is_some() {
            return Err(DbError::Corrupt("wal logical segment is not drained".to_string()));
        }
        if self.transaction_id.is_some() && !writable_highest {
            return Err(DbError::Corrupt("wal logical transaction crosses a sealed boundary".to_string()));
        }
        Ok(self.transaction_id)
    }

    pub fn advance_segment(&mut self) -> Result<(), DbError> {
        self.finish_segment(false)?;
        self.header_seen = false;
        Ok(())
    }
}

/// 📋️ What `ArtifactWal::open` found while recovering a document's WAL.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalRecoveryReport {
    pub segments_seen: u64,
    pub records_replayed: u64,
    /// ✂️ Bytes discarded off the active segment's tail because they came after the last
    /// trusted commit (a torn write from a crash mid-append) — `0` on a clean recovery.
    pub torn_tail_bytes: u64,
    /// 🧯️ Incomplete active transaction durably aborted before this opener returned.
    pub recovered_abort_tx_id: Option<u64>,
}

struct WalChainFrame {
    start: usize,
    pub body: usize,
    pub end: usize,
    pub next: usize,
    pub position: usize,
    pub kind: u8,
    digest: semio_framework_hash::Hasher,
    crc: protocol::codec::Crc32cCursor,
}

pub struct WalSegmentChain {
    pub offset: usize,
    pub frame: Option<WalChainFrame>,
    pending: semio_framework_hash::Hasher,
    records_len: u64,
    pub record_count: u32,
    next_commit: u64,
    pub last_commit_offset: u64,
    pub tip: Option<[u8; 32]>,
    previous_tip: WalPriorChainTip,
    pub segment: u64,
    header_seen: bool,
}

#[derive(Clone, Copy)]
pub enum WalPriorChainTip {
    Genesis,
    RetainedBoundary,
    Verified([u8; 32]),
}

impl WalSegmentChain {
    pub fn new(pages: &dyn WalImmutableByteSource, segment: u64, previous_tip: WalPriorChainTip) -> Result<Self, DbError> {
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
        if reader.varint()? != document.0.len() as u64 {
            return Err(DbError::Corrupt("wal segment document differs".to_string()));
        }
        for byte in document.0.as_bytes() {
            if reader.byte()? != *byte {
                return Err(DbError::Corrupt("wal segment document differs".to_string()));
            }
        }
        if u64::from_le_bytes(reader.array()?) != self.segment {
            return Err(DbError::Corrupt("wal segment index differs".to_string()));
        }
        let previous = match reader.byte()? {
            0 => None,
            1 => Some(reader.array::<32>()?),
            _ => return Err(DbError::Corrupt("wal previous tip marker differs".to_string())),
        };
        let matches = match self.previous_tip {
            WalPriorChainTip::Genesis => previous.is_none(),
            WalPriorChainTip::RetainedBoundary => previous.is_some(),
            WalPriorChainTip::Verified(tip) => previous == Some(tip),
        };
        if !matches || reader.position != frame.end {
            return Err(DbError::Corrupt("wal previous committed chain tip differs".to_string()));
        }
        self.header_seen = true;
        Ok(())
    }

    pub fn step(&mut self, pages: &dyn WalImmutableByteSource, trusted_len: usize, document: &ArtifactId, control: &mut WalCursorControl) -> Result<bool, DbError> {
        control.grant()?;
        if self.frame.is_none() {
            if self.offset == trusted_len {
                if !self.header_seen || self.tip.is_none() || self.record_count != 0 || self.records_len != 0 || self.last_commit_offset.checked_add(protocol::format::COMMIT_FRAME_LEN) != Some(trusted_len as u64) {
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
            if length < 2 || next > trusted_len {
                return Err(DbError::Corrupt("wal chain frame exceeds committed bytes".to_string()));
            }
            let kind = reader.byte()?;
            let flags = reader.byte()?;
            if flags != protocol::wire::FRAME_FLAG_CRITICAL {
                return Err(DbError::Corrupt("wal chain frame flags differ".to_string()));
            }
            self.frame = Some(WalChainFrame { start: self.offset, body, end, next, position: self.offset, kind, digest: semio_framework_hash::Hasher::new(), crc: protocol::codec::Crc32cCursor::new() });
        }
        let frame = self.frame.as_mut().ok_or_else(|| DbError::Internal("wal chain frame disappeared".to_string()))?;
        let fragment = pages.fragment_at(frame.position, frame.next)?;
        let count = fragment.len().min(db_io_core::DB_IO_PAGE_BYTES);
        if count == 0 {
            return Err(DbError::Corrupt("wal chain made no byte progress".to_string()));
        }
        let bytes = &fragment[..count];
        frame.digest.update(bytes);
        let crc_start = frame.position.max(frame.body);
        let crc_end = (frame.position + count).min(frame.end);
        if crc_start < crc_end {
            frame.crc.update_page(&bytes[crc_start - frame.position..crc_end - frame.position]);
        }
        frame.position += count;
        if frame.position != frame.next {
            return Ok(false);
        }
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
            if commit.commit_seq != self.next_commit
                || commit.prev_commit_offset != self.last_commit_offset
                || commit.record_count != self.record_count
                || commit.records_len != self.records_len
                || payload[28..32] != [0; 4]
                || commit.chain_hash != *self.pending.finalize().as_bytes()
            {
                return Err(DbError::Corrupt("wal committed hash chain differs".to_string()));
            }
            self.tip = Some(commit.chain_hash);
            self.pending = semio_framework_hash::Hasher::new();
            self.pending.update(&commit.chain_hash);
            self.record_count = 0;
            self.records_len = 0;
            self.last_commit_offset = frame.start as u64;
            self.next_commit = self.next_commit.checked_add(1).ok_or(DbError::LimitExceeded("wal commit sequence"))?;
        } else {
            if !is_wal_record_kind(frame.kind) {
                return Err(DbError::Corrupt("wal chain contains an unknown record".to_string()));
            }
            if !self.header_seen || frame.kind == WAL_SEGMENT_HEADER {
                self.check_segment_header(pages, &frame, document)?;
            }
            self.pending.update(frame.digest.finalize().as_bytes());
            self.record_count = self.record_count.checked_add(1).ok_or(DbError::LimitExceeded("wal commit record count"))?;
            self.records_len = self.records_len.checked_add((frame.next - frame.start) as u64).ok_or(DbError::LimitExceeded("wal commit record bytes"))?;
        }
        self.offset = frame.next;
        Ok(false)
    }
}

pub enum WalAuthenticatedStep {
    Yield,
    Committed,
    Done,
}

/// 🔐️ Owns the one immutable source shared by physical authentication and logical admission.
pub struct WalAuthenticatedSource<S> {
    pub source: S,
    pub gate: WalTransactionGate,
    pub chain: Option<WalSegmentChain>,
    pub segment: u64,
    pub previous: WalPriorChainTip,
    pub offset: usize,
    pub verified: bool,
    done: bool,
}

impl<S: WalImmutableByteSource> WalAuthenticatedSource<S> {
    pub fn new(source: S, gate: WalTransactionGate, segment: u64, previous: WalPriorChainTip) -> Self {
        Self { source, gate, chain: None, segment, previous, offset: protocol::format::HEADER_SIZE, verified: false, done: false }
    }

    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn verify_step(&mut self, document: &ArtifactId, control: &mut WalCursorControl) -> Result<bool, DbError> {
        control.check()?;
        if self.verified {
            return Ok(true);
        }
        if self.chain.is_none() {
            control.grant()?;
            self.chain = Some(WalSegmentChain::new(&self.source, self.segment, self.previous)?);
            return Ok(false);
        }
        self.verified = self.chain.as_mut().ok_or(DbError::Closed)?.step(&self.source, self.source.byte_len(), document, control)?;
        Ok(self.verified)
    }

    pub fn next_step(&mut self, control: &mut WalCursorControl) -> Result<WalAuthenticatedStep, DbError> {
        control.check()?;
        if !self.verified {
            return Err(DbError::Corrupt("wal source has not completed authentication".to_string()));
        }
        if self.done {
            return Ok(WalAuthenticatedStep::Done);
        }
        if self.gate.ready.is_some() {
            return Err(DbError::Corrupt("wal authenticated transaction remains unfinished".to_string()));
        }
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

    pub fn committed_frame(&self, index: usize) -> Option<WalRecordFrame> {
        self.gate.committed_frame(index)
    }

    pub fn finish_transaction(&mut self) -> Result<(), DbError> {
        self.gate.release()
    }

    pub fn finish(self) -> Result<(S, WalTransactionGate, [u8; 32]), Self> {
        match (self.done, self.chain.as_ref().and_then(|chain| chain.tip)) {
            (true, Some(tip)) => Ok((self.source, self.gate, tip)),
            _ => Err(self),
        }
    }

    /// 🛑️ Destroys verification and transaction continuation, returning only raw ownership for retirement.
    pub fn abort_into_source(self) -> S {
        self.source
    }
}

/// 🔁️ Decodes every `WAL_*` record across a document's ENTIRE WAL (every sealed segment in
/// full, plus the active segment), in segment then on-disk order. Each segment's complete retained
/// bytes must end in a verified commit; torn tails are rejected. Page-bounded verification checks
/// every frame and commit before releasing that segment's records, carrying the validated chain
/// tip into the next exact document/index header. Recovery/truncation belongs to the WAL opener.
/// A compacted suffix has no authenticated preceding tip; use `open_genesis` for a genesis proof.
pub struct WalReplayCursor<'storage, S: db_io_core::WalStorage> {
    pub storage: &'storage S,
    pub document: ArtifactId,
    pub segments: db_io_core::DbIoU64List,
    pub segment: usize,
    pub pages: Option<db_io_core::DbIoPages>,
    pub offset: usize,
    trusted_len: usize,
    validation: Option<WalSegmentChain>,
    decoder: Option<WalRetainedRecordDecoder>,
    previous_tip: Option<[u8; 32]>,
    genesis_required: bool,
    pub failed: bool,
    pub control: WalCursorControl,
    pub closed: bool,
}

pub enum WalReplayStep {
    Record(WalRecord),
    Yield,
    Done,
}

impl<'storage, S: db_io_core::WalStorage> WalReplayCursor<'storage, S> {
    /// 🔋️ Replenishes one bounded caller-owned replay opportunity without replacing the cursor.
    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        self.control.replenish(deadline, fuel)
    }

    pub async fn open(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<Self, DbError> {
        let segments = storage.list_segments(document).await?;
        Ok(Self {
            storage,
            document: document.clone(),
            segments,
            segment: 0,
            pages: None,
            offset: protocol::format::HEADER_SIZE,
            trusted_len: 0,
            validation: None,
            decoder: None,
            previous_tip: None,
            genesis_required: false,
            failed: false,
            control,
            closed: false,
        })
    }

    /// 📍 Starts raw replay at one exact retained segment boundary.
    pub async fn open_at_segment(storage: &'storage S, document: &ArtifactId, segment_index: u64, control: WalCursorControl) -> Result<Self, DbError> {
        let mut cursor = Self::open(storage, document, control).await?;
        let Some(segment) = cursor.segments.as_slice().iter().position(|index| *index == segment_index) else {
            while cursor.close_owner_step()? {}
            return Err(DbError::NotFound(format!("WAL segment {segment_index}")));
        };
        cursor.segment = segment;
        Ok(cursor)
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
        if self.failed {
            return Err(DbError::Corrupt("wal replay remains failed until closed".to_string()));
        }
        let result = self.next_validated_step().await;
        let interrupted = matches!(&result, Err(DbError::LimitExceeded("wal cursor fuel"))) || matches!(&result, Err(DbError::Unavailable(message)) if message == "wal cursor cancelled" || message == "wal cursor deadline reached");
        if result.is_err() && !interrupted {
            self.failed = true;
        }
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
        if let Some(decoder) = self.decoder.as_mut() {
            if decoder.close_owner_step()? {
                return Ok(true);
            }
        }
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

pub async fn replay_document<'storage, S: db_io_core::WalStorage>(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<WalReplayCursor<'storage, S>, DbError> {
    WalReplayCursor::open(storage, document, control).await
}

/// 🛂️ Releases bounded transaction bodies only after their exact logical commit is verified.
pub struct WalCommittedCursor<'storage, S: db_io_core::WalStorage> {
    pub raw: WalReplayCursor<'storage, S>,
    pub gate: WalTransactionGate,
    pub record: Option<WalRecord>,
    pub record_index: usize,
}

pub enum WalCommittedStep<'cursor, 'storage, S: db_io_core::WalStorage> {
    Transaction(WalCommittedTransaction<'cursor, 'storage, S>),
    Yield,
    Done,
}

/// ⏱️ How long one step of a whole-document WAL scan (one verified segment on open, one committed
/// transaction on replay) may make no progress.
pub const WAL_REPLAY_STEP_STALL_BOUND: std::time::Duration = std::time::Duration::from_secs(30);

/// ⛽️ Fuel one step of a whole-document WAL scan may spend.
pub const WAL_REPLAY_STEP_FUEL: usize = 1_000_000;

/// ⏲️ How long one cooperative turn of a WAL walk or staging copy runs before it yields to its executor.
pub const WAL_TURN: std::time::Duration = std::time::Duration::from_millis(1);

/// 🤝️ Batches bounded WAL steps into cooperative turns: a cursor `Yield`, a record close step, a
/// decoded record or a staged fragment continues inline until the turn's time budget is spent, and
/// only then does the caller yield once to its executor. Yielding after every frame, record and
/// fragment handed the caller's whole future chain back to its executor thousands of times per
/// reopened document and once per staged command of every commit.
pub struct WalTurn {
    ends: std::time::Instant,
}

impl WalTurn {
    pub fn start() -> Self {
        Self { ends: std::time::Instant::now() + WAL_TURN }
    }

    pub async fn step(&mut self) {
        if std::time::Instant::now() >= self.ends {
            semio_framework_async::yield_once().await;
            self.ends = std::time::Instant::now() + WAL_TURN;
        }
    }
}

/// 🤝️ Keeps the source segment and current decoded body borrowed until explicit retirement.
pub struct WalCommittedTransaction<'cursor, 'storage, S: db_io_core::WalStorage> {
    pub cursor: &'cursor mut WalCommittedCursor<'storage, S>,
    finished: bool,
}

pub enum WalCommittedRecordStep<'record> {
    Record(&'record WalRecord),
    Yield,
    Done,
}

impl<'storage, S: db_io_core::WalStorage> WalCommittedCursor<'storage, S> {
    /// ⏱️ Renews the stall bound for the next committed transaction of a whole-document replay, so
    /// the replay's total length follows its document's size and only a step that makes no progress
    /// for `WAL_REPLAY_STEP_STALL_BOUND` fails. A single wall-clock budget for the whole replay made
    /// every large enough document unopenable after a crash.
    pub fn renew_step(&mut self) -> Result<(), DbError> {
        self.replenish(std::time::Instant::now() + WAL_REPLAY_STEP_STALL_BOUND, WAL_REPLAY_STEP_FUEL)
    }

    /// 🗂️ Lists retained segments, including header-only segments; admission completes during replay.
    pub fn segment_indices(&self) -> &[u64] {
        self.raw.segments.as_slice()
    }

    pub async fn open(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<Self, DbError> {
        Ok(Self { raw: WalReplayCursor::open(storage, document, control).await?, gate: WalTransactionGate::new(), record: None, record_index: 0 })
    }

    /// 📍 Opens at one exact retained segment boundary. The selected segment is authenticated as
    /// a retained boundary before any committed transaction is released.
    pub async fn open_at_segment(storage: &'storage S, document: &ArtifactId, segment_index: u64, control: WalCursorControl) -> Result<Self, DbError> {
        let mut raw = WalReplayCursor::open(storage, document, control).await?;
        let Some(segment) = raw.segments.as_slice().iter().position(|index| *index == segment_index) else {
            while raw.close_owner_step()? {}
            return Err(DbError::NotFound(format!("WAL segment {segment_index}")));
        };
        raw.segment = segment;
        Ok(Self { raw, gate: WalTransactionGate::new(), record: None, record_index: 0 })
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        self.raw.replenish(deadline, fuel)
    }

    async fn prepare_transaction_step(&mut self) -> Result<Option<bool>, DbError> {
        self.raw.control.check()?;
        if self.raw.closed {
            return Ok(None);
        }
        if self.gate.ready.is_some() {
            return Err(DbError::Corrupt("wal committed transaction was not finished".to_string()));
        }
        if self.raw.pages.is_none() {
            return Ok(self.raw.open_segment().await?.then_some(false));
        }
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
            if self.raw.close_segment_step().await? {
                return Ok(Some(false));
            }
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
        if self.raw.failed {
            return Err(DbError::Corrupt("wal committed replay remains failed until closed".to_string()));
        }
        match self.prepare_transaction_step().await {
            Ok(Some(true)) => Ok(WalCommittedStep::Transaction(WalCommittedTransaction { cursor: self, finished: false })),
            Ok(Some(false)) => Ok(WalCommittedStep::Yield),
            Ok(None) => Ok(WalCommittedStep::Done),
            Err(error) => {
                if !wal_cursor_interrupted(&error) {
                    self.raw.failed = true;
                }
                Err(error)
            }
        }
    }

    pub fn close_owner_step(&mut self) -> Result<bool, DbError> {
        if let Some(record) = self.record.as_mut() {
            if record.close_step()? {
                return Ok(true);
            }
        }
        self.record = None;
        if self.raw.close_owner_step()? {
            return Ok(true);
        }
        self.gate.clear_frames();
        self.gate.ready = None;
        self.gate.transaction_id = None;
        self.record_index = 0;
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.raw.terminal_is_empty() && self.record.is_none() && self.gate.frames.is_empty() && self.gate.ready.is_none() && self.gate.transaction_id.is_none()
    }
}

fn wal_cursor_interrupted(error: &DbError) -> bool {
    matches!(error, DbError::LimitExceeded("wal cursor fuel")) || matches!(error, DbError::Unavailable(message) if message == "wal cursor cancelled" || message == "wal cursor deadline reached")
}

impl<'cursor, 'storage, S: db_io_core::WalStorage> WalCommittedTransaction<'cursor, 'storage, S> {
    pub fn transaction_id(&self) -> u64 {
        self.cursor.gate.ready.expect("committed transaction owns its ready gate")
    }

    pub fn segment_index(&self) -> u64 {
        self.cursor.raw.segments.as_slice()[self.cursor.raw.segment]
    }

    pub fn record_count(&self) -> usize {
        self.cursor.gate.frames.len()
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        self.cursor.replenish(deadline, fuel)
    }

    fn decode_record_step(&mut self) -> Result<bool, DbError> {
        if self.cursor.record.is_some() {
            return Err(DbError::Corrupt("wal committed body must be closed before advancing".to_string()));
        }
        let frame = self.cursor.gate.frames.get(self.cursor.record_index).copied().ok_or_else(|| DbError::Internal("wal committed replay lost body span".to_string()))?;
        if self.cursor.raw.decoder.is_none() {
            self.cursor.raw.decoder = Some(WalRetainedRecordDecoder::new(frame));
        }
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
        if self.cursor.raw.failed {
            return Err(DbError::Corrupt("wal committed replay remains failed until closed".to_string()));
        }
        if self.cursor.record_index == self.record_count() {
            return Ok(WalCommittedRecordStep::Done);
        }
        match self.decode_record_step() {
            Ok(true) => Ok(WalCommittedRecordStep::Record(self.cursor.record.as_ref().expect("decoded body remains cursor-owned"))),
            Ok(false) => Ok(WalCommittedRecordStep::Yield),
            Err(error) => {
                if !wal_cursor_interrupted(&error) {
                    self.cursor.raw.failed = true;
                }
                Err(error)
            }
        }
    }

    pub fn close_record_step(&mut self) -> Result<bool, DbError> {
        let record = self.cursor.record.as_mut().ok_or_else(|| DbError::Corrupt("wal committed body is not borrowed".to_string()))?;
        if record.close_step()? {
            return Ok(true);
        }
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

impl<S: db_io_core::WalStorage> Drop for WalCommittedTransaction<'_, '_, S> {
    fn drop(&mut self) {
        if !self.finished {
            self.cursor.raw.failed = true;
        }
    }
}

pub async fn replay_committed_document<'storage, S: db_io_core::WalStorage>(storage: &'storage S, document: &ArtifactId, control: WalCursorControl) -> Result<WalCommittedCursor<'storage, S>, DbError> {
    WalCommittedCursor::open(storage, document, control).await
}

pub async fn replay_committed_segment<'storage, S: db_io_core::WalStorage>(storage: &'storage S, document: &ArtifactId, segment_index: u64, control: WalCursorControl) -> Result<WalCommittedCursor<'storage, S>, DbError> {
    WalCommittedCursor::open_at_segment(storage, document, segment_index, control).await
}

pub async fn scan_retained_pages(pages: &db_io_core::DbIoPages, control: &mut WalCursorControl) -> Result<protocol::format::retained::VerifiedSprSpan, DbError> {
    use protocol::format::retained::{RetainedSprDiagnostic, RetainedSprLimits, RetainedSprVerification};
    let map_error = |error| match error {
        RetainedSprDiagnostic::Capacity => DbError::LimitExceeded("wal retained verification"),
        RetainedSprDiagnostic::Cancelled => DbError::Unavailable("wal retained verification cancelled".to_string()),
        _ => DbError::Corrupt("wal retained framing or commit verification failed".to_string()),
    };
    let capacity = db_io_core::DB_IO_MAX_READ_BYTES;
    let mut scan = RetainedSprVerification::new(pages.len() as u64, RetainedSprLimits { file_bytes: capacity, frame_body_bytes: capacity, records: capacity / 11 }).map_err(map_error)?;
    for fragment in pages.fragments() {
        control.grant()?;
        let mut fuel = fragment.len();
        if scan.push(fragment, &mut fuel).map_err(map_error)? != fragment.len() {
            return Err(DbError::Corrupt("wal retained scan made partial page progress".to_string()));
        }
        semio_framework_async::yield_once().await;
    }
    let header = protocol::format::read_header(&WalPageSource(pages)).await.map_err(protocol_err)?;
    if header.optional_flags != 0 {
        return Err(DbError::Corrupt("wal optional header flags differ".to_string()));
    }
    scan.finish().map_err(map_error)
}

pub struct WalValidatedPrefix {
    pub records: u64,
    pub next_tx_id: u64,
    pub incomplete_active_tx: Option<u64>,
}

pub async fn validate_wal_prefix(
    pages: &db_io_core::DbIoPages,
    span: &protocol::format::retained::VerifiedSprSpan,
    index: u64,
    prior: WalPriorChainTip,
    document: &ArtifactId,
    gate: &mut WalTransactionGate,
    writable_highest: bool,
    control: &mut WalCursorControl,
) -> Result<WalValidatedPrefix, DbError> {
    let end = usize::try_from(span.end()).map_err(|_| DbError::LimitExceeded("wal verified end"))?;
    let mut chain = WalSegmentChain::new(pages, index, prior)?;
    while !chain.step(pages, end, document, control)? {
        semio_framework_async::yield_once().await;
    }
    if chain.tip != Some(*span.chain()) {
        return Err(DbError::Corrupt("wal verifiers disagree on committed chain".to_string()));
    }
    let mut offset = protocol::format::HEADER_SIZE;
    let mut records = 0u64;
    loop {
        let frame = match wal_next_verified_page_frame(pages, &mut offset, end, control)? {
            WalVerifiedFrameStep::Frame(frame) => frame,
            WalVerifiedFrameStep::PhysicalCommit => {
                semio_framework_async::yield_once().await;
                continue;
            }
            WalVerifiedFrameStep::Done => break,
        };
        if gate.push(pages, frame)? {
            gate.release()?;
        }
        offset = frame.frame_end;
        records = records.checked_add(1).ok_or(DbError::LimitExceeded("wal recovered records"))?;
        semio_framework_async::yield_once().await;
    }
    let incomplete_active_tx = gate.finish_segment(writable_highest)?;
    if incomplete_active_tx.is_none() {
        gate.advance_segment()?;
    }
    Ok(WalValidatedPrefix { records, next_tx_id: gate.next_tx_id, incomplete_active_tx })
}

pub fn exact_prior_tip(prior: WalPriorChainTip) -> Result<Option<[u8; 32]>, DbError> {
    match prior {
        WalPriorChainTip::Genesis => Ok(None),
        WalPriorChainTip::Verified(tip) => Ok(Some(tip)),
        WalPriorChainTip::RetainedBoundary => Err(DbError::Corrupt("wal cannot initialize an unanchored compacted boundary".to_string())),
    }
}
//#endregion 🔖️Recovery

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️retained/🦀️.rs"]
mod retained_tests;
//#endregion 🧪️Tests
