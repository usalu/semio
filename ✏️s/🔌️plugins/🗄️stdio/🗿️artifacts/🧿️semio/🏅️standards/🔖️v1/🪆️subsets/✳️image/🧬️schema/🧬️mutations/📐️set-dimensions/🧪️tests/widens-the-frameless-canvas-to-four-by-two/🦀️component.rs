//! 🧪️ `set-dimensions` fixture — `widens-the-frameless-canvas-to-four-by-two`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: BOTH `width` and `height` already equal is
//! Warning `mutation.no-op`; there is deliberately NO invariant check, because `0x0` is
//! `SemioImageSnapshot::default()`'s resting state. The enum arm then emits each field only when it
//! actually differs, so a width-only change would emit a width-only diff. The before-snapshot is
//! frameless on purpose: `set-dimensions` does not resize pixel buffers, so a fixture with frames
//! would encode an inconsistent `width*height*4` after-state.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_dimensions;

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: `📦️glue.rs` mounts only
/// `📸️set-snapshot`'s triad for this subset, and the enum-level `Mutation::diff` deliberately
/// carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/`mutation.target-missing`/
/// `mutation.invariant` decision for `set-dimensions` lives in that file, so the fixture asserts against it
/// rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️component.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("set-dimensions before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("set-dimensions after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("set-dimensions mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width, height }) = mutation() else { panic!("set-dimensions/widens-the-frameless-canvas-to-four-by-two: the committed mutation must be the set-dimensions variant") };
    leaf_diff::diff(&before(), width, height)
}

/// ▶️ Both dimensions change; colorspace, bit depth and metadata ride along untouched.
#[semio_framework_async_macros::async_test]
async fn widens_and_heightens_the_canvas() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("set-dimensions applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-dimensions/widens-the-frameless-canvas-to-four-by-two: applied state differs from the committed after-snapshot");
    assert_eq!((produced.width, produced.height), (4, 2), "both dimensions must take the payload's absolute values");
    assert_eq!(produced.colorspace, base.colorspace, "set-dimensions must not touch the recorded source colorspace");
    assert_eq!(produced.metadata, base.metadata, "set-dimensions must not touch the metadata");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is a `set-dimensions` back to BASE's own width and height.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_dimensions_restores_the_original_canvas() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: 2, height: 1 })], "the undo must carry BASE's own dimensions, not a delta");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "set-dimensions/widens-the-frameless-canvas-to-four-by-two: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"setDimensions","width":4,"height":2}` payload are canonical — the enum is internally tagged on `mutation` with camelCase VARIANT names.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-dimensions/widens-the-frameless-canvas-to-four-by-two: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-dimensions mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-dimensions mutation reparses");
    assert_eq!(reencoded, original, "set-dimensions/widens-the-frameless-canvas-to-four-by-two: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: both dimensions genuinely differ from the base, so mutation.no-op must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-dimensions/widens-the-frameless-canvas-to-four-by-two: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "both dimensions genuinely differ from the base, so mutation.no-op must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Only the two scalar keys may appear — no frames, no metadata.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-dimensions/widens-the-frameless-canvas-to-four-by-two: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-dimensions diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-dimensions diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-dimensions/widens-the-frameless-canvas-to-four-by-two: committed diff did not carry before to after");
}

/// 🔣️ The committed diff is canonical and carries exactly the two scalar slots.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_carries_only_width_and_height() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-dimensions diff decodes");
    assert_eq!((decoded.width, decoded.height), (Some(4), Some(2)), "both scalar slots must be written");
    assert!(decoded.frames.is_none() && decoded.metadata.is_none() && decoded.icc.is_none(), "set-dimensions must touch no collection and no ICC profile");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-dimensions/widens-the-frameless-canvas-to-four-by-two: committed diff JSON is not canonical");
}
