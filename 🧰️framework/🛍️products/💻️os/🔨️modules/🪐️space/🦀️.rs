//! 🪐️ OS host orchestration for space drafts and collection archives.

pub use semio_framework_artifact_space_collection::*;
pub use semio_framework_artifact_space_space::*;

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
mod tests {
    use super::*;
    use protocol::DiffCodec;
    use protocol::Mutation as _;
    use protocol::MutationDiff as _;
    use store::{ArtifactDsl, BlobStore};

    //#region 🧸️Fixtures
    fn demo_user(id: &str, role: SpaceRole) -> SpaceUser {
        SpaceUser { id: id.into(), name: format!("User {id}"), avatar: None, role }
    }

    fn demo_extension(extension_id: &str, enabled: bool) -> InstalledExtension {
        InstalledExtension { extension_id: extension_id.into(), version: "1.0.0".into(), source_uri: format!("https://example.test/{extension_id}.sxt"), package_hash: format!("hash-{extension_id}"), enabled }
    }

    fn demo_space() -> SpaceSnapshot {
        let mut space = empty_space_snapshot("Atelier Demo", SpaceKind::Atelier, SpaceVisibility::Private);
        space.users.push(demo_user("u1", SpaceRole::Author));
        space.collections.push(CollectionRef { id: "c1".into(), name: "Main".into(), document_id: "doc-c1".into() });
        space
    }

