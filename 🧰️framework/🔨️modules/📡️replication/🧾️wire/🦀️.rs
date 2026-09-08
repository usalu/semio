//! 🧾 Protocol errors, limits, record vocabulary, and wire codecs.

//#region 🔖️Errors
/// @emoji 🚨️ The one error type every `protocol_*` public fn returns; never leaks `std::io::Error`.
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolError {
    Pack(crate::codec::PackError),
    ChainMismatch { commit_seq: u64 },
    TornTail(u64),
    UnknownCriticalRecord(u8),
    DictMiss(u32),
    DictOutOfOrder { expected: u32, actual: u32 },
    VerifierRequired,
    SignatureInvalid { commit_seq: u64 },
    FrameFraming(u64),
    LimitExceeded(&'static str),
    Malformed { what: &'static str, offset: u64, detail: String },
    Io(String),
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pack(error) => error.fmt(formatter),
            Self::ChainMismatch { commit_seq } => write!(formatter, "chain mismatch at commit {commit_seq}"),
            Self::TornTail(offset) => write!(formatter, "torn tail at offset {offset}"),
            Self::UnknownCriticalRecord(kind) => write!(formatter, "unknown critical record kind {kind:#x}"),
            Self::DictMiss(index) => write!(formatter, "dictionary index out of range: {index}"),
            Self::DictOutOfOrder { expected, actual } => write!(formatter, "dictionary out of order: expected base_count {expected}, got {actual}"),
            Self::VerifierRequired => formatter.write_str("signature verification required but no verifier supplied"),
            Self::SignatureInvalid { commit_seq } => write!(formatter, "signature invalid for commit {commit_seq}"),
            Self::FrameFraming(offset) => write!(formatter, "frame back_len mismatch at offset {offset}"),
            Self::LimitExceeded(limit) => write!(formatter, "limit exceeded: {limit}"),
            Self::Malformed { what, offset, detail } => write!(formatter, "malformed {what} at offset {offset}: {detail}"),
            Self::Io(message) => write!(formatter, "io error: {message}"),
        }
    }
}

impl std::error::Error for ProtocolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pack(error) => Some(error),
            _ => None,
        }
    }
}

impl From<crate::codec::PackError> for ProtocolError {
    fn from(error: crate::codec::PackError) -> Self {
        Self::Pack(error)
    }
}

crate::fault_from_error!(ProtocolError, crate::diagnostic::FaultOrigin::Module, "module.protocol");

//#endregion 🔖️Errors

//#region 🔖️Limits
/// @emoji 🛡️ Corruption-hardening ceilings every decoder in this crate family must validate
/// against BEFORE allocating — mirrors `crate::codec::PackLimits`'s stated invariant.
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
// Plain `pub const` u8s (mirrors crate::codec::SegmentKind convention but this family uses bare
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
// Frame flags byte (per-record, not header): bit0 compressed, bit1 critical, bits2..4 = codec id (0..=7).await.
/// @emoji 🗜️ Frame flags bit: the payload is compressed (see `frame_codec_id`).
pub const FRAME_FLAG_COMPRESSED: u8 = 1 << 0;
/// @emoji ❗️ Frame flags bit: an unrecognized `kind` carrying this bit aborts the reader.
pub const FRAME_FLAG_CRITICAL: u8 = 1 << 1;

/// @emoji 🗜️ Extracts the 3-bit codec id (bits 2..4).await from a frame flags byte.
pub fn frame_codec_id(flags: u8) -> u8 {
    (flags >> 2) & 0b111
}

/// @emoji 🏗️ Assembles a frame flags byte from its three logical fields.
pub fn frame_flags(compressed: bool, critical: bool, codec: u8) -> u8 {
    (compressed as u8) | ((critical as u8) << 1) | ((codec & 0b111) << 2)
}
//#endregion 🔖️Flags

//#region 🔖️Policies
// UndoPolicy moved from vcs/rs (unchanged variants), untouched by
// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`. The CRDT-era per-operation
// conflict-declaration surface (five blind merge combinators) is GONE — CLAUDE.md forbids CRDTs
// outright, and the mechanism was unreachable from the real store path. `MergePolicy` replaces it:
// authority-local state (never
// carried on a `MutationEnvelope`/`BackboneMessage`, never part of shared history) that decides
// whether a `MutationOutcome`'s worst `crate::diagnostic::Severity` gets accepted or quarantined as a
// `Conflict` (`📡️spr/⚔️conflict`).

