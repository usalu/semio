//! 🧪️ `add-gcp-observation` fixture — `🟧️adds-the-first-observation-to-gcp-ridge`.
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

/// ▶️ The observation is pushed onto the named GCP's own list; the GCP's surveyed world position
/// is not re-derived from it, and the sibling GCP is untouched.
#[semio_framework_async_macros::async_test]
async fn pushes_the_observation_onto_gcp_ridge_only() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("add-gcp-observation applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "add-gcp-observation/adds-the-first-observation-to-gcp-ridge: applied state differs from committed after-snapshot");
    assert_eq!(applied.gcps[0].observations.len(), 1, "the previously unobserved GCP now carries one observation");
    assert_eq!(applied.gcps[0].observations[0].pixel, [30.0, 40.0], "the payload pixel is stored verbatim");
    assert_eq!(applied.gcps[0].world_position, before().gcps[0].world_position, "adding an observation never re-solves the surveyed position");
    assert_eq!(applied.gcps[1], before().gcps[1], "the sibling GCP is untouched");
}

/// ↩️ The inverse is `remove-gcp-observation` at the BASE observation count — the index the push
/// lands on.
#[semio_framework_async_macros::async_test]
async fn inverse_removes_the_observation_at_the_base_count() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::RemoveGcpObservation(payload)] if payload.id == "gcp-ridge" && payload.observation_index == 0),
        "add-gcp-observation inverts to remove-gcp-observation at index 0, the base observation count, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "add-gcp-observation/adds-the-first-observation-to-gcp-ridge: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the GCP exists and does not already contain an exactly-equal
/// observation, so neither `mutation.target-missing` nor `mutation.no-op` fires.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_does_not_trip_the_duplicate_observation_no_op() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "add-gcp-observation/adds-the-first-observation-to-gcp-ridge declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "an observation not already present raises no mutation.no-op, got {:?}", produced.messages());
    let gcps = produced.diff().gcps.as_ref().expect("add-gcp-observation writes the gcps field");
    assert_eq!(gcps.values[0].observations.len(), 1, "the gcps delta carries the post-push observation list");
    assert_eq!(gcps.values.len(), 2, "the delta always carries the full GCP list, not just the edited record");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-gcp-observation/adds-the-first-observation-to-gcp-ridge: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "add-gcp-observation/adds-the-first-observation-to-gcp-ridge: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `add-gcp-observation` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "add-gcp-observation/adds-the-first-observation-to-gcp-ridge: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let gcps = committed_diff.gcps.as_ref().expect("add-gcp-observation's delta is the GCP list");
    assert_eq!(gcps.values[0].observations.len(), 1, "the committed delta carries gcp-ridge's post-push observation list");
    assert_eq!(gcps.values[1].observations.len(), 1, "and repeats the untouched sibling GCP verbatim");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-gcp-observation/adds-the-first-observation-to-gcp-ridge: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `add-gcp-observation`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "add-gcp-observation/adds-the-first-observation-to-gcp-ridge: committed diff did not carry before to after");
}