    fn demo_collection() -> CollectionSnapshot {
        let mut collection = empty_collection_snapshot("Main");
        collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: None, name: "Renders".into() });
        collection.entries.push(CollectionEntry {
            id: "e1".into(),
            folder_id: Some("f1".into()),
            name: "sketch".into(),
            kind_id: "puzzle.2d".into(),
            body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e1".into() }),
        });
        collection.entries.push(CollectionEntry {
            id: "e2".into(),
            folder_id: None,
            name: "reference.png".into(),
            kind_id: "file.blob".into(),
            body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "blake3-deadbeef".into(), size: 42, media_type: "image/png".into() } }),
        });
        collection
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️SpaceDocumentLaws
    #[test]
    fn empty_space_snapshot_matches_schema() {
        let space = empty_space_snapshot("Demo", SpaceKind::Studio, SpaceVisibility::Public);
        assert_eq!(space.schema, S_SPACE_SCHEMA);
        assert!(space.users.is_empty());
        assert!(space.collections.is_empty());
    }

    #[test]
    fn space_snapshot_dsl_pack_round_trips() {
        store::test_support::assert_dsl_pack_equivalence(&demo_space());
    }

    #[test]
    fn space_default_example_dsl_round_trips() {
        let text = include_str!("📚️examples/🪐️demo.space");
        let parsed = <SpaceSnapshot as ArtifactDsl>::parse_dsl(text).expect("parse default .space example");
        store::test_support::assert_dsl_round_trip(&parsed);
    }
    //#endregion 🧪️SpaceDocumentLaws

    //#region 🧪️CollectionDocumentLaws
    #[test]
    fn empty_collection_snapshot_matches_schema() {
        let collection = empty_collection_snapshot("Demo");
        assert_eq!(collection.schema, S_COLLECTION_SCHEMA);
        assert!(collection.folders.is_empty());
        assert!(collection.entries.is_empty());
    }

    #[test]
    fn collection_projection_dsl_pack_round_trips() {
        store::test_support::assert_dsl_pack_equivalence(&demo_collection());
    }

    #[test]
    fn collection_envelope_id_is_two_dot_segments() {
        let id = <CollectionSnapshot as ArtifactDsl>::envelope_id();
        assert_eq!(id, "os.collection");
        assert_eq!(id.split('.').count(), 2, "SemioEnvelope::from_envelope_id requires plugin.artifact");
        let pack = store::ArtifactPack::encode_pack(&empty_collection_snapshot("test"));
        let decoded = <CollectionSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded.name, "test");
    }

    #[test]
    fn space_envelope_id_is_two_dot_segments() {
        let id = <SpaceSnapshot as ArtifactDsl>::envelope_id();
        assert_eq!(id, "os.space");
        assert_eq!(id.split('.').count(), 2, "SemioEnvelope::from_envelope_id requires plugin.artifact");
        let pack = store::ArtifactPack::encode_pack(&empty_space_snapshot("test", SpaceKind::Atelier, SpaceVisibility::Private));
        let decoded = <SpaceSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded.name, "test");
    }

    #[test]
    fn collection_default_example_dsl_round_trips() {
        let text = include_str!("📚️examples/🎬️demo.collection");
        let parsed = <CollectionSnapshot as ArtifactDsl>::parse_dsl(text).expect("parse default .collection example");
        store::test_support::assert_dsl_round_trip(&parsed);
    }
    //#endregion 🧪️CollectionDocumentLaws

    //#region 🧪️SpaceMutationLaws
    #[test]
    fn space_operation_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SpaceMutation::SetName { name: "Renamed".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::SetKind { kind: SpaceKind::Studio });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::SetVisibility { visibility: SpaceVisibility::Public });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::UpsertUser { user: demo_user("u2", SpaceRole::Spectator) });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::RemoveUser { user_id: "u2".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::AddCollection { collection: CollectionRef { id: "c2".into(), name: "Extra".into(), document_id: "doc-c2".into() } });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::RemoveCollection { collection_id: "c2".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::RenameCollection { collection_id: "c1".into(), name: "Renamed Collection".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::InstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::UninstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::InstallExtension {
            extension_id: "flow-math".into(),
            version: "1.0.0".into(),
            source_uri: "https://example.test/flow-math.sxt".into(),
            package_hash: "hash-flow-math".into(),
            enabled: true,
        });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::UninstallExtension { extension_id: "flow-math".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::SetExtensionEnabled { extension_id: "flow-math".into(), enabled: false });
    }

    #[test]
    fn space_operation_backwards_restores_pre_state() {
        let base = demo_space();
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::SetName { name: "New Name".into() });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::UpsertUser { user: demo_user("u2", SpaceRole::Author) });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::UpsertUser { user: demo_user("u1", SpaceRole::Spectator) });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::RemoveUser { user_id: "u1".into() });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::AddCollection { collection: CollectionRef { id: "c2".into(), name: "Extra".into(), document_id: "doc-c2".into() } });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::RemoveCollection { collection_id: "c1".into() });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::RenameCollection { collection_id: "c1".into(), name: "Renamed".into() });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::InstallProgram { plugin_id: "cad".into() });
        let mut with_program = base.clone();
        with_program.programs.push("cad".into());
        store::test_support::assert_operation_round_trip(&with_program, SpaceMutation::UninstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_operation_round_trip(
            &base,
            SpaceMutation::InstallExtension { extension_id: "flow-math".into(), version: "1.0.0".into(), source_uri: "https://example.test/flow-math.sxt".into(), package_hash: "hash-flow-math".into(), enabled: true },
        );
        let mut with_extension = base.clone();
        with_extension.extensions.push(demo_extension("flow-math", true));
        store::test_support::assert_operation_round_trip(&with_extension, SpaceMutation::UninstallExtension { extension_id: "flow-math".into() });
        store::test_support::assert_operation_round_trip(&with_extension, SpaceMutation::SetExtensionEnabled { extension_id: "flow-math".into(), enabled: false });
        store::test_support::assert_operation_round_trip(
            &with_extension,
            SpaceMutation::InstallExtension { extension_id: "flow-math".into(), version: "2.0.0".into(), source_uri: "https://example.test/flow-math-v2.sxt".into(), package_hash: "hash-flow-math-v2".into(), enabled: false },
        );
    }

    #[test]
    fn space_diff_print_parse_and_encode_decode_round_trip() {
        let diffs = vec![
            SpaceDiff { name: Some("Renamed".into()), ..Default::default() },
            SpaceDiff { upsert_user: Some(demo_user("u2", SpaceRole::Author)), ..Default::default() },
            SpaceDiff { install_program: Some("cad".into()), ..Default::default() },
            SpaceDiff { uninstall_program: Some("cad".into()), ..Default::default() },
            SpaceDiff { install_extension: Some(demo_extension("flow-math", true)), ..Default::default() },
            SpaceDiff { uninstall_extension_id: Some("flow-math".into()), ..Default::default() },
            SpaceDiff { set_extension_enabled_id: Some("flow-math".into()), set_extension_enabled: Some(false), ..Default::default() },
            SpaceDiff::default(),
        ];
        for diff in diffs {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line: {printed:?}");
            let parsed = SpaceDiff::parse_diff(&printed).unwrap_or_else(|error| panic!("parse_diff failed for {printed:?}: {error}"));
            assert_eq!(parsed, diff);
            let encoded = diff.encode_diff().expect("encode_diff");
            let decoded = SpaceDiff::decode_diff(&encoded).expect("decode_diff");
            assert_eq!(decoded, diff);
        }
    }
    //#endregion 🧪️SpaceMutationLaws

    //#region 🧪️CollectionMutationLaws
    #[test]
    fn collection_operation_op_text_round_trips_every_variant() {
        let folder = CollectionFolder { id: "f2".into(), parent_id: None, name: "Extra".into() };
        let entry = CollectionEntry { id: "e3".into(), folder_id: None, name: "extra".into(), kind_id: "puzzle.2d".into(), body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e3".into() }) };
        store::test_support::assert_op_line_round_trip(&CollectionMutation::RenameCollection { new_name: "Renamed".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::CreateFolder { folder: folder.clone(), index: 0 });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::DeleteFolder { folder_id: "f1".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveToCollection { folder_id: "f1".into(), new_parent: Some("f2".into()) });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveToCollection { folder_id: "f1".into(), new_parent: None });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::RenameFolder { folder_id: "f1".into(), new_name: "Renders 2".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::CreateEntry { entry: entry.clone(), index: 0 });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::DeleteEntry { entry_id: "e1".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveToFolder { entry_id: "e1".into(), new_folder: None });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::RenameEntry { entry_id: "e1".into(), new_name: "sketch 2".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::ReplaceEntryBody { entry_id: "e2".into(), new_body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "h2".into(), size: 1, media_type: "image/png".into() } }) });
    }

    #[test]
    fn collection_operation_binary_matches_text() {
        store::test_support::assert_op_text_binary_equivalence(&CollectionMutation::RenameCollection { new_name: "Renamed".into() });
        let entry = CollectionEntry { id: "e3".into(), folder_id: None, name: "extra".into(), kind_id: "puzzle.2d".into(), body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e3".into() }) };
        store::test_support::assert_op_text_binary_equivalence(&CollectionMutation::CreateEntry { entry, index: 0 });
    }

    #[test]
    fn collection_operation_backwards_restores_pre_state() {
        let base = demo_collection();
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::RenameCollection { new_name: "Renamed".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::CreateFolder { folder: CollectionFolder { id: "f2".into(), parent_id: None, name: "Extra".into() }, index: 0 });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::DeleteFolder { folder_id: "f1".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::MoveToCollection { folder_id: "f1".into(), new_parent: None });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::RenameFolder { folder_id: "f1".into(), new_name: "Renders 2".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::DeleteEntry { entry_id: "e1".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::MoveToFolder { entry_id: "e1".into(), new_folder: None });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::RenameEntry { entry_id: "e1".into(), new_name: "sketch 2".into() });
        store::test_support::assert_operation_round_trip(
            &base,
            CollectionMutation::ReplaceEntryBody { entry_id: "e2".into(), new_body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "h2".into(), size: 1, media_type: "image/png".into() } }) },
        );
    }

    /// 🧪️ `DeleteFolder`'s cascade: deleting a folder that contains a nested subfolder plus entries in
    /// both must remove the whole subtree, and `inverse` must restore it leaves-first (proves the
    /// `folder_subtree_ids`/`folder_depth` helpers rather than trusting the shallow single-folder cases
    /// above to exercise them).
    #[test]
    fn delete_folder_cascade_removes_and_restores_whole_subtree() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.folders.push(CollectionFolder { id: "root".into(), parent_id: None, name: "Root".into() });
        collection.folders.push(CollectionFolder { id: "child".into(), parent_id: Some("root".into()), name: "Child".into() });
        collection.entries.push(CollectionEntry {
            id: "e-root".into(),
            folder_id: Some("root".into()),
            name: "in-root".into(),
            kind_id: "puzzle.2d".into(),
            body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e-root".into() }),
        });
        collection.entries.push(CollectionEntry {
            id: "e-child".into(),
            folder_id: Some("child".into()),
            name: "in-child".into(),
            kind_id: "puzzle.2d".into(),
            body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e-child".into() }),
        });

        store::test_support::assert_operation_round_trip(&collection, CollectionMutation::DeleteFolder { folder_id: "root".into() });

        let diff = CollectionMutation::DeleteFolder { folder_id: "root".into() }.diff(&collection).into_parts().0;
        let mut deleted_folders = diff.deleted_folder_ids.clone().unwrap_or_default();
        deleted_folders.sort();
        assert_eq!(deleted_folders, vec!["child".to_string(), "root".to_string()]);
        let mut deleted_entries = diff.deleted_entry_ids.clone().unwrap_or_default();
        deleted_entries.sort();
        assert_eq!(deleted_entries, vec!["e-child".to_string(), "e-root".to_string()]);

        let after = diff.apply(&collection).expect("valid collection diff");
        assert!(after.folders.is_empty());
        assert!(after.entries.is_empty());
    }

    #[test]
    fn collection_diff_print_parse_and_encode_decode_round_trip() {
        let diffs = vec![CollectionDiff { renamed_collection: Some("Renamed".into()), ..Default::default() }, CollectionDiff { deleted_entry_ids: Some(vec!["e1".into()]), ..Default::default() }, CollectionDiff::default()];
        for diff in diffs {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'));
            let parsed = CollectionDiff::parse_diff(&printed).unwrap_or_else(|error| panic!("parse_diff failed for {printed:?}: {error}"));
            assert_eq!(parsed, diff);
            let encoded = diff.encode_diff().expect("encode_diff");
            let decoded = CollectionDiff::decode_diff(&encoded).expect("decode_diff");
            assert_eq!(decoded, diff);
        }
    }
    //#endregion 🧪️CollectionMutationLaws

    //#region 🧪️RoleLaws
    #[test]
    fn can_write_follows_kind_and_role() {
        let mut archive = empty_space_snapshot("Frozen", SpaceKind::Archive, SpaceVisibility::Public);
        archive.users.push(demo_user("u1", SpaceRole::Author));
        assert!(!can_write(&archive, "u1"), "archive never accepts writes, even from an author");

        let mut studio = empty_space_snapshot("Studio", SpaceKind::Studio, SpaceVisibility::Private);
        studio.users.push(demo_user("u1", SpaceRole::Author));
        studio.users.push(demo_user("u2", SpaceRole::Spectator));
        assert!(can_write(&studio, "u1"));
        assert!(!can_write(&studio, "u2"));
        assert!(!can_write(&studio, "unknown"));
    }

    #[test]
    fn atelier_reconcile_keeps_a_single_author_by_smallest_id() {
        let mut atelier = empty_space_snapshot("Atelier", SpaceKind::Atelier, SpaceVisibility::Private);
        atelier.users.push(demo_user("u2", SpaceRole::Author));
        atelier.users.push(demo_user("u1", SpaceRole::Author));
        let (reconciled, reports) = reconcile_space_atelier_invariant(atelier);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].target.first().map(String::as_str), Some("space/atelier-multi-author"));
        assert_eq!(space_role_of(&reconciled, "u1"), Some(SpaceRole::Author));
        assert_eq!(space_role_of(&reconciled, "u2"), Some(SpaceRole::Spectator));
    }

    #[test]
    fn atelier_reconcile_is_a_noop_with_a_single_author() {
        let (_, reports) = reconcile_space_atelier_invariant(demo_space());
        assert!(reports.is_empty());
    }
    //#endregion 🧪️RoleLaws

    //#region 🧪️CollectionReconcileLaws
    #[test]
    fn reconcile_reparents_orphan_folder_to_root() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: Some("missing".into()), name: "Orphan".into() });
        let (reconciled, reports) = reconcile_collection_integrity(collection);
        assert!(reports.iter().any(|r| r.target.first().map(String::as_str) == Some("collection/folder-orphaned")));
        assert_eq!(reconciled.folders[0].parent_id, None);
    }

    #[test]
    fn reconcile_cuts_folder_cycle() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.folders.push(CollectionFolder { id: "a".into(), parent_id: Some("b".into()), name: "A".into() });
        collection.folders.push(CollectionFolder { id: "b".into(), parent_id: Some("a".into()), name: "B".into() });
        let (reconciled, reports) = reconcile_collection_integrity(collection);
        assert!(reports.iter().any(|r| r.target.first().map(String::as_str) == Some("collection/folder-cycle")));
        assert!(reconciled.folders.iter().all(|f| f.parent_id.is_none()), "both cyclic folders must be cut to root");
    }

    #[test]
    fn reconcile_suffixes_duplicate_sibling_folder_names() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: None, name: "Renders".into() });
        collection.folders.push(CollectionFolder { id: "f2".into(), parent_id: None, name: "Renders".into() });
        let (reconciled, reports) = reconcile_collection_integrity(collection);
        assert!(reports.iter().any(|r| r.target.first().map(String::as_str) == Some("collection/folder-name-collision")));
        let names: Vec<&str> = reconciled.folders.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["Renders", "Renders (2)"]);
    }

    #[test]
    fn reconcile_reparents_entry_pointing_at_missing_folder() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.entries.push(CollectionEntry {
            id: "e1".into(),
            folder_id: Some("missing".into()),
            name: "sketch".into(),
            kind_id: "puzzle.2d".into(),
            body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e1".into() }),
        });
        let (reconciled, reports) = reconcile_collection_integrity(collection);
        assert!(reports.iter().any(|r| r.target.first().map(String::as_str) == Some("collection/entry-folder-missing")));
        assert_eq!(reconciled.entries[0].folder_id, None);
    }
    //#endregion 🧪️CollectionReconcileLaws

    //#region 🧪️PathResolverLaws
    #[test]
    fn folder_and_entry_path_round_trip() {
        let collection = demo_collection();
        assert_eq!(folder_path(&collection, "f1"), Some("Renders".into()));
        assert_eq!(entry_path(&collection, "e1"), Some("Renders/sketch".into()));
        assert_eq!(entry_path(&collection, "e2"), Some("reference.png".into()));
        assert_eq!(resolve_entry_by_path(&collection, "Renders/sketch").map(|entry| entry.id.as_str()), Some("e1"));
        assert_eq!(resolve_entry_by_path(&collection, "reference.png").map(|entry| entry.id.as_str()), Some("e2"));
        assert_eq!(resolve_entry_by_path(&collection, "nowhere"), None);
    }

    #[test]
    fn moves_and_renames_never_break_id_based_refs() {
        let mut collection = demo_collection();
        let before_id = collection.entries[0].id.clone();
        // Rename the folder — the path changes, the id-based ref doesn't.
        collection.folders[0].name = "Outputs".into();
        assert_eq!(entry_path(&collection, &before_id), Some("Outputs/sketch".into()));
        assert!(collection.entries.iter().any(|entry| entry.id == before_id));
    }

    #[test]
    fn backbone_uris_are_stable() {
        assert_eq!(space_backbone_uri("space-1"), "space://space-1");
        assert_eq!(collection_backbone_uri("space-1", "col-1"), "space://space-1/collection/col-1");
        assert_eq!(artifact_backbone_uri("space-1", "art-1"), "space://space-1/artifact/art-1");
    }
    //#endregion 🧪️PathResolverLaws

    //#region 🧪️DraftLaws
    fn memory_draft_port() -> Arc<store::BackbonePorts> {
        Arc::new(store::BackbonePorts::Memory(crate::host::resolve_kernel_future(store::MemoryBackbonePort::new())))
    }

    #[test]
    fn draft_create_list_expire_lifecycle() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft_a = catalog.create_draft("puzzle.2d", "test.puzzle2d", "sketch-a", 1_000, Some(500));
        let draft_b = catalog.create_draft("puzzle.2d", "test.puzzle2d", "sketch-b", 1_000, None);
        assert_eq!(draft_a.expires_at_ms, Some(1_500));
        assert_eq!(draft_b.expires_at_ms, None, "None ttl means pinned");

        let listed = catalog.list_drafts();
        assert_eq!(listed.len(), 2);

        let expired = catalog.expire_drafts(1_400, &port);
        assert!(expired.is_empty(), "not yet expired");
        let expired = catalog.expire_drafts(1_500, &port);
        assert_eq!(expired, vec![draft_a.artifact_id.clone()]);
        assert_eq!(catalog.list_drafts().len(), 1, "the pinned draft survives");
    }

    #[test]
    fn list_drafts_sweeping_expired_removes_stale_entries_first() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft = catalog.create_draft("puzzle.2d", "test.puzzle2d", "stale", 0, Some(100));
        assert_eq!(catalog.list_drafts_sweeping_expired(50, &port).len(), 1, "not yet expired");
        assert!(catalog.list_drafts_sweeping_expired(200, &port).is_empty(), "swept before listing");
        assert!(catalog.list_drafts().iter().all(|entry| entry.artifact_id != draft.artifact_id));
    }

    #[test]
    fn discard_draft_removes_bookkeeping_and_tombstones_bytes() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft = catalog.create_draft("puzzle.2d", "test.puzzle2d", "scratch", 0, None);
        port.write(&draft_uri(&draft.artifact_id), b"draft-bytes").expect("seed draft bytes");

        let removed = catalog.discard_draft(&port, &draft.artifact_id).expect("discard");
        assert_eq!(removed.artifact_id, draft.artifact_id);
        assert!(catalog.list_drafts().is_empty());
        assert_eq!(port.read(&draft_uri(&draft.artifact_id)).expect("read tombstone"), Vec::<u8>::new());
        assert!(catalog.discard_draft(&port, &draft.artifact_id).is_none(), "already discarded");
    }

    /// 🧪️ The plan's core promotion invariant, proven with REAL bytes (not a fixture string): the
    /// artifact's envelope bytes at `artifact_backbone_uri` after promotion are byte-for-byte IDENTICAL
    /// to what was at `draft_uri` before promotion — just relocated under a different backbone uri, no
    /// decode/re-encode anywhere in the path.
    #[test]
    fn draft_promote_moves_envelope_bytes_byte_identical() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft = catalog.create_draft("puzzle.2d", "test.puzzle2d", "sketch", 0, None);
        let original_bytes = b"pretend-pack-plus-spr-envelope-bytes-with-full-vcs-history".to_vec();
        port.write(&draft_uri(&draft.artifact_id), &original_bytes).expect("seed draft bytes");

        let (removed_draft, operation) = catalog.promote_draft(&port, "space-1", &draft.artifact_id, Some("f1".into())).expect("promote");
        assert_eq!(removed_draft.artifact_id, draft.artifact_id);
        let CollectionMutation::CreateEntry { entry, .. } = &operation else { panic!("promote_draft must return CreateEntry") };
        assert_eq!(entry.id, draft.artifact_id, "promotion preserves the document id");
        assert_eq!(entry.folder_id, Some("f1".into()));
        assert!(catalog.list_drafts().is_empty(), "promoted draft is no longer a draft");

        let moved_bytes = port.read(&artifact_backbone_uri("space-1", &draft.artifact_id)).expect("read promoted bytes");
        assert_eq!(moved_bytes, original_bytes, "promoted envelope bytes are byte-identical, just at a different backbone uri");
        assert_eq!(port.read(&draft_uri(&draft.artifact_id)).expect("read tombstoned draft uri"), Vec::<u8>::new(), "draft uri is tombstoned after promotion");

        let demote = DraftCatalog::demote_operation(&entry.id);
        assert_eq!(demote, CollectionMutation::DeleteEntry { entry_id: entry.id.clone() });

        // Mutation-sourced round trip: CreateEntry then its inverse restores the empty collection.
        let empty = empty_collection_snapshot("Demo");
        store::test_support::assert_operation_round_trip(&empty, operation);
    }

    /// 🧪️ `demote_asset` is `promote_draft`'s real byte-moving inverse: the SAME bytes travel back
    /// from the asset uri to the draft uri, byte-identical, and fresh draft bookkeeping reappears.
    #[test]
    fn demote_asset_moves_bytes_back_and_reregisters_draft_bookkeeping() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft = catalog.create_draft("puzzle.2d", "test.puzzle2d", "sketch", 0, None);
        let original_bytes = b"envelope-bytes-round-tripping-through-promote-then-demote".to_vec();
        port.write(&draft_uri(&draft.artifact_id), &original_bytes).expect("seed draft bytes");

        let (_, operation) = catalog.promote_draft(&port, "space-1", &draft.artifact_id, None).expect("promote");
        let CollectionMutation::CreateEntry { entry, .. } = operation else { panic!("expected CreateEntry") };

        let demote_operation = catalog.demote_asset(&port, "space-1", &entry, "test.puzzle2d", 2_000, Some(1_000)).expect("demote");
        assert_eq!(demote_operation, CollectionMutation::DeleteEntry { entry_id: entry.id.clone() });

        let restored_bytes = port.read(&draft_uri(&entry.id)).expect("read demoted draft bytes");
        assert_eq!(restored_bytes, original_bytes, "demoted envelope bytes are byte-identical to the originally-promoted ones");
        assert_eq!(port.read(&artifact_backbone_uri("space-1", &entry.id)).expect("read tombstoned asset uri"), Vec::<u8>::new());

        let redrafted = catalog.list_drafts();
        assert_eq!(redrafted.len(), 1);
        assert_eq!(redrafted[0].artifact_id, entry.id);
        assert_eq!(redrafted[0].expires_at_ms, Some(3_000), "demotion re-registers a fresh TTL window");
    }

    #[test]
    fn promote_unknown_draft_errors() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        assert_eq!(catalog.promote_draft(&port, "space-1", "nope", None), Err(SpaceError::UnknownDraft("nope".into())));
    }

    /// 🧪️ `draft_catalog_for` is the port-keyed global registry: the SAME `Arc<store::BackbonePorts>`
    /// identity always resolves to the SAME `DraftCatalog` instance (so callers sharing a port share
    /// draft bookkeeping), while two DISTINCT port identities never share one.
    #[test]
    fn draft_catalog_for_is_keyed_by_port_identity() {
        let port_a = memory_draft_port();
        let port_b = memory_draft_port();

        let catalog_a1 = draft_catalog_for(&port_a);
        let catalog_a1_created = catalog_a1.create_draft("puzzle.2d", "test.puzzle2d", "shared", 0, None);
        let catalog_a2 = draft_catalog_for(&port_a);
        assert_eq!(catalog_a2.list_drafts().iter().map(|entry| entry.artifact_id.clone()).collect::<Vec<_>>(), vec![catalog_a1_created.artifact_id.clone()], "same port identity shares one catalog");

        let catalog_b = draft_catalog_for(&port_b);
        assert!(catalog_b.list_drafts().is_empty(), "a distinct port identity gets its own catalog");
    }
    //#endregion 🧪️DraftLaws

    //#region 🧪️ZipLaws
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn zip_fixture_bytes() -> Vec<u8> {
        let collection = demo_collection();
        let read_artifact = |entry_id: &str| -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> { Ok((format!("pack-bytes-for-{entry_id}").into_bytes(), format!("spr-bytes-for-{entry_id}").into_bytes())) };
        let read_blob = |hash: &str| -> Result<Vec<u8>, SpaceZipError> { Ok(format!("blob-bytes-for-{hash}").into_bytes()) };
        export_collection_zip(&collection, b"collection-spr-bytes", &read_artifact, &read_blob).expect("export")
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn zip_export_import_round_trips_structure_and_bytes() {
        let bytes = zip_fixture_bytes();
        let imported = import_collection_zip(&bytes).expect("import");
        assert_eq!(imported.collection, demo_collection());
        assert_eq!(imported.collection_spr, b"collection-spr-bytes");
        assert_eq!(imported.artifacts.len(), 1, "one document entry");
        let (entry, pack_bytes, spr_bytes) = &imported.artifacts[0];
        assert_eq!(entry.id, "e1");
        assert_eq!(pack_bytes, b"pack-bytes-for-e1");
        assert_eq!(spr_bytes, b"spr-bytes-for-e1");
        assert_eq!(imported.blobs.len(), 1, "one blob entry");
        assert_eq!(imported.blobs[0].0.hash, "blake3-deadbeef");
        assert_eq!(imported.blobs[0].1, b"blob-bytes-for-blake3-deadbeef");
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn zip_export_import_export_is_byte_stable() {
        let once = zip_fixture_bytes();
        let imported = import_collection_zip(&once).expect("import");
        let read_artifact = |entry_id: &str| -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> {
            let (_, pack_bytes, spr_bytes) = imported.artifacts.iter().find(|(entry, _, _)| entry.id == entry_id).expect("artifact bytes");
            Ok((pack_bytes.clone(), spr_bytes.clone()))
        };
        let read_blob = |hash: &str| -> Result<Vec<u8>, SpaceZipError> {
            let (_, bytes) = imported.blobs.iter().find(|(blob, _)| blob.hash == hash).expect("blob bytes");
            Ok(bytes.clone())
        };
        let twice = export_collection_zip(&imported.collection, &imported.collection_spr, &read_artifact, &read_blob).expect("re-export");
        assert_eq!(once, twice, "export -> import -> export must be byte-stable");
    }
    //#endregion 🧪️ZipLaws

    //#region 🧪️ZipStoreBridgeLaws
    /// 🧪️ Minimal in-memory `store::BlobStore` test double — content-addressed via a fast
    /// non-cryptographic hash (no need for `framework_hash`'s real Blake3, this crate has no such
    /// dependency and a test double only needs internal consistency, not a production hash).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[derive(Default)]
    struct TestBlobStore {
        entries: Mutex<HashMap<String, (Vec<u8>, String)>>,
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn test_blob_hash(bytes: &[u8]) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut hasher);
        format!("test-{:016x}", hasher.finish())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl BlobStore for TestBlobStore {
        async fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
            let hash = test_blob_hash(bytes);
            self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(hash.clone(), (bytes.to_vec(), media_type.to_string()));
            Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
        }

        async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
            Ok(self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(hash).map(|(bytes, _)| bytes.clone()))
        }

        async fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
            Ok(self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(hash))
        }

        async fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
            self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(hash);
            Ok(())
        }
    }

    /// 🧪️ End-to-end law with REAL `store` types (not the fixture-string readers `zip_fixture_bytes`
    /// injects above): a real `store::ArtifactStore<SpaceSnapshot, SpaceMutation>` document
    /// artifact (itself an ordinary `#[derive(dsl::DslArtifact)]`/`#[derive(dsl::DslOps)]` document —
    /// exercising exactly the same `ArtifactPack`/`OpBinary`/`OpText` machinery any real app document
    /// would) plus a real blob round-trip through `real_artifact_reader`/`real_blob_reader`/
    /// `import_document_artifact`/`import_blob`, asserting the round trip preserves collection
    /// structure AND artifact envelope bytes byte-for-byte (the plan's "lossless" requirement), and
    /// that export->import->export stays byte-stable with real data too.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn zip_export_import_round_trips_real_store_documents_and_blob() {
        // 🌉️ `ArtifactStore::new`/`dispatch`/`snapshot_pack`/`TestBlobStore::put`/`get` all turned
        // `async fn` under the runtime-dependency sweep; this test stays a plain sync `#[test]` (this
        // crate has no `#[async_test]` harness dependency) and bridges via
        // `crate::host::resolve_kernel_future`, same as this file's other async fallout fixes — every
        // op here is an in-memory fixture operation, never real I/O.
        let mut nested_space_store = crate::host::resolve_kernel_future(store::ArtifactStore::new(store::create_document_envelope::<SpaceSnapshot, SpaceMutation>(S_SPACE_SCHEMA, "art-nested-space", demo_space(), None))).expect("valid artifact store fixture");
        crate::host::resolve_kernel_future(nested_space_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![SpaceMutation::SetName { name: "Nested Space".into() }], description: None })).expect("apply");
        crate::host::resolve_kernel_future(nested_space_store.dispatch(store::ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() })).expect("commit checkpoint");
        let original_pack_files = crate::host::resolve_kernel_future(nested_space_store.snapshot_pack()).expect("snapshot pack");

        let blob_store = TestBlobStore::default();
        let blob_ref = crate::host::resolve_kernel_future(blob_store.put(b"hello blob bytes", "text/plain")).expect("put blob");

        let mut collection = empty_collection_snapshot("RealDemo");
        collection.entries.push(CollectionEntry {
            id: "art-nested-space".into(),
            folder_id: None,
            name: "nested-space".into(),
            kind_id: "os.space".into(),
            body: Box::new(ArtifactBody::Document { schema: S_SPACE_SCHEMA.into(), document_id: "art-nested-space".into() }),
        });
        collection.entries.push(CollectionEntry { id: "blob-1".into(), folder_id: None, name: "note.txt".into(), kind_id: "file.blob".into(), body: Box::new(ArtifactBody::Blob { blob: blob_ref.clone() }) });

        let mut pack_files = HashMap::new();
        pack_files.insert("art-nested-space".to_string(), original_pack_files.clone());
        let read_artifact = real_artifact_reader(&pack_files);
        let read_blob = real_blob_reader(&blob_store);
        let collection_spr = b"collection-history-bytes".to_vec();
        let zip_bytes = export_collection_zip(&collection, &collection_spr, &read_artifact, &read_blob).expect("export");

        let imported = import_collection_zip(&zip_bytes).expect("import");
        assert_eq!(imported.collection, collection, "collection structure survives the round trip");
        assert_eq!(imported.collection_spr, collection_spr);

        let (_, imported_pack, imported_spr) = imported.artifacts.iter().find(|(entry, _, _)| entry.id == "art-nested-space").expect("artifact present");
        assert_eq!(imported_pack, &original_pack_files.pack, "artifact pack bytes are byte-identical after the round trip");
        assert_eq!(imported_spr, &original_pack_files.spr, "artifact spr bytes are byte-identical after the round trip");

        let restored_store = crate::host::resolve_kernel_future(import_document_artifact::<SpaceSnapshot, SpaceMutation>(imported_pack, imported_spr)).expect("reconstruct store");
        assert_eq!(restored_store.snapshot().expect("projection"), nested_space_store.snapshot().expect("projection"), "reconstructed document projection matches the original exactly");

        let (imported_blob, imported_blob_bytes) = imported.blobs.iter().find(|(blob, _)| blob.hash == blob_ref.hash).expect("blob present");
        assert_eq!(imported_blob_bytes, b"hello blob bytes");
        let fresh_blob_store = TestBlobStore::default();
        import_blob(&fresh_blob_store, imported_blob, imported_blob_bytes.clone()).expect("import blob");
        assert_eq!(crate::host::resolve_kernel_future(fresh_blob_store.get(&blob_ref.hash)).expect("get"), Some(b"hello blob bytes".to_vec()));

        // export -> import -> export must stay byte-stable with REAL data too (not just injected
        // fixture strings) — the law `zip_export_import_export_is_byte_stable` proved with mock bytes
        // holds end-to-end.
        let mut reexport_pack_files = HashMap::new();
        reexport_pack_files.insert("art-nested-space".to_string(), store::ArtifactPackFiles { pack: imported_pack.clone(), spr: imported_spr.clone(), ops: String::new() });
        let re_read_artifact = real_artifact_reader(&reexport_pack_files);
        let re_read_blob = real_blob_reader(&fresh_blob_store);
        let twice = export_collection_zip(&imported.collection, &imported.collection_spr, &re_read_artifact, &re_read_blob).expect("re-export");
        assert_eq!(zip_bytes, twice, "export -> import -> export is byte-stable with real store-backed data");
    }
    //#endregion 🧪️ZipStoreBridgeLaws
}
//#endregion 🧪️Tests
