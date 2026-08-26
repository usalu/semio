//! 🧪️ `move-step` fixture — `no-ops-when-the-step-is-already-at-that-index`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ Playbook's steps live in the composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle that cannot be hand-authored —
//! this tree pins the guard branches, which mint nothing.
//!
//! ↔️ `move-step` computes its landing slot on the list with the step ALREADY REMOVED
//! (`steps.remove(position)` then `index.min(steps.len())`), then compares that slot to the original
//! position. The seeded scene holds a single step, so index 0 is both its position and the only
//! reachable slot — the exact arithmetic this second guard exists for.

use crate::artifacts::playbook::mutations::{apply_playbook_mutation, inverse_playbook_mutation, PlaybookMutation};
use crate::artifacts::playbook::{attach_playbook_steps, PlaybookDiff, PlaybookSnapshot, PlaybookStep};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> PlaybookMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> PlaybookSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `flow` child resolved to a one-step scene whose
/// only step is the id the committed payload names. Only that id is load-bearing here: the guard
/// compares list positions, never titles or blocks.
fn before() -> PlaybookSnapshot {
    let mut snapshot: PlaybookSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PlaybookMutation::MoveStep(payload) = mutation() else {
        panic!("no-ops-when-the-step-is-already-at-that-index's committed mutation must be a move-step");
    };
    attach_playbook_steps(&mut snapshot.flow, vec![PlaybookStep { id: payload.step_id.clone(), title: "Intro".into(), description: None, blocks: Vec::new() }]);
    snapshot
}

/// ▶️ A `move-step` onto the slot the step already occupies carries `before` to exactly the
/// committed `after`, leaving both composed child handles untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "move-step/no-ops-when-the-step-is-already-at-that-index: applied state differs from committed after-snapshot");
    assert_eq!((&snapshot.document.child_id, &snapshot.flow.child_id), (&base.document.child_id, &base.flow.child_id), "a positional no-op must not re-mint content handles");
}

/// 🔺️ The delta is exactly the committed all-null `PlaybookDiff`: the guard returns AFTER the
/// target lookup succeeded but BEFORE the reinsertion, so no reordered step list is ever built.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-step/no-ops-when-the-step-is-already-at-that-index: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff(), &PlaybookDiff::default(), "a positional no-op must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to playbook's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-step/no-ops-when-the-step-is-already-at-that-index: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — no ordering
/// field of any kind is set, so the composed flow child keeps its original node order.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.flow.is_none(), "a positional no-op must leave the ordering-bearing flow child unset");
    let produced = <PlaybookDiff as protocol::MutationDiff<PlaybookSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-step/no-ops-when-the-step-is-already-at-that-index: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-step/no-ops-when-the-step-is-already-at-that-index: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-step/no-ops-when-the-step-is-already-at-that-index: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`. A move
/// onto the current slot is a warning, never the Error a missing step would raise.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-step/no-ops-when-the-step-is-already-at-that-index declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an already-there move is a warning, not a missing-target error");
}

/// ↩️ `move-step`'s inverse is BASE-derived: it moves the step back to the position it held in the
/// pre-state. Here that position IS the requested index, so the inverse is the same move — and
/// replaying it leaves the document exactly where it started.
#[semio_framework_async_macros::async_test]
async fn inverse_moves_the_step_back_to_its_base_position() {
    let base = before();
    let inverse = inverse_playbook_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "move-step always undoes with exactly one step, got {inverse:?}");
    let PlaybookMutation::MoveStep(undo) = &inverse[0] else {
        panic!("move-step's inverse must be a move-step, got {:?}", inverse[0]);
    };
    assert_eq!((undo.step_id.as_str(), undo.index), ("s-intro", 0), "the inverse restores the base position of the step the payload named");
    let restored = apply_playbook_mutation(&apply_playbook_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "move-step/no-ops-when-the-step-is-already-at-that-index: inverse did not restore the before-snapshot");
}
