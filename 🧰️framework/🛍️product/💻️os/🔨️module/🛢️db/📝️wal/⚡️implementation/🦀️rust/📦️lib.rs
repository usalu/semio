//! 🗄️ `db_wal` — the `db` crate family's write-ahead log: a per-document, per-segment `.spr`
//! container (reusing `protocol::{SprWriter, FrameCursor, ReverseFrameCursor}` and
//! `protocol_format::recover` directly — no new framing invented) built on top of
//! `db_storage::WalStorage`'s opaque byte segments. Owns the WAL's own record kinds (SPR
//! extension range `0x40..=0x4F`), real group-commit batching (bounded by delay/bytes/count),
//! segment rotation with a cross-segment hash chain, and crash recovery (torn-tail truncation,
//! multi-segment replay). Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice (dependency): the `protocol` facade re-exports `SprWriter`/`FrameCursor`/
//! `ReverseFrameCursor`/`RecoveryMode`/`RecoveryReport`/`ProtocolError`/`ProtocolLimits`, but not
//! the free functions `protocol_format::recover`/`parse_commit_payload`, the constants
//! `COMMIT_FRAME_LEN`/`HEADER_SIZE`, or `protocol_core::REC_COMMIT`/`REQUIRED_HASH_CHAIN` — this
//! crate needs all of those (recovery; reading back a just-sealed segment's tip `chain_hash` to
//! seed the next segment's cross-segment chain; recognizing a `REC_COMMIT` frame explicitly while
//! decoding rather than assuming "not one of ours" means "must be a commit"; opening every segment
//! with hash-chaining required), so it takes direct `protocol_format`/`protocol_core` path
//! dependencies alongside `protocol` rather than duplicating their logic. Every other
//! protocol-family type this crate touches comes from the `protocol` facade, per the contract.

//#region 🔖️RecordKinds
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
/// @emoji ✉️ A `protocol::OperationEnvelope`'s bytes, stored verbatim (zero-copy on the write
/// path — this crate never re-encodes what `db_document` hands it).
pub const WAL_COMMAND: u8 = 0x44;
/// @emoji 🫙️ A command payload, either inlined or referenced by CAS hash (see `WalPayloadRef`).
pub const WAL_PAYLOAD: u8 = 0x45;
/// @emoji 🔀️ An opaque `protocol::OperationDiff`-shaped byte blob (db crates below `db_document`
/// never interpret operation semantics, per the contract's hard dependency rule).
pub const WAL_DIFF: u8 = 0x46;
/// @emoji ⏪️ An opaque inverse/undo byte blob for `db_document`'s inverse-undo pipeline.
pub const WAL_INVERSE: u8 = 0x47;
/// @emoji 📣️ An opaque effect/notification byte blob.
pub const WAL_EVENT: u8 = 0x48;
/// @emoji 📤️ An opaque outgoing-effect outbox entry byte blob.
pub const WAL_OUTBOX: u8 = 0x49;
/// @emoji 🧭️ A structured `db_core::Frontier` snapshot, written periodically so recovery can
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
/// (`decode_records`, `replay_document`) uses to decide whether a frame is one of ours (vs.
/// protocol's own `REC_COMMIT`, which every `.spr` stream also contains and this crate skips).
pub fn is_wal_record_kind(kind: u8) -> bool {
    (WAL_SEGMENT_HEADER..=WAL_MIGRATION).contains(&kind)
}
//#endregion 🔖️RecordKinds

//#region 🔖️Records
/// @emoji 🛡️ Ceiling on any single length-prefixed scalar field (document id, resource name,
/// holder id, vcs ref) this crate decodes — validated BEFORE allocating, mirroring
/// `pack_core::PackLimits`'s stated invariant. Generous for any legitimate identifier, small
/// enough to reject an obviously-corrupt length up front.
const MAX_FIELD_BYTES: u64 = 1024 * 1024;
/// @emoji 🛡️ Ceiling on a `WAL_INDEX_CKPT` record's run-id count, validated before the `Vec` is
/// sized.
const MAX_RUN_IDS: u64 = 1_000_000;

/// @emoji ✍️ Writes a varint-length-prefixed byte field — this crate's one field encoding used by
/// every string/id-shaped record field.
fn write_field(writer: &mut pack_core::ByteWriter, bytes: &[u8]) {
    writer.write_varint_u64(bytes.len() as u64);
    writer.write_bytes(bytes);
}

/// @emoji 📖️ Inverse of `write_field`.
fn read_field_bytes(reader: &mut pack_core::ByteReader<'_>) -> Result<Vec<u8>, db_core::DbError> {
    let len = reader.read_varint_u64()?;
    db_core::check_len(len, MAX_FIELD_BYTES, "wal_record::field")?;
    Ok(reader.read_bytes(len as usize)?.to_vec())
}

/// @emoji 📖️ `read_field_bytes` plus a utf-8 validation, for text fields.
fn read_field_string(reader: &mut pack_core::ByteReader<'_>) -> Result<String, db_core::DbError> {
    String::from_utf8(read_field_bytes(reader)?).map_err(|_| db_core::DbError::Corrupt("wal record field is not valid utf-8".to_string()))
}

fn encode_frontier(writer: &mut pack_core::ByteWriter, frontier: &db_core::Frontier) {
    write_field(writer, frontier.document.0.as_bytes());
    writer.write_u64_le(frontier.head_seq);
    writer.write_u64_le(frontier.commit_seq);
    writer.write_bytes(&frontier.chain_hash);
    writer.write_u64_le(frontier.epoch);
}

fn decode_frontier(reader: &mut pack_core::ByteReader<'_>) -> Result<db_core::Frontier, db_core::DbError> {
    let document = db_core::DocumentId(read_field_string(reader)?);
    let head_seq = reader.read_u64_le()?;
    let commit_seq = reader.read_u64_le()?;
    let chain_hash = reader.read_array32()?;
    let epoch = reader.read_u64_le()?;
    Ok(db_core::Frontier { document, head_seq, commit_seq, chain_hash, epoch })
}

/// @emoji 🫙️ `WAL_PAYLOAD`'s two shapes: small payloads inline, large ones by CAS reference into
/// `db_storage::PayloadStorage` — mirrors that trait's own blake3-CAS design.
#[derive(Clone, Debug, PartialEq)]
pub enum WalPayloadRef {
    Inline(Vec<u8>),
    CasRef(pack_core::ContentHash),
}

