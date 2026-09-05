//! 🧪️ `delete-node` fixture — `🚫️removes-the-text-node-from-the-layer-root`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a path with no parent (a bare layer root) is
//! Error `mutation.target-missing`, and so is an out-of-range index under a real group. Otherwise
//! the diff nests a `children.removed[index]` under the PARENT path — note the mutation addresses
//! the NODE while the diff addresses its parent, which is what `parent_and_index` is for.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioDrawingSnapshot {
    serde_json::from_str(BEFORE).expect("delete-node before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("delete-node after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("delete-node mutation decodes")
}

/// ▶️ The text node goes; its two siblings close up around it.
#[semio_framework_async_macros::async_test]
async fn removes_the_text_child_and_closes_the_gap() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-node applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-node/removes-the-text-node-from-the-layer-root: applied state differs from the committed after-snapshot");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    assert_eq!(children.len(), 2, "delete-node removes exactly one child");
    assert!(!children.iter().any(|node| matches!(node, crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode::Text { .. })), "the addressed text child must be gone");
    assert_eq!(produced.styles, base.styles, "deleting a node must not garbage-collect the style it referenced");
}

/// ↩️ The undo re-creates the node at the same parent and index, with its whole content.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_node_restores_the_captured_child_in_place() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "delete-node of an existing child undoes as exactly one create-node");
    let SemioDrawingMutation::CreateNode(recreate) = &undo[0] else { panic!("delete-node must undo as create-node") };
    assert_eq!(recreate.index, 1, "the undo must re-insert at the ORIGINAL sibling index");
    assert!(recreate.parent.path.is_empty(), "and under the ORIGINAL parent — here the layer root");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-node applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-node applies");
    }
    assert_eq!(current, base, "delete-node/removes-the-text-node-from-the-layer-root: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteNode":{"at":{"layer":0,"path":[1]}}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-node/removes-the-text-node-from-the-layer-root: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-node mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-node mutation reparses");
    assert_eq!(reencoded, original, "delete-node/removes-the-text-node-from-the-layer-root: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the path resolves to a real child of a real group parent, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-node/removes-the-text-node-from-the-layer-root: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing child must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `layers.modified` entry lowering into `root -> Group.children.removed`, carrying the index and no content.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-node/removes-the-text-node-from-the-layer-root: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed delete-node diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(children.removed, vec![1usize], "the removal is recorded by sibling position");
    assert!(children.modified.is_empty() && children.added.is_empty(), "a removal neither modifies nor adds a sibling");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-node/removes-the-text-node-from-the-layer-root: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed delete-node diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-node diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-node/removes-the-text-node-from-the-layer-root: committed diff did not carry before to after");
}
