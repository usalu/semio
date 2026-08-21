//! 🧪️ `set-snapshot` fixture — `bolds-the-tower-run-of-the-opening-paragraph`.
//!
//! A `DocxSnapshot` is two lanes side by side: the lossless OPC package and the typed
//! `word/document.xml` view. Bolding one run must therefore leave the `opc` slot entirely
//! unset — the package bytes are regenerated on encode, not diffed — and reach through
//! `document.body` (index-keyed) into the paragraph's `runs` (also index-keyed) to name a
//! single `bold` flag. `DocxBlockDiff` is kind-shaped, so the block patch must stay
//! `Paragraph` rather than degrading to `Replace`.
//! `DocxParagraphDiff::style` and `DocxStyleDiff::basedOn` are tri-state `Option<Option<String>>`
//! slots whose `Some(None)` 'cleared' state cannot survive a JSON round trip; neither is
//! written here.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::diff::DocxDiff;
use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::mutations::{apply_docx_mutation, DocxMutation};
use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::snapshot::DocxSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> DocxSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> DocxSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> DocxMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` DocxSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_docx_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: applied state differs from committed after-snapshot");
    let paragraph = match &snapshot.document.body[0] {
        crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::snapshot::DocxBlock::Paragraph(paragraph) => paragraph.clone(),
        other => panic!("set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the opening block must stay a paragraph, got {other:?}"),
    };
    assert!(paragraph.runs[1].bold, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the second run must come back bold");
    assert!(!paragraph.runs[0].bold, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the leading run must stay unbolded");
    assert_eq!(paragraph.runs[1].text, "Capsule Tower", "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: bolding a run must not touch its literal text");
    assert_eq!(paragraph.style.as_deref(), Some("Heading1"), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the paragraph's style reference does not move");
    assert_eq!(snapshot.opc, before().opc, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the OPC package is identical on both sides and must survive untouched");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state DocxSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <DocxMutation as protocol::Mutation<DocxSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], DocxMutation::SetSnapshot { .. }), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_docx_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_docx_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed DocxSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DocxSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <DocxMutation as protocol::Mutation<DocxSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_docx_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in DocxDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <DocxMutation as protocol::Mutation<DocxSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().opc.is_none(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: a typed-document edit must never reach into the lossless OPC lane");
    let document = raised.diff().document.as_ref().expect("set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the document diff must be present");
    assert!(document.styles.is_none(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: word/styles.xml is equal on both sides");
    let body = document.body.as_ref().expect("set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the body triple must be present");
    assert!(body.removed.is_empty() && body.added.is_empty(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the block sequence keeps its length");
    let runs = match &body.modified[0].diff {
        crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::diff::DocxBlockDiff::Paragraph(paragraph) => {
            assert!(paragraph.style.is_none(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the paragraph style is unchanged and must stay absent");
            paragraph.runs.clone().expect("set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the runs triple must be present")
        }
        other => panic!("set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: a run-level edit must keep the kind-shaped Paragraph diff, not degrade to Replace: {other:?}"),
    };
    assert_eq!(runs.modified.len(), 1, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: only the second run is patched");
    assert_eq!(runs.modified[0].index, 1, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: run indices are BASE-state positions");
    assert!(runs.modified[0].diff.text.is_none() && runs.modified[0].diff.italic.is_none() && runs.modified[0].diff.underline.is_none(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: only the bold flag moves");
}

/// 🔣️ The committed diff is itself canonical and decodes to DocxDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: DocxDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: committed diff JSON is not canonical");
    let body = decoded.document.as_ref().expect("document diff").body.as_ref().expect("body triple");
    assert!(
        matches!(&body.modified[0].diff, crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::diff::DocxBlockDiff::Paragraph(paragraph) if paragraph.style.is_none()),
        "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: the tri-state style slot must round-trip as absent — a committed null would collapse the Some(None) 'style cleared' state that Option<Option<String>> cannot express in JSON"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: DocxDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <DocxDiff as protocol::MutationDiff<DocxSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/bolds-the-tower-run-of-the-opening-paragraph: committed diff did not carry before to after");
}
