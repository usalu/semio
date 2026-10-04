//! 🔺️ SemioGraphDiff — sparse per-field diff over `SemioGraphSnapshot`. `graph` has exactly two
//! mutable top-level fields (`nodes`/`edges`, both id-keyed sets with no user-meaningful display
//! order per `SEMANTIC-MUTATIONS-OVERHAUL`'s `📓️derivation-rules.md` rule 2), so the diff carries
//! two independent `Option<…List>` slots: whole-list-wrappers rebuilt POSITIONALLY from `base` by
//! each mutation triad's own `🔺️diff` leaf (never a generic `between()` re-derivation) — the same
//! shape `🔤️text`'s `SemioTextDiff`/`SemioTextRunList` uses for its own id-less `runs` collection.
//! No `snapshot: Option<SemioGraphSnapshot>` full-replace slot anywhere — whole-document replace is
//! `ArtifactStore::reset`, outside history.

use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️NodeList
/// 📋 Whole-list wrapper for the `nodes` field diff — every mutation triad rebuilds the full
/// `values` vec from `base` and wraps it here (`SemioTextRunList`'s own shape).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioGraphNodeList {
    pub values: Vec<SemioGraphNode>,
}
//#endregion 🔖️NodeList

//#region 🔖️EdgeList
/// 📋 Whole-list wrapper for the `edges` field diff — same shape as [`SemioGraphNodeList`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioGraphEdgeList {
    pub values: Vec<SemioGraphEdge>,
}
//#endregion 🔖️EdgeList

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.graph.diff")]
pub struct SemioGraphDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<SemioGraphNodeList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<SemioGraphEdgeList>,
}

impl SemioGraphDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.nodes.is_none() && self.edges.is_none()
    }
}

impl MutationDiff<SemioGraphSnapshot> for SemioGraphDiff {
    fn apply(&self, base: &SemioGraphSnapshot) -> protocol::MutationApplyResult<SemioGraphSnapshot> {
        let mut next = base.clone();
        if let Some(list) = &self.nodes {
            next.nodes = list.values.clone();
        }
        if let Some(list) = &self.edges {
            next.edges = list.values.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.nodes.is_some() {
            self.nodes = other.nodes;
        }
        if other.edges.is_some() {
            self.edges = other.edges;
        }
    }
}

/// 🧮️ `graph`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch (`SemioDiff`
/// delegates `between`/`inverse`/`is_empty` straight through to every wrapped subset's own impl).
/// Whole-list `between`/`inverse` are honest here (not apply-then-capture): `graph` has exactly two
/// mutable fields, so a change is fully described by "the new/old `nodes`/`edges` value", same
/// shape every mutation triad's own `🔺️diff` leaf already produces.
impl protocol::command::DiffAlgebra<SemioGraphSnapshot> for SemioGraphDiff {
    fn between(base: &SemioGraphSnapshot, other: &SemioGraphSnapshot) -> Self {
        SemioGraphDiff { nodes: (base.nodes != other.nodes).then(|| SemioGraphNodeList { values: other.nodes.clone() }), edges: (base.edges != other.edges).then(|| SemioGraphEdgeList { values: other.edges.clone() }) }
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Self {
        SemioGraphDiff { nodes: self.nodes.as_ref().map(|_| SemioGraphNodeList { values: base.nodes.clone() }), edges: self.edges.as_ref().map(|_| SemioGraphEdgeList { values: base.edges.clone() }) }
    }
    fn is_empty(&self) -> bool {
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
use crate::standards::v1::subsets::base::schema::triples::{split_top_level,strip_brackets};
use crate::standards::v1::subsets::graph::schema::snapshot::{enc_node,dec_node,enc_edge,dec_edge};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_nodes(list: &SemioGraphNodeList) -> String {
    format!("[{}]", list.values.iter().map(enc_node).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_nodes(s: &str) -> Result<SemioGraphNodeList, String> {
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_node).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioGraphNodeList { values })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_edges(list: &SemioGraphEdgeList) -> String {
    format!("[{}]", list.values.iter().map(enc_edge).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_edges(s: &str) -> Result<SemioGraphEdgeList, String> {
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioGraphEdgeList { values })
}

/// 🖇️ Joins present fields with `;` — ONE physical line, empty when neither field is present,
/// `nodes=[...]`, `edges=[...]`, or `nodes=[...];edges=[...]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_graph_diff(d: &SemioGraphDiff) -> String {
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
fn parse_graph_diff(line: &str) -> Result<SemioGraphDiff, String> {
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
    fn print_diff(&self) -> String {
        print_graph_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_graph_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    /// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`nodes`, bit1=`edges`) are two
    /// REAL fixed fields; when present, each list follows as a real varint count + per-record
    /// binary encoding (reusing the snapshot facet's own `write_node`/`read_node`/`write_edge`/
    /// `read_edge`) rather than a text-blob-in-binary shortcut.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        use crate::standards::v1::subsets::graph::schema::snapshot::{write_edge, write_node};
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
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        use crate::standards::v1::subsets::graph::schema::snapshot::{read_edge, read_node};
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
/// law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioGraphDiff> {
    use crate::standards::v1::subsets::graph::schema::snapshot::demo_graph_snapshot;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
