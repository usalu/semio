//! 🧪️ `edit-block-text` fixture — `replaces-the-intro-paragraphs`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::schema::{block_bounds, find_block};
use crate::artifacts::note::{NoteBlockNode, NoteDiff, NoteSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> NoteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> NoteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> NoteMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `edit-block-text` remints the block's COMPOSED child handle from `(block_id, new_paragraphs)` — the paragraphs themselves never land in the snapshot.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("edit-block-text applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "edit-block-text/replaces-the-intro-paragraphs: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse reads the empty paragraph list owned by the base text-child record.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("edit-block-text applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("edit-block-text inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-block-text/replaces-the-intro-paragraphs: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-block-text/replaces-the-intro-paragraphs: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-block-text/replaces-the-intro-paragraphs: committed mutation JSON is not canonical");
}

/// 🎯️ The block exists AND is a text block, so the `mutation.target-missing` error guard (absent or non-text) does not fire; this leaf has no no-op guard at all.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "edit-block-text/replaces-the-intro-paragraphs: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "edit-block-text/replaces-the-intro-paragraphs: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("edit-block-text/replaces-the-intro-paragraphs: declared applied but the diff would not apply");
}

/// 🔺️ One `blocks.patched` entry whose `blockJson` carries the REMINTED `content.childId` and the same `target` — the paragraphs never appear in the delta, only the handle they address.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-block-text/replaces-the-intro-paragraphs: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `edit-block-text` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-block-text/replaces-the-intro-paragraphs: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`patched` delta carries `before` to `after` using only snapshot-owned records.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-block-text/replaces-the-intro-paragraphs: committed diff did not carry before to after");
}

/// 📝 Content addressing in action: the persisted `content.handle.child_id` changes because the paragraphs changed, while the child's `target` slot (keyed by block id) stays put.
#[semio_framework_async_macros::async_test]
async fn content_child_id_is_reminted_while_the_target_slot_is_stable() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("edit-block-text applies");
    let NoteBlockNode::Text { content: before_content, .. } = find_block(&base.blocks, "blk-text").expect("the base text block exists") else {
        panic!("edit-block-text/replaces-the-intro-paragraphs: the base block must be a text block");
    };
    let NoteBlockNode::Text { content, font_size, .. } = find_block(&applied.blocks, "blk-text").expect("the text block survives") else {
        panic!("edit-block-text must not change the block's kind");
    };
    assert_ne!(content.handle.child_id, before_content.handle.child_id, "edit-block-text/replaces-the-intro-paragraphs: new paragraphs must mint a new content-addressed child id");
    assert_eq!(content.target, before_content.target, "the child SLOT is keyed by block id, so it must not move when the content changes");
    assert_eq!(*font_size, 16.0, "editing the text must not restyle the block");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-text").expect("the text block exists")), (0.0, 0.0, 280.0, 120.0), "editing the text must not reflow the block's box");
}
