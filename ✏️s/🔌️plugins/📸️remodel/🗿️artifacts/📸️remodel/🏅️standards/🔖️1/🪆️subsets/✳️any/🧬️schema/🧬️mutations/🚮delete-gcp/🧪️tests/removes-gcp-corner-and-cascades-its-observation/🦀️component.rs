//! 🧪️ `delete-gcp` fixture — `removes-gcp-corner-and-cascades-its-observation`.
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

/// ▶️ Deleting a GCP takes its observations with it — the cascade here is intrinsic (observations
/// are owned by the record), not a cross-collection rewrite like `delete-stream`'s.
#[semio_framework_async_macros::async_test]
async fn removes_the_gcp_and_the_observation_it_owned() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("delete-gcp applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-gcp/removes-gcp-corner-and-cascades-its-observation: applied state differs from committed after-snapshot");
    let ids: Vec<&str> = applied.gcps.iter().map(|gcp| gcp.id.as_str()).collect();
    assert_eq!(ids, ["gcp-ridge"], "only gcp-corner is retained out of the list");
    assert_eq!(applied.gcps.iter().map(|gcp| gcp.observations.len()).sum::<usize>(), 0, "the document has no observations left at all");
    assert_eq!(applied.streams, before().streams, "the stream the removed observation pointed at is untouched");
}

/// ↩️ The inverse is `create-gcp` carrying the captured record with its observations; because
/// `gcp-corner` was the LAST GCP, the re-append restores list order exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_gcp_with_its_observation() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::CreateGcp(payload)] if payload.gcp.id == "gcp-corner" && payload.gcp.observations.len() == 1),
        "delete-gcp inverts to create-gcp carrying the captured record AND its observations, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-gcp/removes-gcp-corner-and-cascades-its-observation: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied` with one `mutation.cascade` note counting the observations that went
/// with the record.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_reports_one_cascaded_observation() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "delete-gcp/removes-gcp-corner-and-cascades-its-observation declares an applied outcome");
    let produced = produced();
    let codes: Vec<&str> = produced.messages().iter().map(|message| message.code.0.as_str()).collect();
    assert_eq!(codes, ["mutation.cascade"], "a GCP carrying observations reports exactly one cascade note");
    assert_eq!(produced.messages()[0].level, protocol::Severity::Info, "the cascade note is informational");
    assert!(produced.messages()[0].message.contains("1 observation(s)"), "the cascade note counts the observations that went with the record, got {:?}", produced.messages()[0].message);
    assert!(produced.diff().gcps.is_some() && produced.diff().streams.is_none(), "delete-gcp writes gcps alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-gcp/removes-gcp-corner-and-cascades-its-observation: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-gcp/removes-gcp-corner-and-cascades-its-observation: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `delete-gcp` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "delete-gcp/removes-gcp-corner-and-cascades-its-observation: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let gcps = committed_diff.gcps.as_ref().expect("delete-gcp's delta is the GCP list");
    assert_eq!(gcps.values.len(), 1, "the committed delta carries the post-deletion GCP list");
    assert!(committed_diff.streams.is_none(), "delete-gcp's cascade stays inside the observation list the record owned");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-gcp/removes-gcp-corner-and-cascades-its-observation: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `delete-gcp`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "delete-gcp/removes-gcp-corner-and-cascades-its-observation: committed diff did not carry before to after");
}
