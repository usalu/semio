//! 🧪️ `set-snapshot` fixture — `🧫️promotes-the-second-sample-to-a-sync-frame`.
//!
//! ISOBMFF sample payloads are container-typed and payload-opaque here (AVCC length-prefixed
//! bytes, never decoded), so `Mp4SampleDiff` treats `data` as a whole-value slot and keeps
//! `duration`/`ctsOffset`/`sync` as independent scalars. Flipping the `stss` sync flag of one
//! sample must therefore produce exactly one scalar inside a two-level index-keyed chain
//! (tracks → samples), and must not restate the sample's bytes.
//! `Mp4Diff::ftyp` and `Mp4Diff::movie` are whole-record replacement slots for the `ftyp` and
//! `mvhd` boxes; both stay unset here, which is what proves a sample-level edit cannot reach
//! the movie header. Sample bytes serialise as JSON number arrays — `#[dsl(base64)]` governs
//! only the op/DSL codec.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::diff::Mp4Diff;
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::{apply_mp4_mutation, Mp4Mutation};
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Mp4Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Mp4Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Mp4Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` Mp4Snapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_mp4_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: applied state differs from committed after-snapshot");
    assert!(snapshot.tracks[0].samples[1].sync, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the second sample must come back marked as a sync (random-access) sample");
    assert_eq!(snapshot.tracks[0].samples[1].data, vec![0u8, 0, 0, 1, 97], "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the opaque AVCC payload must not be rewritten by an stss flag flip");
    assert_eq!(snapshot.tracks[0].samples[0], before().tracks[0].samples[0], "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the first sample is identical on both sides and must survive untouched");
    assert_eq!(snapshot.tracks[0].chunk_sample_counts, vec![2u32], "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the chunk grouping does not change");
    assert_eq!(snapshot.movie.timescale, 1000, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the mvhd timescale does not move");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state Mp4Snapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], Mp4Mutation::SetSnapshot(_)), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_mp4_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_mp4_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed Mp4Snapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Mp4Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_mp4_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/promotes-the-second-sample-to-a-sync-frame: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in Mp4Diff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(raised.diff().ftyp.is_none(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the ftyp brand record is equal on both sides and must stay absent");
    assert!(raised.diff().movie.is_none(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: a sample-level edit must never reach the mvhd whole-record slot");
    let tracks = raised.diff().tracks.as_ref().expect("set-snapshot/promotes-the-second-sample-to-a-sync-frame: the tracks triple must be present");
    assert!(tracks.removed.is_empty() && tracks.added.is_empty(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the file keeps its single video track");
    let track = &tracks.modified[0].diff;
    assert!(
        track.codec.is_none() && track.metadata.is_none() && track.chunk_sample_counts.is_none(),
        "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the avcC record, the track metadata and the chunk grouping are all unchanged whole-value slots"
    );
    assert!(track.width.is_none() && track.height.is_none() && track.track_id.is_none(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: the track's own scalars do not move");
    let samples = track.samples.as_ref().expect("set-snapshot/promotes-the-second-sample-to-a-sync-frame: the samples triple must be present");
    assert_eq!(samples.modified[0].index, 1, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: sample indices are BASE-state positions");
    assert_eq!(samples.modified[0].diff.sync, Some(true), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: sync is the one scalar this payload moves");
    assert!(
        samples.modified[0].diff.data.is_none() && samples.modified[0].diff.duration.is_none(),
        "set-snapshot/promotes-the-second-sample-to-a-sync-frame: a Some(data) here would mean the delta re-emitted the whole opaque payload for a one-bit flag change"
    );
}

/// 🔣️ The committed diff is itself canonical and decodes to Mp4Diff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Mp4Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/promotes-the-second-sample-to-a-sync-frame: committed diff JSON is not canonical");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").pointer("/tracks/modified/0/diff/samples/modified/0/diff").and_then(serde_json::Value::as_object).map(|o| o.len()),
        Some(1),
        "set-snapshot/promotes-the-second-sample-to-a-sync-frame: exactly one of Mp4SampleDiff's four slots may appear in the committed sample patch"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Mp4Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Mp4Diff as protocol::MutationDiff<Mp4Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/promotes-the-second-sample-to-a-sync-frame: committed diff did not carry before to after");
}
