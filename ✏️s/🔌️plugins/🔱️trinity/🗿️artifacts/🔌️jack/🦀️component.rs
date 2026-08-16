//! 🔺️ `trinity.graph` artifact — in-memory directed property port graph with compile-time manifest.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`trinity→C:graph`, jack side — see this
//! region's own doc for why this is ONE composed child, not two: the design annotation "jack; rewrite
//! = 2 graph children" attributes the two-child shape to the SEPARATE `rewrite` app's LHS/RHS rule
//! windows, not to jack. jack's own persisted `nodes`/`edges` instance data is replaced by a single
//! composed `s.stdio.semio.graph` CHILD slot (`🔖️ContentBridge` below); the compile-time `manifest`
//! (kind/property/port DEFINITIONS, resolved from `manifestId` — see `graph::manifest::GraphManifest`)
//! is NOT graph-shaped in the `SemioGraphSnapshot` sense (that subset is an INSTANCE graph: nodes with
//! position/ports/properties, edges with source/target) and stays an ordinary inline field, unchanged.

pub use crate::artifacts::jack::schema::diff::JackDiff;
pub use crate::artifacts::jack::schema::mutations::TrinityGraphMutation;

use graph::manifest::{manifest_by_id, GraphManifest, ManifestValidationError, TrinityManifest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub use graph::manifest::{ManifestValidator, PortDirection, PropertyBag, PropertyDef, PropertyKind, PropertyValue};

/// 📜️ Compile-time trinity manifest (projection of {@link GraphManifest}).
pub type Manifest = TrinityManifest;

//#region ⚠️ Errors
/// ⚠️ Trinity graph fixture, manifest-validation, and mutation errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityRamError {
    /// 🧬️ JSON (de)serialization failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// 🧭️ VCS store/dispatch failure.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
    /// 📜️ Compile-time manifest validation failure (path-qualified).
    #[error("{}: {}", .0.path, .0.message)]
    Manifest(ManifestValidationError),
    #[error("expected schema {expected}, got {actual}")]
    SchemaMismatch { expected: &'static str, actual: String },
    #[error("unknown manifest id {0}")]
    UnknownManifestId(String),
    #[error("fixture missing manifest or manifestId")]
    ManifestMissing,
    #[error("node {0} not found")]
    NodeNotFound(String),
    #[error("edge {0} not found")]
    EdgeNotFound(String),
    #[error("node {0} already exists")]
    NodeAlreadyExists(String),
    #[error("edge {0} already exists")]
    EdgeAlreadyExists(String),
    #[error("invalid source port key {0}")]
    InvalidSourcePortKey(String),
    #[error("invalid target port key {0}")]
    InvalidTargetPortKey(String),
    #[error("source node {0} not found")]
    SourceNodeNotFound(String),
    #[error("target node {0} not found")]
    TargetNodeNotFound(String),
    #[error("nodes/{node_id}/ports/{port_kind}: port kind {port_kind} not declared on node kind {node_kind}")]
    PortKindNotDeclaredOnFixture { node_id: String, port_kind: String, node_kind: String },
    #[error("nodes/{node_id}/ports/{port_id}: port kind {port_kind} not declared on node kind {node_kind}")]
    PortKindNotDeclaredOnMutation { node_id: String, port_id: String, port_kind: String, node_kind: String },
    #[error("nodes/{kind}: unknown node kind {kind:?}")]
    UnknownNodeKind { kind: String },
    #[error("edges/{kind}: unknown edge kind {kind:?}")]
    UnknownEdgeKind { kind: String },
    #[error("ports/{kind}: unknown port kind {kind:?}")]
    UnknownPortKind { kind: String },
    #[error("{path}: unknown kind")]
    UnknownEntityKind { path: String },
    #[error("{path}: unknown property {key:?}")]
    UnknownPropertyAtPath { path: String, key: String },
    #[error("{path}/{name}: property type mismatch for {value_type}")]
    PropertyTypeMismatch { path: String, name: String, value_type: String },
    #[error("{path}/{key}: unknown property {key:?}")]
    UnknownPropertyInBag { path: String, key: String },
}

/// 🔀️ [`ManifestValidationError`] carries no `std::error::Error` impl of its own (plain path/message struct), so this is a manual conversion rather than `#[from]`.
impl From<ManifestValidationError> for TrinityRamError {
    fn from(error: ManifestValidationError) -> Self {
        Self::Manifest(error)
    }
}
//#endregion ⚠️ Errors

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.graph` document — jack's `nodes`/`edges`
/// instance data now lives in this composed child's own `nodes`/`edges`, not on `JackSnapshot`.
pub type JackContentChild = store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot>;

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{
    GraphEdgeId as SemioGraphEdgeId, GraphNodeId as SemioGraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,
};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

/// 🏷️ `jack.node` is the honest string boundary carrying the FULL [`Node`] (id/kind/name/x/y/
/// width/height/properties/ports — every field this plugin's own rich node model can hold, none of
/// which `SemioGraphNode`'s native fields alone can carry: `width`/`height` have no native slot at
/// all, and a port's own `kind`/`properties` don't survive the native `ports` projection below) as
/// JSON. `id`/`kind`/`label`/`position` are ALSO projected onto `SemioGraphNode`'s own native fields,
/// and `ports` is a best-effort projection (`Port.id` → `SemioGraphPort.name`, `Port.direction` →
/// `SemioGraphPortKind`), for genuine graph-shape tooling that only understands the neutral subset —
/// but the JSON blob is the round-trip SOURCE OF TRUTH on decode (matches `dag`'s own precedent, see
/// `📓️wave4-reports/dag-report.md`).
const JACK_NODE_JSON_PROPERTY: &str = "jack.node";

fn semio_port_kind_from_direction(direction: PortDirection) -> SemioGraphPortKind {
    match direction {
        PortDirection::In => SemioGraphPortKind::In,
        PortDirection::Out => SemioGraphPortKind::Out,
    }
}

fn port_direction_from_semio_port_kind(kind: SemioGraphPortKind) -> PortDirection {
    match kind {
        SemioGraphPortKind::In | SemioGraphPortKind::InOut => PortDirection::In,
        SemioGraphPortKind::Out => PortDirection::Out,
    }
}

fn semio_node_from_jack_node(node: &Node) -> SemioGraphNode {
    let ports = node.ports.iter().map(|port| SemioGraphPort { name: port.id.clone(), kind: semio_port_kind_from_direction(port.direction) }).collect();
    SemioGraphNode {
        id: SemioGraphNodeId::new(node.id.clone()),
        kind: node.kind.clone(),
        label: node.name.clone(),
        position: SemioPoint2 { x: node.x, y: node.y },
        ports,
        properties: vec![SemioValueEntry { key: JACK_NODE_JSON_PROPERTY.into(), value: SemioValue::Str { value: serde_json::to_string(node).unwrap_or_default() } }],
    }
}

/// 🌉 Inverse of [`semio_node_from_jack_node`] — reconstructs the exact [`Node`] from its `jack.node`
/// JSON property. Falls back to a minimal node built from the graph-native `id`/`kind`/`label`/
/// `position`/`ports` fields only if the property is missing (content authored outside this plugin,
/// e.g. a hand-written `graph` doc) — never panics.
fn jack_node_from_semio_node(node: &SemioGraphNode) -> Node {
    for property in &node.properties {
        if property.key == JACK_NODE_JSON_PROPERTY {
            if let SemioValue::Str { value } = &property.value {
                if let Ok(parsed) = serde_json::from_str::<Node>(value) {
                    return parsed;
                }
            }
        }
    }
    Node {
        id: node.id.value.clone(),
        kind: node.kind.clone(),
        name: node.label.clone(),
        x: node.position.x,
        y: node.position.y,
        width: 0.0,
        height: 0.0,
        properties: PropertyBag::new(),
        ports: node.ports.iter().map(|port| Port { id: port.name.clone(), kind: String::new(), direction: port_direction_from_semio_port_kind(port.kind), properties: PropertyBag::new() }).collect(),
    }
}

/// 🏷️ `SemioGraphEdge` has no `properties` slot (unlike `SemioGraphNode`) — its `label` field (which
/// this plugin's own [`Edge`] never populates on its own behalf) is repurposed to carry the FULL
/// `Edge` (port-qualified `source`/`target` endpoint strings, `properties`) as JSON, the round-trip
/// source of truth on decode. `source`/`target`/`kind` are also projected onto their native fields
/// (node-id only, port suffix stripped via [`crate::artifacts::jack::port_node_id`]) for genuine
/// graph-shape tooling.
fn semio_edge_from_jack_edge(edge: &Edge) -> SemioGraphEdge {
    let source_node = port_node_id(&edge.source).unwrap_or(&edge.source);
    let target_node = port_node_id(&edge.target).unwrap_or(&edge.target);
    SemioGraphEdge {
        id: SemioGraphEdgeId::new(edge.id.clone()),
        source: SemioGraphNodeId::new(source_node.to_string()),
        target: SemioGraphNodeId::new(target_node.to_string()),
        kind: edge.kind.clone(),
        label: serde_json::to_string(edge).unwrap_or_default(),
    }
}

/// 🌉 Inverse of [`semio_edge_from_jack_edge`] — falls back to a bare node-id (no port qualifier)
/// edge if `label` isn't valid `Edge` JSON (content authored outside this plugin) — never panics.
fn jack_edge_from_semio_edge(edge: &SemioGraphEdge) -> Edge {
    serde_json::from_str::<Edge>(&edge.label).unwrap_or_else(|_| Edge { id: edge.id.value.clone(), kind: edge.kind.clone(), source: edge.source.value.clone(), target: edge.target.value.clone(), properties: PropertyBag::new() })
}

/// 🌉 REAL bidirectional converter between jack's own live `Node`/`Edge` editing state and the
/// composed child's `SemioGraphSnapshot` node/edge graph (the "ModelBridge"/"DocumentBridge" pattern
/// — see `📓️wave3-reports/cad-report.md` and `📓️wave4-reports/dag-report.md`).
pub fn jack_content_snapshot_from_working(nodes: &[Node], edges: &[Edge]) -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: nodes.iter().map(semio_node_from_jack_node).collect(), edges: edges.iter().map(semio_edge_from_jack_edge).collect() }
}

/// 🌉 Inverse of [`jack_content_snapshot_from_working`].
pub fn working_from_jack_content_snapshot(content: &SemioGraphSnapshot) -> (Vec<Node>, Vec<Edge>) {
    (content.nodes.iter().map(jack_node_from_semio_node).collect(), content.edges.iter().map(jack_edge_from_semio_edge).collect())
}

/// 🕸️ Deterministic content-addressed CHILD handle for the jack content — same `(child_id, target)`
/// for identical `(nodes, edges)`, a different pair once the content actually changes; mirrors
/// `dag_content_child_handle`/`flow_content_child_handle`/`document_child_handle`.
pub fn jack_content_child_handle(nodes: &[Node], edges: &[Edge]) -> JackContentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = jack_content_snapshot_from_working(nodes, edges);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("jack-content-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "graph".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "jack-content".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side working representation of the composed content child's live
/// nodes/edges — NEVER persisted, NEVER a durable field on `JackSnapshot` itself (matches the
/// `EngineRep` contract: wholly derived, droppable at any instant, rebuilt from base). Exists because
/// no `LinkResolver`/child-dispatch seam is wired into `ArtifactApp::handle` yet (checked directly
/// against `🔌️plugin/🦀️component.rs` — same standing gap every prior exemplar's report documents,
/// most recently `dag-report.md`); until one exists, the only way a persisted content-addressed
/// HANDLE can round-trip to real nodes/edges within one process is this cache, keyed by
/// `JackContentChild::child_id` — mirrors `DagWorkingScene`/`FlowWorkingScene`/`WriterWorkingScene`.
///
/// ⚠️ Same documented gap as every prior exemplar: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, and a bare `parse_dsl`/`decode_pack` of persisted bytes recovers
/// only the opaque handle unless the wire format ALSO carries the raw content (see
/// `📸️snapshot/🦀️component.rs`'s own `🔖️CodecPrimitives`, which does — the same lesson `dag`'s
/// report documents finding the hard way). `jack_working_scene`/`jack_working_scene_for_handle` fail
/// soft (an empty scene) rather than panicking.
#[derive(Clone, Debug, Default)]
pub struct JackWorkingScene {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

thread_local! {
    static JACK_SCRATCH: std::cell::RefCell<std::collections::HashMap<String, JackWorkingScene>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// 📝 Seeds the scratch cache for a handle — call whenever new nodes/edges content is about to become
/// a document's `content` field (every mutation-diff/fixture builder in this plugin does, via
/// [`jack_content_child_handle_and_cache`]).
pub fn cache_jack_content(child_id: &str, nodes: Vec<Node>, edges: Vec<Edge>) {
    JACK_SCRATCH.with(|cache| cache.borrow_mut().insert(child_id.to_string(), JackWorkingScene { nodes, edges }));
}

/// 🔎 Reads the cached live scene for a content child handle — an empty scene (never a panic) when
/// nothing has cached it yet.
pub fn jack_working_scene_for_handle(handle: &JackContentChild) -> JackWorkingScene {
    JACK_SCRATCH.with(|cache| cache.borrow().get(&handle.child_id).cloned()).unwrap_or_default()
}

/// 🔎 Reads the current document's live nodes/edges off its `content` child handle — the single read
/// call site every mutation diff/inverse/app command in this plugin uses instead of the old
/// `snapshot.nodes`/`.edges` field access.
pub fn jack_working_scene(snapshot: &JackSnapshot) -> JackWorkingScene {
    jack_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints a new content-addressed handle AND seeds the scratch cache with its scene in one call —
/// the standard way every mutation-diff/fixture builder in this plugin creates a `content` field
/// value; never construct a handle without also caching, or [`jack_working_scene`] will read back
/// empty.
pub fn jack_content_child_handle_and_cache(nodes: Vec<Node>, edges: Vec<Edge>) -> JackContentChild {
    let handle = jack_content_child_handle(&nodes, &edges);
    cache_jack_content(&handle.child_id, nodes, edges);
    handle
}
//#endregion 🔖️WorkingScene

// #region 🔖️Runtime
/// 🔌️ Runtime port on a node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    pub id: String,
    pub kind: String,
    pub direction: PortDirection,
    #[serde(default)]
    pub properties: PropertyBag,
}

/// 🧩️ Runtime node (piece).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    #[serde(default)]
    pub properties: PropertyBag,
    #[serde(default)]
    pub ports: Vec<Port>,
}

/// 🔗️ Runtime edge (connection).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub properties: PropertyBag,
}

/// 📷️ Camera for fixture documents.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 📸️ Persisted jack snapshot — defined in `snapshot::schema`.
pub use super::snapshot::schema::JackSnapshot;

impl JackSnapshot {
    pub const SCHEMA: &'static str = "trinity.graph";

    pub fn validate_schema(&self) -> Result<(), TrinityRamError> {
        if self.schema != Self::SCHEMA {
            return Err(TrinityRamError::SchemaMismatch { expected: Self::SCHEMA, actual: self.schema.clone() });
        }
        Ok(())
    }

    /// 📤️ JSON fixture text — unlike `Serialize`'s derive (which would emit the opaque `content`
    /// handle only, unrecoverable once the working-scene cache that minted it is gone, e.g. across a
    /// process boundary or a persisted embedded fixture string), this hand-rolled JSON shape embeds
    /// the REAL `nodes`/`edges` at the top level, mirroring the old pre-migration wire shape and
    /// matching the same "wire format carries real content, not just the handle" fix the hand-rolled
    /// `ArtifactDsl`/`ArtifactPack` codecs use (see `📸️snapshot/📝️text/🦀️component.rs`'s own doc
    /// comment for the full rationale).
    pub fn to_json(&self) -> Result<String, TrinityRamError> {
        let scene = jack_working_scene(self);
        let value = serde_json::json!({
            "schema": self.schema,
            "name": self.name,
            "manifestId": self.manifest_id,
            "camera": self.camera,
            "nodes": scene.nodes,
            "edges": scene.edges,
            "rootNodeId": self.root_node_id,
        });
        Ok(serde_json::to_string_pretty(&value)?)
    }

    pub fn resolve_manifest(&mut self) -> Result<(), TrinityRamError> {
        if let Some(id) = self.manifest_id.as_deref() {
            self.manifest = manifest_by_id(id).ok_or_else(|| TrinityRamError::UnknownManifestId(id.to_string()))?.to_trinity_manifest();
            return Ok(());
        }
        if self.manifest.node_kinds.is_empty() && self.manifest.edge_kinds.is_empty() && self.manifest.port_kinds.is_empty() {
            return Err(TrinityRamError::ManifestMissing);
        }
        Ok(())
    }

    /// 📥️ Inverse of [`Self::to_json`] — parses the real `nodes`/`edges` JSON arrays and mints+caches
    /// a fresh content-addressed handle from them (deterministic: identical `(nodes, edges)` always
    /// re-derives the same handle, so peers replaying the same JSON text converge).
    pub fn from_json(json: &str) -> Result<Self, TrinityRamError> {
        let value: serde_json::Value = serde_json::from_str(json)?;
        let schema = value.get("schema").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let name = value.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let manifest_id: Option<String> = value.get("manifestId").and_then(|v| v.as_str()).map(str::to_string);
        let camera: Camera = value.get("camera").map(|v| serde_json::from_value(v.clone())).transpose()?.unwrap_or_default();
        let nodes: Vec<Node> = value.get("nodes").map(|v| serde_json::from_value(v.clone())).transpose()?.unwrap_or_default();
        let edges: Vec<Edge> = value.get("edges").map(|v| serde_json::from_value(v.clone())).transpose()?.unwrap_or_default();
        let root_node_id: Option<String> = value.get("rootNodeId").and_then(|v| v.as_str()).map(str::to_string);
        let mut fixture = Self::with_content(schema, name, manifest_id, Manifest::default(), camera, nodes, edges, root_node_id);
        fixture.validate_schema()?;
        fixture.resolve_manifest()?;
        Ok(fixture)
    }

    /// 🏗️ Drop-in constructor mirroring the OLD `nodes`/`edges`-bearing struct literal's field order
    /// — mints+caches the composed content child so every existing fixture-builder call site becomes
    /// a mechanical `JackSnapshot { .., nodes, edges, .. }` → `JackSnapshot::with_content(.., nodes,
    /// edges, ..)` rewrite instead of a hand-rolled handle mint at each site.
    pub fn with_content(schema: String, name: String, manifest_id: Option<String>, manifest: Manifest, camera: Camera, nodes: Vec<Node>, edges: Vec<Edge>, root_node_id: Option<String>) -> Self {
        Self { schema, name, manifest_id, manifest, camera, content: jack_content_child_handle_and_cache(nodes, edges), root_node_id }
    }

    /// 🔎 Live node list, read through the working-scene cache — replaces the old direct `.nodes`
    /// field access (see `🔖️WorkingScene`'s module doc for why this indirection exists).
    pub fn nodes(&self) -> Vec<Node> {
        jack_working_scene(self).nodes
    }

    /// 🔎 Live edge list, read through the working-scene cache.
    pub fn edges(&self) -> Vec<Edge> {
        jack_working_scene(self).edges
    }
}

/// 🧠️ In-memory trinity graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    pub name: String,
    pub manifest: Manifest,
    pub camera: Camera,
    pub nodes: BTreeMap<String, Node>,
    pub edges: BTreeMap<String, Edge>,
    pub root_node_id: Option<String>,
}

