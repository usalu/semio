//! 🧬️ SemioWorkflowMutation — named-variant mutation vocabulary over `SemioWorkflowSnapshot`.
//! Every variant's `diff()` is HANDCRAFTED (constructs the sparse `SemioWorkflowDiff` directly via
//! the `schema::diff` helpers — never apply-and-capture, per this ticket's explicit ban and the
//! schema-design.md svg infinite-recursion warning) and every variant's `inverse()` is
//! handcrafted, key-aware.

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::diff::{
    dec_edge, dec_node, dec_point2, dec_port_ref, dec_str, diff_insert_edge, diff_insert_node, diff_remove_edge,
    diff_remove_node, diff_remove_node_param, diff_set_edge_endpoints, diff_set_edge_kind, diff_set_node_kind, diff_set_node_label,
    diff_set_node_param, diff_set_node_position, diff_set_snapshot, enc_edge, enc_node, enc_point2, enc_port_ref, enc_str,
    SemioWorkflowDiff,
};
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::{PortRef, SemioWorkflowSnapshot, WorkflowEdge, WorkflowNode};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioWorkflowMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `s.stdio.semio.workflow`. Beyond the baseline
/// `{NoMutation, SetSnapshot}`, addresses `nodes`/`edges` by `id` (both id-keyed collections) and
/// a node's own `params` by `(id, key)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioWorkflowMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SemioWorkflowSnapshot,
    },
    /// ➕️ Inserts `node` (whole payload, already carries its own `id`).
    InsertNode {
        node: WorkflowNode,
    },
    /// ➖️ Removes the node with id `id` (and — at the snapshot level, via a real referential
    /// invariant the `SubsetValidator` checks — any edge that still references it).
    RemoveNode {
        id: String,
    },
    /// 🏷️ Sets node `id`'s `kind`.
    SetNodeKind {
        id: String,
        kind: String,
    },
    /// 🏷️ Sets node `id`'s `label`.
    SetNodeLabel {
        id: String,
        label: String,
    },
    /// 📍️ Sets node `id`'s `position`.
    SetNodePosition {
        id: String,
        position: SemioPoint2,
    },
    /// 🎛️ Upserts one param on node `id` (adds if `key` is new, sets if it already exists).
    SetNodeParam {
        id: String,
        key: String,
        value: String,
    },
    /// ➖️ Removes param `key` from node `id`.
    RemoveNodeParam {
        id: String,
        key: String,
    },
    /// ➕️ Inserts `edge` (whole payload, already carries its own `id`).
    InsertEdge {
        edge: WorkflowEdge,
    },
    /// ➖️ Removes the edge with id `id`.
    RemoveEdge {
        id: String,
    },
    /// 🔌️ Sets edge `id`'s `from`/`to` endpoints.
    SetEdgeEndpoints {
        id: String,
        from: PortRef,
        to: PortRef,
    },
    /// 🏷️ Sets edge `id`'s `kind`.
    SetEdgeKind {
        id: String,
        kind: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source (mirrors docx/gif convention).
pub fn apply_semio_workflow_mutation(snapshot: &mut SemioWorkflowSnapshot, mutation: &SemioWorkflowMutation) -> SemioWorkflowDiff {
    let diff = Mutation::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
fn node_at<'a>(base: &'a SemioWorkflowSnapshot, id: &str) -> Option<&'a WorkflowNode> {
    base.nodes.iter().find(|n| n.id == id)
}
fn edge_at<'a>(base: &'a SemioWorkflowSnapshot, id: &str) -> Option<&'a WorkflowEdge> {
    base.edges.iter().find(|e| e.id == id)
}
fn param_value_at<'a>(base: &'a SemioWorkflowSnapshot, id: &str, key: &str) -> Option<&'a str> {
    node_at(base, id)?.params.iter().find(|p| p.key == key).map(|p| p.value.as_str())
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
impl Mutation<SemioWorkflowSnapshot> for SemioWorkflowMutation {
    type Diff = SemioWorkflowDiff;

