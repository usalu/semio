//! 🔺️ `trinity.graph` artifact — in-memory directed property port graph with compile-time manifest.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`trinity→C:graph`, jack side — see this
//! region's own doc for why this is ONE composed child, not two: the design annotation "jack; rewriting
//! = 2 graph children" attributes the two-child shape to the SEPARATE `rewriting` app's LHS/RHS rule
//! windows, not to jack. jack's own persisted `nodes`/`edges` instance data is replaced by a single
//! composed `s.stdio.semio.graph` CHILD slot (`🔖️ContentBridge` below); the compile-time `manifest`
//! (kind/property/port DEFINITIONS, resolved from `manifestId` — see `semio_framework_graph::manifest::GraphManifest`)
//! is NOT graph-shaped in the `SemioGraphSnapshot` sense (that subset is an INSTANCE graph: nodes with
//! position/ports/properties, edges with source/target) and stays an ordinary inline field, unchanged.

#![allow(clippy::unnecessary_wraps)]

#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🦀️.rs"]
pub mod ast;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🦀️.rs"]
pub mod executor;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️.rs"]
pub mod language_service;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔤️lexer/🦀️.rs"]
pub mod lexer;
#[cfg(feature = "component-app-assembly")]
pub use crate::editor::jack::snapshot_to_workflow;
pub use crate::standards::v1::subsets::any::schema::inferences::flat_position::compute_flat_position;
pub use language_service as core;

use semio_framework_graph::manifest::{GraphManifest, ManifestValidationError};
use crate::graph_manifest::manifest_by_id;
use std::collections::{BTreeMap, BTreeSet};

pub use semio_framework_graph::manifest::{ManifestValidator, PortDirection, PropertyBag, PropertyDef, PropertyKind, PropertyValue};

#[path = "🛂️manifest/🦀️.rs"]
pub mod manifest;
pub use manifest::{Manifest, NodeKindDef, EdgeKindDef, PortKindDef};

//#region ⚠️ Errors
/// ⚠️ Trinity graph snapshot, manifest-validation, and mutation errors.
#[derive(Debug)]
pub enum TrinityRamError {
    /// 🧬️ JSON (de)serialization failure.
    Json(String),
    ValueRefusal(semio_framework_value::ValueError),
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
    /// 📏️ A `set-query` beyond [`JACK_QUERY_MAXIMUM_BYTES`].
    QueryTooLarge {
        bytes: usize,
        maximum: usize,
    },
}

impl std::fmt::Display for TrinityRamError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::ValueRefusal(error) => write!(formatter,"{error}"),
            Self::Vcs(error) => write!(formatter, "{error}"),
            Self::MutationApply(error) => write!(formatter, "{error}"),
            Self::Manifest(error) => write!(formatter, "{}: {}", error.path, error.message),
            Self::SchemaMismatch { expected, actual } => write!(formatter, "expected schema {expected}, got {actual}"),
            Self::UnknownManifestId(id) => write!(formatter, "unknown manifest id {id}"),
            Self::ManifestMissing => formatter.write_str("snapshot missing manifest or manifestId"),
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
            Self::QueryTooLarge { bytes, maximum } => write!(formatter, "query: {bytes} bytes exceed the {maximum}-byte bound"),
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

impl From<semio_framework_pack_json::JsonError> for TrinityRamError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<semio_framework_value::ValueError> for TrinityRamError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::ValueRefusal(error)
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

#[path = "🪆️content/🦀️.rs"]
pub mod content;
pub use content::*;

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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub target: String,
    #[value(default)]
    pub properties: PropertyBag,
}

/// 📷️ Camera for snapshot documents.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

impl JackSnapshot {
    pub const SCHEMA: &'static str = "trinity.graph";

    pub fn validate_schema(&self) -> Result<(), TrinityRamError> {
        if self.schema != Self::SCHEMA {
            return Err(TrinityRamError::SchemaMismatch { expected: Self::SCHEMA, actual: self.schema.clone() });
        }
        Ok(())
    }

