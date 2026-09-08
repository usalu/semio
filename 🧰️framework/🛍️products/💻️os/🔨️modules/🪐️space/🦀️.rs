//! 🪐️ OS host orchestration for space drafts and collection archives.

pub use semio_framework_artifact_space_collection::{
    artifact_backbone_uri, collection_backbone_uri, collection_package_from_schema, empty_collection_snapshot, entry_path, folder_path,
    package_descriptor as collection_package_descriptor, reconcile_collection_integrity, resolve_entry_by_path, ArtifactBody, CollectionArtifactPackage,
    CollectionDiff, CollectionEntry, CollectionFolder, CollectionMutation, CollectionPackageSchemaError, CollectionSnapshot, MovedToContainer, RenamedItem,
    ReplacedEntryBody, COLLECTION_ARTIFACT_DEFINITION_SCHEMA, S_COLLECTION_SCHEMA,
};
pub use semio_framework_artifact_space_space::{
    can_write, empty_space_snapshot, package_descriptor as space_package_descriptor, reconcile_space_atelier_invariant, space_backbone_uri,
    space_package_from_schema, space_role_of, CollectionRef, InstalledExtension, SpaceArtifactPackage, SpaceDiff, SpaceKind, SpaceMutation,
    SpacePackageSchemaError, SpaceRole, SpaceSnapshot, SpaceUser, SpaceVisibility, SPACE_ARTIFACT_DEFINITION_SCHEMA, S_SPACE_SCHEMA,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use std::io::{Cursor, Read as _, Seek, Write as _};
use std::sync::{Arc, LazyLock, Mutex};

//#region 🔖️Drafts
/// 🗄️ Every draft artifact lives at `temp://draft/<id>` while it's a draft — volatility is placement,
/// not an envelope flag (see `## Draft vs asset` in the plan).
pub const DRAFT_URI_PREFIX: &str = "temp://draft/";

/// 🔗️ `temp://draft/<artifact_id>`.
pub fn draft_uri(artifact_id: &str) -> String {
    format!("{DRAFT_URI_PREFIX}{artifact_id}")
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpaceError {
    UnknownDraft(String),
    Backbone(String),
}

impl std::fmt::Display for SpaceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownDraft(id) => write!(formatter, "unknown draft: {id}"),
            Self::Backbone(detail) => write!(formatter, "draft backbone error: {detail}"),
        }
    }
}

impl std::error::Error for SpaceError {}

//#region 🔖️DraftBackbone
/// 🔌️ Byte-oriented backbone port for draft<->asset envelope relocation — mirrors os-core's
/// `OsBackbonePort`/blanket-bridge pattern exactly (`🧰️framework/🛍️products/💻️os`'s `host::OsBackbonePort`),
/// but declared HERE rather than reused from there: os-core depends on `space` (`pub use space::*;`),
/// so depending back on os-core's trait would cycle the dependency graph. Every real transport this
/// crate needs (`store::MemoryBackbonePort`, `store::LocalStorageBackbonePort`, the host file/folder
/// ports) is already `store::BackbonePort`-shaped (string payloads) — the blanket impl below bridges
/// bytes<->base64 text exactly like os-core's own bridge, so any `Arc<dyn store::BackbonePort>`-backed
/// concrete port a caller already holds satisfies `Arc<store::BackbonePorts>` for free, and (crucially
/// for `draft_catalog_for`'s per-port keying below) preserves the SAME underlying `Arc` data pointer
/// across both trait-object views since unsizing coercion never reallocates.
pub trait SpaceBackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<Vec<u8>, vcs::VcsError>;
    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), vcs::VcsError>;
}

