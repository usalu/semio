//! 🧪️ `edit-block-math` fixture — `replaces-the-tex-with-pythagoras`.
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

/// ▶️ `edit-block-math` emits ONE whole-block `patched` entry whose only changed field is `tex`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("edit-block-math applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "edit-block-math/replaces-the-tex-with-pythagoras: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse re-issues `edit-block-math` with the base math block's own prior TeX source.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("edit-block-math applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("edit-block-math inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-block-math/replaces-the-tex-with-pythagoras: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-block-math/replaces-the-tex-with-pythagoras: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-block-math/replaces-the-tex-with-pythagoras: committed mutation JSON is not canonical");
}

/// 🎯️ The block exists AND is a math block, and the TeX genuinely differs, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "edit-block-math/replaces-the-tex-with-pythagoras: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "edit-block-math/replaces-the-tex-with-pythagoras: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("edit-block-math/replaces-the-tex-with-pythagoras: declared applied but the diff would not apply");
}

/// 🔺️ One `blocks.patched` entry whose `blockJson` carries the new `tex` verbatim and the same `displayMode`.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-block-math/replaces-the-tex-with-pythagoras: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `edit-block-math` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-block-math/replaces-the-tex-with-pythagoras: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`patched` delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-block-math/replaces-the-tex-with-pythagoras: committed diff did not carry before to after");
}

/// 🧮 The authored TeX SOURCE is replaced verbatim — nothing is parsed or normalized, and `display_mode` is a separate concern this leaf never touches.
#[semio_framework_async_macros::async_test]
async fn tex_source_is_replaced_verbatim_and_display_mode_is_untouched() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("edit-block-math applies");
    let NoteBlockNode::Math { tex: before_tex, .. } = find_block(&base.blocks, "blk-math").expect("the base math block exists") else {
        panic!("edit-block-math/replaces-the-tex-with-pythagoras: the base block must be a math block");
    };
    assert_eq!(before_tex, "E = mc^2", "edit-block-math/replaces-the-tex-with-pythagoras: the base must start from the old TeX");
    let NoteBlockNode::Math { tex, display_mode, .. } = find_block(&applied.blocks, "blk-math").expect("the math block survives") else {
        panic!("edit-block-math must not change the block's kind");
    };
    assert_eq!(tex, "a^2 + b^2 = c^2", "the TeX source must be stored verbatim, unparsed and unnormalized");
    assert!(*display_mode, "display_mode is a separate concern — editing the source must not flip it");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-math").expect("the math block exists")), (0.0, 400.0, 200.0, 80.0), "editing the TeX must not re-lay-out the block");
}
