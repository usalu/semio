//! 🧪️ `move-node` fixture — `moves-the-sink-node-to-a-new-canvas-position`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, whose guards run in this exact order: unknown
//! id ⇒ Error `mutation.target-missing`; non-finite `new_position` ⇒ FATAL `mutation.invariant`;
//! position already equal ⇒ Warning `mutation.no-op`. Otherwise ONLY `nodes[i].position` is
//! assigned. `(6, -2.5)` is dyadic, so the canonical-JSON assertion is exact.
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
    serde_json::from_str(BEFORE).expect("move-node before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("move-node after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("move-node mutation decodes")
}

/// ▶️ Only the addressed node's canvas position changes; its kind, label and the edges do not.
#[semio_framework_async_macros::async_test]
async fn moves_only_the_addressed_node() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("move-node applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "move-node/moves-the-sink-node-to-a-new-canvas-position: applied state differs from the committed after-snapshot");
    assert_eq!((produced.nodes[1].position.x, produced.nodes[1].position.y), (6.0, -2.5), "the node's position must become the payload's absolute coordinates");
    assert_eq!(produced.nodes[1].label, base.nodes[1].label, "move-node must not touch the node's label");
    assert_eq!(produced.nodes[0], base.nodes[0], "the untargeted node must be byte-identical");
    assert_eq!(produced.edges, base.edges, "moving a node must not disturb the edges connected to it");
}

/// ↩️ The undo is a `move-node` back to BASE's own coordinates.
#[semio_framework_async_macros::async_test]
async fn the_undo_move_node_restores_the_original_position() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "move-node of an existing node undoes as exactly one move-node");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward move-node applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo move-node applies to the moved graph");
    }
    assert_eq!(current, base, "move-node/moves-the-sink-node-to-a-new-canvas-position: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points — `GraphNodeId` is a NAMED
/// single-field struct, so every id encodes as `{"value":…}` rather than a bare string.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-node/moves-the-sink-node-to-a-new-canvas-position: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("move-node mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("move-node mutation reparses");
    assert_eq!(reencoded, original, "move-node/moves-the-sink-node-to-a-new-canvas-position: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the node exists, the coordinates are finite and genuinely different, so none of target-missing/invariant/no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-node/moves-the-sink-node-to-a-new-canvas-position: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "the node exists, the coordinates are finite and genuinely different, so none of target-missing/invariant/no-op may fire");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-node/moves-the-sink-node-to-a-new-canvas-position: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and omits `edges` entirely — `move-node` may rewrite `nodes`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_edges_entirely() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed move-node diff decodes");
    assert!(decoded.edges.is_none(), "move-node must leave the edges slot untouched");
    assert_eq!(decoded.nodes.as_ref().map(|list| list.values.len()), Some(2), "the diff must carry the whole rebuilt nodes list");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("edges").is_none(), "the committed diff JSON must not carry a edges key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "move-node/moves-the-sink-node-to-a-new-canvas-position: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed move-node diff decodes");
    let produced = decoded.apply(&before()).expect("committed move-node diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-node/moves-the-sink-node-to-a-new-canvas-position: committed diff did not carry before to after");
}