/// 🌉️ `T: Send + Sync` is added here on the blanket impl itself, not on `store::BackbonePort`'s own
/// supertraits (that trait deliberately omits them — R7) — every real implementor already satisfies
/// it, so this only narrows the blanket, never the port trait.
impl<T: store::BackbonePort + Send + Sync> SpaceBackbonePort for T {
    fn read(&self, uri: &str) -> Result<Vec<u8>, vcs::VcsError> {
        // 🌉️ `store::BackbonePort::read`/`write` turned `async fn` under the runtime-dependency
        // sweep; `crate::host::resolve_kernel_future` (`🖥️host/🦀️.rs`) resolves them
        // synchronously here, same justification as its own doc comment: every real backbone port
        // this crate wraps (`store::MemoryBackbonePort`/`store::LocalStorageBackbonePort`, the host
        // file/folder ports) is an in-memory/local lookup, never a park-worthy network await.
        let text = crate::host::resolve_kernel_future(store::BackbonePort::read(self, uri))?;
        if text.is_empty() {
            return Ok(Vec::new());
        }
        base64_codec::base64_standard_decode(text).map_err(|error| vcs::VcsError::Deserialize(error.to_string()))
    }

    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), vcs::VcsError> {
        if payload.is_empty() {
            return crate::host::resolve_kernel_future(store::BackbonePort::write(self, uri, ""));
        }
        crate::host::resolve_kernel_future(store::BackbonePort::write(self, uri, &base64_codec::base64_standard_encode(payload)))
    }
}
//#endregion 🔖️DraftBackbone

/// 📄️ Bookkeeping for one draft artifact: identity, artifact kind, document schema, display name, and
/// TTL (`expires_at_ms: None` means pinned — the plan's default TTL is 7 days, a caller policy, not a
/// constant this pure catalog hardcodes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DraftEntry {
    pub artifact_id: String,
    pub kind_id: String,
    pub schema: String,
    pub name: String,
    pub created_at_ms: u64,
    pub expires_at_ms: Option<u64>,
}

/// 🗄️ Draft bookkeeping registry (TTL sweep, promote/demote as real operation-sourced byte moves).
///
/// 🎯️ W5 Lane B: promoted from the W2/W4 stub to the real thing. `promote_draft`/`demote_asset` now
/// relocate the draft's actual envelope bytes via an injected `SpaceBackbonePort` — read at
/// `draft_uri`, written at `artifact_backbone_uri`/vice versa, byte-for-byte, no decode/re-encode —
/// while still returning the `CollectionMutation` (`CreateEntry`/`DeleteEntry`) that keeps promotion
/// itself operation-sourced. One `DraftCatalog` per distinct backbone port identity lives in the
/// port-keyed global registry below (`draft_catalog_for`), mirroring os-core's `SPACE_CATALOG_URIS`
/// per-port keying — this crate still doesn't reach into os-core's session state directly (that
/// dependency would cycle), it just now offers the SAME per-port-identity registry SHAPE os-core's
/// `list_os_space_catalog_entries`/`create_os_space` already use, so a caller (os-core, or an app like
/// `home`) wires it in by simply calling `draft_catalog_for(&port)` with whatever
/// `Arc<store::BackbonePorts>` it already holds.
#[derive(Default)]
pub struct DraftCatalog {
    drafts: Mutex<HashMap<String, DraftEntry>>,
}

