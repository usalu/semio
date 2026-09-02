//! 🧪️ `create-block` fixture — `inserts-a-photo-block-at-root-index-2`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::schema::{find_block, find_block_location};
use crate::artifacts::note::{NoteDiff, NoteSnapshot};
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

/// ▶️ `create-block` emits ONE `added` entry carrying the payload's own `(parent_id, index)` — never a whole-`blocks` swap.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("create-block applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-block/inserts-a-photo-block-at-root-index-2: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse is `delete-block` on the created id.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("create-block applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("create-block inverse step applies");
    }
    assert_eq!(snapshot, base, "create-block/inserts-a-photo-block-at-root-index-2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-block/inserts-a-photo-block-at-root-index-2: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-block/inserts-a-photo-block-at-root-index-2: committed mutation JSON is not canonical");
}

/// 🎯️ `blk-photo` is absent and `parent_id` is `None`, so neither the `mutation.duplicate-id` nor the container `mutation.invariant` fatal guard fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "create-block/inserts-a-photo-block-at-root-index-2: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "create-block/inserts-a-photo-block-at-root-index-2: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("create-block/inserts-a-photo-block-at-root-index-2: declared applied but the diff would not apply");
}

/// 🔺️ One `blocks.added` entry carrying `(parentId: null, index: 2)` and the whole new node; `removed`/`patched` stay empty and `reordered` stays `null` — never a whole-`blocks` swap.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-block/inserts-a-photo-block-at-root-index-2: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `create-block` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-block/inserts-a-photo-block-at-root-index-2: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`added` delta carries `before` to `after` on its own, inserting at the addressed index.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-block/inserts-a-photo-block-at-root-index-2: committed diff did not carry before to after");
}

/// ➕ The new block lands at the ADDRESSED root index 2, pushing the table/math/image/group right — not appended at the end.
#[semio_framework_async_macros::async_test]
async fn new_block_lands_at_the_addressed_index_not_appended() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("create-block applies");
    assert!(find_block(&base.blocks, "blk-photo").is_none(), "create-block/inserts-a-photo-block-at-root-index-2: the base must not already carry the new id");
    assert_eq!(applied.blocks.len(), base.blocks.len() + 1, "create-block must grow the root list by exactly one");
    assert_eq!(find_block_location(&applied.blocks, "blk-photo"), Some((None, 2)), "the block must land at the addressed root index 2, not be appended");
    assert_eq!(find_block_location(&applied.blocks, "blk-table"), Some((None, 3)), "the block formerly at index 2 must have been pushed right");
    assert_eq!(find_block_location(&applied.blocks, "blk-text"), Some((None, 0)), "blocks before the insertion point must not move");
}