    fn diff(&self, base: &SemioWorkflowSnapshot) -> Self::Diff {
        match self {
            SemioWorkflowMutation::NoMutation => SemioWorkflowDiff::default(),
            SemioWorkflowMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SemioWorkflowMutation::InsertNode { node } => diff_insert_node(node.clone()),
            SemioWorkflowMutation::RemoveNode { id } => diff_remove_node(id),
            SemioWorkflowMutation::SetNodeKind { id, kind } => diff_set_node_kind(id, kind),
            SemioWorkflowMutation::SetNodeLabel { id, label } => diff_set_node_label(id, label),
            SemioWorkflowMutation::SetNodePosition { id, position } => diff_set_node_position(id, *position),
            SemioWorkflowMutation::SetNodeParam { id, key, value } => diff_set_node_param(base, id, key, value),
            SemioWorkflowMutation::RemoveNodeParam { id, key } => diff_remove_node_param(id, key),
            SemioWorkflowMutation::InsertEdge { edge } => diff_insert_edge(edge.clone()),
            SemioWorkflowMutation::RemoveEdge { id } => diff_remove_edge(id),
            SemioWorkflowMutation::SetEdgeEndpoints { id, from, to } => diff_set_edge_endpoints(id, from.clone(), to.clone()),
            SemioWorkflowMutation::SetEdgeKind { id, kind } => diff_set_edge_kind(id, kind),
        }
    }