impl DraftCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌱️ Mints a fresh draft id and registers its bookkeeping. `now_ms`/`ttl_ms` are caller-supplied
    /// (this crate is pure data plus pure functions — no wall-clock reads, matching `vcs`'s own doc
    /// comment convention).
    pub fn create_draft(&self, kind_id: &str, schema: &str, name: &str, now_ms: u64, ttl_ms: Option<u64>) -> DraftEntry {
        static DRAFT_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let seq = DRAFT_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // 🌉️ `vcs::content_addressed_entity_id` turned `async fn` under the runtime-dependency sweep;
        // this is a pure hash-of-bytes computation with no real suspension, so it's resolved
        // synchronously the same way `SpaceBackbonePort`'s blanket impl resolves its callee futures
        // above — this catalog stays a pure, synchronous type (see this fn's own doc: "no wall-clock
        // reads", matching `vcs`'s own convention).
        let artifact_id = crate::host::resolve_kernel_future(vcs::content_addressed_entity_id("draft", format!("{kind_id}\0{schema}\0{name}\0{now_ms}\0{seq}").as_bytes()));
        let entry = DraftEntry { artifact_id, kind_id: kind_id.into(), schema: schema.into(), name: name.into(), created_at_ms: now_ms, expires_at_ms: ttl_ms.map(|ttl| now_ms + ttl) };
        self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(entry.artifact_id.clone(), entry.clone());
        entry
    }

    pub fn list_drafts(&self) -> Vec<DraftEntry> {
        let mut entries: Vec<DraftEntry> = self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).values().cloned().collect();
        entries.sort_by(|a, b| a.artifact_id.cmp(&b.artifact_id));
        entries
    }

    /// ⏰️ Removes every draft whose `expires_at_ms` is at or before `now_ms` from the bookkeeping,
    /// best-effort tombstoning each one's `draft_uri` bytes via `port` (empty-payload write, same
    /// convention as os-core's `delete_os_space`) so an expired draft doesn't leak backbone storage —
    /// bookkeeping removal is still the source of truth (a tombstone write failure doesn't undo it).
    /// Returns the expired ids.
    pub fn expire_drafts(&self, now_ms: u64, port: &Arc<store::BackbonePorts>) -> Vec<String> {
        let expired: Vec<String> = {
            let mut drafts = self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let expired: Vec<String> = drafts.values().filter(|entry| entry.expires_at_ms.is_some_and(|expires_at| expires_at <= now_ms)).map(|entry| entry.artifact_id.clone()).collect();
            for artifact_id in &expired {
                drafts.remove(artifact_id);
            }
            expired
        };
        for artifact_id in &expired {
            let _ = port.write(&draft_uri(artifact_id), &[]);
        }
        expired
    }

    /// 📚️ `list_drafts` preceded by a real `expire_drafts` sweep — the natural "sweep before listing"
    /// call site (mirrors the spirit of os-core's catalog-listing entry points): any caller that lists
    /// drafts for display should always see a freshly-swept set rather than stale expired entries.
    pub fn list_drafts_sweeping_expired(&self, now_ms: u64, port: &Arc<store::BackbonePorts>) -> Vec<DraftEntry> {
        self.expire_drafts(now_ms, port);
        self.list_drafts()
    }

    /// 🗑️ Discards a draft outright (never promoted) — removes its bookkeeping and best-effort
    /// tombstones its `draft_uri` bytes via `port`. Returns the removed bookkeeping, if any existed.
    pub fn discard_draft(&self, port: &Arc<store::BackbonePorts>, draft_id: &str) -> Option<DraftEntry> {
        let removed = self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(draft_id);
        if removed.is_some() {
            let _ = port.write(&draft_uri(draft_id), &[]);
        }
        removed
    }

    /// ⬆️ REAL promotion: relocates the draft's envelope bytes — whatever opaque blob the caller wrote
    /// at `draft_uri(draft_id)` (a pack+spr snapshot, an `encode_backbone_payload`-framed blob, etc. —
    /// this catalog never decodes it) — to `artifact_backbone_uri(space_id, draft_id)` via `port`,
    /// byte-for-byte (no decode/re-encode anywhere in this path, so the moved bytes are IDENTICAL
    /// before and after, just at a different backbone uri — the plan's exact promotion invariant),
    /// then tombstones the draft uri. Only removes the draft bookkeeping once the byte move fully
    /// succeeds. Returns the draft bookkeeping (removed from this catalog) plus the
    /// `CollectionMutation::CreateEntry` the caller applies to their `CollectionSnapshot` — promotion
    /// stays operation-sourced even though it now really touches bytes. The artifact keeps its id
    /// (`entry.id == draft.artifact_id == document id`); a caller who knows the artifact's concrete
    /// `<P, Mutation>` pair can reconstruct a live `store::ArtifactStore<P, Mutation>` from the SAME
    /// moved bytes via `import_document_artifact` (see `🔖️ZipStoreBridge` above) and register it into
    /// their `store::SpaceHost` (`register_member`/`register_space_documents`) — this catalog stays
    /// type-erased on purpose (same reasoning as `ArtifactBody`'s hand-written `DslVariants`) and never
    /// touches `P`/`Mutation` itself, so it never calls `SpaceHost` directly.
    pub fn promote_draft(&self, port: &Arc<store::BackbonePorts>, space_id: &str, draft_id: &str, folder_id: Option<String>) -> Result<(DraftEntry, CollectionMutation), SpaceError> {
        let draft = {
            let drafts = self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            drafts.get(draft_id).cloned().ok_or_else(|| SpaceError::UnknownDraft(draft_id.to_string()))?
        };

        let source_uri = draft_uri(draft_id);
        let target_uri = artifact_backbone_uri(space_id, draft_id);
        let envelope_bytes = port.read(&source_uri).map_err(|error| SpaceError::Backbone(error.to_string()))?;
        port.write(&target_uri, &envelope_bytes).map_err(|error| SpaceError::Backbone(error.to_string()))?;
        port.write(&source_uri, &[]).map_err(|error| SpaceError::Backbone(error.to_string()))?;

        self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(draft_id);

        let entry =
            CollectionEntry { id: draft.artifact_id.clone(), folder_id, name: draft.name.clone(), kind_id: draft.kind_id.clone(), body: Box::new(ArtifactBody::Document { schema: draft.schema.clone(), document_id: draft.artifact_id.clone() }) };
        Ok((draft, CollectionMutation::CreateEntry { entry, index: u32::MAX }))
    }

    /// ⏪️ Demotion's STRUCTURAL inverse — removing the entry from the collection. `CollectionMutation`'s
    /// own `inverse()` on `CreateEntry` already produces exactly this (see `## Draft vs asset`'s
    /// "demotion is CreateEntry's natural inverse"); kept as a standalone helper since a caller building
    /// a demote flow from scratch (not undoing a specific `CreateEntry`) needs the operation without a
    /// `CreateEntry` in hand. Does NOT move bytes — see `demote_asset` for the real byte-moving version.
    pub fn demote_operation(entry_id: &str) -> CollectionMutation {
        CollectionMutation::DeleteEntry { entry_id: entry_id.to_string() }
    }

    /// ⏪️ REAL demotion: the byte-moving inverse of `promote_draft` — relocates `entry`'s envelope
    /// bytes back from `artifact_backbone_uri(space_id, entry.id)` to `draft_uri(entry.id)`
    /// byte-for-byte, tombstones the asset uri, and re-registers fresh draft bookkeeping (`now_ms`/
    /// `ttl_ms` are caller-supplied, same convention as `create_draft` — a demoted draft gets a fresh
    /// TTL window, it doesn't inherit whatever deadline it had before its original promotion). Returns
    /// the `CollectionMutation::DeleteEntry` for the caller to apply to their `CollectionSnapshot`
    /// (identical to `demote_operation`'s output — this is the byte-touching sibling of that pure
    /// helper, needed whenever a demotion must actually relocate bytes rather than just undo an
    /// in-hand `CreateEntry`).
    pub fn demote_asset(&self, port: &Arc<store::BackbonePorts>, space_id: &str, entry: &CollectionEntry, schema: &str, now_ms: u64, ttl_ms: Option<u64>) -> Result<CollectionMutation, SpaceError> {
        let source_uri = artifact_backbone_uri(space_id, &entry.id);
        let target_uri = draft_uri(&entry.id);
        let envelope_bytes = port.read(&source_uri).map_err(|error| SpaceError::Backbone(error.to_string()))?;
        port.write(&target_uri, &envelope_bytes).map_err(|error| SpaceError::Backbone(error.to_string()))?;
        port.write(&source_uri, &[]).map_err(|error| SpaceError::Backbone(error.to_string()))?;

        let draft = DraftEntry { artifact_id: entry.id.clone(), kind_id: entry.kind_id.clone(), schema: schema.into(), name: entry.name.clone(), created_at_ms: now_ms, expires_at_ms: ttl_ms.map(|ttl| now_ms + ttl) };
        self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(draft.artifact_id.clone(), draft);
        Ok(Self::demote_operation(&entry.id))
    }
}

