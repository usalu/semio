//! 🧪️ `move-frame` fixture — `⏮️moves-the-last-frame-to-the-front`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an out-of-range `from` OR `to` is Error
//! `mutation.target-missing`, `from == to` is Warning `mutation.no-op`. The enum arm then expresses
//! the move as a REMOVED-plus-ADDED pair inside a single `frames` triple, which the shared
//! `apply_indexed` replays in that order (modified, then removed descending, then added ascending)
//! — that ordering is exactly why `2 -> 0` lands the frame first.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::move_frame;

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/
/// `mutation.target-missing`/`mutation.invariant` decision for `move-frame` lives in that file, so the
/// fixture asserts against it rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("move-frame before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("move-frame after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("move-frame mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::MoveFrame(move_frame::MoveFrame { from, to }) = mutation() else { panic!("move-frame/moves-the-last-frame-to-the-front: the committed mutation must be the move-frame variant") };
    leaf_diff::diff(&before(), from, to)
}

/// ▶️ The third frame becomes the first and the other two shift back by one.
#[semio_framework_async_macros::async_test]
async fn moves_frame_two_to_the_head_of_the_sequence() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("move-frame applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "move-frame/moves-the-last-frame-to-the-front: applied state differs from the committed after-snapshot");
    assert_eq!(produced.frames.len(), base.frames.len(), "move-frame is a permutation — it may never add or drop a frame");
    assert_eq!(produced.frames[0], base.frames[2], "the moved frame must sit first afterwards");
    assert_eq!((produced.frames[1].clone(), produced.frames[2].clone()), (base.frames[0].clone(), base.frames[1].clone()), "the frames it jumped over keep their relative order");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is the mirrored `move-frame` — `from`/`to` swapped, no clamping involved.
#[semio_framework_async_macros::async_test]
async fn the_undo_move_frame_swaps_from_and_to() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: 0, to: 2 })], "the undo of a 2 -> 0 move is a 0 -> 2 move");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "move-frame/moves-the-last-frame-to-the-front: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"moveFrame","from":2,"to":0}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-frame/moves-the-last-frame-to-the-front: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("move-frame mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("move-frame mutation reparses");
    assert_eq!(reencoded, original, "move-frame/moves-the-last-frame-to-the-front: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: both indices exist and from differs from to, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-frame/moves-the-last-frame-to-the-front: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "both indices exist and from differs from to, so neither target-missing nor no-op may fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. `removed` AND `added` must both be present — that pair IS the move.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-frame/moves-the-last-frame-to-the-front: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point and touches only the `frames` slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_touches_only_frames() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed move-frame diff decodes");
    assert!(decoded.frames.is_some(), "move-frame must write the frames slot");
    assert!(decoded.width.is_none() && decoded.height.is_none() && decoded.colorspace.is_none() && decoded.bit_depth.is_none() && decoded.icc.is_none() && decoded.metadata.is_none(), "move-frame must touch no scalar field and no metadata");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-frame/moves-the-last-frame-to-the-front: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed move-frame diff decodes");
    let produced = decoded.apply(&before()).expect("committed move-frame diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-frame/moves-the-last-frame-to-the-front: committed diff did not carry before to after");
}
