//! 🧬️ SemioFlowMutation — named-variant mutation vocabulary over `SemioFlowSnapshot`.
//! Every variant's `diff()` is HANDCRAFTED (constructs the sparse `SemioFlowDiff` directly via
//! the `schema::diff` helpers — never apply-and-capture, per this ticket's explicit ban and the
//! schema-design.md svg infinite-recursion warning) and every variant's `inverse()` is
//! handcrafted, key-aware — expressed as `agg_diff`/`agg_inverse` free functions the
//! `dsl::Mutations` derive's synthesized leaves delegate into, per the stdio mutation-leaf
//! migration recipe.

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::diff::{
    dec_edge, dec_node, dec_point2, dec_port_ref, dec_str, diff_insert_edge, diff_insert_node, diff_remove_edge, diff_remove_node, diff_remove_node_param, diff_set_edge_endpoints, diff_set_edge_kind, diff_set_node_kind, diff_set_node_label,
    diff_set_node_param, diff_set_node_position, diff_set_snapshot, enc_edge, enc_node, enc_point2, enc_port_ref, enc_str, SemioFlowDiff,
};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, SemioFlowSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioFlowMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).
use protocol::{OpBinary, OpText};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `s.stdio.semio.flow`. Addresses `nodes`/`edges` by `id` (both
/// id-keyed collections) and a node's own `params` by `(id, key)`.
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🟢insert-node/🦀️.rs"]
pub mod insert_node;
#[path = "🔴remove-node/🦀️.rs"]
pub mod remove_node;
#[path = "🏷set-node-kind/🦀️.rs"]
pub mod set_node_kind;
#[path = "🔤set-node-label/🦀️.rs"]
pub mod set_node_label;
#[path = "📍set-node-position/🦀️.rs"]
pub mod set_node_position;
#[path = "🎛set-node-param/🦀️.rs"]
pub mod set_node_param;
#[path = "🎚remove-node-param/🦀️.rs"]
pub mod remove_node_param;
#[path = "🔗insert-edge/🦀️.rs"]
pub mod insert_edge;
#[path = "✂remove-edge/🦀️.rs"]
pub mod remove_edge;
#[path = "🔌set-edge-endpoints/🦀️.rs"]
pub mod set_edge_endpoints;
#[path = "🎨set-edge-kind/🦀️.rs"]
pub mod set_edge_kind;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none (the
/// stdio mutation-leaf migration recipe's hard constraint #1 — `no` is also not an approved
/// semantic verb). The `#[value(tag = "mutation", rename_all = "camelCase")]` container attribute
/// is KEPT here, unlike the `tiff` reference this migration was derived from (which carries none):
/// serde's internally tagged representation flattens a newtype variant's struct payload into the
/// same JSON object the tag lives in, so `decode_semio_flow_mutation_json`'s committed
/// specification vectors and the `mutate-semio-flow` test adapter's `{"mutation":"insertNode",...}`
/// payloads keep decoding byte-for-byte unchanged after this migration.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioFlowSnapshot, diff = SemioFlowDiff, schema = "SemioFlowMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioFlowMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
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
}

/// 🏷️ The declared mutation vocabulary of `s.stdio.semio.flow`, in `SemioFlowMutation`'s own
/// declaration order and kebab-case spelling — the single source of truth for the binary op frame's
/// `tag` ordinal (see [`variant_ordinal`]), for `parse_flow_mutation`'s keyword match, and for the
/// `semio-v1-flow` catalog in `../../🔣️oracle.json`. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps all three honest.
pub const KINDS: &[&str] = &["set-snapshot", "insert-node", "remove-node", "set-node-kind", "set-node-label", "set-node-position", "set-node-param", "remove-node-param", "insert-edge", "remove-edge", "set-edge-endpoints", "set-edge-kind"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source (mirrors docx/gif convention).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_flow_mutation(snapshot: &mut SemioFlowSnapshot, mutation: &SemioFlowMutation) -> protocol::MutationOutcome<SemioFlowDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Computes `mutation`'s own inverse against `base` — a thin wrapper around
/// `protocol::Mutation::inverse` so external Rust callers that cannot name this crate's private
/// `protocol` extern-crate item (the `mutate-semio-flow` test adapter, whose `inverse-<kind>`
/// scenarios need a mutation's own computed inverse) can still reach the inverse law that
/// [`apply_semio_flow_mutation`] alone cannot. Same shape as `✳️kit`'s
/// `inverse_semio_kit_mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_flow_mutation(mutation: &SemioFlowMutation, base: &SemioFlowSnapshot) -> Vec<SemioFlowMutation> {
    Mutation::inverse(mutation, base)
}

/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", ...}`) JSON
/// projection — the shape `mutate-semio-flow`'s committed specification vectors carry in their
/// `mutation` member — into a real [`SemioFlowMutation`]. A thin `pack::from_json_str` wrapper (over
/// `ToValue`/`FromValue`, first-party, per this ticket's serde→value conversion), so the test adapter reads
/// the committed vector instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_flow_mutation_json(text: &str) -> Result<SemioFlowMutation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
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
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioFlowMutation, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    protocol::MutationOutcome::new(match this {
        SemioFlowMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        SemioFlowMutation::InsertNode(insert_node::InsertNode { node }) => diff_insert_node(node.clone()),
        SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id }) => diff_remove_node(id),
        SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id, kind }) => diff_set_node_kind(id, kind),
        SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id, label }) => diff_set_node_label(id, label),
        SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id, position }) => diff_set_node_position(id, *position),
        SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id, key, value }) => diff_set_node_param(base, id, key, value),
        SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id, key }) => diff_remove_node_param(id, key),
        SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge }) => diff_insert_edge(edge.clone()),
        SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id }) => diff_remove_edge(id),
        SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id, from, to }) => diff_set_edge_endpoints(id, from.clone(), to.clone()),
        SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id, kind }) => diff_set_edge_kind(id, kind),
    })
}

