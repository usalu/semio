//! 🧪️ `create-gcp` fixture — `adds-gcp-tower-with-one-observation`.
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

/// ▶️ The whole GCP record — surveyed position AND its observations — arrives in one payload and
/// is pushed onto the end of `gcps`; the observation is NOT re-validated against the stream it
/// names.
#[semio_framework_async_macros::async_test]
async fn appends_the_gcp_together_with_its_observation() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("create-gcp applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-gcp/adds-gcp-tower-with-one-observation: applied state differs from committed after-snapshot");
    let ids: Vec<&str> = applied.gcps.iter().map(|gcp| gcp.id.as_str()).collect();
    assert_eq!(ids, ["gcp-ridge", "gcp-corner", "gcp-tower"], "the GCP is appended, never inserted");
    let created = applied.gcps.last().expect("gcp-tower is the appended GCP");
    assert_eq!(created.world_position, [7.0, 8.0, 9.0], "the surveyed world position is stored verbatim");
    assert_eq!(created.observations.len(), 1, "create-gcp carries observations in the same payload — no separate add-gcp-observation is needed");
    assert_eq!(applied.streams, before().streams, "create-gcp never touches the streams its observations reference");
}

/// ↩️ For an id absent from `base`, the inverse is one `delete-gcp`.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_single_delete_of_gcp_tower() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelMutation::DeleteGcp(payload)] if payload.id == "gcp-tower"), "create-gcp's inverse for a fresh id is one delete-gcp for that id, got {inverse:?}");
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-gcp/adds-gcp-tower-with-one-observation: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: `gcp-tower` is new, so the FATAL `mutation.duplicate-id` guard — this
/// leaf's only guard — stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_duplicate_id_guard() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "create-gcp/adds-gcp-tower-with-one-observation declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a fresh GCP id raises no mutation.duplicate-id, got {:?}", produced.messages());
    let gcps = produced.diff().gcps.as_ref().expect("create-gcp writes the gcps field");
    assert_eq!(gcps.values.len(), 3, "the gcps delta carries the whole post-append list");
    assert!(produced.diff().streams.is_none() && produced.diff().calibration.is_none(), "create-gcp writes gcps alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-gcp/adds-gcp-tower-with-one-observation: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-gcp/adds-gcp-tower-with-one-observation: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `create-gcp` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "create-gcp/adds-gcp-tower-with-one-observation: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let gcps = committed_diff.gcps.as_ref().expect("create-gcp's delta is the GCP list");
    assert_eq!(gcps.values.len(), 3, "the committed delta carries the full post-append GCP list");
    assert_eq!(gcps.values[2].observations.len(), 1, "with the created record's own observation already inside it");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-gcp/adds-gcp-tower-with-one-observation: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `create-gcp`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "create-gcp/adds-gcp-tower-with-one-observation: committed diff did not carry before to after");
}
