//! 🔺️ SemioGraphDiff — sparse per-field diff over `SemioGraphSnapshot`. `graph` has exactly two
//! mutable top-level fields (`nodes`/`edges`, both id-keyed sets with no user-meaningful display
//! order per `SEMANTIC-MUTATIONS-OVERHAUL`'s `📓️derivation-rules.md` rule 2), so the diff carries
//! two independent `Option<…List>` slots: whole-list-wrappers rebuilt POSITIONALLY from `base` by
//! each mutation triad's own `🔺️diff` leaf (never a generic `between()` re-derivation) — the same
//! shape `✳️text`'s `SemioTextDiff`/`SemioTextRunList` uses for its own id-less `runs` collection.
//! No `snapshot: Option<SemioGraphSnapshot>` full-replace slot anywhere — whole-document replace is
//! `ArtifactStore::reset`, outside history.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️NodeList
/// 📋 Whole-list wrapper for the `nodes` field diff — every mutation triad rebuilds the full
/// `values` vec from `base` and wraps it here (`SemioTextRunList`'s own shape).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SemioGraphNodeList {
    pub values: Vec<SemioGraphNode>,
}
//#endregion 🔖️NodeList

//#region 🔖️EdgeList
/// 📋 Whole-list wrapper for the `edges` field diff — same shape as [`SemioGraphNodeList`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SemioGraphEdgeList {
    pub values: Vec<SemioGraphEdge>,
}
//#endregion 🔖️EdgeList

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.graph.diff")]
pub struct SemioGraphDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<SemioGraphNodeList>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<SemioGraphEdgeList>,
}

impl SemioGraphDiff {
    pub async fn is_empty_diff(&self) -> bool {
        self.nodes.is_none() && self.edges.is_none()
    }
}

