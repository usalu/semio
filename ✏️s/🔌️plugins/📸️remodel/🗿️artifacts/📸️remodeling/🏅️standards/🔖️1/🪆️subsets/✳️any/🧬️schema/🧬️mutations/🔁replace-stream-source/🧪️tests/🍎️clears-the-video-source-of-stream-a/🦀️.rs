//! 🧪️ `replace-stream-source` fixture — `🍎️clears-the-video-source-of-stream-a`.
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

/// ▶️ A `null` payload CLEARS the source outright; frames and media kind are left alone, so the
/// stream keeps its `video` provenance while losing its container detail.
#[semio_framework_async_macros::async_test]
async fn clears_the_source_while_keeping_the_frames() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("replace-stream-source applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-stream-source/clears-the-video-source-of-stream-a: applied state differs from committed after-snapshot");
    assert_eq!(applied.streams[0].source, None, "a null payload clears the source rather than being ignored");
    assert_eq!(applied.streams[0].frames, before().streams[0].frames, "clearing the source never drops the already-extracted frames");
    assert_eq!(applied.streams[0].kind, before().streams[0].kind, "clearing the source never rewrites the media kind");
    assert_eq!(applied.streams[1], before().streams[1], "the sourceless sibling stream is untouched");
}

/// ↩️ The inverse is the same verb carrying the captured base source.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_captured_mp4_source() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::ReplaceStreamSource(payload)] if payload.id == "stream-a" && payload.source.as_ref().is_some_and(|source| source.name == "front.mp4")),
        "replace-stream-source inverts to itself carrying the captured base source, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-stream-source/clears-the-video-source-of-stream-a: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`. Unlike every sibling replace verb, this leaf has NO `mutation.no-op`
/// guard at all — only the missing-stream `mutation.target-missing` rejection.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_has_no_no_op_guard_to_trip() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-stream-source/clears-the-video-source-of-stream-a declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "an existing stream id raises no mutation.target-missing, got {:?}", produced.messages());
    let streams = produced.diff().streams.as_ref().expect("replace-stream-source writes the streams field");
    assert_eq!(streams.values[0].source, None, "the streams delta carries the cleared source");
    assert_eq!(streams.values.len(), 2, "the delta always carries the full stream list, not just the edited stream");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-stream-source/clears-the-video-source-of-stream-a: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-stream-source/clears-the-video-source-of-stream-a: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-stream-source` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-stream-source/clears-the-video-source-of-stream-a: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let streams = committed_diff.streams.as_ref().expect("replace-stream-source's delta is the streams list");
    assert_eq!(streams.values[0].source, None, "the committed delta carries stream-a with its source cleared");
    assert_eq!(streams.values[0].frames.len(), 2, "and its already-extracted frames still attached");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-stream-source/clears-the-video-source-of-stream-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-stream-source`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-stream-source/clears-the-video-source-of-stream-a: committed diff did not carry before to after");
}
