//! 🧪️ `set-frame-delay` fixture — `⏱️slows`.
//!
//! 89a's whole reason for existing over 87a is the Graphic Control Extension, and this
//! fixture edits exactly that: `delayCs` is per-frame GCE state, so
//! the sparse frame diff must name it alone and leave the frame's palette indices, its
//! rectangle and its (absent) local colour table alone. The NETSCAPE2.0 loop count is
//! modelled as `loopCount` rather than as a raw application extension, so it must not appear
//! in `appExtensions` either.
//! `loopCount`, `gct`, `lct`, `transparentIndex` and `plainText` are all tri-state
//! `Option<Option<_>>` slots whose `Some(None)` 'cleared' state cannot survive a JSON round
//! trip; none of them is written here, so the committed diff's fixed point holds.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::standards::v89a::subsets::any::schema::diff::GifDiff;
use crate::standards::v89a::subsets::any::schema::mutations::{apply_gif_mutation, set_frame_delay, GifMutation};

use crate::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️set-frame-delay/⏱️slows/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️set-frame-delay/⏱️slows/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️set-frame-delay/⏱️slows/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️set-frame-delay/⏱️slows/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️set-frame-delay/⏱️slows/🎯️outcome/🔣️.json");

fn before() -> GifSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> GifSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> GifMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `set-frame-delay` carries the committed `before` GifSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_gif_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-frame-delay/slows-the-second-frame: set-frame-delay raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-frame-delay/slows-the-second-frame: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.frames[1].delay_cs, 25, "set-frame-delay/slows-the-second-frame: the second frame must hold for 25 hundredths of a second");
    assert_eq!(snapshot.frames[1].disposal, before().frames[1].disposal, "set-frame-delay/slows-the-second-frame: a delay edit leaves the GCE disposal method alone");
    assert_eq!(snapshot.frames[1].indices, vec![1u8, 0], "set-frame-delay/slows-the-second-frame: the frame's palette indices are unchanged by a timing edit");
    assert_eq!(snapshot.frames[0], before().frames[0], "set-frame-delay/slows-the-second-frame: the first frame is identical on both sides and must survive untouched");
    assert_eq!(snapshot.loop_count, Some(0), "set-frame-delay/slows-the-second-frame: the NETSCAPE2.0 loop count still says loop forever");
    assert!(snapshot.app_extensions.is_empty(), "set-frame-delay/slows-the-second-frame: NETSCAPE2.0 is modelled as loopCount, so it must never show up as a raw application extension");
}

/// ↩️ `set-frame-delay`'s inverse is a single `SetFrameDelay` carrying the pre-state delay back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <GifMutation as protocol::Mutation<GifSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "set-frame-delay/slows-the-second-frame: undoing a delay edit is exactly one step");
    assert_eq!(inverse[0], GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 1, delay_cs: base.frames[1].delay_cs }), "set-frame-delay/slows-the-second-frame: the undo step must itself be a SetFrameDelay carrying the pre-state delay");
    let mut snapshot = base.clone();
    apply_gif_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_gif_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-frame-delay/slows-the-second-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed GifSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GifSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-frame-delay/slows-the-second-frame: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-frame-delay/slows-the-second-frame: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <GifMutation as protocol::Mutation<GifSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-frame-delay/slows-the-second-frame: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_gif_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-frame-delay/slows-the-second-frame: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-frame-delay/slows-the-second-frame: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-frame-delay/slows-the-second-frame: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: the delta names only the one frame row and the one field that differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <GifMutation as protocol::Mutation<GifSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-frame-delay/slows-the-second-frame: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(raised.diff().loop_count.is_none(), "set-frame-delay/slows-the-second-frame: the loop count is equal on both sides");
    assert!(raised.diff().comments.is_none() && raised.diff().app_extensions.is_none(), "set-frame-delay/slows-the-second-frame: neither weak collection changes, so neither may produce a triple");
    assert!(raised.diff().gct.is_none() && raised.diff().width.is_none() && raised.diff().height.is_none(), "set-frame-delay/slows-the-second-frame: the logical screen descriptor and its colour table do not move");
    let frames = raised.diff().frames.as_ref().expect("set-frame-delay/slows-the-second-frame: the frames triple must be present");
    assert!(frames.removed.is_empty() && frames.added.is_empty(), "set-frame-delay/slows-the-second-frame: retiming a frame never adds or drops one");
    assert_eq!(frames.modified[0].index, 1, "set-frame-delay/slows-the-second-frame: GifFrameModified indices are BASE-state indices");
    assert_eq!(frames.modified[0].diff.delay_cs, Some(25), "set-frame-delay/slows-the-second-frame: delayCs is the one GCE field this payload moves");
    assert!(frames.modified[0].diff.indices.is_none(), "set-frame-delay/slows-the-second-frame: the pixel buffer is unchanged — a Some(indices) here would mean a GCE-only edit rewrote the whole frame payload");
    assert!(frames.modified[0].diff.transparent_index.is_none() && frames.modified[0].diff.plain_text.is_none(), "set-frame-delay/slows-the-second-frame: the other GCE-adjacent slots are unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes to GifDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GifDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-frame-delay/slows-the-second-frame: committed diff JSON is not canonical");
    assert!(decoded.loop_count.is_none() && decoded.gct.is_none(), "set-frame-delay/slows-the-second-frame: the tri-state loopCount/gct slots must round-trip as absent — a committed null would collapse the Some(None) 'extension dropped' state that Option<Option<_>> cannot express in JSON");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-frame-delay` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GifDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-frame-delay/slows-the-second-frame: committed diff did not carry before to after");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff, and replaying them restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
