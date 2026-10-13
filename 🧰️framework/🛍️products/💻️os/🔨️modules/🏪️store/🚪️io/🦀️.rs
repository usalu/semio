//! 🚪️ The serialization seam every artifact implements: `ArtifactDsl`, `ArtifactPack`, their file bundles, the native and
//! SQLite snapshot capabilities and the erased `ArtifactCodec` table. The table's generic constructors stay with the engine
//! (they instantiate it), so this vocabulary never names the engine.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use semio_framework_value::ValueError;
use crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use crate::os_vcs::VcsError;
use semio_framework_diagnostic::TextError;
use crate::os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError};
use crate::os_pack::pack_rt;
use semio_framework_dsl_record::{RecordSpecProducer,BorrowedRecordSpecProducer};
use crate::io::control::{NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner};
#[cfg(target_arch = "wasm32")]
use super::backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use super::backbone::local_storage_backbone_key;
use super::ArtifactTextFiles;

/// 📜️ Handcrafted textual representation of a document snapshot, implemented once per
/// technology next to its `Snapshot` type. LAW: `P::parse_dsl(&snapshot.print_dsl())` recovers
/// an equal snapshot — canonical `print_dsl` output is always a `parse_dsl` fixpoint; hand-written
/// text may normalize (whitespace, ordering) before reaching that fixpoint.
pub trait ArtifactDsl: Sized {
    /// 🏷️ Legacy single-segment suffix used by fixture paths and codecs.
    const EXTENSION: &'static str;
    fn parse_dsl(text: &str) -> Result<Self, TextError>;
    fn print_dsl(&self) -> String;
    /// 🪪️ Dotted `plugin.artifact` identity for `.semio` preambles and on-disk names.
    fn envelope_id() -> &'static str {
        Self::EXTENSION
    }
}

/// 📦️ Binary counterpart to `ArtifactDsl` — same shape, opposite face. LAW: `P::decode_pack(
/// &p.encode_pack())` recovers an equal `p`, AND (structurally, not just by test) `decode_pack(
/// encode_pack(p)) == parse_dsl(print_dsl(p))` — dsl and pack are two encodings of the identical
/// `(RecordSpec, RecordValue)` pair keyed by the same stable `u16` field ids `dsl_derive` assigns,
/// never two independent sources of truth. The `_with` methods are required (the seam
/// `dsl_derive`'s generated impl calls through `::semio_framework_os_kernel::os_pack::pack_rt`); the plain names are provided
/// defaults over `Pack{Encode,Decode}Options::default()`.
pub trait ArtifactPack: Sized {
    /// 📣️ The actual native factory may declare one exact zero-touch registry coordinate.
    fn native_snapshot_registration() -> Option<(semio_framework_artifact_reference::Dialect, ArtifactCodec)> { None }

    /// 🪶️ Optional relational snapshot capability declared by this snapshot owner.
    fn sqlite_snapshot_codec() -> Option<ArtifactSqliteSnapshotCodec> {
        None
    }

    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError>;
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError>;

    /// 📦️ `encode_pack_with` at default options — infallible in practice (mirrors
    /// `ArtifactDsl::print_dsl`'s infallible signature); panics only on a `PackLimits` overflow.
    fn encode_pack(&self) -> Vec<u8> {
        self.encode_pack_with(&PackEncodeOptions::default()).expect("default-options pack encode is infallible")
    }

    /// 📦️ `decode_pack_with` at default (Standard) verification.
    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        Self::decode_pack_with(bytes, &PackDecodeOptions::default())
    }

    /// 🧬️ This document kind's structural field spec, for `ArtifactCodec::pack_schema_hash`
    /// (W5.7's semio_hub schema-hash validation — see that field's doc). Default `None` for hand-written
    /// `ArtifactPack` impls with no `RecordSpec` (schema-erased or synthetic fixture types, e.g.
    /// `serde_json::Value` above): those document kinds simply opt out (a zero hash reads as
    /// "schema-agnostic" everywhere this is consumed). `#[derive(crate::os_dsl::DslArtifact)]` overrides this
    /// with the real generated `__dsl_spec()`, giving every derive-based app kind (the overwhelming
    /// majority) a genuine structural fingerprint with zero manual per-app wiring.
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        None
    }

    /// 🏭️ Exposes only an authentically authored native metadata constructor.
    fn record_spec_producer() -> Option<RecordSpecProducer> { None }

    /// 🫳️ Exposes an immutable authored declaration without creating schema owners.
    fn borrowed_record_spec_producer() -> Option<BorrowedRecordSpecProducer> { None }
}

