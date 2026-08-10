//! 🧾 Protocol errors, limits, record vocabulary, and wire codecs.

//#region 🔖️Errors
/// @emoji 🚨️ The one error type every `protocol_*` public fn returns; never leaks `std::io::Error`.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ProtocolError {
    #[error(transparent)]
    Pack(#[from] crate::os_pack::codec::PackError),
    #[error("chain mismatch at commit {commit_seq}")]
    ChainMismatch { commit_seq: u64 },
    #[error("torn tail at offset {0}")]
    TornTail(u64),
    #[error("unknown critical record kind {0:#x}")]
    UnknownCriticalRecord(u8),
    #[error("dictionary index out of range: {0}")]
    DictMiss(u32),
    #[error("dictionary out of order: expected base_count {expected}, got {actual}")]
    DictOutOfOrder { expected: u32, actual: u32 },
    #[error("signature verification required but no verifier supplied")]
    VerifierRequired,
    #[error("signature invalid for commit {commit_seq}")]
    SignatureInvalid { commit_seq: u64 },
    #[error("frame back_len mismatch at offset {0}")]
    FrameFraming(u64),
    #[error("limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("malformed {what} at offset {offset}: {detail}")]
    Malformed { what: &'static str, offset: u64, detail: String },
    #[error("io error: {0}")]
    Io(String),
}

crate::fault_from_thiserror!(ProtocolError, crate::os_dsl::diagnostic::FaultOrigin::Module, "module.protocol");

//#endregion 🔖️Errors

//#region 🔖️Limits
/// @emoji 🛡️ Corruption-hardening ceilings every decoder in this crate family must validate
/// against BEFORE allocating — mirrors `crate::os_pack::codec::PackLimits`'s stated invariant.
#[derive(Clone, Debug)]
pub struct ProtocolLimits {
    pub max_file_len: u64,
    pub max_frame_len: u64,
    pub max_record_count: u64,
    pub max_dict_entries: u32,
    pub max_op_count_per_edit: u32,
    pub max_total_alloc: u64,
}

impl Default for ProtocolLimits {
    fn default() -> Self {
        Self { max_file_len: 64 * 1024 * 1024 * 1024, max_frame_len: 2 * 1024 * 1024 * 1024, max_record_count: 256_000_000, max_dict_entries: 1_000_000, max_op_count_per_edit: 100_000, max_total_alloc: 4 * 1024 * 1024 * 1024 }
    }
}
//#endregion 🔖️Limits

//#region 🔖️RecordKinds
// Plain `pub const` u8s (mirrors crate::os_pack::codec::SegmentKind convention but this family uses bare
// u8 kind bytes directly in the frame, no wrapper newtype — simpler, and every downstream crate
// matches on the byte).
/// @emoji 🔚️ Marks the end of the record stream.
pub const REC_END: u8 = 0x00;
/// @emoji 📄️ The document-identity record: doc id + schema id.
pub const REC_DOC: u8 = 0x01;
/// @emoji 🎭️ A delta into the actor-id dictionary.
pub const REC_ACTOR_DICT: u8 = 0x02;
/// @emoji 🔤️ A delta into the general string dictionary.
pub const REC_STR_DICT: u8 = 0x03;
/// @emoji ✏️ One edit (a batch of forward ops, optionally inverse ops + explicit meta).
pub const REC_EDIT: u8 = 0x04;
/// @emoji 💾️ One named change (a save point referencing edits).
pub const REC_CHANGE: u8 = 0x05;
/// @emoji 🚩️ One checkpoint (a durable milestone referencing changes).
pub const REC_CHECKPOINT: u8 = 0x06;
/// @emoji 🌿️ One named alternative (branch) referencing checkpoints.
pub const REC_ALTERNATIVE: u8 = 0x07;
/// @emoji 🎯️ Marks the currently-active alternative.
pub const REC_ACTIVE: u8 = 0x08;
/// @emoji 🏔️ A frontier summary snapshot.
pub const REC_FRONTIER: u8 = 0x09;
/// @emoji 📸️ A materialized snapshot body (opaque to this crate family).
pub const REC_PROJECTION: u8 = 0x0A;
/// @emoji 🔎️ An advisory, rebuildable offset index.
pub const REC_INDEX: u8 = 0x0B;
/// @emoji ⛓️ The commit frame: hash-chains everything written since the previous commit.
pub const REC_COMMIT: u8 = 0x0C;
/// @emoji ✍️ A detached signature over a commit's chain hash.
pub const REC_SIGNATURE: u8 = 0x0D;
/// @emoji 🕳️ A tombstone recording a redaction; the original bytes are physically gone.
pub const REC_REDACTION: u8 = 0x0E;
/// @emoji ⬆️ Records a schema-version upcast applied during a rewrite.
pub const REC_UPCAST: u8 = 0x0F;
/// @emoji 👻️ Ephemeral/preview-lane data, dropped freely by compaction.
pub const REC_EPHEMERAL: u8 = 0x10;
/// @emoji 🔏️ Marks a range of the file as sealed (immutable, already compacted).
pub const REC_SEALED: u8 = 0x11;
/// @emoji ♻️ A compaction batch: sealed, replayable inner record frames.
pub const REC_COMPACTION: u8 = 0x12;
/// @emoji ⬜️ Padding, always safely skippable.
pub const REC_PADDING: u8 = 0x7F;
// Extension range 0x40..=0x7E is caller-defined, never critical unless the frame's critical bit is set.

/// @emoji ❗️ True iff an unrecognized `kind` byte with this value must abort the reader rather
/// than being skipped (see `protocol_format`'s skip-unknown rule).
pub fn is_critical_kind(kind: u8) -> bool {
    matches!(kind, REC_DOC | REC_EDIT | REC_CHANGE | REC_CHECKPOINT | REC_ALTERNATIVE | REC_ACTIVE | REC_COMMIT | REC_ACTOR_DICT | REC_STR_DICT)
}
//#endregion 🔖️RecordKinds

//#region 🔖️Flags
// Header required/optional flags (32-byte header, see protocol_format).
/// @emoji ⛓️ Required flag bit: every commit frame's `chain_hash` must verify.
pub const REQUIRED_HASH_CHAIN: u32 = 1 << 0;
/// @emoji ✍️ Required flag bit: every commit frame must carry a valid `REC_SIGNATURE`.
pub const REQUIRED_SIGNED: u32 = 1 << 1;
/// @emoji 🔒️ Required flag bit: reserved for encryption, never set by this crate family.
pub const REQUIRED_ENCRYPTED: u32 = 1 << 2;
/// @emoji 🧮️ Optional flag bit: the document body was encoded in canonical form.
pub const OPTIONAL_CANONICAL: u32 = 1 << 0;
/// @emoji 📸️ Optional flag bit: this file contains at least one `REC_PROJECTION`.
pub const OPTIONAL_HAS_PROJECTIONS: u32 = 1 << 1;
/// @emoji 🔎️ Optional flag bit: this file contains at least one `REC_INDEX`.
pub const OPTIONAL_HAS_INDEX: u32 = 1 << 2;
/// @emoji 🕳️ Optional flag bit: this file contains at least one `REC_REDACTION`.
pub const OPTIONAL_REDACTED: u32 = 1 << 3;
// Frame flags byte (per-record, not header): bit0 compressed, bit1 critical, bits2..4 = codec id (0..=7).
/// @emoji 🗜️ Frame flags bit: the payload is compressed (see `frame_codec_id`).
pub const FRAME_FLAG_COMPRESSED: u8 = 1 << 0;
/// @emoji ❗️ Frame flags bit: an unrecognized `kind` carrying this bit aborts the reader.
pub const FRAME_FLAG_CRITICAL: u8 = 1 << 1;

/// @emoji 🗜️ Extracts the 3-bit codec id (bits 2..4) from a frame flags byte.
pub fn frame_codec_id(flags: u8) -> u8 {
    (flags >> 2) & 0b111
}

/// @emoji 🏗️ Assembles a frame flags byte from its three logical fields.
pub fn frame_flags(compressed: bool, critical: bool, codec: u8) -> u8 {
    (compressed as u8) | ((critical as u8) << 1) | ((codec & 0b111) << 2)
}
//#endregion 🔖️Flags


//#region 🔖️Policies
// UndoPolicy moved from vcs/rs (unchanged variants); MergeStrategyKind moved from framework/core
// L6636-6668 (unchanged variants). New: ConflictRule, the per-operation conflict declaration
// surface today's code lacks (crate::os_store::merge_concurrent_diffs collapses everything to absorb
// regardless of declared strategy — protocol_crdt fixes this using ConflictRule).
//
// Note: the contract's prose also mentions `ArtifactKind` moving alongside `MergeStrategyKind`,
// but the frozen signature block below only defines `UndoPolicy`/`MergeStrategyKind`/
// `ConflictRule` — `ArtifactKind` is not redefined here (it stays in `framework/core` until a
// later wave's contract actually specifies its new home).

/// @emoji ↩️ How an undo of this operation kind should be computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UndoPolicy {
    ExactBaseOnly,
    TransformAgainstConcurrent,
    SemanticUndo,
    CompensatingAction,
}

