//! 🔺️ `trinity.graph` artifact — in-memory directed property port graph with compile-time manifest.


pub use crate::artifacts::jack::schema::mutations::TrinityGraphMutation;
pub use crate::artifacts::jack::schema::diff::JackDiff;

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

    pub fn to_json(&self) -> Result<String, TrinityRamError> {
        Ok(serde_json::to_string_pretty(self)?)
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

    pub fn from_json(json: &str) -> Result<Self, TrinityRamError> {
        let mut fixture: Self = serde_json::from_str(json)?;
        fixture.validate_schema()?;
        fixture.resolve_manifest()?;
        Ok(fixture)
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
        let mut nodes = BTreeMap::new();
        for node in fixture.nodes {
            nodes.insert(node.id.clone(), node);
        }
        let mut edges = BTreeMap::new();
        for edge in fixture.edges {
            edges.insert(edge.id.clone(), edge);
        }
        Ok(Self { name: fixture.name, manifest: fixture.manifest, camera: fixture.camera, nodes, edges, root_node_id: fixture.root_node_id })
    }

    pub fn to_fixture(&self) -> JackSnapshot {
        JackSnapshot {
            schema: JackSnapshot::SCHEMA.to_string(),
            name: self.name.clone(),
            manifest_id: Some("nakagin".into()),
            manifest: self.manifest.clone(),
            camera: self.camera.clone(),
            nodes: self.nodes.values().cloned().collect(),
            edges: self.edges.values().cloned().collect(),
            root_node_id: self.root_node_id.clone(),
        }
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
        JackSnapshot { schema: JackSnapshot::SCHEMA.to_string(), name: format!("{} subgraph", self.name), manifest_id: Some("nakagin".into()), manifest: self.manifest.clone(), camera: self.camera.clone(), nodes, edges, root_node_id }
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
    for node in &fixture.nodes {
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
    for edge in &fixture.edges {
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

pub fn empty_trinity_graph_fixture() -> JackSnapshot {
    JackSnapshot { schema: JackSnapshot::SCHEMA.into(), name: "trinity".into(), manifest_id: Some("nakagin".into()), manifest: Manifest::nakagin_default(), camera: Camera::default(), nodes: Vec::new(), edges: Vec::new(), root_node_id: None }
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
/// a plugin `.setup()` callback. `crate::apps::jack::config::schema::register_app_schema()` is the
/// one exception, kept alive via the plugin root's own narrowed `.setup()`: it registers the
/// `TrinityJackPlayApp` CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration`
/// deliberately has no field for (see that struct's own doc).
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.jack")
        .schema(crate::artifacts::jack::schema::jack_artifact_schema_descriptor())
        .inferences([crate::artifacts::jack::standards::v1::subsets::any::schema::inferences::jack_artifact_inference_descriptor()])
        .composers(crate::artifacts::jack::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::jack::TrinityJackPlayApp>()
        .build()
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
        JackSnapshot {
            schema: JackSnapshot::SCHEMA.into(),
            name: "mini".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
            root_node_id: Some("root".into()),
            nodes: vec![
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
            edges: vec![Edge {
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
        }
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
        assert_eq!(back.nodes.len(), 2);
        assert_eq!(back.edges.len(), 1);
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
        dispatch_trinity_graph_mutations(&mut store, vec![create_node(Node { id: "new".into(), kind: "Piece".into(), name: "new-piece".into(), x: 200.0, y: 40.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] })]).expect("create");
        assert_eq!(store.snapshot().expect("projection").nodes.len(), 3);
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("projection").nodes.len(), 2);
    }

    #[test]
    fn graph_op_dispatch_validates_create_edge_batch_incrementally() {
        let mut fixture = mini_fixture();
        while fixture.nodes.len() < 9 {
            fixture.nodes.push(Node { id: format!("pad-{}", fixture.nodes.len()), kind: "Piece".into(), name: format!("pad-{}", fixture.nodes.len()), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] });
        }
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
        assert_eq!(projection.nodes.len(), 11);
        assert_eq!(projection.edges.len(), 2);
    }

    #[test]
    fn graph_op_rejects_unknown_node_kind() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&create_node(Node { id: "new".into(), kind: "Piece2".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] }), &fixture).expect_err("unknown kind");
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
        let mut fixture = JackSnapshot { schema: JackSnapshot::SCHEMA.into(), name: "x".into(), manifest_id: None, manifest: Manifest::default(), camera: Camera::default(), nodes: vec![], edges: vec![], root_node_id: None };
        let err = fixture.resolve_manifest().expect_err("missing manifest");
        assert!(matches!(err, TrinityRamError::ManifestMissing));
    }

    #[test]
    fn resolve_manifest_errors_on_unknown_id() {
        let mut fixture = JackSnapshot { schema: JackSnapshot::SCHEMA.into(), name: "x".into(), manifest_id: Some("nope".into()), manifest: Manifest::default(), camera: Camera::default(), nodes: vec![], edges: vec![], root_node_id: None };
        let err = fixture.resolve_manifest().expect_err("unknown manifest id");
        assert!(err.to_string().contains("unknown manifest id nope"));
    }

    #[test]
    fn graph_from_fixture_rejects_port_kind_not_declared_on_node_kind() {
        let mut fixture = mini_fixture();
        fixture.nodes[0].ports.push(Port { id: "bad".into(), kind: "core circular bottom".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
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
        assert_eq!(fixture.nodes.len(), 2);
        assert_eq!(fixture.manifest_id.as_deref(), Some("nakagin"));
        let json = g.fixture_json().expect("fixture json");
        assert!(json.contains("\"schema\""));
    }

    #[test]
    fn subgraph_fixture_filters_entities_and_keeps_root_when_included() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let node_ids: BTreeSet<String> = ["root".to_string()].into_iter().collect();
        let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
        assert_eq!(sub.nodes.len(), 1);
        assert!(sub.edges.is_empty());
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
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::jack::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("JackComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
