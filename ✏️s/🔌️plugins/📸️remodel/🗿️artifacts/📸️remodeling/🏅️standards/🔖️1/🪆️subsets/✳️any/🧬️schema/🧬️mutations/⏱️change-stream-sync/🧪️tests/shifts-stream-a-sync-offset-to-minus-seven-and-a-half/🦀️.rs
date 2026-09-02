//! 🧪️ `change-stream-sync` fixture — `shifts-stream-a-sync-offset-to-minus-seven-and-a-half`.
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

/// ▶️ Only `sync_offset_ms` on the named stream moves; the payload carries FINAL state, not a delta.
#[semio_framework_async_macros::async_test]
async fn writes_the_final_sync_offset_onto_stream_a_only() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("change-stream-sync applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-stream-sync/shifts-stream-a-sync-offset-to-minus-seven-and-a-half: applied state differs from committed after-snapshot");
    assert_eq!(applied.streams[0].sync_offset_ms, -7.5, "the payload's newSyncOffsetMs is the FINAL value, not an increment on 12.5");
    assert_eq!(applied.streams[1].sync_offset_ms, before().streams[1].sync_offset_ms, "the sibling stream's offset is untouched");
    assert_eq!(applied.streams[0].frames, before().streams[0].frames, "change-stream-sync never re-times the frames themselves");
    assert_eq!(applied.streams[0].source, before().streams[0].source, "change-stream-sync never touches the stream source");
}

/// ↩️ The inverse is the same verb carrying the captured base offset.
#[semio_framework_async_macros::async_test]
async fn inverse_is_the_same_verb_carrying_the_base_offset() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelingMutation::ChangeStreamSync(payload)] if payload.id == "stream-a" && payload.new_sync_offset_ms == 12.5), "change-stream-sync inverts to itself with the base offset 12.5, got {inverse:?}");
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-stream-sync/shifts-stream-a-sync-offset-to-minus-seven-and-a-half: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: -7.5 differs from the base 12.5 so the `mutation.no-op` warning is
/// silent, and -7.5 is finite so the `mutation.invariant` guard is silent too.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_trips_neither_the_no_op_nor_the_finite_guard() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "change-stream-sync/shifts-stream-a-sync-offset-to-minus-seven-and-a-half declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a finite, genuinely different offset raises nothing, got {:?}", produced.messages());
    assert!(produced.diff().streams.is_some(), "change-stream-sync writes the streams field");
    assert!(produced.diff().job.is_none() && produced.diff().params.is_none(), "change-stream-sync writes streams alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-stream-sync/shifts-stream-a-sync-offset-to-minus-seven-and-a-half: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-stream-sync/shifts-stream-a-sync-offset-to-minus-seven-and-a-half: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `change-stream-sync` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "change-stream-sync/shifts-stream-a-sync-offset-to-minus-seven-and-a-half: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let streams = committed_diff.streams.as_ref().expect("change-stream-sync's delta is the streams list");
    assert_eq!(streams.values[0].sync_offset_ms, -7.5, "the committed delta pins the new offset onto stream-a");
    assert_eq!(streams.values[1].sync_offset_ms, 0.0, "and repeats the untouched sibling stream verbatim");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-stream-sync/shifts-stream-a-sync-offset-to-minus-seven-and-a-half: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `change-stream-sync`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "change-stream-sync/shifts-stream-a-sync-offset-to-minus-seven-and-a-half: committed diff did not carry before to after");
}