/// @emoji 🧩️ Which CRDT-style merge algorithm reconciles concurrent diffs of this kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MergeStrategyKind {
    LwwRegister,
    OrderedSequence,
    TextSequence,
    TombstonedGraphSet,
    ContentAddressedBlob,
}

/// @emoji ⚖️ The per-operation conflict declaration surface: how two concurrent instances of the
/// same operation kind resolve. `Commutes`/`Transform` need no merge strategy; `Merge`/`Crdt`
/// carry the `MergeStrategyKind` that arbitrates them (see `protocol_crdt`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConflictRule {
    Commutes,
    Transform,
    Merge(MergeStrategyKind),
    Crdt(MergeStrategyKind),
}
//#endregion 🔖️Policies

//#region 🔖️StateClass
// New: the explicit persistent/shared-ui/local-ui/preview/effect separation the db spec requires.
// Carried on MutationDescriptor (protocol_command) and on wire envelopes (protocol_wire).

/// @emoji 🗂️ Which durability/visibility class an operation's diffs belong to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StateClass {
    Persistent,
    SharedUi,
    LocalUi,
    Preview,
    Effect,
}
//#endregion 🔖️StateClass

//#region 🔖️ArtifactSchemaCatalog
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// 🍃 Five handcrafted leaf bodies for one artifact schema facet.
#[derive(Clone, Debug)]
pub struct KernelFacetLeaves {
    pub rust: &'static str,
    pub typescript: &'static str,
    pub graphql: &'static str,
    pub json_schema: &'static str,
    pub proto: &'static str,
}

