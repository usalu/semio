//! 📝️ Text representation codec surface for `stdio.semio.graph` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::graph::schema::diff::*;
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
/// 🧪️ Hand-rolled `protocol::DiffCodec` — `graph`'s two collection fields print as
/// `nodes=[<node>,...]`/`edges=[<edge>,...]` joined by `;` (empty string = no-op diff), reusing the
/// snapshot facet's own real hex/bracket node/edge encoders (duplicated locally, same convention
/// every sibling subset's `🔺️diff` facet already establishes — see that facet's own doc comment
/// for why).
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, dec_opt, enc_indexed_triple, enc_opt};
use crate::standards::v1::subsets::base::schema::geometry::{native, SemioPoint2};
use crate::standards::v1::subsets::base::schema::triples::{IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::graph::io::text::snapshot::{dec_f64_hex, dec_node_id, dec_optional, dec_port, dec_property, dec_str, enc_node_id, enc_optional, enc_point2_fields, enc_port, enc_property, enc_str};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value, enc_semio_value};
use crate::standards::v1::subsets::graph::io::text::snapshot::{dec_node};
use crate::standards::v1::subsets::graph::io::text::snapshot::{enc_node};
use crate::standards::v1::subsets::graph::io::text::snapshot::{dec_edge};
use crate::standards::v1::subsets::graph::io::text::snapshot::{enc_edge};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_f64(value: f64) -> String {
    enc_str(&native::NativeF64(value).to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_position(p: &SemioPoint2) -> String {
    format!("[{}]", enc_point2_fields(p))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_position(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("position: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: dec_f64_hex(x)?, y: dec_f64_hex(y)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_properties_diff(d: &SemioGraphPropertiesDiff) -> String {
    enc_indexed_triple(d, |e| format!("[{}]", enc_opt(e.value.as_ref(), enc_semio_value)), enc_property)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_properties_diff(s: &str) -> Result<SemioGraphPropertiesDiff, String> {
    dec_indexed_triple(
        s,
        |e| {
            let parts = split_top_level(strip_brackets(e)?, ',');
            let [value] = parts.as_slice() else { return Err(format!("entry diff: expected 1 field, got {}", parts.len())) };
            Ok(SemioGraphEntryDiff { value: dec_opt(value, dec_semio_value)? })
        },
        dec_property,
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff(d: &SemioGraphNodeDiff) -> String {
    let ports = enc_opt(d.ports.as_ref(), |ports| enc_indexed_triple(ports, |p| enc_port(&p.value), enc_port));
    format!(
        "[{},{},{},{},{},{},{},{}]",
        enc_opt(d.id.as_ref(), enc_node_id),
        enc_opt(d.kind.as_ref(), |v| enc_str(v)),
        enc_opt(d.label.as_ref(), |v| enc_str(v)),
        enc_opt(d.position.as_ref(), enc_position),
        enc_opt(d.width.as_ref(), |v| enc_f64(*v)),
        enc_opt(d.height.as_ref(), |v| enc_f64(*v)),
        ports,
        enc_opt(d.properties.as_ref(), enc_properties_diff)
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff(s: &str) -> Result<SemioGraphNodeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, label, position, width, height, ports, properties] = parts.as_slice() else { return Err(format!("node diff: expected 8 fields, got {}", parts.len())) };
    Ok(SemioGraphNodeDiff {
        id: dec_opt(id, dec_node_id)?,
        kind: dec_opt(kind, dec_str)?,
        label: dec_opt(label, dec_str)?,
        position: dec_opt(position, dec_position)?,
        width: dec_opt(width, dec_f64_hex)?,
        height: dec_opt(height, dec_f64_hex)?,
        ports: dec_opt(ports, |triple| dec_indexed_triple(triple, |p| dec_port(p).map(|value| Replace { value }), dec_port))?,
        properties: dec_opt(properties, dec_properties_diff)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge_diff(d: &SemioGraphEdgeDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        enc_opt(d.source.as_ref(), enc_node_id),
        enc_opt(d.target.as_ref(), enc_node_id),
        enc_opt(d.kind.as_ref(), |v| enc_str(v)),
        enc_opt(d.label.as_ref(), |v| enc_str(v)),
        enc_opt(d.source_port.as_ref(), |v| enc_optional(v.as_deref())),
        enc_opt(d.target_port.as_ref(), |v| enc_optional(v.as_deref())),
        enc_opt(d.properties.as_ref(), enc_properties_diff)
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge_diff(s: &str) -> Result<SemioGraphEdgeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [source, target, kind, label, source_port, target_port, properties] = parts.as_slice() else { return Err(format!("edge diff: expected 7 fields, got {}", parts.len())) };
    Ok(SemioGraphEdgeDiff {
        source: dec_opt(source, dec_node_id)?,
        target: dec_opt(target, dec_node_id)?,
        kind: dec_opt(kind, dec_str)?,
        label: dec_opt(label, dec_str)?,
        source_port: dec_opt(source_port, dec_optional)?,
        target_port: dec_opt(target_port, dec_optional)?,
        properties: dec_opt(properties, dec_properties_diff)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_nodes(nodes: &IndexedTripleDiff<SemioGraphNodeDiff, SemioGraphNode>) -> String {
    format!("[{}]", enc_indexed_triple(nodes, enc_node_diff, enc_node))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_nodes(s: &str) -> Result<IndexedTripleDiff<SemioGraphNodeDiff, SemioGraphNode>, String> {
    dec_indexed_triple(strip_brackets(s)?, dec_node_diff, dec_node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edges(edges: &IndexedTripleDiff<SemioGraphEdgeDiff, SemioGraphEdge>) -> String {
    format!("[{}]", enc_indexed_triple(edges, enc_edge_diff, enc_edge))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edges(s: &str) -> Result<IndexedTripleDiff<SemioGraphEdgeDiff, SemioGraphEdge>, String> {
    dec_indexed_triple(strip_brackets(s)?, dec_edge_diff, dec_edge)
}

/// 🖇️ Joins present fields with `;` — ONE physical line, empty when neither field is present,
/// `nodes=[...]`, `edges=[...]`, or `nodes=[...];edges=[...]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_graph_diff(d: &SemioGraphDiff) -> String {
    let mut parts = Vec::new();
    if let Some(list) = &d.nodes {
        parts.push(format!("nodes={}", enc_nodes(list)));
    }
    if let Some(list) = &d.edges {
        parts.push(format!("edges={}", enc_edges(list)));
    }
    parts.join(";")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_graph_diff(line: &str) -> Result<SemioGraphDiff, String> {
    if line.is_empty() {
        return Ok(SemioGraphDiff::default());
    }
    let mut diff = SemioGraphDiff::default();
    for part in split_top_level(line, ';') {
        if let Some(rest) = part.strip_prefix("nodes=") {
            diff.nodes = Some(dec_nodes(rest)?);
        } else if let Some(rest) = part.strip_prefix("edges=") {
            diff.edges = Some(dec_edges(rest)?);
        } else {
            return Err(format!("graph diff: unknown token {part:?}"));
        }
    }
    Ok(diff)
}

impl protocol::DiffText for SemioGraphDiff {
fn print_diff(&self) -> String {
    print_graph_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_graph_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
