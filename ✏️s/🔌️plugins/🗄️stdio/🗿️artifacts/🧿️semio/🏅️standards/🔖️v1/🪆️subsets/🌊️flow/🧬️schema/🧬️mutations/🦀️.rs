//! 🧬️ SemioFlowMutation — named-variant mutation vocabulary over `SemioFlowSnapshot`.
//! Every variant's `diff()` is HANDCRAFTED (constructs the sparse `SemioFlowDiff` directly via
//! the `schema::diff` helpers — never apply-and-capture, per this ticket's explicit ban and the
//! schema-design.md svg infinite-recursion warning) and every variant's `inverse()` is
//! handcrafted, key-aware — expressed as `agg_diff`/`agg_inverse` free functions the
//! `dsl::Mutations` derive's synthesized leaves delegate into, per the stdio mutation-leaf
//! migration recipe.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;


use crate::standards::v1::subsets::flow::schema::diff::{diff_insert_edge, diff_insert_node, diff_remove_edge, diff_remove_node, diff_remove_node_param, diff_set_edge_endpoints, diff_set_edge_kind, diff_set_node_kind, diff_set_node_label, diff_set_node_param, diff_set_node_position, SemioFlowDiff};












use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, SemioFlowSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioFlowMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).


//#region 🔖️Mutations
#[path = "🌉️insert-edge/🦀️.rs"]
pub mod insert_edge;
#[path = "➕️insert-node/🦀️.rs"]
pub mod insert_node;
#[path = "✂️remove-edge/🦀️.rs"]
pub mod remove_edge;
#[path = "🗑️remove-node/🦀️.rs"]
pub mod remove_node;
#[path = "🧹️remove-node-param/🦀️.rs"]
pub mod remove_node_param;
#[path = "🔌️set-edge-endpoints/🦀️.rs"]
pub mod set_edge_endpoints;
#[path = "🎨️set-edge-kind/🦀️.rs"]
pub mod set_edge_kind;
#[path = "🏷️set-node-kind/🦀️.rs"]
pub mod set_node_kind;
#[path = "🔤️set-node-label/🦀️.rs"]
pub mod set_node_label;
#[path = "🎛️set-node-param/🦀️.rs"]
pub mod set_node_param;
#[path = "📍️set-node-position/🦀️.rs"]
pub mod set_node_position;
#[path = "✋️drag-nodes/🦀️.rs"]
pub mod drag_nodes;
/// 📐️ Typed content mutation for `s.stdio.semio.flow`. Addresses `nodes`/`edges` by `id` (both
/// id-keyed collections) and a node's own `params` by `(id, key)`.
//#region 🔖️Leaves
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none (the
/// stdio mutation-leaf migration recipe's hard constraint #1 — `no` is also not an approved
/// semantic verb). The `#[value(tag = "mutation", rename_all = "camelCase")]` container attribute
/// is KEPT here, unlike the `tiff` reference this migration was derived from (which carries none):
/// serde's internally tagged representation flattens a newtype variant's struct payload into the
/// same JSON object the tag lives in, so `decode_semio_flow_mutation_json`'s committed
/// specification vectors and the `🌊️mutate-semio-flow` test adapter's `{"mutation":"insertNode",...}`
/// payloads keep decoding byte-for-byte unchanged after this migration.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioFlowSnapshot, diff = SemioFlowDiff, schema = "SemioFlowMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioFlowMutation {
    /// ➕️ Inserts `node` (whole payload, already carries its own `id`).
    InsertNode(insert_node::InsertNode),
    /// ➖️ Removes the node with id `id` (and — at the snapshot level, via a real referential
    /// invariant the `SubsetValidator` checks — any edge that still references it).
    RemoveNode(remove_node::RemoveNode),
    /// 🏷️ Sets node `id`'s `kind`.
    SetNodeKind(set_node_kind::SetNodeKind),
    /// 🏷️ Sets node `id`'s `label`.
    SetNodeLabel(set_node_label::SetNodeLabel),
    /// 📍️ Sets node `id`'s `position`.
    SetNodePosition(set_node_position::SetNodePosition),
    /// 🎛️ Upserts one param on node `id` (adds if `key` is new, sets if it already exists).
    SetNodeParam(set_node_param::SetNodeParam),
    /// ➖️ Removes param `key` from node `id`.
    RemoveNodeParam(remove_node_param::RemoveNodeParam),
    /// ➕️ Inserts `edge` (whole payload, already carries its own `id`).
    InsertEdge(insert_edge::InsertEdge),
    /// ➖️ Removes the edge with id `id`.
    RemoveEdge(remove_edge::RemoveEdge),
    /// 🔌️ Sets edge `id`'s `from`/`to` endpoints.
    SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints),
    /// 🏷️ Sets edge `id`'s `kind`.
    SetEdgeKind(set_edge_kind::SetEdgeKind),
    /// ✋️ Drags the `targets` nodes by one relative `(dx, dy)` offset.
    DragNodes(drag_nodes::DragNodes),
}

