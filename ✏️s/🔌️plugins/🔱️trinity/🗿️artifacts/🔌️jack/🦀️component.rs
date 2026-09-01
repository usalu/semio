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

use graph::manifest::{manifest_by_id, GraphManifest, ManifestValidationError, TrinityManifest};
use std::collections::{BTreeMap, BTreeSet};

pub use graph::manifest::{ManifestValidator, PortDirection, PropertyBag, PropertyDef, PropertyKind, PropertyValue};

/// 📜️ Compile-time trinity manifest (projection of {@link GraphManifest}).
pub type Manifest = TrinityManifest;

//#region ⚠️ Errors
/// ⚠️ Trinity graph fixture, manifest-validation, and mutation errors.
#[derive(Debug)]
pub enum TrinityRamError {
    /// 🧬️ JSON (de)serialization failure.
    Json(String),
    /// 🧭️ VCS store/dispatch failure.
    Vcs(vcs::VcsError),
    /// 🧬️ Persisted mutation diff rejection.
    MutationApply(protocol::MutationApplyError),
    /// 📜️ Compile-time manifest validation failure (path-qualified).
    Manifest(ManifestValidationError),
    SchemaMismatch {
        expected: &'static str,
        actual: String,
    },
    UnknownManifestId(String),
    ManifestMissing,
    NodeNotFound(String),
    EdgeNotFound(String),
    NodeAlreadyExists(String),
    EdgeAlreadyExists(String),
    InvalidSourcePortKey(String),
    InvalidTargetPortKey(String),
    SourceNodeNotFound(String),
    TargetNodeNotFound(String),
    PortKindNotDeclaredOnFixture {
        node_id: String,
        port_kind: String,
        node_kind: String,
    },
    PortKindNotDeclaredOnMutation {
        node_id: String,
        port_id: String,
        port_kind: String,
        node_kind: String,
    },
    UnknownNodeKind {
        kind: String,
    },
    UnknownEdgeKind {
        kind: String,
    },
    UnknownPortKind {
        kind: String,
    },
    UnknownEntityKind {
        path: String,
    },
    UnknownPropertyAtPath {
        path: String,
        key: String,
    },
    PropertyTypeMismatch {
        path: String,
        name: String,
        value_type: String,
    },
    UnknownPropertyInBag {
        path: String,
        key: String,
    },
}

impl std::fmt::Display for TrinityRamError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Vcs(error) => write!(formatter, "{error}"),
            Self::MutationApply(error) => write!(formatter, "{error}"),
            Self::Manifest(error) => write!(formatter, "{}: {}", error.path, error.message),
            Self::SchemaMismatch { expected, actual } => write!(formatter, "expected schema {expected}, got {actual}"),
            Self::UnknownManifestId(id) => write!(formatter, "unknown manifest id {id}"),
            Self::ManifestMissing => formatter.write_str("fixture missing manifest or manifestId"),
            Self::NodeNotFound(id) => write!(formatter, "node {id} not found"),
            Self::EdgeNotFound(id) => write!(formatter, "edge {id} not found"),
            Self::NodeAlreadyExists(id) => write!(formatter, "node {id} already exists"),
            Self::EdgeAlreadyExists(id) => write!(formatter, "edge {id} already exists"),
            Self::InvalidSourcePortKey(key) => write!(formatter, "invalid source port key {key}"),
            Self::InvalidTargetPortKey(key) => write!(formatter, "invalid target port key {key}"),
            Self::SourceNodeNotFound(id) => write!(formatter, "source node {id} not found"),
            Self::TargetNodeNotFound(id) => write!(formatter, "target node {id} not found"),
            Self::PortKindNotDeclaredOnFixture { node_id, port_kind, node_kind } => write!(formatter, "nodes/{node_id}/ports/{port_kind}: port kind {port_kind} not declared on node kind {node_kind}"),
            Self::PortKindNotDeclaredOnMutation { node_id, port_id, port_kind, node_kind } => write!(formatter, "nodes/{node_id}/ports/{port_id}: port kind {port_kind} not declared on node kind {node_kind}"),
            Self::UnknownNodeKind { kind } => write!(formatter, "nodes/{kind}: unknown node kind {kind:?}"),
            Self::UnknownEdgeKind { kind } => write!(formatter, "edges/{kind}: unknown edge kind {kind:?}"),
            Self::UnknownPortKind { kind } => write!(formatter, "ports/{kind}: unknown port kind {kind:?}"),
            Self::UnknownEntityKind { path } => write!(formatter, "{path}: unknown kind"),
            Self::UnknownPropertyAtPath { path, key } => write!(formatter, "{path}: unknown property {key:?}"),
            Self::PropertyTypeMismatch { path, name, value_type } => write!(formatter, "{path}/{name}: property type mismatch for {value_type}"),
            Self::UnknownPropertyInBag { path, key } => write!(formatter, "{path}/{key}: unknown property {key:?}"),
        }
    }
}