impl Graph {
    pub fn from_fixture(mut fixture: JackSnapshot) -> Result<Self, TrinityRamError> {
        fixture.validate_schema()?;
        fixture.resolve_manifest()?;
        if let Some(id) = fixture.manifest_id.as_deref() {
            if let Some(gm) = manifest_by_id(id) {
                validate_trinity_fixture(&gm, &fixture)?;
            }
        }
        let scene = jack_working_scene(&fixture);
        let mut nodes = BTreeMap::new();
        for node in scene.nodes {
            nodes.insert(node.id.clone(), node);
        }
        let mut edges = BTreeMap::new();
        for edge in scene.edges {
            edges.insert(edge.id.clone(), edge);
        }
        Ok(Self { name: fixture.name, manifest: fixture.manifest, camera: fixture.camera, nodes, edges, root_node_id: fixture.root_node_id })
    }

    pub fn to_fixture(&self) -> JackSnapshot {
        JackSnapshot::with_content(
            JackSnapshot::SCHEMA.to_string(),
            self.name.clone(),
            Some("nakagin".into()),
            self.manifest.clone(),
            self.camera.clone(),
            self.nodes.values().cloned().collect(),
            self.edges.values().cloned().collect(),
            self.root_node_id.clone(),
        )
    }

    pub fn load_json(json: &str) -> Result<Self, TrinityRamError> {
        Self::from_fixture(JackSnapshot::from_json(json)?)
    }

