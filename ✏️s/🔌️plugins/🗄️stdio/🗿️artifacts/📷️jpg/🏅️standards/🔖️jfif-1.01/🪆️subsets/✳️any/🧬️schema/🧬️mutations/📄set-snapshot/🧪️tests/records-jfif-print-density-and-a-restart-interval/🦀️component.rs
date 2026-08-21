//! 🧪️ `📄set-snapshot` fixture — `records-jfif-print-density-and-a-restart-interval`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 📏️ The case rewrites the JFIF APP0 density triple (aspect 1×1 → 72×72 pixels-per-inch) and
//! turns on a `DRI` restart interval of 4 MCU rows, while `jfif_version`, the verbatim-retained
//! COM segment and the (still-undecoded) `frame`/`sof_marker` stay put — so `JpgDiff::between`
//! must emit exactly those four scalars.

use crate::artifacts::jpg::schema::diff::JpgDiff;
use crate::artifacts::jpg::schema::mutations::{apply_jpg_mutation, JpgMutation};
use crate::artifacts::jpg::schema::snapshot::JfifDensityUnits;
use crate::artifacts::jpg::JpgSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> JpgSnapshot {
    serde_json::from_str(BEFORE).expect("before JPG snapshot decodes")
}
fn expected_after() -> JpgSnapshot {
    serde_json::from_str(AFTER).expect("after JPG snapshot decodes")
}
fn mutation() -> JpgMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the swatch to exactly the committed `after`: JFIF APP0 now declares an
/// absolute 72 dpi resolution and the DRI segment appears.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_jpg_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "jpg/set-snapshot: a genuinely changed snapshot must not raise any message");
    assert_eq!(snapshot.jfif_density_units, JfifDensityUnits::PixelsPerInch, "jpg/set-snapshot: the APP0 density unit must become an absolute resolution");
    assert_eq!((snapshot.jfif_x_density, snapshot.jfif_y_density), (72, 72), "jpg/set-snapshot: both APP0 density axes must become 72 dpi");
    assert_eq!(snapshot.restart_interval, Some(4), "jpg/set-snapshot: the DRI restart interval must be introduced");
    assert_eq!(snapshot, expected_after(), "jpg/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must put the APP0 block back to
/// the spec-default 1×1 aspect ratio and drop the DRI segment again.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <JpgMutation as protocol::Mutation<JpgSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_jpg_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_jpg_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot.restart_interval, None, "jpg/set-snapshot: the inverse must remove the DRI segment again");
    assert_eq!(snapshot, base, "jpg/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed JPG snapshots and the mutation are already canonical: `jfif_version` is a
/// `(u8, u8)` tuple and must be spelled as a two-element array, and `pixels`/`other_segments[].data`
/// as plain arrays of byte numbers rather than base64 strings.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JpgSnapshot = serde_json::from_str(text).expect("JPG snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("JPG snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("JPG snapshot reparses");
        assert_eq!(reencoded, original, "jpg/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "jpg/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the payload genuinely differs from the base, so no
/// `mutation.no-op` warning is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "jpg/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_jpg_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "jpg/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "jpg/set-snapshot: an applied set-snapshot must actually move the snapshot");
}

/// 🔺️ The sparse `JpgDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only the three APP0 density scalars and `restart_interval` move, and
/// that the id-keyed `quant_tables`/`huffman_tables` triples and the index-keyed `other_segments`
/// triple stay absent instead of being re-emitted wholesale.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <JpgMutation as protocol::Mutation<JpgSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced JPG diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed JPG diff decodes");
    assert_eq!(produced, committed, "jpg/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `JpgDiff`: the tri-state
/// `restart_interval: Option<Option<u16>>` must survive as `Some(Some(4))`, i.e. "DRI set", never
/// "DRI cleared".
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: JpgDiff = serde_json::from_str(DIFF).expect("committed JPG diff decodes");
    assert_eq!(decoded.restart_interval, Some(Some(4)), "jpg/set-snapshot: the committed diff must set the DRI interval, not clear it");
    assert!(decoded.frame.is_none() && decoded.quant_tables.is_none() && decoded.huffman_tables.is_none() && decoded.other_segments.is_none(), "jpg/set-snapshot: the committed diff must leave frame and every table collection untouched");
    let reencoded = serde_json::to_value(&decoded).expect("JPG diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed JPG diff reparses");
    assert_eq!(reencoded, original, "jpg/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the delta is
/// a complete description of the APP0 + DRI change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: JpgDiff = serde_json::from_str(DIFF).expect("committed JPG diff decodes");
    let produced = <JpgDiff as protocol::MutationDiff<JpgSnapshot>>::apply(&decoded, &before()).expect("committed JPG diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "jpg/set-snapshot: committed diff did not carry before to after");
}
