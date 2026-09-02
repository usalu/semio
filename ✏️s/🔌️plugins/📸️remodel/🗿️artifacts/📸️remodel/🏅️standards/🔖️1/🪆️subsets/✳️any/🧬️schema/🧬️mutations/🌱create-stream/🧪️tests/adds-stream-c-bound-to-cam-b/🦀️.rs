//! 🧪️ `create-stream` fixture — `adds-stream-c-bound-to-cam-b`.
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

/// ▶️ The payload stream is pushed onto the END of `streams`; nothing else in the document moves.
#[semio_framework_async_macros::async_test]
async fn appends_stream_c_after_the_two_existing_streams() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("create-stream applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-stream/adds-stream-c-bound-to-cam-b: applied state differs from committed after-snapshot");
    let ids: Vec<&str> = applied.streams.iter().map(|stream| stream.id.as_str()).collect();
    assert_eq!(ids, ["stream-a", "stream-b", "stream-c"], "create-stream pushes to the end; it never reorders or replaces an existing stream");
    let created = applied.streams.last().expect("stream-c is the appended stream");
    assert_eq!(created.camera_id.as_deref(), Some("cam-b"), "the payload's camera binding is stored verbatim");
    assert!(created.frames.is_empty(), "create-stream never invents frames for the new stream");
    assert_eq!(applied.calibration, before().calibration, "create-stream reads calibration to validate camera-id, but never writes it");
}

/// ↩️ For an id that is absent from `base`, the inverse is exactly one `delete-stream` naming it.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_single_delete_of_stream_c() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelMutation::DeleteStream(payload)] if payload.id == "stream-c"), "create-stream's inverse for a fresh id is one delete-stream for that id, got {inverse:?}");
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-stream/adds-stream-c-bound-to-cam-b: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: `stream-c` is new and `cam-b` exists, so neither `mutation.duplicate-id`
/// nor the unknown-camera `mutation.invariant` fires, and only `streams` is written.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_writes_streams_only() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "create-stream/adds-stream-c-bound-to-cam-b declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a fresh id with a known camera raises neither mutation.duplicate-id nor mutation.invariant, got {:?}", produced.messages());
    assert!(produced.diff().streams.is_some(), "create-stream writes the streams field");
    assert!(produced.diff().calibration.is_none() && produced.diff().gcps.is_none() && produced.diff().results.is_none() && produced.diff().assets.is_none(), "create-stream writes streams alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-stream/adds-stream-c-bound-to-cam-b: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-stream/adds-stream-c-bound-to-cam-b: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `create-stream` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "create-stream/adds-stream-c-bound-to-cam-b: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let streams = committed_diff.streams.as_ref().expect("create-stream's delta is the streams list");
    assert_eq!(streams.values.len(), 3, "the committed delta carries the full post-append stream list");
    assert_eq!(streams.values[2].id, "stream-c", "with the created stream last");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-stream/adds-stream-c-bound-to-cam-b: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `create-stream`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "create-stream/adds-stream-c-bound-to-cam-b: committed diff did not carry before to after");
}
