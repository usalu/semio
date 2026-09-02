//! 🧪️ `replace-qc` fixture — `records-a-qc-report-carrying-a-watertight-summary`.
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

/// ▶️ The QC report carries its OWN copy of the watertight summary, independent of the one on
/// `results.mesh`; replacing the report also clears the base's GCP checkpoint RMSE and warnings.
#[semio_framework_async_macros::async_test]
async fn records_its_own_watertight_copy_and_clears_the_base_warnings() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("replace-qc applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-qc/records-a-qc-report-carrying-a-watertight-summary: applied state differs from committed after-snapshot");
    let qc = applied.results.qc.as_ref().expect("the QC report is present");
    assert!(qc.watertight.is_some(), "the report now carries its own watertight summary, where the base report had none");
    assert_eq!(qc.gcp_checkpoint_rmse, None, "the base checkpoint RMSE is cleared by the wholesale replace");
    assert!(qc.warnings.is_empty(), "the base warning list is cleared by the wholesale replace");
    assert_eq!(qc.registered_frame_ratio, 0.875, "the new registration ratio is written");
    assert_eq!(applied.results.mesh.watertight, before().results.mesh.watertight, "the mesh result's own watertight report is a separate field and is untouched");
}

/// ↩️ The inverse is the same verb carrying the captured base report.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_warning_carrying_base_report() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::ReplaceQc(payload)] if payload.qc.as_ref().is_some_and(|qc| qc.warnings.len() == 1 && qc.watertight.is_none())),
        "replace-qc inverts to itself carrying the captured base report, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-qc/records-a-qc-report-carrying-a-watertight-summary: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`. This leaf rejects a both-null clear with `mutation.target-missing`
/// ("There is no QC report to clear."); a non-null payload never reaches that branch.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_never_reaches_the_nothing_to_clear_rejection() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-qc/records-a-qc-report-carrying-a-watertight-summary declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a non-null, genuinely different QC report raises nothing, got {:?}", produced.messages());
    let results = produced.diff().results.as_ref().expect("replace-qc writes the results field");
    assert!(results.qc.as_ref().is_some_and(|qc| qc.watertight.is_some()), "the results delta carries the new QC report");
    assert!(produced.diff().job.is_none(), "replace-qc writes results alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-qc/records-a-qc-report-carrying-a-watertight-summary: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-qc/records-a-qc-report-carrying-a-watertight-summary: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-qc` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-qc/records-a-qc-report-carrying-a-watertight-summary: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let results = committed_diff.results.as_ref().expect("replace-qc's delta is the whole results block");
    assert!(results.qc.as_ref().is_some_and(|qc| qc.watertight.is_some()), "the committed delta carries the report with its own watertight copy");
    assert_eq!(results.mesh.watertight, before().results.mesh.watertight, "and repeats the mesh's separate watertight report unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-qc/records-a-qc-report-carrying-a-watertight-summary: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-qc`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-qc/records-a-qc-report-carrying-a-watertight-summary: committed diff did not carry before to after");
}
