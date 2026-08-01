//! 🕸️ Headless computation of an OS studio's workflow — no UI involved. A `SpaceRunner` walks
//! `OsWorkflow` in topological order, drives each node's app through `AppChannelHost` — the exact
//! `protocol::AppCommand`/`AppFrame` binary channel a live UI speaks, so a headless run never needs a
//! UI-mock API — moves `Media` along edges, and skips any node whose inputs, document, and effective
//! config are all unchanged since the last run.
//! Importing media is emitting operations: a headless run is an ordinary editing session (actor `runner`)
//! recorded in each app document's own VCS envelope, so a later UI open sees it as normal history.

//#region 🔖️Types
/// 🎞️ The exact binary channel a live UI speaks — re-exported so an `AppChannelHost` implementor
/// never needs a direct `protocol` dependency just to name these types.
pub use protocol::{AppCommand, AppFrame, CHANNEL_VERSION};
use semio_framework_core::{Media, MediaError, MediaFingerprint, MediaPayload, MediaWireFormat};
use semio_framework_os::{OsAppInstance, OsWorkflow, OsWorkflowNode};
use store::{decode_document_pack_bytes, encode_document_pack_bytes, BlobStore};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// 🚧️ A failure computing a studio's workflow headlessly.
#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("unknown workflow node {0}")]
    UnknownNode(String),
    #[error("unknown app instance {0}")]
    UnknownInstance(String),
    #[error("workflow edge {edge_id} type mismatch: producer is `{produced}`, consumer accepts `{accepted}`")]
    Incompatible { edge_id: String, produced: String, accepted: String },
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
    let descriptor_value = serde_json::to_value(&descriptor)?;
    Ok((store::pack_rt::encode_wire_value(&descriptor_value), data))
}

/// 🔁️ Inverse of [`media_to_artifact`]. A `Binary` wire artifact's `data` is written into
/// `blob_store` (content-addressed, idempotent) rather than kept inline, mirroring `Media`'s own
/// "binary payloads never carry bytes directly" invariant — the freshly computed hash supersedes
/// whatever `blob_hash` the artifact's own descriptor claimed.
pub fn media_from_artifact(descriptor: &[u8], data: Vec<u8>, blob_store: &dyn BlobStore) -> Result<Media, RunError> {
    let value = store::pack_rt::decode_wire_value(descriptor).map_err(|error| RunError::Host(error.to_string()))?;
    let descriptor: semio_framework_plugin::app::MediaArtifactDescriptor = serde_json::from_value(value)?;
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
/// unsolicited/handshake shapes (`Welcome`, `DocumentChanged`) that never carry one.
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
        AppFrame::Welcome { .. } | AppFrame::DocumentChanged { .. } => None,
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

//#region 🔖️RunState
/// 📇️ Everything the runner remembers about one workflow node between runs: the document
/// fingerprint that produced its current outputs, the fingerprints of its inputs and outputs at that
/// time, and the fingerprint of the effective config it last ran with. A node is dirty iff any of
/// these four no longer match reality.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRunRecord {
    pub document_fingerprint: String,
    pub input_fingerprints: BTreeMap<String, String>,
    pub output_fingerprints: BTreeMap<String, String>,
    /// 🧮️ Hash of the node's effective config bytes as of its last run. `#[serde(default)]` so a
    /// `run/state.json` written before this field existed just deserializes to `""` — since no config
    /// bytes anywhere hash to `""` (see `framework_hash::hash_bytes`), every pre-existing record reads
    /// back as config-dirty exactly once, which is the correct conservative behavior (no state files
    /// exist in practice yet, so this never actually fires — see this crate's `HEADLESS-RUNNER-AND-
    /// WORKFLOW-CONFIG-MODEL` ticket for the call).
    #[serde(default)]
    pub config_fingerprint: String,
}

