//! 🧪️ `replace-trajectory` fixture — `🍐️clears-the-camera-trajectory`.
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

/// ▶️ A `null` payload clears `results.trajectory` outright. The job's own live pose preview is
/// a separate field and survives, so the document keeps a preview with no finished trajectory.
#[semio_framework_async_macros::async_test]
async fn clears_the_trajectory_while_the_job_pose_preview_survives() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("replace-trajectory applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-trajectory/clears-the-camera-trajectory: applied state differs from committed after-snapshot");
    assert_eq!(applied.results.trajectory, None, "the finished trajectory is cleared");
    assert_eq!(applied.job.camera_poses_preview.len(), 1, "the job's live pose preview is a different field and is untouched");
    assert_eq!(applied.calibration.rig, before().calibration.rig, "clearing the trajectory never touches the rig extrinsics");
}

/// ↩️ The inverse is the same verb carrying the captured base trajectory.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_two_pose_trajectory() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::ReplaceTrajectory(payload)] if payload.trajectory.as_ref().is_some_and(|trajectory| trajectory.poses.len() == 2)),
        "replace-trajectory inverts to itself carrying the captured two-pose trajectory, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-trajectory/clears-the-camera-trajectory: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`. This leaf carries a rejection its sparse/dense/mesh siblings do not:
/// clearing an ALREADY-absent trajectory is `mutation.target-missing`. Here the base has one, so
/// the clear is a real change and no message is raised.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_a_trajectory_that_actually_exists() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-trajectory/clears-the-camera-trajectory declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "clearing a trajectory that exists raises neither mutation.target-missing nor mutation.no-op, got {:?}", produced.messages());
    let results = produced.diff().results.as_ref().expect("replace-trajectory writes the results field");
    assert_eq!(results.trajectory, None, "the results delta carries the cleared trajectory");
    assert!(produced.diff().job.is_none(), "replace-trajectory writes results alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-trajectory/clears-the-camera-trajectory: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-trajectory/clears-the-camera-trajectory: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-trajectory` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-trajectory/clears-the-camera-trajectory: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let results = committed_diff.results.as_ref().expect("replace-trajectory's delta is the whole results block");
    assert_eq!(results.trajectory, None, "the committed delta carries the cleared trajectory");
    assert_eq!(results.mesh, before().results.mesh, "and repeats every results sibling unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-trajectory/clears-the-camera-trajectory: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-trajectory`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-trajectory/clears-the-camera-trajectory: committed diff did not carry before to after");
}