/// 🧬️ Registered descriptor for one artifact's three schema facets.
#[derive(Clone, Debug)]
pub struct KernelArtifactSchemaDescriptor {
    pub id: &'static str,
    pub artifact: KernelFacetLeaves,
    pub snapshot: KernelFacetLeaves,
    pub diff: KernelFacetLeaves,
}

struct KernelArtifactSchemaCatalog {
    by_id: HashMap<&'static str, KernelArtifactSchemaDescriptor>,
}

static KERNEL_ARTIFACT_SCHEMA_CATALOG: OnceLock<Mutex<KernelArtifactSchemaCatalog>> = OnceLock::new();

fn kernel_artifact_schema_catalog() -> &'static Mutex<KernelArtifactSchemaCatalog> {
    KERNEL_ARTIFACT_SCHEMA_CATALOG.get_or_init(|| Mutex::new(KernelArtifactSchemaCatalog { by_id: HashMap::new() }))
}

/// 📎 Registers one artifact's handcrafted schema descriptor into the OS-wide catalog.
pub fn register_kernel_artifact_schema_descriptor(descriptor: KernelArtifactSchemaDescriptor) {
    kernel_artifact_schema_catalog()
        .lock()
        .expect("kernel artifact schema catalog lock")
        .by_id
        .insert(descriptor.id, descriptor);
}

/// 🔎 Whether `id` is registered in the OS-wide artifact schema catalog.
pub fn kernel_artifact_schema_descriptor_registered(id: &str) -> bool {
    kernel_artifact_schema_catalog()
        .lock()
        .expect("kernel artifact schema catalog lock")
        .by_id
        .contains_key(id)
}

/// 🔢 Count of registered artifact schema ids.
pub fn kernel_artifact_schema_catalog_len() -> usize {
    kernel_artifact_schema_catalog()
        .lock()
        .expect("kernel artifact schema catalog lock")
        .by_id
        .len()
}

/// 📚 Invokes `visit` with every registered kernel descriptor.
pub fn with_kernel_artifact_schema_catalog<R>(visit: impl FnOnce(&[KernelArtifactSchemaDescriptor]) -> R) -> R {
    let guard = kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock");
    let mut entries: Vec<KernelArtifactSchemaDescriptor> = guard.by_id.values().cloned().collect();
    entries.sort_by_key(|entry| entry.id);
    visit(&entries)
}
//#endregion 🔖️ArtifactSchemaCatalog

//#region 🔖️AppSchemaCatalog
/// 🧬️ Registered descriptor for one app owner's config + presence schema facets.
#[derive(Clone, Debug)]
pub struct KernelAppSchemaDescriptor {
    pub id: &'static str,
    pub config: KernelFacetLeaves,
    pub presence: KernelFacetLeaves,
}

struct KernelAppSchemaCatalog {
    by_id: HashMap<&'static str, KernelAppSchemaDescriptor>,
}

static KERNEL_APP_SCHEMA_CATALOG: OnceLock<Mutex<KernelAppSchemaCatalog>> = OnceLock::new();

fn kernel_app_schema_catalog() -> &'static Mutex<KernelAppSchemaCatalog> {
    KERNEL_APP_SCHEMA_CATALOG.get_or_init(|| Mutex::new(KernelAppSchemaCatalog { by_id: HashMap::new() }))
}

/// 📎 Registers one app owner's handcrafted schema descriptor into the OS-wide catalog.
pub fn register_kernel_app_schema_descriptor(descriptor: KernelAppSchemaDescriptor) {
    kernel_app_schema_catalog()
        .lock()
        .expect("kernel app schema catalog lock")
        .by_id
        .insert(descriptor.id, descriptor);
}

/// 🔎 Whether `id` is registered in the OS-wide app schema catalog.
pub fn kernel_app_schema_descriptor_registered(id: &str) -> bool {
    kernel_app_schema_catalog()
        .lock()
        .expect("kernel app schema catalog lock")
        .by_id
        .contains_key(id)
}

/// 🔢 Count of registered app schema owner ids.
pub fn kernel_app_schema_catalog_len() -> usize {
    kernel_app_schema_catalog()
        .lock()
        .expect("kernel app schema catalog lock")
        .by_id
        .len()
}