/// 🗄️ The runner's persisted incremental-recompute state for one studio bundle, keyed by workflow
/// node id (not instance id — a node's record is tied to its position in the graph).
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
/// binary pack+dsl form — see `semio_framework_os::encode_os_space_payload`), one plain document per
/// app instance under `documents/`, content-addressed blobs under `blobs/` (backing a
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

    pub fn document_pack_path(&self, document_id: &str) -> PathBuf {
        self.root.join("documents").join(format!("{document_id}.pack"))
    }

    pub fn document_spr_path(&self, document_id: &str) -> PathBuf {
        self.root.join("documents").join(format!("{document_id}.spr"))
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

    /// @emoji 📦️ Reads the studio's pack+spr bytes, matching the per-instance `document_pack_path`/
    /// `document_spr_path`'s "empty spr if never persisted" convention (a bare pack with no history
    /// is a valid fresh studio).
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

    /// @emoji 📦️ Reads one app instance's pack+spr bytes, `(Vec::new(), Vec::new())` if never persisted.
    pub fn read_document(&self, document_id: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.document_pack_path(document_id);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.document_spr_path(document_id);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_document(&self, document_id: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.document_pack_path(document_id);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.document_spr_path(document_id), spr).map_err(|source| RunError::Io { path: self.document_spr_path(document_id), source })
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
/// over `graph`'s nodes. `Err(RunError::Cycle)` names whichever nodes never became ready — the media
/// graph's own `validate_workflow` should be called first to reject cycles with a friendlier
/// message; this is the runner's authoritative order once that check has passed.
fn topological_order(graph: &OsWorkflow) -> Result<Vec<String>, RunError> {
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

//#region 🔖️SpaceRunner
/// 📊️ What actually happened in one `run()` call — which nodes were recomputed and which were left
/// untouched because neither their document, inputs, nor config changed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunReport {
    pub recomputed: Vec<String>,
    pub clean: Vec<String>,
}

/// 🩺️ Computes which nodes `SpaceRunner::run` would recompute, without instantiating a single host
/// — the `--dry` plan. Reuses exactly the dirty check `run` applies, so the plan can never drift
/// from what an actual run would do. `configs` maps app-instance id → effective config bytes, same
/// keying as `documents` (empty/missing means "no config").
pub fn plan(graph: &OsWorkflow, documents: &BTreeMap<String, Vec<u8>>, configs: &BTreeMap<String, Vec<u8>>, state: &RunState) -> Result<RunReport, RunError> {
    SpaceRunner::<NullHost>::validate_edge_kinds(graph)?;
    let order = topological_order(graph)?;
    let node_by_id: HashMap<&str, &OsWorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut incoming: HashMap<&str, Vec<&semio_framework_os::OsWorkflowEdge>> = HashMap::new();
    for edge in &graph.edges {
        incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
    }
    let mut report = RunReport::default();
    for node_id in &order {
        let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
        let document_bytes = documents.get(&node.instance_id).cloned().unwrap_or_default();
        let document_fingerprint = framework_hash::hash_bytes(&document_bytes);
        let config_bytes = configs.get(&node.instance_id).cloned().unwrap_or_default();
        let config_fingerprint = framework_hash::hash_bytes(&config_bytes);
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

/// 🚫️ An `AppChannelHost` that always errors — only ever used as `plan`'s unreachable type
/// parameter so it can call `SpaceRunner`'s edge-validation helper without needing a real host.
pub struct NullHost;
impl AppChannelHost for NullHost {
    fn open(&mut self, _plugin_id: &str, _app_id: &str) -> Result<u32, RunError> {
        Err(RunError::Host("NullHost never opens".into()))
    }
    fn exchange(&mut self, _node: u32, _commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
        Err(RunError::Host("NullHost never exchanges".into()))
    }
}

/// 🕸️ Computes one studio's workflow against an `AppChannelHost`. Node dirtiness is decided purely
/// from `NodeRunRecord`: the document's own fingerprint (did the app's document change since last
/// run — e.g. a UI edit), its resolved input fingerprints (did anything upstream change), and its
/// effective config's fingerprint. A clean node is never opened at all; its cached output
/// fingerprints feed straight into its consumers.
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

    /// 🩹️ Baseline wire-compatibility check: plain `artifact_kind` string equality. `OsMediaPort`
    /// doesn't carry a typed `MediaType` yet (that unification is a separate, concurrently in-flight
    /// ticket) — once it does, this is where `media_types_compatible` conversion-insertion lands.
    fn validate_edge_kinds(graph: &OsWorkflow) -> Result<(), RunError> {
        let node_by_id: HashMap<&str, &OsWorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        for edge in &graph.edges {
            let produced = node_by_id
                .get(edge.source_node_id.as_str())
                .and_then(|node| node.outputs.iter().find(|port| port.id == edge.source_port_id))
                .ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
            let accepted = node_by_id
                .get(edge.target_node_id.as_str())
                .and_then(|node| node.inputs.iter().find(|port| port.id == edge.target_port_id))
                .ok_or_else(|| RunError::UnknownNode(edge.target_node_id.clone()))?;
            if produced.artifact_kind != accepted.artifact_kind {
                return Err(RunError::Incompatible { edge_id: edge.id.clone(), produced: produced.artifact_kind.clone(), accepted: accepted.artifact_kind.clone() });
            }
        }
        Ok(())
    }

    /// 🔌️ Returns node_id's already-open handle, opening it (`host.open(plugin_id, app_id)`) and
    /// caching the handle in `live` on first use. Lazy by construction (unlike a plain
    /// `HashMap::entry(..).or_insert(expr)`, which would evaluate `expr` — and so call `host.open`
    /// — unconditionally even when the entry already exists).
    fn open_node(&mut self, live: &mut HashMap<String, u32>, node_id: &str, instance: &OsAppInstance) -> Result<u32, RunError> {
        if let Some(handle) = live.get(node_id) {
            return Ok(*handle);
        }
        let handle = self.host.open(&instance.plugin_id, &instance.app_id)?;
        live.insert(node_id.to_string(), handle);
        Ok(handle)
    }

    /// 🎬️ Runs one node's whole frame script — `Hello`, an optional `Configure`, `LoadDocument`, one
    /// `MediaIn` per resolved input, one `MediaOut`+`MediaFingerprint` pair per output port, then
    /// `ReadDocument` to persist whatever the imports mutated (see this file's header doc: "importing
    /// media is emitting operations") — as a single batched `host.exchange` call. Returns the node's
    /// mutated document bytes plus, per output port, the exported `Media` and its wire fingerprint
    /// string.
    fn compute_node(
        &mut self,
        live: &mut HashMap<String, u32>,
        node: &OsWorkflowNode,
        instance: &OsAppInstance,
        document_bytes: &[u8],
        config_bytes: &[u8],
        input_media: &BTreeMap<String, Media>,
    ) -> Result<(Vec<u8>, BTreeMap<String, (Media, String)>), RunError> {
        let handle = self.open_node(live, &node.id, instance)?;
        let (document_pack, document_spr) = decode_document_pack_bytes(document_bytes).map_err(|error| RunError::Host(error.to_string())).unwrap_or_default();

        let mut seq: u64 = 0;
        let mut next_seq = move || {
            seq += 1;
            seq
        };

        let mut commands = vec![AppCommand::Hello { channel_version: CHANNEL_VERSION, app_id: instance.app_id.clone(), actor: "runner".to_string(), config: config_bytes.to_vec() }];

        let configure_seq = if config_bytes.is_empty() {
            None
        } else {
            let this_seq = next_seq();
            commands.push(AppCommand::Configure { seq: this_seq, config: config_bytes.to_vec() });
            Some(this_seq)
        };

        let load_seq = next_seq();
        commands.push(AppCommand::LoadDocument { seq: load_seq, pack: document_pack, spr: document_spr });

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

        let read_seq = next_seq();
        commands.push(AppCommand::ReadDocument { seq: read_seq });

        let frames = self.host.exchange(handle, commands)?;

        if let Some(AppFrame::Error { code, message, .. }) = frames.iter().find(|frame| matches!(frame, AppFrame::Error { in_reply_to: None, .. })) {
            return Err(RunError::Host(format!("`{}` rejected the handshake ({code}): {message}", instance.app_id)));
        }

        let reply_to = |seq: u64| -> Result<&AppFrame, RunError> {
            frames.iter().find(|frame| frame_in_reply_to(frame) == Some(seq)).ok_or_else(|| RunError::Host(format!("`{}` sent no reply to seq {seq}", instance.app_id)))
        };
        let expect_done = |seq: u64, frame: &AppFrame| -> Result<(), RunError> {
            match frame {
                AppFrame::Done { .. } => Ok(()),
                AppFrame::Error { code, message, .. } => Err(RunError::Host(format!("`{}` rejected seq {seq} ({code}): {message}", instance.app_id))),
                other => Err(RunError::Host(format!("`{}` sent an unexpected frame for seq {seq}: {other:?}", instance.app_id))),
            }
        };

        if let Some(this_seq) = configure_seq {
            expect_done(this_seq, reply_to(this_seq)?)?;
        }
        expect_done(load_seq, reply_to(load_seq)?)?;
        for this_seq in &media_in_seqs {
            expect_done(*this_seq, reply_to(*this_seq)?)?;
        }

        let mut outputs = BTreeMap::new();
        for (port_id, media_out_seq, fingerprint_seq) in &output_seqs {
            let media = match reply_to(*media_out_seq)? {
                AppFrame::Media { descriptor, data, .. } => media_from_artifact(descriptor, data.clone(), self.blob_store.as_ref())?,
                AppFrame::Error { code, message, .. } => return Err(RunError::Host(format!("`{}` failed to produce media on `{port_id}` ({code}): {message}", instance.app_id))),
                other => return Err(RunError::Host(format!("`{}` sent an unexpected frame for media-out `{port_id}`: {other:?}", instance.app_id))),
            };
            let fingerprint = match reply_to(*fingerprint_seq)? {
                AppFrame::MediaFingerprint { fingerprint, .. } => decode_fingerprint_wire(fingerprint)?,
                AppFrame::Error { code, message, .. } => return Err(RunError::Host(format!("`{}` failed to fingerprint `{port_id}` ({code}): {message}", instance.app_id))),
                other => return Err(RunError::Host(format!("`{}` sent an unexpected frame for media-fingerprint `{port_id}`: {other:?}", instance.app_id))),
            };
            outputs.insert(port_id.clone(), (media, fingerprint));
        }

        let mutated_document = match reply_to(read_seq)? {
            AppFrame::Document { pack, spr, .. } => encode_document_pack_bytes(pack, spr),
            AppFrame::Error { code, message, .. } => return Err(RunError::Host(format!("`{}` failed to read its document ({code}): {message}", instance.app_id))),
            other => return Err(RunError::Host(format!("`{}` sent an unexpected frame reading its document: {other:?}", instance.app_id))),
        };

        Ok((mutated_document, outputs))
    }

    /// 🕸️ Runs every dirty node in `graph`'s topological order, importing media across each edge and
    /// persisting mutated documents back into `documents`. `documents`/`configs` map app-instance id
    /// → current document pack+spr bytes (`store::encode_document_pack_bytes`) / effective config
    /// bytes (empty/missing means "no config"); the returned map has `documents`'s same keys, updated
    /// wherever a node actually ran.
    pub fn run(
        &mut self,
        graph: &OsWorkflow,
        instances: &[OsAppInstance],
        documents: &BTreeMap<String, Vec<u8>>,
        configs: &BTreeMap<String, Vec<u8>>,
        state: &mut RunState,
        cache: &mut dyn MediaCache,
    ) -> Result<(BTreeMap<String, Vec<u8>>, RunReport), RunError> {
        Self::validate_edge_kinds(graph)?;
        let order = topological_order(graph)?;
        let node_by_id: HashMap<&str, &OsWorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let instance_by_id: HashMap<&str, &OsAppInstance> = instances.iter().map(|instance| (instance.id.as_str(), instance)).collect();
        let mut incoming: HashMap<&str, Vec<&semio_framework_os::OsWorkflowEdge>> = HashMap::new();
        for edge in &graph.edges {
            incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
        }

        let mut documents_out = documents.clone();
        let mut report = RunReport::default();
        let mut live: HashMap<String, u32> = HashMap::new();

        for node_id in &order {
            let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
            let instance = *instance_by_id.get(node.instance_id.as_str()).ok_or_else(|| RunError::UnknownInstance(node.instance_id.clone()))?;
            let document_bytes = documents_out.get(&instance.id).cloned().unwrap_or_default();
            let document_fingerprint = framework_hash::hash_bytes(&document_bytes);
            let config_bytes = configs.get(&instance.id).cloned().unwrap_or_default();
            let config_fingerprint = framework_hash::hash_bytes(&config_bytes);

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
                        let source_instance = *instance_by_id.get(source_node.instance_id.as_str()).ok_or_else(|| RunError::UnknownInstance(source_node.instance_id.clone()))?;
                        let source_document_bytes = documents_out.get(&source_instance.id).cloned().unwrap_or_default();
                        let source_config_bytes = configs.get(&source_instance.id).cloned().unwrap_or_default();
                        let (_source_document, source_outputs) = self.compute_node(&mut live, source_node, source_instance, &source_document_bytes, &source_config_bytes, &BTreeMap::new())?;
                        let (media, _fresh_fingerprint) = source_outputs
                            .get(&edge.source_port_id)
                            .cloned()
                            .ok_or_else(|| RunError::Host(format!("upstream node `{}` produced no output on port `{}`", edge.source_node_id, edge.source_port_id)))?;
                        cache.put(&fingerprint, &media);
                        media
                    }
                };
                input_media.insert(edge.target_port_id.clone(), media);
            }

            let (mutated_document, outputs) = self.compute_node(&mut live, node, instance, &document_bytes, &config_bytes, &input_media)?;
            documents_out.insert(instance.id.clone(), mutated_document);

            let mut output_fingerprints = BTreeMap::new();
            for (port_id, (media, fingerprint)) in &outputs {
                output_fingerprints.insert(port_id.clone(), fingerprint.clone());
                cache.put(&MediaFingerprint(fingerprint.clone()), media);
            }
            state.nodes.insert(node_id.clone(), NodeRunRecord { document_fingerprint, input_fingerprints, output_fingerprints, config_fingerprint });
        }

        Ok((documents_out, report))
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
    /// 🗺️ `plugin_path_for_plugin` maps a plugin id (`OsAppInstance::plugin_id`, the same id
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
    use semio_framework_core::{MediaClass, MediaForm, MediaType};
    use semio_framework_os::{placeholder_media_contract, OsWorkflowEdge, OsMediaPort};

    /// 🧪️ A fake `AppChannelHost` for tests: no wasm at all, just a per-instance document, a fixed
    /// structured output per port, and an in-process `InMemoryBlobStore` — enough to interpret the
    /// exact `AppCommand`/`AppFrame` frame script `SpaceRunner::compute_node` sends, so `SpaceRunner`'s
    /// dirty/clean bookkeeping can be exercised without a real program.
    /// 🧪️ Outputs are keyed by app id, not by handle — a real app's export is a function of its
    /// document/logic, not of the ephemeral instance handle a host happens to mint this call, and a
    /// node genuinely does get re-opened (a fresh handle) on every dirty re-run.
    #[derive(Default)]
    struct FakeHost {
        documents: HashMap<u32, (Vec<u8>, Vec<u8>)>,
        handle_app: HashMap<u32, String>,
        outputs: HashMap<(String, String), Media>,
        configs: HashMap<u32, Vec<u8>>,
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
                    AppCommand::Hello { channel_version, config, .. } => {
                        if channel_version != CHANNEL_VERSION {
                            frames.push(AppFrame::Error { in_reply_to: None, code: "channel-version".into(), message: "mismatched channel version".into() });
                            continue;
                        }
                        if !config.is_empty() {
                            self.configs.insert(node, config);
                        }
                        frames.push(AppFrame::Welcome { channel_version: CHANNEL_VERSION, instance: node, manifest: Vec::new() });
                    }
                    AppCommand::Configure { seq, config } => {
                        self.configs.insert(node, config);
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
                            let value = serde_json::to_value(&fingerprint).unwrap_or_default();
                            frames.push(AppFrame::MediaFingerprint { in_reply_to: seq, port: String::new(), fingerprint: store::pack_rt::encode_wire_value(&value) });
                        }
                        None => frames.push(AppFrame::Error { in_reply_to: Some(seq), code: "handler".into(), message: "no output".into() }),
                    },
                    AppCommand::ReadDocument { seq } => {
                        let (pack, spr) = self.documents.get(&node).cloned().unwrap_or_default();
                        frames.push(AppFrame::Document { in_reply_to: seq, pack, spr, ops: String::new() });
                    }
                    _ => {}
                }
            }
            Ok(frames)
        }
    }

    fn two_node_graph() -> (OsWorkflow, Vec<OsAppInstance>) {
        let source = OsWorkflowNode {
            id: "node-a".into(),
            instance_id: "instance-a".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs: Vec::new(),
            outputs: vec![OsMediaPort { id: "out".into(), artifact_kind: "data.value".into(), direction: "out".into() }],
        };
        let target = OsWorkflowNode {
            id: "node-b".into(),
            instance_id: "instance-b".into(),
            x: 1.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs: vec![OsMediaPort { id: "in".into(), artifact_kind: "data.value".into(), direction: "in".into() }],
            outputs: Vec::new(),
        };
        let edge = OsWorkflowEdge { id: "edge-1".into(), source_node_id: "node-a".into(), source_port_id: "out".into(), target_node_id: "node-b".into(), target_port_id: "in".into(), contract: placeholder_media_contract("data.value") };
        let graph = OsWorkflow { schema: "s.workflow".into(), nodes: vec![source, target], edges: vec![edge] };
        let instances = vec![
            OsAppInstance { id: "instance-a".into(), plugin_id: "program".into(), app_id: "app-a".into(), label: "A".into(), yields: "data.value".into(), document: semio_framework_os::OsDocumentRef { document_id: "instance-a".into(), schema: "app-a.document".into() }, config: None },
            OsAppInstance { id: "instance-b".into(), plugin_id: "program".into(), app_id: "app-b".into(), label: "B".into(), yields: "".into(), document: semio_framework_os::OsDocumentRef { document_id: "instance-b".into(), schema: "app-b.document".into() }, config: None },
        ];
        (graph, instances)
    }

    fn empty_documents() -> BTreeMap<String, Vec<u8>> {
        [("instance-a".to_string(), encode_document_pack_bytes(&[], &[])), ("instance-b".to_string(), encode_document_pack_bytes(&[], &[]))].into()
    }

    #[test]
    fn topological_order_respects_edges() {
        let (graph, _) = two_node_graph();
        let order = topological_order(&graph).expect("acyclic");
        assert_eq!(order, vec!["node-a".to_string(), "node-b".to_string()]);
    }

    #[test]
    fn detects_cycles() {
        let (mut graph, _) = two_node_graph();
        graph.edges.push(OsWorkflowEdge { id: "edge-2".into(), source_node_id: "node-b".into(), source_port_id: "in".into(), target_node_id: "node-a".into(), target_port_id: "out".into(), contract: placeholder_media_contract("data.value") });
        assert!(matches!(topological_order(&graph), Err(RunError::Cycle(_))));
    }

    #[test]
    fn first_run_recomputes_every_node_second_run_is_a_no_operation() {
        let (graph, instances) = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-a", "out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()));
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents = empty_documents();
        let configs: BTreeMap<String, Vec<u8>> = BTreeMap::new();

        let (documents_1, report_1) = runner.run(&graph, &instances, &documents, &configs, &mut state, &mut cache).expect("first run");
        assert_eq!(report_1.recomputed, vec!["node-a".to_string(), "node-b".to_string()]);
        assert!(report_1.clean.is_empty());

        let (_, report_2) = runner.run(&graph, &instances, &documents_1, &configs, &mut state, &mut cache).expect("second run");
        assert!(report_2.recomputed.is_empty(), "unchanged documents must not re-trigger recompute: {:?}", report_2.recomputed);
        assert_eq!(report_2.clean, vec!["node-a".to_string(), "node-b".to_string()]);
    }

    #[test]
    fn editing_upstream_document_dirties_downstream_only_through_the_wire() {
        let (graph, instances) = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-a", "out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()));
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents = empty_documents();
        let configs: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let (documents_1, _) = runner.run(&graph, &instances, &documents, &configs, &mut state, &mut cache).expect("first run");

        let mut documents_2 = documents_1;
        documents_2.insert("instance-a".to_string(), b"edited".to_vec());
        let (_, report_2) = runner.run(&graph, &instances, &documents_2, &configs, &mut state, &mut cache).expect("second run");
        assert_eq!(report_2.recomputed, vec!["node-a".to_string()], "node-a's own document changed, so node-a must recompute");
        assert_eq!(report_2.clean, vec!["node-b".to_string()], "node-a's FakeHost output is fixed, so its output fingerprint is unchanged — node-b must stay clean (the early-cutoff this whole design exists for)");
    }

    /// 🧪️ New for `HEADLESS-RUNNER-AND-WORKFLOW-CONFIG-MODEL`: changing a node's own effective config
    /// — document and resolved inputs held constant — must dirty exactly that node on the very next
    /// `plan()`/`run()`, mirroring `editing_upstream_document_dirties_downstream_only_through_the_wire`'s
    /// shape but on the config dimension instead of the document one.
    #[test]
    fn changing_a_nodes_config_alone_dirties_it_without_touching_document_or_inputs() {
        let (graph, instances) = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-a", "out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()));
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents = empty_documents();
        let configs_1: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        runner.run(&graph, &instances, &documents, &configs_1, &mut state, &mut cache).expect("first run");

        let plan_unchanged = plan(&graph, &documents, &configs_1, &state).expect("plan with unchanged config");
        assert!(plan_unchanged.recomputed.is_empty(), "nothing changed, plan must report every node clean: {:?}", plan_unchanged.recomputed);

        let configs_2: BTreeMap<String, Vec<u8>> = [("instance-a".to_string(), b"threshold=2".to_vec())].into();
        let plan_changed = plan(&graph, &documents, &configs_2, &state).expect("plan with changed config");
        assert_eq!(plan_changed.recomputed, vec!["node-a".to_string()], "only node-a's own config changed, so only node-a should be recomputed by the plan");

        let (_, report_2) = runner.run(&graph, &instances, &documents, &configs_2, &mut state, &mut cache).expect("second run with changed config");
        assert_eq!(report_2.recomputed, vec!["node-a".to_string()], "node-a's config changed, so node-a must recompute even though its document and inputs did not");
        assert_eq!(report_2.clean, vec!["node-b".to_string()], "node-a's FakeHost output is fixed regardless of config, so node-b must stay clean");
    }

    #[test]
    fn rejects_mismatched_edge_artifact_kinds() {
        let (mut graph, instances) = two_node_graph();
        graph.nodes[1].inputs[0].artifact_kind = "other.kind".into();
        let host = FakeHost::default();
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()));
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents = empty_documents();
        let configs: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let result = runner.run(&graph, &instances, &documents, &configs, &mut state, &mut cache);
        assert!(matches!(result, Err(RunError::Incompatible { .. })));
    }
}
//#endregion 🔖️Tests
