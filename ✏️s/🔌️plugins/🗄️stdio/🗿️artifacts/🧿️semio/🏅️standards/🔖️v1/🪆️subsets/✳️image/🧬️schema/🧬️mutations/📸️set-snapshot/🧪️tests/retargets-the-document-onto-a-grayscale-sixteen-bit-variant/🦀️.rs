//! 🧪️ `set-snapshot` fixture — `retargets-the-document-onto-a-grayscale-sixteen-bit-variant`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs` and the `diff_set_snapshot` →
//! `DiffAlgebra::between` chain it delegates to. `set-snapshot` is NOT a full-replace slot: it
//! computes a SPARSE field-by-field delta, so a target that keeps `width`/`height`/`icc` unchanged
//! must produce a diff that does not mention them at all. That is the whole reason this case's
//! target deliberately changes only some fields — a full-replace implementation would still reach
//! the same after-snapshot and only the diff would catch it.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_snapshot;

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.target-missing`
/// decision for `set-snapshot` lives in that file, so the fixture asserts against it rather than against
/// the guardless enum arm.
#[path = "../../🔺️diff/🦀️.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("set-snapshot before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("set-snapshot after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) = mutation() else { panic!("set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: the committed mutation must be the set-snapshot variant") };
    leaf_diff::diff(&before(), &snapshot)
}

/// ▶️ The document takes the target's colorspace, bit depth, frame and extra metadata entry.
#[semio_framework_async_macros::async_test]
async fn retargets_the_document_onto_the_committed_variant() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("set-snapshot applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: applied state differs from the committed after-snapshot");
    assert_eq!((produced.width, produced.height), (base.width, base.height), "the target keeps the base dimensions — so the diff must not mention them");
    assert_eq!(produced.bit_depth, 16, "the target's bit depth must land");
    assert_eq!(produced.metadata.len(), base.metadata.len() + 1, "the target adds one metadata entry");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is a `set-snapshot` carrying BASE itself — the only inverse a whole-document
/// retarget can have.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_snapshot_carries_the_whole_base_document() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })], "the undo must carry BASE verbatim");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"setSnapshot","snapshot":{…}}` payload are canonical — the payload embeds a whole `SemioImageSnapshot`, so its inner `bitDepth`/`delayMs` are camelCase even though the mutation's own fields are not.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the target genuinely differs from the base, so mutation.no-op must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "the target genuinely differs from the base, so mutation.no-op must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. The delta must be SPARSE: colorspace, bitDepth, one modified frame and one added metadata entry — and nothing else.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point and is scoped as narrowly as the leaf builds it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-snapshot diff decodes");

    assert!(decoded.width.is_none() && decoded.height.is_none() && decoded.icc.is_none(), "fields the target left alone must be absent — this is the assertion a full-replace implementation would fail");
    assert!(decoded.colorspace.is_some() && decoded.bit_depth.is_some(), "the two changed scalars must be present");
    let frames = decoded.frames.as_ref().expect("the changed frame must be present");
    assert_eq!(frames.modified.len(), 1, "the single frame is MODIFIED per field, not removed and re-added");
    let metadata = decoded.metadata.as_ref().expect("the added metadata entry must be present");
    assert_eq!(metadata.added.len(), 1, "exactly one entry is added");
    assert!(metadata.removed.is_empty() && metadata.modified.is_empty(), "the surviving Author entry is unchanged, so it must not appear at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-snapshot diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-snapshot diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/retargets-the-document-onto-a-grayscale-sixteen-bit-variant: committed diff did not carry before to after");
}
