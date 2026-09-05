//! 🧪️ `delete-stream` fixture — `🚫️removes-stream-b-and-cascades-its-gcp-observation`.
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

/// ▶️ Deleting `stream-b` also rewrites `gcps`: every GCP observation that pointed at the stream
/// is dropped in the same diff — the only leaf in this tree that writes two collections at once.
#[semio_framework_async_macros::async_test]
async fn drops_stream_b_and_the_gcp_observation_that_referenced_it() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("delete-stream applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-stream/removes-stream-b-and-cascades-its-gcp-observation: applied state differs from committed after-snapshot");
    let ids: Vec<&str> = applied.streams.iter().map(|stream| stream.id.as_str()).collect();
    assert_eq!(ids, ["stream-a"], "only stream-b is removed");
    assert!(applied.gcps.iter().all(|gcp| gcp.observations.iter().all(|observation| observation.stream_id != "stream-b")), "every observation referencing the deleted stream is cascaded away");
    assert_eq!(applied.gcps.len(), before().gcps.len(), "the cascade removes observations, never whole GCPs");
    assert_eq!(applied.assets, before().assets, "delete-stream leaves the assets its frames referenced in place");
}

/// ↩️ The inverse is one `create-stream` — it restores the stream but NOT the cascaded observation:
/// the observation cascade is deliberately one-way (the leaf's inverse only reads `base.streams`).
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_stream_but_not_the_cascaded_observation() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelingMutation::CreateStream(payload)] if payload.stream.id == "stream-b"), "delete-stream's inverse is one create-stream carrying the captured base stream, got {inverse:?}");
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot.streams, base.streams, "the stream itself comes back at its original position");
    assert_ne!(snapshot, base, "the cascaded GCP observation is NOT restored — delete-stream's cascade is one-way");
    assert!(snapshot.gcps.iter().all(|gcp| gcp.observations.is_empty()), "the observation that referenced stream-b stays gone after the inverse");
}

/// 🎯️ Declared `applied` with one `mutation.cascade` note counting the removed observations.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_reports_one_cascaded_observation() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "delete-stream/removes-stream-b-and-cascades-its-gcp-observation declares an applied outcome");
    let produced = produced();
    let codes: Vec<&str> = produced.messages().iter().map(|message| message.code.0.as_str()).collect();
    assert_eq!(codes, ["mutation.cascade"], "a stream with GCP references reports exactly one cascade note");
    assert_eq!(produced.messages()[0].level, protocol::Severity::Info, "a cascade is informational, never a rejection");
    assert!(produced.messages()[0].message.contains("1 GCP observation(s)"), "the cascade note counts the removed observations, got {:?}", produced.messages()[0].message);
    assert!(produced.diff().streams.is_some() && produced.diff().gcps.is_some(), "a cascading delete-stream writes both streams and gcps");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-stream/removes-stream-b-and-cascades-its-gcp-observation: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-stream/removes-stream-b-and-cascades-its-gcp-observation: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `delete-stream` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "delete-stream/removes-stream-b-and-cascades-its-gcp-observation: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let streams = committed_diff.streams.as_ref().expect("delete-stream's delta carries the streams list");
    let gcps = committed_diff.gcps.as_ref().expect("a CASCADING delete-stream also carries the gcps list — the only two-field delta in this tree");
    assert_eq!(streams.values.len(), 1, "the committed delta carries the post-deletion stream list");
    assert!(gcps.values.iter().all(|gcp| gcp.observations.is_empty()), "and the GCP list with every stream-b observation already removed");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-stream/removes-stream-b-and-cascades-its-gcp-observation: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `delete-stream`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "delete-stream/removes-stream-b-and-cascades-its-gcp-observation: committed diff did not carry before to after");
}
