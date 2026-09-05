//! 🧪️ `create-edge` fixture — `🟩️connects-the-source-node-to-the-sink-node`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, which has THREE rejection branches, all FATAL:
//! a duplicate `GraphEdgeId` (`mutation.duplicate-id`), an unknown `source` node and an unknown
//! `target` node (both `mutation.invariant`). Referential integrity is therefore checked BEFORE the
//! edge is pushed, and the edge lands at the end of `edges`. `nodes` stays `None` — connecting two
//! existing nodes rewrites no node.
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioGraphSnapshot {
    serde_json::from_str(BEFORE).expect("create-edge before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("create-edge after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("create-edge mutation decodes")
}

/// ▶️ The edge appears with both endpoints resolved, and neither endpoint node is rewritten.
#[semio_framework_async_macros::async_test]
async fn creates_the_edge_between_two_existing_nodes() {
    let base = before();
    assert!(base.edges.is_empty(), "the fixture starts with two nodes and no edge at all");
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-edge applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-edge/connects-the-source-node-to-the-sink-node: applied state differs from the committed after-snapshot");
    assert_eq!(produced.edges.len(), 1, "create-edge adds exactly one edge");
    let created = &produced.edges[0];
    assert_eq!(created.id.value, "e1", "the edge keeps the id the payload named");
    assert_eq!((created.source.value.as_str(), created.target.value.as_str()), ("a", "b"), "source and target are ordinary data fields carried straight from the payload");
    assert!(produced.nodes.iter().any(|node| node.id == created.source) && produced.nodes.iter().any(|node| node.id == created.target), "both endpoints must resolve to real nodes — that is what the two invariant guards protect");
    assert_eq!(produced.nodes, base.nodes, "connecting two nodes must not rewrite either of them");
}

/// ↩️ `create-edge`'s undo is a single `delete-edge` addressing the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_edge_removes_the_connection_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-edge undoes as exactly one delete-edge");
    assert!(matches!(undo[0], SemioGraphMutation::DeleteEdge(_)), "the undo of a create is the matching delete");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-edge applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-edge applies to the connected graph");
    }
    assert_eq!(current, base, "create-edge/connects-the-source-node-to-the-sink-node: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `GraphEdgeId` and `GraphNodeId` are both NAMED single-field structs, so every id encodes as `{"value":…}`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-edge/connects-the-source-node-to-the-sink-node: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-edge mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-edge mutation reparses");
    assert_eq!(reencoded, original, "create-edge/connects-the-source-node-to-the-sink-node: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the edge id is free and both endpoint nodes exist, so neither the duplicate-id nor either invariant branch may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-edge/connects-the-source-node-to-the-sink-node: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "the edge id is free and both endpoint nodes exist, so neither the duplicate-id nor either invariant branch may fire");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-edge/connects-the-source-node-to-the-sink-node: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and omits `nodes` entirely — `create-edge` may rewrite `edges`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_nodes_entirely() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed create-edge diff decodes");
    assert!(decoded.nodes.is_none(), "create-edge must leave the nodes slot untouched");
    assert_eq!(decoded.edges.as_ref().map(|list| list.values.len()), Some(1), "the diff must carry the whole rebuilt edges list");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("nodes").is_none(), "the committed diff JSON must not carry a nodes key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "create-edge/connects-the-source-node-to-the-sink-node: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed create-edge diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-edge diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-edge/connects-the-source-node-to-the-sink-node: committed diff did not carry before to after");
}
