//! 🔺️ Jack graph model: the runtime `Port`, `Node`, `Edge` and `Camera` records, the graph effects a query applies, the entity references, the port key helpers and the graph constants.

use semio_framework_graph_manifest::{PortDirection, PropertyBag, PropertyValue};

// #region 🔖️Runtime
/// 🔌️ Runtime port on a node.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Port {
    pub id: String,
    pub kind: String,
    pub direction: PortDirection,
    #[value(default)]
    pub properties: PropertyBag,
}

/// 🧩️ Runtime node (piece).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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

/// 🎯️ Entity reference for mutations.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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

pub const TRINITY_GRAPH_SCHEMA: &str = "trinity.graph";

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

/// 🧮️ One effect a query applies to its working graph. A run publishes its effects as graph leaves of the document's
/// composed `content` child through its authored effect-preparation behavior, never as parent-lane leaves (design §20.15).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum GraphEffect {
    CreateNode(Node),
    DeleteNode(String),
    CreateEdge(Edge),
    DeleteEdge(String),
    RenameNode { id: String, name: String },
    MoveNode { id: String, x: f64, y: f64 },
    SetProperty { entity: EntityRef, key: String, value: PropertyValue },
    RemoveProperty { entity: EntityRef, key: String },
}

semio_framework_value::artifact_retire_struct!(Camera {x,y,zoom});