    pub fn fixture_json(&self) -> Result<String, TrinityRamError> {
        self.to_fixture().to_json()
    }

    /// 🧩️ Build a `trinity.graph` fixture containing only the given node and edge ids.
    pub fn subgraph_fixture(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> JackSnapshot {
        let nodes: Vec<Node> = node_ids.iter().filter_map(|id| self.nodes.get(id).cloned()).collect();
        let edges: Vec<Edge> = edge_ids.iter().filter_map(|id| self.edges.get(id).cloned()).collect();
        let root_node_id = self.root_node_id.clone().filter(|id| node_ids.contains(id));
        JackSnapshot::with_content(JackSnapshot::SCHEMA.to_string(), format!("{} subgraph", self.name), Some("nakagin".into()), self.manifest.clone(), self.camera.clone(), nodes, edges, root_node_id)
    }

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn remove_node(&mut self, id: &str) -> bool {
        if self.nodes.remove(id).is_none() {
            return false;
        }
        let edge_ids: Vec<String> = self.edges.iter().filter(|(_, e)| port_node_id(&e.source) == Some(id) || port_node_id(&e.target) == Some(id)).map(|(id, _)| id.clone()).collect();
        for eid in edge_ids {
            self.edges.remove(&eid);
        }
        if self.root_node_id.as_deref() == Some(id) {
            self.root_node_id = None;
        }
        true
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge.id.clone(), edge);
    }

