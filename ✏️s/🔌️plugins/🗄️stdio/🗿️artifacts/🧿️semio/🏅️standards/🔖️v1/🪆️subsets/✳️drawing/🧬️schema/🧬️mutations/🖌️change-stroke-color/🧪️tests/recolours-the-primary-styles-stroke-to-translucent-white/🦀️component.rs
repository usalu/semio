//! 🧪️ `change-stroke-color` fixture — `recolours-the-primary-styles-stroke-to-translucent-white`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: the same three guards as `replace-fill` —
//! unknown name ⇒ Error `mutation.target-missing`, non-finite channel ⇒ FATAL `mutation.invariant`,
//! unchanged colour ⇒ Warning `mutation.no-op` — but writing the `stroke` slot instead. Stroke
//! colour and stroke WIDTH are two separate triads, so this diff must not mention `strokeWidth`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioDrawingSnapshot {
    serde_json::from_str(BEFORE).expect("change-stroke-color before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("change-stroke-color after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("change-stroke-color mutation decodes")
}

/// ▶️ Only the stroke colour changes; the fill, the width and every node stay put.
#[semio_framework_async_macros::async_test]
async fn recolours_the_stroke_without_rethickening_it() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-stroke-color applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-stroke-color/recolours-the-primary-styles-stroke-to-translucent-white: applied state differs from the committed after-snapshot");
    let stroke = produced.styles[0].stroke.expect("the style still has a stroke colour");
    assert_eq!((stroke.r, stroke.a), (1.0, 0.5), "the stroke takes the payload's own colour, alpha included");
    assert_eq!(produced.styles[0].stroke_width, base.styles[0].stroke_width, "recolouring must NOT change the stroke width — that is a separate triad");
    assert_eq!(produced.styles[0].fill, base.styles[0].fill, "recolouring the stroke must not touch the fill");
    assert_eq!(produced.layers, base.layers, "a style edit must not rewrite a single node");
}

/// ↩️ The undo is a `change-stroke-color` carrying BASE's captured colour.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_stroke_color_restores_the_opaque_black() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-stroke-color of an existing style undoes as exactly one change-stroke-color");
    let SemioDrawingMutation::ChangeStrokeColor(restore) = &undo[0] else { panic!("change-stroke-color must undo as change-stroke-color") };
    assert_eq!(restore.new_color, base.styles[0].stroke, "the undo must recapture BASE's own stroke colour, Option and all");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-stroke-color applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-stroke-color applies");
    }
    assert_eq!(current, base, "change-stroke-color/recolours-the-primary-styles-stroke-to-translucent-white: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `SemioRgba` channels are `f32` and 0.5 is exactly representable.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-stroke-color/recolours-the-primary-styles-stroke-to-translucent-white: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-stroke-color mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-stroke-color mutation reparses");
    assert_eq!(reencoded, original, "change-stroke-color/recolours-the-primary-styles-stroke-to-translucent-white: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the style exists, every channel is finite and the colour genuinely differs, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-stroke-color/recolours-the-primary-styles-stroke-to-translucent-white: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "recolouring a stroke to a finite, genuinely different colour must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One name-keyed `styles.modified` entry carrying `stroke` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-stroke-color/recolours-the-primary-styles-stroke-to-translucent-white: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed change-stroke-color diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let styles = decoded.styles.as_ref().expect("the styles triple must be present");
    assert!(styles.removed.is_empty() && styles.added.is_empty(), "a style edit is a per-field modification, never a remove-and-re-add");
    assert_eq!(styles.modified.len(), 1, "exactly one style is modified");
    assert_eq!(styles.modified[0].key, "primary", "styles are NAME-keyed, so the modification carries the name");
    let style_diff = &styles.modified[0].diff;
    assert!(decoded.layers.is_none(), "a style edit must not touch a single layer or node");
    assert!(matches!(style_diff.stroke, Some(Some(_))), "the stroke slot must decode as Some(Some(colour)) — set, not cleared");
    assert!(style_diff.fill.is_none() && style_diff.stroke_width.is_none() && style_diff.opacity.is_none(), "no other style field may be written");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-stroke-color/recolours-the-primary-styles-stroke-to-translucent-white: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed change-stroke-color diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-stroke-color diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-stroke-color/recolours-the-primary-styles-stroke-to-translucent-white: committed diff did not carry before to after");
}