/// 📦️ Binary counterpart to `ArtifactTextFiles`. `pack` (whole `.spk` container bytes) and
/// `spr` (whole `.spr` op-log bytes, carrying real `inverse`/binary op payloads/cursor — see
/// `print_document_spr`) are AUTHORITATIVE; `ops` stays the op-log TEXT as a human-readable mirror
/// only (format-invariant across text/pack/spr, but forwards-only — see `print_ops_log`'s doc).
#[derive(Clone, Debug, Default, PartialEq,semio_framework_value::RetireOwned)]
pub struct ArtifactPackFiles {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
    pub ops: String,
}

/// 🔌️ Wire codec for the authoritative half of `ArtifactPackFiles` (`pack` + `spr`; `ops` is
/// a derived text mirror, not carried — `parse_document_pack` never reads it) — one length-prefixed
/// `pack` blob followed by the remaining bytes as `spr`. Used wherever a single binary blob must
/// stand in for a whole document (media document wire, WIT `list<u8>` document hops).
pub async fn encode_document_pack_bytes(pack: &[u8], spr: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::os_pack::write_varint_u64(&mut out, pack.len() as u64);
    out.extend_from_slice(pack);
    out.extend_from_slice(spr);
    out
}

/// 🔌️ Inverse of `encode_document_pack_bytes`.
pub async fn decode_document_pack_bytes(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), VcsError> {
    let mut pos = 0usize;
    let pack_len = crate::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| VcsError::Deserialize(error.to_string()))? as usize;
    let pack_end = pos.checked_add(pack_len).ok_or_else(|| VcsError::Deserialize("document pack bytes overflow".to_string()))?;
    if pack_end > bytes.len() {
        return Err(VcsError::Deserialize("document pack bytes truncated".to_string()));
    }
    Ok((bytes[pos..pack_end].to_vec(), bytes[pack_end..].to_vec()))
}

/// 📍️ Retains the actual positioned Text refusal at the artifact Pack boundary.
pub fn text_error_to_pack_error(error:TextError)->PackError{PackError::from(error)}

// P6: dsl::op_rt deleted. Handcrafted OpBinary impls use pack_rt record-body helpers; see OP_BINARY_FORMAT.

/// 🗂️ Type-erased document codec — the bridge a schema-string-keyed caller (chiefly
/// `framework/sync`'s `FolderEndpoint`) uses to print/parse pack+ops without naming the concrete
/// `P`/`Mutation` types at that layer. Built once per document kind via `artifact_codec`
/// (wrapped one line per app by `register_document_codec_for_app` in `framework/plugin/rs/lib.rs`,
/// wave 2) and looked up by `schema` string through `register_document_codec`/`document_codec`.
#[cfg(not(target_arch = "wasm32"))]
pub type ArtifactCodecApplyFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<(Vec<u8>, Vec<u8>, String), VcsError>> + Send + 'a>>;

#[cfg(target_arch = "wasm32")]
pub type ArtifactCodecApplyFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<(Vec<u8>, Vec<u8>, String), VcsError>> + 'a>>;

