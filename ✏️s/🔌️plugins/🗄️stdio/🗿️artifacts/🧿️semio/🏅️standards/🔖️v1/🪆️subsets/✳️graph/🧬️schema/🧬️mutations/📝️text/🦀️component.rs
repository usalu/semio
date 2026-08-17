//! ⚡️ Semio graph artifact — hand-rolled `OpText` for `SemioGraphMutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` — the wire-text codec
//! stays handcrafted here, one keyword per semantic verb, grammar `keyword:arg1,arg2,...`
//! (`✳️text`'s own hex/bracket-encoded value convention, reused so this facet's grammar can lean on
//! the shared `hex` macro instead of a quoted-string production).

pub use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{
    add_node_port::mutation::AddNodePort, add_node_property::mutation::AddNodeProperty, change_node_kind::mutation::ChangeNodeKind, change_node_label::mutation::ChangeNodeLabel, create_edge::mutation::CreateEdge, create_node::mutation::CreateNode,
    delete_edge::mutation::DeleteEdge, delete_node::mutation::DeleteNode, move_node::mutation::MoveNode, remove_node_port::mutation::RemoveNodePort, remove_node_property::mutation::RemoveNodeProperty,
};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphPort, SemioGraphPortKind};
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{dec_semio_value_entry, enc_semio_value_entry};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Primitives
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

fn enc_node_id(id: &GraphNodeId) -> String {
    enc_str(&id.value)
}
fn dec_node_id(s: &str) -> Result<GraphNodeId, String> {
    Ok(GraphNodeId::new(dec_str(s)?))
}
fn enc_edge_id(id: &GraphEdgeId) -> String {
    enc_str(&id.value)
}
fn dec_edge_id(s: &str) -> Result<GraphEdgeId, String> {
    Ok(GraphEdgeId::new(dec_str(s)?))
}

fn enc_point2_fields(p: &SemioPoint2) -> String {
    format!("{},{}", enc_str(&p.x.to_string()), enc_str(&p.y.to_string()))
}
fn dec_f64_hex(s: &str) -> Result<f64, String> {
    dec_str(s)?.parse::<f64>().map_err(|e| e.to_string())
}

fn enc_port_kind(k: SemioGraphPortKind) -> char {
    crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::enc_port_kind(k)
}
fn dec_port_kind(s: &str) -> Result<SemioGraphPortKind, String> {
    crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::dec_port_kind(s)
}
fn enc_port(p: &SemioGraphPort) -> String {
    format!("[{},{}]", enc_str(&p.name), enc_port_kind(p.kind))
}
fn dec_port(s: &str) -> Result<SemioGraphPort, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, kind] = parts.as_slice() else { return Err(format!("port: expected 2 fields, got {}", parts.len())) };
    Ok(SemioGraphPort { name: dec_str(name)?, kind: dec_port_kind(kind)? })
}
fn enc_property(p: &SemioValueEntry) -> String {
    enc_semio_value_entry(p)
}
fn dec_property(s: &str) -> Result<SemioValueEntry, String> {
    dec_semio_value_entry(s)
}

fn dec_ports(s: &str) -> Result<Vec<SemioGraphPort>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_port).collect()
}
fn dec_properties(s: &str) -> Result<Vec<SemioValueEntry>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_property).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️OpText
fn print_graph_mutation(m: &SemioGraphMutation) -> String {
    match m {
        SemioGraphMutation::CreateNode(p) => format!(
            "createNode:{},{},{},{},[{}],[{}]",
            enc_node_id(&p.id),
            enc_str(&p.kind),
            enc_str(&p.label),
            enc_point2_fields(&p.position),
            p.ports.iter().map(enc_port).collect::<Vec<_>>().join(","),
            p.properties.iter().map(enc_property).collect::<Vec<_>>().join(","),
        ),
        SemioGraphMutation::DeleteNode(p) => format!("deleteNode:{}", enc_node_id(&p.id)),
        SemioGraphMutation::ChangeNodeKind(p) => format!("changeNodeKind:{},{}", enc_node_id(&p.id), enc_str(&p.new_kind)),
        SemioGraphMutation::ChangeNodeLabel(p) => format!("changeNodeLabel:{},{}", enc_node_id(&p.id), enc_str(&p.new_label)),
        SemioGraphMutation::MoveNode(p) => format!("moveNode:{},{}", enc_node_id(&p.id), enc_point2_fields(&p.new_position)),
        SemioGraphMutation::AddNodePort(p) => format!("addNodePort:{},{},{}", enc_node_id(&p.node_id), p.index, enc_port(&p.port)),
        SemioGraphMutation::RemoveNodePort(p) => format!("removeNodePort:{},{}", enc_node_id(&p.node_id), p.index),
        SemioGraphMutation::AddNodeProperty(p) => format!("addNodeProperty:{},{},{}", enc_node_id(&p.node_id), p.index, enc_property(&p.property)),
        SemioGraphMutation::RemoveNodeProperty(p) => format!("removeNodeProperty:{},{}", enc_node_id(&p.node_id), p.index),
        SemioGraphMutation::CreateEdge(p) => format!("createEdge:{},{},{},{},{}", enc_edge_id(&p.id), enc_node_id(&p.source), enc_node_id(&p.target), enc_str(&p.kind), enc_str(&p.label)),
        SemioGraphMutation::DeleteEdge(p) => format!("deleteEdge:{}", enc_edge_id(&p.id)),
    }
}