    pub fn remove_edge(&mut self, id: &str) -> bool {
        self.edges.remove(id).is_some()
    }

    pub fn set_property(&mut self, entity: EntityRef, key: &str, value: PropertyValue) -> Result<(), TrinityRamError> {
        match entity {
            EntityRef::Node(id) => {
                let node = self.nodes.get_mut(&id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
                node.properties.insert(key.to_string(), value);
            }
            EntityRef::Edge(id) => {
                let edge = self.edges.get_mut(&id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
                edge.properties.insert(key.to_string(), value);
            }
        }
        Ok(())
    }
}

/// 🛡️ Validates trinity fixture instances against a compile-time graph manifest.
fn validate_trinity_fixture(gm: &GraphManifest, fixture: &JackSnapshot) -> Result<(), TrinityRamError> {
    let validator = ManifestValidator::new(gm);
    let scene = jack_working_scene(fixture);
    for node in &scene.nodes {
        validator.validate_node_kind(&node.kind)?;
        validator.validate_node_properties(&node.kind, &node.properties)?;
        if let Some(node_def) = gm.node_kind(&node.kind) {
            for port in &node.ports {
                validator.validate_port_kind(&port.kind)?;
                if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == &port.kind) {
                    return Err(TrinityRamError::PortKindNotDeclaredOnFixture { node_id: node.id.clone(), port_kind: port.kind.clone(), node_kind: node.kind.clone() });
                }
            }
        }
    }
    for edge in &scene.edges {
        validator.validate_edge_kind(&edge.kind)?;
        validator.validate_edge_properties(&edge.kind, &edge.properties)?;
    }
    Ok(())
}

/// 🎯️ Entity reference for mutations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "entity", content = "id")]
pub enum EntityRef {
    Node(String),
    Edge(String),
}

/// 🔑️ Parse `nodeId@portId` port key (`@` is the unified syntax's one port sigil — `:` is reserved for typing).
pub fn parse_port_key(key: &str) -> Option<(&str, &str)> {
    let (node, port) = key.split_once('@')?;
    if node.is_empty() || port.is_empty() {
        return None;
    }
    Some((node, port))
}

/// 🧩️ Node id from a port key.
pub fn port_node_id(key: &str) -> Option<&str> {
    parse_port_key(key).map(|(n, _)| n)
}

/// 🔌️ Port id from a port key.
pub fn port_port_id(key: &str) -> Option<&str> {
    parse_port_key(key).map(|(_, p)| p)
}

/// 🏗️ Build a port key.
pub fn port_key(node_id: &str, port_id: &str) -> String {
    format!("{node_id}@{port_id}")
}

pub const TRINITY_GRAPH_SCHEMA: &str = JackSnapshot::SCHEMA;

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (editor AND viewer) of this artifact shares — lives at the ARTIFACT level, not under
/// `editor`, so a viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.trinity.jack"` matches `#[artifact_schema(id = "s.trinity.jack")]` in this
/// subset's own `🧬️schema/🦀️component.rs`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.trinity.jack@1/*#editor` / `s.trinity.jack@1/*#viewer` (contract §1 grammar).
pub const TRINITY_JACK_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.trinity.jack", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };

pub fn empty_trinity_graph_fixture() -> JackSnapshot {
    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "trinity".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), Vec::new(), Vec::new(), None)
}