    /// 📤️ Encodes the actual Jack parent; composed Semio content travels through its independent child owner.
    pub fn to_json(&self) -> Result<String, TrinityRamError> {
        let value = crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_value::ToValue::to_value(self), false)?;
        Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&value)))
    }

    pub fn resolve_manifest(&mut self) -> Result<(), TrinityRamError> {
        if let Some(id) = self.manifest_id.as_deref() {
            self.manifest = Manifest::from_graph(&manifest_by_id(id).ok_or_else(|| TrinityRamError::UnknownManifestId(id.to_string()))?);
            return Ok(());
        }
        if self.manifest.node_kinds.is_empty() && self.manifest.edge_kinds.is_empty() && self.manifest.port_kinds.is_empty() {
            return Err(TrinityRamError::ManifestMissing);
        }
        Ok(())
    }

    /// 📥️ Decodes the literal parent with its unresolved content address; the host supplies the exact child owner.
    pub fn from_json(json: &str) -> Result<Self, TrinityRamError> {
        let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::decode_jack_snapshot_json(json)?;
        snapshot.validate_schema()?;
        Ok(snapshot)
    }

    /// 🏗️ Transfers one working scene into the snapshot's exact composed content owner. The query starts empty: a document
    /// constructor names its own (`Self { query, ..Self::with_content(..) }`).
    pub fn with_content(schema: String, name: String, manifest_id: Option<String>, manifest: Manifest, camera: Camera, scene: JackWorkingScene, root_node_id: Option<String>) -> Self {
        Self { schema, name, manifest_id, manifest, camera, content: jack_content_child_with_owner(scene.nodes, scene.edges), root_node_id, query: String::new() }
    }

    /// 🔎 Live node list, read through the working-scene cache — replaces the old direct `.nodes`
    /// field access (see `🔖️WorkingScene`'s module doc for why this indirection exists).
    pub fn nodes(&self) -> Result<Vec<Node>, semio_framework_value::ValueError> {
        Ok(jack_working_scene(self)?.nodes)
    }

    /// 🔎 Live edge list, read through the working-scene cache.
    pub fn edges(&self) -> Result<Vec<Edge>, semio_framework_value::ValueError> {
        Ok(jack_working_scene(self)?.edges)
    }
}

/// 🧠️ In-memory trinity graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    pub name: String,
    pub manifest_id: Option<String>,
    pub manifest: Manifest,
    pub camera: Camera,
    pub nodes: BTreeMap<String, Node>,
    pub edges: BTreeMap<String, Edge>,
    pub root_node_id: Option<String>,
    pub query: String,
}

impl Graph {
    pub fn from_snapshot(snapshot: JackSnapshot) -> Result<Self, TrinityRamError> {
        let scene = jack_working_scene(&snapshot)?;
        Self::with_scene(snapshot, scene)
    }

    /// 🧸️ The in-memory graph of a parent document over the scene its composed `content` child holds.
    pub fn with_scene(mut snapshot: JackSnapshot, scene: JackWorkingScene) -> Result<Self, TrinityRamError> {
        snapshot.validate_schema()?;
        snapshot.resolve_manifest()?;
        if let Some(id) = snapshot.manifest_id.as_deref() {
            if let Some(gm) = manifest_by_id(id) {
                validate_trinity_scene(&gm, &scene)?;
            }
        }
        let mut nodes = BTreeMap::new();
        for node in scene.nodes {
            nodes.insert(node.id.clone(), node);
        }
        let mut edges = BTreeMap::new();
        for edge in scene.edges {
            edges.insert(edge.id.clone(), edge);
        }
        Ok(Self { name: snapshot.name, manifest_id: snapshot.manifest_id, manifest: snapshot.manifest, camera: snapshot.camera, nodes, edges, root_node_id: snapshot.root_node_id, query: snapshot.query })
    }

    pub fn to_snapshot(&self) -> JackSnapshot {
        JackSnapshot { query: self.query.clone(), ..JackSnapshot::with_content(JackSnapshot::SCHEMA.to_string(), self.name.clone(), self.manifest_id.clone(), self.manifest.clone(), self.camera.clone(), JackWorkingScene { nodes: self.nodes.values().cloned().collect(), edges: self.edges.values().cloned().collect() }, self.root_node_id.clone()) }
    }

    pub fn load_json(json: &str) -> Result<Self, TrinityRamError> {
        Self::from_snapshot(JackSnapshot::from_json(json)?)
    }

