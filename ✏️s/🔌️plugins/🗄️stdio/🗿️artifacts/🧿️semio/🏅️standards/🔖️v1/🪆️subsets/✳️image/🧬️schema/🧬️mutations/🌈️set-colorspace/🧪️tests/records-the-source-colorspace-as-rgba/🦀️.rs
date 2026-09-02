//! 🧪️ `set-colorspace` fixture — `records-the-source-colorspace-as-rgba`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a colorspace already equal to `base`'s is
//! Warning `mutation.no-op`, otherwise the enum arm emits `colorspace` and nothing else. The field
//! records the SOURCE colorspace only — frame buffers are always normalized RGBA8 — so this
//! mutation must never rewrite a single pixel, which is the load-bearing claim below.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_colorspace;

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/
/// `mutation.target-missing`/`mutation.invariant` decision for `set-colorspace` lives in that file, so the
/// fixture asserts against it rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("set-colorspace before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("set-colorspace after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("set-colorspace mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace }) = mutation() else { panic!("set-colorspace/records-the-source-colorspace-as-rgba: the committed mutation must be the set-colorspace variant") };
    leaf_diff::diff(&before(), colorspace)
}

/// ▶️ The recorded source colorspace flips to `Rgba`; the normalized pixel buffers are untouched.
#[semio_framework_async_macros::async_test]
async fn records_rgba_without_touching_a_single_pixel() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("set-colorspace applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-colorspace/records-the-source-colorspace-as-rgba: applied state differs from the committed after-snapshot");
    assert_eq!(produced.colorspace, crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioColorspace::Rgba, "the recorded source colorspace must become the payload's value");
    assert_eq!(produced.frames, base.frames, "set-colorspace records provenance only — it must never rewrite a frame's RGBA8 buffer");
    assert_eq!(produced.bit_depth, base.bit_depth, "set-colorspace must not touch the recorded bit depth");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is a `set-colorspace` back to BASE's own recorded colorspace.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_colorspace_restores_the_recorded_rgb() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo.len(), 1, "set-colorspace undoes as exactly one set-colorspace");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "set-colorspace/records-the-source-colorspace-as-rgba: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"setColorspace","colorspace":"rgba"}` payload are canonical — `SemioColorspace` is `rename_all = "camelCase"`, so `GrayscaleAlpha` would encode as `"grayscaleAlpha"`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-colorspace/records-the-source-colorspace-as-rgba: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-colorspace mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-colorspace mutation reparses");
    assert_eq!(reencoded, original, "set-colorspace/records-the-source-colorspace-as-rgba: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the new colorspace genuinely differs from the base, so mutation.no-op must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-colorspace/records-the-source-colorspace-as-rgba: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "the new colorspace genuinely differs from the base, so mutation.no-op must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Exactly one scalar key.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-colorspace/records-the-source-colorspace-as-rgba: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-colorspace diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-colorspace diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-colorspace/records-the-source-colorspace-as-rgba: committed diff did not carry before to after");
}

/// 🔣️ The committed diff is canonical and carries only the `colorspace` slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_carries_only_colorspace() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-colorspace diff decodes");
    assert!(decoded.colorspace.is_some(), "the colorspace slot must be written");
    assert!(decoded.width.is_none() && decoded.height.is_none() && decoded.bit_depth.is_none() && decoded.icc.is_none() && decoded.frames.is_none() && decoded.metadata.is_none(), "no other slot may be touched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-colorspace/records-the-source-colorspace-as-rgba: committed diff JSON is not canonical");
}
