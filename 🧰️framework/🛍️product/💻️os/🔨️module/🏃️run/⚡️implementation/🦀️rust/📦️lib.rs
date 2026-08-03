//! 🕸️ Headless computation of an OS studio's workflow — no UI involved. A `SpaceRunner` walks
//! `workflow::Workflow` (the kernel-crate persisted graph — a node IS the app-instance, identified
//! solely by `WorkflowNode::id`) in topological order, drives each node's app through
//! `AppChannelHost` — the exact `protocol::AppCommand`/`AppFrame` binary channel a live UI speaks, so
//! a headless run never needs a UI-mock API — moves `Media` along edges, and skips any node whose
//! inputs, document, and config are all unchanged since the last run. Every node's frame script is
//! `Hello → LoadConfig → LoadDocument → MediaIn* → (MediaOut+MediaFingerprint)* → ReadDocument →
//! ReadConfig` (see `SpaceRunner::compute_node`); documents/configs are addressed by their node's own
//! `document_ref`/`config_ref` string, never by a separate instance id.
//! Importing media is emitting operations: a headless run is an ordinary editing session (actor `runner`)
//! recorded in each app document's own VCS envelope, so a later UI open sees it as normal history.

//#region 🔖️Types
/// 🎞️ The exact binary channel a live UI speaks — re-exported so an `AppChannelHost` implementor
/// never needs a direct `protocol` dependency just to name these types.
pub use protocol::{AppCommand, AppFrame, CHANNEL_VERSION};
use semio_framework_core::{media_types_compatible, Media, MediaClass, MediaCompat, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaType, MediaWireFormat, PortMultiplicity};
use workflow::{MediaContract, Workflow, WorkflowEdge, WorkflowNode};
use dsl::{from_dsl_value, to_dsl_value};
use store::BlobStore;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};

/// 🚧️ A failure computing a studio's workflow headlessly.
#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("unknown workflow node {0}")]
    UnknownNode(String),
    #[error("workflow edge {edge_id} references unknown port `{port_id}` on node `{node_id}`")]
    UnknownPort { edge_id: String, node_id: String, port_id: String },
    #[error("workflow edge {edge_id} type mismatch: producer offers {produced:?}, consumer accepts {accepted:?}")]
    Incompatible { edge_id: String, produced: MediaType, accepted: MediaType },
    #[error("workflow edge {edge_id} negotiated a conversion {from:?} -> {to:?} but no converter is registered for it")]
    UnregisteredConversion { edge_id: String, from: MediaForm, to: MediaForm },
    #[error("input port `{port_id}` on node `{node_id}` is required but has no incoming edge")]
    MissingRequiredInput { node_id: String, port_id: String },
    #[error("input port `{port_id}` on node `{node_id}` accepts at most one connection but has {count}")]
    MultiplicityViolation { node_id: String, port_id: String, count: usize },
    #[error("no media converter registered for {class:?}: {from:?} -> {to:?}")]
    NoConverter { class: MediaClass, from: MediaForm, to: MediaForm },
    #[error("workflow has a cycle (unreachable nodes: {0:?})")]
    Cycle(Vec<String>),
    #[error("host error: {0}")]
    Host(String),
    #[error("media error: {0}")]
    Media(#[from] MediaError),
    #[error("io error at {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("(de)serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
//#endregion 🔖️Types

//#region 🔖️AppChannelHost
/// 🔌️ The one seam `SpaceRunner` calls through — every concrete plugin host (native wasmtime,
/// browser worker, or an in-process fake for tests) implements this the same way, driving a node
/// through exactly the binary `AppCommand`/`AppFrame` channel a live UI speaks (see
/// `protocol_channel`) — a headless run is never a separate UI-mock API. `open` mints an opaque
/// handle the runner threads back on every later `exchange` call; `exchange` is a single batched,
/// synchronous duplex round trip (`WasmPluginRuntime::exchange`'s native counterpart).
pub trait AppChannelHost {
    fn open(&mut self, plugin_id: &str, app_id: &str) -> Result<u32, RunError>;
    fn exchange(&mut self, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError>;
}
//#endregion 🔖️AppChannelHost

//#region 🔖️MediaCache
/// 📦️ Content-addressed cache of exported `Media` values, keyed by `MediaFingerprint`. Lets a
/// downstream dirty node import a clean upstream node's last output without re-instantiating that
/// upstream node at all — the whole point of fingerprint-based incrementality.
pub trait MediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media>;
    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media);
}

/// 🧠️ Process-local `MediaCache` — sufficient for a single `run()` call; nothing survives the process.
#[derive(Default)]
pub struct InMemoryMediaCache {
    entries: HashMap<String, Media>,
}

impl MediaCache for InMemoryMediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media> {
        self.entries.get(&fingerprint.0).cloned()
    }

    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media) {
        self.entries.insert(fingerprint.0.clone(), media.clone());
    }
}

/// 💾️ Disk-backed `MediaCache` under `<studio>/run/media/<fingerprint>.json` — the persistent
/// counterpart to `InMemoryMediaCache`, so a cold-started runner still skips re-exporting a clean
/// node's output when a prior run already cached it.
pub struct FileMediaCache {
    root: PathBuf,
}

impl FileMediaCache {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn entry_path(&self, fingerprint: &MediaFingerprint) -> PathBuf {
        self.root.join(format!("{}.json", fingerprint.0))
    }
}

impl MediaCache for FileMediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media> {
        let text = std::fs::read_to_string(self.entry_path(fingerprint)).ok()?;
        serde_json::from_str(&text).ok()
    }

    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media) {
        if std::fs::create_dir_all(&self.root).is_err() {
            return;
        }
        if let Ok(text) = serde_json::to_string(media) {
            let _ = std::fs::write(self.entry_path(fingerprint), text);
        }
    }
}
//#endregion 🔖️MediaCache

//#region 🔖️BlobStore
/// 💾️ Disk-backed `store::BlobStore` under `<studio>/blobs/<hash>` — backs both a `WasmPluginRuntime`'s
/// guest-side `write-blob`/`read-blob` host calls (via `WasmtimeNodeHost` registering it on every
/// runtime it loads) and `media_to_artifact`/`media_from_artifact`'s own resolution of a
/// `MediaPayload::Binary` value's bytes on the way on/off the wire.
pub struct FileBlobStore {
    root: PathBuf,
}

impl FileBlobStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn blob_path(&self, hash: &str) -> PathBuf {
        self.root.join(hash)
    }
}

impl BlobStore for FileBlobStore {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
        let hash = framework_hash::hash_bytes(bytes);
        std::fs::create_dir_all(&self.root).map_err(|error| store::VcsError::Backbone(error.to_string()))?;
        std::fs::write(self.blob_path(&hash), bytes).map_err(|error| store::VcsError::Backbone(error.to_string()))?;
        Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
        match std::fs::read(self.blob_path(hash)) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(store::VcsError::Backbone(error.to_string())),
        }
    }

    fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
        Ok(self.blob_path(hash).exists())
    }

    fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
        match std::fs::remove_file(self.blob_path(hash)) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(store::VcsError::Backbone(error.to_string())),
        }
    }
}

/// 🧠️ Process-local `store::BlobStore` — the `InMemoryMediaCache` counterpart for blob bytes, used
/// wherever a full `SpaceBundle` (and its `blobs/` dir) isn't in play, chiefly `SpaceRunner`'s own
/// `FakeHost`-based unit tests.
#[derive(Default)]
pub struct InMemoryBlobStore {
    entries: Mutex<HashMap<String, (Vec<u8>, String)>>,
}

impl BlobStore for InMemoryBlobStore {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
        let hash = framework_hash::hash_bytes(bytes);
        let mut entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        entries.insert(hash.clone(), (bytes.to_vec(), media_type.to_string()));
        Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
        let entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        Ok(entries.get(hash).map(|(bytes, _)| bytes.clone()))
    }

    fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
        let entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        Ok(entries.contains_key(hash))
    }

    fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
        let mut entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        entries.remove(hash);
        Ok(())
    }
}
//#endregion 🔖️BlobStore

