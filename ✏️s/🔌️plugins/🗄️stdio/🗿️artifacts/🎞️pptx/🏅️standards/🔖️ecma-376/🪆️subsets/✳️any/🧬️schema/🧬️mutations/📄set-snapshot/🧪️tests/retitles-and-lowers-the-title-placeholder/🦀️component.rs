//! 🧪️ `set-snapshot` fixture — `retitles-and-lowers-the-title-placeholder`.
//!
//! Presentations are ORDERED, so slides and shapes are both index-keyed, and the delta walks
//! slide 0 → shape 0 → textFrame paragraph 0 → run 0 before it can name the changed text.
//! `PptxTransform` is a weak value entity, so moving the placeholder replaces the whole EMU
//! rectangle in one slot instead of patching `y` alone — this fixture pins that asymmetry
//! against the run-level edit happening in the same shape.
//! Two tagging details this artifact does not share with its OPC siblings: the shape enum's
//! internal tag is `shapeKind` (because `Placeholder` already owns a field literally named
//! `kind`), while the shape DIFF enum's tag is plain `kind`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::diff::PptxDiff;
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::mutations::{apply_pptx_mutation, PptxMutation};
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::snapshot::PptxSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> PptxSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> PptxSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> PptxMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` PptxSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_pptx_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/retitles-and-lowers-the-title-placeholder: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/retitles-and-lowers-the-title-placeholder: applied state differs from committed after-snapshot");
    match &snapshot.presentation.slides[0].shapes[0] {
        crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::snapshot::PptxShape::Placeholder { kind, text_frame, position } => {
            assert_eq!(kind, "title", "set-snapshot/retitles-and-lowers-the-title-placeholder: the placeholder type attribute is untouched");
            assert_eq!(text_frame[0].runs[0].text, "Nakagin Capsule Tower", "set-snapshot/retitles-and-lowers-the-title-placeholder: the title run must carry the full tower name");
            assert_eq!(position.y, 457200, "set-snapshot/retitles-and-lowers-the-title-placeholder: the placeholder must drop by half an inch in EMU");
            assert_eq!(position.cx, 9144000, "set-snapshot/retitles-and-lowers-the-title-placeholder: the rest of the transform rides along unchanged inside the whole-value slot");
        }
        other => panic!("set-snapshot/retitles-and-lowers-the-title-placeholder: shape 0 must stay a placeholder, got {other:?}"),
    }
    assert_eq!(snapshot.presentation.slides[0].shapes[1], before().presentation.slides[0].shapes[1], "set-snapshot/retitles-and-lowers-the-title-placeholder: the picture shape is identical on both sides and must survive untouched");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state PptxSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <PptxMutation as protocol::Mutation<PptxSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/retitles-and-lowers-the-title-placeholder: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], PptxMutation::SetSnapshot { .. }), "set-snapshot/retitles-and-lowers-the-title-placeholder: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_pptx_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_pptx_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/retitles-and-lowers-the-title-placeholder: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed PptxSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PptxSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/retitles-and-lowers-the-title-placeholder: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/retitles-and-lowers-the-title-placeholder: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <PptxMutation as protocol::Mutation<PptxSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/retitles-and-lowers-the-title-placeholder: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_pptx_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/retitles-and-lowers-the-title-placeholder: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/retitles-and-lowers-the-title-placeholder: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/retitles-and-lowers-the-title-placeholder: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in PptxDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <PptxMutation as protocol::Mutation<PptxSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/retitles-and-lowers-the-title-placeholder: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().opc.is_none() && raised.diff().xml_parts.is_none(), "set-snapshot/retitles-and-lowers-the-title-placeholder: a presentation-level edit must reach neither the OPC lane nor the retained logical XML parts");
    let slides = raised
        .diff()
        .presentation
        .as_ref()
        .expect("set-snapshot/retitles-and-lowers-the-title-placeholder: the presentation diff must be present")
        .slides
        .as_ref()
        .expect("set-snapshot/retitles-and-lowers-the-title-placeholder: the slides triple must be present");
    assert!(slides.removed.is_empty() && slides.added.is_empty(), "set-snapshot/retitles-and-lowers-the-title-placeholder: the deck keeps its single slide");
    let shapes = slides.modified[0].diff.shapes.as_ref().expect("set-snapshot/retitles-and-lowers-the-title-placeholder: the shapes triple must be present");
    assert_eq!(shapes.modified.len(), 1, "set-snapshot/retitles-and-lowers-the-title-placeholder: only the title placeholder is patched — the picture must not appear");
    match &shapes.modified[0].diff {
        crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::diff::PptxShapeDiff::Placeholder(placeholder) => {
            assert!(placeholder.kind.is_none(), "set-snapshot/retitles-and-lowers-the-title-placeholder: the placeholder type is unchanged");
            assert_eq!(placeholder.position.expect("position slot").y, 457200, "set-snapshot/retitles-and-lowers-the-title-placeholder: PptxTransform is a weak value entity — the delta carries the whole new rectangle, not just the moved axis");
            let runs = placeholder.text_frame.as_ref().expect("text frame triple").modified[0].diff.runs.as_ref().expect("runs triple");
            assert_eq!(runs.modified[0].diff.text.as_deref(), Some("Nakagin Capsule Tower"), "set-snapshot/retitles-and-lowers-the-title-placeholder: the run patch names text");
            assert!(runs.modified[0].diff.font_size.is_none(), "set-snapshot/retitles-and-lowers-the-title-placeholder: the run's tri-state fontSize slot is unchanged");
        }
        other => panic!("set-snapshot/retitles-and-lowers-the-title-placeholder: a field-level edit must keep the kind-shaped Placeholder diff, not degrade to Replace: {other:?}"),
    }
}

/// 🔣️ The committed diff is itself canonical and decodes to PptxDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PptxDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/retitles-and-lowers-the-title-placeholder: committed diff JSON is not canonical");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").pointer("/presentation/slides/modified/0/diff/shapes/modified/0/kind").and_then(serde_json::Value::as_str),
        Some("placeholder"),
        "set-snapshot/retitles-and-lowers-the-title-placeholder: PptxShapeDiff is tagged `kind` — the SNAPSHOT enum's own tag is `shapeKind`, and mixing the two up is exactly the collision this artifact renamed around"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PptxDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <PptxDiff as protocol::MutationDiff<PptxSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/retitles-and-lowers-the-title-placeholder: committed diff did not carry before to after");
}