/// @emoji 📜️ One decoded WAL record — the typed shape every `WAL_*` kind decodes to/encodes from.
/// `Command`/`Diff`/`Inverse`/`Event`/`Outbox`/`Migration` carry opaque bytes verbatim (per the
/// contract, no db crate below `db_document` interprets operation semantics); the rest are
/// structured since this crate itself owns their meaning (transaction boundaries, segment
/// chaining, frontiers, leases).
#[derive(Clone, Debug, PartialEq)]
pub enum WalRecord {
    SegmentHeader { document: db_core::DocumentId, segment_index: u64, prev_chain_hash: Option<[u8; 32]> },
    TxBegin { tx_id: u64 },
    TxCommit { tx_id: u64, record_count: u32 },
    TxAbort { tx_id: u64 },
    Command(Vec<u8>),
    Payload(WalPayloadRef),
    Diff(Vec<u8>),
    Inverse(Vec<u8>),
    Event(Vec<u8>),
    Outbox(Vec<u8>),
    Frontier(db_core::Frontier),
    VcsRef(String),
    SnapshotPub { generation: u64, frontier: db_core::Frontier },
    IndexCkpt { run_ids: Vec<u64> },
    Lease { resource: String, holder: String, fence: u64, expires_at_ms: u64 },
    Migration(Vec<u8>),
}

impl WalRecord {
    /// @emoji 🔢️ The transaction id carried by a `TxBegin`/`TxCommit`/`TxAbort` record, or `None`
    /// for every other kind — used by `DocumentWal::open` to resume `next_tx_id` past whatever was
    /// already durable.
    fn tx_id(&self) -> Option<u64> {
        match self {
            WalRecord::TxBegin { tx_id } | WalRecord::TxCommit { tx_id, .. } | WalRecord::TxAbort { tx_id } => Some(*tx_id),
            _ => None,
        }
    }

    /// @emoji ✍️ Encodes `self` to its on-disk `(kind, critical, payload)` triple, ready for
    /// `protocol::SprWriter::write_record`. Every kind is critical: unlike protocol's own
    /// history-log records (where e.g. a dictionary delta can plausibly be "skippable" to some
    /// future reader), every `WAL_*` record is load-bearing for correct replay — there is no
    /// optional WAL record in this crate's design.
    pub fn encode(&self) -> (u8, bool, Vec<u8>) {
        let mut writer = pack_core::ByteWriter::new();
        let kind = match self {
            WalRecord::SegmentHeader { document, segment_index, prev_chain_hash } => {
                write_field(&mut writer, document.0.as_bytes());
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
                writer.write_bytes(bytes);
                WAL_COMMAND
            }
            WalRecord::Payload(WalPayloadRef::Inline(bytes)) => {
                writer.write_u8(0);
                write_field(&mut writer, bytes);
                WAL_PAYLOAD
            }
            WalRecord::Payload(WalPayloadRef::CasRef(hash)) => {
                writer.write_u8(1);
                writer.write_bytes(&hash.0);
                WAL_PAYLOAD
            }
            WalRecord::Diff(bytes) => {
                writer.write_bytes(bytes);
                WAL_DIFF
            }
            WalRecord::Inverse(bytes) => {
                writer.write_bytes(bytes);
                WAL_INVERSE
            }
            WalRecord::Event(bytes) => {
                writer.write_bytes(bytes);
                WAL_EVENT
            }
            WalRecord::Outbox(bytes) => {
                writer.write_bytes(bytes);
                WAL_OUTBOX
            }
            WalRecord::Frontier(frontier) => {
                encode_frontier(&mut writer, frontier);
                WAL_FRONTIER
            }
            WalRecord::VcsRef(id) => {
                write_field(&mut writer, id.as_bytes());
                WAL_VCS_REF
            }
            WalRecord::SnapshotPub { generation, frontier } => {
                writer.write_u64_le(*generation);
                encode_frontier(&mut writer, frontier);
                WAL_SNAPSHOT_PUB
            }
            WalRecord::IndexCkpt { run_ids } => {
                writer.write_varint_u64(run_ids.len() as u64);
                for run_id in run_ids {
                    writer.write_u64_le(*run_id);
                }
                WAL_INDEX_CKPT
            }
            WalRecord::Lease { resource, holder, fence, expires_at_ms } => {
                write_field(&mut writer, resource.as_bytes());
                write_field(&mut writer, holder.as_bytes());
                writer.write_u64_le(*fence);
                writer.write_u64_le(*expires_at_ms);
                WAL_LEASE
            }
            WalRecord::Migration(bytes) => {
                writer.write_bytes(bytes);
                WAL_MIGRATION
            }
        };
        (kind, true, writer.into_bytes())
    }