//#region 🔖️MediaArtifact
/// 🔁️ Lossless bridge from `Media` to the wire-level `(descriptor, data)` byte pair carried by
/// `AppCommand::MediaIn`/`AppFrame::Media` — reuses `semio_framework_plugin::app::MediaArtifactDescriptor`
/// directly (not a hand-mirrored duplicate) so the runner and every guest plugin's
/// `plugin_consume_media`/`plugin_produce_media` glue agree on the shape by construction. A `Binary`
/// payload's bytes never live inline in `Media` (only its content-addressed `blob_hash` does) — this
/// is the one place that boundary is crossed, resolving them through `blob_store` into the wire's
/// inline `data`.
pub fn media_to_artifact(media: &Media, blob_store: &dyn BlobStore) -> Result<(Vec<u8>, Vec<u8>), RunError> {
    let (wire, blob_hash, data) = match &media.payload {
        MediaPayload::Structured { schema, json } => (MediaWireFormat::Document { schema: schema.clone() }, None, json.clone().into_bytes()),
        MediaPayload::Binary { format, blob_hash } => {
            let bytes = blob_store.get(blob_hash).map_err(|error| RunError::Host(error.to_string()))?.ok_or_else(|| RunError::Host(format!("blob not found: {blob_hash}")))?;
            (MediaWireFormat::Binary { format: *format }, Some(blob_hash.clone()), bytes)
        }
    };
    let descriptor = semio_framework_plugin::app::MediaArtifactDescriptor { edge_id: None, port_id: None, kind_id: None, media_type: Some(media.media_type), wire, blob_hash };
    let descriptor_value = to_dsl_value(&descriptor).map_err(|error| RunError::Host(error))?;
    Ok((store::pack_rt::encode_wire_value(&descriptor_value), data))
}

/// 🔁️ Inverse of [`media_to_artifact`]. A `Binary` wire artifact's `data` is written into
/// `blob_store` (content-addressed, idempotent) rather than kept inline, mirroring `Media`'s own
/// "binary payloads never carry bytes directly" invariant — the freshly computed hash supersedes
/// whatever `blob_hash` the artifact's own descriptor claimed.
pub fn media_from_artifact(descriptor: &[u8], data: Vec<u8>, blob_store: &dyn BlobStore) -> Result<Media, RunError> {
    let value = store::pack_rt::decode_wire_value(descriptor).map_err(|error| RunError::Host(error.to_string()))?;
    let descriptor: semio_framework_plugin::app::MediaArtifactDescriptor =
        from_dsl_value(value).map_err(|error| RunError::Host(error))?;
    let media_type = descriptor.media_type.ok_or_else(|| RunError::Host("media artifact descriptor is missing media_type".to_string()))?;
    let payload = match descriptor.wire {
        MediaWireFormat::Document { schema } => MediaPayload::Structured { schema, json: String::from_utf8(data).map_err(|error| RunError::Host(error.to_string()))? },
        MediaWireFormat::Binary { format } => {
            let blob_ref = blob_store.put(&data, format.mime_type()).map_err(|error| RunError::Host(error.to_string()))?;
            MediaPayload::Binary { format, blob_hash: blob_ref.hash }
        }
    };
    Ok(Media { media_type, payload })
}

/// 🔎️ Which `AppCommand::seq` (if any) an `AppFrame` replies to — `None` for the handful of
/// unsolicited/handshake shapes (`Welcome`, `DocumentChanged`, `ConfigChanged`) that never carry one.
fn frame_in_reply_to(frame: &AppFrame) -> Option<u64> {
    match frame {
        AppFrame::Done { in_reply_to } => Some(*in_reply_to),
        AppFrame::Invocation { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Document { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::ContextMenu { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Media { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::MediaFingerprint { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::UiSection { in_reply_to, .. } => *in_reply_to,
        AppFrame::Effects { in_reply_to, .. } => *in_reply_to,
        AppFrame::Events { in_reply_to, .. } => *in_reply_to,
        AppFrame::Error { in_reply_to, .. } => *in_reply_to,
        AppFrame::Welcome { .. } | AppFrame::DocumentChanged { .. } | AppFrame::ConfigChanged { .. } => None,
        AppFrame::Config { in_reply_to, .. } => Some(*in_reply_to),
    }
}

/// 🔑️ Decodes an `AppFrame::MediaFingerprint::fingerprint`/`MediaFingerprint`'s
/// `store::pack_rt::encode_wire_value`-encoded wire payload back into its plain string (a
/// `MediaFingerprint(String)` newtype serializes transparently, so the wire value is just a string).
fn decode_fingerprint_wire(bytes: &[u8]) -> Result<String, RunError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| RunError::Host(error.to_string()))?;
    value.as_str().map(str::to_string).ok_or_else(|| RunError::Host("media fingerprint wire value was not a string".to_string()))
}
//#endregion 🔖️MediaArtifact

//#region 🔖️MediaConverters
/// 🔀️ A pure `Media -> Media` conversion for one `(class, from, to)` triple — e.g. `(ThreeD, Brep,
/// Mesh)` for a CAD tessellator. Registered once per plugin load (Wave 2); this crate ships only the
/// registry mechanism and the `compute_node` call site — no converter bodies.
pub type MediaConvertFn = fn(&Media) -> Result<Media, RunError>;

/// 🗺️ Process-wide converter table — `LazyLock`/`Mutex` because registration happens at plugin-load
/// time (an ordinary function call, not a `SpaceRunner` method) while lookup happens per-input inside
/// `SpaceRunner::run`; both need the same shared table without threading it through every signature.
static MEDIA_CONVERTERS: LazyLock<Mutex<HashMap<(MediaClass, MediaForm, MediaForm), MediaConvertFn>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// 🔌️ Registers (or replaces) the converter for one `(class, from, to)` triple.
pub fn register_media_converter(class: MediaClass, from: MediaForm, to: MediaForm, f: MediaConvertFn) {
    MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned").insert((class, from, to), f);
}

/// 🖼️ Vector→Raster: rasterizes an SVG `Media` payload (as produced by a TwoD×Vector app's
/// `"vector:out"` port — e.g. `draw`'s `2d.drawing`, `MediaPayload::Structured { schema:
/// "2d.drawing", json: <svg text> }`) into a base64 PNG `Media` payload consumable by a TwoD×Raster
/// `"image:in"` port (e.g. `raster`'s `2d.image`) — reuses `semio_framework_os`'s
/// `rasterize_svg_to_png_base64` (the same engine `raster_engine::raster_document_json_from_dwg`'s
/// DWG-import rasterizer already uses), auto-sizing from the SVG's own intrinsic viewBox (`width: 0,
/// height: 0`). The FIRST real `MediaConvertFn` body this registry carries (WORKFLOWS-END-TO-END-
/// TYPED-PORTS Wave 2, draw+raster package) — Brep→Mesh/Design→Kit/Type→Kit remain unregistered
/// until their own owning packages land a body.
fn vector_to_raster(media: &Media) -> Result<Media, RunError> {
    let MediaPayload::Structured { json: svg, .. } = &media.payload else {
        return Err(RunError::Media(MediaError::Payload("vector-to-raster".into(), "expected a Structured (SVG text) payload".into())));
    };
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(svg, 0, 0).map_err(RunError::Host)?;
    Ok(Media {
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 },
    })
}

/// 🔌️ Registers every framework-builtin media converter this crate ships a real body for — called
/// once at process/composition-root startup (see `bin.rs::run`, right beside where plugins are
/// resolved/loaded). Wave 2 (draw+raster package) adds the first real body (Vector→Raster); later
/// waves add Brep→Mesh/Design→Kit/Type→Kit here as their owning packages land bodies for them.
pub fn register_builtin_converters() {
    register_media_converter(MediaClass::TwoD, MediaForm::Vector, MediaForm::Raster, vector_to_raster);
}

/// 🔎️ Whether a converter is registered for `(class, from, to)` — `validate_edge_kinds` calls this at
/// plan/connect time so a workflow carrying an edge nobody can actually run is rejected before any
/// node ever opens (fail-closed).
fn media_converter_registered(class: MediaClass, from: MediaForm, to: MediaForm) -> bool {
    MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned").contains_key(&(class, from, to))
}

/// 🔀️ Applies `contract.conversion` (if any) to `media` — identity passthrough when the negotiated
/// contract carries no conversion, looks up + applies the registered `MediaConvertFn` otherwise.
/// `validate_edge_kinds` already rejects a graph carrying an edge whose conversion has no registered
/// converter, so `Err(RunError::NoConverter)` here means the registry changed between validation and
/// this call (e.g. a hot-swapped plugin) — still handled correctly, just not the common path.
pub fn convert_media(contract: &MediaContract, media: Media) -> Result<Media, RunError> {
    match contract.conversion {
        None => Ok(media),
        Some((from, to)) => {
            let class = media.media_type.class;
            let converters = MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned");
            match converters.get(&(class, from, to)) {
                Some(f) => f(&media),
                None => Err(RunError::NoConverter { class, from, to }),
            }
        }
    }
}
//#endregion 🔖️MediaConverters

//#region 🔖️RunState
/// 📇️ Everything the runner remembers about one workflow node between runs: the document
/// fingerprint that produced its current outputs, the fingerprints of its inputs and outputs at that
/// time, and the fingerprint of the config it last ran with. A node is dirty iff any of these four no
/// longer match reality.
///
/// 🧮️ `document_fingerprint`/`config_fingerprint` are `framework_hash::hash_bytes` of the artifact's
/// `.spr` (op-log) bytes ALONE, not the full pack+spr — the intent (see this crate's
/// `WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE` ticket, task 4) was to hash the
/// artifact's head edit id instead of any content hash at all, but `store`'s `parse_document_pack`
/// needs the document's own concrete `Operation` type to decode far enough to reach the cursor/edit
/// list, and this crate is generic over arbitrary apps' documents — it never has that type. The `.spr`
/// bytes are the next best thing: far smaller than the full pack, and (like a head edit id) they
/// change if and only if the op-log actually changed.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRunRecord {
    pub document_fingerprint: String,
    pub input_fingerprints: BTreeMap<String, String>,
    pub output_fingerprints: BTreeMap<String, String>,
    #[serde(default)]
    pub config_fingerprint: String,
}

/// 🗄️ The runner's persisted incremental-recompute state for one studio bundle, keyed by workflow
/// node id.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunState {
    pub nodes: BTreeMap<String, NodeRunRecord>,
}

