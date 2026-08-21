//! 🧪️ `replace-tracks` fixture — `replaces-the-moving-track-with-two-static-tracks`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodel::mutations::{apply_remodel_mutation, inverse_remodel_mutation, RemodelMutation};
use crate::artifacts::remodel::TrackClass;
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

/// ▶️ `tracks` is a plain `Vec`, not an `Option`: the payload list replaces the base list
/// wholesale, so ids present only in the base simply disappear.
#[semio_framework_async_macros::async_test]
async fn replaces_the_track_list_wholesale() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("replace-tracks applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-tracks/replaces-the-moving-track-with-two-static-tracks: applied state differs from committed after-snapshot");
    let ids: Vec<&str> = applied.results.tracks.iter().map(|track| track.id.as_str()).collect();
    assert_eq!(ids, ["track-b", "track-c"], "the base track-a is gone — this is a replace, not a merge");
    assert!(applied.results.tracks.iter().all(|track| track.class == TrackClass::Static), "every replacement track carries the static class");
    assert_eq!(applied.params.motion, before().params.motion, "replacing tracks never enables the motion params that would produce them");
    assert_eq!(applied.results.qc, before().results.qc, "the QC report's own mean-track-length summary is not recomputed");
}

/// ↩️ The inverse is the same verb carrying the captured base list.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_single_moving_track() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::ReplaceTracks(payload)] if payload.tracks.len() == 1 && payload.tracks[0].class == TrackClass::Moving),
        "replace-tracks inverts to itself carrying the captured single moving track, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-tracks/replaces-the-moving-track-with-two-static-tracks: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the payload list differs from the base list, so the `mutation.no-op`
/// warning — this leaf's only guard — stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_has_only_a_no_op_guard() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-tracks/replaces-the-moving-track-with-two-static-tracks declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a genuinely different track list raises no mutation.no-op, got {:?}", produced.messages());
    let results = produced.diff().results.as_ref().expect("replace-tracks writes the results field");
    assert_eq!(results.tracks.len(), 2, "the results delta carries the replacement track list");
    assert!(produced.diff().params.is_none(), "replace-tracks writes results alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-tracks/replaces-the-moving-track-with-two-static-tracks: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-tracks/replaces-the-moving-track-with-two-static-tracks: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-tracks` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-tracks/replaces-the-moving-track-with-two-static-tracks: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let results = committed_diff.results.as_ref().expect("replace-tracks' delta is the whole results block");
    assert_eq!(results.tracks.len(), 2, "the committed delta carries the replacement track list");
    assert_eq!(results.trajectory, before().results.trajectory, "and repeats every results sibling unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-tracks/replaces-the-moving-track-with-two-static-tracks: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-tracks`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-tracks/replaces-the-moving-track-with-two-static-tracks: committed diff did not carry before to after");
}
