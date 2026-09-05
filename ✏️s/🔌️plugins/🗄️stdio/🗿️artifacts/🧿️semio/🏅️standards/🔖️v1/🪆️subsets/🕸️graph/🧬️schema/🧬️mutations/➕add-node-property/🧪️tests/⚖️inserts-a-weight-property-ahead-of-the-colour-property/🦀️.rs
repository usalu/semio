//! 🧪️ `add-node-property` fixture — `⚖️inserts-a-weight-property-ahead-of-the-colour-property`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown `node_id` ⇒ Error
//! `mutation.target-missing`; a property whose KEY already exists on that node ⇒ Warning
//! `mutation.no-op`; otherwise the entry is inserted into the node's nested ordered `properties` at
//! `min(index, len)`. The property type is `🔢️value`'s own `SemioValueEntry`, reused verbatim — a
//! `Float` keeps its SOURCE LEXEME as a string, never a JSON number.
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
    serde_json::from_str(BEFORE).expect("add-node-property before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("add-node-property after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("add-node-property mutation decodes")
}

/// ▶️ The new `weight` entry takes nested index 0 and pushes `colour` to index 1.
#[semio_framework_async_macros::async_test]
async fn inserts_the_weight_property_ahead_of_the_colour_property() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("add-node-property applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "add-node-property/inserts-a-weight-property-ahead-of-the-colour-property: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes[0].properties.len(), base.nodes[0].properties.len() + 1, "the nested properties collection grows by exactly one");
    assert_eq!(produced.nodes[0].properties[0].key, "weight", "the new entry must occupy the FINAL-state index it was addressed with");
    assert_eq!(produced.nodes[0].properties[1], base.nodes[0].properties[0], "the pre-existing colour entry must survive, merely shifted");
    assert_eq!(produced.nodes[0].ports, base.nodes[0].ports, "adding a property must not touch the node's ports");
}

/// ↩️ The undo is a `remove-node-property` at the clamped index the entry landed at.
#[semio_framework_async_macros::async_test]
async fn the_undo_remove_node_property_detaches_the_weight_entry_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "add-node-property undoes as exactly one remove-node-property");
    assert!(matches!(undo[0], SemioGraphMutation::RemoveNodeProperty(_)), "the undo of an add is the matching remove");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward add-node-property applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo remove-node-property applies to the enriched graph");
    }
    assert_eq!(current, base, "add-node-property/inserts-a-weight-property-ahead-of-the-colour-property: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `SemioValue` is internally tagged, so the float entry encodes as `{"kind":"float","lexeme":"0.5"}` with a STRING lexeme.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-node-property/inserts-a-weight-property-ahead-of-the-colour-property: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("add-node-property mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("add-node-property mutation reparses");
    assert_eq!(reencoded, original, "add-node-property/inserts-a-weight-property-ahead-of-the-colour-property: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the node exists and carries no property keyed weight, so neither target-missing nor the key-collision no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-node-property/inserts-a-weight-property-ahead-of-the-colour-property: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "the node exists and carries no property keyed weight, so neither target-missing nor the key-collision no-op may fire");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "add-node-property/inserts-a-weight-property-ahead-of-the-colour-property: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and omits `edges` entirely — `add-node-property` may rewrite `nodes`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_edges_entirely() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed add-node-property diff decodes");
    assert!(decoded.edges.is_none(), "add-node-property must leave the edges slot untouched");
    assert_eq!(decoded.nodes.as_ref().map(|list| list.values.len()), Some(2), "the diff must carry the whole rebuilt nodes list");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("edges").is_none(), "the committed diff JSON must not carry a edges key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "add-node-property/inserts-a-weight-property-ahead-of-the-colour-property: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed add-node-property diff decodes");
    let produced = decoded.apply(&before()).expect("committed add-node-property diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-node-property/inserts-a-weight-property-ahead-of-the-colour-property: committed diff did not carry before to after");
}