/// 🏷️ The declared mutation vocabulary of `s.stdio.semio.flow`, in `SemioFlowMutation`'s own
/// declaration order and kebab-case spelling — the single source of truth for the binary op frame's
/// `tag` ordinal (see [`wire_tag`]), for `parse_flow_mutation`'s keyword match, and for the
/// `semio-v1-flow` catalog in `../../🔣️oracle.json`. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps all three honest.
pub const KINDS: &[&str] = &["insert-node", "remove-node", "set-node-kind", "set-node-label", "set-node-position", "set-node-param", "remove-node-param", "insert-edge", "remove-edge", "set-edge-endpoints", "set-edge-kind", "drag-nodes"];
//#endregion 🔖️Mutations

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_flow_mutation(mutation: &SemioFlowMutation, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    <SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::diff(mutation, base)
}


/// ↩️ Computes `mutation`'s own inverse against `base` — a thin wrapper around
/// `protocol::Mutation::inverse` so external Rust callers that cannot name this crate's private
/// `protocol` extern-crate item (the `🌊️mutate-semio-flow` test adapter, whose `inverse-<kind>`
/// scenarios need a mutation's own computed inverse) can still reach the inverse law that
/// `diff_semio_*_mutation` alone cannot. Same shape as `🧰️kit`'s
/// `inverse_semio_kit_mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_flow_mutation(mutation: &SemioFlowMutation, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    Ok({
    Mutation::inverse(mutation, base)?

    })
}


//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_at<'a>(base: &'a SemioFlowSnapshot, id: &str) -> Option<&'a FlowNode> {
    base.nodes.iter().find(|n| n.id == id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_at<'a>(base: &'a SemioFlowSnapshot, id: &str) -> Option<&'a FlowEdge> {
    base.edges.iter().find(|e| e.id == id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn param_value_at<'a>(base: &'a SemioFlowSnapshot, id: &str, key: &str) -> Option<&'a str> {
    node_at(base, id)?.params.iter().find(|p| p.key == key).map(|p| p.value.as_str())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn param_index_at(base: &SemioFlowSnapshot, id: &str, key: &str) -> Option<usize> {
    node_at(base, id)?.params.iter().position(|p| p.key == key)
}
//#endregion 🔖️Helpers




//#endregion 🔖️MutationTrait

//#region OpCodecs













/// ⚡️ P2 pilot: real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary
/// shortcut. `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`KINDS`]) are two REAL fixed fields; the variant's own `key=value ...` argument payload
/// follows as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_flow_mutation`/`parse_flow_mutation` text codec rather than re-deriving a second
/// independent encoding.



//#endregion OpCodecs

//#region 🔖️Demo



#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fixture() -> SemioFlowSnapshot {
    SemioFlowSnapshot {
        schema: crate::standards::v1::subsets::flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(),
        nodes: vec![node("n1", "source", "Source", 0.0, 0.0), node("n2", "sink", "Sink", 10.0, 10.0)],
        edges: vec![edge("e1", "n1", "n2", "data")],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioFlowMutation> {
    vec![
        SemioFlowMutation::InsertNode(insert_node::InsertNode { node: node("n3", "transform", "T", 5.0, 5.0), at: None }),
        SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: "n2".into() }),
        SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id: "n1".into(), kind: "changed".into() }),
        SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id: "n1".into(), label: "Changed".into() }),
        SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id: "n1".into(), position: SemioPoint2 { x: 99.0, y: -1.0 } }),
        SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "k".into(), value: "new".into(), at: None }),
        SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "fresh".into(), value: "added".into(), at: Some(0) }),
        SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id: "n1".into(), key: "k".into() }),
        SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge: edge("e2", "n2", "n1", "back"), at: None }),
        SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: "e1".into() }),
        SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: "e1".into(), from: PortRef { node: "n2".into(), port: "out".into() }, to: PortRef { node: "n1".into(), port: "in".into() } }),
        SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id: "e1".into(), kind: "changed".into() }),
        SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["n1".into(), "n2".into()], dx: 12.5, dy: -4.0 }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

/// 🌱 Shared fixture helpers + representative `SemioFlowMutation` cases (one per variant) —
/// single source of truth for this facet's own tests AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn node(id: &str, kind: &str, label: &str, x: f64, y: f64) -> FlowNode {
    FlowNode { id: id.into(), kind: kind.into(), label: label.into(), params: vec![crate::standards::v1::subsets::flow::schema::snapshot::FlowParam { key: "k".into(), value: "v".into() }], position: SemioPoint2 { x, y } }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn edge(id: &str, from_node: &str, to_node: &str, kind: &str) -> FlowEdge {
    FlowEdge { id: id.into(), from: PortRef { node: from_node.into(), port: "out".into() }, to: PortRef { node: to_node.into(), port: "in".into() }, kind: kind.into() }
}

#[cfg(test)]
use protocol::{OpBinary,OpText};
