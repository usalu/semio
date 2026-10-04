//! 🧪️ `delete-edge` fixture — `✂️removes`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown edge id is Error
//! `mutation.target-missing`; otherwise `edges` is rebuilt without that id and `nodes` stays
//! `None`. This is the asymmetry with `delete-node`, which cascades into `edges`: deleting an EDGE
//! never cascades into `nodes`, and the two-edge before-snapshot proves the sibling edge survives.
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-edge/✂️removes/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-edge/✂️removes/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-edge/✂️removes/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-edge/✂️removes/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-edge/✂️removes/🎯️outcome/🔣️.json");

fn before() -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::schema::snapshot::decode_semio_graph_snapshot_json(BEFORE).expect("delete-edge before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::schema::snapshot::decode_semio_graph_snapshot_json(AFTER).expect("delete-edge after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-edge mutation decodes")
}

/// ▶️ Only the feedback edge goes; the forward edge and both endpoint nodes stay.
#[semio_framework_async_macros::async_test]
async fn removes_only_the_addressed_edge() {
    let base = before();
    assert_eq!(base.edges.len(), 2, "the fixture needs a sibling edge for the no-cascade claim to mean anything");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-edge applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-edge/removes-the-feedback-edge-and-keeps-both-endpoints: applied state differs from the committed after-snapshot");
    assert!(!produced.edges.iter().any(|edge| edge.id.value == "e2"), "the addressed edge must be gone");
    assert_eq!(produced.edges, vec![base.edges[0].clone()], "the sibling edge must survive byte-identical");
    assert_eq!(produced.nodes, base.nodes, "deleting an edge must NOT cascade into the nodes it connected");
}

/// ↩️ The undo re-creates the edge with its captured endpoints, kind and label at its BASE index — not a bare stub.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_edge_restores_the_full_captured_edge_in_place() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    let [SemioGraphMutation::CreateEdge(recreate)] = undo.as_slice() else { panic!("delete-edge must undo as one create-edge: {undo:?}") };
    assert_eq!(recreate.label, "B back to A", "the undo must recapture the deleted edge's own label from base");
    assert_eq!(recreate.kind, "feedback", "the undo must recapture the deleted edge's own kind from base");
    assert_eq!(recreate.at, base.edges.iter().position(|edge| edge.id == recreate.id), "the undo names the edge's base index");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-edge applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-edge applies to the disconnected graph");
    }
    assert_eq!(current, base, "delete-edge/removes-the-feedback-edge-and-keeps-both-endpoints: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteEdge":{"id":{"value":"e2"}}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = crate::standards::v1::subsets::graph::schema::snapshot::decode_semio_graph_snapshot_json(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::graph::schema::snapshot::encode_semio_graph_snapshot_json(&decoded).expect("snapshot encodes")).expect("snapshot reparses");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-edge/removes-the-feedback-edge-and-keeps-both-endpoints: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("delete-edge mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-edge mutation reparses");
    assert_eq!(reencoded, original, "delete-edge/removes-the-feedback-edge-and-keeps-both-endpoints: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the edge exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-edge/removes-the-feedback-edge-and-keeps-both-endpoints: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "the edge exists, so mutation.target-missing must not fire");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-edge/removes-the-feedback-edge-and-keeps-both-endpoints: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and omits `nodes` entirely — `delete-edge` may rewrite `edges`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_nodes_entirely() {
    let decoded: SemioGraphDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed delete-edge diff decodes");
    assert!(decoded.nodes.is_none(), "delete-edge must leave the nodes slot untouched");
    assert_eq!(decoded.edges.as_ref().map(|list| list.values.len()), Some(1), "the diff must carry the whole rebuilt edges list");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("nodes").is_none(), "the committed diff JSON must not carry a nodes key at all");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "delete-edge/removes-the-feedback-edge-and-keeps-both-endpoints: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed delete-edge diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-edge diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-edge/removes-the-feedback-edge-and-keeps-both-endpoints: committed diff did not carry before to after");
}
