//! 🧪️ `change-node-label` fixture — `relabels-the-source-node-without-retyping-it`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown id ⇒ Error `mutation.target-missing`,
//! label already equal ⇒ Warning `mutation.no-op`; otherwise ONLY `nodes[i].label` is assigned.
//! `label` and `kind` are two SEPARATE scalar fields with two separate triads, so the load-bearing
//! claim here is that relabelling leaves `kind` alone.
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
    serde_json::from_str(BEFORE).expect("change-node-label before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("change-node-label after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("change-node-label mutation decodes")
}

/// ▶️ The node's display label changes and its type tag does not.
#[semio_framework_async_macros::async_test]
async fn relabels_the_node_without_touching_its_kind() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-node-label applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-node-label/relabels-the-source-node-without-retyping-it: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes[0].label, "Sensor", "the node's label must become new_label");
    assert_eq!(produced.nodes[0].kind, base.nodes[0].kind, "change-node-label must NOT touch the separate kind field");
    assert_eq!(produced.nodes[0].position, base.nodes[0].position, "change-node-label must not move the node");
    assert_eq!(produced.nodes[1], base.nodes[1], "the untargeted node must be byte-identical");
}

/// ↩️ The undo is a `change-node-label` carrying BASE's captured label.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_node_label_restores_the_original_label() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-node-label of an existing node undoes as exactly one change-node-label");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-node-label applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-node-label applies to the relabelled graph");
    }
    assert_eq!(current, base, "change-node-label/relabels-the-source-node-without-retyping-it: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points — `GraphNodeId` is a NAMED
/// single-field struct, so every id encodes as `{"value":…}` rather than a bare string.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-node-label/relabels-the-source-node-without-retyping-it: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-node-label mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-node-label mutation reparses");
    assert_eq!(reencoded, original, "change-node-label/relabels-the-source-node-without-retyping-it: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the node exists and the new label genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-node-label/relabels-the-source-node-without-retyping-it: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "the node exists and the new label genuinely differs, so neither target-missing nor no-op may fire");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-node-label/relabels-the-source-node-without-retyping-it: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and omits `edges` entirely — `change-node-label` may rewrite `nodes`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_edges_entirely() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed change-node-label diff decodes");
    assert!(decoded.edges.is_none(), "change-node-label must leave the edges slot untouched");
    assert_eq!(decoded.nodes.as_ref().map(|list| list.values.len()), Some(2), "the diff must carry the whole rebuilt nodes list");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("edges").is_none(), "the committed diff JSON must not carry a edges key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "change-node-label/relabels-the-source-node-without-retyping-it: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed change-node-label diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-node-label diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-node-label/relabels-the-source-node-without-retyping-it: committed diff did not carry before to after");
}
