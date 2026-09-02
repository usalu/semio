//! 🧪️ `remove-gcp-observation` fixture — `removes-the-only-observation-of-gcp-corner`.
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

/// ▶️ `observation_index` is a BASE-state position within that GCP's own observation list; the
/// GCP record itself survives with an empty list rather than being removed.
#[semio_framework_async_macros::async_test]
async fn empties_the_observation_list_without_removing_the_gcp() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("remove-gcp-observation applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "remove-gcp-observation/removes-the-only-observation-of-gcp-corner: applied state differs from committed after-snapshot");
    assert!(applied.gcps[1].observations.is_empty(), "the observation at position 0 is removed");
    assert_eq!(applied.gcps.len(), 2, "an unobserved GCP is kept, not garbage-collected");
    assert_eq!(applied.gcps[1].world_position, before().gcps[1].world_position, "removing an observation never re-solves the surveyed position");
    assert_eq!(applied.gcps[0], before().gcps[0], "the sibling GCP is untouched");
}

/// ↩️ The inverse is `add-gcp-observation` carrying the captured observation; because it was the
/// LAST one, the re-push restores list order exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_re_pushes_the_captured_observation() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::AddGcpObservation(payload)] if payload.id == "gcp-corner" && payload.observation.stream_id == "stream-b"),
        "remove-gcp-observation inverts to add-gcp-observation carrying the captured observation, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-gcp-observation/removes-the-only-observation-of-gcp-corner: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: index 0 is inside the base observation list, so neither the missing-GCP
/// nor the out-of-range `mutation.target-missing` rejection fires.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_in_range_index_check() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "remove-gcp-observation/removes-the-only-observation-of-gcp-corner declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "an in-range observation index on an existing GCP raises no mutation.target-missing, got {:?}", produced.messages());
    let gcps = produced.diff().gcps.as_ref().expect("remove-gcp-observation writes the gcps field");
    assert!(gcps.values[1].observations.is_empty(), "the gcps delta carries the emptied observation list");
    assert!(produced.diff().streams.is_none(), "remove-gcp-observation writes gcps alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-gcp-observation/removes-the-only-observation-of-gcp-corner: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-gcp-observation/removes-the-only-observation-of-gcp-corner: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `remove-gcp-observation` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "remove-gcp-observation/removes-the-only-observation-of-gcp-corner: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let gcps = committed_diff.gcps.as_ref().expect("remove-gcp-observation's delta is the GCP list");
    assert!(gcps.values[1].observations.is_empty(), "the committed delta carries gcp-corner's emptied observation list");
    assert_eq!(gcps.values.len(), 2, "and keeps the now-unobserved record in the list");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-gcp-observation/removes-the-only-observation-of-gcp-corner: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `remove-gcp-observation`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "remove-gcp-observation/removes-the-only-observation-of-gcp-corner: committed diff did not carry before to after");
}
