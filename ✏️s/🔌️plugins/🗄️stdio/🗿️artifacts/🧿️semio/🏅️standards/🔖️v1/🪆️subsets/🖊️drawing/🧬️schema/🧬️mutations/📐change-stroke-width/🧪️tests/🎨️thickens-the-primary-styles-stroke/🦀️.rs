//! 🧪️ `change-stroke-width` fixture — `🎨️thickens-the-primary-styles-stroke`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown name ⇒ Error
//! `mutation.target-missing`, a non-finite width ⇒ FATAL `mutation.invariant`, an unchanged width ⇒
//! Warning `mutation.no-op`. The mirror of `change-stroke-color`: committing both halves is what
//! proves the stroke triads really are decomposed — this diff must mention `strokeWidth` and NOT
//! `stroke`.

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
    serde_json::from_str(BEFORE).expect("change-stroke-width before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("change-stroke-width after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("change-stroke-width mutation decodes")
}

/// ▶️ Only the stroke width changes; the stroke colour, the fill and every node stay put.
#[semio_framework_async_macros::async_test]
async fn thickens_the_stroke_without_recolouring_it() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-stroke-width applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-stroke-width/thickens-the-primary-styles-stroke: applied state differs from the committed after-snapshot");
    assert_eq!(produced.styles[0].stroke_width, Some(2.5), "the width takes the payload's own value");
    assert_eq!(produced.styles[0].stroke, base.styles[0].stroke, "thickening must NOT recolour the stroke — that is a separate triad");
    assert_eq!(produced.styles[0].opacity, base.styles[0].opacity, "nor change the opacity");
    assert_eq!(produced.layers, base.layers, "a style edit must not rewrite a single node");
}

/// ↩️ The undo is a `change-stroke-width` carrying BASE's captured width.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_stroke_width_restores_the_original_width() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-stroke-width of an existing style undoes as exactly one change-stroke-width");
    let SemioDrawingMutation::ChangeStrokeWidth(restore) = &undo[0] else { panic!("change-stroke-width must undo as change-stroke-width") };
    assert_eq!(restore.new_width, base.styles[0].stroke_width, "the undo must recapture BASE's own width, Option and all");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-stroke-width applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-stroke-width applies");
    }
    assert_eq!(current, base, "change-stroke-width/thickens-the-primary-styles-stroke: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ChangeStrokeWidth":{"style_name":"primary","new_width":2.5}}` payload are canonical — the DIFF spells the field `strokeWidth`, the SNAPSHOT too, the PAYLOAD `new_width`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-stroke-width/thickens-the-primary-styles-stroke: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-stroke-width mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-stroke-width mutation reparses");
    assert_eq!(reencoded, original, "change-stroke-width/thickens-the-primary-styles-stroke: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the style exists, the width is finite and genuinely differs, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-stroke-width/thickens-the-primary-styles-stroke: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "thickening to a finite, genuinely different width must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One name-keyed `styles.modified` entry carrying `strokeWidth` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-stroke-width/thickens-the-primary-styles-stroke: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed change-stroke-width diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let styles = decoded.styles.as_ref().expect("the styles triple must be present");
    assert!(styles.removed.is_empty() && styles.added.is_empty(), "a style edit is a per-field modification, never a remove-and-re-add");
    assert_eq!(styles.modified.len(), 1, "exactly one style is modified");
    assert_eq!(styles.modified[0].key, "primary", "styles are NAME-keyed, so the modification carries the name");
    let style_diff = &styles.modified[0].diff;
    assert!(decoded.layers.is_none(), "a style edit must not touch a single layer or node");
    assert!(matches!(style_diff.stroke_width, Some(Some(_))), "the width slot must decode as Some(Some(width)) — set, not cleared");
    assert!(style_diff.fill.is_none() && style_diff.stroke.is_none() && style_diff.opacity.is_none(), "no other style field may be written");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-stroke-width/thickens-the-primary-styles-stroke: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed change-stroke-width diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-stroke-width diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-stroke-width/thickens-the-primary-styles-stroke: committed diff did not carry before to after");
}