fn parse_graph_mutation(line: &str) -> Result<SemioGraphMutation, String> {
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("graph mutation: missing ':' in {line:?}"))?;
    match tag {
        "createNode" => {
            let parts = split_top_level(rest, ',');
            let [id, kind, label, x, y, ports, properties] = parts.as_slice() else {
                return Err(format!("createNode: expected 7 fields, got {}", parts.len()));
            };
            Ok(SemioGraphMutation::CreateNode(CreateNode {
                id: dec_node_id(id)?,
                kind: dec_str(kind)?,
                label: dec_str(label)?,
                position: SemioPoint2 { x: dec_f64_hex(x)?, y: dec_f64_hex(y)? },
                ports: dec_ports(ports)?,
                properties: dec_properties(properties)?,
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
            let (node_id, index) = rest.split_once(',').ok_or_else(|| "removeNodeProperty: missing comma".to_string())?;
            Ok(SemioGraphMutation::RemoveNodeProperty(RemoveNodeProperty { node_id: dec_node_id(node_id)?, index: parse_usize(index)? }))
        }
        "createEdge" => {
            let parts = split_top_level(rest, ',');
            let [id, source, target, kind, label] = parts.as_slice() else { return Err(format!("createEdge: expected 5 fields, got {}", parts.len())) };
            Ok(SemioGraphMutation::CreateEdge(CreateEdge { id: dec_edge_id(id)?, source: dec_node_id(source)?, target: dec_node_id(target)?, kind: dec_str(kind)?, label: dec_str(label)? }))
        }
        "deleteEdge" => Ok(SemioGraphMutation::DeleteEdge(DeleteEdge { id: dec_edge_id(rest)? })),
        other => Err(format!("graph mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioGraphMutation {
    fn print_op(&self) -> String {
        print_graph_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_graph_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️DemoCases
/// 🌱 One representative value per variant — single source of truth for `ops_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️component.rs` and this file's own round-trip test.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SemioGraphMutation> {
    vec![
        SemioGraphMutation::CreateNode(CreateNode {
            id: GraphNodeId::new("n1"),
            kind: "source".into(),
            label: "Source".into(),
            position: SemioPoint2 { x: 0.0, y: 0.0 },
            ports: vec![SemioGraphPort { name: "out".into(), kind: SemioGraphPortKind::Out }],
            properties: vec![],
        }),
        SemioGraphMutation::DeleteNode(DeleteNode { id: GraphNodeId::new("n1") }),
        SemioGraphMutation::ChangeNodeKind(ChangeNodeKind { id: GraphNodeId::new("n1"), new_kind: "relay".into() }),
        SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: GraphNodeId::new("n1"), new_label: "Renamed".into() }),
        SemioGraphMutation::MoveNode(MoveNode { id: GraphNodeId::new("n1"), new_position: SemioPoint2 { x: 99.0, y: -1.0 } }),
        SemioGraphMutation::AddNodePort(AddNodePort { node_id: GraphNodeId::new("n1"), index: 0, port: SemioGraphPort { name: "in".into(), kind: SemioGraphPortKind::In } }),
        SemioGraphMutation::RemoveNodePort(RemoveNodePort { node_id: GraphNodeId::new("n1"), index: 0 }),
        SemioGraphMutation::AddNodeProperty(AddNodeProperty {
            node_id: GraphNodeId::new("n1"),
            index: 0,
            property: SemioValueEntry { key: "weight".into(), value: crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue::Int { lexeme: "7".into() } },
        }),
        SemioGraphMutation::RemoveNodeProperty(RemoveNodeProperty { node_id: GraphNodeId::new("n1"), index: 0 }),
        SemioGraphMutation::CreateEdge(CreateEdge { id: GraphEdgeId::new("e1"), source: GraphNodeId::new("n1"), target: GraphNodeId::new("n2"), kind: "flow".into(), label: "Main".into() }),
        SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new("e1") }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OpText;

    #[test]
    fn op_text_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <SemioGraphMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
        }
    }
}
//#endregion 🧪️Tests
