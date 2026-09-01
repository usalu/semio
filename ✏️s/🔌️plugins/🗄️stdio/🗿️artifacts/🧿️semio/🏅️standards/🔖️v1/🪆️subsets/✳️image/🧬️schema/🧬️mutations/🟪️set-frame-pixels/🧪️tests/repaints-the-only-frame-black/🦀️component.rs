//! 🧪️ `set-frame-pixels` fixture — `repaints-the-only-frame-black`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`, the strictest guard pair in this subset: an
//! out-of-range index is Error `mutation.target-missing`, AND a buffer whose length is not exactly
//! `width * height * 4` is FATAL `mutation.invariant`. The 2x1 before-snapshot therefore takes an
//! 8-byte buffer and nothing else. The per-frame diff sets `rgba8` and leaves `delay_ms` at `None`
//! — the mirror image of `set-frame-delay`.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_frame_pixels;

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/
/// `mutation.target-missing`/`mutation.invariant` decision for `set-frame-pixels` lives in that file, so the
/// fixture asserts against it rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️component.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("set-frame-pixels before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("set-frame-pixels after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("set-frame-pixels mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, rgba8 }) = mutation() else { panic!("set-frame-pixels/repaints-the-only-frame-black: the committed mutation must be the set-frame-pixels variant") };
    leaf_diff::diff(&before(), index, rgba8)
}

/// ▶️ Only the pixel buffer changes; the frame's animation delay survives.
#[semio_framework_async_macros::async_test]
async fn repaints_the_frame_without_changing_its_delay() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("set-frame-pixels applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-frame-pixels/repaints-the-only-frame-black: applied state differs from the committed after-snapshot");
    assert_eq!(produced.frames[0].rgba8.len(), produced.width as usize * produced.height as usize * 4, "the new buffer must satisfy the width*height*4 invariant the leaf enforces");
    assert_ne!(produced.frames[0].rgba8, base.frames[0].rgba8, "the buffer really must have changed");
    assert_eq!(produced.frames[0].delay_ms, base.frames[0].delay_ms, "repainting must not disturb the frame's animation delay");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is a `set-frame-pixels` carrying BASE's captured buffer, byte for byte.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_frame_pixels_restores_the_captured_buffer() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 0, rgba8: base.frames[0].rgba8.clone() })], "the undo must recapture BASE's own pixel buffer");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "set-frame-pixels/repaints-the-only-frame-black: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"setFramePixels","index":0,"rgba8":[…]}` payload are canonical — pixel bytes travel as a plain JSON number array, never base64.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-frame-pixels/repaints-the-only-frame-black: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-frame-pixels mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-frame-pixels mutation reparses");
    assert_eq!(reencoded, original, "set-frame-pixels/repaints-the-only-frame-black: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: index 0 exists and the buffer is exactly width*height*4 bytes, so neither target-missing nor the FATAL invariant may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-frame-pixels/repaints-the-only-frame-black: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "index 0 exists and the buffer is exactly width*height*4 bytes, so neither target-missing nor the FATAL invariant may fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Only `modified` may be present, and its per-frame diff must carry `rgba8` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-frame-pixels/repaints-the-only-frame-black: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point and is scoped as narrowly as the leaf builds it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-frame-pixels diff decodes");

    let frames = decoded.frames.as_ref().expect("set-frame-pixels must write the frames slot");
    assert!(frames.removed.is_empty() && frames.added.is_empty(), "a repaint adds and removes no frame");
    assert_eq!(frames.modified.len(), 1, "exactly one frame is modified");
    assert!(frames.modified[0].diff.delay_ms.is_none(), "the per-frame diff must leave delayMs unset — the mirror image of set-frame-delay");
    assert!(decoded.metadata.is_none() && decoded.width.is_none(), "no other slot may be touched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-frame-pixels/repaints-the-only-frame-black: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-frame-pixels diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-frame-pixels diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-frame-pixels/repaints-the-only-frame-black: committed diff did not carry before to after");
}
