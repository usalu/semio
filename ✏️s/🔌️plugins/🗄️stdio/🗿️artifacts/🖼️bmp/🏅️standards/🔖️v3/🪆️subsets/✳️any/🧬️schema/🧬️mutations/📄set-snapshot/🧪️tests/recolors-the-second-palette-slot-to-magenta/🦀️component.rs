//! 🧪️ `📄set-snapshot` fixture — `recolors-the-second-palette-slot-to-magenta`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎨️ The case repaints colour-table slot 1 of a 2×2 8-bit BMP from green to magenta and updates
//! the two canonical-RGBA pixels that reference it. Every BITMAPINFOHEADER field — `headerSize`,
//! the 2835 px/m resolution pair, `colorsUsed`, and the bottom-up `rowOrder` — stays put, so
//! `BmpDiff::between` must emit exactly the `palette` triple plus the whole-buffer `pixels`.

use crate::artifacts::bmp::schema::diff::BmpDiff;
use crate::artifacts::bmp::schema::mutations::{apply_bmp_mutation, BmpMutation};
use crate::artifacts::bmp::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
use crate::artifacts::bmp::BmpSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> BmpSnapshot {
    serde_json::from_str(BEFORE).expect("before BMP snapshot decodes")
}
fn expected_after() -> BmpSnapshot {
    serde_json::from_str(AFTER).expect("after BMP snapshot decodes")
}
fn mutation() -> BmpMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the swatch to exactly the committed `after`: slot 1 is magenta and the
/// two pixels that index it follow.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_bmp_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "bmp/set-snapshot: a genuinely changed bitmap must not raise any message");
    assert_eq!(snapshot.palette[1], BmpPaletteEntry { b: 255, g: 0, r: 255, reserved: 0 }, "bmp/set-snapshot: palette slot 1 must become magenta in on-disk BGRA field order");
    assert_eq!(snapshot.row_order, BmpRowOrder::BottomUp, "bmp/set-snapshot: a palette edit must not flip the signed-height row order");
    assert_eq!(snapshot, expected_after(), "bmp/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must put green back in slot 1 and
/// restore the original 16-byte RGBA buffer.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <BmpMutation as protocol::Mutation<BmpSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_bmp_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_bmp_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "bmp/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed BMP snapshots and the mutation are already canonical: every
/// BITMAPINFOHEADER field is emitted (no `skip_serializing_if` on the snapshot), `BmpRowOrder`
/// is a plain unit-variant enum written as `"bottomUp"`, and a palette entry keeps its on-disk
/// `b`/`g`/`r`/`reserved` order.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: BmpSnapshot = serde_json::from_str(text).expect("BMP snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("BMP snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("BMP snapshot reparses");
        assert_eq!(reencoded, original, "bmp/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "bmp/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the bitmap really moves, so no diagnostic is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "bmp/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_bmp_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "bmp/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "bmp/set-snapshot: an applied set-snapshot must actually move the bitmap");
}

/// 🔺️ The sparse `BmpDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: all twelve header scalars must stay absent, slot 0 must not appear in
/// `palette.modified`, and the palette must be patched rather than removed and re-added.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <BmpMutation as protocol::Mutation<BmpSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced BMP diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed BMP diff decodes");
    assert_eq!(produced, committed, "bmp/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `BmpDiff`: a one-entry palette patch
/// plus the replacement pixel buffer, and no header field at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: BmpDiff = serde_json::from_str(DIFF).expect("committed BMP diff decodes");
    assert!(decoded.width.is_none() && decoded.height.is_none() && decoded.bits_per_pixel.is_none() && decoded.colors_used.is_none() && decoded.row_order.is_none(), "bmp/set-snapshot: no BITMAPINFOHEADER field may be re-emitted");
    let palette = decoded.palette.as_ref().expect("the committed diff carries a palette triple");
    assert!(palette.removed.is_empty() && palette.added.is_empty() && palette.modified.len() == 1 && palette.modified[0].index == 1, "bmp/set-snapshot: exactly palette slot 1 may be patched in place");
    let reencoded = serde_json::to_value(&decoded).expect("BMP diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed BMP diff reparses");
    assert_eq!(reencoded, original, "bmp/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the palette
/// entry plus pixel buffer is a complete description of the recolor, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: BmpDiff = serde_json::from_str(DIFF).expect("committed BMP diff decodes");
    let produced = <BmpDiff as protocol::MutationDiff<BmpSnapshot>>::apply(&decoded, &before()).expect("committed BMP diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "bmp/set-snapshot: committed diff did not carry before to after");
}