    pub fn host_snapshot_json(&self) -> Result<String, TrinityRamError> {
        self.to_snapshot().to_json()
    }

    /// 🧩️ Build a `trinity.graph` snapshot containing only the given node and edge ids.
    pub fn subgraph_snapshot(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> JackSnapshot {
        let nodes: Vec<Node> = node_ids.iter().filter_map(|id| self.nodes.get(id).cloned()).collect();
        let edges: Vec<Edge> = edge_ids.iter().filter_map(|id| self.edges.get(id).cloned()).collect();
        let root_node_id = self.root_node_id.clone().filter(|id| node_ids.contains(id));
        JackSnapshot::with_content(JackSnapshot::SCHEMA.to_string(), format!("{} subgraph", self.name), self.manifest_id.clone(), self.manifest.clone(), self.camera.clone(), JackWorkingScene { nodes: nodes, edges: edges }, root_node_id)
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

/// 🛡️ Validates trinity snapshot instances against a compile-time graph manifest.
fn validate_trinity_scene(gm: &GraphManifest, scene: &JackWorkingScene) -> Result<(), TrinityRamError> {
    let validator = ManifestValidator::new(gm);
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

/// 🔎️ The Jack query a new document opens with — `JackSnapshot::query` is document content, undoable and shared like the graph.
pub const TRINITY_JACK_DEFAULT_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";

/// 📏️ Largest query text a jack document holds: one `set-query` stays inside the 4 KiB document-mutation admission
/// (`TRINITY_JACK_ARTIFACT_MUTATION_MAXIMUM_BYTES`) and the store initializer's 4 KiB owned-field bound.
pub const JACK_QUERY_MAXIMUM_BYTES: usize = 3_584;

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (editor AND viewer) of this artifact shares — lives at the ARTIFACT level, not under
/// `editor`, so a viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.trinity.jack"` matches `#[artifact_schema(id = "s.trinity.jack")]` in this
/// subset's own `🧬️schema/🦀️component.rs`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.trinity.jack@1/*#editor` / `s.trinity.jack@1/*#viewer` (contract §1 grammar).
pub const TRINITY_JACK_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.trinity.jack", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };

pub fn empty_trinity_graph_snapshot() -> JackSnapshot {
    JackSnapshot { query: TRINITY_JACK_DEFAULT_QUERY.into(), ..JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "trinity".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: Vec::new(), edges: Vec::new() }, None) }
}

/// 🎯️ `ArtifactKindSpec` identity shared by every `jack`-family app that mounts this artifact.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "graph.trinity".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Trinity Graph", "Trinity-Graph"),
        source_format: "trinity.graph".into(),
        component_kind: "trinity".into(),
        dimension: "graph".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Graph, form: semio_framework_plugin::MediaForm::Trinity },
        schema: "trinity.graph".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}
// #endregion 🔖️Runtime

