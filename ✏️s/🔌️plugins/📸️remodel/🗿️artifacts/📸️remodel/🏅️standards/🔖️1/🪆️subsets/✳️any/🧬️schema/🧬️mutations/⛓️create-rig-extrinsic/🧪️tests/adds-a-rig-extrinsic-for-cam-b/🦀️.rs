//! 🧪️ `create-rig-extrinsic` fixture — `adds-a-rig-extrinsic-for-cam-b`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodel::mutations::{apply_remodel_mutation, inverse_remodel_mutation, RemodelMutation};
use crate::artifacts::remodel::{RemodelDiff, RemodelSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

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

/// ▶️ The rig list is keyed by `camera_id`; the new pose is appended and the camera records
/// themselves are untouched.
#[semio_framework_async_macros::async_test]
async fn appends_a_rig_pose_keyed_to_the_existing_cam_b() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("create-rig-extrinsic applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-rig-extrinsic/adds-a-rig-extrinsic-for-cam-b: applied state differs from committed after-snapshot");
    let camera_ids: Vec<&str> = applied.calibration.rig.iter().map(|extrinsic| extrinsic.camera_id.as_str()).collect();
    assert_eq!(camera_ids, ["cam-a", "cam-b"], "the rig pose is appended after the existing cam-a pose");
    assert_eq!(applied.calibration.rig[1].translation_m, [0.25, 0.0, -0.125], "the payload translation is stored verbatim");
    assert_eq!(applied.calibration.cameras, before().calibration.cameras, "create-rig-extrinsic reads the camera list to validate, but never writes it");
}

/// ↩️ For a camera without an existing rig pose, the inverse is one `delete-rig-extrinsic`.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_single_delete_of_the_cam_b_pose() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelMutation::DeleteRigExtrinsic(payload)] if payload.camera_id == "cam-b"), "create-rig-extrinsic's inverse for an unposed camera is one delete-rig-extrinsic, got {inverse:?}");
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-rig-extrinsic/adds-a-rig-extrinsic-for-cam-b: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: `cam-b` has no rig pose yet (no `mutation.duplicate-id`) and does exist
/// as a calibrated camera (no unknown-camera `mutation.invariant`). Both guards are FATAL here.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_both_fatal_guards() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "create-rig-extrinsic/adds-a-rig-extrinsic-for-cam-b declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "an unposed but calibrated camera raises neither fatal guard, got {:?}", produced.messages());
    let calibration = produced.diff().calibration.as_ref().expect("create-rig-extrinsic writes the calibration field");
    assert_eq!(calibration.rig.len(), 2, "the delta carries the post-append rig list");
    assert!(produced.diff().job.is_none() && produced.diff().results.is_none(), "create-rig-extrinsic writes calibration alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-rig-extrinsic/adds-a-rig-extrinsic-for-cam-b: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-rig-extrinsic/adds-a-rig-extrinsic-for-cam-b: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `create-rig-extrinsic` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "create-rig-extrinsic/adds-a-rig-extrinsic-for-cam-b: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let calibration = committed_diff.calibration.as_ref().expect("create-rig-extrinsic's delta is the whole calibration block");
    assert_eq!(calibration.rig.len(), 2, "the committed delta carries the post-append rig list");
    assert_eq!(calibration.cameras.len(), 2, "and repeats the camera list it only READ to validate cam-b");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-rig-extrinsic/adds-a-rig-extrinsic-for-cam-b: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `create-rig-extrinsic`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "create-rig-extrinsic/adds-a-rig-extrinsic-for-cam-b: committed diff did not carry before to after");
}
