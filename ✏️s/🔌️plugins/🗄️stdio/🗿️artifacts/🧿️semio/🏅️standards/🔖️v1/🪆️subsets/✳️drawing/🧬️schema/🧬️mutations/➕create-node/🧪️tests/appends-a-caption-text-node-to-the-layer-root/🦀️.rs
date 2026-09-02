//! 🧪️ `create-node` fixture — `appends-a-caption-text-node-to-the-layer-root`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a `parent` that does not resolve to a `Group`
//! is FATAL `mutation.invariant` (there is no separate target-missing arm — an unresolvable path
//! and a non-group parent share it); otherwise the node lands at `min(index, children.len())`. The
//! diff is lowered through `diff_at_path`, so it nests `layers.modified -> root -> Group.children`
//! — the empty parent path means exactly one level of nesting here.

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
    serde_json::from_str(BEFORE).expect("create-node before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("create-node after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("create-node mutation decodes")
}

/// ▶️ The caption lands as the fourth child of the layer root; nothing else moves.
#[semio_framework_async_macros::async_test]
async fn appends_the_caption_as_the_fourth_child() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-node applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-node/appends-a-caption-text-node-to-the-layer-root: applied state differs from the committed after-snapshot");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    assert_eq!(children.len(), 4, "create-node adds exactly one child");
    assert!(matches!(children[3], crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode::Text { .. }), "the payload's own node kind lands");
    assert_eq!(produced.styles, base.styles, "creating a node that references a style must not create the style");
}

/// ↩️ The undo is a `delete-node` addressing the FULL path the node landed at.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_node_addresses_the_path_the_node_landed_at() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-node undoes as exactly one delete-node");
    let SemioDrawingMutation::DeleteNode(remove) = &undo[0] else { panic!("create-node must undo as delete-node") };
    assert_eq!(remove.at.path, vec![3usize], "the undo path is the parent path plus the clamped landing index");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-node applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-node applies");
    }
    assert_eq!(current, base, "create-node/appends-a-caption-text-node-to-the-layer-root: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `NodePath` encodes as `{"layer":…,"path":[…]}` and the embedded `DrawNode` keeps its `kind` tag.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/appends-a-caption-text-node-to-the-layer-root: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-node mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-node mutation reparses");
    assert_eq!(reencoded, original, "create-node/appends-a-caption-text-node-to-the-layer-root: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the parent path resolves to the layer root group, so the FATAL mutation.invariant branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-node/appends-a-caption-text-node-to-the-layer-root: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a node under a real group parent must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `layers.modified` entry lowering into `root -> Group.children.added`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-node/appends-a-caption-text-node-to-the-layer-root: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed create-node diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(children.added.len(), 1, "exactly one child is added");
    assert_eq!(children.added[0].index, 3, "the add carries its target position among the siblings");
    assert!(children.removed.is_empty() && children.modified.is_empty(), "a create neither removes nor modifies a sibling");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-node/appends-a-caption-text-node-to-the-layer-root: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed create-node diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-node diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-node/appends-a-caption-text-node-to-the-layer-root: committed diff did not carry before to after");
}
