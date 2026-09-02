//! 🧪️ `set-snapshot` fixture — `retitles-the-id3v2-tit2-frame`.
//!
//! `Mp3Diff` is a three-slot container patch: `id3v2` and `id3v1` are independently nullable
//! tags and therefore tri-state `Option<Option<_>>`, while `frames` is a plain
//! changed-or-not `Option<Vec<Mp3Frame>>` whole-value slot. Retagging touches only the ID3v2
//! side, so the MPEG frame list — whose payload bytes this artifact honestly retains opaque,
//! with no Huffman/MDCT decode — must not appear in the delta at all.
//! The tri-state slots' `Some(None)` 'tag cleared' state serialises to `null` and decodes
//! back as `None`; this payload never clears a tag, so the fixed point holds and the
//! limitation is pinned rather than papered over.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::Mp3Diff;
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::{apply_mp3_mutation, Mp3Mutation};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Mp3Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Mp3Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Mp3Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` Mp3Snapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_mp3_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/retitles-the-id3v2-tit2-frame: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/retitles-the-id3v2-tit2-frame: applied state differs from committed after-snapshot");
    let id3v2 = snapshot.id3v2.as_ref().expect("set-snapshot/retitles-the-id3v2-tit2-frame: the ID3v2 tag must still be present");
    assert_eq!(id3v2.frames[0].id, "TIT2", "set-snapshot/retitles-the-id3v2-tit2-frame: the retagged frame is still the title frame");
    assert_eq!(id3v2.frames[0].data, vec![66u8], "set-snapshot/retitles-the-id3v2-tit2-frame: the TIT2 payload byte must land on 0x42");
    assert_eq!(id3v2.major_version, 3, "set-snapshot/retitles-the-id3v2-tit2-frame: the ID3v2 header version is untouched");
    assert_eq!(snapshot.frames, before().frames, "set-snapshot/retitles-the-id3v2-tit2-frame: the MPEG audio frames are equal on both sides and must survive untouched");
    assert!(snapshot.id3v1.is_none(), "set-snapshot/retitles-the-id3v2-tit2-frame: there is no ID3v1 trailer on either side");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state Mp3Snapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Mp3Mutation as protocol::Mutation<Mp3Snapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/retitles-the-id3v2-tit2-frame: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], Mp3Mutation::SetSnapshot(_)), "set-snapshot/retitles-the-id3v2-tit2-frame: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_mp3_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_mp3_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/retitles-the-id3v2-tit2-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed Mp3Snapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Mp3Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/retitles-the-id3v2-tit2-frame: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/retitles-the-id3v2-tit2-frame: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Mp3Mutation as protocol::Mutation<Mp3Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/retitles-the-id3v2-tit2-frame: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_mp3_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/retitles-the-id3v2-tit2-frame: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/retitles-the-id3v2-tit2-frame: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/retitles-the-id3v2-tit2-frame: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in Mp3Diff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <Mp3Mutation as protocol::Mutation<Mp3Snapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/retitles-the-id3v2-tit2-frame: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(raised.diff().frames.is_none(), "set-snapshot/retitles-the-id3v2-tit2-frame: the MPEG frame list is unchanged and must stay absent — a Some(frames) here would mean the delta re-wrote every opaque audio payload");
    assert!(raised.diff().id3v1.is_none(), "set-snapshot/retitles-the-id3v2-tit2-frame: the ID3v1 slot is unchanged on both sides");
    let id3v2 = raised.diff().id3v2.as_ref().expect("set-snapshot/retitles-the-id3v2-tit2-frame: the ID3v2 slot must be written").as_ref().expect("set-snapshot/retitles-the-id3v2-tit2-frame: the tag is set, not cleared");
    assert_eq!(id3v2.frames[0].data, vec![66u8], "set-snapshot/retitles-the-id3v2-tit2-frame: Id3v2Tag is a whole-record replacement slot, so the delta carries the complete new tag");
}

/// 🔣️ The committed diff is itself canonical and decodes to Mp3Diff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Mp3Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/retitles-the-id3v2-tit2-frame: committed diff JSON is not canonical");
    assert!(
        decoded.id3v1.is_none(),
        "set-snapshot/retitles-the-id3v2-tit2-frame: id3v1 must round-trip as absent — a committed null would be indistinguishable from the Some(None) 'trailer cleared' state that Option<Option<Id3v1Tag>> cannot express in JSON"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Mp3Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Mp3Diff as protocol::MutationDiff<Mp3Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/retitles-the-id3v2-tit2-frame: committed diff did not carry before to after");
}
