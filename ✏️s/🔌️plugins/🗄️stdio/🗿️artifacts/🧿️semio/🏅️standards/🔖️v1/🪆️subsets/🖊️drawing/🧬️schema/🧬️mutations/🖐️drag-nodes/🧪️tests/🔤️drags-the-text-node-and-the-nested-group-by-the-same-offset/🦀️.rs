//! 🧪️ `drag-nodes` fixture — `🔤️drags-the-text-node-and-the-nested-group-by-the-same-offset`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a non-finite offset is FATAL
//! `mutation.invariant`, a zero offset is Warning `mutation.no-op`, and a non-empty `ats` where
//! NONE resolve is Error `mutation.target-missing`. Unlike `move-node`, the offset is RELATIVE —
//! each node's own origin is read from `base` and shifted. The per-node diffs are then ABSORBED
//! into one, so the committed diff carries two sibling entries under a single `children` triple
//! rather than two separate layer diffs.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioDrawingSnapshot {
    serde_json::from_str(BEFORE).expect("drag-nodes before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("drag-nodes after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("drag-nodes mutation decodes")
}

/// ▶️ Both addressed nodes shift by the same delta, each from its OWN starting origin.
#[semio_framework_async_macros::async_test]
async fn shifts_both_nodes_by_the_same_relative_offset() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("drag-nodes applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "drag-nodes/drags-the-text-node-and-the-nested-group-by-the-same-offset: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Text { at, .. } = &children[1] else { panic!("child #1 is the text node") };
    assert_eq!((at.x, at.y), (7.0, 4.0), "the text anchor is BASE origin plus offset, not the offset itself");
    let DrawNode::Group { transform, .. } = &children[2] else { panic!("child #2 is the nested group") };
    assert_eq!((transform.translation.x, transform.translation.y), (2.0, -1.0), "a group is dragged through its transform translation, from its own base origin");
    assert_eq!(transform.translation.z, 0.0, "a 2D drag must leave the Z translation alone");
    assert_eq!(children[0], base_children_of(&base)[0], "the unaddressed sibling must be byte-identical");
}

// 🚫️async: local reader for the layer-root children of a snapshot — this fixture reads them twice.
fn base_children_of(snapshot: &SemioDrawingSnapshot) -> Vec<DrawNode> {
    let DrawNode::Group { children, .. } = &snapshot.layers[0].root else { panic!("the layer root is a group") };
    children.clone()
}

/// ↩️ The undo is a single `drag-nodes` over the SAME paths with the NEGATED offset.
#[semio_framework_async_macros::async_test]
async fn the_undo_drag_nodes_negates_the_offset() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "drag-nodes undoes as exactly one drag-nodes — one step for any number of dragged nodes");
    let SemioDrawingMutation::DragNodes(back) = &undo[0] else { panic!("drag-nodes must undo as drag-nodes") };
    assert_eq!((back.offset.x, back.offset.y), (-2.0, 1.0), "the undo negates the offset rather than capturing origins");
    assert_eq!(back.ats.len(), 2, "and addresses exactly the same paths");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward drag-nodes applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo drag-nodes applies");
    }
    assert_eq!(current, base, "drag-nodes/drags-the-text-node-and-the-nested-group-by-the-same-offset: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `ats` is an array of `NodePath` objects and the offset is a `SemioPoint2`, all dyadic.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "drag-nodes/drags-the-text-node-and-the-nested-group-by-the-same-offset: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("drag-nodes mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("drag-nodes mutation reparses");
    assert_eq!(reencoded, original, "drag-nodes/drags-the-text-node-and-the-nested-group-by-the-same-offset: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the offset is finite and non-zero and both paths resolve, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "drag-nodes/drags-the-text-node-and-the-nested-group-by-the-same-offset: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, non-zero drag over resolvable paths must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. ONE `children` triple carrying two modified entries — the per-node diffs were absorbed together, not emitted as two layer diffs.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "drag-nodes/drags-the-text-node-and-the-nested-group-by-the-same-offset: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed drag-nodes diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(layers.modified.len(), 1, "the two per-node diffs absorbed into a SINGLE layer entry");
    assert_eq!(children.modified.len(), 2, "and into a single children triple with one entry per dragged node");
    assert_eq!((children.modified[0].index, children.modified[1].index), (1, 2), "the entries keep the order the ats were listed in");
    assert!(matches!(children.modified[0].diff, crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Text(_)), "the text node gets the Text arm");
    assert!(matches!(children.modified[1].diff, crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(_)), "the group node gets the Group arm — the diff shape follows the node KIND");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "drag-nodes/drags-the-text-node-and-the-nested-group-by-the-same-offset: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed drag-nodes diff decodes");
    let produced = decoded.apply(&before()).expect("committed drag-nodes diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "drag-nodes/drags-the-text-node-and-the-nested-group-by-the-same-offset: committed diff did not carry before to after");
}
