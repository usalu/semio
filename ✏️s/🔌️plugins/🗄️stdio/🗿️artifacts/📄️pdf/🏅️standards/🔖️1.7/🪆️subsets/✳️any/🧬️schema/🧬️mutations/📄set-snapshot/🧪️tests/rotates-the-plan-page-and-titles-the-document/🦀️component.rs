//! 🧪️ `set-snapshot` fixture — `rotates-the-plan-page-and-titles-the-document`.
//!
//! 1.7 is the full model, not 1.4's frozen single page: `pages` is the resolved editable view
//! (index-keyed), `objects` is the whole logical COS graph, and `trailer` reuses the dict
//! triple. `PdfInfo` is a whole-record replacement slot, so stamping a title re-states the
//! entire `/Info` dictionary while the page edit stays a sparse one-scalar patch — the
//! committed diff has to show both shapes side by side, and must leave `objects`/`trailer`
//! untouched.
//! Encoding note: `PdfObject` is internally tagged with newtype variants over non-map
//! payloads (`Int`, `Name`, `Array`, `Dict`, …), which serde cannot serialize, so this
//! fixture deliberately keeps the COS graph and trailer empty and exercises only the typed
//! page/info lanes.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> PdfSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> PdfSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> PdfMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` PdfSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_pdf_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.pages[1].rotate, 90, "set-snapshot/rotates-the-plan-page-and-titles-the-document: the plan page must land quarter-turned");
    assert_eq!(snapshot.pages[1].media_box, [0.0, 0.0, 612.0, 792.0], "set-snapshot/rotates-the-plan-page-and-titles-the-document: rotating a page never rewrites its MediaBox");
    assert_eq!(snapshot.pages[0], before().pages[0], "set-snapshot/rotates-the-plan-page-and-titles-the-document: the cover page is identical on both sides and must survive untouched");
    assert_eq!(snapshot.info.title.as_deref(), Some("Capsule Tower"), "set-snapshot/rotates-the-plan-page-and-titles-the-document: the /Info title must be stamped");
    assert!(snapshot.info.author.is_none(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: the other /Info fields stay unset");
    assert_eq!(snapshot.declared_version, "1.7", "set-snapshot/rotates-the-plan-page-and-titles-the-document: the header version token does not move");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state PdfSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <PdfMutation as protocol::Mutation<PdfSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/rotates-the-plan-page-and-titles-the-document: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], PdfMutation::SetSnapshot { .. }), "set-snapshot/rotates-the-plan-page-and-titles-the-document: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_pdf_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_pdf_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/rotates-the-plan-page-and-titles-the-document: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed PdfSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PdfSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/rotates-the-plan-page-and-titles-the-document: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/rotates-the-plan-page-and-titles-the-document: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <PdfMutation as protocol::Mutation<PdfSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/rotates-the-plan-page-and-titles-the-document: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_pdf_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/rotates-the-plan-page-and-titles-the-document: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in PdfDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <PdfMutation as protocol::Mutation<PdfSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/rotates-the-plan-page-and-titles-the-document: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().declared_version.is_none(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: the declared PDF version is equal on both sides");
    assert!(raised.diff().objects.is_none() && raised.diff().trailer.is_none(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: the COS object graph and the trailer dictionary are untouched and must not produce triples");
    assert_eq!(
        raised.diff().info.as_ref().expect("info slot").title.as_deref(),
        Some("Capsule Tower"),
        "set-snapshot/rotates-the-plan-page-and-titles-the-document: PdfInfo is a whole-record slot, so the delta carries the complete new /Info dictionary"
    );
    let pages = raised.diff().pages.as_ref().expect("set-snapshot/rotates-the-plan-page-and-titles-the-document: the pages triple must be present");
    assert!(pages.removed.is_empty() && pages.added.is_empty(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: the page count does not change");
    assert_eq!(pages.modified[0].index, 1, "set-snapshot/rotates-the-plan-page-and-titles-the-document: PdfPageModified indices are BASE-state positions");
    assert_eq!(pages.modified[0].diff.rotate, Some(90), "set-snapshot/rotates-the-plan-page-and-titles-the-document: the page patch is genuinely sparse — rotate only");
    assert!(pages.modified[0].diff.media_box.is_none() && pages.modified[0].diff.text.is_none(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: the page's box and extracted text must not be rewritten");
}

/// 🔣️ The committed diff is itself canonical and decodes to PdfDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PdfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/rotates-the-plan-page-and-titles-the-document: committed diff JSON is not canonical");
    assert!(
        decoded.pages.as_ref().expect("pages triple").modified[0].diff.crop_box.is_none(),
        "set-snapshot/rotates-the-plan-page-and-titles-the-document: the tri-state cropBox slot must round-trip as absent — a committed null would collapse the Some(None) 'CropBox cleared' state that Option<Option<[f64; 4]>> cannot express in JSON"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PdfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/rotates-the-plan-page-and-titles-the-document: committed diff did not carry before to after");
}
