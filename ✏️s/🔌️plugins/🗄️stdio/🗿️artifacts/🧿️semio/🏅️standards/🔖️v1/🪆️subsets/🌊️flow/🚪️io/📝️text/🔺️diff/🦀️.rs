//! 📝️ Text representation codec surface for `s.stdio.semio.flow` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::flow::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{dec_named_triple, enc_named_triple, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_flow_diff(d: &SemioFlowDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.nodes {
        tokens.push(format!("nodes={}", enc_nodes_diff(v)));
    }
    if let Some(v) = &d.edges {
        tokens.push(format!("edges={}", enc_edges_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_flow_diff(line: &str) -> Result<SemioFlowDiff, String> {
    let mut d = SemioFlowDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("nodes=") {
            d.nodes = Some(dec_nodes_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("edges=") {
            d.edges = Some(dec_edges_diff(rest)?);
        } else {
            return Err(format!("flow diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioFlowDiff {
fn print_diff(&self) -> String {
    print_flow_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_flow_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_f64(v: f64) -> String {
    format!("{v}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", enc_f64(p.x), enc_f64(p.y))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: dec_f64(x)?, y: dec_f64(y)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_port_ref(p: &PortRef) -> String {
    format!("[{},{}]", enc_str(&p.node), enc_str(&p.port))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_port_ref(s: &str) -> Result<PortRef, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [node, port] = parts.as_slice() else { return Err(format!("port ref: expected 2 fields, got {}", parts.len())) };
    Ok(PortRef { node: dec_str(node)?, port: dec_str(port)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_param(p: &FlowParam) -> String {
    format!("[{},{}]", enc_str(&p.key), enc_str(&p.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_param(s: &str) -> Result<FlowParam, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("param: expected 2 fields, got {}", parts.len())) };
    Ok(FlowParam { key: dec_str(key)?, value: dec_str(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node(n: &FlowNode) -> String {
    format!("[{},{},{},{},{}]", enc_str(&n.id), enc_str(&n.kind), enc_str(&n.label), format_args!("[{}]", n.params.iter().map(enc_param).collect::<Vec<_>>().join(",")), enc_point2(&n.position))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node(s: &str) -> Result<FlowNode, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, kind, label, params, position] = parts.as_slice() else { return Err(format!("node: expected 5 fields, got {}", parts.len())) };
    let params = split_top_level(strip_brackets(params)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_param).collect::<Result<Vec<_>, String>>()?;
    Ok(FlowNode { id: dec_str(id)?, kind: dec_str(kind)?, label: dec_str(label)?, params, position: dec_point2(position)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge(e: &FlowEdge) -> String {
    format!("[{},{},{},{}]", enc_str(&e.id), enc_port_ref(&e.from), enc_port_ref(&e.to), enc_str(&e.kind))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge(s: &str) -> Result<FlowEdge, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, from, to, kind] = parts.as_slice() else { return Err(format!("edge: expected 4 fields, got {}", parts.len())) };
    Ok(FlowEdge { id: dec_str(id)?, from: dec_port_ref(from)?, to: dec_port_ref(to)?, kind: dec_str(kind)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_param_diff(d: &FlowParamDiff) -> String {
    format!("[{}]", encode_option(&d.value, |v| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_param_diff(s: &str) -> Result<FlowParamDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(FlowParamDiff { value: decode_option(inner, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_params_diff(d: &FlowParamsDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_param_diff, enc_param)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_params_diff(s: &str) -> Result<FlowParamsDiff, String> {
    dec_named_triple(s, dec_str, dec_param_diff, dec_param)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff(d: &FlowNodeDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.kind, |v| enc_str(v)), encode_option(&d.label, |v| enc_str(v)), encode_option(&d.params, enc_params_diff), encode_option(&d.position, enc_point2))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff(s: &str) -> Result<FlowNodeDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [kind, label, params, position] = parts.as_slice() else { return Err(format!("node diff: expected 4 fields, got {}", parts.len())) };
    Ok(FlowNodeDiff { kind: decode_option(kind, dec_str)?, label: decode_option(label, dec_str)?, params: decode_option(params, dec_params_diff)?, position: decode_option(position, dec_point2)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_nodes_diff(d: &FlowNodesDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_node_diff, enc_node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_nodes_diff(s: &str) -> Result<FlowNodesDiff, String> {
    dec_named_triple(s, dec_str, dec_node_diff, dec_node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge_diff(d: &FlowEdgeDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.from, enc_port_ref), encode_option(&d.to, enc_port_ref), encode_option(&d.kind, |v| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge_diff(s: &str) -> Result<FlowEdgeDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [from, to, kind] = parts.as_slice() else { return Err(format!("edge diff: expected 3 fields, got {}", parts.len())) };
    Ok(FlowEdgeDiff { from: decode_option(from, dec_port_ref)?, to: decode_option(to, dec_port_ref)?, kind: decode_option(kind, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edges_diff(d: &FlowEdgesDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_edge_diff, enc_edge)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edges_diff(s: &str) -> Result<FlowEdgesDiff, String> {
    dec_named_triple(s, dec_str, dec_edge_diff, dec_edge)
}
}
pub use diff_codec::*;