/// @emoji ↩️ How an undo of this operation kind should be computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UndoPolicy {
    ExactBaseOnly,
    TransformAgainstConcurrent,
    SemanticUndo,
    CompensatingAction,
}

/// 🌉️ Hand-written, not derived — same DAG reason as `HybridLogicalTimestamp`/`ids` (this crate
/// sits below `os-kernel`). A fieldless enum with no `#[serde(...)]` tag attribute serializes as
/// its bare variant name string; mirrored here exactly.
impl crate::value::ToValue for UndoPolicy {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(
            match self {
                UndoPolicy::ExactBaseOnly => "ExactBaseOnly",
                UndoPolicy::TransformAgainstConcurrent => "TransformAgainstConcurrent",
                UndoPolicy::SemanticUndo => "SemanticUndo",
                UndoPolicy::CompensatingAction => "CompensatingAction",
            }
            .to_string(),
        )
    }
}
impl crate::value::FromValue for UndoPolicy {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::String(tag) = value else {
            return Err(crate::value::ValueError::new(format!("expected a string for UndoPolicy, found {value:?}")));
        };
        match tag.as_str() {
            "ExactBaseOnly" => Ok(UndoPolicy::ExactBaseOnly),
            "TransformAgainstConcurrent" => Ok(UndoPolicy::TransformAgainstConcurrent),
            "SemanticUndo" => Ok(UndoPolicy::SemanticUndo),
            "CompensatingAction" => Ok(UndoPolicy::CompensatingAction),
            other => Err(crate::value::ValueError::new(format!("unknown UndoPolicy variant `{other}`"))),
        }
    }
}

/// @emoji ⚖️ How strict an authority is about accepting a `MutationOutcome` whose messages reach a
/// given `crate::diagnostic::Severity`. Local/authority state only — never wire-carried, never part of
/// an artifact's shared history (see the region doc above). `Normal` is the default: the
/// least-surprising choice for a fresh authority that has never been configured.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MergePolicy {
    LaissezFaire,
    #[default]
    Normal,
    Vigilant,
}

/// 🌉️ Hand-written, not derived — same DAG reason as `UndoPolicy` above. A fieldless enum with no
/// `#[serde(...)]` tag attribute serializes as its bare variant name string; mirrored here exactly.
impl crate::value::ToValue for MergePolicy {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(
            match self {
                MergePolicy::LaissezFaire => "LaissezFaire",
                MergePolicy::Normal => "Normal",
                MergePolicy::Vigilant => "Vigilant",
            }
            .to_string(),
        )
    }
}
impl crate::value::FromValue for MergePolicy {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::String(tag) = value else {
            return Err(crate::value::ValueError::new(format!("expected a string for MergePolicy, found {value:?}")));
        };
        match tag.as_str() {
            "LaissezFaire" => Ok(MergePolicy::LaissezFaire),
            "Normal" => Ok(MergePolicy::Normal),
            "Vigilant" => Ok(MergePolicy::Vigilant),
            other => Err(crate::value::ValueError::new(format!("unknown MergePolicy variant `{other}`"))),
        }
    }
}

impl MergePolicy {
    /// @emoji 🚫️ Whether this policy rejects an outcome whose worst level is `level`:
    /// `LaissezFaire` only rejects `Fatal`; `Normal` rejects `Error` and `Fatal`; `Vigilant` rejects
    /// `Warning`, `Error`, and `Fatal`. `Info` is never rejected by any policy.
    pub fn rejects(self, level: crate::diagnostic::Severity) -> bool {
        let floor = match self {
            MergePolicy::LaissezFaire => crate::diagnostic::Severity::Fatal,
            MergePolicy::Normal => crate::diagnostic::Severity::Error,
            MergePolicy::Vigilant => crate::diagnostic::Severity::Warning,
        };
        level >= floor
    }

