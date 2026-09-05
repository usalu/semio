//! 🧪️ `remove-stream-frame` fixture — `🚫️removes-the-last-frame-of-stream-a`.
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

/// ▶️ `frame_index` is a BASE-state position in `frames`, not a `FrameRef.index` value; the
/// removal shifts nothing else and leaves the stream's own kind alone.
#[semio_framework_async_macros::async_test]
async fn removes_the_frame_at_base_position_one() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("remove-stream-frame applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "remove-stream-frame/removes-the-last-frame-of-stream-a: applied state differs from committed after-snapshot");
    let indices: Vec<u32> = applied.streams[0].frames.iter().map(|frame| frame.index).collect();
    assert_eq!(indices, [0], "position 1 is removed, position 0 stays");
    assert_eq!(applied.streams[0].kind, MediaKind::Video, "remove-stream-frame never re-stamps the media kind");
    assert_eq!(applied.streams[1].frames.len(), 1, "the sibling stream's frames are untouched");
    assert_eq!(applied.assets, before().assets, "the asset the removed frame pointed at survives");
}

/// ↩️ The inverse is `add-stream-frame` carrying the captured frame and the stream's BASE kind;
/// because the removed frame was the LAST one, the re-append restores list order exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_re_appends_the_captured_last_frame() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::AddStreamFrame(payload)] if payload.id == "stream-a" && payload.frame.index == 1 && payload.kind == MediaKind::Video),
        "remove-stream-frame inverts to add-stream-frame carrying the captured frame and base kind, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-stream-frame/removes-the-last-frame-of-stream-a: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: index 1 is inside the base frame list, so neither the missing-stream nor
/// the out-of-range `mutation.target-missing` rejection fires.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_in_range_index_check() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "remove-stream-frame/removes-the-last-frame-of-stream-a declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "an in-range frame index on an existing stream raises no mutation.target-missing, got {:?}", produced.messages());
    let streams = produced.diff().streams.as_ref().expect("remove-stream-frame writes the streams field");
    assert_eq!(streams.values[0].frames.len(), 1, "the streams delta carries the full post-removal frame list");
    assert!(produced.diff().gcps.is_none(), "remove-stream-frame writes streams alone — GCP observations keyed to the frame are left dangling");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-stream-frame/removes-the-last-frame-of-stream-a: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-stream-frame/removes-the-last-frame-of-stream-a: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `remove-stream-frame` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "remove-stream-frame/removes-the-last-frame-of-stream-a: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let streams = committed_diff.streams.as_ref().expect("remove-stream-frame's delta is the streams list");
    assert_eq!(streams.values[0].frames.len(), 1, "the committed delta carries stream-a's post-removal frame list");
    assert_eq!(streams.values[0].frames[0].index, 0, "the survivor is the frame that sat at base position 0");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-stream-frame/removes-the-last-frame-of-stream-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `remove-stream-frame`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "remove-stream-frame/removes-the-last-frame-of-stream-a: committed diff did not carry before to after");
}