impl RunState {
    pub fn load(path: &Path) -> Result<Self, RunError> {
        match std::fs::read_to_string(path) {
            Ok(text) => Ok(serde_json::from_str(&text)?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(source) => Err(RunError::Io { path: path.to_path_buf(), source }),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), RunError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        let text = serde_json::to_string_pretty(self)?;
        std::fs::write(path, text).map_err(|source| RunError::Io { path: path.to_path_buf(), source })
    }
}
//#endregion 🔖️RunState

//#region 🔖️SpaceBundle
/// 📁️ The on-disk shape of a studio: `space.os.pack`+`space.os.spr` (the `OsDocument` VCS envelope's
/// binary pack+dsl form — see `semio_framework_os::encode_os_space_payload`), one document artifact
/// per workflow node at `<document_ref>.pack|.spr` and one config artifact per node at
/// `<config_ref>.pack|.spr` (a node's `document_ref`/`config_ref` — e.g. `documents/<node id>` /
/// `config/<node id>`, see `workflow::workflow_node_for_app` — IS the bundle-relative path stem; no
/// extra directory is joined on top of it), content-addressed blobs under `blobs/` (backing a
/// `MediaPayload::Binary` value's bytes — see `FileBlobStore`), and the runner's own `run/state.json`
/// + `run/media/` cache. Ids only — no paths inside the space document itself — so the bundle is
/// relocatable and syncs the same way over `file://` or a semio_hub backbone.
pub struct SpaceBundle {
    root: PathBuf,
}

impl SpaceBundle {
    pub fn open(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn space_document_pack_path(&self) -> PathBuf {
        self.root.join("space.os.pack")
    }

    pub fn space_document_spr_path(&self) -> PathBuf {
        self.root.join("space.os.spr")
    }

    pub fn document_pack_path(&self, document_ref: &str) -> PathBuf {
        self.root.join(format!("{document_ref}.pack"))
    }

    pub fn document_spr_path(&self, document_ref: &str) -> PathBuf {
        self.root.join(format!("{document_ref}.spr"))
    }

    /// 🔖️ Config counterpart of `document_pack_path` — same "the ref IS the relative path stem"
    /// convention (a node's `config_ref` is already e.g. `config/<node id>`).
    pub fn config_pack_path(&self, config_ref: &str) -> PathBuf {
        self.root.join(format!("{config_ref}.pack"))
    }

    pub fn config_spr_path(&self, config_ref: &str) -> PathBuf {
        self.root.join(format!("{config_ref}.spr"))
    }

    pub fn run_state_path(&self) -> PathBuf {
        self.root.join("run").join("state.json")
    }

    pub fn media_cache_dir(&self) -> PathBuf {
        self.root.join("run").join("media")
    }

    pub fn blobs_dir(&self) -> PathBuf {
        self.root.join("blobs")
    }

    /// @emoji 📦️ Reads the studio's pack+spr bytes, matching `read_document`/`read_config`'s "empty
    /// spr if never persisted" convention (a bare pack with no history is a valid fresh studio).
    pub fn read_space_document(&self) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.space_document_pack_path();
        let pack = std::fs::read(&pack_path).map_err(|source| RunError::Io { path: pack_path, source })?;
        let spr_path = self.space_document_spr_path();
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_space_document(&self, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.space_document_pack_path();
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.space_document_spr_path(), spr).map_err(|source| RunError::Io { path: self.space_document_spr_path(), source })
    }

    /// @emoji 📦️ Reads one node's document pack+spr bytes, `(Vec::new(), Vec::new())` if never persisted.
    pub fn read_document(&self, document_ref: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.document_pack_path(document_ref);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.document_spr_path(document_ref);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_document(&self, document_ref: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.document_pack_path(document_ref);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.document_spr_path(document_ref), spr).map_err(|source| RunError::Io { path: self.document_spr_path(document_ref), source })
    }

    /// @emoji 📦️ Reads one node's config pack+spr bytes — mirrors `read_document` exactly (same
    /// "never persisted" fallback, same error shape).
    pub fn read_config(&self, config_ref: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.config_pack_path(config_ref);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.config_spr_path(config_ref);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    /// @emoji 📦️ Writes one node's config pack+spr bytes — mirrors `write_document` exactly
    /// (directory-created-if-missing, same error shape).
    pub fn write_config(&self, config_ref: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.config_pack_path(config_ref);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.config_spr_path(config_ref), spr).map_err(|source| RunError::Io { path: self.config_spr_path(config_ref), source })
    }

    pub fn load_run_state(&self) -> Result<RunState, RunError> {
        RunState::load(&self.run_state_path())
    }

    pub fn save_run_state(&self, state: &RunState) -> Result<(), RunError> {
        state.save(&self.run_state_path())
    }

    pub fn media_cache(&self) -> FileMediaCache {
        FileMediaCache::new(self.media_cache_dir())
    }

