//! 🧪️ `delete-rig-extrinsic` fixture — `✂️drops-the-cam-a-rig-extrinsic`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodeling::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};
use crate::artifacts::remodeling::{RemodelingDiff, RemodelingSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> RemodelingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RemodelingSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RemodelingMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<RemodelingDiff> {
    <RemodelingMutation as protocol::Mutation<RemodelingSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ The pose is retained out of `calibration.rig` by `camera_id`; the camera record it was
/// keyed to survives, so the camera stays calibrated but unposed.
#[semio_framework_async_macros::async_test]
async fn empties_the_rig_while_keeping_the_cam_a_record() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("delete-rig-extrinsic applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-rig-extrinsic/drops-the-cam-a-rig-extrinsic: applied state differs from committed after-snapshot");
    assert!(applied.calibration.rig.is_empty(), "the only rig pose is removed");
    assert_eq!(applied.calibration.cameras, before().calibration.cameras, "the camera record the pose was keyed to survives");
    assert_eq!(applied.results.trajectory, before().results.trajectory, "removing a rig pose never rewrites the recovered trajectory");
}

/// ↩️ The inverse is `create-rig-extrinsic` carrying the captured pose.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_captured_identity_pose() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::CreateRigExtrinsic(payload)] if payload.extrinsic.camera_id == "cam-a" && payload.extrinsic.rotation_wxyz == [1.0, 0.0, 0.0, 0.0]),
        "delete-rig-extrinsic inverts to create-rig-extrinsic carrying the captured identity pose, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-rig-extrinsic/drops-the-cam-a-rig-extrinsic: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: a pose for `cam-a` exists, so the `mutation.target-missing` rejection —
/// this leaf's only guard — stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_target_missing_guard() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "delete-rig-extrinsic/drops-the-cam-a-rig-extrinsic declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "an existing rig pose raises no mutation.target-missing, got {:?}", produced.messages());
    let calibration = produced.diff().calibration.as_ref().expect("delete-rig-extrinsic writes the calibration field");
    assert!(calibration.rig.is_empty(), "the delta carries the emptied rig list");
    assert_eq!(calibration.cameras.len(), 2, "the delta carries the camera list unchanged alongside it");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-rig-extrinsic/drops-the-cam-a-rig-extrinsic: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-rig-extrinsic/drops-the-cam-a-rig-extrinsic: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `delete-rig-extrinsic` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "delete-rig-extrinsic/drops-the-cam-a-rig-extrinsic: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let calibration = committed_diff.calibration.as_ref().expect("delete-rig-extrinsic's delta is the whole calibration block");
    assert!(calibration.rig.is_empty(), "the committed delta carries the emptied rig list");
    assert_eq!(calibration.cameras.len(), 2, "and repeats the camera list the removed pose was keyed to");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-rig-extrinsic/drops-the-cam-a-rig-extrinsic: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `delete-rig-extrinsic`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "delete-rig-extrinsic/drops-the-cam-a-rig-extrinsic: committed diff did not carry before to after");
}