/// 📚 Invokes `visit` with every registered kernel app descriptor.
pub fn with_kernel_app_schema_catalog<R>(visit: impl FnOnce(&[KernelAppSchemaDescriptor]) -> R) -> R {
    let guard = kernel_app_schema_catalog().lock().expect("kernel app schema catalog lock");
    let mut entries: Vec<KernelAppSchemaDescriptor> = guard.by_id.values().cloned().collect();
    entries.sort_by_key(|entry| entry.id);
    visit(&entries)
}
//#endregion 🔖️AppSchemaCatalog


//#region 🔖️WireCodec
/// @emoji 🎞️ Shared primitive codec for `protocol_causal`'s envelope records and `protocol_wire`'s
/// frame bodies (W5) — hand-rolled, not `crate::os_pack::value::encode_record_body`, because the TS twin on
/// the other end of the wire must reproduce these bytes exactly and has no pack engine to port;
/// these primitives are simple enough to hand-implement identically in both languages. All
/// integers are varint (documents stay `< 2^53`, safe for JS numbers); `Vec<u8>` and `[u8; 32]`
/// are the only fixed-width forms. Field order within a record is always declaration order — no
/// tags, mirroring `crate::os_dsl::op_rt`'s "position is the format" convention one level down.
pub fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
    crate::os_pack::codec::write_varint_u64(out, value);
}

pub fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, ProtocolError> {
    Ok(crate::os_pack::codec::read_varint_u64(bytes, pos)?)
}

