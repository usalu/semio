//! 🧪️ `📄set-snapshot` fixture — `retunes-gamma-and-repaints-the-second-pixel`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎨️ The case swaps the `gAMA` value (100000 → 45455, i.e. 1.0 → the sRGB 1/2.2 gamma) and
//! repaints the second RGBA pixel from blue to green, while leaving IHDR, the single `tEXt`
//! chunk and the five-slot `chunk_order` untouched — so `PngDiff::between` must emit exactly
//! `gama` + `pixels` and nothing else.

use crate::artifacts::png::schema::diff::PngDiff;
use crate::artifacts::png::schema::mutations::{apply_png_mutation, PngMutation};
use crate::artifacts::png::PngSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> PngSnapshot {
    serde_json::from_str(BEFORE).expect("before PNG snapshot decodes")
}
fn expected_after() -> PngSnapshot {
    serde_json::from_str(AFTER).expect("after PNG snapshot decodes")
}
fn mutation() -> PngMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the two-pixel swatch to exactly the committed `after`: gAMA 45455
/// and a green second pixel.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_png_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "png/set-snapshot: a genuinely changed snapshot must not raise any message");
    assert_eq!(snapshot.gama, Some(45455), "png/set-snapshot: gAMA must be retuned to the sRGB 1/2.2 value");
    assert_eq!(snapshot.pixels, vec![255, 0, 0, 255, 0, 255, 0, 255], "png/set-snapshot: the second RGBA pixel must be repainted green");
    assert_eq!(snapshot, expected_after(), "png/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must restore gAMA 100000 and the
/// blue second pixel byte-for-byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <PngMutation as protocol::Mutation<PngSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_png_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_png_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "png/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed PNG snapshots and the mutation are already canonical: `PngSnapshot` emits
/// every IHDR/ancillary field (no `skip_serializing_if`), so `null` ancillaries must be spelled
/// out and `pixels` must be a plain array of byte numbers, not a base64 string.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PngSnapshot = serde_json::from_str(text).expect("PNG snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("PNG snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("PNG snapshot reparses");
        assert_eq!(reencoded, original, "png/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "png/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — a set-snapshot whose payload really differs from the
/// base never raises the `mutation.no-op` warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "png/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_png_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "png/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "png/set-snapshot: an applied set-snapshot must actually move the snapshot");
}

/// 🔺️ The sparse `PngDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `gama` and `pixels` are touched, and that `textChunks`,
/// `chunkOrder`, `plte` and the IHDR scalars stay absent from the delta rather than being
/// rewritten wholesale.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <PngMutation as protocol::Mutation<PngSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced PNG diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed PNG diff decodes");
    assert_eq!(produced, committed, "png/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `PngDiff`: its tri-state
/// `gama: Option<Option<u32>>` must survive the round trip as `Some(Some(45455))`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PngDiff = serde_json::from_str(DIFF).expect("committed PNG diff decodes");
    assert_eq!(decoded.gama, Some(Some(45455)), "png/set-snapshot: the committed diff must set gAMA, not clear it");
    assert!(decoded.text_chunks.is_none() && decoded.chunk_order.is_none(), "png/set-snapshot: the committed diff must leave the text and chunk-order collections untouched");
    let reencoded = serde_json::to_value(&decoded).expect("PNG diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed PNG diff reparses");
    assert_eq!(reencoded, original, "png/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the delta is
/// a complete description of the gamma + pixel change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PngDiff = serde_json::from_str(DIFF).expect("committed PNG diff decodes");
    let produced = <PngDiff as protocol::MutationDiff<PngSnapshot>>::apply(&decoded, &before()).expect("committed PNG diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "png/set-snapshot: committed diff did not carry before to after");
}
