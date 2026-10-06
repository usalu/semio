//! 📝️ Text representation codec surface for `stdio.semio.graph` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::graph::schema::diff::*;
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
/// 🧪️ Hand-rolled `protocol::DiffCodec` — `graph`'s two collection fields print as
/// `nodes=[<node>,...]`/`edges=[<edge>,...]` joined by `;` (empty string = no-op diff), reusing the
/// snapshot facet's own real hex/bracket node/edge encoders (duplicated locally, same convention
/// every sibling subset's `🔺️diff` facet already establishes — see that facet's own doc comment
/// for why).
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::flow::io::text::diff::{dec_node};
use crate::flow::io::text::diff::{enc_node};
use crate::flow::io::text::diff::{dec_edge};
use crate::flow::io::text::diff::{enc_edge};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_nodes(list: &SemioGraphNodeList) -> String {
    format!("[{}]", list.values.iter().map(enc_node).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_nodes(s: &str) -> Result<SemioGraphNodeList, String> {
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_node).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioGraphNodeList { values })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edges(list: &SemioGraphEdgeList) -> String {
    format!("[{}]", list.values.iter().map(enc_edge).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edges(s: &str) -> Result<SemioGraphEdgeList, String> {
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioGraphEdgeList { values })
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
