//! 🧪️ `remove-frame` fixture — `removes-the-leading-frame`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an out-of-range index is Error
//! `mutation.target-missing`; otherwise the diff is a bare `frames.removed[index]` — note it does
//! NOT carry the removed frame's content, which is why the INVERSE has to read it back out of
//! `base`. Removing index 0 is the case that renumbers everything behind it.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/
/// `mutation.target-missing`/`mutation.invariant` decision for `remove-frame` lives in that file, so the
/// fixture asserts against it rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️component.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("remove-frame before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("remove-frame after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("remove-frame mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::RemoveFrame { index } = mutation() else { panic!("remove-frame/removes-the-leading-frame: the committed mutation must be the remove-frame variant") };
    leaf_diff::diff(&before(), index)
}

/// ▶️ The leading frame goes and the second frame becomes index 0.
#[semio_framework_async_macros::async_test]
async fn removes_the_frame_at_index_zero() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("remove-frame applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-frame/removes-the-leading-frame: applied state differs from the committed after-snapshot");
    assert_eq!(produced.frames.len(), base.frames.len() - 1, "remove-frame shortens the frame sequence by exactly one");
    assert_eq!(produced.frames[0], base.frames[1], "the frame that followed the removed head must take index 0");
    assert_eq!(produced.metadata, base.metadata, "removing a frame must not touch the metadata");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo re-inserts the frame the diff never carried, read back out of `base`.
#[semio_framework_async_macros::async_test]
async fn the_undo_insert_frame_restores_the_head_frame_from_base() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::InsertFrame { index: 0, frame: base.frames[0].clone() }], "the undo must recapture the removed frame's full content from base");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "remove-frame/removes-the-leading-frame: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"removeFrame","index":0}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-frame/removes-the-leading-frame: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("remove-frame mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("remove-frame mutation reparses");
    assert_eq!(reencoded, original, "remove-frame/removes-the-leading-frame: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: index 0 exists in a two-frame base, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-frame/removes-the-leading-frame: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "index 0 exists in a two-frame base, so mutation.target-missing must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Only `removed` may be present inside the triple — a remove diff carries no content.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-frame/removes-the-leading-frame: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point and touches only the `frames` slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_touches_only_frames() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed remove-frame diff decodes");
    assert!(decoded.frames.is_some(), "remove-frame must write the frames slot");
    assert!(decoded.width.is_none() && decoded.height.is_none() && decoded.colorspace.is_none() && decoded.bit_depth.is_none() && decoded.icc.is_none() && decoded.metadata.is_none(), "remove-frame must touch no scalar field and no metadata");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-frame/removes-the-leading-frame: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed remove-frame diff decodes");
    let produced = decoded.apply(&before()).expect("committed remove-frame diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-frame/removes-the-leading-frame: committed diff did not carry before to after");
}