//#region 🔖️DraftRegistry
/// 🗄️ Port-keyed global `DraftCatalog` registry — mirrors os-core's `SPACE_CATALOG_URIS`/`port_key`
/// per-port-identity keying exactly (`Arc::as_ptr` truncated to a bare data-pointer `usize`, dropping
/// the vtable so two differently-vtabled trait-object views of the SAME underlying `Arc` allocation
/// still key identically — see `SpaceBackbonePort`'s own doc). One `DraftCatalog` per distinct port
/// identity, shared by every caller holding (a clone of) that port.
static DRAFT_CATALOG_REGISTRY: LazyLock<Mutex<HashMap<usize, Arc<DraftCatalog>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn draft_catalog_port_key(port: &Arc<store::BackbonePorts>) -> usize {
    Arc::as_ptr(port) as *const () as usize
}

/// 🔎️ Gets (or lazily creates) the `DraftCatalog` for `port`'s identity. Returns a cheap `Arc` clone —
/// every caller sharing the same port shares the same draft bookkeeping, exactly the way
/// `list_os_space_catalog_entries`/`create_os_space` share `SPACE_CATALOG_URIS` per port in os-core.
pub fn draft_catalog_for(port: &Arc<store::BackbonePorts>) -> Arc<DraftCatalog> {
    DRAFT_CATALOG_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(draft_catalog_port_key(port)).or_insert_with(|| Arc::new(DraftCatalog::new())).clone()
}
//#endregion 🔖️DraftRegistry
//#endregion 🔖️Drafts

