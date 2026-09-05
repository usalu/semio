//! 🧪️ `flatten` fixture — `🪪️flattens-an-identity-nested-group-into-its-leaves`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs` and its `collect_flattened_leaves` helper:
//! every branch except the real one is a WARNING, not an error — a non-group node, an
//! already-flat group, and a group whose descendant carries a NON-identity transform (which cannot
//! be flattened without losing that transform) all warn `mutation.no-op` with an EMPTY diff. Only
//! an unresolvable path is Error `mutation.target-missing`. When it does apply, the node diff is
//! `Replace` — a structural rewrite, not a field patch.

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
    serde_json::from_str(BEFORE).expect("flatten before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("flatten after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("flatten mutation decodes")
}

/// ▶️ The nested identity group dissolves into the parent group's leaf list, order preserved.
#[semio_framework_async_macros::async_test]
async fn hoists_the_nested_identity_groups_leaves() {
    let base = before();
    let DrawNode::Group { children: base_children, .. } = &base.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { children: before_inner, .. } = &base_children[2] else { panic!("child #2 is the group being flattened") };
    assert!(matches!(before_inner[0], DrawNode::Group { .. }), "the fixture needs a nested group for flattening to have anything to do");
    let produced = mutation().diff(&base).diff().apply(&base).expect("flatten applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "flatten/flattens-an-identity-nested-group-into-its-leaves: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { children: inner, transform } = &children[2] else { panic!("child #2 is still a group") };
    assert_eq!(inner.len(), 2, "the nested group is replaced by its single leaf, alongside the existing sibling leaf");
    assert!(!inner.iter().any(|node| matches!(node, DrawNode::Group { .. })), "no group may remain inside a flattened group");
    assert_eq!(transform.scale.x, 1.0, "the flattened group keeps its OWN transform — only descendants are hoisted");
}

/// ↩️ The undo is an `unflatten` carrying the WHOLE captured node — the diff discarded the
/// hierarchy, so nothing less would restore it.
#[semio_framework_async_macros::async_test]
async fn the_undo_unflatten_carries_the_captured_hierarchy() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "flatten undoes as exactly one unflatten");
    let SemioDrawingMutation::UnflattenNode(restore) = &undo[0] else { panic!("flatten must undo as unflatten") };
    let DrawNode::Group { children: original_children, .. } = &restore.original else { panic!("the captured original is the group itself") };
    assert!(matches!(original_children[0], DrawNode::Group { .. }), "the captured original still carries the nested group the flatten dissolved");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward flatten applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo unflatten applies");
    }
    assert_eq!(current, base, "flatten/flattens-an-identity-nested-group-into-its-leaves: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"Flatten":{"at":{"layer":0,"path":[2]}}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "flatten/flattens-an-identity-nested-group-into-its-leaves: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("flatten mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("flatten mutation reparses");
    assert_eq!(reencoded, original, "flatten/flattens-an-identity-nested-group-into-its-leaves: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the path resolves to a group whose descendants all carry identity transforms and is not already flat, so neither the target-missing error nor any of the three no-op warnings may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "flatten/flattens-an-identity-nested-group-into-its-leaves: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "flattening a genuinely nested, identity-transformed group must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children.modified` entry whose node diff is the REPLACE arm — a structural rewrite carries the whole new node.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "flatten/flattens-an-identity-nested-group-into-its-leaves: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed flatten diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(children.modified.len(), 1, "exactly one child is rewritten");
    assert!(matches!(children.modified[0].diff, crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Replace { .. }), "a hierarchy change goes through Replace, never through the per-field Group arm");
    assert!(children.removed.is_empty() && children.added.is_empty(), "the node keeps its position — only its contents are rewritten");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "flatten/flattens-an-identity-nested-group-into-its-leaves: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed flatten diff decodes");
    let produced = decoded.apply(&before()).expect("committed flatten diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "flatten/flattens-an-identity-nested-group-into-its-leaves: committed diff did not carry before to after");
}