    /// @emoji 📖️ Inverse of `encode`. Errors `DbError::Corrupt` on an unrecognized `kind` (a
    /// genuinely corrupt or future-version record) rather than silently dropping it — every
    /// `WAL_*` kind is critical (see `encode`'s doc).
    pub fn decode(kind: u8, payload: &[u8]) -> Result<WalRecord, db_core::DbError> {
        let mut reader = pack_core::ByteReader::new(payload);
        let record = match kind {
            WAL_SEGMENT_HEADER => {
                let document = db_core::DocumentId(read_field_string(&mut reader)?);
                let segment_index = reader.read_u64_le()?;
                let prev_chain_hash = if reader.read_u8()? == 1 { Some(reader.read_array32()?) } else { None };
                WalRecord::SegmentHeader { document, segment_index, prev_chain_hash }
            }
            WAL_TX_BEGIN => WalRecord::TxBegin { tx_id: reader.read_u64_le()? },
            WAL_TX_COMMIT => WalRecord::TxCommit { tx_id: reader.read_u64_le()?, record_count: reader.read_u32_le()? },
            WAL_TX_ABORT => WalRecord::TxAbort { tx_id: reader.read_u64_le()? },
            WAL_COMMAND => WalRecord::Command(payload.to_vec()),
            WAL_PAYLOAD => match reader.read_u8()? {
                0 => WalRecord::Payload(WalPayloadRef::Inline(read_field_bytes(&mut reader)?)),
                1 => WalRecord::Payload(WalPayloadRef::CasRef(pack_core::ContentHash(reader.read_array32()?))),
                other => return Err(db_core::DbError::Corrupt(format!("unknown wal payload tag {other}"))),
            },
            WAL_DIFF => WalRecord::Diff(payload.to_vec()),
            WAL_INVERSE => WalRecord::Inverse(payload.to_vec()),
            WAL_EVENT => WalRecord::Event(payload.to_vec()),
            WAL_OUTBOX => WalRecord::Outbox(payload.to_vec()),
            WAL_FRONTIER => WalRecord::Frontier(decode_frontier(&mut reader)?),
            WAL_VCS_REF => WalRecord::VcsRef(read_field_string(&mut reader)?),
            WAL_SNAPSHOT_PUB => {
                let generation = reader.read_u64_le()?;
                WalRecord::SnapshotPub { generation, frontier: decode_frontier(&mut reader)? }
            }
            WAL_INDEX_CKPT => {
                let count = reader.read_varint_u64()?;
                db_core::check_len(count, MAX_RUN_IDS, "wal_record::index_ckpt run_ids")?;
                let mut run_ids = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    run_ids.push(reader.read_u64_le()?);
                }
                WalRecord::IndexCkpt { run_ids }
            }
            WAL_LEASE => {
                let resource = read_field_string(&mut reader)?;
                let holder = read_field_string(&mut reader)?;
                let fence = reader.read_u64_le()?;
                let expires_at_ms = reader.read_u64_le()?;
                WalRecord::Lease { resource, holder, fence, expires_at_ms }
            }
            WAL_MIGRATION => WalRecord::Migration(payload.to_vec()),
            other => return Err(db_core::DbError::Corrupt(format!("unknown wal record kind {other:#x}"))),
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
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, db_core::DbError>;
    /// @emoji 🔓️ Inverts `encrypt` — must exactly reconstruct the original bytes.
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, db_core::DbError>;
}

/// @emoji 🪟️ A `PayloadTransform` that passes bytes through unchanged — the default for a
/// deployment with no encryption configured.
#[derive(Clone, Copy, Default, Debug)]
pub struct IdentityPayloadTransform;

impl PayloadTransform for IdentityPayloadTransform {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, db_core::DbError> {
        Ok(plaintext.to_vec())
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, db_core::DbError> {
        Ok(ciphertext.to_vec())
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
    /// @emoji ⏰️ True iff any bound is currently exceeded — `DocumentWal::submit` also always
    /// commits immediately regardless of this policy when the caller requests `Fsync`/`Quorum`
    /// durability (a durability request stronger than what's already durable can never be
    /// satisfied by deferring the commit).
    fn is_due(&self, pending_bytes: u64, pending_records: u32, oldest_pending_at_ms: Option<u64>, now_ms: u64) -> bool {
        pending_bytes >= self.max_bytes
            || pending_records >= self.max_records
            || oldest_pending_at_ms.is_some_and(|started| now_ms.saturating_sub(started) >= self.max_delay_ms)
    }
}
//#endregion 🔖️GroupCommit

//#region 🔖️Sink
/// @emoji 🪞️ A `pack_core::PackSink` over a shared, growing in-memory buffer. `protocol::SprWriter`
/// owns one clone of the underlying `Arc<Mutex<Vec<u8>>>`; `SegmentWriter` retains a second clone
/// so it can snapshot the writer's accumulated bytes to flush the unflushed suffix to
/// `db_storage::WalStorage` — `SprWriter` has no public accessor for its private `sink` field, and
/// (per this crate's module doc) no resume-mid-stream constructor either, so holding the buffer
/// open for a segment's whole lifetime via a second handle is the only way to both keep writing
/// AND read back what's been written so far without prematurely consuming the writer.
#[derive(Clone)]
struct SharedBuf(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

/// @emoji 🩹️ Recovers a poisoned lock instead of panicking — one panicking document actor must
/// never turn every other document's WAL access into a cascading panic (mirrors `db_storage`'s
/// own `MemoryStorage` convention).
fn lock<T>(mutex: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl SharedBuf {
    fn new() -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(Vec::new())))
    }

    fn len(&self) -> u64 {
        lock(&self.0).len() as u64
    }

    fn snapshot(&self) -> Vec<u8> {
        lock(&self.0).clone()
    }
}

impl pack_core::PackSink for SharedBuf {
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), pack_core::PackError> {
        lock(&self.0).extend_from_slice(bytes);
        Ok(())
    }

    fn position(&self) -> u64 {
        self.len()
    }
}

/// @emoji ⛓️ Every `db_wal` segment is opened requiring `protocol_core::REQUIRED_HASH_CHAIN` —
/// `SprWriter` always computes the commit chain regardless of this flag, but stamping it into the
/// header makes the requirement an explicit, reader-enforced part of the file's contract rather
/// than an implicit convention only this crate happens to uphold.
fn segment_write_options() -> protocol::WriteOptions {
    protocol::WriteOptions { required_flags: protocol_core::REQUIRED_HASH_CHAIN, optional_flags: 0 }
}
//#endregion 🔖️Sink

//#region 🔖️Recovery
/// @emoji 🚨️ Maps `protocol::ProtocolError` (this crate never leaks it in a public signature, per
/// the family's `DbError`-only rule) onto the closest `DbError` variant.
fn protocol_err(err: protocol::ProtocolError) -> db_core::DbError {
    match err {
        protocol::ProtocolError::Pack(pack_error) => db_core::DbError::from(pack_error),
        protocol::ProtocolError::Io(message) => db_core::DbError::Io(message),
        protocol::ProtocolError::LimitExceeded(what) => db_core::DbError::LimitExceeded(what),
        other => db_core::DbError::Corrupt(other.to_string()),
    }
}

/// @emoji 👓️ Decodes every `WAL_*`-kind frame in `trusted` (a segment's already-recovered,
/// trustworthy byte prefix) into a `WalRecord`, in on-disk order — protocol's own `REC_COMMIT`
/// frames (and any other non-`WAL_*` kind, though none should occur in a `db_wal` segment) are
/// skipped via `is_wal_record_kind`, matching `protocol_format`'s "cursors are kind-agnostic,
/// interpretation is a caller policy" design note.
fn decode_records(trusted: &[u8]) -> Result<Vec<WalRecord>, db_core::DbError> {
    let mut cursor = protocol::FrameCursor::new(trusted, protocol_format::HEADER_SIZE as u64);
    let mut records = Vec::new();
    while let Some(frame) = cursor.next_frame().map_err(protocol_err)? {
        if is_wal_record_kind(frame.kind) {
            records.push(WalRecord::decode(frame.kind, frame.payload())?);
        } else if frame.kind != protocol_core::REC_COMMIT {
            return Err(db_core::DbError::Corrupt(format!("unexpected non-wal, non-commit frame kind {:#x} in a db_wal segment", frame.kind)));
        }
    }
    Ok(records)
}

/// @emoji 📋️ What `DocumentWal::open` found while recovering a document's WAL.
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
/// primitive `db_document`'s materialize-from-WAL-suffix step builds on. Every sealed segment is
/// expected fully trusted (a torn sealed segment is `DbError::Corrupt`, since `truncate_tail` only
/// targets an unsealed segment); the last (possibly active, possibly unsealed) segment is
/// recovered via `protocol_format::recover` first.
pub fn replay_document(storage: &dyn db_storage::WalStorage, document: &db_core::DocumentId) -> Result<Vec<WalRecord>, db_core::DbError> {
    let mut indices = storage.list_segments(document)?;
    indices.sort_unstable();
    let mut all = Vec::new();
    for index in indices {
        let len = storage.segment_len(document, index)?;
        let bytes = storage.read(document, index, pack_core::ByteRange { offset: 0, len })?;
        let report = protocol_format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).map_err(protocol_err)?;
        if report.bytes_recovered != bytes.len() as u64 {
            return Err(db_core::DbError::Corrupt(format!(
                "wal segment {index} for {document} has a torn tail ({} of {} bytes trusted)",
                report.bytes_recovered,
                bytes.len()
            )));
        }
        all.extend(decode_records(&bytes[..report.bytes_recovered as usize])?);
    }
    Ok(all)
}
//#endregion 🔖️Recovery