//#region 🔖️Zip
/// 📦️ `export_collection_zip`/`import_collection_zip` errors.
#[derive(Debug)]
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub enum SpaceZipError {
    Zip(zip::result::ZipError),
    Io(std::io::Error),
    Pack(String),
    MissingPath(String),
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::fmt::Display for SpaceZipError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zip(error) => write!(formatter, "zip error: {error}"),
            Self::Io(error) => write!(formatter, "io error: {error}"),
            Self::Pack(detail) => write!(formatter, "pack error: {detail}"),
            Self::MissingPath(path) => write!(formatter, "missing path for entry {path}"),
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::error::Error for SpaceZipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Zip(error) => Some(error),
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl From<zip::result::ZipError> for SpaceZipError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Zip(error)
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl From<std::io::Error> for SpaceZipError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// 📤️ Everything `import_collection_zip` recovers from a zip byte stream: the collection snapshot
/// itself (plus its own serialized history bytes), and per-entry artifact/blob bytes keyed by the
/// entry they belong to.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct ImportedCollection {
    pub collection: CollectionSnapshot,
    pub collection_spr: Vec<u8>,
    pub artifacts: Vec<(CollectionEntry, Vec<u8>, Vec<u8>)>,
    pub blobs: Vec<(store::BlobRef, Vec<u8>)>,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn zip_file_options() -> zip::write::SimpleFileOptions {
    // 🕰️ Fixed (epoch) timestamp on every entry — the export→import→export byte-stability law
    // depends on nothing time-varying leaking into the zip's central directory.
    zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).last_modified_time(zip::DateTime::default())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn write_zip_file<W: std::io::Write + Seek>(writer: &mut zip::ZipWriter<W>, name: &str, bytes: &[u8], options: zip::write::SimpleFileOptions) -> Result<(), SpaceZipError> {
    writer.start_file(name, options)?;
    writer.write_all(bytes)?;
    Ok(())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn read_zip_entry<R: std::io::Read + Seek>(archive: &mut zip::ZipArchive<R>, name: &str) -> Result<Vec<u8>, SpaceZipError> {
    let mut file = archive.by_name(name)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// 📤️ Exports a collection to a zip byte stream: `collection.collection.pack`/`.spr` at the root, each
/// document artifact at `<folder path>/<name>.pack` + `.spr` (lossless — full VCS history survives via
/// the injected `read_artifact` bytes), each blob raw at its path. IO-free: `read_artifact`/`read_blob`
/// are injected so this crate never touches a live store/filesystem itself — the caller supplies
/// already-serialized bytes from wherever they actually live (a `ArtifactEnvelope`'s pack/spr, a
/// `BlobStore`). Entries are written in id order for determinism (the byte-stability law below).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn export_collection_zip(
    collection: &CollectionSnapshot,
    collection_spr: &[u8],
    read_artifact: &dyn Fn(&str) -> Result<(Vec<u8>, Vec<u8>), SpaceZipError>,
    read_blob: &dyn Fn(&str) -> Result<Vec<u8>, SpaceZipError>,
) -> Result<Vec<u8>, SpaceZipError> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip_file_options();

    write_zip_file(&mut writer, "collection.collection.pack", &store::ArtifactPack::encode_pack(collection), options)?;
    write_zip_file(&mut writer, "collection.collection.spr", collection_spr, options)?;

    let mut entries: Vec<&CollectionEntry> = collection.entries.iter().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    for entry in entries {
        let path = entry_path(collection, &entry.id).ok_or_else(|| SpaceZipError::MissingPath(entry.id.clone()))?;
        match entry.body.as_ref() {
            ArtifactBody::Document { .. } => {
                let (pack_bytes, spr_bytes) = read_artifact(&entry.id)?;
                write_zip_file(&mut writer, &format!("{path}.pack"), &pack_bytes, options)?;
                write_zip_file(&mut writer, &format!("{path}.spr"), &spr_bytes, options)?;
            }
            ArtifactBody::Blob { blob } => {
                let bytes = read_blob(&blob.hash)?;
                write_zip_file(&mut writer, &path, &bytes, options)?;
            }
        }
    }

    let cursor = writer.finish()?;
    Ok(cursor.into_inner())
}

/// 📥️ Inverse of `export_collection_zip`. Parses `collection.collection.pack` first to learn the
/// folder tree/entries, then walks entries in the same id order to know exactly which zip paths to
/// read back — never guesses a layout from the zip's own directory listing.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn import_collection_zip(bytes: &[u8]) -> Result<ImportedCollection, SpaceZipError> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
    let collection_pack = read_zip_entry(&mut archive, "collection.collection.pack")?;
    let collection_spr = read_zip_entry(&mut archive, "collection.collection.spr")?;
    let collection = <CollectionSnapshot as store::ArtifactPack>::decode_pack(&collection_pack).map_err(|error| SpaceZipError::Pack(error.to_string()))?;

    let mut artifacts = Vec::new();
    let mut blobs = Vec::new();
    let mut entries: Vec<&CollectionEntry> = collection.entries.iter().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    for entry in entries {
        let path = entry_path(&collection, &entry.id).ok_or_else(|| SpaceZipError::MissingPath(entry.id.clone()))?;
        match entry.body.as_ref() {
            ArtifactBody::Document { .. } => {
                let pack_bytes = read_zip_entry(&mut archive, &format!("{path}.pack"))?;
                let spr_bytes = read_zip_entry(&mut archive, &format!("{path}.spr"))?;
                artifacts.push((entry.clone(), pack_bytes, spr_bytes));
            }
            ArtifactBody::Blob { blob } => {
                let raw = read_zip_entry(&mut archive, &path)?;
                blobs.push((blob.clone(), raw));
            }
        }
    }

    Ok(ImportedCollection { collection, collection_spr, artifacts, blobs })
}

