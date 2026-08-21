//! 🧪️ `replace-job` fixture — `advances-the-job-to-texturing`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodel::mutations::{apply_remodel_mutation, inverse_remodel_mutation, RemodelMutation};
use crate::artifacts::remodel::{RemodelDiff, RemodelSnapshot};
use crate::artifacts::remodel::ReconstructionStage;

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

/// ▶️ This is the only leaf that writes `job`, and it writes the WHOLE record — the payload is
/// stored verbatim rather than merged onto the base job.
#[semio_framework_async_macros::async_test]
async fn stores_the_whole_job_record_verbatim() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("replace-job applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-job/advances-the-job-to-texturing: applied state differs from committed after-snapshot");
    assert_eq!(applied.job.stage, ReconstructionStage::Texturing, "the stage advances past the base bundle-adjusting");
    assert_eq!(applied.job.progress_0_1, 0.75, "the progress fraction is written");
    assert_eq!(applied.job.stage_cursor, 11, "the stage cursor is written");
    assert_eq!(applied.job.id, before().job.id, "the payload repeats the job id, so it survives the wholesale replace");
    assert_eq!(applied.results, before().results, "advancing the job never writes the results block");
}

/// ↩️ The inverse is the same verb carrying the captured base job.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_bundle_adjusting_job() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::ReplaceJob(payload)] if payload.job.stage == ReconstructionStage::BundleAdjusting && payload.job.stage_cursor == 3),
        "replace-job inverts to itself with the captured base job, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-job/advances-the-job-to-texturing: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the payload differs from the base job, so the `mutation.no-op` warning —
/// this leaf's ONLY guard, with no invariant check at all — stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_writes_the_job_field_only() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-job/advances-the-job-to-texturing declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a genuinely different job raises no mutation.no-op, got {:?}", produced.messages());
    let job = produced.diff().job.as_ref().expect("replace-job writes the job field");
    assert_eq!(job.stage, ReconstructionStage::Texturing, "the job delta is the payload itself, not a merge onto base");
    assert!(produced.diff().results.is_none() && produced.diff().params.is_none(), "replace-job writes job alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-job/advances-the-job-to-texturing: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-job/advances-the-job-to-texturing: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-job` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-job/advances-the-job-to-texturing: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let job = committed_diff.job.as_ref().expect("replace-job's delta is the job record itself");
    assert_eq!(job.stage, ReconstructionStage::Texturing, "the committed delta IS the payload job — no surrounding block is carried");
    assert_eq!(job.stage_cursor, 11, "including the stage cursor that moved with it");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-job/advances-the-job-to-texturing: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-job`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-job/advances-the-job-to-texturing: committed diff did not carry before to after");
}
