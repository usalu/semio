//! 🧪️ `rotate` fixture — `🔄️rotates-the-nested-group-a-half-turn-about-z`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unresolvable path ⇒ Error
//! `mutation.target-missing`; a non-finite quaternion component ⇒ FATAL `mutation.invariant`; an
//! unchanged rotation on a `Group` ⇒ Warning `mutation.no-op`. `diff_rotate_node` only produces a
//! diff for a `Group` — `Path`/`Text`/`Image` carry no rotation, so it honestly returns an EMPTY
//! diff for them rather than approximating one. The half turn `(0, 0, 1, 0)` is exactly
//! representable, so the canonical-JSON assertion holds without float slack.

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
    serde_json::from_str(BEFORE).expect("rotate before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("rotate after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("rotate mutation decodes")
}

/// ▶️ The group's rotation flips; its translation, scale and children stay put.
#[semio_framework_async_macros::async_test]
async fn rotates_the_group_and_keeps_translation_scale_and_children() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("rotate applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "rotate/rotates-the-nested-group-a-half-turn-about-z: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { transform, children: nested } = &children[2] else { panic!("child #2 is the nested group") };
    assert_eq!((transform.rotation.z, transform.rotation.w), (1.0, 0.0), "the half turn about Z is the quaternion (0, 0, 1, 0)");
    assert_eq!(transform.translation.x, 0.0, "rotating must not translate the group");
    assert_eq!(transform.scale.x, 1.0, "rotating must not rescale the group");
    assert_eq!(nested.len(), 1, "rotating a group must not touch its children");
}

/// ↩️ The undo is a `rotate` carrying BASE's captured rotation.
#[semio_framework_async_macros::async_test]
async fn the_undo_rotate_restores_the_identity_rotation() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "rotate of a group undoes as exactly one rotate");
    let SemioDrawingMutation::RotateNode(restore) = &undo[0] else { panic!("rotate must undo as rotate") };
    assert_eq!(restore.new_rotation.w, 1.0, "the undo must recapture BASE's own identity rotation");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward rotate applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo rotate applies");
    }
    assert_eq!(current, base, "rotate/rotates-the-nested-group-a-half-turn-about-z: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the quaternion is a NAMED four-field struct, never a bare array.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rotate/rotates-the-nested-group-a-half-turn-about-z: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("rotate mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rotate mutation reparses");
    assert_eq!(reencoded, original, "rotate/rotates-the-nested-group-a-half-turn-about-z: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the path resolves to a group, the quaternion is finite and genuinely different, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rotate/rotates-the-nested-group-a-half-turn-about-z: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, genuinely-different rotation on a real group must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children.modified` entry whose node diff is the GROUP arm carrying the WHOLE rebuilt transform.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rotate/rotates-the-nested-group-a-half-turn-about-z: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed rotate diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(node_group) = &children.modified[0].diff else { panic!("rotating a Group must produce the Group arm") };
    let transform = node_group.transform.as_ref().expect("the transform must be written");
    assert_eq!(transform.scale.x, 1.0, "the diff carries the WHOLE transform, so the untouched scale is BASE's");
    assert!(node_group.children.is_none(), "the group's children triple must stay unwritten");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rotate/rotates-the-nested-group-a-half-turn-about-z: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed rotate diff decodes");
    let produced = decoded.apply(&before()).expect("committed rotate diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rotate/rotates-the-nested-group-a-half-turn-about-z: committed diff did not carry before to after");
}