/// 🎯️ `ArtifactKindSpec` identity shared by every `jack`-family app that mounts this artifact.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "graph.trinity".into(),
        name: "Trinity Graph".into(),
        source_format: "trinity.graph".into(),
        component_kind: "trinity".into(),
        dimension: "graph".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Graph, form: semio_framework_plugin::MediaForm::Trinity },
        schema: "trinity.graph".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"],
    }
}
// #endregion 🔖️Runtime

//#region 🔖️Register
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `io_registry::entries()`'s own `OnceLock` convention.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "jack.document",
                    extension: Some("trinity"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::jack::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::jack::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::jack::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::jack::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("jack.document"),
                },
                dsl::LanguageSpec {
                    id: "jack.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::jack::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::jack::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::jack::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::jack::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("jack.op"),
                },
                dsl::LanguageSpec {
                    id: "jack.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::jack::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::jack::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("jack.diff"),
                },
                dsl::LanguageSpec {
                    id: "jack.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::jack::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::jack::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("jack.pack"),
                },
                dsl::LanguageSpec {
                    id: "jack.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::jack::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::jack::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("jack.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback. `crate::editor::jack::config::schema::register_app_schema()` is the
/// one exception, kept alive via the plugin root's own narrowed `.setup()`: it registers the
/// `TrinityJackPlayApp` CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration`
/// deliberately has no field for (see that struct's own doc).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.jack.standard.v1", "standard", "1", &[], None),
        ("s.jack.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.jack.schema.artifact", "schema", "s.trinity.jack", &[("schema", "s.trinity.jack")], None),
        ("s.jack.inference.artifact", "inference", "s.trinity.jack.inference", &[("schema", "s.trinity.jack.inference")], None),
        ("s.jack.composer.native", "composer", "s.jack@1/*", &[("dialect", "s.jack@1/*")], None),
        ("s.jack.composer.format-1", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.jack.composer.format-2", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.jack.composer.format-3", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.jack.composer.format-4", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.jack.composer.format-5", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.jack.grammar.1", "grammar", "jack.document", &[("grammar", "jack.document")], None),
        ("s.jack.grammar.2", "grammar", "jack.op", &[("grammar", "jack.op")], None),
        ("s.jack.grammar.3", "grammar", "jack.diff", &[("grammar", "jack.diff")], None),
        ("s.jack.grammar.4", "grammar", "jack.pack", &[("grammar", "jack.pack")], None),
        ("s.jack.grammar.5", "grammar", "jack.spr", &[("grammar", "jack.spr")], None),
        ("s.jack.codec.document-1", "codec", "trinity.graph:jack", &[("codec", "trinity.graph"), ("extension", "jack")], None),
        ("s.jack.localization.en", "localization", "Jack", &[], Some(("en", "Jack"))),
        ("s.jack.localization.de", "localization", "Buchse", &[], Some(("de", "Buchse"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.jack")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::jack::schema::jack_artifact_schema_descriptor())
        .inferences([crate::artifacts::jack::standards::v1::subsets::any::schema::inferences::jack_artifact_inference_descriptor()])
        .composers(crate::artifacts::jack::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::jack::TrinityJackPlayApp>>()
        .try_build()
}
//#endregion 🔖️Register

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::mutations::{create_edge, create_node};
    use crate::artifacts::jack::op::{dispatch_trinity_graph_mutations, validate_trinity_graph_operation};
    use store::ArtifactCommand;

    fn mini_fixture() -> JackSnapshot {
        JackSnapshot::with_content(
            JackSnapshot::SCHEMA.into(),
            "mini".into(),
            Some("nakagin".into()),
            Manifest::nakagin_default(),
            Camera::default(),
            vec![
                Node {
                    id: "root".into(),
                    kind: "Piece".into(),
                    name: "core".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: {
                        let mut p = PropertyBag::new();
                        let mut pos = BTreeMap::new();
                        pos.insert("x".into(), PropertyValue::Number(0.0));
                        pos.insert("y".into(), PropertyValue::Number(0.0));
                        pos.insert("z".into(), PropertyValue::Number(0.0));
                        p.insert("position".into(), PropertyValue::Object(pos));
                        p
                    },
                    ports: vec![Port { id: "out-a".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                Node {
                    id: "child".into(),
                    kind: "Piece".into(),
                    name: "capsule".into(),
                    x: 120.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in-a".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
            ],
            vec![Edge {
                id: "e1".into(),
                kind: "Connection".into(),
                source: "root@out-a".into(),
                target: "child@in-a".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(1.2));
                    p.insert("v".into(), PropertyValue::Number(-0.6));
                    p
                },
            }],
            Some("root".into()),
        )
    }

    #[test]
    fn manifest_nakagin_has_piece_and_connection() {
        let m = Manifest::nakagin_default();
        assert!(m.node_kind("Piece").is_some());
        assert!(m.edge_kind("Connection").is_some());
    }

    #[test]
    fn fixture_loads_manifest_id_only() {
        let json = r#"{"schema":"trinity.graph","name":"mini","manifestId":"nakagin","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
        let graph = Graph::load_json(json).unwrap();
        assert!(graph.manifest.node_kind("Piece").is_some());
    }

    #[test]
    fn fixture_round_trip() {
        let fixture = mini_fixture();
        let json = fixture.to_json().unwrap();
        let back = JackSnapshot::from_json(&json).unwrap();
        assert_eq!(back.nodes().len(), 2);
        assert_eq!(back.edges().len(), 1);
    }

    #[test]
    fn remove_node_cascades_edges() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        assert!(g.remove_node("root"));
        assert!(g.edges.is_empty());
        assert!(g.nodes.contains_key("child"));
    }

    #[test]
    fn graph_op_create_node_and_undo() {
        let fixture = mini_fixture();
        let mut store = crate::artifacts::jack::op::TrinityGraphStore::new(crate::artifacts::jack::op::create_trinity_graph_envelope("test", fixture));
        dispatch_trinity_graph_mutations(&mut store, vec![create_node(Node { id: "new".into(), kind: "Piece".into(), name: "new-piece".into(), x: 200.0, y: 40.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] })])
            .expect("create");
        assert_eq!(store.snapshot().expect("projection").nodes().len(), 3);
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("projection").nodes().len(), 2);
    }

    #[test]
    fn graph_op_dispatch_validates_create_edge_batch_incrementally() {
        let fixture = mini_fixture();
        let mut nodes = fixture.nodes();
        while nodes.len() < 9 {
            nodes.push(Node { id: format!("pad-{}", nodes.len()), kind: "Piece".into(), name: format!("pad-{}", nodes.len()), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] });
        }
        let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, fixture.edges(), fixture.root_node_id.clone());
        let mut store = crate::artifacts::jack::op::TrinityGraphStore::new(crate::artifacts::jack::op::create_trinity_graph_envelope("test", fixture));
        dispatch_trinity_graph_mutations(
            &mut store,
            vec![
                create_node(Node {
                    id: "x-9".into(),
                    kind: "Piece".into(),
                    name: "x".into(),
                    x: 1080.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                }),
                create_node(Node {
                    id: "y-10".into(),
                    kind: "Piece".into(),
                    name: "y".into(),
                    x: 1200.0,
                    y: 80.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                }),
                create_edge(Edge { id: "e-batch".into(), kind: "Connection".into(), source: port_key("x-9", "out"), target: port_key("y-10", "in"), properties: PropertyBag::new() }),
            ],
        )
        .expect("batch create edge");
        let projection = store.snapshot().expect("projection");
        assert_eq!(projection.nodes().len(), 11);
        assert_eq!(projection.edges().len(), 2);
    }

    #[test]
    fn graph_op_rejects_unknown_node_kind() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&create_node(Node { id: "new".into(), kind: "Piece2".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] }), &fixture)
            .expect_err("unknown kind");
        assert!(err.to_string().contains("unknown node kind"));
    }

    #[test]
    fn from_json_rejects_wrong_schema() {
        let json = r#"{"schema":"bogus","name":"x","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
        let err = JackSnapshot::from_json(json).expect_err("schema mismatch");
        assert!(err.to_string().contains("expected schema trinity.graph"));
    }

    #[test]
    fn resolve_manifest_errors_when_missing_and_empty() {
        let mut fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "x".into(), None, Manifest::default(), Camera::default(), vec![], vec![], None);
        let err = fixture.resolve_manifest().expect_err("missing manifest");
        assert!(matches!(err, TrinityRamError::ManifestMissing));
    }

    #[test]
    fn resolve_manifest_errors_on_unknown_id() {
        let mut fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "x".into(), Some("nope".into()), Manifest::default(), Camera::default(), vec![], vec![], None);
        let err = fixture.resolve_manifest().expect_err("unknown manifest id");
        assert!(err.to_string().contains("unknown manifest id nope"));
    }

    #[test]
    fn graph_from_fixture_rejects_port_kind_not_declared_on_node_kind() {
        let fixture = mini_fixture();
        let mut nodes = fixture.nodes();
        nodes[0].ports.push(Port { id: "bad".into(), kind: "core circular bottom".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
        let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, fixture.edges(), fixture.root_node_id.clone());
        let err = Graph::from_fixture(fixture).expect_err("undeclared port kind");
        assert!(matches!(err, TrinityRamError::PortKindNotDeclaredOnFixture { .. }));
        assert!(err.to_string().contains("root"));
    }

    #[test]
    fn graph_accessors_and_mutators() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        assert!(g.node("root").is_some());
        assert!(g.node("ghost").is_none());
        assert!(g.edge("e1").is_some());
        g.node_mut("root").unwrap().name = "renamed".into();
        assert_eq!(g.node("root").unwrap().name, "renamed");

        g.add_node(Node { id: "extra".into(), kind: "Piece".into(), name: "extra".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, properties: PropertyBag::new(), ports: vec![] });
        assert!(g.node("extra").is_some());

        g.add_edge(Edge { id: "e2".into(), kind: "Connection".into(), source: "root@out-a".into(), target: "extra@in-a".into(), properties: PropertyBag::new() });
        assert!(g.edge("e2").is_some());
        assert!(g.remove_edge("e2"));
        assert!(!g.remove_edge("e2"));
    }

    #[test]
    fn graph_remove_node_clears_root_node_id() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        assert!(g.remove_node("root"));
        assert!(g.edges.is_empty());
        assert!(g.nodes.contains_key("child"));
        assert!(g.root_node_id.is_none());
        assert!(!g.remove_node("root"));
    }

    #[test]
    fn graph_set_property_success_and_errors() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        g.set_property(EntityRef::Node("root".into()), "label", PropertyValue::String("hi".into())).expect("set node prop");
        assert_eq!(g.node("root").unwrap().properties.get("label"), Some(&PropertyValue::String("hi".into())));
        let err = g.set_property(EntityRef::Node("ghost".into()), "label", PropertyValue::Null).expect_err("missing node");
        assert!(matches!(err, TrinityRamError::NodeNotFound(_)));

        g.set_property(EntityRef::Edge("e1".into()), "gap", PropertyValue::Number(1.0)).expect("set edge prop");
        assert_eq!(g.edge("e1").unwrap().properties.get("gap"), Some(&PropertyValue::Number(1.0)));
        let err = g.set_property(EntityRef::Edge("ghost".into()), "gap", PropertyValue::Null).expect_err("missing edge");
        assert!(matches!(err, TrinityRamError::EdgeNotFound(_)));
    }

    #[test]
    fn graph_to_fixture_and_fixture_json() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let fixture = g.to_fixture();
        assert_eq!(fixture.nodes().len(), 2);
        assert_eq!(fixture.manifest_id.as_deref(), Some("nakagin"));
        let json = g.fixture_json().expect("fixture json");
        assert!(json.contains("\"schema\""));
    }

    #[test]
    fn subgraph_fixture_filters_entities_and_keeps_root_when_included() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let node_ids: BTreeSet<String> = ["root".to_string()].into_iter().collect();
        let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
        assert_eq!(sub.nodes().len(), 1);
        assert!(sub.edges().is_empty());
        assert_eq!(sub.root_node_id.as_deref(), Some("root"));
        assert!(sub.name.contains("subgraph"));
    }

    #[test]
    fn subgraph_fixture_drops_root_when_not_included() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let node_ids: BTreeSet<String> = ["child".to_string()].into_iter().collect();
        let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
        assert!(sub.root_node_id.is_none());
    }

    #[test]
    fn port_key_helpers_handle_malformed_keys() {
        assert_eq!(parse_port_key("node@port"), Some(("node", "port")));
        assert_eq!(parse_port_key("noport"), None);
        assert_eq!(parse_port_key("@port"), None);
        assert_eq!(parse_port_key("node@"), None);
        assert_eq!(port_node_id("node@port"), Some("node"));
        assert_eq!(port_port_id("node@port"), Some("port"));
        assert_eq!(port_key("a", "b"), "a@b");
    }
}
// #endregion 🧪️Tests