impl std::error::Error for TrinityRamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Vcs(error) => std::error::Error::source(error),
            Self::MutationApply(error) => std::error::Error::source(error),
            _ => None,
        }
    }
}

impl From<pack::JsonError> for TrinityRamError {
    fn from(error: pack::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<dsl::ValueError> for TrinityRamError {
    fn from(error: dsl::ValueError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<vcs::VcsError> for TrinityRamError {
    fn from(error: vcs::VcsError) -> Self {
        Self::Vcs(error)
    }
}

impl From<protocol::MutationApplyError> for TrinityRamError {
    fn from(error: protocol::MutationApplyError) -> Self {
        Self::MutationApply(error)
    }
}

/// 🔀️ [`ManifestValidationError`] carries no `std::error::Error` impl of its own (plain path/message struct), so this conversion remains explicit.
impl From<ManifestValidationError> for TrinityRamError {
    fn from(error: ManifestValidationError) -> Self {
        Self::Manifest(error)
    }
}
//#endregion ⚠️ Errors

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.graph` document — jack's `nodes`/`edges`
/// instance data now lives in this composed child's own `nodes`/`edges`, not on `JackSnapshot`.
pub type JackContentChild = store::ArtifactChild<SemioGraphSnapshot>;

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
        properties: vec![SemioValueEntry { key: JACK_NODE_JSON_PROPERTY.into(), value: SemioValue::Str { value: pack::to_json_string(node).unwrap_or_default() } }],
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
                if let Ok(parsed) = pack::from_json_str::<Node>(value) {
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
        label: pack::to_json_string(edge).unwrap_or_default(),
    }
}

/// 🌉 Inverse of [`semio_edge_from_jack_edge`] — falls back to a bare node-id (no port qualifier)
/// edge if `label` isn't valid `Edge` JSON (content authored outside this plugin) — never panics.
fn jack_edge_from_semio_edge(edge: &SemioGraphEdge) -> Edge {
    pack::from_json_str::<Edge>(&edge.label).unwrap_or_else(|_| Edge { id: edge.id.value.clone(), kind: edge.kind.clone(), source: edge.source.value.clone(), target: edge.target.value.clone(), properties: PropertyBag::new() })
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
    let content_json = pack::to_json_string(&snapshot).unwrap_or_default();
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
/// 🌱 Ephemeral node/edge representation owned by one exact composed content child. It is
/// never serialized or process-global and retires with that owner.
#[derive(Clone, Debug, Default)]
pub struct JackWorkingScene {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

/// 📝 Transfers decoded or test-provided content into one exact child owner.
pub fn materialize_jack_content(handle: &mut JackContentChild, nodes: Vec<Node>, edges: Vec<Edge>) {
    handle.set_local_owner(std::sync::Arc::new(JackWorkingScene { nodes, edges }));
}

/// 🔎 Reads only the addressed child owner. A wire-only handle fails soft until host
/// materialization.
pub fn jack_working_scene_for_handle(handle: &JackContentChild) -> JackWorkingScene {
    handle.local_owner::<JackWorkingScene>().map(|scene| scene.as_ref().clone()).unwrap_or_default()
}

/// 🔎 Reads the current document's live nodes/edges off its `content` child handle — the single read
/// call site every mutation diff/inverse/app command in this plugin uses instead of the old
/// `snapshot.nodes`/`.edges` field access.
pub fn jack_working_scene(snapshot: &JackSnapshot) -> JackWorkingScene {
    jack_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints a new content-addressed handle and transfers its scene into that exact owner.
pub fn jack_content_child_with_owner(nodes: Vec<Node>, edges: Vec<Edge>) -> JackContentChild {
    let handle = jack_content_child_handle(&nodes, &edges);
    handle.with_local_owner(std::sync::Arc::new(JackWorkingScene { nodes, edges }))
}
//#endregion 🔖️WorkingScene

// #region 🔖️Runtime
/// 🔌️ Runtime port on a node.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Port {
    pub id: String,
    pub kind: String,
    pub direction: PortDirection,
    #[value(default)]
    pub properties: PropertyBag,
}

/// 🧩️ Runtime node (piece).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    #[value(default)]
    pub width: f64,
    #[value(default)]
    pub height: f64,
    #[value(default)]
    pub properties: PropertyBag,
    #[value(default)]
    pub ports: Vec<Port>,
}

/// 🔗️ Runtime edge (connection).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub target: String,
    #[value(default)]
    pub properties: PropertyBag,
}

/// 📷️ Camera for fixture documents.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
        let value = pack::json!({
            "schema": self.schema,
            "name": self.name,
            "manifestId": self.manifest_id,
            "camera": self.camera,
            "nodes": scene.nodes,
            "edges": scene.edges,
            "rootNodeId": self.root_node_id,
        });
        Ok(pack::json_to_string_pretty(&value))
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
        let value: pack::JsonValue = pack::parse_json(json)?;
        let schema = value.get("schema").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let name = value.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let manifest_id: Option<String> = value.get("manifestId").and_then(|v| v.as_str()).map(str::to_string);
        let camera: Camera = value.get("camera").map(|v| dsl::FromValue::from_value(pack::json_to_dsl_value(v))).transpose()?.unwrap_or_default();
        let nodes: Vec<Node> = value.get("nodes").map(|v| dsl::FromValue::from_value(pack::json_to_dsl_value(v))).transpose()?.unwrap_or_default();
        let edges: Vec<Edge> = value.get("edges").map(|v| dsl::FromValue::from_value(pack::json_to_dsl_value(v))).transpose()?.unwrap_or_default();
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
        Self { schema, name, manifest_id, manifest, camera, content: jack_content_child_with_owner(nodes, edges), root_node_id }
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
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", tag = "entity", content = "id")]
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
/// `io_registry::entries()`'s own `OnceLock` convention. `pub` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, fleet-trinity-recipe): the new declaration
/// tree's `🪆️subsets/✳️any/🦀️component.rs` reads these same five `LanguageSpec`s to build its
/// `NativeCodecs` `LanguagePair`s (see that file's own doc for why it does not delegate to a sibling
/// `io::io()` the way `🗒️note`/`🖍️draw` do).
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
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

/// 🔖️ This artifact's OLD-channel definition (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1).
/// KEPT unread by the new declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
/// debt D1 — deleted repo-wide only once every plugin has migrated, not this pass — `🗒️note`/`🖍️draw`
/// precedent, `📓️terra-fleet-trinity-recipe-report.md`): the real en/de localized names
/// (`"Jack"`/`"Buchse"`) still live only on these `ArtifactCapability` rows.
/// `crate::editor::jack::config::schema::register_app_schema()` is the one exception, kept alive via
/// the plugin root's own narrowed `.setup()`: it registers the `TrinityJackPlayApp` CONFIG/PRESENCE
/// schema, an app-scope concern neither the old nor the new declaration type has a field for.
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
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<TrinityJackPlayApp>>()`
        // derives its extension claim from `<JackSnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`), which is `"trinity"`, not `"jack"`.
        ("s.jack.codec.document-1", "codec", "trinity.graph:trinity", &[("codec", "trinity.graph"), ("extension", "trinity")], None),
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

/// 🌳️ This artifact's declaration tree root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
/// MECHANISM design.md §2, fleet-trinity-recipe) — replaces the old `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).languages(...)
/// .document_codec(...)` chain, deleted outright, no dual channel) as the ONLY registration channel
/// for schema/io/viewer/editor rows. `definition()` (old `ArtifactDefinition`/capability rows, above)
/// is kept per debt D1, and `artifact_kind()` is kept because this crate's own plugin-root
/// `.activation(...)` still reads `artifact_kind().id`; neither has any caller left in this function.
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::TrinityApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.trinity.jack").expect("canonical jack kind"), localization: &[], standards: vec![crate::artifacts::jack::standards::v1::standard()] }
}
//#endregion 🔖️Register

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    trait JackChildOwnerOracle {
        fn expected() -> pack::JsonValue;
    }

    struct SerdeJsonJackChildOwnerOracle;

    impl JackChildOwnerOracle for SerdeJsonJackChildOwnerOracle {
        fn expected() -> pack::JsonValue {
            pack::parse_json(include_str!("🧪️fixtures/🎯️child-owner-isolation.json")).expect("language-neutral Jack child-owner fixture")
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn working_scene_belongs_to_the_exact_content_child() {
        let owned = jack_content_child_with_owner(Vec::new(), Vec::new());
        let wire = pack::json_to_string(&pack::json_from_dsl_value(&dsl::to_dsl_value(&owned).expect("Jack child wire identity"))).into_bytes();
        let reconstructed: JackContentChild = dsl::from_dsl_value(pack::json_to_dsl_value(&pack::parse_json_bytes(&wire).expect("Jack child wire roundtrip"))).expect("Jack child wire roundtrip");
        let observed = pack::json!({
            "ownedHasScene": owned.local_owner::<JackWorkingScene>().is_some(),
            "wireIdentityMatches": owned == reconstructed,
            "wireHasScene": reconstructed.local_owner::<JackWorkingScene>().is_some(),
        });

        assert_eq!(observed, SerdeJsonJackChildOwnerOracle::expected());
    }
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

    #[semio_framework_async_macros::async_test]
    async fn manifest_nakagin_has_piece_and_connection() {
        let m = Manifest::nakagin_default();
        assert!(m.node_kind("Piece").is_some());
        assert!(m.edge_kind("Connection").is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn fixture_loads_manifest_id_only() {
        let json = r#"{"schema":"trinity.graph","name":"mini","manifestId":"nakagin","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
        let graph = Graph::load_json(json).unwrap();
        assert!(graph.manifest.node_kind("Piece").is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn fixture_round_trip() {
        let fixture = mini_fixture();
        let json = fixture.to_json().unwrap();
        let back = JackSnapshot::from_json(&json).unwrap();
        assert_eq!(back.nodes().len(), 2);
        assert_eq!(back.edges().len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_node_cascades_edges() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        assert!(g.remove_node("root"));
        assert!(g.edges.is_empty());
        assert!(g.nodes.contains_key("child"));
    }

    #[semio_framework_async_macros::async_test]
    async fn graph_op_create_node_and_undo() {
        let fixture = mini_fixture();
        let mut store = crate::artifacts::jack::op::TrinityGraphStore::new(crate::artifacts::jack::op::create_trinity_graph_envelope("test", fixture));
        dispatch_trinity_graph_mutations(&mut store, vec![create_node(Node { id: "new".into(), kind: "Piece".into(), name: "new-piece".into(), x: 200.0, y: 40.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] })])
            .expect("create");
        assert_eq!(store.snapshot().expect("projection").nodes().len(), 3);
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("projection").nodes().len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn graph_op_dispatch_validates_create_edge_batch_incrementally() {
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

    #[semio_framework_async_macros::async_test]
    async fn graph_op_rejects_unknown_node_kind() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&create_node(Node { id: "new".into(), kind: "Piece2".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] }), &fixture)
            .expect_err("unknown kind");
        assert!(err.to_string().contains("unknown node kind"));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_json_rejects_wrong_schema() {
        let json = r#"{"schema":"bogus","name":"x","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
        let err = JackSnapshot::from_json(json).expect_err("schema mismatch");
        assert!(err.to_string().contains("expected schema trinity.graph"));
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_manifest_errors_when_missing_and_empty() {
        let mut fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "x".into(), None, Manifest::default(), Camera::default(), vec![], vec![], None);
        let err = fixture.resolve_manifest().expect_err("missing manifest");
        assert!(matches!(err, TrinityRamError::ManifestMissing));
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_manifest_errors_on_unknown_id() {
        let mut fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "x".into(), Some("nope".into()), Manifest::default(), Camera::default(), vec![], vec![], None);
        let err = fixture.resolve_manifest().expect_err("unknown manifest id");
        assert!(err.to_string().contains("unknown manifest id nope"));
    }

    #[semio_framework_async_macros::async_test]
    async fn graph_from_fixture_rejects_port_kind_not_declared_on_node_kind() {
        let fixture = mini_fixture();
        let mut nodes = fixture.nodes();
        nodes[0].ports.push(Port { id: "bad".into(), kind: "core circular bottom".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
        let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, fixture.edges(), fixture.root_node_id.clone());
        let err = Graph::from_fixture(fixture).expect_err("undeclared port kind");
        assert!(matches!(err, TrinityRamError::PortKindNotDeclaredOnFixture { .. }));
        assert!(err.to_string().contains("root"));
    }

    #[semio_framework_async_macros::async_test]
    async fn graph_accessors_and_mutators() {
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

    #[semio_framework_async_macros::async_test]
    async fn graph_remove_node_clears_root_node_id() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        assert!(g.remove_node("root"));
        assert!(g.edges.is_empty());
        assert!(g.nodes.contains_key("child"));
        assert!(g.root_node_id.is_none());
        assert!(!g.remove_node("root"));
    }

    #[semio_framework_async_macros::async_test]
    async fn graph_set_property_success_and_errors() {
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

    #[semio_framework_async_macros::async_test]
    async fn graph_to_fixture_and_fixture_json() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let fixture = g.to_fixture();
        assert_eq!(fixture.nodes().len(), 2);
        assert_eq!(fixture.manifest_id.as_deref(), Some("nakagin"));
        let json = g.fixture_json().expect("fixture json");
        assert!(json.contains("\"schema\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn subgraph_fixture_filters_entities_and_keeps_root_when_included() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let node_ids: BTreeSet<String> = ["root".to_string()].into_iter().collect();
        let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
        assert_eq!(sub.nodes().len(), 1);
        assert!(sub.edges().is_empty());
        assert_eq!(sub.root_node_id.as_deref(), Some("root"));
        assert!(sub.name.contains("subgraph"));
    }

    #[semio_framework_async_macros::async_test]
    async fn subgraph_fixture_drops_root_when_not_included() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let node_ids: BTreeSet<String> = ["child".to_string()].into_iter().collect();
        let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
        assert!(sub.root_node_id.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn port_key_helpers_handle_malformed_keys() {
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
