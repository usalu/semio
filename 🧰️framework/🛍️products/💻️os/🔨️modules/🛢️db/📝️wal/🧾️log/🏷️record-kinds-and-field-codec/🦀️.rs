//! 🏷️ Record kinds and field codec: the record kind constants, the varint and field codecs and the cursor control. See [`super`].

use super::*;

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
pub(super) const MAX_RUN_IDS: u64 = 1_000_000;

/// ✍️ Writes a varint-length-prefixed byte field — this crate's one field encoding used by
/// every string/id-shaped record field.
#[cfg(any(test, feature = "testkit"))]
pub(super) async fn write_field(writer: &mut pack::ByteWriter, bytes: &[u8]) {
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
pub(super) async fn read_field_string(reader: &mut pack::ByteReader<'_>) -> Result<String, DbError> {
    String::from_utf8(read_field_bytes(reader).await?).map_err(|_| DbError::Corrupt("wal record field is not valid utf-8".to_string()))
}

#[cfg(any(test, feature = "testkit"))]
pub(super) async fn encode_frontier(writer: &mut pack::ByteWriter, frontier: &Frontier) {
    write_field(writer, frontier.document.0.as_bytes()).await;
    writer.write_u64_le(frontier.head_seq);
    writer.write_u64_le(frontier.commit_seq);
    writer.write_bytes(&frontier.chain_hash);
    writer.write_u64_le(frontier.epoch);
}

pub(super) fn wal_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
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

pub(super) fn wal_frontier_len(frontier: &Frontier) -> usize {
    wal_field_len(frontier.document.0.as_bytes()) + 8 + 8 + 32 + 8
}

pub async fn wal_record_write(record: &mut protocol::SprIdentityRecord<'_, SharedBuf>, bytes: &[u8]) -> Result<(), DbError> {
    record.write_fragment(bytes).await.map_err(protocol_err)
}

pub(super) async fn wal_record_write_field(record: &mut protocol::SprIdentityRecord<'_, SharedBuf>, bytes: &[u8]) -> Result<(), DbError> {
    let mut length = [0u8; 10];
    wal_record_write(record, wal_varint(bytes.len() as u64, &mut length)).await?;
    wal_record_write(record, bytes).await
}

pub(super) async fn wal_record_write_frontier(record: &mut protocol::SprIdentityRecord<'_, SharedBuf>, frontier: &Frontier) -> Result<(), DbError> {
    wal_record_write_field(record, frontier.document.0.as_bytes()).await?;
    wal_record_write(record, &frontier.head_seq.to_le_bytes()).await?;
    wal_record_write(record, &frontier.commit_seq.to_le_bytes()).await?;
    wal_record_write(record, &frontier.chain_hash).await?;
    wal_record_write(record, &frontier.epoch.to_le_bytes()).await
}

#[cfg(test)]
pub(super) async fn decode_frontier(reader: &mut pack::ByteReader<'_>) -> Result<Frontier, DbError> {
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
//#endregion 🔖️Records
