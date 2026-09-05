//! 🧪️ `change-node-kind` fixture — `🔤️retypes-the-source-node-without-relabelling-it`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown id ⇒ Error `mutation.target-missing`,
//! kind already equal ⇒ Warning `mutation.no-op`; otherwise ONLY `nodes[i].kind` is assigned.
//! The mirror image of `change-node-label`: here the freeform TYPE tag moves and the human-facing
//! label must stay exactly where it was.
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
    serde_json::from_str(BEFORE).expect("change-node-kind before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("change-node-kind after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("change-node-kind mutation decodes")
}

/// ▶️ The node's type tag changes and its display label does not.
#[semio_framework_async_macros::async_test]
async fn retypes_the_node_without_touching_its_label() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-node-kind applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-node-kind/retypes-the-source-node-without-relabelling-it: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes[0].kind, "generator", "the node's kind must become new_kind");
    assert_eq!(produced.nodes[0].label, base.nodes[0].label, "change-node-kind must NOT touch the separate label field");
    assert_eq!(produced.nodes[0].ports, base.nodes[0].ports, "retyping a node must not rewrite its ports");
    assert_eq!(produced.nodes[1], base.nodes[1], "the untargeted node must be byte-identical");
}

/// ↩️ The undo is a `change-node-kind` carrying BASE's captured kind.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_node_kind_restores_the_original_kind() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-node-kind of an existing node undoes as exactly one change-node-kind");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-node-kind applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-node-kind applies to the retyped graph");
    }
    assert_eq!(current, base, "change-node-kind/retypes-the-source-node-without-relabelling-it: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points — `GraphNodeId` is a NAMED
/// single-field struct, so every id encodes as `{"value":…}` rather than a bare string.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-node-kind/retypes-the-source-node-without-relabelling-it: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-node-kind mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-node-kind mutation reparses");
    assert_eq!(reencoded, original, "change-node-kind/retypes-the-source-node-without-relabelling-it: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the node exists and the new kind genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-node-kind/retypes-the-source-node-without-relabelling-it: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "the node exists and the new kind genuinely differs, so neither target-missing nor no-op may fire");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-node-kind/retypes-the-source-node-without-relabelling-it: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and omits `edges` entirely — `change-node-kind` may rewrite `nodes`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_edges_entirely() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed change-node-kind diff decodes");
    assert!(decoded.edges.is_none(), "change-node-kind must leave the edges slot untouched");
    assert_eq!(decoded.nodes.as_ref().map(|list| list.values.len()), Some(2), "the diff must carry the whole rebuilt nodes list");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("edges").is_none(), "the committed diff JSON must not carry a edges key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "change-node-kind/retypes-the-source-node-without-relabelling-it: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed change-node-kind diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-node-kind diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-node-kind/retypes-the-source-node-without-relabelling-it: committed diff did not carry before to after");
}
