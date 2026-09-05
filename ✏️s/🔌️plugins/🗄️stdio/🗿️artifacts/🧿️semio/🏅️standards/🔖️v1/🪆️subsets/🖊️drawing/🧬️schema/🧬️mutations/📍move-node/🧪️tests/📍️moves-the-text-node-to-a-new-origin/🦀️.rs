//! 🧪️ `move-node` fixture — `📍️moves-the-text-node-to-a-new-origin`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, guards in order: an unresolvable path ⇒ Error
//! `mutation.target-missing`; a non-finite origin ⇒ FATAL `mutation.invariant`; an unchanged origin
//! ⇒ Warning `mutation.no-op`. `diff_move_node` then branches on node KIND: a `Text` (this case)
//! gets a `DrawTextDiff { at }`, a `Group` gets a whole rebuilt `transform`, an `Image` gets its
//! own `at` — and a `Path` has no origin at all, so it silently produces an EMPTY diff.

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
    serde_json::from_str(BEFORE).expect("move-node before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("move-node after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("move-node mutation decodes")
}

/// ▶️ Only the text node's anchor moves; its value, style and every sibling stay.
#[semio_framework_async_macros::async_test]
async fn moves_the_text_anchor_and_nothing_else() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("move-node applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "move-node/moves-the-text-node-to-a-new-origin: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Text { at, value, .. } = &children[1] else { panic!("child #1 is the text node") };
    assert_eq!((at.x, at.y), (12.0, 8.0), "the anchor takes the payload's ABSOLUTE origin, not an offset");
    assert_eq!(value, "Hello", "moving a text node must not rewrite its value");
    let DrawNode::Group { children: base_children, .. } = &base.layers[0].root else { panic!("the layer root is a group") };
    assert_eq!((children[0].clone(), children[2].clone()), (base_children[0].clone(), base_children[2].clone()), "the sibling nodes must be byte-identical");
}

/// ↩️ The undo is a `move-node` carrying BASE's captured origin for that path.
#[semio_framework_async_macros::async_test]
async fn the_undo_move_node_restores_the_captured_origin() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "move-node of a resolvable node undoes as exactly one move-node");
    let SemioDrawingMutation::MoveNode(restore) = &undo[0] else { panic!("move-node must undo as move-node") };
    assert_eq!((restore.new_origin.x, restore.new_origin.y), (5.0, 5.0), "the undo must recapture BASE's own anchor");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward move-node applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo move-node applies");
    }
    assert_eq!(current, base, "move-node/moves-the-text-node-to-a-new-origin: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — every coordinate is dyadic, so decode→encode is exact.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-node/moves-the-text-node-to-a-new-origin: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("move-node mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("move-node mutation reparses");
    assert_eq!(reencoded, original, "move-node/moves-the-text-node-to-a-new-origin: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the path resolves, the origin is finite and genuinely different, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-node/moves-the-text-node-to-a-new-origin: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, genuinely-different origin must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children.modified` entry whose node diff is the TEXT arm carrying `at` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-node/moves-the-text-node-to-a-new-origin: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed move-node diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(children.modified.len(), 1, "exactly one child is modified");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Text(text_diff) = &children.modified[0].diff else { panic!("moving a Text node must produce the Text arm, never Replace") };
    assert!(text_diff.at.is_some(), "the anchor must be written");
    assert!(text_diff.value.is_none() && text_diff.style.is_none(), "neither the value nor the style may be written");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-node/moves-the-text-node-to-a-new-origin: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed move-node diff decodes");
    let produced = decoded.apply(&before()).expect("committed move-node diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-node/moves-the-text-node-to-a-new-origin: committed diff did not carry before to after");
}