    /// 🔢️ Stable numeric mirror of declaration order, 0..2.
    pub fn as_u8(self) -> u8 {
        match self {
            MergePolicy::LaissezFaire => 0,
            MergePolicy::Normal => 1,
            MergePolicy::Vigilant => 2,
        }
    }

    /// 🔢️ Inverse of [`as_u8`](Self::as_u8); `None` for any value outside 0..2.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(MergePolicy::LaissezFaire),
            1 => Some(MergePolicy::Normal),
            2 => Some(MergePolicy::Vigilant),
            _ => None,
        }
    }
}
//#endregion 🔖️Policies

//#region 🔖️StateClass
// The four — and only four — state mechanisms the architecture admits, spanning the
// durability × visibility square. Carried on MutationDescriptor (protocol_command) and on wire
// envelopes (protocol_wire).

/// @emoji 🗂️ Which of the four state lanes an operation's diffs belong to. Exhaustive by
/// construction: `Artifact` = persisted shared, `Config` = persisted local-only, `Presence` =
/// ephemeral shared, `Transient` = ephemeral local-only UI state.
///
/// Draftness is a LANE property — which store a record lives in — never a field annotation, so a
/// draft artifact's fields are still [`StateClass::Artifact`]. Derivation travels on its own axis
/// (`#[derived]` / `x-semio-derived`), never as a state class: a derived field is not state at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateClass {
    Artifact,
    Config,
    Presence,
    Transient,
}

