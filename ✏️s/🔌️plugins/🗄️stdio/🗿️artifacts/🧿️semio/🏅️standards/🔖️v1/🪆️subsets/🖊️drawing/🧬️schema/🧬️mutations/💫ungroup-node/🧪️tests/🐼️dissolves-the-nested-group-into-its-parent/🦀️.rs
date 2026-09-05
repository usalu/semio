//! 🧪️ `ungroup` fixture — `🐼️dissolves-the-nested-group-into-its-parent`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a path with no parent (a layer root) is Error
//! `mutation.target-missing`, and so is a node that is not a `Group`. Otherwise the diff removes
//! the group index and adds each child back at `group_index + i` — the children's ORDER is
//! preserved by those explicit ascending indices, and the group's own transform is DROPPED, which
//! is why the inverse has to recapture it.

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
    serde_json::from_str(BEFORE).expect("ungroup before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("ungroup after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("ungroup mutation decodes")
}

/// ▶️ The group vanishes and its two children take its place, in order.
#[semio_framework_async_macros::async_test]
async fn splices_the_groups_children_into_the_parent_in_order() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("ungroup applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "ungroup/dissolves-the-nested-group-into-its-parent: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { children: base_children, .. } = &base.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { children: inner, .. } = &base_children[2] else { panic!("child #2 was the nested group") };
    assert_eq!(children.len(), base_children.len() - 1 + inner.len(), "one group is replaced by exactly its children");
    assert_eq!((children[2].clone(), children[3].clone()), (inner[0].clone(), inner[1].clone()), "the children keep their relative order at the group's old position");
    assert_eq!(children[0], base_children[0], "the siblings before the group are untouched");
}

/// ↩️ The undo re-groups exactly the spliced range AND recaptures the dissolved group's transform.
#[semio_framework_async_macros::async_test]
async fn the_undo_group_recaptures_the_range_and_the_transform() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "ungroup undoes as exactly one group");
    let SemioDrawingMutation::GroupNodes(regroup) = &undo[0] else { panic!("ungroup must undo as group") };
    assert_eq!(regroup.indices, vec![2usize, 3], "the undo re-groups exactly the contiguous range the children were spliced into");
    assert_eq!(regroup.transform.scale.x, 1.0, "and carries the dissolved group's own transform, which the forward diff discarded");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward ungroup applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo group applies");
    }
    assert_eq!(current, base, "ungroup/dissolves-the-nested-group-into-its-parent: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"Ungroup":{"at":{"layer":0,"path":[2]}}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "ungroup/dissolves-the-nested-group-into-its-parent: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("ungroup mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("ungroup mutation reparses");
    assert_eq!(reencoded, original, "ungroup/dissolves-the-nested-group-into-its-parent: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the path resolves to a real group with a real parent, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "ungroup/dissolves-the-nested-group-into-its-parent: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "dissolving a real group under a real parent must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children` triple that removes the group index and adds ONE entry per child, at explicit ascending indices.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "ungroup/dissolves-the-nested-group-into-its-parent: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed ungroup diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(children.removed, vec![2usize], "the group leaves its position");
    assert_eq!(children.added.len(), 2, "and each of its children is added back individually");
    assert_eq!((children.added[0].index, children.added[1].index), (2, 3), "at ascending indices starting where the group was — that is what preserves order");
    assert!(children.modified.is_empty(), "ungrouping modifies no sibling in place");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "ungroup/dissolves-the-nested-group-into-its-parent: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed ungroup diff decodes");
    let produced = decoded.apply(&before()).expect("committed ungroup diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "ungroup/dissolves-the-nested-group-into-its-parent: committed diff did not carry before to after");
}
