//! 🧪️ `change-pencil-width` fixture — `🪄️thickens-pencil`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::find_block;
use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
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

/// ▶️ `change-pencil-width` writes `NoteDiff.pencil_width` only.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("change-pencil-width applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-pencil-width/thickens-pencil: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse restores the base's own `pencil_width`, here `Some(3.0)`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("change-pencil-width applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("change-pencil-width inverse step applies");
    }
    assert_eq!(snapshot, base, "change-pencil-width/thickens-pencil: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-pencil-width/thickens-pencil: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-pencil-width/thickens-pencil: committed mutation JSON is not canonical");
}

/// 🎯️ 5.0 is finite and strictly positive, so the `mutation.invariant` fatal guard does not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "change-pencil-width/thickens-pencil: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "change-pencil-width/thickens-pencil: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("change-pencil-width/thickens-pencil: declared applied but the diff would not apply");
}

/// 🔺️ Only the scalar `pencilWidth` slot is set; `blocks` stays `None`, which is what proves the tool setting cannot reach an already-drawn stroke.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-pencil-width/thickens-pencil: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `change-pencil-width` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-pencil-width/thickens-pencil: committed diff JSON is not canonical");
}

/// 🩹 The committed `pencilWidth`-only delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-pencil-width/thickens-pencil: committed diff did not carry before to after");
}

/// ✏️ The pencil TOOL width changes; the already-drawn `blk-ink` stroke keeps its own 2.0 width.
#[semio_framework_async_macros::async_test]
async fn tool_width_changes_but_existing_ink_keeps_its_own_width() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-pencil-width applies");
    assert_eq!(base.pencil_width, Some(3.0), "change-pencil-width/thickens-pencil: the base pencil must start at 3.0");
    assert_eq!(applied.pencil_width, Some(5.0), "change-pencil-width/thickens-pencil: the pencil must thicken to 5.0");
    let NoteBlockNode::Ink { stroke_width, .. } = find_block(&applied.blocks, "blk-ink").expect("the document still carries its ink block") else {
        panic!("change-pencil-width/thickens-pencil: blk-ink must still be an ink block");
    };
    assert_eq!(*stroke_width, 2.0, "the pencil tool width is a document setting — it must never retro-edit an already-drawn stroke");
    assert_eq!(applied.eraser_radius, Some(12.0), "the eraser is a separate tool setting");
}