    pub fn blob_store(&self) -> FileBlobStore {
        FileBlobStore::new(self.blobs_dir())
    }
}
//#endregion 🔖️SpaceBundle

//#region 🔖️Topology
/// 🔢️ Deterministic topological order (Kahn's algorithm, lexicographically-smallest-ready-node-first)
/// over `graph`'s nodes. `Err(RunError::Cycle)` names whichever nodes never became ready —
/// `workflow::validate_workflow` should be called first to reject cycles with a friendlier message;
/// this is the runner's authoritative order once that check has passed.
///
/// 🧷️ Not redundant with `workflow::plan_workflow`: that function only walks the downstream closure
/// of an already-dirty node set to emit `WorkflowDelivery` records, and its own topological helper is
/// private to the kernel crate. The runner needs a total Kahn order over EVERY node (dirty, clean, or
/// edge-less) to decide host-open/execution sequence deterministically — a genuinely different job.
fn topological_order(graph: &Workflow) -> Result<Vec<String>, RunError> {
    let mut indegree: BTreeMap<String, usize> = graph.nodes.iter().map(|node| (node.id.clone(), 0)).collect();
    let mut outgoing: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        *indegree.entry(edge.target_node_id.clone()).or_insert(0) += 1;
        outgoing.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut ready: BTreeSet<String> = indegree.iter().filter(|(_, degree)| **degree == 0).map(|(id, _)| id.clone()).collect();
    let mut order = Vec::with_capacity(graph.nodes.len());
    while let Some(node_id) = ready.iter().next().cloned() {
        ready.remove(&node_id);
        order.push(node_id.clone());
        for next in outgoing.get(&node_id).into_iter().flatten() {
            if let Some(degree) = indegree.get_mut(next) {
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(next.clone());
                }
            }
        }
    }
    if order.len() != graph.nodes.len() {
        let unreached: Vec<String> = graph.nodes.iter().map(|node| node.id.clone()).filter(|id| !order.contains(id)).collect();
        return Err(RunError::Cycle(unreached));
    }
    Ok(order)
}
//#endregion 🔖️Topology

//#region 🔖️EdgeValidation
/// ✅️ Full connect-time edge validation, called by both `plan` (the `--dry` path never opens a
/// single node) and `run`: every edge's endpoints must resolve to real ports; the producer's
/// `MediaType` must be `media_types_compatible` with the consumer's (typed rejection, no more
/// stringly `artifact_kind` equality); a negotiated `contract.conversion` must have a registered
/// converter (fail-closed — an edge nobody can actually run is worse than one caught here at plan
/// time); and every input port's `required`/`multiplicity` constraint must hold across the WHOLE
/// node, not just one edge (so it's checked in a second pass once every edge's incoming count is known).
fn validate_edge_kinds(graph: &Workflow) -> Result<(), RunError> {
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut incoming_count: HashMap<(&str, &str), usize> = HashMap::new();

    for edge in &graph.edges {
        let source_node = *node_by_id.get(edge.source_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
        let target_node = *node_by_id.get(edge.target_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.target_node_id.clone()))?;
        let source_port = source_node
            .outputs
            .iter()
            .find(|port| port.id == edge.source_port_id)
            .ok_or_else(|| RunError::UnknownPort { edge_id: edge.id.clone(), node_id: edge.source_node_id.clone(), port_id: edge.source_port_id.clone() })?;
        let target_port = target_node
            .inputs
            .iter()
            .find(|port| port.id == edge.target_port_id)
            .ok_or_else(|| RunError::UnknownPort { edge_id: edge.id.clone(), node_id: edge.target_node_id.clone(), port_id: edge.target_port_id.clone() })?;

        if matches!(media_types_compatible(&source_port.spec.media_type, &target_port.spec.media_type), MediaCompat::Reject) {
            return Err(RunError::Incompatible { edge_id: edge.id.clone(), produced: source_port.spec.media_type, accepted: target_port.spec.media_type });
        }

        if let Some((from, to)) = edge.contract.conversion {
            if !media_converter_registered(edge.contract.media_type.class, from, to) {
                return Err(RunError::UnregisteredConversion { edge_id: edge.id.clone(), from, to });
            }
        }

        *incoming_count.entry((edge.target_node_id.as_str(), edge.target_port_id.as_str())).or_insert(0) += 1;
    }

    for node in &graph.nodes {
        for port in &node.inputs {
            let count = incoming_count.get(&(node.id.as_str(), port.id.as_str())).copied().unwrap_or(0);
            if port.spec.required && count == 0 {
                return Err(RunError::MissingRequiredInput { node_id: node.id.clone(), port_id: port.id.clone() });
            }
            if matches!(port.spec.multiplicity, PortMultiplicity::One) && count > 1 {
                return Err(RunError::MultiplicityViolation { node_id: node.id.clone(), port_id: port.id.clone(), count });
            }
        }
    }
    Ok(())
}
//#endregion 🔖️EdgeValidation

//#region 🔖️SpaceRunner
/// 📊️ What actually happened in one `run()` call — which nodes were recomputed and which were left
/// untouched because neither their document, inputs, nor config changed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunReport {
    pub recomputed: Vec<String>,
    pub clean: Vec<String>,
}

/// 🩺️ Computes which nodes `SpaceRunner::run` would recompute, without instantiating a single host
/// — the `--dry` plan. Reuses exactly the dirty check `run` applies, so the plan can never drift from
/// what an actual run would do. `documents`/`configs` map a node's `document_ref`/`config_ref` string
/// to its current `(pack, spr)` artifact bytes — missing/absent means "never persisted".
pub fn plan(graph: &Workflow, documents: &BTreeMap<String, (Vec<u8>, Vec<u8>)>, configs: &BTreeMap<String, (Vec<u8>, Vec<u8>)>, state: &RunState) -> Result<RunReport, RunError> {
    validate_edge_kinds(graph)?;
    let order = topological_order(graph)?;
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut incoming: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
    for edge in &graph.edges {
        incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
    }
    let mut report = RunReport::default();
    for node_id in &order {
        let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
        let document = documents.get(&node.document_ref).cloned().unwrap_or_default();
        let document_fingerprint = framework_hash::hash_bytes(&document.1);
        let config = configs.get(&node.config_ref).cloned().unwrap_or_default();
        let config_fingerprint = framework_hash::hash_bytes(&config.1);
        let mut input_fingerprints: BTreeMap<String, String> = BTreeMap::new();
        for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
            let fingerprint = state.nodes.get(&edge.source_node_id).and_then(|record| record.output_fingerprints.get(&edge.source_port_id)).cloned().unwrap_or_default();
            input_fingerprints.insert(edge.target_port_id.clone(), fingerprint);
        }
        let dirty = match state.nodes.get(node_id.as_str()) {
            None => true,
            Some(record) => record.document_fingerprint != document_fingerprint || record.input_fingerprints != input_fingerprints || record.config_fingerprint != config_fingerprint,
        };
        if dirty {
            report.recomputed.push(node_id.clone());
        } else {
            report.clean.push(node_id.clone());
        }
    }
    Ok(report)
}

/// 🕸️ Computes one studio's workflow against an `AppChannelHost`. Node dirtiness is decided purely
/// from `NodeRunRecord`: the document's own fingerprint (did the app's document change since last
/// run — e.g. a UI edit), its resolved input fingerprints (did anything upstream change), and its
/// config's fingerprint. A clean node is never opened at all; its cached output fingerprints feed
/// straight into its consumers.
pub struct SpaceRunner<H: AppChannelHost> {
    host: H,
    blob_store: Arc<dyn BlobStore>,
}

impl<H: AppChannelHost> SpaceRunner<H> {
    pub fn new(host: H, blob_store: Arc<dyn BlobStore>) -> Self {
        Self { host, blob_store }
    }

    pub fn into_host(self) -> H {
        self.host
    }

    /// 🔌️ Returns `node`'s already-open handle, opening it (`host.open(node.plugin_id, node.app_id)`)
    /// and caching the handle in `live` on first use. Lazy by construction (unlike a plain
    /// `HashMap::entry(..).or_insert(expr)`, which would evaluate `expr` — and so call `host.open` —
    /// unconditionally even when the entry already exists).
    fn open_node(&mut self, live: &mut HashMap<String, u32>, node: &WorkflowNode) -> Result<u32, RunError> {
        if let Some(handle) = live.get(&node.id) {
            return Ok(*handle);
        }
        let handle = self.host.open(&node.plugin_id, &node.app_id)?;
        live.insert(node.id.clone(), handle);
        Ok(handle)
    }

