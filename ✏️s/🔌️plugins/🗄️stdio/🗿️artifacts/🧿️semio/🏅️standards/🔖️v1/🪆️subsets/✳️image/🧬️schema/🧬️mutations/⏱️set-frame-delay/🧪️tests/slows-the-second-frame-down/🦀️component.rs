//! 🧪️ `set-frame-delay` fixture — `slows-the-second-frame-down`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an out-of-range index is Error
//! `mutation.target-missing`. The enum arm then builds a `frames.modified` entry whose per-frame
//! diff sets `delay_ms` and leaves `rgba8` at `None` — that half-populated `SemioImageFrameDiff` is
//! the whole point: a frame is a STRONG entity, diffed per field, not replaced wholesale.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/
/// `mutation.target-missing`/`mutation.invariant` decision for `set-frame-delay` lives in that file, so the
/// fixture asserts against it rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️component.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("set-frame-delay before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("set-frame-delay after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("set-frame-delay mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::SetFrameDelay { index, delay_ms } = mutation() else { panic!("set-frame-delay/slows-the-second-frame-down: the committed mutation must be the set-frame-delay variant") };
    leaf_diff::diff(&before(), index, delay_ms)
}

/// ▶️ Only the second frame's delay changes; its pixels and the first frame are untouched.
#[semio_framework_async_macros::async_test]
async fn changes_only_the_second_frames_delay() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("set-frame-delay applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-frame-delay/slows-the-second-frame-down: applied state differs from the committed after-snapshot");
    assert_eq!(produced.frames[1].delay_ms, 200, "the addressed frame's delay must become the payload's value");
    assert_eq!(produced.frames[1].rgba8, base.frames[1].rgba8, "a delay change must not rewrite a single pixel");
    assert_eq!(produced.frames[0], base.frames[0], "the untargeted frame must be byte-identical");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is a `set-frame-delay` carrying BASE's captured delay for that frame.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_frame_delay_restores_the_captured_delay() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::SetFrameDelay { index: 1, delay_ms: base.frames[1].delay_ms }], "the undo must carry BASE's own delay for that frame");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "set-frame-delay/slows-the-second-frame-down: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"setFrameDelay","index":1,"delay_ms":200}` payload are canonical — the payload field is snake_case (no `rename_all_fields` on the enum) while the frame DIFF spells it `delayMs`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-frame-delay/slows-the-second-frame-down: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-frame-delay mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-frame-delay mutation reparses");
    assert_eq!(reencoded, original, "set-frame-delay/slows-the-second-frame-down: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: index 1 exists in a two-frame base, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-frame-delay/slows-the-second-frame-down: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "index 1 exists in a two-frame base, so mutation.target-missing must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Only `modified` may be present, and its per-frame diff must carry `delayMs` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-frame-delay/slows-the-second-frame-down: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point and is scoped as narrowly as the leaf builds it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-frame-delay diff decodes");
    
    let frames = decoded.frames.as_ref().expect("set-frame-delay must write the frames slot");
    assert!(frames.removed.is_empty() && frames.added.is_empty(), "a delay change adds and removes no frame");
    assert_eq!(frames.modified.len(), 1, "exactly one frame is modified");
    assert!(frames.modified[0].diff.rgba8.is_none(), "the per-frame diff must leave rgba8 unset — that is what makes it a per-FIELD diff");
    assert!(decoded.metadata.is_none() && decoded.width.is_none(), "no other slot may be touched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-frame-delay/slows-the-second-frame-down: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-frame-delay diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-frame-delay diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-frame-delay/slows-the-second-frame-down: committed diff did not carry before to after");
}