//#region 🔖️Segment
/// @emoji 📦️ The live write path for exactly one WAL segment: an in-memory `protocol::SprWriter`
/// over a `SharedBuf`, plus how much of that buffer has actually been flushed (durably appended)
/// to `db_storage::WalStorage` so far. Held open for the segment's entire lifetime (see
/// `SharedBuf`'s doc for why) — sealed and replaced by a fresh one on rotation.
struct SegmentWriter {
    document: db_core::DocumentId,
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
    fn begin(
        storage: &dyn db_storage::WalStorage,
        document: db_core::DocumentId,
        index: u64,
        prev_chain_hash: Option<[u8; 32]>,
        now_ms: u64,
    ) -> Result<Self, db_core::DbError> {
        storage.create_segment(&document, index)?;
        let buf = SharedBuf::new();
        let writer = protocol::SprWriter::begin(buf.clone(), &segment_write_options()).map_err(protocol_err)?;
        let mut segment = Self { document: document.clone(), index, buf, writer, flushed_len: 0, pending_records: 0, oldest_pending_at_ms: None };
        segment.append_record(&WalRecord::SegmentHeader { document, segment_index: index, prev_chain_hash }, now_ms)?;
        segment.commit_and_flush(storage, db_core::DurabilityClass::Fsync)?;
        Ok(segment)
    }

    fn append_record(&mut self, record: &WalRecord, now_ms: u64) -> Result<u64, db_core::DbError> {
        let (kind, critical, payload) = record.encode();
        let offset = self.writer.write_record(kind, critical, &payload, pack_core::CodecId(0)).map_err(protocol_err)?;
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
    /// `DocumentWal::submit` compares against its segment-rotation threshold.
    fn total_len(&self) -> u64 {
        self.buf.len()
    }

    /// @emoji ⛓️ Physically commits (`SprWriter::commit`, hash-chaining everything pending) and
    /// flushes the newly-committed suffix to `WalStorage::append` + `sync(class)` — the group-
    /// commit primitive `DocumentWal::submit`/`force_flush`/`rotate` all funnel through. A no-op
    /// (`Ok(None)`) if nothing is pending.
    fn commit_and_flush(&mut self, storage: &dyn db_storage::WalStorage, class: db_core::DurabilityClass) -> Result<Option<u64>, db_core::DbError> {
        if self.pending_records == 0 {
            return Ok(None);
        }
        let commit_offset = self.writer.commit().map_err(protocol_err)?;
        let snapshot = self.buf.snapshot();
        let delta = &snapshot[self.flushed_len as usize..];
        let new_len = storage.append(&self.document, self.index, delta)?;
        storage.sync(&self.document, self.index, class)?;
        self.flushed_len = new_len;
        self.pending_records = 0;
        self.oldest_pending_at_ms = None;
        Ok(Some(commit_offset))
    }

    /// @emoji ⛓️ The chain_hash of this segment's last commit (falling back to `blake3(header)` if
    /// nothing has committed beyond the segment's own header write, which `begin` always performs,
    /// so this should never actually hit that branch in practice — handled honestly rather than
    /// assumed away). Used by `DocumentWal::rotate` to seed the next segment's
    /// `WAL_SEGMENT_HEADER.prev_chain_hash`.
    fn tip_chain_hash(&self) -> Result<[u8; 32], db_core::DbError> {
        let snapshot = self.buf.snapshot();
        let report = protocol_format::recover(&snapshot, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).map_err(protocol_err)?;
        if report.last_commit_seq == 0 {
            return Ok(*blake3::hash(&snapshot[..protocol_format::HEADER_SIZE]).as_bytes());
        }
        let frame_end = (report.last_commit_offset + protocol_format::COMMIT_FRAME_LEN) as usize;
        let frame_bytes = &snapshot[report.last_commit_offset as usize..frame_end];
        let mut cursor = protocol::FrameCursor::new(frame_bytes, 0);
        let frame =
            cursor.next_frame().map_err(protocol_err)?.ok_or_else(|| db_core::DbError::Corrupt("expected a commit frame while sealing wal segment".to_string()))?;
        if frame.kind != protocol_core::REC_COMMIT {
            return Err(db_core::DbError::Corrupt(format!("expected REC_COMMIT at the recovered commit offset, found kind {:#x}", frame.kind)));
        }
        Ok(protocol_format::parse_commit_payload(frame.payload()).map_err(protocol_err)?.chain_hash)
    }
}
//#endregion 🔖️Segment

//#region 🔖️DocumentWal
/// @emoji 📏️ Default segment-rotation threshold (this crate's own choice — the contract fixes
/// "per-document segment files", not an exact size): large enough that rotation stays rare under
/// ordinary load, small enough that a single segment's crash-recovery replay stays bounded.
const DEFAULT_MAX_SEGMENT_BYTES: u64 = 16 * 1024 * 1024;

/// @emoji 🧾️ What `DocumentWal::submit` did with one transaction's records.
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
/// `db_document`'s authority actor owns one of per open document.
pub struct DocumentWal {
    document: db_core::DocumentId,
    policy: GroupCommitPolicy,
    max_segment_bytes: u64,
    next_segment_index: u64,
    active: SegmentWriter,
    next_tx_id: u64,
}

impl DocumentWal {
    /// @emoji 🌱️ Creates a brand new WAL for `document` (segment 0, genesis — no prior segment to
    /// chain from). Errors `AlreadyExists` if `document` already has WAL segments in `storage`.
    pub fn create(storage: &dyn db_storage::WalStorage, document: db_core::DocumentId, policy: GroupCommitPolicy, now_ms: u64) -> Result<Self, db_core::DbError> {
        let active = SegmentWriter::begin(storage, document.clone(), 0, None, now_ms)?;
        Ok(Self { document, policy, max_segment_bytes: DEFAULT_MAX_SEGMENT_BYTES, next_segment_index: 1, active, next_tx_id: 1 })
    }

