//! 🔌️ Wire-literal compiled DAG text notation — delegates all lexing/parsing/printing to
//! `dsl_schema`'s unified `Shape::Wire` grammar (`->`/`<-`/`--`, `{k=v}` double-quoted
//! properties), keeping only this module's own public row types (`WireNode`/`WireEdge`) and
//! its domain-specific rule that an edge's ports are mandatory on both ends — a validation
//! layer on top of the shared parse, not a syntax difference. ~8 downstream crates depend on
//! these exact type/function signatures, unchanged by this unification.

// #region wire

use crate::GraphDslError;
use semio_framework_graph_manifest::{PropertyBag, PropertyValue};

// #region 🔖️WireTypes
/// 🧩️ Neutral node row for wire-literal emission.
#[derive(Clone, Debug, PartialEq)]
pub struct WireNode {
    pub id: String,
    pub kind: String,
    pub port: Option<String>,
    pub properties: PropertyBag,
}

/// 🪢️ Neutral edge row for wire-literal emission.
#[derive(Clone, Debug, PartialEq)]
pub struct WireEdge {
    pub from: String,
    pub from_port: String,
    pub to: String,
    pub to_port: String,
    pub directed: bool,
    pub properties: PropertyBag,
}
// #endregion 🔖️WireTypes

// #region 🔖️PropertyBridge
/// 🌉️ `semio_framework_graph_manifest::PropertyValue` <-> `dsl_core::DslValue` — the two crates'
/// dynamic-JSON-equivalent literal types are structurally identical, so this is a pure reshape.
fn dsl_value_from_property_value(value: &PropertyValue) -> semio_framework_value::DslValue {
    match value {
        PropertyValue::Null => semio_framework_value::DslValue::Null,
        PropertyValue::Bool(b) => semio_framework_value::DslValue::Bool(*b),
        PropertyValue::Number(n) => semio_framework_value::DslValue::float(*n),
        PropertyValue::String(s) => semio_framework_value::DslValue::String(s.clone()),
        PropertyValue::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(dsl_value_from_property_value(item));
            }
            semio_framework_value::DslValue::Array(out)
        }
        PropertyValue::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (k, v) in map {
                out.push((k.clone(), dsl_value_from_property_value(v)));
            }
            semio_framework_value::DslValue::Object(out)
        }
    }
}

fn property_value_from_dsl_value(value: &semio_framework_value::DslValue) -> PropertyValue {
    match value {
        semio_framework_value::DslValue::Null => PropertyValue::Null,
        semio_framework_value::DslValue::Bool(b) => PropertyValue::Bool(*b),
        semio_framework_value::DslValue::Number(n) => PropertyValue::Number(n.as_f64()),
        semio_framework_value::DslValue::String(s) => PropertyValue::String(s.clone()),
        semio_framework_value::DslValue::Bytes(bytes) => PropertyValue::Array(bytes.iter().map(|byte| PropertyValue::Number(f64::from(*byte))).collect()),
        semio_framework_value::DslValue::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(property_value_from_dsl_value(item));
            }
            PropertyValue::Array(out)
        }
        semio_framework_value::DslValue::Object(entries) => {
            let mut out = PropertyBag::new();
            for (k, v) in entries {
                out.insert(k.clone(), property_value_from_dsl_value(v));
            }
            PropertyValue::Object(out)
        }
    }
}

fn properties_to_dsl_object(properties: &PropertyBag) -> semio_framework_value::DslValue {
    let mut out = Vec::with_capacity(properties.len());
    for (k, v) in properties {
        out.push((k.clone(), dsl_value_from_property_value(v)));
    }
    semio_framework_value::DslValue::Object(out)
}

fn properties_from_dsl_value(value: &semio_framework_value::DslValue) -> PropertyBag {
    match value {
        semio_framework_value::DslValue::Object(entries) => {
            let mut out = PropertyBag::new();
            for (k, v) in entries {
                out.insert(k.clone(), property_value_from_dsl_value(v));
            }
            out
        }
        _ => PropertyBag::new(),
    }
}
// #endregion 🔖️PropertyBridge

// #region 🔖️WireLiteral
fn render_wire_line(value: &semio_framework_dsl_record::WireValue) -> String {
    let mut writer = semio_framework_dsl_record::Writer::new();
    semio_framework_dsl_record::print_shape(&semio_framework_dsl_record::FieldValue::Wire(value.clone()), &semio_framework_dsl_record::Shape::Wire, &mut writer);
    writer.render(semio_framework_dsl_record::JoinMode::Inline)
}

/// 📝️ Render wire-literal text from neutral node/edge rows, one unified `dsl_core::Wire`
/// statement per line.
pub fn wire_literal_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> String {
    let mut lines = Vec::new();
    for node in nodes {
        let value = semio_framework_dsl_record::WireValue {
            from: semio_framework_dsl_record::WireNode { id: node.id.clone(), kind: Some(node.kind.clone()), port: node.port.clone() },
            edge: None,
            edge_label: semio_framework_dsl_record::WireEdgeLabel::default(),
            properties: properties_to_dsl_object(&node.properties),
        };
        lines.push(render_wire_line(&value));
    }
    for edge in edges {
        let from_kind = nodes.iter().find(|n| n.id == edge.from).map_or("node", |n| n.kind.as_str());
        let to_kind = nodes.iter().find(|n| n.id == edge.to).map_or("node", |n| n.kind.as_str());
        let value = semio_framework_dsl_record::WireValue {
            from: semio_framework_dsl_record::WireNode { id: edge.from.clone(), kind: Some(from_kind.to_string()), port: Some(edge.from_port.clone()) },
            edge: Some((edge.directed, semio_framework_dsl_record::WireNode { id: edge.to.clone(), kind: Some(to_kind.to_string()), port: Some(edge.to_port.clone()) })),
            edge_label: semio_framework_dsl_record::WireEdgeLabel::default(),
            properties: properties_to_dsl_object(&edge.properties),
        };
        lines.push(render_wire_line(&value));
    }
    lines.join("\n")
}

/// 🔍️ Parse wire-literal text into neutral node/edge rows. Delegates lexing+parsing to
/// `dsl_core::parse_wire_text` (the one unified wire grammar — `->`/`<-` sugar/`--`,
/// `{k=v}` double-quoted properties) one statement (line) at a time, then enforces this
/// module's own DAG domain rule on top: an edge's ports are mandatory on BOTH ends (the
/// shared grammar itself leaves ports optional on every endpoint — that's the engine's
/// business, not a syntax difference this module should encode into the lexer/parser).
pub fn dag_from_wire_literal(text: &str) -> Result<(Vec<WireNode>, Vec<WireEdge>), GraphDslError> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value = semio_framework_dsl_record::parse_wire_text(line)?;
        match value.edge {
            None => nodes.push(WireNode { id: value.from.id, kind: value.from.kind.unwrap_or_else(|| "node".to_string()), port: value.from.port, properties: properties_from_dsl_value(&value.properties) }),
            Some((directed, to)) => {
                let from_port = value.from.port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                let to_port = to.port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                edges.push(WireEdge { from: value.from.id, from_port, to: to.id, to_port, directed, properties: properties_from_dsl_value(&value.properties) });
            }
        }
    }
    Ok((nodes, edges))
}
// #endregion 🔖️WireLiteral

// #region 🔖️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️wire-unit/🦀️.rs");
// #endregion 🔖️Tests
// #endregion wire
