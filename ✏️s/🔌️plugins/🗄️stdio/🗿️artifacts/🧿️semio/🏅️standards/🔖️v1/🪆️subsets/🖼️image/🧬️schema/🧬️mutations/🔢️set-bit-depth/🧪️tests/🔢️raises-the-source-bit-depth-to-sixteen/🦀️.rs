//! 🧪️ `set-bit-depth` fixture — `🔢️raises-the-source-bit-depth-to-sixteen`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a bit depth already equal to `base`'s is
//! Warning `mutation.no-op`, otherwise the enum arm emits `bitDepth` and nothing else. NOTE the
//! wire asymmetry this case pins: the DIFF field is `bitDepth` (its struct carries
//! `rename_all = "camelCase"`) while the MUTATION payload field stays `bit_depth` — the mutation
//! enum declares `rename_all` (variants) but not `rename_all_fields`.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_bit_depth;

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/
/// `mutation.target-missing`/`mutation.invariant` decision for `set-bit-depth` lives in that file, so the
/// fixture asserts against it rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("set-bit-depth before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("set-bit-depth after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("set-bit-depth mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth }) = mutation() else { panic!("set-bit-depth/raises-the-source-bit-depth-to-sixteen: the committed mutation must be the set-bit-depth variant") };
    leaf_diff::diff(&before(), bit_depth)
}

/// ▶️ The recorded source bit depth doubles; pixels and colorspace are untouched.
#[semio_framework_async_macros::async_test]
async fn raises_the_recorded_bit_depth_only() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("set-bit-depth applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-bit-depth/raises-the-source-bit-depth-to-sixteen: applied state differs from the committed after-snapshot");
    assert_eq!(produced.bit_depth, 16, "the recorded bit depth must become the payload's value");
    assert_eq!(produced.frames, base.frames, "set-bit-depth records provenance only — frame buffers stay normalized RGBA8");
    assert_eq!(produced.colorspace, base.colorspace, "set-bit-depth must not touch the recorded colorspace");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is a `set-bit-depth` back to BASE's own recorded depth.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_bit_depth_restores_the_original_eight() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: 8 })], "the undo must carry BASE's own bit depth");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "set-bit-depth/raises-the-source-bit-depth-to-sixteen: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"setBitDepth","bit_depth":16}` payload are canonical — the payload field is snake_case on the wire because the enum has no `rename_all_fields`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-bit-depth/raises-the-source-bit-depth-to-sixteen: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-bit-depth mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-bit-depth mutation reparses");
    assert_eq!(reencoded, original, "set-bit-depth/raises-the-source-bit-depth-to-sixteen: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the new bit depth genuinely differs from the base, so mutation.no-op must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-bit-depth/raises-the-source-bit-depth-to-sixteen: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "the new bit depth genuinely differs from the base, so mutation.no-op must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Exactly one scalar key, spelled `bitDepth` on the DIFF side.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-bit-depth/raises-the-source-bit-depth-to-sixteen: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-bit-depth diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-bit-depth diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-bit-depth/raises-the-source-bit-depth-to-sixteen: committed diff did not carry before to after");
}

/// 🔣️ The committed diff is canonical, keyed `bitDepth`, and touches nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_keyed_bit_depth() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-bit-depth diff decodes");
    assert_eq!(decoded.bit_depth, Some(16), "the bit depth slot must be written");
    assert!(decoded.width.is_none() && decoded.height.is_none() && decoded.colorspace.is_none() && decoded.icc.is_none() && decoded.frames.is_none() && decoded.metadata.is_none(), "no other slot may be touched");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("bitDepth").is_some(), "the DIFF spells the field camelCase, unlike the mutation payload");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "set-bit-depth/raises-the-source-bit-depth-to-sixteen: committed diff JSON is not canonical");
}
