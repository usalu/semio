//! 🧪️ `add-stream-frame` fixture — `🎞️appends-a-third-frame-to-stream-a`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodeling::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};
use crate::artifacts::remodeling::MediaKind;
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

/// ▶️ The frame is appended AND the payload's `kind` is written onto the stream — this leaf
/// re-stamps the media kind on every append, which no sibling frame verb does.
#[semio_framework_async_macros::async_test]
async fn appends_frame_two_and_restamps_the_stream_kind() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("add-stream-frame applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "add-stream-frame/appends-a-third-frame-to-stream-a: applied state differs from committed after-snapshot");
    let indices: Vec<u32> = applied.streams[0].frames.iter().map(|frame| frame.index).collect();
    assert_eq!(indices, [0, 1, 2], "the frame is pushed onto the end of the frame list");
    assert_eq!(applied.streams[0].kind, MediaKind::Video, "the payload's kind is stamped back onto the stream");
    assert_eq!(applied.streams[0].frames[2].asset_id, "asset-a", "the frame's asset reference is stored verbatim");
    assert_eq!(applied.streams[1], before().streams[1], "the sibling stream is untouched");
    assert_eq!(applied.assets, before().assets, "add-stream-frame never creates the asset its frame points at");
}

/// ↩️ The inverse is `remove-stream-frame` at the BASE frame count — the index the append lands on.
#[semio_framework_async_macros::async_test]
async fn inverse_removes_the_frame_at_the_base_frame_count() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelingMutation::RemoveStreamFrame(payload)] if payload.id == "stream-a" && payload.frame_index == 2), "add-stream-frame inverts to remove-stream-frame at index 2, the base frame count, got {inverse:?}");
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "add-stream-frame/appends-a-third-frame-to-stream-a: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: frame 2 is not already present, so the `mutation.no-op` warning that
/// fires on an exactly-equal existing frame stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_does_not_trip_the_duplicate_frame_no_op() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "add-stream-frame/appends-a-third-frame-to-stream-a declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a frame not already in the list raises no mutation.no-op, got {:?}", produced.messages());
    let streams = produced.diff().streams.as_ref().expect("add-stream-frame writes the streams field");
    assert_eq!(streams.values[0].frames.len(), 3, "the streams delta carries the full post-append frame list");
    assert!(produced.diff().assets.is_none(), "add-stream-frame writes streams alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-stream-frame/appends-a-third-frame-to-stream-a: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "add-stream-frame/appends-a-third-frame-to-stream-a: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `add-stream-frame` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "add-stream-frame/appends-a-third-frame-to-stream-a: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let streams = committed_diff.streams.as_ref().expect("add-stream-frame's delta is the streams list");
    assert_eq!(streams.values[0].frames.len(), 3, "the committed delta carries stream-a's post-append frame list");
    assert_eq!(streams.values[1].frames.len(), 1, "and repeats the untouched sibling stream verbatim");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-stream-frame/appends-a-third-frame-to-stream-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `add-stream-frame`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "add-stream-frame/appends-a-third-frame-to-stream-a: committed diff did not carry before to after");
}