/// 🪶️ A snapshot's handwritten relational model, independently interpretable through SQLite.
pub trait ArtifactSqliteSnapshot: Sized {
    const SQLITE_SCHEMA: &'static str;
    fn to_sqlite_database(&self, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<crate::sqlite_snapshot::SqliteDatabase, ValueError>;
    fn from_sqlite_database(database: &crate::sqlite_snapshot::SqliteDatabase, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, ValueError>;

    /// 🫴️ Receives original SQL row ownership before any projection effect can refuse; the default runs the authored projection under the original owner, whose control carries the operation workspace recipient the codec installed.
    fn to_sqlite_database_receiving(&self,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<crate::sqlite_snapshot::SqliteDatabase,ValueError>{super::sqlite_snapshot_workspace::project(self,control,owner)}
    /// 🪆️ Receives original semantic tree ownership before any reconstruction effect can refuse; the default runs the authored reconstruction under the original owner, whose control carries the operation workspace recipient the codec installed.
    fn from_sqlite_database_receiving(database:&crate::sqlite_snapshot::SqliteDatabase,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError> {super::sqlite_snapshot_workspace::reconstruct(database,control,owner)}

    /// ♻️ Retires a snapshot through its declared owner after an erased conversion.
    fn retire_sqlite_snapshot(self) {
        drop(self);
    }

    /// 🛬️ Materializes native snapshot fields under the caller's limits and cancellation control.
    fn decode_sqlite_snapshot_native(_payload: &crate::io_schema::IoPayload, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        native_owner.native().checkpoint()?;
        control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative, 0, 1)?;
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "snapshot owner has no controlled native decoding implementation"))
    }

    /// 🛫️ Emits native state through an owner-declared bounded and cancellable encoder.
    fn encode_sqlite_snapshot_native(&self, _encoding: crate::sqlite_snapshot::SnapshotEncoding, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<crate::io_schema::IoPayload, ValueError> {
        native_owner.native().checkpoint()?;
        control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative, 0, 1)?;
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "snapshot owner has no controlled native encoding implementation"))
    }

    /// 🚧️ Checks the owner's native encoding expansion before printing or packing any fields.
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: crate::sqlite_snapshot::SnapshotEncoding, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative, 0, 1)?;
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "snapshot owner has no bounded native encoding preflight"))
    }

    /// 🛡️ Validates the exact owned subset without lowering fields into a native wire format.
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _database: &crate::sqlite_snapshot::SqliteDatabase, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<()> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(crate::io_schema::IoError::from_value_error)?;
        if dialect.subset == "*" {
            return Ok(crate::io_schema::IoOutcome::clean(()));
        }
        Err(crate::io_schema::IoError::from_value_error(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("owned snapshot subset {} has no semantic validator", dialect.to_coordinate()))))
    }

    /// 🛂️ Validates the authoritative subset with original decode receiving authority; the default applies the authored subset policy.
    fn validate_sqlite_snapshot_subset_decoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&crate::sqlite_snapshot::SqliteDatabase,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotDecodeOwner<'_,'_>)->crate::io_schema::IoResult<()>{super::sqlite_snapshot_workspace::validate_decoding(self,dialect,database,control,native)}
    /// 🛃️ Validates the authoritative subset with original encode receiving authority; the default applies the authored subset policy.
    fn validate_sqlite_snapshot_subset_encoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&crate::sqlite_snapshot::SqliteDatabase,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotEncodeOwner<'_,'_>)->crate::io_schema::IoResult<()>{super::sqlite_snapshot_workspace::validate_encoding(self,dialect,database,control,native)}

    /// 🪶️ Builds the relational codec only when the snapshot actually implements it.
    fn sqlite_codec() -> ArtifactSqliteSnapshotCodec
    where
        Self: ArtifactDsl + ArtifactPack + semio_framework_value::retirement::RetireOwned,
    {
        fn export_snapshot_impl<P: ArtifactDsl + ArtifactPack + ArtifactSqliteSnapshot + semio_framework_value::retirement::RetireOwned>(schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,payload:&crate::io_schema::IoPayload,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotDecodeOwner<'_,'_>)->crate::io_schema::IoResult<crate::sqlite_snapshot::SqliteDatabase>{super::sqlite_snapshot_retirement::export::<P>(schema,dialect,payload,control,native)}
        fn import_snapshot_impl<P: ArtifactDsl + ArtifactPack + ArtifactSqliteSnapshot + semio_framework_value::retirement::RetireOwned>(schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,input:&mut Option<crate::sqlite_snapshot::SqliteDatabase>,encoding:crate::sqlite_snapshot::SnapshotEncoding,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotEncodeOwner<'_,'_>)->crate::io_schema::IoResult<crate::io_schema::IoPayload>{super::sqlite_snapshot_retirement::import::<P>(schema,dialect,input,encoding,control,native)}

        ArtifactSqliteSnapshotCodec { schema: std::borrow::Cow::Borrowed(Self::SQLITE_SCHEMA), snapshot_type: Some(std::any::TypeId::of::<Self>()), export: export_snapshot_impl::<Self>, import: import_snapshot_impl::<Self> }
    }
}

/// 🔌️ Erased native payload bridge to an artifact's explicit relational SQLite implementation.
#[derive(Clone)]
pub struct ArtifactSqliteSnapshotCodec {
    pub schema: std::borrow::Cow<'static, str>,
    pub snapshot_type: Option<std::any::TypeId>,
    pub export: fn(&str, &semio_framework_artifact_reference::ArtifactDialect, &crate::io_schema::IoPayload, &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>, &mut NativeSnapshotDecodeOwner<'_, '_>) -> crate::io_schema::IoResult<crate::sqlite_snapshot::SqliteDatabase>,
    pub import:
        fn(&str, &semio_framework_artifact_reference::ArtifactDialect, &mut Option<crate::sqlite_snapshot::SqliteDatabase>, crate::sqlite_snapshot::SnapshotEncoding, &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>, &mut NativeSnapshotEncodeOwner<'_, '_>) -> crate::io_schema::IoResult<crate::io_schema::IoPayload>,
}

impl ArtifactSqliteSnapshotCodec {
    /// 🧬️ Compares the explicit schema and concrete native conversion ownership.
    pub fn identical_to(&self, other: &Self) -> bool {
        self.schema == other.schema && self.snapshot_type == other.snapshot_type && std::ptr::fn_addr_eq(self.export, other.export) && std::ptr::fn_addr_eq(self.import, other.import)
    }
}

