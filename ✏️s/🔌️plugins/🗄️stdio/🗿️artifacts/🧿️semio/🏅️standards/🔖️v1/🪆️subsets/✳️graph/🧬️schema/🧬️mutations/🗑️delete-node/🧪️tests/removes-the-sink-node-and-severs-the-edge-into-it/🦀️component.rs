//! 🧪️ `delete-node` fixture — `removes-the-sink-node-and-severs-the-edge-into-it`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an unknown id is Error `mutation.target-missing`;
//! otherwise the node is retained-out of `nodes` AND every edge whose `source` OR `target` is that
//! id is retained-out of `edges`, with an INFO `mutation.cascade` counting the severed edges. The
//! before-snapshot deliberately has an edge pointing INTO the deleted node, so the cascade is real
//! and the committed diff must carry BOTH slots.
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioGraphSnapshot {
    serde_json::from_str(BEFORE).expect("delete-node before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("delete-node after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("delete-node mutation decodes")
}

/// ▶️ The sink node goes and the edge that referenced it goes with it — a dangling edge would be
/// an invalid graph.
#[semio_framework_async_macros::async_test]
async fn deletes_the_sink_node_and_severs_the_edge_that_referenced_it() {
    let base = before();
    assert!(base.edges.iter().any(|edge| edge.target.value == "b"), "the fixture needs an edge INTO the deleted node for the cascade to mean anything");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-node applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-node/removes-the-sink-node-and-severs-the-edge-into-it: applied state differs from the committed after-snapshot");
    assert!(!produced.nodes.iter().any(|node| node.id.value == "b"), "the named node must be gone");
    assert!(produced.edges.is_empty(), "the only edge referenced the deleted node, so it must have been severed");
    assert_eq!(produced.nodes, vec![base.nodes[0].clone()], "the untargeted source node must survive byte-identical");
}

/// ↩️ The undo re-creates the node AND re-creates every severed edge — one `create-node` followed
/// by one `create-edge` per severed edge, in that order.
#[semio_framework_async_macros::async_test]
async fn the_undo_recreates_the_node_then_every_severed_edge() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 2, "one create-node plus one create-edge for the single severed edge");
    assert!(matches!(undo[0], SemioGraphMutation::CreateNode(_)), "the node must be re-created FIRST — create-edge rejects an unknown endpoint");
    assert!(matches!(undo[1], SemioGraphMutation::CreateEdge(_)), "the severed edge is re-created after its endpoint exists again");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-node applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("each undo step applies to the running state");
    }
    assert_eq!(current, base, "delete-node/removes-the-sink-node-and-severs-the-edge-into-it: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteNode":{"id":{"value":"b"}}}` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-node/removes-the-sink-node-and-severs-the-edge-into-it: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-node mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-node mutation reparses");
    assert_eq!(reencoded, original, "delete-node/removes-the-sink-node-and-severs-the-edge-into-it: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` WITH the `mutation.cascade` note — exactly one edge was severed, and an
/// INFO message never turns an applied mutation into a rejected one.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_including_the_cascade_note() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-node/removes-the-sink-node-and-severs-the-edge-into-it: this case is declared applied");
    let produced = mutation().diff(&before());
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "deleting a node that really severed an edge raises exactly one message");
    assert_eq!(messages[0].code.0, "mutation.cascade", "the message must be the cascade note, not a rejection");
    assert_eq!(messages[0].level, protocol::Severity::Info, "a cascade note is INFO — the mutation still applies");
}

/// 🔺️ The produced delta equals the committed diff — both slots, because the severing is part of
/// the same diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-node/removes-the-sink-node-and-severs-the-edge-into-it: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and its `edges` list is already emptied.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_carries_both_slots() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed delete-node diff decodes");
    assert_eq!(decoded.nodes.as_ref().map(|list| list.values.len()), Some(1), "the diff must carry the single surviving node");
    assert_eq!(decoded.edges.as_ref().map(|list| list.values.len()), Some(0), "the diff must ALSO carry the emptied edge list — the sever is part of the same diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-node/removes-the-sink-node-and-severs-the-edge-into-it: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed delete-node diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-node diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-node/removes-the-sink-node-and-severs-the-edge-into-it: committed diff did not carry before to after");
}