    /// 🎬️ Runs one node's whole frame script — `Hello`, `LoadConfig`, `LoadDocument`, one `MediaIn`
    /// per resolved input, one `MediaOut`+`MediaFingerprint` pair per output port, then `ReadDocument`
    /// and finally `ReadConfig` to persist whatever the imports mutated on either artifact (see this
    /// file's header doc: "importing media is emitting operations") — as a single batched
    /// `host.exchange` call. Returns the node's mutated document bytes, its mutated config bytes, and
    /// per output port the exported `Media` plus its wire fingerprint string.
    fn compute_node(
        &mut self,
        live: &mut HashMap<String, u32>,
        node: &WorkflowNode,
        document: &(Vec<u8>, Vec<u8>),
        config: &(Vec<u8>, Vec<u8>),
        input_media: &BTreeMap<String, Media>,
    ) -> Result<((Vec<u8>, Vec<u8>), (Vec<u8>, Vec<u8>), BTreeMap<String, (Media, String)>), RunError> {
        let handle = self.open_node(live, node)?;

        let mut seq: u64 = 0;
        let mut next_seq = move || {
            seq += 1;
            seq
        };

        let mut commands = vec![AppCommand::Hello { channel_version: CHANNEL_VERSION, app_id: node.app_id.clone(), actor: "runner".to_string(), config: Vec::new() }];

        let load_config_seq = next_seq();
        commands.push(AppCommand::LoadConfig { seq: load_config_seq, pack: config.0.clone(), spr: config.1.clone() });

        let load_document_seq = next_seq();
        commands.push(AppCommand::LoadDocument { seq: load_document_seq, pack: document.0.clone(), spr: document.1.clone() });

        let mut media_in_seqs = Vec::with_capacity(input_media.len());
        for (port, media) in input_media {
            let (descriptor, data) = media_to_artifact(media, self.blob_store.as_ref())?;
            let this_seq = next_seq();
            commands.push(AppCommand::MediaIn { seq: this_seq, port: port.clone(), descriptor, data });
            media_in_seqs.push(this_seq);
        }

        let mut output_seqs = Vec::with_capacity(node.outputs.len());
        for port in &node.outputs {
            let media_out_seq = next_seq();
            commands.push(AppCommand::MediaOut { seq: media_out_seq, port: port.id.clone(), request: Vec::new() });
            let fingerprint_seq = next_seq();
            commands.push(AppCommand::MediaFingerprint { seq: fingerprint_seq, port: port.id.clone() });
            output_seqs.push((port.id.clone(), media_out_seq, fingerprint_seq));
        }

        let read_document_seq = next_seq();
        commands.push(AppCommand::ReadDocument { seq: read_document_seq });

        let read_config_seq = next_seq();
        commands.push(AppCommand::ReadConfig { seq: read_config_seq });

        let frames = self.host.exchange(handle, commands)?;

        if let Some(AppFrame::Error { code, message, .. }) = frames.iter().find(|frame| matches!(frame, AppFrame::Error { in_reply_to: None, .. })) {
            return Err(RunError::Host(format!("`{}` rejected the handshake ({code}): {message}", node.app_id)));
        }

        let reply_to = |seq: u64| -> Result<&AppFrame, RunError> {
            frames.iter().find(|frame| frame_in_reply_to(frame) == Some(seq)).ok_or_else(|| RunError::Host(format!("`{}` sent no reply to seq {seq}", node.app_id)))
        };
        let expect_done = |seq: u64, frame: &AppFrame| -> Result<(), RunError> {
            match frame {
                AppFrame::Done { .. } => Ok(()),
                AppFrame::Error { code, message, .. } => Err(RunError::Host(format!("`{}` rejected seq {seq} ({code}): {message}", node.app_id))),
                other => Err(RunError::Host(format!("`{}` sent an unexpected frame for seq {seq}: {other:?}", node.app_id))),
            }
        };

        expect_done(load_config_seq, reply_to(load_config_seq)?)?;
        expect_done(load_document_seq, reply_to(load_document_seq)?)?;
        for this_seq in &media_in_seqs {
            expect_done(*this_seq, reply_to(*this_seq)?)?;
        }

        let mut outputs = BTreeMap::new();
        for (port_id, media_out_seq, fingerprint_seq) in &output_seqs {
            let media = match reply_to(*media_out_seq)? {
                AppFrame::Media { descriptor, data, .. } => media_from_artifact(descriptor, data.clone(), self.blob_store.as_ref())?,
                AppFrame::Error { code, message, .. } => return Err(RunError::Host(format!("`{}` failed to produce media on `{port_id}` ({code}): {message}", node.app_id))),
                other => return Err(RunError::Host(format!("`{}` sent an unexpected frame for media-out `{port_id}`: {other:?}", node.app_id))),
            };
            let fingerprint = match reply_to(*fingerprint_seq)? {
                AppFrame::MediaFingerprint { fingerprint, .. } => decode_fingerprint_wire(fingerprint)?,
                AppFrame::Error { code, message, .. } => return Err(RunError::Host(format!("`{}` failed to fingerprint `{port_id}` ({code}): {message}", node.app_id))),
                other => return Err(RunError::Host(format!("`{}` sent an unexpected frame for media-fingerprint `{port_id}`: {other:?}", node.app_id))),
            };
            outputs.insert(port_id.clone(), (media, fingerprint));
        }

        let mutated_document = match reply_to(read_document_seq)? {
            AppFrame::Document { pack, spr, .. } => (pack.clone(), spr.clone()),
            AppFrame::Error { code, message, .. } => return Err(RunError::Host(format!("`{}` failed to read its document ({code}): {message}", node.app_id))),
            other => return Err(RunError::Host(format!("`{}` sent an unexpected frame reading its document: {other:?}", node.app_id))),
        };

        let mutated_config = match reply_to(read_config_seq)? {
            AppFrame::Config { pack, spr, .. } => (pack.clone(), spr.clone()),
            AppFrame::Error { code, message, .. } => return Err(RunError::Host(format!("`{}` failed to read its config ({code}): {message}", node.app_id))),
            other => return Err(RunError::Host(format!("`{}` sent an unexpected frame reading its config: {other:?}", node.app_id))),
        };

        Ok((mutated_document, mutated_config, outputs))
    }