/// 🌱️ Hand-written, not derived — same DAG reason `UndoPolicy`'s hand-written twin above
/// documents. A fieldless enum with no `#[serde(...)]` tag attribute serializes as its bare
/// variant name string; mirrored here exactly.
impl crate::value::ToValue for StateClass {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(match self { StateClass::Artifact => "Artifact", StateClass::Config => "Config", StateClass::Presence => "Presence", StateClass::Transient => "Transient" }.to_string())
    }
}
impl crate::value::FromValue for StateClass {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(s) => match s.as_str() {
                "Artifact" => Ok(StateClass::Artifact),
                "Config" => Ok(StateClass::Config),
                "Presence" => Ok(StateClass::Presence),
                "Transient" => Ok(StateClass::Transient),
                other => Err(crate::value::ValueError::new(format!("unknown StateClass variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
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

/// 🧬️ Registered descriptor for one artifact's four schema facets.
#[derive(Clone, Debug)]
pub struct KernelArtifactSchemaDescriptor {
    pub id: &'static str,
    pub artifact: KernelFacetLeaves,
    pub snapshot: KernelFacetLeaves,
    pub diff: KernelFacetLeaves,
    pub mutations: KernelFacetLeaves,
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
    kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock").by_id.insert(descriptor.id, descriptor);
}

/// 🔎 Whether `id` is registered in the OS-wide artifact schema catalog.
pub fn kernel_artifact_schema_descriptor_registered(id: &str) -> bool {
    kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock").by_id.contains_key(id)
}

/// 🔢 Count of registered artifact schema ids.
pub fn kernel_artifact_schema_catalog_len() -> usize {
    kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock").by_id.len()
}

/// 📚 Invokes `visit` with every registered kernel descriptor.
pub fn with_kernel_artifact_schema_catalog<R>(visit: impl FnOnce(&[KernelArtifactSchemaDescriptor]) -> R) -> R {
    let guard = kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock");
    let mut entries: Vec<KernelArtifactSchemaDescriptor> = guard.by_id.values().cloned().collect();
    entries.sort_by_key(|entry| entry.id);
    visit(&entries)
}
//#endregion 🔖️ArtifactSchemaCatalog

//#region 🔖️ArtifactInferenceCatalog
/// 💡️ Registered descriptor for one artifact's 💡️inference schema facet — a SIBLING registry to
/// [`KernelArtifactSchemaDescriptor`], not a field on it: the four-facet descriptor already has ~107
/// handcrafted call sites across every migrated artifact, and none of them need to change as
/// artifacts adopt inference one at a time (seed-then-shrink fan-out, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). `id` is the
/// inference schema's own id (`"{artifact_id}.inference"`).await, matching how snapshot/diff already key
/// their GraphQL SDL catalog entries off `{id}.snapshot`/`{id}.diff`.
#[derive(Clone, Debug)]
pub struct KernelArtifactInferenceDescriptor {
    pub id: &'static str,
    pub inference: KernelFacetLeaves,
}

struct KernelArtifactInferenceCatalog {
    by_id: HashMap<&'static str, KernelArtifactInferenceDescriptor>,
}

static KERNEL_ARTIFACT_INFERENCE_CATALOG: OnceLock<Mutex<KernelArtifactInferenceCatalog>> = OnceLock::new();

fn kernel_artifact_inference_catalog() -> &'static Mutex<KernelArtifactInferenceCatalog> {
    KERNEL_ARTIFACT_INFERENCE_CATALOG.get_or_init(|| Mutex::new(KernelArtifactInferenceCatalog { by_id: HashMap::new() }))
}

/// 📎 Registers one artifact's handcrafted inference descriptor into the OS-wide catalog.
pub fn register_kernel_artifact_inference_descriptor(descriptor: KernelArtifactInferenceDescriptor) {
    kernel_artifact_inference_catalog().lock().expect("kernel artifact inference catalog lock").by_id.insert(descriptor.id, descriptor);
}

/// 🔎 Whether `id` (the inference schema id, `"{artifact_id}.inference"`) is registered.
pub fn kernel_artifact_inference_descriptor_registered(id: &str) -> bool {
    kernel_artifact_inference_catalog().lock().expect("kernel artifact inference catalog lock").by_id.contains_key(id)
}

/// 🔢 Count of registered artifact inference facets.
pub fn kernel_artifact_inference_catalog_len() -> usize {
    kernel_artifact_inference_catalog().lock().expect("kernel artifact inference catalog lock").by_id.len()
}

/// 📚 Invokes `visit` with every registered kernel inference descriptor.
pub fn with_kernel_artifact_inference_catalog<R>(visit: impl FnOnce(&[KernelArtifactInferenceDescriptor]) -> R) -> R {
    let guard = kernel_artifact_inference_catalog().lock().expect("kernel artifact inference catalog lock");
    let mut entries: Vec<KernelArtifactInferenceDescriptor> = guard.by_id.values().cloned().collect();
    entries.sort_by_key(|entry| entry.id);
    visit(&entries)
}
//#endregion 🔖️ArtifactInferenceCatalog

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
    kernel_app_schema_catalog().lock().expect("kernel app schema catalog lock").by_id.insert(descriptor.id, descriptor);
}

/// 🔎 Whether `id` is registered in the OS-wide app schema catalog.
pub fn kernel_app_schema_descriptor_registered(id: &str) -> bool {
    kernel_app_schema_catalog().lock().expect("kernel app schema catalog lock").by_id.contains_key(id)
}

/// 🔢 Count of registered app schema owner ids.
pub fn kernel_app_schema_catalog_len() -> usize {
    kernel_app_schema_catalog().lock().expect("kernel app schema catalog lock").by_id.len()
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
/// frame bodies (W5) — hand-rolled, not `crate::codec::value::encode_record_body`, because the TS twin on
/// the other end of the wire must reproduce these bytes exactly and has no pack engine to port;
/// these primitives are simple enough to hand-implement identically in both languages. All
/// integers are varint (documents stay `< 2^53`, safe for JS numbers); `Vec<u8>` and `[u8; 32]`
/// are the only fixed-width forms. Field order within a record is always declaration order — no
/// tags, mirroring `os_dsl::op_rt`'s "position is the format" convention one level down.
pub fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
    crate::codec::write_varint_u64(out, value);
}

pub fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, ProtocolError> {
    Ok(crate::codec::read_varint_u64(bytes, pos)?)
}

/// @emoji 1️⃣ Single raw byte, no length prefix (fixed width) — the `read_*` twin every
/// `write_*` primitive above expects but this file never gave a single-byte field, forcing
/// hand-rolled `bytes.get(pos)` + manual cursor bumps at call sites instead.
pub fn read_u8(bytes: &[u8], pos: &mut usize) -> Result<u8, ProtocolError> {
    let byte = *bytes.get(*pos).ok_or(ProtocolError::Malformed { what: "wire u8", offset: *pos as u64, detail: "truncated".to_string() })?;
    *pos += 1;
    Ok(byte)
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