/// @emoji 🔤️ `varint len + utf8 bytes`.
pub fn write_str(out: &mut Vec<u8>, s: &str) {
    write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

pub fn read_str(bytes: &[u8], pos: &mut usize) -> Result<String, ProtocolError> {
    let len = read_varint_u64(bytes, pos)? as usize;
    let end = pos.checked_add(len).ok_or(ProtocolError::Malformed { what: "wire str", offset: *pos as u64, detail: "length overflow".to_string() })?;
    let slice = bytes.get(*pos..end).ok_or(ProtocolError::Malformed { what: "wire str", offset: *pos as u64, detail: "truncated".to_string() })?;
    let s = std::str::from_utf8(slice).map_err(|_| ProtocolError::Malformed { what: "wire str", offset: *pos as u64, detail: "invalid utf8".to_string() })?.to_string();
    *pos = end;
    Ok(s)
}

/// @emoji 📦️ `varint len + raw bytes`.
pub fn write_bytes(out: &mut Vec<u8>, b: &[u8]) {
    write_varint_u64(out, b.len() as u64);
    out.extend_from_slice(b);
}

pub fn read_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<u8>, ProtocolError> {
    let len = read_varint_u64(bytes, pos)? as usize;
    let end = pos.checked_add(len).ok_or(ProtocolError::Malformed { what: "wire bytes", offset: *pos as u64, detail: "length overflow".to_string() })?;
    let slice = bytes.get(*pos..end).ok_or(ProtocolError::Malformed { what: "wire bytes", offset: *pos as u64, detail: "truncated".to_string() })?;
    *pos = end;
    Ok(slice.to_vec())
}

/// @emoji #⃣ 32 raw bytes, no length prefix (fixed width).
pub fn write_hash32(out: &mut Vec<u8>, h: &[u8; 32]) {
    out.extend_from_slice(h);
}

pub fn read_hash32(bytes: &[u8], pos: &mut usize) -> Result<[u8; 32], ProtocolError> {
    let end = pos.checked_add(32).ok_or(ProtocolError::Malformed { what: "wire hash32", offset: *pos as u64, detail: "length overflow".to_string() })?;
    let slice = bytes.get(*pos..end).ok_or(ProtocolError::Malformed { what: "wire hash32", offset: *pos as u64, detail: "truncated".to_string() })?;
    let mut out = [0u8; 32];
    out.copy_from_slice(slice);
    *pos = end;
    Ok(out)
}

/// @emoji ✅️❌️ `bool` as one byte (0/1) — never a varint, to keep single-byte fields self-evident
/// when eyeballing a hex dump.
pub fn write_bool(out: &mut Vec<u8>, value: bool) {
    out.push(if value { 1 } else { 0 });
}

pub fn read_bool(bytes: &[u8], pos: &mut usize) -> Result<bool, ProtocolError> {
    let byte = *bytes.get(*pos).ok_or(ProtocolError::Malformed { what: "wire bool", offset: *pos as u64, detail: "truncated".to_string() })?;
    *pos += 1;
    Ok(byte != 0)
}

/// @emoji 🔢️ `f64` as 8 little-endian bytes — for wire fields with no varint-friendly shape
/// (coordinates, zoom levels).
pub fn write_f64(out: &mut Vec<u8>, value: f64) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn read_f64(bytes: &[u8], pos: &mut usize) -> Result<f64, ProtocolError> {
    let end = *pos + 8;
    let slice = bytes.get(*pos..end).ok_or(ProtocolError::Malformed { what: "wire f64", offset: *pos as u64, detail: "truncated".to_string() })?;
    let mut buf = [0u8; 8];
    buf.copy_from_slice(slice);
    *pos = end;
    Ok(f64::from_le_bytes(buf))
}
//#endregion 🔖️WireCodec


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DictBuilder, DictReader, ArtifactVersion, HybridLogicalTimestamp, MutationId, RecordHasher};

    //#region 🔖️Errors
    #[test]
    fn pack_error_converts_into_protocol_error_via_from() {
        let pack_err = crate::os_pack::codec::PackError::Truncated(7);
        let protocol_err: ProtocolError = pack_err.clone().into();
        assert_eq!(protocol_err, ProtocolError::Pack(pack_err));
    }
    //#endregion 🔖️Errors

    //#region 🔖️Limits
    #[test]
    fn protocol_limits_default_matches_contract() {
        let limits = ProtocolLimits::default();
        assert_eq!(limits.max_file_len, 64 * 1024 * 1024 * 1024);
        assert_eq!(limits.max_frame_len, 2 * 1024 * 1024 * 1024);
        assert_eq!(limits.max_record_count, 256_000_000);
        assert_eq!(limits.max_dict_entries, 1_000_000);
        assert_eq!(limits.max_op_count_per_edit, 100_000);
        assert_eq!(limits.max_total_alloc, 4 * 1024 * 1024 * 1024);
    }
    //#endregion 🔖️Limits

    //#region 🔖️RecordKinds
    #[test]
    fn record_kind_constants_match_contract() {
        assert_eq!(REC_END, 0x00);
        assert_eq!(REC_DOC, 0x01);
        assert_eq!(REC_ACTOR_DICT, 0x02);
        assert_eq!(REC_STR_DICT, 0x03);
        assert_eq!(REC_EDIT, 0x04);
        assert_eq!(REC_CHANGE, 0x05);
        assert_eq!(REC_CHECKPOINT, 0x06);
        assert_eq!(REC_ALTERNATIVE, 0x07);
        assert_eq!(REC_ACTIVE, 0x08);
        assert_eq!(REC_FRONTIER, 0x09);
        assert_eq!(REC_PROJECTION, 0x0A);
        assert_eq!(REC_INDEX, 0x0B);
        assert_eq!(REC_COMMIT, 0x0C);
        assert_eq!(REC_SIGNATURE, 0x0D);
        assert_eq!(REC_REDACTION, 0x0E);
        assert_eq!(REC_UPCAST, 0x0F);
        assert_eq!(REC_EPHEMERAL, 0x10);
        assert_eq!(REC_SEALED, 0x11);
        assert_eq!(REC_COMPACTION, 0x12);
        assert_eq!(REC_PADDING, 0x7F);
    }

    #[test]
    fn is_critical_kind_matches_contract_set() {
        for kind in [REC_DOC, REC_EDIT, REC_CHANGE, REC_CHECKPOINT, REC_ALTERNATIVE, REC_ACTIVE, REC_COMMIT, REC_ACTOR_DICT, REC_STR_DICT] {
            assert!(is_critical_kind(kind), "{kind:#x} should be critical");
        }
        for kind in [REC_END, REC_FRONTIER, REC_PROJECTION, REC_INDEX, REC_SIGNATURE, REC_REDACTION, REC_UPCAST, REC_EPHEMERAL, REC_SEALED, REC_COMPACTION, REC_PADDING, 0x50] {
            assert!(!is_critical_kind(kind), "{kind:#x} should not be critical");
        }
    }
    //#endregion 🔖️RecordKinds

    //#region 🔖️Flags
    #[test]
    fn frame_flags_round_trips_codec_id() {
        for codec in 0u8..=7 {
            for compressed in [false, true] {
                for critical in [false, true] {
                    let flags = frame_flags(compressed, critical, codec);
                    assert_eq!(flags & FRAME_FLAG_COMPRESSED != 0, compressed);
                    assert_eq!(flags & FRAME_FLAG_CRITICAL != 0, critical);
                    assert_eq!(frame_codec_id(flags), codec);
                }
            }
        }
    }

    #[test]
    fn frame_codec_id_masks_to_three_bits() {
        assert_eq!(frame_codec_id(0b1111_1100), 0b111);
    }
    //#endregion 🔖️Flags

    //#region 🔖️Scalars
    mod scalars {
        use crate::os_spr::scalar::scalar::{read_id, read_timestamp, write_id, write_timestamp};
        use crate::os_pack::codec::{ByteReader, ByteWriter};

        #[test]
        fn timestamp_round_trips_canonical_utc_no_fraction() {
            let raw = "2024-01-15T10:30:00Z";
            let mut out = ByteWriter::new();
            let epoch = write_timestamp(&mut out, raw, None);
            assert!(epoch.is_some());
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (decoded, epoch_back) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
            assert_eq!(epoch_back, epoch);
        }

        #[test]
        fn timestamp_round_trips_canonical_utc_with_fraction() {
            let raw = "2024-01-15T10:30:00.123Z";
            let mut out = ByteWriter::new();
            write_timestamp(&mut out, raw, None);
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
        }

        #[test]
        fn timestamp_falls_back_to_raw_for_non_canonical_text() {
            let raw = "not-a-timestamp";
            let mut out = ByteWriter::new();
            let epoch = write_timestamp(&mut out, raw, None);
            assert_eq!(epoch, None);
            let bytes = out.into_bytes();
            assert_eq!(bytes[0], 0, "tag byte must be 0 (raw)");
            let mut reader = ByteReader::new(&bytes);
            let (decoded, epoch_back) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
            assert_eq!(epoch_back, None);
        }

        #[test]
        fn timestamp_falls_back_to_raw_for_non_utc_offset() {
            let raw = "2024-01-15T10:30:00+02:00";
            let mut out = ByteWriter::new();
            let epoch = write_timestamp(&mut out, raw, None);
            assert_eq!(epoch, None);
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
        }

        #[test]
        fn timestamp_chain_uses_delta_tag_after_first_absolute() {
            let mut out = ByteWriter::new();
            let e1 = write_timestamp(&mut out, "2024-01-15T10:30:00Z", None).unwrap();
            let e2 = write_timestamp(&mut out, "2024-01-15T10:30:05Z", Some(e1)).unwrap();
            assert_eq!(e2 - e1, 5_000);
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (d1, p1) = read_timestamp(&mut reader, None).unwrap();
            let (d2, p2) = read_timestamp(&mut reader, p1).unwrap();
            assert_eq!(d1, "2024-01-15T10:30:00Z");
            assert_eq!(d2, "2024-01-15T10:30:05Z");
            assert_eq!(p2, Some(e2));
        }

        #[test]
        fn timestamp_epoch_zero_round_trips() {
            let raw = "1970-01-01T00:00:00Z";
            let mut out = ByteWriter::new();
            let epoch = write_timestamp(&mut out, raw, None);
            assert_eq!(epoch, Some(0));
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
        }

        #[test]
        fn id_round_trips_via_edit_ordinal_tag() {
            let mut out = ByteWriter::new();
            write_id(&mut out, "edit-7", |_| unreachable!("must not intern"), |id| (id == "edit-7").then_some(7)).unwrap();
            let bytes = out.into_bytes();
            assert_eq!(bytes[0], 3, "tag byte must be 3 (edit-ordinal)");
            let mut reader = ByteReader::new(&bytes);
            let decoded = read_id(&mut reader, |_| unreachable!("must not resolve"), |ordinal| if ordinal == 7 { Ok("edit-7") } else { Err(crate::os_pack::codec::PackError::Truncated(0)) }).unwrap();
            assert_eq!(decoded, "edit-7");
        }

        #[test]
        fn id_round_trips_via_prefix_uuid_tag() {
            let id = "actor-3fa85f64-5717-4562-b3fc-2c963f66afa6";
            let mut out = ByteWriter::new();
            write_id(
                &mut out,
                id,
                |s| {
                    assert_eq!(s, "actor");
                    0
                },
                |_| None,
            )
            .unwrap();
            let bytes = out.into_bytes();
            assert_eq!(bytes[0], 2, "tag byte must be 2 (prefix+uuid)");
            let mut reader = ByteReader::new(&bytes);
            let decoded = read_id(&mut reader, |idx| if idx == 0 { Ok("actor") } else { Err(crate::os_pack::codec::PackError::Truncated(0)) }, |_| unreachable!("must not resolve ordinal")).unwrap();
            assert_eq!(decoded, id);
        }

        #[test]
        fn id_falls_back_to_dictref_tag_for_plain_strings() {
            let mut out = ByteWriter::new();
            write_id(
                &mut out,
                "hello-world",
                |s| {
                    assert_eq!(s, "hello-world");
                    42
                },
                |_| None,
            )
            .unwrap();
            let bytes = out.into_bytes();
            assert_eq!(bytes[0], 1, "tag byte must be 1 (dictref)");
            let mut reader = ByteReader::new(&bytes);
            let decoded = read_id(&mut reader, |idx| if idx == 42 { Ok("hello-world") } else { Err(crate::os_pack::codec::PackError::Truncated(0)) }, |_| unreachable!("must not resolve ordinal")).unwrap();
            assert_eq!(decoded, "hello-world");
        }

        #[test]
        fn id_raw_tag_is_readable_even_though_writer_never_emits_it() {
            let mut out = ByteWriter::new();
            out.write_u8(0);
            out.write_varint_u64(5);
            out.write_bytes(b"hello");
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let decoded = read_id(&mut reader, |_| unreachable!(), |_| unreachable!()).unwrap();
            assert_eq!(decoded, "hello");
        }

        #[test]
        fn id_dictref_dedupes_repeated_ids_through_intern_closure() {
            let mut dict: Vec<String> = Vec::new();
            let mut out = ByteWriter::new();
            {
                let mut intern = |s: &str| -> u32 {
                    if let Some(pos) = dict.iter().position(|e| e == s) {
                        pos as u32
                    } else {
                        dict.push(s.to_string());
                        (dict.len() - 1) as u32
                    }
                };
                write_id(&mut out, "same-id", &mut intern, |_| None).unwrap();
                write_id(&mut out, "same-id", &mut intern, |_| None).unwrap();
            }
            assert_eq!(dict.len(), 1, "second write must reuse the same dictionary slot");
        }
    }
    //#endregion 🔖️Scalars

    //#region 🔖️Dictionary
    #[test]
    fn dict_builder_interns_deterministically_and_dedupes() {
        let mut builder = DictBuilder::new();
        assert!(builder.is_empty());
        assert_eq!(builder.intern("a"), 0);
        assert_eq!(builder.intern("b"), 1);
        assert_eq!(builder.intern("a"), 0);
        assert_eq!(builder.len(), 2);
        assert_eq!(builder.entries_since(0), &["a".to_string(), "b".to_string()]);
        assert_eq!(builder.entries_since(1), &["b".to_string()]);
    }

    #[test]
    fn dict_reader_round_trips_builder_deltas_in_order() {
        let mut builder = DictBuilder::new();
        builder.intern("x");
        builder.intern("y");
        let mut reader = DictReader::new();
        reader.extend(0, builder.entries_since(0).to_vec()).unwrap();
        assert_eq!(reader.resolve(0).unwrap(), "x");
        assert_eq!(reader.resolve(1).unwrap(), "y");
        assert_eq!(reader.len(), 2);

        builder.intern("z");
        reader.extend(2, builder.entries_since(2).to_vec()).unwrap();
        assert_eq!(reader.resolve(2).unwrap(), "z");
    }

    #[test]
    fn dict_reader_rejects_out_of_order_deltas() {
        let mut reader = DictReader::new();
        let err = reader.extend(5, vec!["late".to_string()]).unwrap_err();
        assert_eq!(err, ProtocolError::DictOutOfOrder { expected: 0, actual: 5 });
    }

    #[test]
    fn dict_reader_reports_miss_past_the_end() {
        let reader = DictReader::new();
        assert_eq!(reader.resolve(0).unwrap_err(), ProtocolError::DictMiss(0));
    }
    //#endregion 🔖️Dictionary

    //#region 🔖️Crypto
    struct FixedHasher;
    impl RecordHasher for FixedHasher {
        fn hash(&self, bytes: &[u8]) -> [u8; 32] {
            let mut out = [0u8; 32];
            out[0] = bytes.len() as u8;
            out
        }
    }

    #[test]
    fn record_hasher_trait_is_object_usable() {
        let hasher = FixedHasher;
        assert_eq!(hasher.hash(b"abc")[0], 3);
    }
    //#endregion 🔖️Crypto

    //#region 🔖️HybridLogicalTimestamp
    #[test]
    fn hlc_tick_advances_on_newer_physical_time() {
        let mut hlc = HybridLogicalTimestamp::new(1, 100);
        hlc.tick(200);
        assert_eq!(hlc.physical_ms, 200);
        assert_eq!(hlc.logical, 0);
    }

    #[test]
    fn hlc_tick_bumps_logical_on_equal_or_older_physical_time() {
        let mut hlc = HybridLogicalTimestamp::new(1, 100);
        hlc.tick(100);
        assert_eq!(hlc.logical, 1);
        hlc.tick(50);
        assert_eq!(hlc.logical, 2);
    }

    #[test]
    fn hlc_merge_adopts_the_greater_remote_tick_then_bumps() {
        let mut local = HybridLogicalTimestamp::new(1, 100);
        let remote = HybridLogicalTimestamp { actor: 2, physical_ms: 150, logical: 3 };
        local.merge(&remote);
        assert_eq!(local.physical_ms, 150);
        assert_eq!(local.logical, 4);
    }

    #[test]
    fn hlc_ordering_uses_actor_as_final_tiebreak() {
        let a = HybridLogicalTimestamp { actor: 1, physical_ms: 100, logical: 5 };
        let b = HybridLogicalTimestamp { actor: 2, physical_ms: 100, logical: 5 };
        assert!(a < b, "equal physical_ms/logical must tiebreak by actor, not compare Equal");
        assert_ne!(a.cmp_key(), b.cmp_key());
    }

    #[test]
    fn hlc_ordering_prioritizes_physical_then_logical_then_actor() {
        let older = HybridLogicalTimestamp { actor: 9, physical_ms: 100, logical: 0 };
        let newer_physical = HybridLogicalTimestamp { actor: 0, physical_ms: 101, logical: 0 };
        let newer_logical = HybridLogicalTimestamp { actor: 0, physical_ms: 100, logical: 1 };
        assert!(older < newer_physical);
        assert!(older < newer_logical);
        assert!(newer_logical < newer_physical);
    }
    //#endregion 🔖️HybridLogicalTimestamp

    //#region 🔖️Identifiers
    #[test]
    fn identifier_newtypes_serde_round_trip_transparently() {
        let op = MutationId("op-1".to_string());
        let json = serde_json::to_string(&op).unwrap();
        assert_eq!(json, "\"op-1\"");
        assert_eq!(serde_json::from_str::<MutationId>(&json).unwrap(), op);

        let version = ArtifactVersion(42);
        let json = serde_json::to_string(&version).unwrap();
        assert_eq!(json, "42");
        assert_eq!(serde_json::from_str::<ArtifactVersion>(&json).unwrap(), version);
    }

    #[test]
    fn document_version_orders_numerically() {
        assert!(ArtifactVersion(1) < ArtifactVersion(2));
    }
    //#endregion 🔖️Identifiers

    //#region 🔖️Policies
    #[test]
    fn conflict_rule_carries_merge_strategy_variants() {
        let commutes = ConflictRule::Commutes;
        let merge = ConflictRule::Merge(MergeStrategyKind::LwwRegister);
        let crdt = ConflictRule::Crdt(MergeStrategyKind::TextSequence);
        assert_ne!(commutes, merge);
        assert_ne!(merge, crdt);
        assert_eq!(ConflictRule::Merge(MergeStrategyKind::LwwRegister), merge);
    }

    #[test]
    fn undo_policy_and_state_class_serde_round_trip() {
        for policy in [UndoPolicy::ExactBaseOnly, UndoPolicy::TransformAgainstConcurrent, UndoPolicy::SemanticUndo, UndoPolicy::CompensatingAction] {
            let json = serde_json::to_string(&policy).unwrap();
            assert_eq!(serde_json::from_str::<UndoPolicy>(&json).unwrap(), policy);
        }
        for class in [StateClass::Persistent, StateClass::SharedUi, StateClass::LocalUi, StateClass::Preview, StateClass::Effect] {
            let json = serde_json::to_string(&class).unwrap();
            assert_eq!(serde_json::from_str::<StateClass>(&json).unwrap(), class);
        }
    }
    //#endregion 🔖️Policies

    //#region 🔖️WireCodec
    #[test]
    fn wire_str_round_trips_including_multibyte_utf8() {
        let mut out = Vec::new();
        write_str(&mut out, "héllo wörld 🎞️");
        let mut pos = 0;
        assert_eq!(read_str(&out, &mut pos).unwrap(), "héllo wörld 🎞️");
        assert_eq!(pos, out.len());
    }

    #[test]
    fn wire_str_empty_round_trips() {
        let mut out = Vec::new();
        write_str(&mut out, "");
        let mut pos = 0;
        assert_eq!(read_str(&out, &mut pos).unwrap(), "");
        assert_eq!(pos, out.len());
    }

    #[test]
    fn wire_bytes_round_trips_and_consumes_exact_length() {
        let mut out = Vec::new();
        write_bytes(&mut out, &[1, 2, 3, 4, 5]);
        write_bytes(&mut out, &[9]);
        let mut pos = 0;
        assert_eq!(read_bytes(&out, &mut pos).unwrap(), vec![1, 2, 3, 4, 5]);
        assert_eq!(read_bytes(&out, &mut pos).unwrap(), vec![9]);
        assert_eq!(pos, out.len());
    }

    #[test]
    fn wire_hash32_round_trips_fixed_width_no_length_prefix() {
        let hash = [7u8; 32];
        let mut out = Vec::new();
        write_hash32(&mut out, &hash);
        assert_eq!(out.len(), 32, "hash32 must be fixed-width with no length prefix");
        let mut pos = 0;
        assert_eq!(read_hash32(&out, &mut pos).unwrap(), hash);
        assert_eq!(pos, 32);
    }

    #[test]
    fn wire_bool_round_trips_as_a_single_byte() {
        let mut out = Vec::new();
        write_bool(&mut out, true);
        write_bool(&mut out, false);
        assert_eq!(out, vec![1, 0]);
        let mut pos = 0;
        assert!(read_bool(&out, &mut pos).unwrap());
        assert!(!read_bool(&out, &mut pos).unwrap());
    }

    #[test]
    fn wire_varint_u64_round_trips_via_pack_core() {
        let mut out = Vec::new();
        write_varint_u64(&mut out, 300);
        let mut pos = 0;
        assert_eq!(read_varint_u64(&out, &mut pos).unwrap(), 300);
    }

    #[test]
    fn wire_str_rejects_truncated_input() {
        let mut out = Vec::new();
        write_str(&mut out, "hello");
        out.truncate(out.len() - 1);
        let mut pos = 0;
        assert!(matches!(read_str(&out, &mut pos), Err(ProtocolError::Malformed { .. })));
    }

    #[test]
    fn wire_bytes_rejects_truncated_input() {
        let mut out = Vec::new();
        write_bytes(&mut out, &[1, 2, 3]);
        out.truncate(out.len() - 1);
        let mut pos = 0;
        assert!(matches!(read_bytes(&out, &mut pos), Err(ProtocolError::Malformed { .. })));
    }

    #[test]
    fn wire_hash32_rejects_truncated_input() {
        let bytes = [0u8; 10];
        let mut pos = 0;
        assert!(matches!(read_hash32(&bytes, &mut pos), Err(ProtocolError::Malformed { .. })));
    }
    //#endregion 🔖️WireCodec
}
//#endregion 🧪️Tests