    /// 🕸️ Runs every dirty node in `graph`'s topological order, importing media across each edge
    /// (applying `convert_media` per edge's negotiated `contract` first) and persisting mutated
    /// documents/configs back into `documents`/`configs`. Both maps key a node's `document_ref`/
    /// `config_ref` string to its current `(pack, spr)` artifact bytes; the returned maps have the
    /// input maps' same keys, updated wherever a node actually ran.
    pub fn run(
        &mut self,
        graph: &Workflow,
        documents: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
        configs: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
        state: &mut RunState,
        cache: &mut dyn MediaCache,
    ) -> Result<(BTreeMap<String, (Vec<u8>, Vec<u8>)>, BTreeMap<String, (Vec<u8>, Vec<u8>)>, RunReport), RunError> {
        validate_edge_kinds(graph)?;
        let order = topological_order(graph)?;
        let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let mut incoming: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
        for edge in &graph.edges {
            incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
        }

        let mut documents_out = documents.clone();
        let mut configs_out = configs.clone();
        let mut report = RunReport::default();
        let mut live: HashMap<String, u32> = HashMap::new();

        for node_id in &order {
            let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
            let document = documents_out.get(&node.document_ref).cloned().unwrap_or_default();
            let document_fingerprint = framework_hash::hash_bytes(&document.1);
            let config = configs_out.get(&node.config_ref).cloned().unwrap_or_default();
            let config_fingerprint = framework_hash::hash_bytes(&config.1);

            let mut input_fingerprints: BTreeMap<String, String> = BTreeMap::new();
            for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
                let source_record = state.nodes.get(&edge.source_node_id);
                let fingerprint = source_record.and_then(|record| record.output_fingerprints.get(&edge.source_port_id)).cloned().unwrap_or_default();
                input_fingerprints.insert(edge.target_port_id.clone(), fingerprint);
            }

            let previous = state.nodes.get(node_id.as_str());
            let dirty = match previous {
                None => true,
                Some(record) => record.document_fingerprint != document_fingerprint || record.input_fingerprints != input_fingerprints || record.config_fingerprint != config_fingerprint,
            };

            if !dirty {
                report.clean.push(node_id.clone());
                continue;
            }
            report.recomputed.push(node_id.clone());

            let mut input_media: BTreeMap<String, Media> = BTreeMap::new();
            for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
                let fingerprint = MediaFingerprint(input_fingerprints.get(&edge.target_port_id).cloned().unwrap_or_default());
                let media = match cache.get(&fingerprint) {
                    Some(media) => media,
                    None => {
                        // 🩹️ Defensive one-hop fallback (mirrors the pre-`AppChannelHost` runner's own
                        // behavior): a clean upstream node's output should already be in `cache` from a
                        // prior run; reaching here means it genuinely isn't (e.g. an evicted media
                        // cache dir) — recompute the source node directly, WITHOUT recursively
                        // resolving ITS OWN inputs (a clean node's inputs are, by definition, unchanged
                        // since it was last fully computed).
                        let source_node = *node_by_id.get(edge.source_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
                        let source_document = documents_out.get(&source_node.document_ref).cloned().unwrap_or_default();
                        let source_config = configs_out.get(&source_node.config_ref).cloned().unwrap_or_default();
                        let (_source_document, _source_config, source_outputs) = self.compute_node(&mut live, source_node, &source_document, &source_config, &BTreeMap::new())?;
                        let (media, _fresh_fingerprint) = source_outputs
                            .get(&edge.source_port_id)
                            .cloned()
                            .ok_or_else(|| RunError::Host(format!("upstream node `{}` produced no output on port `{}`", edge.source_node_id, edge.source_port_id)))?;
                        cache.put(&fingerprint, &media);
                        media
                    }
                };
                let converted = convert_media(&edge.contract, media)?;
                input_media.insert(edge.target_port_id.clone(), converted);
            }

            let (mutated_document, mutated_config, outputs) = self.compute_node(&mut live, node, &document, &config, &input_media)?;
            documents_out.insert(node.document_ref.clone(), mutated_document);
            configs_out.insert(node.config_ref.clone(), mutated_config);

            let mut output_fingerprints = BTreeMap::new();
            for (port_id, (media, fingerprint)) in &outputs {
                output_fingerprints.insert(port_id.clone(), fingerprint.clone());
                cache.put(&MediaFingerprint(fingerprint.clone()), media);
            }
            state.nodes.insert(node_id.clone(), NodeRunRecord { document_fingerprint, input_fingerprints, output_fingerprints, config_fingerprint });
        }

        Ok((documents_out, configs_out, report))
    }
}
//#endregion 🔖️SpaceRunner