    /// @emoji 🚑️ Opens `document`'s existing WAL, recovering it: every sealed segment (all but the
    /// highest index) must be fully trusted end-to-end (`DbError::Corrupt` otherwise — a torn
    /// sealed segment is unrecoverable damage, not a normal crash artifact); the highest-indexed
    /// segment is treated as possibly-active and recovered via `protocol_format::recover`,
    /// discarding any torn tail. Creates a fresh WAL (equivalent to `create`) if `document` has no
    /// segments yet.
    pub fn open(
        storage: &dyn db_storage::WalStorage,
        document: db_core::DocumentId,
        policy: GroupCommitPolicy,
        now_ms: u64,
    ) -> Result<(Self, WalRecoveryReport), db_core::DbError> {
        let mut indices = storage.list_segments(&document)?;
        indices.sort_unstable();
        if indices.is_empty() {
            return Ok((Self::create(storage, document, policy, now_ms)?, WalRecoveryReport::default()));
        }
        let last_index = *indices.last().expect("checked non-empty above");

        for &index in &indices[..indices.len() - 1] {
            let len = storage.segment_len(&document, index)?;
            let bytes = storage.read(&document, index, pack_core::ByteRange { offset: 0, len })?;
            let report = protocol_format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).map_err(protocol_err)?;
            if report.bytes_recovered != bytes.len() as u64 {
                return Err(db_core::DbError::Corrupt(format!(
                    "wal segment {index} for {document} has a torn tail ({} of {} bytes trusted) but is not the active segment",
                    report.bytes_recovered,
                    bytes.len()
                )));
            }
        }

        let len = storage.segment_len(&document, last_index)?;
        let bytes = storage.read(&document, last_index, pack_core::ByteRange { offset: 0, len })?;
        let report = protocol_format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).map_err(protocol_err)?;
        let torn_tail_bytes = bytes.len() as u64 - report.bytes_recovered;
        let records = decode_records(&bytes[..report.bytes_recovered as usize])?;

        // 🎯️ Design choice (forced by `protocol::SprWriter`'s API — the same constraint
        // `protocol_io::HistoryFile::open_append` documents and works around identically): there
        // is no public constructor that resumes a writer mid-stream with its running chain hash
        // intact. The only correctness-preserving resume is O(segment size): discard the physical
        // segment and replay every trusted `WAL_*` record through a freshly-begun writer, which
        // reproduces the exact same logical WAL content (fewer, coalesced commit frames is fine —
        // recovery only has to restore the committed record set, not the original commit
        // boundaries).
        storage.delete_segment(&document, last_index)?;
        storage.create_segment(&document, last_index)?;
        let buf = SharedBuf::new();
        let mut writer = protocol::SprWriter::begin(buf.clone(), &segment_write_options()).map_err(protocol_err)?;
        for record in &records {
            let (kind, critical, payload) = record.encode();
            writer.write_record(kind, critical, &payload, pack_core::CodecId(0)).map_err(protocol_err)?;
        }
        let mut active = SegmentWriter {
            document: document.clone(),
            index: last_index,
            buf,
            writer,
            flushed_len: 0,
            pending_records: records.len() as u32,
            oldest_pending_at_ms: if records.is_empty() { None } else { Some(now_ms) },
        };
        active.commit_and_flush(storage, db_core::DurabilityClass::Fsync)?;

