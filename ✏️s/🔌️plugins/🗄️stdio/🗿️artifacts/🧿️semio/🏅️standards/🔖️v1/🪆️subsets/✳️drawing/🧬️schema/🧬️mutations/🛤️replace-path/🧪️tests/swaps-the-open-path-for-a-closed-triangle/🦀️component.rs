//! 🧪️ `replace-path` fixture — `swaps-the-open-path-for-a-closed-triangle`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: a node that does not exist OR is not a `Path`
//! is Error `mutation.target-missing` (one arm for both — the kind check is part of resolution
//! here); identical segments are Warning `mutation.no-op`. `segments` is a weak value list, so the
//! per-path diff replaces it WHOLE and leaves `style` at `None`: replacing geometry must not
//! silently rebind the style.

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
    serde_json::from_str(BEFORE).expect("replace-path before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("replace-path after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("replace-path mutation decodes")
}

/// ▶️ The path's segment list is replaced wholesale; its style reference survives.
#[semio_framework_async_macros::async_test]
async fn replaces_the_segments_and_keeps_the_style_reference() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("replace-path applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "replace-path/swaps-the-open-path-for-a-closed-triangle: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Path { segments, style } = &children[0] else { panic!("child #0 is the path node") };
    assert_eq!(segments.len(), 4, "the triangle is four segments — the list is replaced whole, not appended to");
    assert_eq!(style.as_deref(), Some("primary"), "replacing segments must not rebind the style");
    assert!(produced.styles.iter().any(|entry| Some(entry.name.as_str()) == style.as_deref()), "and the style it still names must remain in the table");
}

/// ↩️ The undo is a `replace-path` carrying BASE's captured segment list.
#[semio_framework_async_macros::async_test]
async fn the_undo_replace_path_restores_the_captured_segments() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "replace-path of a real path undoes as exactly one replace-path");
    let SemioDrawingMutation::ReplacePath(restore) = &undo[0] else { panic!("replace-path must undo as replace-path") };
    assert_eq!(restore.new_segments.len(), 3, "the undo must recapture BASE's own three segments");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward replace-path applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo replace-path applies");
    }
    assert_eq!(current, base, "replace-path/swaps-the-open-path-for-a-closed-triangle: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `PathSegment` is internally tagged on `kind`, so the unit variant `Close` encodes as the bare object `{"kind":"close"}`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-path/swaps-the-open-path-for-a-closed-triangle: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-path mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-path mutation reparses");
    assert_eq!(reencoded, original, "replace-path/swaps-the-open-path-for-a-closed-triangle: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the node resolves to a real path and the segments genuinely differ, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-path/swaps-the-open-path-for-a-closed-triangle: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "replacing a path with genuinely different segments must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children.modified` entry whose node diff is the PATH arm carrying `segments` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-path/swaps-the-open-path-for-a-closed-triangle: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed replace-path diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Path(path_diff) = &children.modified[0].diff else { panic!("replacing a Path must produce the Path arm, never Replace") };
    assert_eq!(path_diff.segments.as_ref().map(Vec::len), Some(4), "the whole new segment list travels in the diff");
    assert!(path_diff.style.is_none(), "the style reference must stay unwritten");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-path/swaps-the-open-path-for-a-closed-triangle: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed replace-path diff decodes");
    let produced = decoded.apply(&before()).expect("committed replace-path diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-path/swaps-the-open-path-for-a-closed-triangle: committed diff did not carry before to after");
}
