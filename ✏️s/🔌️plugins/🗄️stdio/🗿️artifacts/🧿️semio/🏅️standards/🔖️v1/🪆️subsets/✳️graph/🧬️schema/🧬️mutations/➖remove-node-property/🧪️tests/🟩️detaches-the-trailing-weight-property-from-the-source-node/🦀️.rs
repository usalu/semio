//! 🧪️ `remove-node-property` fixture — `🟩️detaches-the-trailing-weight-property-from-the-source-node`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, which has TWO distinct Error
//! `mutation.target-missing` branches: an unknown `node_id` (target `[node_id]`) and an `index`
//! past the end of that node's `properties` (target `[node_id, index]`). Neither fires here.
//! Removing the TRAILING entry keeps the surviving entry's index stable, which is what lets the
//! inverse re-insert at the very same index.
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
    serde_json::from_str(BEFORE).expect("remove-node-property before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("remove-node-property after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("remove-node-property mutation decodes")
}

/// ▶️ The trailing `weight` entry goes; the leading `colour` entry stays at index 0.
#[semio_framework_async_macros::async_test]
async fn detaches_only_the_trailing_property() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("remove-node-property applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-node-property/detaches-the-trailing-weight-property-from-the-source-node: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes[0].properties.len(), base.nodes[0].properties.len() - 1, "the nested properties collection shrinks by exactly one");
    assert_eq!(produced.nodes[0].properties[0], base.nodes[0].properties[0], "the untargeted leading entry must stay exactly where it was");
    assert!(!produced.nodes[0].properties.iter().any(|entry| entry.key == "weight"), "the entry addressed by nested index 1 must be gone");
    assert_eq!(produced.nodes[0].label, base.nodes[0].label, "removing a property must not touch the node's own scalar fields");
}

/// ↩️ The undo re-attaches the captured entry at the same nested BASE index.
#[semio_framework_async_macros::async_test]
async fn the_undo_add_node_property_reattaches_the_weight_entry_in_place() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "remove-node-property of an existing entry undoes as exactly one add-node-property");
    assert!(matches!(undo[0], SemioGraphMutation::AddNodeProperty(_)), "the undo of a remove is the matching add");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward remove-node-property applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo add-node-property applies to the stripped graph");
    }
    assert_eq!(current, base, "remove-node-property/detaches-the-trailing-weight-property-from-the-source-node: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"RemoveNodeProperty":{"node_id":{"value":"a"},"index":1}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-node-property/detaches-the-trailing-weight-property-from-the-source-node: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("remove-node-property mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("remove-node-property mutation reparses");
    assert_eq!(reencoded, original, "remove-node-property/detaches-the-trailing-weight-property-from-the-source-node: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the node exists and index 1 is within its properties, so neither target-missing branch may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-node-property/detaches-the-trailing-weight-property-from-the-source-node: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "the node exists and index 1 is within its properties, so neither target-missing branch may fire");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-node-property/detaches-the-trailing-weight-property-from-the-source-node: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and omits `edges` entirely — `remove-node-property` may rewrite `nodes`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_edges_entirely() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed remove-node-property diff decodes");
    assert!(decoded.edges.is_none(), "remove-node-property must leave the edges slot untouched");
    assert_eq!(decoded.nodes.as_ref().map(|list| list.values.len()), Some(2), "the diff must carry the whole rebuilt nodes list");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("edges").is_none(), "the committed diff JSON must not carry a edges key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "remove-node-property/detaches-the-trailing-weight-property-from-the-source-node: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed remove-node-property diff decodes");
    let produced = decoded.apply(&before()).expect("committed remove-node-property diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-node-property/detaches-the-trailing-weight-property-from-the-source-node: committed diff did not carry before to after");
}