//#region 🔖️ZipStoreBridge
/// 🌉️ Real (non-mock) reader/writer bridge from `export_collection_zip`/`import_collection_zip`'s
/// injected-callback shape to the actual `store` crate types (`store::ArtifactStore`/
/// `store::ArtifactPackFiles`/`store::BlobStore`). This crate depends on `store` (see this crate's
/// `Cargo.toml`), never the reverse, so the bridge lives HERE rather than inside `store` — the
/// direction that keeps the dependency graph acyclic; `store` itself stays app-agnostic and never
/// names a collection/space type. W2 shipped `export_collection_zip`/`import_collection_zip` IO-free
/// with caller-injected reader closures and only fixture-string-backed unit tests exercising them;
/// this region is what a real caller (a live `store::SpaceHost`'s registered members, W4's storage
/// wave) plugs into those closures so a real collection with real document/blob artifacts actually
/// round-trips through a real `.zip` byte stream.
///
/// 📤️ EXPORT side: a caller snapshots each open document artifact's `store::ArtifactStore<P,
/// Mutation>` via its own `snapshot_pack()` into a `document_id -> store::ArtifactPackFiles` table,
/// then hands `real_artifact_reader`/`real_blob_reader` (closures over that table and a live
/// `store::BlobStore`) straight to `export_collection_zip`.
///
/// 📥️ IMPORT side: `import_document_artifact`/`import_blob` are the inverse — reconstructing a real
/// `store::ArtifactStore<P, Mutation>` from one `ImportedCollection::artifacts` entry's pack+spr
/// bytes (generic over the artifact's own concrete schema, mirroring `store::parse_document_pack`'s
/// own genericity — this crate never knows a document artifact's concrete type, only its `schema`
/// string, so the caller supplies `P`/`Mutation` at the call site), and re-`put`-ing one imported
/// blob's bytes into a live `store::BlobStore`, verifying the freshly computed content hash still
/// matches the `store::BlobRef` recorded in the collection.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn real_artifact_reader(pack_files: &HashMap<String, store::ArtifactPackFiles>) -> impl Fn(&str) -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> + '_ {
    move |entry_id: &str| -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> {
        let files = pack_files.get(entry_id).ok_or_else(|| SpaceZipError::MissingPath(entry_id.to_string()))?;
        Ok((files.pack.clone(), files.spr.clone()))
    }
}