/// ↩️ Lifted verbatim from the former `impl Mutation`, except every `None`-target fallback that
/// used to construct `NoMutation` now returns `Vec::new()` (an inverse with nothing to restore) —
/// the convention this migration's fleet coordinator ruled on, since `NoMutation` is no longer a
/// constructible variant. `SetNodeParam`'s and `RemoveNodeParam`'s "param was absent" fallbacks
/// already inverted into a real opposite mutation (`RemoveNodeParam`/`SetNodeParam`) rather than a
/// no-op, so only their OWN "node absent" arm changes shape here.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioFlowMutation, base: &SemioFlowSnapshot) -> Vec<SemioFlowMutation> {
    match this {
        SemioFlowMutation::SetSnapshot(_) => vec![SemioFlowMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        SemioFlowMutation::InsertNode(insert_node::InsertNode { node }) => vec![SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: node.id.clone() })],
        SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id }) => match node_at(base, id) {
            Some(node) => vec![SemioFlowMutation::InsertNode(insert_node::InsertNode { node: node.clone() })],
            None => Vec::new(),
        },
        SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id, .. }) => match node_at(base, id) {
            Some(node) => vec![SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id: id.clone(), kind: node.kind.clone() })],
            None => Vec::new(),
        },
        SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id, .. }) => match node_at(base, id) {
            Some(node) => vec![SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id: id.clone(), label: node.label.clone() })],
            None => Vec::new(),
        },
        SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id, .. }) => match node_at(base, id) {
            Some(node) => vec![SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id: id.clone(), position: node.position })],
            None => Vec::new(),
        },
        SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id, key, .. }) => match param_value_at(base, id, key) {
            Some(value) => vec![SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: id.clone(), key: key.clone(), value: value.to_string() })],
            None => vec![SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id: id.clone(), key: key.clone() })],
        },
        SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id, key }) => match param_value_at(base, id, key) {
            Some(value) => vec![SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: id.clone(), key: key.clone(), value: value.to_string() })],
            None => Vec::new(),
        },
        SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge }) => vec![SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: edge.id.clone() })],
        SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id }) => match edge_at(base, id) {
            Some(edge) => vec![SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge: edge.clone() })],
            None => Vec::new(),
        },
        SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id, .. }) => match edge_at(base, id) {
            Some(edge) => vec![SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: id.clone(), from: edge.from.clone(), to: edge.to.clone() })],
            None => Vec::new(),
        },
        SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id, .. }) => match edge_at(base, id) {
            Some(edge) => vec![SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id: id.clone(), kind: edge.kind.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ Hand-rolled `OpText`/`OpBinary` (per this ticket: no `#[derive(dsl::DslOps)]` fight —
/// `FlowNode`/`FlowEdge`/`SemioFlowSnapshot` are not `#[derive(dsl::DslRecord)]`, same
/// family of gap `DocxMutation`'s doc comment documents for its own `DocxBlock`/`DocxSnapshot`
/// payloads). Grammar: `keyword arg=value ...` (space-separated), reusing `schema::diff`'s
/// `pub(crate)` grammar primitives. `no-mutation` is no longer a keyword this codec parses (there
/// is nothing left to construct for it); a `🧪️tests/mutate-*` adapter that must still honor the
/// `no-mutation` scenario id maps it to the identity `set-snapshot` mutation itself, ahead of this
/// codec.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_semio_flow_snapshot(s: &SemioFlowSnapshot) -> String {
    format!("[{},{},{}]", enc_str(&s.schema), format!("[{}]", s.nodes.iter().map(enc_node).collect::<Vec<_>>().join(",")), format!("[{}]", s.edges.iter().map(enc_edge).collect::<Vec<_>>().join(",")))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_semio_flow_snapshot(s: &str) -> Result<SemioFlowSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, nodes, edges] = parts.as_slice() else { return Err(format!("snapshot: expected 3 fields, got {}", parts.len())) };
    let nodes = split_top_level(strip_brackets(nodes)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_node).collect::<Result<Vec<_>, String>>()?;
    let edges = split_top_level(strip_brackets(edges)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioFlowSnapshot { schema: dec_str(schema)?, nodes, edges })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_flow_mutation(m: &SemioFlowMutation) -> String {
    match m {
        SemioFlowMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_semio_flow_snapshot(snapshot)),
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
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_flow_mutation(line: &str) -> Result<SemioFlowMutation, String> {
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
        other => Err(format!("flow mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioFlowMutation {
    fn print_op(&self) -> String {
        print_flow_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_flow_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioFlowMutation) -> u8 {
    match m {
        SemioFlowMutation::SetSnapshot(_) => 0,
        SemioFlowMutation::InsertNode(_) => 1,
        SemioFlowMutation::RemoveNode(_) => 2,
        SemioFlowMutation::SetNodeKind(_) => 3,
        SemioFlowMutation::SetNodeLabel(_) => 4,
        SemioFlowMutation::SetNodePosition(_) => 5,
        SemioFlowMutation::SetNodeParam(_) => 6,
        SemioFlowMutation::RemoveNodeParam(_) => 7,
        SemioFlowMutation::InsertEdge(_) => 8,
        SemioFlowMutation::RemoveEdge(_) => 9,
        SemioFlowMutation::SetEdgeEndpoints(_) => 10,
        SemioFlowMutation::SetEdgeKind(_) => 11,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_flow_mutation` — the binary frame's `tag`
/// byte already carries the keyword, so the text keyword itself is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_flow_mutation_args(m: &SemioFlowMutation) -> String {
    match print_flow_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ P2 pilot: real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary
/// shortcut. `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`KINDS`]) are two REAL fixed fields; the variant's own `key=value ...` argument payload
/// follows as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_flow_mutation`/`parse_flow_mutation` text codec rather than re-deriving a second
/// independent encoding.
impl OpBinary for SemioFlowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_flow_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = KINDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", KINDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Shared fixture helpers + representative `SemioFlowMutation` cases (one per variant) —
/// single source of truth for this facet's own tests AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node(id: &str, kind: &str, label: &str, x: f64, y: f64) -> FlowNode {
    FlowNode { id: id.into(), kind: kind.into(), label: label.into(), params: vec![crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::FlowParam { key: "k".into(), value: "v".into() }], position: SemioPoint2 { x, y } }
}
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge(id: &str, from_node: &str, to_node: &str, kind: &str) -> FlowEdge {
    FlowEdge { id: id.into(), from: PortRef { node: from_node.into(), port: "out".into() }, to: PortRef { node: to_node.into(), port: "in".into() }, kind: kind.into() }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fixture() -> SemioFlowSnapshot {
    SemioFlowSnapshot {
        schema: crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(),
        nodes: vec![node("n1", "source", "Source", 0.0, 0.0), node("n2", "sink", "Sink", 10.0, 10.0)],
        edges: vec![edge("e1", "n1", "n2", "data")],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioFlowMutation> {
    vec![
        SemioFlowMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: fixture() }),
        SemioFlowMutation::InsertNode(insert_node::InsertNode { node: node("n3", "transform", "T", 5.0, 5.0) }),
        SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: "n2".into() }),
        SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id: "n1".into(), kind: "changed".into() }),
        SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id: "n1".into(), label: "Changed".into() }),
        SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id: "n1".into(), position: SemioPoint2 { x: 99.0, y: -1.0 } }),
        SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "k".into(), value: "new".into() }),
        SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "fresh".into(), value: "added".into() }),
        SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id: "n1".into(), key: "k".into() }),
        SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge: edge("e2", "n2", "n1", "back") }),
        SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: "e1".into() }),
        SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: "e1".into(), from: PortRef { node: "n2".into(), port: "out".into() }, to: PortRef { node: "n1".into(), port: "in".into() } }),
        SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id: "e1".into(), kind: "changed".into() }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;

    //#region 🔖️MutationDiffLaw
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        for mutation in demo_mutation_cases() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = protocol::MutationDiff::apply(diff_direct.diff(), &base).expect("apply must succeed for a well-formed fixture");

            let mut via_apply = base.clone();
            let diff_from_apply = apply_semio_flow_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        for mutation in demo_mutation_cases() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_semio_flow_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::inverse(&mutation, &base) {
                apply_semio_flow_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = protocol::MutationDiff::apply(diff.diff(), &base).expect("apply must succeed for a well-formed fixture");
            let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
            let restored = protocol::MutationDiff::apply(&inverse_diff, &next).expect("apply must succeed for a well-formed fixture");
            assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️OpTextBinaryRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioFlowMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = SemioFlowMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw

    //#region 🔖️KindsCatalog
    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling the binary op
    /// frame's `tag` ordinal and the text grammar's keyword both use, and every one of those
    /// spellings must also appear in the committed oracle manifest's catalog. The framework never
    /// parses Rust, so this is what makes the declaration honest.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        assert_eq!(KINDS.len(), 12, "KINDS must name exactly one entry per declared SemioFlowMutation variant");
        let mut seen = vec![false; KINDS.len()];
        for mutation in demo_mutation_cases() {
            let keyword = print_flow_mutation(&mutation).split(' ').next().expect("printed op is never empty").to_string();
            let ordinal = variant_ordinal(&mutation) as usize;
            assert_eq!(KINDS[ordinal], keyword, "KINDS must match the declaration order and spelling for {mutation:?}");
            seen[ordinal] = true;
        }
        assert!(seen.iter().all(|hit| *hit), "demo_mutation_cases must reach every KINDS entry, missing {:?}", KINDS.iter().zip(seen.iter()).filter(|(_, hit)| !**hit).map(|(kind, _)| *kind).collect::<Vec<_>>());
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🔖️KindsCatalog

    //#region 🔖️VariantBehavior
    #[semio_framework_async_macros::async_test]
    async fn insert_then_remove_node_apply_and_inverse() {
        let base = fixture();
        let insert = SemioFlowMutation::InsertNode(insert_node::InsertNode { node: node("n3", "transform", "T", 5.0, 5.0) });
        let mut after = base.clone();
        apply_semio_flow_mutation(&mut after, &insert);
        assert_eq!(after.nodes.len(), 3);
        for inv in Mutation::inverse(&insert, &base) {
            apply_semio_flow_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn node_param_mutations_apply_and_inverse() {
        let base = fixture();
        let set = SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "k".into(), value: "new".into() });
        let mut after = base.clone();
        apply_semio_flow_mutation(&mut after, &set);
        assert_eq!(param_value_at(&after, "n1", "k"), Some("new"));
        for inv in Mutation::inverse(&set, &base) {
            apply_semio_flow_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let add = SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "fresh".into(), value: "added".into() });
        let mut after2 = base.clone();
        apply_semio_flow_mutation(&mut after2, &add);
        assert_eq!(param_value_at(&after2, "n1", "fresh"), Some("added"));
        for inv in Mutation::inverse(&add, &base) {
            apply_semio_flow_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn edge_mutations_apply_and_inverse() {
        let base = fixture();
        let set = SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: "e1".into(), from: PortRef { node: "n2".into(), port: "out".into() }, to: PortRef { node: "n1".into(), port: "in".into() } });
        let mut after = base.clone();
        apply_semio_flow_mutation(&mut after, &set);
        assert_eq!(edge_at(&after, "e1").unwrap().from.node, "n2");
        for inv in Mutation::inverse(&set, &base) {
            apply_semio_flow_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }
    //#endregion 🔖️VariantBehavior
}
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/relabels-and-repositions-the-transform-node/🦀️.rs"]
mod set_snapshot_relabels_and_repositions_the_transform_node;
//#endregion 🧪️FixtureCases