/// 🪪️ Retains actual native ownership independently from optional relational capability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactNativeSnapshotIdentity {
    Typed { snapshot_type: std::any::TypeId, owner: &'static str },
    Guest { plugin_id: String, package_hash: [u8; 32], schema: String },
}

impl ArtifactNativeSnapshotIdentity {
    /// 🧬️ Binds the concrete owner and its stable native type label together.
    pub fn typed<P: 'static>() -> Self { Self::Typed { snapshot_type: std::any::TypeId::of::<P>(), owner: std::any::type_name::<P>() } }
    /// 🔎️ Resolves a genuine linked owner without assigning a host type to a guest.
    pub fn snapshot_type(&self) -> Option<std::any::TypeId> { match self { Self::Typed { snapshot_type, .. } => Some(*snapshot_type), Self::Guest { .. } => None } }
    /// 🛂️ Checks independent native provenance against any declared relational provider.
    pub fn validates_provider(&self, schema: &str, provider: Option<&ArtifactSqliteSnapshotCodec>) -> bool {
        match self {
            Self::Typed { snapshot_type, owner } => !owner.is_empty() && provider.is_none_or(|provider| provider.snapshot_type == Some(*snapshot_type)),
            Self::Guest { plugin_id, schema: owned_schema, .. } => !plugin_id.is_empty() && owned_schema == schema && !schema.is_empty() && provider.is_none_or(|provider| provider.snapshot_type.is_none()),
        }
    }
}

#[derive(Clone)]
pub struct ArtifactCodec {
    pub schema: String,
    pub extension: &'static str,
    pub native_identity: ArtifactNativeSnapshotIdentity,
    /// 🏛️ Handcrafted semantic SQLite schema and native snapshot conversion provider.
    pub snapshot_sqlite: Option<ArtifactSqliteSnapshotCodec>,
    /// 🧬️ W5.7: a structural fingerprint of this document kind's field shape —
    /// `crate::os_pack::schema_hash(&spec)` over `P::record_spec()`, or `[0u8; 32]` when `P` has no
    /// `RecordSpec` (hand-written `ArtifactPack` impls, see that trait method's doc). Hub actors
    /// send this in `ClientFrame::SocketHelloV1`; the semio_hub validates it against the durable
    /// `(space, document)` scope and rejects a later mismatching one before `Welcome` — a zero
    /// hash always skips validation (schema-agnostic client). Durable pinning belongs in the db
    /// catalog once it grows a column for it; this in-memory pin is this wave's scope.
    pub pack_schema_hash: [u8; 32],
    /// 📤️ `(dsl text, ops text) -> (pack files, dsl mirror text)` — the hand-authored/
    /// imported fallback path: compiles text straight to binary pack+spr (no JSON envelope
    /// currency anywhere in between). Returns the re-printed canonical dsl mirror alongside the
    /// pack files so a caller can write all four files (`.pack`/`.spr`/`.dsl`/`.ops`) in one shot.
    /// 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii): ComposeFuture/IoFuture-shaped) — an
    /// `async fn` item's pointer type is unnameable, so the table stores a plain `fn` that itself
    /// returns a boxed future.
    pub compile_dsl: for<'a> fn(&'a str, &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + Send + 'a>>,
    /// 📥️ `(pack bytes, spr bytes) -> (dsl text, ops text)` — the sanctioned human/agent
    /// LOGGING mirror, produced from the authoritative binary for schema-agnostic callers
    /// (`store_sync`'s `FolderEndpoint::Pack` write path) that never touch a concrete `P`/`Mutation`.
    /// 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub print_mirror: for<'a> fn(&'a [u8], &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + Send + 'a>>,
    /// 🧩️ One `MutationEnvelope` -> one complete {@link print_edit_lines} append unit (edit
    /// header, indented op line, inverse record, one metadata record), for `FolderTextStorage::append_ops`'s hot-path logging append — decodes the
    /// envelope's opaque `OpBinary` payload back into a concrete `Mutation` just long enough to
    /// print it, for schema-agnostic callers that otherwise never see a concrete op type.
    /// 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub edit_text_from_envelope: for<'a> fn(&'a crate::os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>>,
    /// Host-authoritative Emit apply: (pack, spr, encode_ops_vec) -> (pack, spr, ops text).
    /// 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub apply_ops_binary: for<'a, 'b> fn(&'a [u8], &'a [u8], &'a [u8], &'a mut EntityIdentityAuthority<'b>) -> ArtifactCodecApplyFuture<'a>,
    /// 📜️ Hub Check In fold: `(pack, spr, encode_envelopes) -> (pack, spr, ops text)` through the
    /// replica gate, see [`replay_envelopes_onto_pair`].
    /// 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub replay_envelopes: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecApplyFuture<'a>,
}