//#region 🔖️WasmtimeNodeHost
/// 🧩️ Native `AppChannelHost` over `semio-framework-plugin-host`'s wasmtime runtime — `open` lazily
/// loads a `WasmPluginRuntime` per plugin id (via `plugin_path_for_plugin`, resolved from the plugin
/// registry's generated `PLUGIN_WASM_ARTIFACTS` — see `bin.rs`), registering `blob_store` on it so a
/// guest's `write-blob`/`read-blob` host calls resolve, then calls `create_app`; `exchange` is a thin
/// binary encode/decode shim over `WasmPluginRuntime::exchange` — every former per-verb call
/// (`handle-action`, `handle-command`, `update-window`, `refresh-ui`, `context-menu`,
/// `apply-operations[-text]`, `read/load-app-document-{text,pack}`, `attach/detach-backbone`,
/// `consume/produce-media`) is now just a caller-encoded `AppCommand` batch on this one WIT call.
#[cfg(not(target_arch = "wasm32"))]
pub struct WasmtimeNodeHost {
    runtimes: HashMap<String, semio_framework_plugin_host::WasmPluginRuntime>,
    plugin_path_for_plugin: HashMap<String, PathBuf>,
    blob_store: Arc<dyn BlobStore>,
    next_handle: u32,
    instances: HashMap<u32, (String, u32)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl WasmtimeNodeHost {
    /// 🗺️ `plugin_path_for_plugin` maps a plugin id (`WorkflowNode::plugin_id`, the same id
    /// `PLUGIN_WASM_ARTIFACTS`' first tuple element names) to the compiled `.wasm` component path the
    /// dev shell build already produces under `framework/os/dev/plugin-modules/<plugin id>/`.
    pub fn new(plugin_path_for_plugin: HashMap<String, PathBuf>, blob_store: Arc<dyn BlobStore>) -> Self {
        Self { runtimes: HashMap::new(), plugin_path_for_plugin, blob_store, next_handle: 1, instances: HashMap::new() }
    }

    fn runtime_for(&mut self, plugin_id: &str) -> Result<&semio_framework_plugin_host::WasmPluginRuntime, RunError> {
        if !self.runtimes.contains_key(plugin_id) {
            let path = self.plugin_path_for_plugin.get(plugin_id).ok_or_else(|| RunError::Host(format!("no compiled program registered for plugin `{plugin_id}`")))?;
            let runtime = semio_framework_plugin_host::WasmPluginRuntime::load(path).map_err(|error| RunError::Host(error.to_string()))?;
            runtime.register_host_blob_store(Arc::clone(&self.blob_store)).map_err(|error| RunError::Host(error.to_string()))?;
            self.runtimes.insert(plugin_id.to_string(), runtime);
        }
        Ok(self.runtimes.get(plugin_id).expect("just inserted"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl AppChannelHost for WasmtimeNodeHost {
    fn open(&mut self, plugin_id: &str, app_id: &str) -> Result<u32, RunError> {
        let instance_id = self.runtime_for(plugin_id)?.create_app(app_id).map_err(|error| RunError::Host(error.to_string()))?;
        let handle = self.next_handle;
        self.next_handle += 1;
        self.instances.insert(handle, (plugin_id.to_string(), instance_id));
        Ok(handle)
    }

    fn exchange(&mut self, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
        let (plugin_id, instance_id) = self.instances.get(&node).ok_or_else(|| RunError::Host(format!("unknown node handle {node}")))?;
        let encoded: Vec<Vec<u8>> = commands.iter().map(protocol::encode_app_command).collect();
        let runtime = self.runtimes.get(plugin_id).ok_or_else(|| RunError::Host(format!("no runtime for plugin `{plugin_id}`")))?;
        let response = runtime.exchange(*instance_id, encoded).map_err(|error| RunError::Host(error.to_string()))?;
        response.iter().map(|bytes| protocol::decode_app_frame(bytes).map_err(|error| RunError::Host(error.to_string()))).collect()
    }
}
//#endregion 🔖️WasmtimeNodeHost

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_core::{MediaPortDirection, MediaPortSpec};
    use workflow::{placeholder_media_contract, WorkflowMediaPort, WORKFLOW_SCHEMA};

    /// 🧪️ A fake `AppChannelHost` for tests: no wasm at all, just a per-instance document/config, a
    /// fixed structured output per port, and an in-process `InMemoryBlobStore` — enough to interpret
    /// the exact `AppCommand`/`AppFrame` frame script `SpaceRunner::compute_node` sends, so
    /// `SpaceRunner`'s dirty/clean bookkeeping can be exercised without a real program.
    /// 🧪️ Outputs are keyed by app id, not by handle — a real app's export is a function of its
    /// document/logic, not of the ephemeral instance handle a host happens to mint this call, and a
    /// node genuinely does get re-opened (a fresh handle) on every dirty re-run.
    #[derive(Default)]
    struct FakeHost {
        documents: HashMap<u32, (Vec<u8>, Vec<u8>)>,
        configs: HashMap<u32, (Vec<u8>, Vec<u8>)>,
        handle_app: HashMap<u32, String>,
        outputs: HashMap<(String, String), Media>,
        blob_store: InMemoryBlobStore,
        next: u32,
        imported: Vec<(u32, String, Media)>,
    }

    impl FakeHost {
        fn set_output(&mut self, app_id: &str, port: &str, json: &str) {
            self.outputs.insert((app_id.to_string(), port.to_string()), Media { media_type: fake_media_type(), payload: MediaPayload::Structured { schema: "test".into(), json: json.into() } });
        }
    }

    fn fake_media_type() -> MediaType {
        MediaType { class: MediaClass::Data, form: MediaForm::Value }
    }

    impl AppChannelHost for FakeHost {
        fn open(&mut self, _plugin_id: &str, app_id: &str) -> Result<u32, RunError> {
            self.next += 1;
            self.handle_app.insert(self.next, app_id.to_string());
            Ok(self.next)
        }

        fn exchange(&mut self, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
            let app_id = self.handle_app.get(&node).cloned().unwrap_or_default();
            let mut frames = Vec::new();
            for command in commands {
                match command {
                    AppCommand::Hello { channel_version, .. } => {
                        if channel_version != CHANNEL_VERSION {
                            frames.push(AppFrame::Error { in_reply_to: None, code: "channel-version".into(), message: "mismatched channel version".into() });
                            continue;
                        }
                        frames.push(AppFrame::Welcome { channel_version: CHANNEL_VERSION, instance: node, manifest: Vec::new() });
                    }
                    AppCommand::LoadConfig { seq, pack, spr } => {
                        self.configs.insert(node, (pack, spr));
                        frames.push(AppFrame::Done { in_reply_to: seq });
                    }
                    AppCommand::LoadDocument { seq, pack, spr } => {
                        self.documents.insert(node, (pack, spr));
                        frames.push(AppFrame::Done { in_reply_to: seq });
                    }
                    AppCommand::MediaIn { seq, port, descriptor, data } => match media_from_artifact(&descriptor, data, &self.blob_store) {
                        Ok(media) => {
                            self.imported.push((node, port, media));
                            frames.push(AppFrame::Done { in_reply_to: seq });
                        }
                        Err(error) => frames.push(AppFrame::Error { in_reply_to: Some(seq), code: "handler".into(), message: error.to_string() }),
                    },
                    AppCommand::MediaOut { seq, port, .. } => match self.outputs.get(&(app_id.clone(), port.clone())) {
                        Some(media) => match media_to_artifact(media, &self.blob_store) {
                            Ok((descriptor, data)) => frames.push(AppFrame::Media { in_reply_to: seq, port, descriptor, data }),
                            Err(error) => frames.push(AppFrame::Error { in_reply_to: Some(seq), code: "handler".into(), message: error.to_string() }),
                        },
                        None => frames.push(AppFrame::Error { in_reply_to: Some(seq), code: "handler".into(), message: "no output".into() }),
                    },
                    AppCommand::MediaFingerprint { seq, port } => match self.outputs.get(&(app_id.clone(), port)) {
                        Some(media) => {
                            let fingerprint = MediaFingerprint::of(media);
                            // 🩹️ Pre-existing bug fixed in passing: `store::pack_rt::encode_wire_value` takes
                            // `&dsl::DslValue`, not `&serde_json::Value` — `decode_fingerprint_wire` (this
                            // file) already expects the `DslValue`-transparent-string encoding `to_dsl_value`
                            // produces for a `MediaFingerprint(String)` newtype.
                            let value = to_dsl_value(&fingerprint).unwrap_or(dsl::DslValue::Null);
                            frames.push(AppFrame::MediaFingerprint { in_reply_to: seq, port: String::new(), fingerprint: store::pack_rt::encode_wire_value(&value) });
                        }
                        None => frames.push(AppFrame::Error { in_reply_to: Some(seq), code: "handler".into(), message: "no output".into() }),
                    },
                    AppCommand::ReadDocument { seq } => {
                        let (pack, spr) = self.documents.get(&node).cloned().unwrap_or_default();
                        frames.push(AppFrame::Document { in_reply_to: seq, pack, spr, ops: String::new() });
                    }
                    AppCommand::ReadConfig { seq } => {
                        let (pack, spr) = self.configs.get(&node).cloned().unwrap_or_default();
                        frames.push(AppFrame::Config { in_reply_to: seq, pack, spr, ops: String::new() });
                    }
                    _ => {}
                }
            }
            Ok(frames)
        }
    }

    fn media_port(node_id: &str, spec_id: &str, direction: MediaPortDirection, kind_id: &str, multiplicity: PortMultiplicity, required: bool) -> WorkflowMediaPort {
        let direction_word = match direction {
            MediaPortDirection::In => "in",
            MediaPortDirection::Out => "out",
        };
        WorkflowMediaPort {
            id: format!("{node_id}:{spec_id}:{direction_word}"),
            spec: MediaPortSpec { id: spec_id.into(), label: spec_id.into(), direction, media_type: fake_media_type(), kind_id: Some(kind_id.into()), required, multiplicity },
        }
    }

    fn workflow_node(id: &str, outputs: Vec<WorkflowMediaPort>, inputs: Vec<WorkflowMediaPort>) -> WorkflowNode {
        WorkflowNode {
            id: id.into(),
            plugin_id: "program".into(),
            app_id: format!("app-{id}"),
            label: id.into(),
            yields: String::new(),
            document_ref: format!("documents/{id}"),
            config_ref: format!("config/{id}"),
            x: 0.0,
            y: 0.0,
            width: 220.0,
            height: 100.0,
            inputs,
            outputs,
        }
    }

    fn two_node_graph() -> Workflow {
        let source = workflow_node("node-a", vec![media_port("node-a", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
        let target = workflow_node("node-b", Vec::new(), vec![media_port("node-b", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, true)]);
        let edge = WorkflowEdge { id: "edge-1".into(), source_node_id: "node-a".into(), source_port_id: "node-a:out:out".into(), target_node_id: "node-b".into(), target_port_id: "node-b:in:in".into(), contract: placeholder_media_contract("data.value") };
        Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![source, target], edges: vec![edge] }
    }

    fn empty_documents(graph: &Workflow) -> BTreeMap<String, (Vec<u8>, Vec<u8>)> {
        graph.nodes.iter().map(|node| (node.document_ref.clone(), (Vec::new(), Vec::new()))).collect()
    }

    fn empty_configs(graph: &Workflow) -> BTreeMap<String, (Vec<u8>, Vec<u8>)> {
        graph.nodes.iter().map(|node| (node.config_ref.clone(), (Vec::new(), Vec::new()))).collect()
    }

    #[test]
    fn topological_order_respects_edges() {
        let graph = two_node_graph();
        let order = topological_order(&graph).expect("acyclic");
        assert_eq!(order, vec!["node-a".to_string(), "node-b".to_string()]);
    }

    #[test]
    fn detects_cycles() {
        let mut graph = two_node_graph();
        graph.edges.push(WorkflowEdge { id: "edge-2".into(), source_node_id: "node-b".into(), source_port_id: "b-out".into(), target_node_id: "node-a".into(), target_port_id: "a-in".into(), contract: placeholder_media_contract("data.value") });
        assert!(matches!(topological_order(&graph), Err(RunError::Cycle(_))));
    }

    #[test]
    fn first_run_recomputes_every_node_second_run_is_a_no_operation() {
        let graph = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()));
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);

        let (documents_1, configs_1, report_1) = runner.run(&graph, &documents, &configs, &mut state, &mut cache).expect("first run");
        assert_eq!(report_1.recomputed, vec!["node-a".to_string(), "node-b".to_string()]);
        assert!(report_1.clean.is_empty());

        let (_, _, report_2) = runner.run(&graph, &documents_1, &configs_1, &mut state, &mut cache).expect("second run");
        assert!(report_2.recomputed.is_empty(), "unchanged documents must not re-trigger recompute: {:?}", report_2.recomputed);
        assert_eq!(report_2.clean, vec!["node-a".to_string(), "node-b".to_string()]);
    }

    #[test]
    fn editing_upstream_document_dirties_downstream_only_through_the_wire() {
        let graph = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()));
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);
        let (documents_1, _, _) = runner.run(&graph, &documents, &configs, &mut state, &mut cache).expect("first run");

        let mut documents_2 = documents_1;
        // 🧮️ Fingerprints are now `.spr`-bytes hashes (see `NodeRunRecord`'s doc) — editing the op log
        // (`spr`), not the pack, is what must dirty the node.
        documents_2.insert("documents/node-a".to_string(), (Vec::new(), b"edited".to_vec()));
        let (_, _, report_2) = runner.run(&graph, &documents_2, &configs, &mut state, &mut cache).expect("second run");
        assert_eq!(report_2.recomputed, vec!["node-a".to_string()], "node-a's own document changed, so node-a must recompute");
        assert_eq!(report_2.clean, vec!["node-b".to_string()], "node-a's FakeHost output is fixed, so its output fingerprint is unchanged — node-b must stay clean (the early-cutoff this whole design exists for)");
    }

