//! 🧪️ `update-camera-calibration` fixture — `refines-the-cam-a-focal-length-and-rms`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodel::mutations::{apply_remodel_mutation, inverse_remodel_mutation, RemodelMutation};
use crate::artifacts::remodel::{RemodelDiff, RemodelSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> RemodelSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RemodelSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RemodelMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<RemodelDiff> {
    <RemodelMutation as protocol::Mutation<RemodelSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ The payload is FINAL state for the whole record: the matching camera is replaced wholesale
/// in place, keeping its position in the list.
#[semio_framework_async_macros::async_test]
async fn replaces_cam_a_in_place_with_the_refined_record() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("update-camera-calibration applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-camera-calibration/refines-the-cam-a-focal-length-and-rms: applied state differs from committed after-snapshot");
    assert_eq!(applied.calibration.cameras[0].fx, 1024.0, "the refined focal length is written");
    assert_eq!(applied.calibration.cameras[0].rms_reprojection_px, Some(0.25), "the refined reprojection RMS is written");
    assert_eq!(applied.calibration.cameras[0].id, "cam-a", "the record stays at index 0 — update never reorders");
    assert_eq!(applied.calibration.cameras[1], before().calibration.cameras[1], "the sibling camera is untouched");
    assert_eq!(applied.calibration.rig, before().calibration.rig, "refining intrinsics never touches the rig extrinsics keyed to the same camera");
}

/// ↩️ The inverse is the same verb carrying the captured base record.
#[semio_framework_async_macros::async_test]
async fn inverse_is_the_same_verb_carrying_the_base_camera_record() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelMutation::UpdateCameraCalibration(payload)] if payload.camera.id == "cam-a" && payload.camera.fx == 1000.0), "update-camera-calibration inverts to itself with the base fx of 1000, got {inverse:?}");
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-camera-calibration/refines-the-cam-a-focal-length-and-rms: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the record exists, differs from base, and every intrinsic plus the
/// optional RMS is finite, so none of the three guards fires.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_all_three_guards() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "update-camera-calibration/refines-the-cam-a-focal-length-and-rms declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "an existing, genuinely different, finite camera record raises none of mutation.target-missing / mutation.no-op / mutation.invariant, got {:?}", produced.messages());
    let calibration = produced.diff().calibration.as_ref().expect("update-camera-calibration writes the calibration field");
    assert_eq!(calibration.cameras.len(), 2, "the delta carries the whole camera list, not just the edited record");
    assert!(produced.diff().params.is_none(), "update-camera-calibration writes calibration alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-camera-calibration/refines-the-cam-a-focal-length-and-rms: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-camera-calibration/refines-the-cam-a-focal-length-and-rms: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `update-camera-calibration` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "update-camera-calibration/refines-the-cam-a-focal-length-and-rms: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let calibration = committed_diff.calibration.as_ref().expect("update-camera-calibration's delta is the whole calibration block");
    assert_eq!(calibration.cameras[0].fx, 1024.0, "the committed delta carries the refined intrinsics in place at index 0");
    assert_eq!(calibration.rig, before().calibration.rig, "and repeats the rig list keyed to the same camera");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-camera-calibration/refines-the-cam-a-focal-length-and-rms: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `update-camera-calibration`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "update-camera-calibration/refines-the-cam-a-focal-length-and-rms: committed diff did not carry before to after");
}
