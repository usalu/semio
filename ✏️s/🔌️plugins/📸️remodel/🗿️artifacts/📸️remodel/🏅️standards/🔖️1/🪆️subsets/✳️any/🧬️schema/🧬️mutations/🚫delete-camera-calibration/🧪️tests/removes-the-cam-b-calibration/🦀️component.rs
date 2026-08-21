//! 🧪️ `delete-camera-calibration` fixture — `removes-the-cam-b-calibration`.
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

/// ▶️ Only the matching camera is retained out of `calibration.cameras`. Unlike `delete-stream`,
/// this leaf has NO cascade at all: the rig extrinsic list and every stream `camera_id` binding
/// are left exactly as they were.
#[semio_framework_async_macros::async_test]
async fn retains_every_other_camera_and_cascades_nothing() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("delete-camera-calibration applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-camera-calibration/removes-the-cam-b-calibration: applied state differs from committed after-snapshot");
    let ids: Vec<&str> = applied.calibration.cameras.iter().map(|camera| camera.id.as_str()).collect();
    assert_eq!(ids, ["cam-a"], "cam-b is retained out of the list");
    assert_eq!(applied.calibration.rig, before().calibration.rig, "the rig list is never cascaded by a camera deletion");
    assert_eq!(applied.streams, before().streams, "a stream bound to a deleted camera keeps its binding");
}

/// ↩️ The inverse is `create-camera-calibration` carrying the captured record; because `cam-b` was
/// the LAST camera, the re-append restores list order exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_captured_cam_b_record() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::CreateCameraCalibration(payload)] if payload.camera.id == "cam-b" && payload.camera.locked),
        "delete-camera-calibration inverts to create-camera-calibration carrying the captured record, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-camera-calibration/removes-the-cam-b-calibration: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: `cam-b` exists so `mutation.target-missing` stays silent, and this leaf
/// emits no cascade note even though the deleted camera is referenced elsewhere.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_is_silent_about_dangling_references() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "delete-camera-calibration/removes-the-cam-b-calibration declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "delete-camera-calibration raises no mutation.cascade note, unlike delete-stream and delete-asset, got {:?}", produced.messages());
    let calibration = produced.diff().calibration.as_ref().expect("delete-camera-calibration writes the calibration field");
    assert_eq!(calibration.cameras.len(), 1, "the delta carries the post-deletion camera list");
    assert!(produced.diff().streams.is_none(), "delete-camera-calibration writes calibration alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-camera-calibration/removes-the-cam-b-calibration: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-camera-calibration/removes-the-cam-b-calibration: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `delete-camera-calibration` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "delete-camera-calibration/removes-the-cam-b-calibration: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let calibration = committed_diff.calibration.as_ref().expect("delete-camera-calibration's delta is the whole calibration block");
    assert_eq!(calibration.cameras.len(), 1, "the committed delta carries the post-deletion camera list");
    assert_eq!(calibration.rig, before().calibration.rig, "and repeats the rig list verbatim — this leaf cascades nothing");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-camera-calibration/removes-the-cam-b-calibration: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `delete-camera-calibration`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "delete-camera-calibration/removes-the-cam-b-calibration: committed diff did not carry before to after");
}
