//! 🧪️ `set-snapshot` fixture — `🖼️repaints-the-right-pixel-of-the-single-image`.
//!
//! GIF87a has no Graphic Control Extension at all — no delay, no disposal, no transparency —
//! so a `GifImage` is a screen sub-rectangle plus its losslessly retained PALETTE INDICES
//! (never decoded RGBA; `rgba()` is a derived accessor, not stored state). Repainting one
//! pixel therefore has to reduce to a single `GifImageModified` whose patch names `indices`
//! and nothing else, with the two-entry global colour table left completely alone.
//! `GifDiff::gct` and `GifImageDiff::lct` are tri-state `Option<Option<GifColorTable>>`
//! slots: their `Some(None)` 'table dropped' state serialises to `null` and decodes back as
//! `None`, so this fixture keeps both absent and pins that limitation instead.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::diff::GifDiff;
use crate::artifacts::gif::standards::v87a::subsets::any::schema::mutations::{apply_gif_mutation, GifMutation};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> GifSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> GifSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> GifMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` GifSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_gif_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.images[0].indices, vec![0u8, 1], "set-snapshot/repaints-the-right-pixel-of-the-single-image: the right pixel must switch to palette entry 1");
    assert_eq!(snapshot.gct, before().gct, "set-snapshot/repaints-the-right-pixel-of-the-single-image: the global colour table is identical on both sides and must survive untouched");
    assert_eq!(snapshot.images[0].width, 2, "set-snapshot/repaints-the-right-pixel-of-the-single-image: the image rectangle does not move");
    assert!(snapshot.images[0].lct.is_none(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: this image has no local colour table on either side");
    assert_eq!(
        snapshot.images[0].rgba(snapshot.gct.as_ref()),
        vec![0u8, 0, 0, 255, 255, 255, 255, 255],
        "set-snapshot/repaints-the-right-pixel-of-the-single-image: the derived RGBA view must show black then white — 87a has no transparency, so every pixel stays fully opaque"
    );
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state GifSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <GifMutation as protocol::Mutation<GifSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/repaints-the-right-pixel-of-the-single-image: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], GifMutation::SetSnapshot(_)), "set-snapshot/repaints-the-right-pixel-of-the-single-image: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_gif_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_gif_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/repaints-the-right-pixel-of-the-single-image: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed GifSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GifSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/repaints-the-right-pixel-of-the-single-image: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/repaints-the-right-pixel-of-the-single-image: committed mutation JSON is not canonical");
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
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/repaints-the-right-pixel-of-the-single-image: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_gif_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/repaints-the-right-pixel-of-the-single-image: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in GifDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <GifMutation as protocol::Mutation<GifSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/repaints-the-right-pixel-of-the-single-image: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(raised.diff().width.is_none() && raised.diff().height.is_none(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: the logical screen descriptor does not move");
    assert!(raised.diff().gct.is_none(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: the global colour table is equal on both sides and must stay out of the delta");
    assert!(raised.diff().background_color_index.is_none() && raised.diff().pixel_aspect_ratio.is_none(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: neither screen-descriptor byte moves");
    let images = raised.diff().images.as_ref().expect("set-snapshot/repaints-the-right-pixel-of-the-single-image: the images triple must be present");
    assert!(images.removed.is_empty() && images.added.is_empty(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: repainting a pixel never adds or drops an image block");
    assert_eq!(images.modified[0].index, 0, "set-snapshot/repaints-the-right-pixel-of-the-single-image: GifImageModified indices are BASE-state indices");
    assert_eq!(images.modified[0].diff.indices.as_deref(), Some(&[0u8, 1][..]), "set-snapshot/repaints-the-right-pixel-of-the-single-image: the palette-index buffer is a whole-value slot — GIF87a pixel data is never sub-diffed per pixel");
    assert!(images.modified[0].diff.lct.is_none() && images.modified[0].diff.width.is_none(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: the image's own rectangle and (absent) local table must not be rewritten");
}

/// 🔣️ The committed diff is itself canonical and decodes to GifDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GifDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/repaints-the-right-pixel-of-the-single-image: committed diff JSON is not canonical");
    assert!(decoded.gct.is_none(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: the tri-state gct slot must round-trip as absent — a committed null would collapse the Some(None) 'global table dropped' state that Option<Option<GifColorTable>> cannot express in JSON");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GifDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <GifDiff as protocol::MutationDiff<GifSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/repaints-the-right-pixel-of-the-single-image: committed diff did not carry before to after");
}
