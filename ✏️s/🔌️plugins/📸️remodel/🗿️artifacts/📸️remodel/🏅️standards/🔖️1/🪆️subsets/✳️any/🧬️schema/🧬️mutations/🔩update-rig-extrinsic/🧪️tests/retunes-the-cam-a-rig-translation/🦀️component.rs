//! 🧪️ `update-rig-extrinsic` fixture — `retunes-the-cam-a-rig-translation`.
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

/// ▶️ The payload is FINAL state for the whole pose record, written in place at its existing
/// position — the rotation stays at identity because the payload repeats it, not because the leaf
/// merges fields.
#[semio_framework_async_macros::async_test]
async fn writes_the_whole_pose_record_in_place() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("update-rig-extrinsic applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-rig-extrinsic/retunes-the-cam-a-rig-translation: applied state differs from committed after-snapshot");
    assert_eq!(applied.calibration.rig[0].translation_m, [0.125, -0.25, 0.5], "the retuned translation is written");
    assert_eq!(applied.calibration.rig[0].rotation_wxyz, [1.0, 0.0, 0.0, 0.0], "the payload's repeated identity rotation is written back verbatim");
    assert_eq!(applied.calibration.rig.len(), 1, "update never appends a second pose for the same camera");
    assert_eq!(applied.calibration.cameras, before().calibration.cameras, "retuning extrinsics never touches intrinsics");
}

/// ↩️ The inverse is the same verb carrying the captured base pose.
#[semio_framework_async_macros::async_test]
async fn inverse_is_the_same_verb_carrying_the_base_pose() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::UpdateRigExtrinsic(payload)] if payload.extrinsic.camera_id == "cam-a" && payload.extrinsic.translation_m == [0.0, 0.0, 0.0]),
        "update-rig-extrinsic inverts to itself with the base zero translation, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-rig-extrinsic/retunes-the-cam-a-rig-translation: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the pose exists, every rotation and translation component is finite, and
/// the record genuinely differs — so target-missing, invariant and no-op all stay silent.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_finite_pose_guard() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "update-rig-extrinsic/retunes-the-cam-a-rig-translation declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a finite, genuinely different pose raises none of the three guards, got {:?}", produced.messages());
    let calibration = produced.diff().calibration.as_ref().expect("update-rig-extrinsic writes the calibration field");
    assert_eq!(calibration.rig.len(), 1, "the delta carries the whole rig list");
    assert!(produced.diff().results.is_none(), "update-rig-extrinsic writes calibration alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-rig-extrinsic/retunes-the-cam-a-rig-translation: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-rig-extrinsic/retunes-the-cam-a-rig-translation: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `update-rig-extrinsic` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "update-rig-extrinsic/retunes-the-cam-a-rig-translation: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let calibration = committed_diff.calibration.as_ref().expect("update-rig-extrinsic's delta is the whole calibration block");
    assert_eq!(calibration.rig[0].translation_m, [0.125, -0.25, 0.5], "the committed delta carries the retuned pose");
    assert_eq!(calibration.cameras, before().calibration.cameras, "and repeats the intrinsics it never touches");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-rig-extrinsic/retunes-the-cam-a-rig-translation: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `update-rig-extrinsic`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "update-rig-extrinsic/retunes-the-cam-a-rig-translation: committed diff did not carry before to after");
}
