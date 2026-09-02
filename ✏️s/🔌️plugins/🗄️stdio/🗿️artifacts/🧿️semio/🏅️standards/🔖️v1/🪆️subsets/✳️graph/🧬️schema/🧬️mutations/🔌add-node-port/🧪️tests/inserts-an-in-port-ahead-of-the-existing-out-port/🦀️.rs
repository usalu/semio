//! 🧪️ `add-node-port` fixture — `inserts-an-in-port-ahead-of-the-existing-out-port`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown `node_id` ⇒ Error
//! `mutation.target-missing`; a port whose NAME already exists on that node ⇒ Warning
//! `mutation.no-op` (name-collision, not value-equality); otherwise the port is inserted into the
//! node's NESTED ordered `ports` at `min(index, len)`. Inserting at index 0 in front of an existing
//! port is what pins position rather than mere membership.
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
    serde_json::from_str(BEFORE).expect("add-node-port before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("add-node-port after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("add-node-port mutation decodes")
}

/// ▶️ The new `in` port takes nested index 0 and pushes the existing `out` port to index 1.
#[semio_framework_async_macros::async_test]
async fn inserts_the_reset_port_ahead_of_the_out_port() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("add-node-port applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "add-node-port/inserts-an-in-port-ahead-of-the-existing-out-port: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes[0].ports.len(), base.nodes[0].ports.len() + 1, "the nested ports collection grows by exactly one");
    assert_eq!(produced.nodes[0].ports[0].name, "reset", "the new port must occupy the FINAL-state index it was addressed with");
    assert_eq!(produced.nodes[0].ports[1], base.nodes[0].ports[0], "the pre-existing out port must survive, merely shifted");
    assert_eq!(produced.nodes[1], base.nodes[1], "add-node-port must not touch any other node");
}

/// ↩️ The undo is a `remove-node-port` at the clamped index the port landed at.
#[semio_framework_async_macros::async_test]
async fn the_undo_remove_node_port_detaches_the_reset_port_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "add-node-port undoes as exactly one remove-node-port");
    assert!(matches!(undo[0], SemioGraphMutation::RemoveNodePort(_)), "the undo of an add is the matching remove");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward add-node-port applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo remove-node-port applies to the ported graph");
    }
    assert_eq!(current, base, "add-node-port/inserts-an-in-port-ahead-of-the-existing-out-port: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `SemioGraphPortKind` is `rename_all = "camelCase"`, so `In` encodes as `"in"` and `InOut` would encode as `"inOut"`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-node-port/inserts-an-in-port-ahead-of-the-existing-out-port: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("add-node-port mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("add-node-port mutation reparses");
    assert_eq!(reencoded, original, "add-node-port/inserts-an-in-port-ahead-of-the-existing-out-port: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the node exists and carries no port named reset, so neither target-missing nor the name-collision no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-node-port/inserts-an-in-port-ahead-of-the-existing-out-port: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "the node exists and carries no port named reset, so neither target-missing nor the name-collision no-op may fire");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "add-node-port/inserts-an-in-port-ahead-of-the-existing-out-port: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and omits `edges` entirely — `add-node-port` may rewrite `nodes`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_edges_entirely() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed add-node-port diff decodes");
    assert!(decoded.edges.is_none(), "add-node-port must leave the edges slot untouched");
    assert_eq!(decoded.nodes.as_ref().map(|list| list.values.len()), Some(2), "the diff must carry the whole rebuilt nodes list");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("edges").is_none(), "the committed diff JSON must not carry a edges key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "add-node-port/inserts-an-in-port-ahead-of-the-existing-out-port: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed add-node-port diff decodes");
    let produced = decoded.apply(&before()).expect("committed add-node-port diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-node-port/inserts-an-in-port-ahead-of-the-existing-out-port: committed diff did not carry before to after");
}