        let next_tx_id = records.iter().filter_map(WalRecord::tx_id).max().map_or(1, |id| id + 1);
        let recovery = WalRecoveryReport { segments_seen: indices.len() as u64, records_replayed: records.len() as u64, torn_tail_bytes };
        Ok((Self { document, policy, max_segment_bytes: DEFAULT_MAX_SEGMENT_BYTES, next_segment_index: last_index + 1, active, next_tx_id }, recovery))
    }

    pub fn document(&self) -> &db_core::DocumentId {
        &self.document
    }

    pub fn active_segment_index(&self) -> u64 {
        self.active.index
    }

    /// @emoji ✍️ Appends `records` as one transaction (`WAL_TX_BEGIN` .. `WAL_TX_COMMIT`), then
    /// group-commits per `GroupCommitPolicy` — except `durability >= Fsync` always forces an
    /// immediate commit, since deferring one can never satisfy a durability request stronger than
    /// what's already flushed. Rotates to a new segment (sealing this one first, which forces a
    /// commit if anything is still pending) once the active segment crosses `max_segment_bytes`.
    pub fn submit(
        &mut self,
        storage: &dyn db_storage::WalStorage,
        records: &[WalRecord],
        durability: db_core::DurabilityClass,
        now_ms: u64,
    ) -> Result<WalAppendReceipt, db_core::DbError> {
        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;
        let segment_index = self.active.index;

        self.active.append_record(&WalRecord::TxBegin { tx_id }, now_ms)?;
        for record in records {
            self.active.append_record(record, now_ms)?;
        }
        self.active.append_record(&WalRecord::TxCommit { tx_id, record_count: records.len() as u32 }, now_ms)?;

        let forced = matches!(durability, db_core::DurabilityClass::Fsync | db_core::DurabilityClass::Quorum(_));
        let due = forced || self.policy.is_due(self.active.pending_bytes(), self.active.pending_records, self.active.oldest_pending_at_ms, now_ms);
        let mut committed = false;
        if due {
            committed = self.active.commit_and_flush(storage, durability)?.is_some();
        }
        if self.active.total_len() >= self.max_segment_bytes {
            self.rotate(storage, now_ms)?;
            committed = true;
        }
        Ok(WalAppendReceipt { segment_index, tx_id, committed })
    }

    /// @emoji 🚿️ Forces a commit+flush of whatever is currently pending, regardless of policy —
    /// the primitive a timer-driven group-commit loop or a clean-shutdown drain calls. Returns
    /// `true` iff there was anything to flush.
    pub fn force_flush(&mut self, storage: &dyn db_storage::WalStorage) -> Result<bool, db_core::DbError> {
        Ok(self.active.commit_and_flush(storage, db_core::DurabilityClass::Fsync)?.is_some())
    }

    /// @emoji 🔄️ Seals the active segment (after a final commit+flush) and begins a fresh one,
    /// carrying the sealed segment's tip `chain_hash` forward as the new segment's
    /// `WAL_SEGMENT_HEADER.prev_chain_hash` — the cross-segment hash-chain link.
    fn rotate(&mut self, storage: &dyn db_storage::WalStorage, now_ms: u64) -> Result<(), db_core::DbError> {
        self.active.commit_and_flush(storage, db_core::DurabilityClass::Fsync)?;
        let chain_hash = self.active.tip_chain_hash()?;
        let sealed_index = self.active.index;
        storage.seal(&self.document, sealed_index)?;
        let new_index = self.next_segment_index;
        self.next_segment_index += 1;
        self.active = SegmentWriter::begin(storage, self.document.clone(), new_index, Some(chain_hash), now_ms)?;
        Ok(())
    }
}
//#endregion 🔖️DocumentWal

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_core::{DocumentId, DurabilityClass, Frontier};
    use db_storage::{MemoryStorage, WalStorage};

    fn doc(id: &str) -> DocumentId {
        DocumentId::from(id)
    }

    fn sample_frontier(document: &DocumentId) -> Frontier {
        Frontier { document: document.clone(), head_seq: 7, commit_seq: 3, chain_hash: [9u8; 32], epoch: 1 }
    }

    //#region 🔖️RecordKinds
    #[test]
    fn record_kinds_fill_the_extension_range_uniquely() {
        let kinds = [
            WAL_SEGMENT_HEADER,
            WAL_TX_BEGIN,
            WAL_TX_COMMIT,
            WAL_TX_ABORT,
            WAL_COMMAND,
            WAL_PAYLOAD,
            WAL_DIFF,
            WAL_INVERSE,
            WAL_EVENT,
            WAL_OUTBOX,
            WAL_FRONTIER,
            WAL_VCS_REF,
            WAL_SNAPSHOT_PUB,
            WAL_INDEX_CKPT,
            WAL_LEASE,
            WAL_MIGRATION,
        ];
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
        // 🧪️ `0x0C` (protocol_core::REC_COMMIT) hard-coded rather than depending on protocol_core
        // directly just for this one assertion — this crate's extension range never overlaps it.
        assert!(!is_wal_record_kind(0x0C));
    }
    //#endregion 🔖️RecordKinds

    //#region 🔖️Records
    #[test]
    fn wal_record_round_trips_every_kind_through_encode_decode() {
        let document = doc("doc-1");
        let samples = vec![
            WalRecord::SegmentHeader { document: document.clone(), segment_index: 0, prev_chain_hash: None },
            WalRecord::SegmentHeader { document: document.clone(), segment_index: 1, prev_chain_hash: Some([3u8; 32]) },
            WalRecord::TxBegin { tx_id: 42 },
            WalRecord::TxCommit { tx_id: 42, record_count: 5 },
            WalRecord::TxAbort { tx_id: 7 },
            WalRecord::Command(b"envelope-bytes".to_vec()),
            WalRecord::Payload(WalPayloadRef::Inline(b"small-payload".to_vec())),
            WalRecord::Payload(WalPayloadRef::CasRef(pack_core::ContentHash([5u8; 32]))),
            WalRecord::Diff(b"diff-bytes".to_vec()),
            WalRecord::Inverse(b"inverse-bytes".to_vec()),
            WalRecord::Event(b"event-bytes".to_vec()),
            WalRecord::Outbox(b"outbox-bytes".to_vec()),
            WalRecord::Frontier(sample_frontier(&document)),
            WalRecord::VcsRef("ck-abc123".to_string()),
            WalRecord::SnapshotPub { generation: 4, frontier: sample_frontier(&document) },
            WalRecord::IndexCkpt { run_ids: vec![1, 2, 3, 100] },
            WalRecord::Lease { resource: "shard-0".to_string(), holder: "node-a".to_string(), fence: 9, expires_at_ms: 12345 },
            WalRecord::Migration(b"migration-bytes".to_vec()),
        ];
        for sample in samples {
            let (kind, critical, payload) = sample.encode();
            assert!(critical, "every wal record is critical by design");
            let decoded = WalRecord::decode(kind, &payload).unwrap();
            assert_eq!(decoded, sample);
        }
    }

    #[test]
    fn decode_rejects_unknown_kind_and_malformed_payload() {
        assert!(matches!(WalRecord::decode(0x7E, b""), Err(db_core::DbError::Corrupt(_))));
        assert!(WalRecord::decode(WAL_TX_BEGIN, b"").is_err());
    }
    //#endregion 🔖️Records

    //#region 🔖️PayloadTransform
    #[test]
    fn identity_payload_transform_round_trips_without_changing_bytes() {
        let transform = IdentityPayloadTransform;
        let plaintext = b"hello wal";
        let encrypted = transform.encrypt(plaintext).unwrap();
        assert_eq!(encrypted, plaintext);
        assert_eq!(transform.decrypt(&encrypted).unwrap(), plaintext);
    }

    /// @emoji 🔐️ A reversing "cipher" — enough to prove a caller can thread a non-identity
    /// `PayloadTransform` through `WalPayloadRef::Inline` end-to-end via this crate's own
    /// encode/decode, without `db_wal` itself needing to know encryption happened.
    struct ReversingTransform;
    impl PayloadTransform for ReversingTransform {
        fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, db_core::DbError> {
            Ok(plaintext.iter().rev().copied().collect())
        }
        fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, db_core::DbError> {
            Ok(ciphertext.iter().rev().copied().collect())
        }
    }

    #[test]
    fn non_identity_payload_transform_round_trips_through_an_inline_wal_payload_record() {
        let transform = ReversingTransform;
        let plaintext = b"round trip me through the wal".to_vec();

        let ciphertext = transform.encrypt(&plaintext).unwrap();
        assert_ne!(ciphertext, plaintext, "the transform must actually have changed the bytes");

        let record = WalRecord::Payload(WalPayloadRef::Inline(ciphertext));
        let (kind, _critical, payload) = record.encode();
        let decoded = WalRecord::decode(kind, &payload).unwrap();
        let WalRecord::Payload(WalPayloadRef::Inline(stored_ciphertext)) = decoded else {
            panic!("expected an inline payload record");
        };

        let recovered = transform.decrypt(&stored_ciphertext).unwrap();
        assert_eq!(recovered, plaintext);
    }
    //#endregion 🔖PayloadTransform

    //#region 🔖GroupCommit
    #[test]
    fn group_commit_policy_is_due_on_any_threshold() {
        let policy = GroupCommitPolicy { max_delay_ms: 100, max_bytes: 1000, max_records: 10 };
        assert!(!policy.is_due(0, 0, None, 0));
        assert!(policy.is_due(1000, 0, None, 0));
        assert!(policy.is_due(0, 10, None, 0));
        assert!(policy.is_due(0, 0, Some(0), 100));
        assert!(!policy.is_due(0, 0, Some(50), 100));
    }
    //#endregion 🔖GroupCommit

    //#region 🔖Segment + DocumentWal
    #[test]
    fn single_segment_write_commit_flush_recovers_cleanly() {
        let storage = MemoryStorage::new();
        let document = doc("doc-1");
        let mut wal = DocumentWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).unwrap();

        let receipt = wal.submit(&storage, &[WalRecord::Command(b"cmd-1".to_vec())], DurabilityClass::Fsync, 1).unwrap();
        assert!(receipt.committed, "Fsync durability must force an immediate commit");
        assert_eq!(receipt.tx_id, 1);

        let len = storage.segment_len(&document, 0).unwrap();
        let bytes = storage.read(&document, 0, pack_core::ByteRange { offset: 0, len }).unwrap();
        let report = protocol_format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).unwrap();
        assert_eq!(report.bytes_recovered, bytes.len() as u64);
        assert_eq!(report.torn_tail_bytes, 0);

        let records = replay_document(&storage, &document).unwrap();
        assert_eq!(
            records,
            vec![
                WalRecord::SegmentHeader { document, segment_index: 0, prev_chain_hash: None },
                WalRecord::TxBegin { tx_id: 1 },
                WalRecord::Command(b"cmd-1".to_vec()),
                WalRecord::TxCommit { tx_id: 1, record_count: 1 },
            ]
        );
    }

    #[test]
    fn group_commit_batches_until_policy_threshold_then_commits() {
        let storage = MemoryStorage::new();
        let document = doc("doc-1");
        let policy = GroupCommitPolicy { max_delay_ms: 1_000_000, max_bytes: u64::MAX, max_records: 5 };
        let mut wal = DocumentWal::create(&storage, document.clone(), policy, 0).unwrap();

        // Each submit writes 3 records (begin/command/commit); Memory durability never forces a
        // commit, so nothing should be flushed to storage until pending_records >= 5.
        let receipt_1 = wal.submit(&storage, &[WalRecord::Command(b"a".to_vec())], DurabilityClass::Memory, 10).unwrap();
        assert!(!receipt_1.committed);
        assert_eq!(storage.segment_len(&document, 0).unwrap(), wal.active.flushed_len, "nothing new should have flushed yet");

        // Second submit pushes pending_records to 6 (>= max_records 5), which must commit.
        let receipt_2 = wal.submit(&storage, &[WalRecord::Command(b"b".to_vec())], DurabilityClass::Memory, 11).unwrap();
        assert!(receipt_2.committed);
        assert_eq!(wal.active.pending_records, 0);
        assert!(storage.segment_len(&document, 0).unwrap() > 32, "flush must have appended past the bare header");
    }

    #[test]
    fn fsync_durability_forces_immediate_commit_regardless_of_policy() {
        let storage = MemoryStorage::new();
        let document = doc("doc-1");
        let policy = GroupCommitPolicy { max_delay_ms: u64::MAX, max_bytes: u64::MAX, max_records: u32::MAX };
        let mut wal = DocumentWal::create(&storage, document, policy, 0).unwrap();
        let receipt = wal.submit(&storage, &[WalRecord::Command(b"a".to_vec())], DurabilityClass::Fsync, 0).unwrap();
        assert!(receipt.committed);
    }

    #[test]
    fn torn_tail_is_recovered_by_truncating_and_replaying_trusted_prefix() {
        let storage = MemoryStorage::new();
        let document = doc("doc-1");
        {
            let mut wal = DocumentWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).unwrap();
            wal.submit(&storage, &[WalRecord::Command(b"trusted".to_vec())], DurabilityClass::Fsync, 1).unwrap();
        }

        // Simulate a crash mid-append: bytes physically present past the last trusted commit,
        // written directly to storage (bypassing SprWriter, exactly like a torn OS-level write).
        storage.append(&document, 0, b"\x0Fgarbage-not-a-valid-frame-tail").unwrap();

        let (wal, report) = DocumentWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100).unwrap();
        assert!(report.torn_tail_bytes > 0);
        assert_eq!(report.segments_seen, 1);
        drop(wal);

        let records = replay_document(&storage, &document).unwrap();
        assert_eq!(
            records,
            vec![
                WalRecord::SegmentHeader { document: document.clone(), segment_index: 0, prev_chain_hash: None },
                WalRecord::TxBegin { tx_id: 1 },
                WalRecord::Command(b"trusted".to_vec()),
                WalRecord::TxCommit { tx_id: 1, record_count: 1 },
            ]
        );

        let len = storage.segment_len(&document, 0).unwrap();
        let bytes = storage.read(&document, 0, pack_core::ByteRange { offset: 0, len }).unwrap();
        let post_recovery = protocol_format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).unwrap();
        assert_eq!(post_recovery.bytes_recovered, bytes.len() as u64, "the rebuilt segment must itself be torn-tail-free");
    }

    #[test]
    fn recovery_resumes_next_tx_id_and_accepts_further_submits() {
        let storage = MemoryStorage::new();
        let document = doc("doc-1");
        {
            let mut wal = DocumentWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).unwrap();
            wal.submit(&storage, &[WalRecord::Command(b"one".to_vec())], DurabilityClass::Fsync, 1).unwrap();
            wal.submit(&storage, &[WalRecord::Command(b"two".to_vec())], DurabilityClass::Fsync, 2).unwrap();
        }

        let (mut wal, _report) = DocumentWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3).unwrap();
        let receipt = wal.submit(&storage, &[WalRecord::Command(b"three".to_vec())], DurabilityClass::Fsync, 4).unwrap();
        assert_eq!(receipt.tx_id, 3, "recovery must resume tx ids strictly past whatever was already durable");

        let records = replay_document(&storage, &document).unwrap();
        let commands: Vec<_> = records.into_iter().filter(|record| matches!(record, WalRecord::Command(_))).collect();
        assert_eq!(commands, vec![WalRecord::Command(b"one".to_vec()), WalRecord::Command(b"two".to_vec()), WalRecord::Command(b"three".to_vec())]);
    }

    #[test]
    fn multi_segment_rotation_chains_prev_hash_and_replay_spans_segments() {
        let storage = MemoryStorage::new();
        let document = doc("doc-1");
        let mut wal = DocumentWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).unwrap();
        wal.max_segment_bytes = 200; // force rotation quickly for this test

        for i in 0..20u32 {
            wal.submit(&storage, &[WalRecord::Command(format!("cmd-{i}").into_bytes())], DurabilityClass::Fsync, u64::from(i)).unwrap();
        }

        let segments = storage.list_segments(&document).unwrap();
        assert!(segments.len() >= 2, "the byte threshold must have forced at least one rotation");

        // Cross-check segment 1's WAL_SEGMENT_HEADER.prev_chain_hash against segment 0's
        // independently-recomputed tip chain_hash.
        let seg0_len = storage.segment_len(&document, 0).unwrap();
        let seg0_bytes = storage.read(&document, 0, pack_core::ByteRange { offset: 0, len: seg0_len }).unwrap();
        let seg0_report = protocol_format::recover(&seg0_bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).unwrap();
        let commit_frame_end = (seg0_report.last_commit_offset + protocol_format::COMMIT_FRAME_LEN) as usize;
        let mut cursor = protocol::FrameCursor::new(&seg0_bytes[seg0_report.last_commit_offset as usize..commit_frame_end], 0);
        let commit_frame = cursor.next_frame().unwrap().unwrap();
        let expected_chain_hash = protocol_format::parse_commit_payload(commit_frame.payload()).unwrap().chain_hash;

        let seg0_records = decode_records(&seg0_bytes[..seg0_report.bytes_recovered as usize]).unwrap();
        let seg1_records = {
            let seg1_len = storage.segment_len(&document, 1).unwrap();
            let seg1_bytes = storage.read(&document, 1, pack_core::ByteRange { offset: 0, len: seg1_len }).unwrap();
            let seg1_report = protocol_format::recover(&seg1_bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).unwrap();
            decode_records(&seg1_bytes[..seg1_report.bytes_recovered as usize]).unwrap()
        };
        match &seg1_records[0] {
            WalRecord::SegmentHeader { segment_index, prev_chain_hash, .. } => {
                assert_eq!(*segment_index, 1);
                assert_eq!(*prev_chain_hash, Some(expected_chain_hash), "segment 1's header must chain from segment 0's tip commit");
            }
            other => panic!("expected segment 1's first record to be a SegmentHeader, got {other:?}"),
        }
        assert!(matches!(seg0_records[0], WalRecord::SegmentHeader { segment_index: 0, prev_chain_hash: None, .. }));

        let full_replay = replay_document(&storage, &document).unwrap();
        let commands_in_order: Vec<String> = full_replay
            .into_iter()
            .filter_map(|record| match record {
                WalRecord::Command(bytes) => Some(String::from_utf8(bytes).unwrap()),
                _ => None,
            })
            .collect();
        let expected: Vec<String> = (0..20u32).map(|i| format!("cmd-{i}")).collect();
        assert_eq!(commands_in_order, expected, "replay must span every segment in rotation order");
    }

    #[test]
    fn recovery_rejects_a_torn_non_active_sealed_segment() {
        let storage = MemoryStorage::new();
        let document = doc("doc-1");
        let mut wal = DocumentWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).unwrap();
        wal.max_segment_bytes = 1; // rotate on the very next submit
        wal.submit(&storage, &[WalRecord::Command(b"forces-rotation".to_vec())], DurabilityClass::Fsync, 0).unwrap();
        assert!(storage.list_segments(&document).unwrap().len() >= 2);

        // Corrupt the now-sealed segment 0 by truncating a byte off its tail directly in storage
        // — WalStorage::truncate_tail refuses a sealed segment, so simulate on-disk bit rot
        // instead via delete+recreate+append of a shortened copy.
        let seg0_len = storage.segment_len(&document, 0).unwrap();
        let seg0_bytes = storage.read(&document, 0, pack_core::ByteRange { offset: 0, len: seg0_len }).unwrap();
        storage.delete_segment(&document, 0).unwrap();
        storage.create_segment(&document, 0).unwrap();
        storage.append(&document, 0, &seg0_bytes[..seg0_bytes.len() - 1]).unwrap();
        storage.seal(&document, 0).unwrap();

        let result = DocumentWal::open(&storage, document, GroupCommitPolicy::default(), 100);
        assert!(matches!(result, Err(db_core::DbError::Corrupt(_))), "a torn sealed (non-active) segment must be a hard recovery error");
    }

    #[test]
    fn empty_document_open_creates_a_fresh_wal() {
        let storage = MemoryStorage::new();
        let document = doc("doc-1");
        let (wal, report) = DocumentWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 0).unwrap();
        assert_eq!(report, WalRecoveryReport::default());
        assert_eq!(wal.active_segment_index(), 0);
        assert_eq!(storage.list_segments(&document).unwrap(), vec![0]);
    }
    //#endregion 🔖️Segment + DocumentWal
}
//#endregion 🧪️Tests