/// 🌉️ Generic over `B: store::BlobStore` rather than `&dyn store::BlobStore`: `BlobStore`'s methods
/// turned `async fn` under the runtime-dependency sweep, which makes the trait no longer dyn
/// compatible (native `async fn` in a trait has no fixed-size vtable-callable return type) — static
/// dispatch is the same fix already adopted by every other already-migrated `BlobStore` consumer in
/// this repo (e.g. `🏃️run/🦀️.rs`'s `SpaceRunner<H, B: BlobStore + 'static>`).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn real_blob_reader<B: store::BlobStore>(blob_store: &B) -> impl Fn(&str) -> Result<Vec<u8>, SpaceZipError> + '_ {
    move |hash: &str| -> Result<Vec<u8>, SpaceZipError> {
        // 🌉️ Resolved synchronously via `crate::host::resolve_kernel_future`, same justification as
        // `SpaceBackbonePort`'s blanket impl above — `export_collection_zip`'s injected
        // `read_blob: &dyn Fn(&str) -> Result<Vec<u8>, SpaceZipError>` callback shape stays sync.
        crate::host::resolve_kernel_future(blob_store.get(hash)).map_err(|error| SpaceZipError::Pack(error.to_string()))?.ok_or_else(|| SpaceZipError::MissingPath(hash.to_string()))
    }
}

/// 📥️ Reconstructs a real `store::ArtifactStore<P, Mutation>` from one imported document artifact's
/// pack+spr bytes — the import-side counterpart to snapshotting it for `real_artifact_reader`.
/// 🌉️ `async fn` and `Send + 'static`-bounded: mirrors `store::ArtifactStore<P, Mutation>`'s own
/// where-clause exactly (`🏪️store/🦀️.rs`'s `impl<P, Mutation> ArtifactStore<P, Mutation>`)
/// since `ArtifactStore::new`/`store::parse_document_pack` both turned `async fn` under the
/// runtime-dependency sweep — no `resolve_kernel_future` bridge here (unlike this file's other async
/// fallout fixes): reconstructing a document store from real pack/spr bytes is exactly the kind of
/// caller-visible async boundary that sweep is meant to expose, not hide.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub async fn import_document_artifact<P, Mutation>(pack_bytes: &[u8], spr_bytes: &[u8]) -> Result<store::ArtifactStore<P, Mutation>, SpaceZipError>
where
    P: Clone + store::ToValue + store::FromValue + store::ArtifactPack + Send + 'static,
    Mutation: Clone + store::ToValue + store::FromValue + protocol::Mutation<P> + protocol::OpBinary + protocol::OpText + Send + 'static,
{
    let parsed = store::parse_document_pack::<P, Mutation>(pack_bytes, spr_bytes).await.map_err(|error| SpaceZipError::Pack(error.to_string()))?;
    store::ArtifactStore::new(parsed.envelope).await.map_err(|error| SpaceZipError::Pack(error.to_string()))
}

/// 📥️ Puts one imported blob's bytes into a live `store::BlobStore`, verifying the freshly computed
/// content hash matches the `store::BlobRef` recorded in the collection (a mismatch means the zip was
/// tampered with or corrupted in transit).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn import_blob<B: store::BlobStore>(blob_store: &B, blob: &store::BlobRef, bytes: Vec<u8>) -> Result<(), SpaceZipError> {
    let stored = crate::host::resolve_kernel_future(blob_store.put(&bytes, &blob.media_type)).map_err(|error| SpaceZipError::Pack(error.to_string()))?;
    if stored.hash != blob.hash {
        return Err(SpaceZipError::Pack(format!("blob hash mismatch on import: expected {}, got {}", blob.hash, stored.hash)));
    }
    Ok(())
}
//#endregion 🔖️ZipStoreBridge
//#endregion 🔖️Zip
//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