//#region 🔖️Register
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `io_registry::entries()`'s own `OnceLock` convention. `pub` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, fleet-trinity-recipe): the new declaration
/// tree's `🪆️subsets/✳️any/🦀️.rs` reads these same five `LanguageSpec`s to build its
/// `NativeCodecs` `LanguagePair`s (see that file's own doc for why it does not delegate to a sibling
/// `crate::standards::v1::subsets::any::io::io()` the way `🗒️note`/`🖍️draw` do).
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "jack.document",
                    extension: Some("trinity"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("jack.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.spr"),
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
/// Jack query source and execution output now register through their exact window owners.
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::jack::TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::jack::TrinityJackViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::jack::TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::jack::TrinityJackViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.trinity.jack.standard.v1", "standard", "1", &[], None),
        ("s.trinity.jack.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.trinity.jack.schema.artifact", "schema", "s.trinity.jack", &[("schema", "s.trinity.jack")], None),
        ("s.trinity.jack.inference.artifact", "inference", "s.trinity.jack.inference", &[("schema", "s.trinity.jack.inference")], None),
        ("s.trinity.jack.composer.native", "composer", "s.trinity.jack@1/*", &[("dialect", "s.trinity.jack@1/*")], None),
        ("s.trinity.jack.composer.format-5", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.trinity.jack.grammar.1", "grammar", "jack.document", &[("grammar", "jack.document")], None),
        ("s.trinity.jack.grammar.2", "grammar", "jack.op", &[("grammar", "jack.op")], None),
        ("s.trinity.jack.grammar.3", "grammar", "jack.diff", &[("grammar", "jack.diff")], None),
        ("s.trinity.jack.grammar.4", "grammar", "jack.pack", &[("grammar", "jack.pack")], None),
        ("s.trinity.jack.grammar.5", "grammar", "jack.spr", &[("grammar", "jack.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<TrinityJackPlayApp>>()`
        // derives its extension claim from `<JackSnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🚪️io/📝️text/📸️snapshot/🦀️.rs`), which is `"trinity"`, not `"jack"`.
        ("s.trinity.jack.codec.document-1", "codec", "trinity.graph:trinity", &[("codec", "trinity.graph"), ("codec-extension", "13:trinity.graph:trinity")], None),
        ("s.trinity.jack.localization.en", "localization", "Jack", &[], Some(("en", "Jack"))),
        ("s.trinity.jack.localization.de", "localization", "Buchse", &[], Some(("de", "Buchse"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.trinity.jack")?);
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
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.trinity.jack").expect("canonical jack kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}
//#endregion 🔖️Register

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[cfg(feature = "component-app-assembly")]
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        #[cfg(feature = "component-app-assembly")]
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[cfg(feature = "component-app-assembly")]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                #[cfg(feature = "component-app-assembly")]
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod flat_position {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod operations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod set_query {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🧪️tests/🔎️replaces-the-query/🦀️.rs"]
                            mod tests_replaces_the_query;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::schema::diff::JackDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
pub use executor::GraphEffect;
pub use crate::standards::v1::subsets::any::schema::operations::*;
/// 📸️ Persisted Jack snapshot shared by the artifact's schema and app surfaces.
pub use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jack {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️lod/🦀️.rs"]
        pub mod lod;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🦀️.rs"]
        pub mod window_config;

        #[path = "."]
        pub mod transient {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub(crate) mod query {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            mod load_document_json_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️load-document-json/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use load_document_json_leaf::load_document_json;

            #[path = "."]
            mod delete_selection_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use delete_selection_leaf::delete_selection;

            #[path = "."]
            mod patch_nodes_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-nodes/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use patch_nodes_leaf::patch_nodes;

            #[path = "."]
            mod set_active_example_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-active-example/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_active_example_leaf::{preset_query, set_active_example};

            #[path = "."]
            mod format_document_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✨️format-document/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use format_document_leaf::format_document;

            #[path = "."]
            mod set_viewport_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖥️set-viewport/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_viewport_leaf::set_viewport;

            #[path = "."]
            mod text_edit_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️text-edit/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use text_edit_leaf::text_edit;

            #[path = "."]
            mod set_lod_mode_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️set-lod/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_lod_mode_leaf::set_lod_mode;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs"]
                    pub mod reorganize;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub(crate) mod graph {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod editor {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod results {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub(crate) mod document {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jack {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️graph/🦀️.rs"]
                    pub mod graph;
                }
            }
        }
    }
}

//#region 🧬️ChildRestoreProjection
/// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields — the ONE
/// definition the editor and the viewer both declare. The `ArtifactEditor`/`ArtifactViewer` trait default
/// refuses it (`… did not declare a loaded-parent child projection`), and since PX1 the live envelope load
/// asks for it before the decoded document may replace the store, so an undeclared app fails every live load.
pub fn jack_child_restore_projection(snapshot: &crate::JackSnapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("jack.child-projection"), error.to_string()))
}
//#endregion 🧬️ChildRestoreProjection

#[cfg(test)]
#[path = "🛂️manifest/🧪️tests/🔬️unit/🦀️.rs"]
mod graph_manifest_tests;

#[cfg(test)]
#[path = "🧪️tests/🪆️record-owner/🦀️.rs"]
mod canonical_record_owner_tests;

#[path = "🔨️modules/🏠️host/🦀️.rs"]
pub mod host;

pub use crate::standards::v1::subsets::any::io::{JackBuilderConstruction, JackParts, JackAnalyzerAnalysis, JackBuilderFacets, JackBuilder, JackAnalyzer, JackComposer};
