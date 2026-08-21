//! 🧪️ `scale` fixture — `scales-the-nested-group-non-uniformly`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: the exact mirror of `rotate` — unresolvable
//! path ⇒ Error `mutation.target-missing`, non-finite component ⇒ FATAL `mutation.invariant`,
//! unchanged scale on a `Group` ⇒ Warning `mutation.no-op`. Note the guard is only FINITENESS here,
//! not positivity (unlike `✳️object`'s `scale-object`), so a zero or negative factor would be
//! accepted; this case stays deliberately positive and non-uniform.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioDrawingSnapshot {
    serde_json::from_str(BEFORE).expect("scale before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("scale after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("scale mutation decodes")
}

/// ▶️ The group's scale becomes non-uniform; its rotation, translation and children stay put.
#[semio_framework_async_macros::async_test]
async fn scales_the_group_non_uniformly_and_keeps_everything_else() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("scale applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "scale/scales-the-nested-group-non-uniformly: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { transform, children: nested } = &children[2] else { panic!("child #2 is the nested group") };
    assert_eq!((transform.scale.x, transform.scale.y), (2.0, 0.5), "each axis takes its own factor — this case is deliberately NON-uniform");
    assert_eq!(transform.rotation.w, 1.0, "scaling must not rotate the group");
    assert_eq!(transform.translation.x, 0.0, "scaling must not translate the group");
    assert_eq!(nested.len(), 1, "scaling a group must not touch its children");
}

/// ↩️ The undo is a `scale` carrying BASE's captured unit scale.
#[semio_framework_async_macros::async_test]
async fn the_undo_scale_restores_the_unit_scale() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "scale of a group undoes as exactly one scale");
    let SemioDrawingMutation::Scale(restore) = &undo[0] else { panic!("scale must undo as scale") };
    assert_eq!((restore.new_scale.x, restore.new_scale.y), (1.0, 1.0), "the undo must recapture BASE's own unit scale");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward scale applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo scale applies");
    }
    assert_eq!(current, base, "scale/scales-the-nested-group-non-uniformly: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the scale is a three-axis `SemioPoint3` even in a 2D drawing, and every factor is dyadic.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "scale/scales-the-nested-group-non-uniformly: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("scale mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("scale mutation reparses");
    assert_eq!(reencoded, original, "scale/scales-the-nested-group-non-uniformly: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the path resolves to a group, the factors are finite and genuinely different, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "scale/scales-the-nested-group-non-uniformly: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, genuinely-different scale on a real group must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children.modified` entry whose node diff is the GROUP arm carrying the WHOLE rebuilt transform.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "scale/scales-the-nested-group-non-uniformly: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed scale diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(node_group) = &children.modified[0].diff else { panic!("scaling a Group must produce the Group arm") };
    let transform = node_group.transform.as_ref().expect("the transform must be written");
    assert_eq!(transform.rotation.w, 1.0, "the diff carries the WHOLE transform, so the untouched rotation is BASE's");
    assert!(node_group.children.is_none(), "the group's children triple must stay unwritten");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "scale/scales-the-nested-group-non-uniformly: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed scale diff decodes");
    let produced = decoded.apply(&before()).expect("committed scale diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "scale/scales-the-nested-group-non-uniformly: committed diff did not carry before to after");
}
