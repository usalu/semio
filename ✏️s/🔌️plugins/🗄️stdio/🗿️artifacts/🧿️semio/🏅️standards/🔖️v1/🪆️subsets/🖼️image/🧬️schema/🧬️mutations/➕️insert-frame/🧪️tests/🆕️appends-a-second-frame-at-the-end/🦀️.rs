//! 🧪️ `insert-frame` fixture — `🆕️appends-a-second-frame-at-the-end`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: the index is CLAMPED to `frames.len()` and a
//! clamp raises Warning `mutation.clamped`. Index 1 against a one-frame base is exactly at the
//! boundary — `min(1, 1) == 1` — so the clamp does not fire and the diff is a pure
//! `frames.added[{index, item}]` triple entry, with `removed`/`modified` omitted because
//! `IndexedTripleDiff` skips empty vectors.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::insert_frame;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::remove_frame;

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/
/// `mutation.target-missing`/`mutation.invariant` decision for `insert-frame` lives in that file, so the
/// fixture asserts against it rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("insert-frame before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("insert-frame after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("insert-frame mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index, frame }) = mutation() else { panic!("insert-frame/appends-a-second-frame-at-the-end: the committed mutation must be the insert-frame variant") };
    leaf_diff::diff(&before(), index, frame)
}

/// ▶️ The second frame lands at index 1, keeping its own delay and pixel buffer.
#[semio_framework_async_macros::async_test]
async fn appends_the_second_frame_with_its_own_delay_and_pixels() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("insert-frame applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "insert-frame/appends-a-second-frame-at-the-end: applied state differs from the committed after-snapshot");
    assert_eq!(produced.frames.len(), base.frames.len() + 1, "insert-frame lengthens the frame sequence by exactly one");
    assert_eq!(produced.frames[0], base.frames[0], "the pre-existing frame must be byte-identical and keep index 0");
    assert_eq!(produced.frames[1].delay_ms, 80, "the inserted frame carries its own animation delay");
    assert_eq!(produced.frames[1].rgba8.len(), produced.width as usize * produced.height as usize * 4, "the inserted frame's buffer must match width*height*4");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is a `remove-frame` at the index the frame landed at.
#[semio_framework_async_macros::async_test]
async fn the_undo_remove_frame_takes_the_second_frame_back_out() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: 1 })], "insert-frame at #1 must undo as remove-frame at #1");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "insert-frame/appends-a-second-frame-at-the-end: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"insertFrame","index":1,"frame":{…}}` payload are canonical — `SemioImageFrame` is `rename_all = "camelCase"`, so the frame's delay is `delayMs` even inside the snake_case-fielded mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-frame/appends-a-second-frame-at-the-end: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("insert-frame mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("insert-frame mutation reparses");
    assert_eq!(reencoded, original, "insert-frame/appends-a-second-frame-at-the-end: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: index 1 is exactly frames.len(), so the clamp is a no-op and mutation.clamped must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "insert-frame/appends-a-second-frame-at-the-end: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "index 1 is exactly frames.len(), so the clamp is a no-op and mutation.clamped must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Only `added` may be present inside the triple.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "insert-frame/appends-a-second-frame-at-the-end: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point and touches only the `frames` slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_touches_only_frames() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed insert-frame diff decodes");
    assert!(decoded.frames.is_some(), "insert-frame must write the frames slot");
    assert!(decoded.width.is_none() && decoded.height.is_none() && decoded.colorspace.is_none() && decoded.bit_depth.is_none() && decoded.icc.is_none() && decoded.metadata.is_none(), "insert-frame must touch no scalar field and no metadata");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "insert-frame/appends-a-second-frame-at-the-end: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed insert-frame diff decodes");
    let produced = decoded.apply(&before()).expect("committed insert-frame diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "insert-frame/appends-a-second-frame-at-the-end: committed diff did not carry before to after");
}
