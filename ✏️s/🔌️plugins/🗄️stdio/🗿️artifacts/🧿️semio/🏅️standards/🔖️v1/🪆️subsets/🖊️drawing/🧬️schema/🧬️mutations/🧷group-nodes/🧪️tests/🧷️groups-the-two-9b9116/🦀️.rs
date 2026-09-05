//! 🧪️ `group` fixture — `🧷️groups-the-two-leading-children-into-a-new-group`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: the indices must be a NON-EMPTY, CONTIGUOUS,
//! ASCENDING run or the outcome is Error `mutation.target-missing` (the same arm covers a
//! non-group parent and an out-of-range index). The diff removes every grouped index and adds one
//! new `Group` node back at `indices[0]`, carrying the grouped children inside it — so the node
//! content travels in the diff rather than being reconstructed by the applier.

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
    serde_json::from_str(BEFORE).expect("group before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("group after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("group mutation decodes")
}

/// ▶️ The two leading siblings disappear into a new group that takes their place.
#[semio_framework_async_macros::async_test]
async fn wraps_the_two_leading_children_in_a_new_group() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("group applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "group/groups-the-two-leading-children-into-a-new-group: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { children: base_children, .. } = &base.layers[0].root else { panic!("the layer root is a group") };
    assert_eq!(children.len(), base_children.len() - 1, "two siblings become one, so the root loses exactly one child");
    let DrawNode::Group { children: wrapped, transform } = &children[0] else { panic!("the new group takes the first grouped index") };
    assert_eq!(wrapped.len(), 2, "both grouped nodes moved inside the new group");
    assert_eq!(wrapped[0], base_children[0], "and kept their original order and content");
    assert_eq!(transform.scale.x, 1.0, "the payload's own transform lands on the new group");
    assert_eq!(children[1], base_children[2], "the ungrouped sibling survives, merely shifted");
}

/// ↩️ `group`'s undo is a single `ungroup` addressing the path the new group landed at.
#[semio_framework_async_macros::async_test]
async fn the_undo_ungroup_dissolves_the_new_group_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "group undoes as exactly one ungroup");
    let SemioDrawingMutation::UngroupNode(dissolve) = &undo[0] else { panic!("group must undo as ungroup") };
    assert_eq!(dissolve.at.path, vec![0usize], "the undo path is the parent path plus the FIRST grouped index");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward group applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo ungroup applies");
    }
    assert_eq!(current, base, "group/groups-the-two-leading-children-into-a-new-group: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `indices` is a plain array and the payload transform is a full `SemioTransform`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "group/groups-the-two-leading-children-into-a-new-group: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("group mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("group mutation reparses");
    assert_eq!(reencoded, original, "group/groups-the-two-leading-children-into-a-new-group: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the indices are a contiguous ascending in-range run under a real group parent, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "group/groups-the-two-leading-children-into-a-new-group: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "grouping a contiguous run under a real group parent must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children` triple that removes BOTH grouped indices and adds ONE new group node carrying them.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "group/groups-the-two-leading-children-into-a-new-group: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed group diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(children.removed, vec![0usize, 1], "both grouped indices leave the parent");
    assert_eq!(children.added.len(), 1, "and exactly one new node takes their place");
    assert_eq!(children.added[0].index, 0, "at the first grouped index");
    assert!(children.modified.is_empty(), "grouping modifies no sibling in place");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "group/groups-the-two-leading-children-into-a-new-group: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed group diff decodes");
    let produced = decoded.apply(&before()).expect("committed group diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "group/groups-the-two-leading-children-into-a-new-group: committed diff did not carry before to after");
}
