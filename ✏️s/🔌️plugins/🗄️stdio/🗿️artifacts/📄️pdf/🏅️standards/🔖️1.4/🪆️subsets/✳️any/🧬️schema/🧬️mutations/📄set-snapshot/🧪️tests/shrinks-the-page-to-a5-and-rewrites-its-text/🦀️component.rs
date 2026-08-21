//! 🧪️ `set-snapshot` fixture — `shrinks-the-page-to-a5-and-rewrites-its-text`.
//!
//! The 1.4 standard is deliberately frozen at a single `PageDoc{width,height,text}` page —
//! no object graph, no page list — so `PdfDiff` is a flat three-scalar patch and
//! `PdfDiff::between` sets exactly the scalars that differ. This payload moves all three at
//! once, which is what makes the fixture prove that `schema` (an identity field with no slot
//! on `PdfDiff` at all) still cannot be reached by a whole-snapshot replacement.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;

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
    assert!(outcome.messages().is_empty(), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.page.width, 420.0, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: the page must narrow to the A5 width");
    assert_eq!(snapshot.page.height, 595.0, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: the page must shorten to the A5 height");
    assert_eq!(snapshot.page.text, "Final", "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: the page text must be rewritten");
    assert_eq!(snapshot.schema, "stdio.pdf", "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: PdfDiff has no schema slot, so the identity field survives a whole-snapshot replacement");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state PdfSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <PdfMutation as protocol::Mutation<PdfSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], PdfMutation::SetSnapshot { .. }), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_pdf_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_pdf_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed PdfSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PdfSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: committed mutation JSON is not canonical");
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
    assert_eq!(produced, declared, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_pdf_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: unknown outcome status {other:?}"),
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
    assert_eq!(produced, committed, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(raised.diff().width, Some(420.0), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: width is one of the three flat scalars this payload moves");
    assert_eq!(raised.diff().height, Some(595.0), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: height is one of the three flat scalars this payload moves");
    assert_eq!(raised.diff().text.as_deref(), Some("Final"), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: text is one of the three flat scalars this payload moves");
}

/// 🔣️ The committed diff is itself canonical and decodes to PdfDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PdfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: committed diff JSON is not canonical");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").as_object().expect("diff is an object").len(),
        3,
        "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: the 1.4 diff has exactly three slots and all three are written here — an extra key would mean the frozen 1.4 model grew a field this fixture never checked"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PdfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/shrinks-the-page-to-a5-and-rewrites-its-text: committed diff did not carry before to after");
}
