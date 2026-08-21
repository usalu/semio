//! 🧪️ `create-camera-calibration` fixture — `adds-the-cam-c-fisheye-calibration`.
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

/// ▶️ The camera is pushed onto the end of `calibration.cameras`; the rig list is not extended
/// alongside it, so a newly created camera starts with no rig pose.
#[semio_framework_async_macros::async_test]
async fn appends_cam_c_without_giving_it_a_rig_pose() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("create-camera-calibration applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-camera-calibration/adds-the-cam-c-fisheye-calibration: applied state differs from committed after-snapshot");
    let ids: Vec<&str> = applied.calibration.cameras.iter().map(|camera| camera.id.as_str()).collect();
    assert_eq!(ids, ["cam-a", "cam-b", "cam-c"], "the camera is appended, never inserted or reordered");
    assert_eq!(applied.calibration.rig, before().calibration.rig, "create-camera-calibration never mints a rig extrinsic for the new camera");
    let created = applied.calibration.cameras.last().expect("cam-c is the appended camera");
    assert_eq!(created.model, "fisheye", "the distortion model label is stored verbatim");
    assert_eq!(created.rms_reprojection_px, None, "an uncalibrated camera keeps a null RMS rather than a fabricated zero");
}

/// ↩️ For an id absent from `base`, the inverse is one `delete-camera-calibration`.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_single_delete_of_cam_c() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::DeleteCameraCalibration(payload)] if payload.camera_id == "cam-c"),
        "create-camera-calibration's inverse for a fresh id is one delete-camera-calibration, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-camera-calibration/adds-the-cam-c-fisheye-calibration: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: `cam-c` is new so `mutation.duplicate-id` stays silent. Note this leaf
/// carries NO finite-intrinsics guard — unlike its `update` sibling, it validates the id only.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_checks_only_the_id() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "create-camera-calibration/adds-the-cam-c-fisheye-calibration declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a fresh camera id raises no mutation.duplicate-id, got {:?}", produced.messages());
    let calibration = produced.diff().calibration.as_ref().expect("create-camera-calibration writes the calibration field");
    assert_eq!(calibration.cameras.len(), 3, "the calibration delta carries the whole camera list");
    assert_eq!(calibration.rig.len(), 1, "the calibration delta carries the rig list unchanged");
    assert!(produced.diff().streams.is_none(), "create-camera-calibration writes calibration alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-camera-calibration/adds-the-cam-c-fisheye-calibration: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-camera-calibration/adds-the-cam-c-fisheye-calibration: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `create-camera-calibration` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "create-camera-calibration/adds-the-cam-c-fisheye-calibration: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let calibration = committed_diff.calibration.as_ref().expect("create-camera-calibration's delta is the whole calibration block");
    assert_eq!(calibration.cameras.len(), 3, "the committed delta carries the post-append camera list");
    assert_eq!(calibration.rig.len(), 1, "and repeats the rig list — no pose is minted for cam-c");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-camera-calibration/adds-the-cam-c-fisheye-calibration: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `create-camera-calibration`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "create-camera-calibration/adds-the-cam-c-fisheye-calibration: committed diff did not carry before to after");
}
