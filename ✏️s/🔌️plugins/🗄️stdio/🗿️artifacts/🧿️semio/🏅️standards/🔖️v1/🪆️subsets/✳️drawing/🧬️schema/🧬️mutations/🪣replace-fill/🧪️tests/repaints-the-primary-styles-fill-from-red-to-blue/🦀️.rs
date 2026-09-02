//! 🧪️ `replace-fill` fixture — `repaints-the-primary-styles-fill-from-red-to-blue`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, guards in order: an unknown style name ⇒ Error
//! `mutation.target-missing`; a non-finite CHANNEL in the new fill ⇒ FATAL `mutation.invariant`; an
//! unchanged fill ⇒ Warning `mutation.no-op`. `styles` is NAME-keyed, and `DrawStyleDiff::fill` is
//! tri-state `Option<Option<SemioRgba>>` — this case takes the SET arm, so its inner value is a
//! real object and the committed diff survives a JSON round trip (the `null`/clear arm would not).

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
    serde_json::from_str(BEFORE).expect("replace-fill before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("replace-fill after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("replace-fill mutation decodes")
}

/// ▶️ Only the fill changes; stroke, width, opacity and every node stay put.
#[semio_framework_async_macros::async_test]
async fn repaints_the_fill_and_leaves_the_rest_of_the_style_alone() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("replace-fill applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "replace-fill/repaints-the-primary-styles-fill-from-red-to-blue: applied state differs from the committed after-snapshot");
    let fill = produced.styles[0].fill.expect("the style still has a fill");
    assert_eq!((fill.r, fill.b), (0.0, 1.0), "the fill takes the payload's own colour");
    assert_eq!(produced.styles[0].stroke, base.styles[0].stroke, "replacing the fill must not touch the stroke colour");
    assert_eq!(produced.styles[0].stroke_width, base.styles[0].stroke_width, "nor the stroke width");
    assert_eq!(produced.layers, base.layers, "a style edit must not rewrite a single node");
}

/// ↩️ The undo is a `replace-fill` carrying BASE's captured fill.
#[semio_framework_async_macros::async_test]
async fn the_undo_replace_fill_restores_the_red() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "replace-fill of an existing style undoes as exactly one replace-fill");
    let SemioDrawingMutation::ReplaceFill(restore) = &undo[0] else { panic!("replace-fill must undo as replace-fill") };
    assert_eq!(restore.new_fill, base.styles[0].fill, "the undo must recapture BASE's own fill, Option and all");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward replace-fill applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo replace-fill applies");
    }
    assert_eq!(current, base, "replace-fill/repaints-the-primary-styles-fill-from-red-to-blue: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `DrawStyle` spells its width `strokeWidth` (camelCase) while the payload key stays `new_fill`, and an absent style field is an OMITTED key, never a null.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-fill/repaints-the-primary-styles-fill-from-red-to-blue: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-fill mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-fill mutation reparses");
    assert_eq!(reencoded, original, "replace-fill/repaints-the-primary-styles-fill-from-red-to-blue: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the style exists, every channel is finite and the colour genuinely differs, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-fill/repaints-the-primary-styles-fill-from-red-to-blue: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "repainting a fill to a finite, genuinely different colour must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One name-keyed `styles.modified` entry carrying `fill` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-fill/repaints-the-primary-styles-fill-from-red-to-blue: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed replace-fill diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let styles = decoded.styles.as_ref().expect("the styles triple must be present");
    assert!(styles.removed.is_empty() && styles.added.is_empty(), "a style edit is a per-field modification, never a remove-and-re-add");
    assert_eq!(styles.modified.len(), 1, "exactly one style is modified");
    assert_eq!(styles.modified[0].key, "primary", "styles are NAME-keyed, so the modification carries the name");
    let style_diff = &styles.modified[0].diff;
    assert!(decoded.layers.is_none(), "a style edit must not touch a single layer or node");
    assert!(matches!(style_diff.fill, Some(Some(_))), "the fill slot must decode as Some(Some(colour)) — set, not cleared");
    assert!(style_diff.stroke.is_none() && style_diff.stroke_width.is_none() && style_diff.opacity.is_none(), "no other style field may be written");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-fill/repaints-the-primary-styles-fill-from-red-to-blue: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed replace-fill diff decodes");
    let produced = decoded.apply(&before()).expect("committed replace-fill diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-fill/repaints-the-primary-styles-fill-from-red-to-blue: committed diff did not carry before to after");
}
