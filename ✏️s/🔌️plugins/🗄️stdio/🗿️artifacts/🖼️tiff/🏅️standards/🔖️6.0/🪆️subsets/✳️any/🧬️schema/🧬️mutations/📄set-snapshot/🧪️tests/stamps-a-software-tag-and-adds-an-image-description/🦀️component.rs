//! 🧪️ `📄set-snapshot` fixture — `stamps-a-software-tag-and-adds-an-image-description`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🏷️ The case rewrites the `Software` ASCII entry (tag 305) of the single IFD and inserts a new
//! `ImageDescription` entry (tag 270) ahead of it in the spec-mandated ascending tag order, while
//! the five baseline `Short` tags, the little-endian byte order and the decoded RGBA `pixels` are
//! all untouched — so `TiffDiff::between` must emit a TAG-ID-keyed triple with one `modified` and
//! one `added` entry and nothing else.

use crate::artifacts::tiff::schema::diff::TiffDiff;
use crate::artifacts::tiff::schema::mutations::{apply_tiff_mutation, TiffMutation};
use crate::artifacts::tiff::schema::snapshot::{TiffValues, TAG_IMAGE_WIDTH};
use crate::artifacts::tiff::TiffSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> TiffSnapshot {
    serde_json::from_str(BEFORE).expect("before TIFF snapshot decodes")
}
fn expected_after() -> TiffSnapshot {
    serde_json::from_str(AFTER).expect("after TIFF snapshot decodes")
}
fn mutation() -> TiffMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the single-IFD image to exactly the committed `after`: seven entries,
/// still in ascending tag order.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_tiff_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "tiff/set-snapshot: a genuinely changed image must not raise any message");
    let entries = &snapshot.ifds[0].entries;
    assert_eq!(entries.len(), 7, "tiff/set-snapshot: the new ImageDescription entry must join the six existing ones");
    assert!(entries.windows(2).all(|pair| pair[0].tag < pair[1].tag), "tiff/set-snapshot: TIFF6 §2 requires ascending tag order within an IFD");
    assert_eq!(entries[0].tag, TAG_IMAGE_WIDTH, "tiff/set-snapshot: ImageWidth must remain the first entry");
    assert_eq!(snapshot, expected_after(), "tiff/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must drop tag 270 again and put
/// the original `Software` string back.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <TiffMutation as protocol::Mutation<TiffSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_tiff_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_tiff_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot.ifds[0].entries.len(), 6, "tiff/set-snapshot: the inverse must remove the added ImageDescription entry");
    assert_eq!(snapshot, base, "tiff/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed TIFF snapshots and the mutation are already canonical: `TiffValues` is
/// ADJACENTLY tagged (`kind` + `value`) because its variants wrap arrays and strings rather than
/// structs, and each `TiffTag` therefore carries the field type twice — once as its own `kind`
/// and once inside `values`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: TiffSnapshot = serde_json::from_str(text).expect("TIFF snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("TIFF snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("TIFF snapshot reparses");
        assert_eq!(reencoded, original, "tiff/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "tiff/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the image really moves, so no diagnostic is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "tiff/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_tiff_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "tiff/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "tiff/set-snapshot: an applied set-snapshot must actually move the image");
}

/// 🔺️ The sparse `TiffDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: `byte_order` and the whole-buffer `pixels` slot must stay absent, the
/// five untouched baseline tags must not be re-listed, and the new tag must arrive as `added`
/// rather than as a wholesale IFD replacement.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced TIFF diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed TIFF diff decodes");
    assert_eq!(produced, committed, "tiff/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `TiffDiff`: one modified IFD carrying
/// one modified and one added tag, keyed by TAG ID (not by position).
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: TiffDiff = serde_json::from_str(DIFF).expect("committed TIFF diff decodes");
    assert!(decoded.byte_order.is_none() && decoded.pixels.is_none(), "tiff/set-snapshot: neither the byte order nor the decoded raster may be re-emitted");
    let ifds = decoded.ifds.as_ref().expect("the committed diff carries an ifds triple");
    assert!(ifds.removed.is_empty() && ifds.added.is_empty() && ifds.modified.len() == 1, "tiff/set-snapshot: the single IFD must be patched in place");
    let tags = &ifds.modified[0].diff.entries;
    assert!(tags.removed.is_empty() && tags.modified.len() == 1 && tags.added.len() == 1, "tiff/set-snapshot: exactly one tag is rewritten and exactly one is introduced");
    assert_eq!(tags.modified[0].tag, 305, "tiff/set-snapshot: the rewritten entry is the Software tag");
    assert_eq!(tags.added[0].tag, 270, "tiff/set-snapshot: the introduced entry is the ImageDescription tag");
    assert!(matches!(&tags.added[0].values, TiffValues::Ascii(text) if text == "Two by two swatch"), "tiff/set-snapshot: the added entry must carry its decoded ASCII value, not a byte blob");
    let reencoded = serde_json::to_value(&decoded).expect("TIFF diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed TIFF diff reparses");
    assert_eq!(reencoded, original, "tiff/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — including
/// re-sorting the IFD into ascending tag order, which is what makes the delta complete rather than
/// a summary.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: TiffDiff = serde_json::from_str(DIFF).expect("committed TIFF diff decodes");
    let produced = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&decoded, &before()).expect("committed TIFF diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "tiff/set-snapshot: committed diff did not carry before to after");
}