    fn inverse(&self, base: &SemioWorkflowSnapshot) -> Vec<Self> {
        match self {
            SemioWorkflowMutation::NoMutation => vec![SemioWorkflowMutation::NoMutation],
            SemioWorkflowMutation::SetSnapshot { .. } => vec![SemioWorkflowMutation::SetSnapshot { snapshot: base.clone() }],
            SemioWorkflowMutation::InsertNode { node } => vec![SemioWorkflowMutation::RemoveNode { id: node.id.clone() }],
            SemioWorkflowMutation::RemoveNode { id } => match node_at(base, id) {
                Some(node) => vec![SemioWorkflowMutation::InsertNode { node: node.clone() }],
                None => vec![SemioWorkflowMutation::NoMutation],
            },
            SemioWorkflowMutation::SetNodeKind { id, .. } => match node_at(base, id) {
                Some(node) => vec![SemioWorkflowMutation::SetNodeKind { id: id.clone(), kind: node.kind.clone() }],
                None => vec![SemioWorkflowMutation::NoMutation],
            },
            SemioWorkflowMutation::SetNodeLabel { id, .. } => match node_at(base, id) {
                Some(node) => vec![SemioWorkflowMutation::SetNodeLabel { id: id.clone(), label: node.label.clone() }],
                None => vec![SemioWorkflowMutation::NoMutation],
            },
            SemioWorkflowMutation::SetNodePosition { id, .. } => match node_at(base, id) {
                Some(node) => vec![SemioWorkflowMutation::SetNodePosition { id: id.clone(), position: node.position }],
                None => vec![SemioWorkflowMutation::NoMutation],
            },
            SemioWorkflowMutation::SetNodeParam { id, key, .. } => match param_value_at(base, id, key) {
                Some(value) => vec![SemioWorkflowMutation::SetNodeParam { id: id.clone(), key: key.clone(), value: value.to_string() }],
                None => vec![SemioWorkflowMutation::RemoveNodeParam { id: id.clone(), key: key.clone() }],
            },
            SemioWorkflowMutation::RemoveNodeParam { id, key } => match param_value_at(base, id, key) {
                Some(value) => vec![SemioWorkflowMutation::SetNodeParam { id: id.clone(), key: key.clone(), value: value.to_string() }],
                None => vec![SemioWorkflowMutation::NoMutation],
            },
            SemioWorkflowMutation::InsertEdge { edge } => vec![SemioWorkflowMutation::RemoveEdge { id: edge.id.clone() }],
            SemioWorkflowMutation::RemoveEdge { id } => match edge_at(base, id) {
                Some(edge) => vec![SemioWorkflowMutation::InsertEdge { edge: edge.clone() }],
                None => vec![SemioWorkflowMutation::NoMutation],
            },
            SemioWorkflowMutation::SetEdgeEndpoints { id, .. } => match edge_at(base, id) {
                Some(edge) => vec![SemioWorkflowMutation::SetEdgeEndpoints { id: id.clone(), from: edge.from.clone(), to: edge.to.clone() }],
                None => vec![SemioWorkflowMutation::NoMutation],
            },
            SemioWorkflowMutation::SetEdgeKind { id, .. } => match edge_at(base, id) {
                Some(edge) => vec![SemioWorkflowMutation::SetEdgeKind { id: id.clone(), kind: edge.kind.clone() }],
                None => vec![SemioWorkflowMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ Hand-rolled `OpText`/`OpBinary` (per this ticket: no `#[derive(dsl::DslOps)]` fight —
/// `WorkflowNode`/`WorkflowEdge`/`SemioWorkflowSnapshot` are not `#[derive(dsl::DslRecord)]`, same
/// family of gap `DocxMutation`'s doc comment documents for its own `DocxBlock`/`DocxSnapshot`
/// payloads). Grammar: `keyword arg=value ...` (space-separated), reusing `schema::diff`'s
/// `pub(crate)` grammar primitives.
fn enc_semio_workflow_snapshot(s: &SemioWorkflowSnapshot) -> String {
    format!(
        "[{},{},{}]",
        enc_str(&s.schema),
        format!("[{}]", s.nodes.iter().map(enc_node).collect::<Vec<_>>().join(",")),
        format!("[{}]", s.edges.iter().map(enc_edge).collect::<Vec<_>>().join(","))
    )
}
fn dec_semio_workflow_snapshot(s: &str) -> Result<SemioWorkflowSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, nodes, edges] = parts.as_slice() else { return Err(format!("snapshot: expected 3 fields, got {}", parts.len())) };
    let nodes = split_top_level(strip_brackets(nodes)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_node).collect::<Result<Vec<_>, String>>()?;
    let edges = split_top_level(strip_brackets(edges)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioWorkflowSnapshot { schema: dec_str(schema)?, nodes, edges })
}

fn print_workflow_mutation(m: &SemioWorkflowMutation) -> String {
    match m {
        SemioWorkflowMutation::NoMutation => "no-mutation".to_string(),
        SemioWorkflowMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_semio_workflow_snapshot(snapshot)),
        SemioWorkflowMutation::InsertNode { node } => format!("insert-node node={}", enc_node(node)),
        SemioWorkflowMutation::RemoveNode { id } => format!("remove-node id={}", enc_str(id)),
        SemioWorkflowMutation::SetNodeKind { id, kind } => format!("set-node-kind id={} kind={}", enc_str(id), enc_str(kind)),
        SemioWorkflowMutation::SetNodeLabel { id, label } => format!("set-node-label id={} label={}", enc_str(id), enc_str(label)),
        SemioWorkflowMutation::SetNodePosition { id, position } => format!("set-node-position id={} position={}", enc_str(id), enc_point2(position)),
        SemioWorkflowMutation::SetNodeParam { id, key, value } => format!("set-node-param id={} key={} value={}", enc_str(id), enc_str(key), enc_str(value)),
        SemioWorkflowMutation::RemoveNodeParam { id, key } => format!("remove-node-param id={} key={}", enc_str(id), enc_str(key)),
        SemioWorkflowMutation::InsertEdge { edge } => format!("insert-edge edge={}", enc_edge(edge)),
        SemioWorkflowMutation::RemoveEdge { id } => format!("remove-edge id={}", enc_str(id)),
        SemioWorkflowMutation::SetEdgeEndpoints { id, from, to } => format!("set-edge-endpoints id={} from={} to={}", enc_str(id), enc_port_ref(from), enc_port_ref(to)),
        SemioWorkflowMutation::SetEdgeKind { id, kind } => format!("set-edge-kind id={} kind={}", enc_str(id), enc_str(kind)),
    }
}
fn parse_workflow_mutation(line: &str) -> Result<SemioWorkflowMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioWorkflowMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("workflow mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("workflow mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioWorkflowMutation::SetSnapshot { snapshot: dec_semio_workflow_snapshot(arg("snapshot")?)? }),
        "insert-node" => Ok(SemioWorkflowMutation::InsertNode { node: dec_node(arg("node")?)? }),
        "remove-node" => Ok(SemioWorkflowMutation::RemoveNode { id: dec_str(arg("id")?)? }),
        "set-node-kind" => Ok(SemioWorkflowMutation::SetNodeKind { id: dec_str(arg("id")?)?, kind: dec_str(arg("kind")?)? }),
        "set-node-label" => Ok(SemioWorkflowMutation::SetNodeLabel { id: dec_str(arg("id")?)?, label: dec_str(arg("label")?)? }),
        "set-node-position" => Ok(SemioWorkflowMutation::SetNodePosition { id: dec_str(arg("id")?)?, position: dec_point2(arg("position")?)? }),
        "set-node-param" => Ok(SemioWorkflowMutation::SetNodeParam { id: dec_str(arg("id")?)?, key: dec_str(arg("key")?)?, value: dec_str(arg("value")?)? }),
        "remove-node-param" => Ok(SemioWorkflowMutation::RemoveNodeParam { id: dec_str(arg("id")?)?, key: dec_str(arg("key")?)? }),
        "insert-edge" => Ok(SemioWorkflowMutation::InsertEdge { edge: dec_edge(arg("edge")?)? }),
        "remove-edge" => Ok(SemioWorkflowMutation::RemoveEdge { id: dec_str(arg("id")?)? }),
        "set-edge-endpoints" => Ok(SemioWorkflowMutation::SetEdgeEndpoints { id: dec_str(arg("id")?)?, from: dec_port_ref(arg("from")?)?, to: dec_port_ref(arg("to")?)? }),
        "set-edge-kind" => Ok(SemioWorkflowMutation::SetEdgeKind { id: dec_str(arg("id")?)?, kind: dec_str(arg("kind")?)? }),
        other => Err(format!("workflow mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioWorkflowMutation {
    fn print_op(&self) -> String {
        print_workflow_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_workflow_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification `DocxMutation`'s hand-rolled codec
/// uses.
impl protocol::OpBinary for SemioWorkflowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;

    fn node(id: &str, kind: &str, label: &str, x: f64, y: f64) -> WorkflowNode {
        WorkflowNode { id: id.into(), kind: kind.into(), label: label.into(), params: vec![crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::WorkflowParam { key: "k".into(), value: "v".into() }], position: SemioPoint2 { x, y } }
    }
    fn edge(id: &str, from_node: &str, to_node: &str, kind: &str) -> WorkflowEdge {
        WorkflowEdge { id: id.into(), from: PortRef { node: from_node.into(), port: "out".into() }, to: PortRef { node: to_node.into(), port: "in".into() }, kind: kind.into() }
    }

    fn fixture() -> SemioWorkflowSnapshot {
        SemioWorkflowSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA.into(),
            nodes: vec![node("n1", "source", "Source", 0.0, 0.0), node("n2", "sink", "Sink", 10.0, 10.0)],
            edges: vec![edge("e1", "n1", "n2", "data")],
        }
    }

    fn sample_mutations() -> Vec<SemioWorkflowMutation> {
        vec![
            SemioWorkflowMutation::NoMutation,
            SemioWorkflowMutation::SetSnapshot { snapshot: fixture() },
            SemioWorkflowMutation::InsertNode { node: node("n3", "transform", "T", 5.0, 5.0) },
            SemioWorkflowMutation::RemoveNode { id: "n2".into() },
            SemioWorkflowMutation::SetNodeKind { id: "n1".into(), kind: "changed".into() },
            SemioWorkflowMutation::SetNodeLabel { id: "n1".into(), label: "Changed".into() },
            SemioWorkflowMutation::SetNodePosition { id: "n1".into(), position: SemioPoint2 { x: 99.0, y: -1.0 } },
            SemioWorkflowMutation::SetNodeParam { id: "n1".into(), key: "k".into(), value: "new".into() },
            SemioWorkflowMutation::SetNodeParam { id: "n1".into(), key: "fresh".into(), value: "added".into() },
            SemioWorkflowMutation::RemoveNodeParam { id: "n1".into(), key: "k".into() },
            SemioWorkflowMutation::InsertEdge { edge: edge("e2", "n2", "n1", "back") },
            SemioWorkflowMutation::RemoveEdge { id: "e1".into() },
            SemioWorkflowMutation::SetEdgeEndpoints { id: "e1".into(), from: PortRef { node: "n2".into(), port: "out".into() }, to: PortRef { node: "n1".into(), port: "in".into() } },
            SemioWorkflowMutation::SetEdgeKind { id: "e1".into(), kind: "changed".into() },
        ]
    }

    //#region 🔖️MutationDiffLaw
    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = protocol::MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = apply_semio_workflow_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_semio_workflow_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <SemioWorkflowMutation as Mutation<SemioWorkflowSnapshot>>::inverse(&mutation, &base) {
                apply_semio_workflow_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = protocol::MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = protocol::MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️OpTextBinaryRoundtripLaw
    #[test]
    fn op_text_binary_roundtrip_law() {
        for mutation in sample_mutations() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioWorkflowMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = SemioWorkflowMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw

    //#region 🔖️VariantBehavior
    #[test]
    fn insert_then_remove_node_apply_and_inverse() {
        let base = fixture();
        let insert = SemioWorkflowMutation::InsertNode { node: node("n3", "transform", "T", 5.0, 5.0) };
        let mut after = base.clone();
        apply_semio_workflow_mutation(&mut after, &insert);
        assert_eq!(after.nodes.len(), 3);
        for inv in Mutation::inverse(&insert, &base) {
            apply_semio_workflow_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn node_param_mutations_apply_and_inverse() {
        let base = fixture();
        let set = SemioWorkflowMutation::SetNodeParam { id: "n1".into(), key: "k".into(), value: "new".into() };
        let mut after = base.clone();
        apply_semio_workflow_mutation(&mut after, &set);
        assert_eq!(param_value_at(&after, "n1", "k"), Some("new"));
        for inv in Mutation::inverse(&set, &base) {
            apply_semio_workflow_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let add = SemioWorkflowMutation::SetNodeParam { id: "n1".into(), key: "fresh".into(), value: "added".into() };
        let mut after2 = base.clone();
        apply_semio_workflow_mutation(&mut after2, &add);
        assert_eq!(param_value_at(&after2, "n1", "fresh"), Some("added"));
        for inv in Mutation::inverse(&add, &base) {
            apply_semio_workflow_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);
    }

    #[test]
    fn edge_mutations_apply_and_inverse() {
        let base = fixture();
        let set = SemioWorkflowMutation::SetEdgeEndpoints { id: "e1".into(), from: PortRef { node: "n2".into(), port: "out".into() }, to: PortRef { node: "n1".into(), port: "in".into() } };
        let mut after = base.clone();
        apply_semio_workflow_mutation(&mut after, &set);
        assert_eq!(edge_at(&after, "e1").unwrap().from.node, "n2");
        for inv in Mutation::inverse(&set, &base) {
            apply_semio_workflow_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }
    //#endregion 🔖️VariantBehavior
}
//#endregion 🔖️Tests
