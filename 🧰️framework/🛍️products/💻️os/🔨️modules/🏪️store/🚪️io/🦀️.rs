//! 🏪️ OS snapshot receiving capabilities, document wire framing and erased application codecs.
//! Neutral snapshot representation contracts are owned by framework IO and packaged by Pack.
#[path = "💾️binary/🪪️operation-identity/🦀️.rs"]
pub mod operation_identity;
#[path = "📝️text/🦀️.rs"]
pub mod text_format;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::ValueError;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::VcsError;
use semio_framework_diagnostic::TextError;
use semio_framework_os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions};
use semio_framework_os_pack::pack_rt;
use semio_framework_dsl_record::{RecordSpecProducer,BorrowedRecordSpecProducer};
use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_os_store_backbone::local_storage_backbone_key;
use crate::text_format::ArtifactTextFiles;
#[doc(hidden)]
pub use semio_framework_diagnostic as __diagnostic;
#[doc(hidden)]
pub use semio_framework_pack_json as __pack_json;
#[doc(hidden)]
pub use semio_framework_value as __value;
#[doc(hidden)]
pub use semio_framework_os_pack as os_pack;
pub use semio_framework_os_pack::PackError;
pub use semio_framework_os_spr::{OpBinary, OpText, ProtocolError};

/// 🏪️ Explicit OS application capabilities authored independently of snapshot encoding.
pub trait ArtifactStoreSnapshot: pack::snapshot::ArtifactPack {
    /// 📣️ The actual native factory may declare one exact zero-touch registry coordinate.
    fn native_snapshot_registration() -> Option<(semio_framework_artifact_reference::Dialect, ArtifactCodec)> { None }

    /// 🪶️ Optional relational snapshot capability declared by this snapshot owner.
    fn sqlite_snapshot_codec() -> Option<ArtifactSqliteSnapshotCodec> {
        None
    }

}

use pack::snapshot::{ArtifactDsl,ArtifactPack};

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
    semio_framework_os_pack::write_varint_u64(&mut out, pack.len() as u64);
    out.extend_from_slice(pack);
    out.extend_from_slice(spr);
    out
}

/// 🔌️ Inverse of `encode_document_pack_bytes`.
pub async fn decode_document_pack_bytes(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), VcsError> {
    let mut pos = 0usize;
    let pack_len = semio_framework_os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| VcsError::Deserialize(error.to_string()))? as usize;
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



/// 📚️ Registers distinct raw-payload and complete captured-snapshot SQLite capabilities.
pub type ArtifactSqliteSnapshotExportFile=fn(&semio_framework_artifact_reference::ArtifactDialect,&semio_framework_os_store_read::ErasedSnapshotRead,semio_framework_io_sqlite_snapshot::SnapshotEncoding,&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,&mut NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_io_schema::IoResult<Vec<u8>>;
/// 🪆️ Receives a complete snapshot into an exact empty caller slot without creating erased backing.
pub type ArtifactSqliteSnapshotImportFile=fn(&semio_framework_artifact_reference::ArtifactDialect,&[u8],&mut(dyn std::any::Any+Send),&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,&mut NativeSnapshotEncodeOwner<'_,'_>)->semio_framework_io_schema::IoResult<()>;

/// 🔌️ An artifact's explicit complete snapshot files and native payload relational bridge.
#[derive(Clone)]
pub struct ArtifactSqliteSnapshotCodec {
    pub schema: std::borrow::Cow<'static, str>,
    pub snapshot_type: Option<std::any::TypeId>,
    pub export: fn(&str, &semio_framework_artifact_reference::ArtifactDialect, &semio_framework_io_schema::IoPayload, &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>, &mut NativeSnapshotDecodeOwner<'_, '_>) -> semio_framework_io_schema::IoResult<semio_framework_io_sqlite_snapshot::SqliteDatabase>,
    pub import:
        fn(&str, &semio_framework_artifact_reference::ArtifactDialect, &mut Option<semio_framework_io_sqlite_snapshot::SqliteDatabase>, semio_framework_io_sqlite_snapshot::SnapshotEncoding, &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>, &mut NativeSnapshotEncodeOwner<'_, '_>) -> semio_framework_io_schema::IoResult<semio_framework_io_schema::IoPayload>,
    pub export_file:Option<ArtifactSqliteSnapshotExportFile>,
    pub import_file:Option<ArtifactSqliteSnapshotImportFile>,
}

impl ArtifactSqliteSnapshotCodec {
    /// 🧬️ Compares the explicit schema and concrete native conversion ownership.
    pub fn identical_to(&self, other: &Self) -> bool {
        self.schema == other.schema && self.snapshot_type == other.snapshot_type && std::ptr::fn_addr_eq(self.export, other.export) && std::ptr::fn_addr_eq(self.import, other.import)&&match(self.export_file,other.export_file){(Some(left),Some(right))=>std::ptr::fn_addr_eq(left,right),(None,None)=>true,_=>false}&&match(self.import_file,other.import_file){(Some(left),Some(right))=>std::ptr::fn_addr_eq(left,right),(None,None)=>true,_=>false}
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
            Self::Guest { plugin_id, schema: owned_schema, .. } => !plugin_id.is_empty() && owned_schema == schema && !schema.is_empty() && provider.is_none_or(|provider| provider.snapshot_type.is_none()&&provider.export_file.is_none()&&provider.import_file.is_none()),
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
    pub edit_text_from_envelope: for<'a> fn(&'a semio_framework_os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>>,
    /// Host-authoritative Emit apply: (pack, spr, encode_ops_vec) -> (pack, spr, ops text).
    /// 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub apply_ops_binary: for<'a, 'b> fn(&'a [u8], &'a [u8], &'a [u8], &'a mut EntityIdentityAuthority<'b>) -> ArtifactCodecApplyFuture<'a>,
    /// 📜️ Hub Check In fold: `(pack, spr, encode_envelopes) -> (pack, spr, ops text)` through the
    /// replica gate, see [`replay_envelopes_onto_pair`].
    /// 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub replay_envelopes: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecApplyFuture<'a>,
}

#[path="💾️binary/📸️snapshot/🦀️.rs"]
mod binary_snapshot;
use semio_framework_os_store_space_schema::binary_mutations;
pub use semio_framework_os_store_space_history::lifted_serialization::{ArtifactSqliteSnapshotHost};
#[path="📝️text/📸️snapshot/🦀️.rs"]
mod text_snapshot;
#[path="📝️text/🧬️mutations/🦀️.rs"]
mod text_mutations;
#[path="📝️text/🧬️mutations/🎚️fields/🦀️.rs"]
mod text_fields;
#[path="💾️binary/🧬️mutations/🎚️fields/🦀️.rs"]
mod binary_fields;
