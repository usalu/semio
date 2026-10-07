//! 📝️ Text representation codec surface for `s.stdio.semio.flow` (mutations) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::flow::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::flow::schema::diff::{diff_insert_edge, diff_insert_node, diff_remove_edge, diff_remove_node, diff_remove_node_param, diff_set_edge_endpoints, diff_set_edge_kind, diff_set_node_kind, diff_set_node_label, diff_set_node_param, diff_set_node_position, diff_set_snapshot, SemioFlowDiff};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_edge};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_edge};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_node};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_node};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_port_ref};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_port_ref};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_point2};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_point2};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_f64};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_f64};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, SemioFlowSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioFlowMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).
use protocol::{OpBinary, OpText};

/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", ...}`) JSON
/// projection — the shape `🌊️mutate-semio-flow`'s committed specification vectors carry in their
/// `mutation` member — into a real [`SemioFlowMutation`]. A thin `pack::from_json_str` wrapper (over
/// `ToValue`/`FromValue`, first-party, per this ticket's serde→value conversion), so the test adapter reads
/// the committed vector instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_flow_mutation_json(text: &str) -> Result<SemioFlowMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🧪️ Hand-rolled `OpText`/`OpBinary` (per this ticket: no `#[derive(dsl::DslOps)]` fight —
/// `FlowNode`/`FlowEdge`/`SemioFlowSnapshot` are not `#[derive(dsl::DslRecord)]`, same
/// family of gap `DocxMutation`'s doc comment documents for its own `DocxBlock`/`DocxSnapshot`
/// payloads). Grammar: `keyword arg=value ...` (space-separated), reusing `schema::diff`'s
/// `pub(crate)` grammar primitives. `no-mutation` is no longer a keyword this codec parses (there
/// is nothing left to construct for it); a `🧪️tests/mutate-*` adapter that must still honor the
/// `no-mutation` scenario id maps it to the identity `set-snapshot` mutation itself, ahead of this
/// codec.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_flow_snapshot(s: &SemioFlowSnapshot) -> String {
    format!("[{},{},{}]", enc_str(&s.schema), format_args!("[{}]", s.nodes.iter().map(enc_node).collect::<Vec<_>>().join(",")), format_args!("[{}]", s.edges.iter().map(enc_edge).collect::<Vec<_>>().join(",")))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_flow_snapshot(s: &str) -> Result<SemioFlowSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, nodes, edges] = parts.as_slice() else { return Err(format!("snapshot: expected 3 fields, got {}", parts.len())) };
    let nodes = split_top_level(strip_brackets(nodes)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_node).collect::<Result<Vec<_>, String>>()?;
    let edges = split_top_level(strip_brackets(edges)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioFlowSnapshot { schema: dec_str(schema)?, nodes, edges })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_flow_mutation(m: &SemioFlowMutation) -> String {
    match m {
        SemioFlowMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_semio_flow_snapshot(snapshot)),
        SemioFlowMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        SemioFlowMutation::InsertNode(insert_node::InsertNode { node }) => format!("insert-node node={}", enc_node(node)),
        SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id }) => format!("remove-node id={}", enc_str(id)),
        SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id, kind }) => format!("set-node-kind id={} kind={}", enc_str(id), enc_str(kind)),
        SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id, label }) => format!("set-node-label id={} label={}", enc_str(id), enc_str(label)),
        SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id, position }) => format!("set-node-position id={} position={}", enc_str(id), enc_point2(position)),
        SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id, key, value }) => format!("set-node-param id={} key={} value={}", enc_str(id), enc_str(key), enc_str(value)),
        SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id, key }) => format!("remove-node-param id={} key={}", enc_str(id), enc_str(key)),
        SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge }) => format!("insert-edge edge={}", enc_edge(edge)),
        SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id }) => format!("remove-edge id={}", enc_str(id)),
        SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id, from, to }) => format!("set-edge-endpoints id={} from={} to={}", enc_str(id), enc_port_ref(from), enc_port_ref(to)),
        SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id, kind }) => format!("set-edge-kind id={} kind={}", enc_str(id), enc_str(kind)),
        SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets, dx, dy }) => format!("drag-nodes targets=[{}] dx={} dy={}", targets.iter().map(|id| enc_str(id)).collect::<Vec<_>>().join(","), enc_f64(*dx), enc_f64(*dy)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_flow_mutation(line: &str) -> Result<SemioFlowMutation, String> {
    if let Some(source) = line.strip_prefix("patch-snapshot patch=") {
        let patch = semio_s_artifact_stdio_contract::editing::snapshot_patch_from_hex(source)?;
        return Ok(SemioFlowMutation::PatchSnapshot(crate::standards::v1::subsets::flow::schema::mutations::patch_snapshot::PatchSnapshot { patch }));
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("flow mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("flow mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioFlowMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_semio_flow_snapshot(arg("snapshot")?)? })),
        "insert-node" => Ok(SemioFlowMutation::InsertNode(insert_node::InsertNode { node: dec_node(arg("node")?)? })),
        "remove-node" => Ok(SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: dec_str(arg("id")?)? })),
        "set-node-kind" => Ok(SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id: dec_str(arg("id")?)?, kind: dec_str(arg("kind")?)? })),
        "set-node-label" => Ok(SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id: dec_str(arg("id")?)?, label: dec_str(arg("label")?)? })),
        "set-node-position" => Ok(SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id: dec_str(arg("id")?)?, position: dec_point2(arg("position")?)? })),
        "set-node-param" => Ok(SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: dec_str(arg("id")?)?, key: dec_str(arg("key")?)?, value: dec_str(arg("value")?)? })),
        "remove-node-param" => Ok(SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id: dec_str(arg("id")?)?, key: dec_str(arg("key")?)? })),
        "insert-edge" => Ok(SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge: dec_edge(arg("edge")?)? })),
        "remove-edge" => Ok(SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: dec_str(arg("id")?)? })),
        "set-edge-endpoints" => Ok(SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: dec_str(arg("id")?)?, from: dec_port_ref(arg("from")?)?, to: dec_port_ref(arg("to")?)? })),
        "set-edge-kind" => Ok(SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id: dec_str(arg("id")?)?, kind: dec_str(arg("kind")?)? })),
        "drag-nodes" => Ok(SemioFlowMutation::DragNodes(drag_nodes::DragNodes {
            targets: split_top_level(strip_brackets(arg("targets")?)?, ',').into_iter().filter(|id| !id.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?,
            dx: dec_f64(arg("dx")?)?,
            dy: dec_f64(arg("dy")?)?,
        })),
        other => Err(format!("flow mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioFlowMutation {
    fn print_op(&self) -> String {
        print_flow_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_flow_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}




}
pub use mutations_codec::*;