    /// 🧪️ Changing a node's own effective config — document and resolved inputs held constant — must
    /// dirty exactly that node on the very next `plan()`/`run()`, mirroring
    /// `editing_upstream_document_dirties_downstream_only_through_the_wire`'s shape but on the config
    /// dimension instead of the document one.
    #[test]
    fn changing_a_nodes_config_alone_dirties_it_without_touching_document_or_inputs() {
        let graph = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()));
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents = empty_documents(&graph);
        let configs_1 = empty_configs(&graph);
        runner.run(&graph, &documents, &configs_1, &mut state, &mut cache).expect("first run");

        let plan_unchanged = plan(&graph, &documents, &configs_1, &state).expect("plan with unchanged config");
        assert!(plan_unchanged.recomputed.is_empty(), "nothing changed, plan must report every node clean: {:?}", plan_unchanged.recomputed);

        let mut configs_2 = configs_1.clone();
        configs_2.insert("config/node-a".to_string(), (Vec::new(), b"threshold=2".to_vec()));
        let plan_changed = plan(&graph, &documents, &configs_2, &state).expect("plan with changed config");
        assert_eq!(plan_changed.recomputed, vec!["node-a".to_string()], "only node-a's own config changed, so only node-a should be recomputed by the plan");

        let (_, _, report_2) = runner.run(&graph, &documents, &configs_2, &mut state, &mut cache).expect("second run with changed config");
        assert_eq!(report_2.recomputed, vec!["node-a".to_string()], "node-a's config changed, so node-a must recompute even though its document and inputs did not");
        assert_eq!(report_2.clean, vec!["node-b".to_string()], "node-a's FakeHost output is fixed regardless of config, so node-b must stay clean");
    }

    #[test]
    fn rejects_incompatible_edge_media_types() {
        let mut graph = two_node_graph();
        graph.nodes[1].inputs[0].spec.media_type = MediaType { class: MediaClass::Text, form: MediaForm::Document };
        let host = FakeHost::default();
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()));
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);
        let result = runner.run(&graph, &documents, &configs, &mut state, &mut cache);
        assert!(matches!(result, Err(RunError::Incompatible { .. })));
    }

    #[test]
    fn validate_rejects_missing_required_input() {
        let node = workflow_node("solo", Vec::new(), vec![media_port("solo", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, true)]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node], edges: Vec::new() };
        assert!(matches!(validate_edge_kinds(&graph), Err(RunError::MissingRequiredInput { .. })));
    }

    #[test]
    fn validate_rejects_multiplicity_one_input_with_two_incoming_edges() {
        let source_a = workflow_node("src-a", vec![media_port("src-a", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
        let source_b = workflow_node("src-b", vec![media_port("src-b", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
        let target = workflow_node("target", Vec::new(), vec![media_port("target", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, false)]);
        let graph = Workflow {
            schema: WORKFLOW_SCHEMA.into(),
            nodes: vec![source_a, source_b, target],
            edges: vec![
                WorkflowEdge { id: "e1".into(), source_node_id: "src-a".into(), source_port_id: "src-a:out:out".into(), target_node_id: "target".into(), target_port_id: "target:in:in".into(), contract: placeholder_media_contract("data.value") },
                WorkflowEdge { id: "e2".into(), source_node_id: "src-b".into(), source_port_id: "src-b:out:out".into(), target_node_id: "target".into(), target_port_id: "target:in:in".into(), contract: placeholder_media_contract("data.value") },
            ],
        };
        assert!(matches!(validate_edge_kinds(&graph), Err(RunError::MultiplicityViolation { .. })));
    }

    #[test]
    fn validate_rejects_unregistered_conversion() {
        let source = workflow_node("src", vec![media_port("src", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
        let target = workflow_node("dst", Vec::new(), vec![media_port("dst", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, false)]);
        let mut contract = placeholder_media_contract("data.value");
        // 🧪️ A conversion form pair this test file never registers a converter for — distinct from
        // any pair the `media_converter_registry_*` tests below register, so the global
        // `MEDIA_CONVERTERS` table (shared across all tests in this process) can't race with this one.
        contract.conversion = Some((MediaForm::Trinity, MediaForm::Dag));
        let graph = Workflow {
            schema: WORKFLOW_SCHEMA.into(),
            nodes: vec![source, target],
            edges: vec![WorkflowEdge { id: "e1".into(), source_node_id: "src".into(), source_port_id: "src:out:out".into(), target_node_id: "dst".into(), target_port_id: "dst:in:in".into(), contract }],
        };
        assert!(matches!(validate_edge_kinds(&graph), Err(RunError::UnregisteredConversion { .. })));
    }

    #[test]
    fn convert_media_is_identity_when_contract_has_no_conversion() {
        let contract = placeholder_media_contract("data.value");
        let media = Media { media_type: fake_media_type(), payload: MediaPayload::Structured { schema: "test".into(), json: "\"hi\"".into() } };
        let converted = convert_media(&contract, media.clone()).expect("identity conversion never fails");
        assert_eq!(converted, media);
    }

    #[test]
    fn media_converter_registry_applies_registered_converter() {
        // 🧪️ A `(class, from, to)` triple unique to this test (see `validate_rejects_unregistered_
        // conversion`'s note on the shared global table).
        register_media_converter(MediaClass::Kit, MediaForm::Design, MediaForm::Sequence, |media| {
            Ok(Media { media_type: media.media_type, payload: MediaPayload::Structured { schema: "converted".into(), json: "\"converted\"".into() } })
        });
        let mut contract = placeholder_media_contract("kit.design");
        contract.media_type = MediaType { class: MediaClass::Kit, form: MediaForm::Sequence };
        contract.conversion = Some((MediaForm::Design, MediaForm::Sequence));
        let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design }, payload: MediaPayload::Structured { schema: "design".into(), json: "\"raw\"".into() } };
        let converted = convert_media(&contract, media).expect("registered converter applies");
        assert_eq!(converted.payload, MediaPayload::Structured { schema: "converted".into(), json: "\"converted\"".into() });
    }

    #[test]
    fn vector_to_raster_rasterizes_svg_to_a_2d_image_media() {
        let svg = Media {
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 4 4" width="4" height="4"><rect width="4" height="4" fill="#ff0000"/></svg>"##.into() },
        };
        let raster = vector_to_raster(&svg).expect("rasterizes real svg text");
        assert_eq!(raster.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Raster });
        let MediaPayload::Structured { schema, json } = raster.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty(), "png_base64 must be non-empty for real svg content");
    }

    #[test]
    fn vector_to_raster_rejects_non_structured_payload() {
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Binary { format: semio_framework_core::OsMediaFormat::Png, blob_hash: "hash".into() } };
        assert!(vector_to_raster(&media).is_err());
    }

    #[test]
    fn register_builtin_converters_wires_vector_to_raster_through_convert_media() {
        register_builtin_converters();
        let mut contract = placeholder_media_contract("2d.drawing");
        contract.media_type = MediaType { class: MediaClass::TwoD, form: MediaForm::Raster };
        contract.conversion = Some((MediaForm::Vector, MediaForm::Raster));
        let media = Media {
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2 2" width="2" height="2"><rect width="2" height="2" fill="#00ff00"/></svg>"##.into() },
        };
        let converted = convert_media(&contract, media).expect("builtin vector->raster converter is registered");
        assert_eq!(converted.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Raster });
    }
}
//#endregion 🔖️Tests