impl MutationDiff<SemioGraphSnapshot> for SemioGraphDiff {
    async fn apply(&self, base: &SemioGraphSnapshot) -> protocol::MutationApplyResult<SemioGraphSnapshot> {
        let mut next = base.clone();
        if let Some(list) = &self.nodes {
            next.nodes = list.values.clone();
        }
        if let Some(list) = &self.edges {
            next.edges = list.values.clone();
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        if other.nodes.is_some() {
            self.nodes = other.nodes;
        }
        if other.edges.is_some() {
            self.edges = other.edges;
        }
    }
}

/// 🧮️ `graph`'s own `DiffAlgebra` — required by the `✳️any` envelope's own dispatch (`SemioDiff`
/// delegates `between`/`inverse`/`is_empty` straight through to every wrapped subset's own impl).
/// Whole-list `between`/`inverse` are honest here (not apply-then-capture): `graph` has exactly two
/// mutable fields, so a change is fully described by "the new/old `nodes`/`edges` value", same
/// shape every mutation triad's own `🔺️diff` leaf already produces.
impl protocol::command::DiffAlgebra<SemioGraphSnapshot> for SemioGraphDiff {
    async fn between(base: &SemioGraphSnapshot, other: &SemioGraphSnapshot) -> Self {
        SemioGraphDiff { nodes: (base.nodes != other.nodes).then(|| SemioGraphNodeList { values: other.nodes.clone() }), edges: (base.edges != other.edges).then(|| SemioGraphEdgeList { values: other.edges.clone() }) }
    }
    async fn inverse(&self, base: &SemioGraphSnapshot) -> Self {
        SemioGraphDiff { nodes: self.nodes.as_ref().map(|_| SemioGraphNodeList { values: base.nodes.clone() }), edges: self.edges.as_ref().map(|_| SemioGraphEdgeList { values: base.edges.clone() }) }
    }
    async fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` — `graph`'s two collection fields print as
/// `nodes=[<node>,...]`/`edges=[<edge>,...]` joined by `;` (empty string = no-op diff), reusing the
/// snapshot facet's own real hex/bracket node/edge encoders (duplicated locally, same convention
/// every sibling subset's `🔺️diff` facet already establishes — see that facet's own doc comment
/// for why).
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphPort, SemioGraphPortKind};
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{dec_semio_value_entry, enc_semio_value_entry};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;

async fn enc_node_id(id: &GraphNodeId) -> String {
    enc_str(&id.value)
}
async fn dec_node_id(s: &str) -> Result<GraphNodeId, String> {
    Ok(GraphNodeId::new(dec_str(s)?))
}
async fn enc_edge_id(id: &GraphEdgeId) -> String {
    enc_str(&id.value)
}
async fn dec_edge_id(s: &str) -> Result<GraphEdgeId, String> {
    Ok(GraphEdgeId::new(dec_str(s)?))
}

async fn enc_point2_fields(p: &SemioPoint2) -> String {
    format!("{},{}", enc_str(&p.x.to_string()), enc_str(&p.y.to_string()))
}
async fn dec_f64_hex(s: &str) -> Result<f64, String> {
    dec_str(s)?.parse::<f64>().map_err(|e| e.to_string())
}

async fn enc_port_kind(k: SemioGraphPortKind) -> char {
    crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::enc_port_kind(k)
}
async fn dec_port_kind(s: &str) -> Result<SemioGraphPortKind, String> {
    crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::dec_port_kind(s)
}
async fn enc_port(p: &SemioGraphPort) -> String {
    format!("[{},{}]", enc_str(&p.name), enc_port_kind(p.kind))
}
async fn dec_port(s: &str) -> Result<SemioGraphPort, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, kind] = parts.as_slice() else { return Err(format!("port: expected 2 fields, got {}", parts.len())) };
    Ok(SemioGraphPort { name: dec_str(name)?, kind: dec_port_kind(kind)? })
}
async fn enc_property(p: &SemioValueEntry) -> String {
    enc_semio_value_entry(p)
}
async fn dec_property(s: &str) -> Result<SemioValueEntry, String> {
    dec_semio_value_entry(s)
}

async fn enc_node(n: &SemioGraphNode) -> String {
    let ports = n.ports.iter().map(enc_port).collect::<Vec<_>>().join(",");
    let properties = n.properties.iter().map(enc_property).collect::<Vec<_>>().join(",");
    format!("[{},{},{},{},[{}],[{}]]", enc_node_id(&n.id), enc_str(&n.kind), enc_str(&n.label), enc_point2_fields(&n.position), ports, properties)
}
async fn dec_node(s: &str) -> Result<SemioGraphNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, label, x, y, ports, properties] = parts.as_slice() else {
        return Err(format!("node: expected 7 fields, got {}", parts.len()));
    };
    let ports = split_top_level(strip_brackets(ports)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_port).collect::<Result<Vec<_>, String>>()?;
    let properties = split_top_level(strip_brackets(properties)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_property).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioGraphNode { id: dec_node_id(id)?, kind: dec_str(kind)?, label: dec_str(label)?, position: SemioPoint2 { x: dec_f64_hex(x)?, y: dec_f64_hex(y)? }, ports, properties })
}
async fn enc_edge(e: &SemioGraphEdge) -> String {
    format!("[{},{},{},{},{}]", enc_edge_id(&e.id), enc_node_id(&e.source), enc_node_id(&e.target), enc_str(&e.kind), enc_str(&e.label))
}
async fn dec_edge(s: &str) -> Result<SemioGraphEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, source, target, kind, label] = parts.as_slice() else {
        return Err(format!("edge: expected 5 fields, got {}", parts.len()));
    };
    Ok(SemioGraphEdge { id: dec_edge_id(id)?, source: dec_node_id(source)?, target: dec_node_id(target)?, kind: dec_str(kind)?, label: dec_str(label)? })
}
async fn enc_nodes(list: &SemioGraphNodeList) -> String {
    format!("[{}]", list.values.iter().map(enc_node).collect::<Vec<_>>().join(","))
}
async fn dec_nodes(s: &str) -> Result<SemioGraphNodeList, String> {
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_node).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioGraphNodeList { values })
}
async fn enc_edges(list: &SemioGraphEdgeList) -> String {
    format!("[{}]", list.values.iter().map(enc_edge).collect::<Vec<_>>().join(","))
}
async fn dec_edges(s: &str) -> Result<SemioGraphEdgeList, String> {
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioGraphEdgeList { values })
}

/// 🖇️ Joins present fields with `;` — ONE physical line, empty when neither field is present,
/// `nodes=[...]`, `edges=[...]`, or `nodes=[...];edges=[...]`.
async fn print_graph_diff(d: &SemioGraphDiff) -> String {
    let mut parts = Vec::new();
    if let Some(list) = &d.nodes {
        parts.push(format!("nodes={}", enc_nodes(list)));
    }
    if let Some(list) = &d.edges {
        parts.push(format!("edges={}", enc_edges(list)));
    }
    parts.join(";")
}
async fn parse_graph_diff(line: &str) -> Result<SemioGraphDiff, String> {
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

impl protocol::DiffCodec for SemioGraphDiff {
    async fn print_diff(&self) -> String {
        print_graph_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_graph_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    /// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`nodes`, bit1=`edges`) are two
    /// REAL fixed fields; when present, each list follows as a real varint count + per-record
    /// binary encoding (reusing the snapshot facet's own `write_node`/`read_node`/`write_edge`/
    /// `read_edge`) rather than a text-blob-in-binary shortcut.
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{write_edge, write_node};
        let presence: u8 = (if self.nodes.is_some() { 0b0000_0001 } else { 0 }) | (if self.edges.is_some() { 0b0000_0010 } else { 0 });
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(list) = &self.nodes {
            store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
            for n in &list.values {
                write_node(&mut out, n);
            }
        }
        if let Some(list) = &self.edges {
            store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
            for e in &list.values {
                write_edge(&mut out, e);
            }
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{read_edge, read_node};
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let nodes = if presence & 0b0000_0001 != 0 {
            let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff nodes count", offset: 2, detail: e.to_string() })?;
            let mut values = Vec::with_capacity(count as usize);
            for _ in 0..count {
                values.push(read_node(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff node", offset: 2, detail: e })?);
            }
            Some(SemioGraphNodeList { values })
        } else {
            None
        };
        let edges = if presence & 0b0000_0010 != 0 {
            let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff edges count", offset: 2, detail: e.to_string() })?;
            let mut values = Vec::with_capacity(count as usize);
            for _ in 0..count {
                values.push(read_edge(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff edge", offset: 2, detail: e })?);
            }
            Some(SemioGraphEdgeList { values })
        } else {
            None
        };
        Ok(SemioGraphDiff { nodes, edges })
    }
}
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioGraphDiff` cases — single source of truth for `diff_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️component.rs`.
#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<SemioGraphDiff> {
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::demo_graph_snapshot;
    let demo = demo_graph_snapshot();
    vec![
        SemioGraphDiff::default(),
        SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: demo.nodes.clone() }), edges: None },
        SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: demo.edges.clone() }) },
        SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: demo.nodes }), edges: Some(SemioGraphEdgeList { values: demo.edges }) },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
    use protocol::DiffCodec;

    #[test]
    async fn apply_replaces_nodes_and_edges_wholesale() {
        let base = SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: vec![SemioGraphNode { id: GraphNodeId::new("a"), ..Default::default() }], edges: vec![] };
        let diff = SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: vec![SemioGraphNode { id: GraphNodeId::new("b"), ..Default::default() }] }), edges: None };
        let next = diff.apply(&base).expect("apply must succeed for a well-formed fixture");
        assert_eq!(next.nodes[0].id, GraphNodeId::new("b"));
    }

    #[test]
    async fn absorb_last_write_wins() {
        let mut d1 = SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: vec![SemioGraphNode { id: GraphNodeId::new("a"), ..Default::default() }] }), edges: None };
        let d2 = SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: vec![SemioGraphNode { id: GraphNodeId::new("b"), ..Default::default() }] }), edges: None };
        d1.absorb(d2.clone());
        assert_eq!(d1, d2);
    }

    #[test]
    async fn diff_codec_graph_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioGraphDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioGraphDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }

    #[test]
    async fn empty_diff_prints_empty_string() {
        assert_eq!(SemioGraphDiff::default().print_diff(), "");
    }

    #[test]
    async fn edge_id_helper_smoke() {
        assert_eq!(dec_edge_id("").unwrap(), GraphEdgeId::new(""));
    }
}
//#endregion 🔖️Tests
