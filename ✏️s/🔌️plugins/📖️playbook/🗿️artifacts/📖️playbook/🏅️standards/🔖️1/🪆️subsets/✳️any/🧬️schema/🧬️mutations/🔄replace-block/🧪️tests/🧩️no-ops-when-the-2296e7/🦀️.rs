//! 🧪️ `replace-block` fixture — `🧩️no-ops-when-the-block-is-already-identical`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ Playbook's steps live in the composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle that cannot be hand-authored —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🔄 `replace-block` is the only playbook verb whose third guard is a WHOLE-VALUE equality test:
//! it compares the incoming `PlaybookBlock` against the stored one field by field (all ~18 of the
//! kind-dependent optional ones included) and warns instead of rewriting when they match. The
//! seeded scene therefore holds the committed payload's own block, verbatim — nothing is invented,
//! and the committed payload sets a genuinely multi-field block so the equality test has real work.

use crate::artifacts::playbook::mutations::{apply_playbook_mutation, inverse_playbook_mutation, PlaybookMutation};
use crate::artifacts::playbook::{attach_playbook_steps, PlaybookDiff, PlaybookSnapshot, PlaybookStep};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> PlaybookMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> PlaybookSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `flow` child resolved to a step holding exactly
/// the block the committed payload carries — the value identity `replace-block` answers with a
/// warning.
fn before() -> PlaybookSnapshot {
    let mut snapshot: PlaybookSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PlaybookMutation::ReplaceBlock(payload) = mutation() else {
        panic!("no-ops-when-the-block-is-already-identical's committed mutation must be a replace-block");
    };
    attach_playbook_steps(&mut snapshot.flow, vec![PlaybookStep { id: payload.step_id.clone(), title: "Intro".into(), description: None, blocks: vec![payload.block.clone()] }]);
    snapshot
}

/// ▶️ Replacing a block with its own current value carries `before` to exactly the committed
/// `after`, leaving both composed child handles untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-block/no-ops-when-the-block-is-already-identical: applied state differs from committed after-snapshot");
    assert_eq!((&snapshot.document.child_id, &snapshot.flow.child_id), (&base.document.child_id, &base.flow.child_id), "an identical replacement must not re-mint content handles");
}

/// 🔺️ The delta is exactly the committed all-null `PlaybookDiff` — a value-identical replacement
/// never reaches `diff_replace_content`, so it cannot rewrite the whole scene under the guise of a
/// one-block edit.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-block/no-ops-when-the-block-is-already-identical: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &PlaybookDiff::default(), "an identical replacement must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to playbook's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-block/no-ops-when-the-block-is-already-identical: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`, with the
/// block-bearing `flow` slot never set.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.flow.is_none() && decoded.document.is_none(), "a value-identical replacement must leave both composed slots unset");
    let produced = <PlaybookDiff as protocol::MutationDiff<PlaybookSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-block/no-ops-when-the-block-is-already-identical: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — every unset
/// optional block field is OMITTED, never serialized as null.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-block/no-ops-when-the-block-is-already-identical: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-block/no-ops-when-the-block-is-already-identical: committed mutation JSON is not canonical");
    assert!(original.get("block").and_then(|block| block.get("placeholder")).is_none(), "unset block fields are omitted, never null");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-block/no-ops-when-the-block-is-already-identical declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an unchanged replacement is a warning, not a missing-target error");
}

/// ↩️ `replace-block`'s inverse is BASE-derived: it replays the block as it stood before. Here that
/// value IS the payload's own block, so a value-identical replacement is its own inverse.
#[semio_framework_async_macros::async_test]
async fn inverse_replays_the_captured_block_and_is_its_own_inverse_here() {
    let base = before();
    let PlaybookMutation::ReplaceBlock(payload) = mutation() else {
        panic!("committed mutation must be a replace-block");
    };
    let inverse = inverse_playbook_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "replace-block undoes with exactly one step, got {inverse:?}");
    let PlaybookMutation::ReplaceBlock(undo) = &inverse[0] else {
        panic!("replace-block's inverse must be a replace-block, got {:?}", inverse[0]);
    };
    assert_eq!((undo.step_id.as_str(), &undo.block), (payload.step_id.as_str(), &payload.block), "the inverse restores the captured block, which here equals the requested one");
    let restored = apply_playbook_mutation(&apply_playbook_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "replace-block/no-ops-when-the-block-is-already-identical: inverse did not restore the before-snapshot");
}
