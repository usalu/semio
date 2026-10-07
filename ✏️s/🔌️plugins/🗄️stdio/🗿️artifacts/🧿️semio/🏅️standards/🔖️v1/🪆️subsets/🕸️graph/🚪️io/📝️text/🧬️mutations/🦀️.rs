//! ⚡️ Semio graph artifact — hand-rolled `OpText` for `SemioGraphMutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` — the wire-text codec
//! stays handcrafted here, one keyword per semantic verb, grammar `keyword:arg1,arg2,...`
//! (`🔤️text`'s own hex/bracket-encoded value convention, reused so this facet's grammar can lean on
//! the shared `hex` macro instead of a quoted-string production).

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::graph::schema::mutations::{
    set_snapshot::SetSnapshot,
    add_node_port::AddNodePort, add_node_property::AddNodeProperty, change_node_kind::ChangeNodeKind, change_node_label::ChangeNodeLabel, create_edge::CreateEdge, create_node::CreateNode, delete_edge::DeleteEdge, delete_node::DeleteNode,
    drag_nodes::DragNodes, move_node::MoveNode, remove_node_port::RemoveNodePort, remove_node_property::RemoveNodeProperty, rename_node::RenameNode, resize_node::ResizeNode,
    set_node_property::SetNodeProperty, add_edge_property::AddEdgeProperty, remove_edge_property::RemoveEdgeProperty, set_edge_property::SetEdgeProperty,
};
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphPort};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value_entry};
use crate::standards::v1::subsets::value::io::text::diff::{enc_semio_value_entry};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_node_id(id: &GraphNodeId) -> String {
    enc_str(&id.value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_node_id(s: &str) -> Result<GraphNodeId, String> {
    Ok(GraphNodeId::new(dec_str(s)?))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_edge_id(id: &GraphEdgeId) -> String {
    enc_str(&id.value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_edge_id(s: &str) -> Result<GraphEdgeId, String> {
    Ok(GraphEdgeId::new(dec_str(s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_point2_fields(p: &SemioPoint2) -> String {
    crate::standards::v1::subsets::graph::io::text::snapshot::enc_point2_fields(p)
}
/// 📍️ An optional insert index: `-` when absent (append), else its decimal digits.
fn enc_at(at: Option<usize>) -> String {
    at.map_or_else(|| "-".to_string(), |at| at.to_string())
}
/// 📍️ The inverse of [`enc_at`].
fn dec_at(s: &str) -> Result<Option<usize>, String> {
    if s == "-" { Ok(None) } else { parse_usize(s).map(Some) }
}
/// 🔢️ A width/height in the exact native float spelling `dec_f64_hex` reads back bit for bit.
fn enc_native_f64(value: f64) -> String {
    enc_str(&crate::standards::v1::subsets::base::schema::geometry::native::NativeF64(value).to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_f64_hex(s: &str) -> Result<f64, String> {
    crate::standards::v1::subsets::graph::io::text::snapshot::dec_f64_hex(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_port(p: &SemioGraphPort) -> String {
    crate::standards::v1::subsets::graph::io::text::snapshot::enc_port(p)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_port(s:&str)->Result<SemioGraphPort,String>{crate::standards::v1::subsets::graph::io::text::snapshot::dec_port(s)}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_property(p: &SemioValueEntry) -> String {
    enc_semio_value_entry(p)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_property(s: &str) -> Result<SemioValueEntry, String> {
    dec_semio_value_entry(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_ports(s: &str) -> Result<Vec<SemioGraphPort>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_port).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_properties(s: &str) -> Result<Vec<SemioValueEntry>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_property).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️OpText
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_graph_mutation(m: &SemioGraphMutation) -> String {
    match m {
        SemioGraphMutation::PatchSnapshot(payload) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(&payload.patch),
        SemioGraphMutation::SetSnapshot(p) => format!("setSnapshot:{}", hex_encode(semio_framework_pack_json::to_json_string(&p.snapshot).as_bytes())),
        SemioGraphMutation::CreateNode(p) => format!(
            "createNode:{},{},{},{},{},{},[{}],[{}],{}",
            enc_node_id(&p.id),
            enc_str(&p.kind),
            enc_str(&p.label),
            enc_point2_fields(&p.position),
            enc_native_f64(p.width),
            enc_native_f64(p.height),
            p.ports.iter().map(enc_port).collect::<Vec<_>>().join(","),
            p.properties.iter().map(enc_property).collect::<Vec<_>>().join(","),
            enc_at(p.at),
        ),
        SemioGraphMutation::DeleteNode(p) => format!("deleteNode:{}", enc_node_id(&p.id)),
        SemioGraphMutation::ChangeNodeKind(p) => format!("changeNodeKind:{},{}", enc_node_id(&p.id), enc_str(&p.new_kind)),
        SemioGraphMutation::ChangeNodeLabel(p) => format!("changeNodeLabel:{},{}", enc_node_id(&p.id), enc_str(&p.new_label)),
        SemioGraphMutation::MoveNode(p) => format!("moveNode:{},{}", enc_node_id(&p.id), enc_point2_fields(&p.new_position)),
        SemioGraphMutation::AddNodePort(p) => format!("addNodePort:{},{},{}", enc_node_id(&p.node_id), p.index, enc_port(&p.port)),
        SemioGraphMutation::RemoveNodePort(p) => format!("removeNodePort:{},{}", enc_node_id(&p.node_id), p.index),
        SemioGraphMutation::AddNodeProperty(p) => format!("addNodeProperty:{},{},{}", enc_node_id(&p.node_id), p.index, enc_property(&p.property)),
        SemioGraphMutation::RemoveNodeProperty(p) => format!("removeNodeProperty:{},{}", enc_node_id(&p.node_id), enc_str(&p.key)),
        SemioGraphMutation::CreateEdge(p) => format!("createEdge:{},{},{},{},{},{},{},[{}],{}", enc_edge_id(&p.id), enc_node_id(&p.source), enc_node_id(&p.target), enc_str(&p.kind), enc_str(&p.label),crate::standards::v1::subsets::graph::io::text::snapshot::enc_optional(p.source_port.as_deref()),crate::standards::v1::subsets::graph::io::text::snapshot::enc_optional(p.target_port.as_deref()),p.properties.iter().map(enc_property).collect::<Vec<_>>().join(","), enc_at(p.at)),
        SemioGraphMutation::DeleteEdge(p) => format!("deleteEdge:{}", enc_edge_id(&p.id)),
        SemioGraphMutation::DragNodes(p) => format!("dragNodes:[{}],{},{}", p.targets.iter().map(enc_node_id).collect::<Vec<_>>().join(","), enc_str(&p.dx.to_string()), enc_str(&p.dy.to_string())),
        SemioGraphMutation::SetNodeProperty(p) => format!("setNodeProperty:{},{}", enc_node_id(&p.node_id), enc_property(&SemioValueEntry { key: p.key.clone(), value: p.value.clone() })),
        SemioGraphMutation::ResizeNode(p) => format!("resizeNode:{},{},{}", enc_node_id(&p.id), enc_native_f64(p.width), enc_native_f64(p.height)),
        SemioGraphMutation::RenameNode(p) => format!("renameNode:{},{}", enc_node_id(&p.id), enc_node_id(&p.new_id)),
        SemioGraphMutation::SetEdgeProperty(p) => format!("setEdgeProperty:{},{}", enc_edge_id(&p.edge_id), enc_property(&SemioValueEntry { key: p.key.clone(), value: p.value.clone() })),
        SemioGraphMutation::AddEdgeProperty(p) => format!("addEdgeProperty:{},{},{}", enc_edge_id(&p.edge_id), p.index, enc_property(&p.property)),
        SemioGraphMutation::RemoveEdgeProperty(p) => format!("removeEdgeProperty:{},{}", enc_edge_id(&p.edge_id), enc_str(&p.key)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_graph_mutation(line: &str) -> Result<SemioGraphMutation, String> {
    if let Some(source) = line.strip_prefix("patch-snapshot patch=") {
        let patch = semio_s_artifact_stdio_contract::editing::snapshot_patch_from_hex(source)?;
        return Ok(SemioGraphMutation::PatchSnapshot(crate::standards::v1::subsets::graph::schema::mutations::patch_snapshot::PatchSnapshot { patch }));
    }
    if let Some(payload) = line.strip_prefix("setSnapshot:") {
        let bytes = hex_decode(payload)?;
        let json = String::from_utf8(bytes).map_err(|error| error.to_string())?;
        let parsed = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        let snapshot = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())?;
        return Ok(SemioGraphMutation::SetSnapshot(SetSnapshot { snapshot }));
    }
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("graph mutation: missing ':' in {line:?}"))?;
    match tag {
        "createNode" => {
            let parts = split_top_level(rest, ',');
            let [id, kind, label, x, y, width, height, ports, properties, at] = parts.as_slice() else {
                return Err(format!("createNode: expected 10 fields, got {}", parts.len()));
            };
            Ok(SemioGraphMutation::CreateNode(CreateNode {
                id: dec_node_id(id)?,
                kind: dec_str(kind)?,
                label: dec_str(label)?,
                position: SemioPoint2 { x: dec_f64_hex(x)?, y: dec_f64_hex(y)? },
                width:dec_f64_hex(width)?,height:dec_f64_hex(height)?,
                ports: dec_ports(ports)?,
                properties: dec_properties(properties)?,
                at: dec_at(at)?,
            }))
        }
        "deleteNode" => Ok(SemioGraphMutation::DeleteNode(DeleteNode { id: dec_node_id(rest)? })),
        "changeNodeKind" => {
            let (id, new_kind) = rest.split_once(',').ok_or_else(|| "changeNodeKind: missing comma".to_string())?;
            Ok(SemioGraphMutation::ChangeNodeKind(ChangeNodeKind { id: dec_node_id(id)?, new_kind: dec_str(new_kind)? }))
        }
        "changeNodeLabel" => {
            let (id, new_label) = rest.split_once(',').ok_or_else(|| "changeNodeLabel: missing comma".to_string())?;
            Ok(SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: dec_node_id(id)?, new_label: dec_str(new_label)? }))
        }
        "moveNode" => {
            let parts = split_top_level(rest, ',');
            let [id, x, y] = parts.as_slice() else { return Err(format!("moveNode: expected 3 fields, got {}", parts.len())) };
            Ok(SemioGraphMutation::MoveNode(MoveNode { id: dec_node_id(id)?, new_position: SemioPoint2 { x: dec_f64_hex(x)?, y: dec_f64_hex(y)? } }))
        }
        "addNodePort" => {
            let parts = split_top_level(rest, ',');
            let [node_id, index, port] = parts.as_slice() else { return Err(format!("addNodePort: expected 3 fields, got {}", parts.len())) };
            Ok(SemioGraphMutation::AddNodePort(AddNodePort { node_id: dec_node_id(node_id)?, index: parse_usize(index)?, port: dec_port(port)? }))
        }
        "removeNodePort" => {
            let (node_id, index) = rest.split_once(',').ok_or_else(|| "removeNodePort: missing comma".to_string())?;
            Ok(SemioGraphMutation::RemoveNodePort(RemoveNodePort { node_id: dec_node_id(node_id)?, index: parse_usize(index)? }))
        }
        "addNodeProperty" => {
            let parts = split_top_level(rest, ',');
            let [node_id, index, property] = parts.as_slice() else { return Err(format!("addNodeProperty: expected 3 fields, got {}", parts.len())) };
            Ok(SemioGraphMutation::AddNodeProperty(AddNodeProperty { node_id: dec_node_id(node_id)?, index: parse_usize(index)?, property: dec_property(property)? }))
        }
        "removeNodeProperty" => {
            let (node_id, key) = rest.split_once(',').ok_or_else(|| "removeNodeProperty: missing comma".to_string())?;
            Ok(SemioGraphMutation::RemoveNodeProperty(RemoveNodeProperty { node_id: dec_node_id(node_id)?, key: dec_str(key)? }))
        }
        "createEdge" => {
            let parts = split_top_level(rest, ',');
            let [id, source, target, kind, label, source_port, target_port, properties, at] = parts.as_slice() else { return Err(format!("createEdge: expected 9 fields, got {}", parts.len())) };
            Ok(SemioGraphMutation::CreateEdge(CreateEdge { id: dec_edge_id(id)?, source: dec_node_id(source)?, target: dec_node_id(target)?, kind: dec_str(kind)?, label: dec_str(label)?,source_port:crate::standards::v1::subsets::graph::io::text::snapshot::dec_optional(source_port)?,target_port:crate::standards::v1::subsets::graph::io::text::snapshot::dec_optional(target_port)?,properties:dec_properties(properties)?, at: dec_at(at)? }))
        }
        "deleteEdge" => Ok(SemioGraphMutation::DeleteEdge(DeleteEdge { id: dec_edge_id(rest)? })),
        "dragNodes" => {
            let parts = split_top_level(rest, ',');
            let [targets, dx, dy] = parts.as_slice() else { return Err(format!("dragNodes: expected 3 fields, got {}", parts.len())) };
            let targets = split_top_level(strip_brackets(targets)?, ',').into_iter().filter(|id| !id.is_empty()).map(dec_node_id).collect::<Result<Vec<_>, _>>()?;
            Ok(SemioGraphMutation::DragNodes(DragNodes { targets, dx: dec_f64_hex(dx)?, dy: dec_f64_hex(dy)? }))
        }
        "setNodeProperty" => {
            let parts = split_top_level(rest, ',');
            let [node_id, property] = parts.as_slice() else { return Err(format!("setNodeProperty: expected 2 fields, got {}", parts.len())) };
            let SemioValueEntry { key, value } = dec_property(property)?;
            Ok(SemioGraphMutation::SetNodeProperty(SetNodeProperty { node_id: dec_node_id(node_id)?, key, value }))
        }
        "resizeNode" => {
            let parts = split_top_level(rest, ',');
            let [id, width, height] = parts.as_slice() else { return Err(format!("resizeNode: expected 3 fields, got {}", parts.len())) };
            Ok(SemioGraphMutation::ResizeNode(ResizeNode { id: dec_node_id(id)?, width: dec_f64_hex(width)?, height: dec_f64_hex(height)? }))
        }
        "renameNode" => {
            let (id, new_id) = rest.split_once(',').ok_or_else(|| "renameNode: missing comma".to_string())?;
            Ok(SemioGraphMutation::RenameNode(RenameNode { id: dec_node_id(id)?, new_id: dec_node_id(new_id)? }))
        }
        "setEdgeProperty" => {
            let parts = split_top_level(rest, ',');
            let [edge_id, property] = parts.as_slice() else { return Err(format!("setEdgeProperty: expected 2 fields, got {}", parts.len())) };
            let SemioValueEntry { key, value } = dec_property(property)?;
            Ok(SemioGraphMutation::SetEdgeProperty(SetEdgeProperty { edge_id: dec_edge_id(edge_id)?, key, value }))
        }
        "addEdgeProperty" => {
            let parts = split_top_level(rest, ',');
            let [edge_id, index, property] = parts.as_slice() else { return Err(format!("addEdgeProperty: expected 3 fields, got {}", parts.len())) };
            Ok(SemioGraphMutation::AddEdgeProperty(AddEdgeProperty { edge_id: dec_edge_id(edge_id)?, index: parse_usize(index)?, property: dec_property(property)? }))
        }
        "removeEdgeProperty" => {
            let (edge_id, key) = rest.split_once(',').ok_or_else(|| "removeEdgeProperty: missing comma".to_string())?;
            Ok(SemioGraphMutation::RemoveEdgeProperty(RemoveEdgeProperty { edge_id: dec_edge_id(edge_id)?, key: dec_str(key)? }))
        }
        other => Err(format!("graph mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioGraphMutation {
    fn print_op(&self) -> String {
        print_graph_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_graph_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️DemoCases
/// 🌱 One representative value per variant — single source of truth for `ops_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️.rs` and this file's own round-trip test.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioGraphMutation> {
    use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphPortKind;
    vec![
        SemioGraphMutation::PatchSnapshot(crate::standards::v1::subsets::graph::schema::mutations::patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        SemioGraphMutation::CreateNode(CreateNode {
            id: GraphNodeId::new("n1"),
            kind: "source".into(),
            label: "Source".into(),
            position: SemioPoint2 { x: 0.0, y: 0.0 },width:0.0,height:0.0,
            ports: vec![SemioGraphPort { name: "out".into(), kind: SemioGraphPortKind::Out,category:String::new(),properties:Vec::new() }],
            properties: vec![],
            at: Some(3),
        }),
        SemioGraphMutation::DeleteNode(DeleteNode { id: GraphNodeId::new("n1") }),
        SemioGraphMutation::ChangeNodeKind(ChangeNodeKind { id: GraphNodeId::new("n1"), new_kind: "relay".into() }),
        SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: GraphNodeId::new("n1"), new_label: "Renamed".into() }),
        SemioGraphMutation::MoveNode(MoveNode { id: GraphNodeId::new("n1"), new_position: SemioPoint2 { x: 99.0, y: -1.0 } }),
        SemioGraphMutation::AddNodePort(AddNodePort { node_id: GraphNodeId::new("n1"), index: 0, port: SemioGraphPort { name: "in".into(), kind: SemioGraphPortKind::In,category:String::new(),properties:Vec::new() } }),
        SemioGraphMutation::RemoveNodePort(RemoveNodePort { node_id: GraphNodeId::new("n1"), index: 0 }),
        SemioGraphMutation::AddNodeProperty(AddNodeProperty {
            node_id: GraphNodeId::new("n1"),
            index: 0,
            property: SemioValueEntry { key: "weight".into(), value: crate::standards::v1::subsets::value::schema::snapshot::SemioValue::Int { lexeme: "7".into() } },
        }),
        SemioGraphMutation::RemoveNodeProperty(RemoveNodeProperty { node_id: GraphNodeId::new("n1"), key: "weight".into() }),
        SemioGraphMutation::CreateEdge(CreateEdge { id: GraphEdgeId::new("e1"), source: GraphNodeId::new("n1"), target: GraphNodeId::new("n2"), kind: "flow".into(), label: "Main".into(),source_port:None,target_port:None,properties:Vec::new(), at: None }),
        SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new("e1") }),
        SemioGraphMutation::DragNodes(DragNodes { targets: vec![GraphNodeId::new("n1"), GraphNodeId::new("n2")], dx: 12.5, dy: -4.0 }),
        SemioGraphMutation::SetNodeProperty(SetNodeProperty { node_id: GraphNodeId::new("n1"), key: "weight".into(), value: crate::standards::v1::subsets::value::schema::snapshot::SemioValue::Float { lexeme: "0.25".into() } }),
        SemioGraphMutation::ResizeNode(ResizeNode { id: GraphNodeId::new("n1"), width: 120.5, height: 0.1 }),
        SemioGraphMutation::RenameNode(RenameNode { id: GraphNodeId::new("n1"), new_id: GraphNodeId::new("origin") }),
        SemioGraphMutation::SetEdgeProperty(SetEdgeProperty { edge_id: GraphEdgeId::new("e1"), key: "label".into(), value: crate::standards::v1::subsets::value::schema::snapshot::SemioValue::Str { value: "feeds".into() } }),
        SemioGraphMutation::AddEdgeProperty(AddEdgeProperty { edge_id: GraphEdgeId::new("e1"), index: 1, property: SemioValueEntry { key: "since".into(), value: crate::standards::v1::subsets::value::schema::snapshot::SemioValue::Int { lexeme: "1972".into() } } }),
        SemioGraphMutation::RemoveEdgeProperty(RemoveEdgeProperty { edge_id: GraphEdgeId::new("e1"), key: "label".into() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::graph::schema::mutations::*;
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::graph::schema::mutations::add_edge_property;
use crate::standards::v1::subsets::graph::schema::mutations::add_node_port;
use crate::standards::v1::subsets::graph::schema::mutations::add_node_property;
use crate::standards::v1::subsets::graph::schema::mutations::change_node_kind;
use crate::standards::v1::subsets::graph::schema::mutations::change_node_label;
use crate::standards::v1::subsets::graph::schema::mutations::create_edge;
use crate::standards::v1::subsets::graph::schema::mutations::create_node;
use crate::standards::v1::subsets::graph::schema::mutations::delete_edge;
use crate::standards::v1::subsets::graph::schema::mutations::delete_node;
use crate::standards::v1::subsets::graph::schema::mutations::drag_nodes;
use crate::standards::v1::subsets::graph::schema::mutations::move_node;
use crate::standards::v1::subsets::graph::schema::mutations::remove_node_port;
use crate::standards::v1::subsets::graph::schema::mutations::remove_edge_property;
use crate::standards::v1::subsets::graph::schema::mutations::remove_node_property;
use crate::standards::v1::subsets::graph::schema::mutations::rename_node;
use crate::standards::v1::subsets::graph::schema::mutations::resize_node;
use crate::standards::v1::subsets::graph::schema::mutations::set_edge_property;
use crate::standards::v1::subsets::graph::schema::mutations::set_node_property;
/// 🧬️ Every variant wraps exactly one `protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation>`
/// payload struct declared in the corresponding triad leaf's `🦠️mutation/🦀️.rs`. Eleven
/// triads, in dispatch order: node lifecycle (`create-node`/`delete-node`), node scalar fields
/// (`change-node-kind`/`change-node-label`/`move-node`), node nested collections
/// (`add-node-port`/`remove-node-port`/`add-node-property`/`remove-node-property`), then edge
/// lifecycle (`create-edge`/`delete-edge`).
use crate::standards::v1::subsets::graph::schema::mutations::set_snapshot::SetSnapshot;

/// 📥️ Decodes this facet's own externally-tagged (`{"<VariantName>": {<snake_case payload>}}`)
/// JSON projection — no `#[value(rename_all)]` sits on this enum or its payload structs, which is
/// exactly the shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors
/// carry — into a real [`SemioGraphMutation`]. Payload fields are snake_case (`new_position`, `new_label`) while the snapshot side is
/// camelCase — two different conventions in one specification vector, which is precisely the kind of
/// detail a transcribed Rust literal gets wrong silently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_graph_mutation_json(text: &str) -> Result<SemioGraphMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;
