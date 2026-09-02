//! 🧪️ `change-block-font-size` fixture — `enlarges-the-intro-font`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::schema::{block_bounds, find_block};
use crate::artifacts::note::{NoteBlockNode, NoteDiff, NoteSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> NoteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> NoteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> NoteMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `change-block-font-size` emits ONE whole-block `patched` entry whose only changed field is `font_size`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("change-block-font-size applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-block-font-size/enlarges-the-intro-font: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse re-issues `change-block-font-size` with the base text block's own prior size.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("change-block-font-size applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("change-block-font-size inverse step applies");
    }
    assert_eq!(snapshot, base, "change-block-font-size/enlarges-the-intro-font: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-block-font-size/enlarges-the-intro-font: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-block-font-size/enlarges-the-intro-font: committed mutation JSON is not canonical");
}

/// 🎯️ The block exists AND is a text block, and 24.0 differs from 16.0, so neither the `mutation.target-missing` error (absent or non-text) nor the `mutation.no-op` warn fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "change-block-font-size/enlarges-the-intro-font: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "change-block-font-size/enlarges-the-intro-font: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("change-block-font-size/enlarges-the-intro-font: declared applied but the diff would not apply");
}

/// 🔺️ One `blocks.patched` entry whose `blockJson` still carries the UNCHANGED `content` child handle — the delta itself proves a font change never reminis composed content.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-block-font-size/enlarges-the-intro-font: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `change-block-font-size` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-block-font-size/enlarges-the-intro-font: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`patched` delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-block-font-size/enlarges-the-intro-font: committed diff did not carry before to after");
}

/// 🔤 A TEXT-ONLY field: the font size changes while the block's composed content handle, weight, alignment and box are untouched.
#[semio_framework_async_macros::async_test]
async fn text_only_font_size_changes_leaving_the_content_handle_alone() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-block-font-size applies");
    let NoteBlockNode::Text { font_size: before_size, content: before_content, .. } = find_block(&base.blocks, "blk-text").expect("the base text block exists") else {
        panic!("change-block-font-size/enlarges-the-intro-font: the base block must be a text block");
    };
    assert_eq!(*before_size, 16.0, "change-block-font-size/enlarges-the-intro-font: the base must start at 16.0");
    let NoteBlockNode::Text { font_size, content, font_weight, align, .. } = find_block(&applied.blocks, "blk-text").expect("the text block survives") else {
        panic!("change-block-font-size must not change the block's kind");
    };
    assert_eq!(*font_size, 24.0, "change-block-font-size/enlarges-the-intro-font: the font must grow to 24.0");
    assert_eq!(content, before_content, "resizing the font must not remint the composed text child handle");
    assert_eq!((font_weight.as_str(), align.as_str()), ("normal", "left"), "resizing the font must not restyle the block");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-text").expect("the text block exists")), (0.0, 0.0, 280.0, 120.0), "resizing the font must not reflow the block's box");
}
