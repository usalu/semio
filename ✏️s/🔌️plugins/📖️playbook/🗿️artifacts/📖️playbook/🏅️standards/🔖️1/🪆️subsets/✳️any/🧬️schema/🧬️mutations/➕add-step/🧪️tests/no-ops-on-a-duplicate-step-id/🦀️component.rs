//! 🧪️ `add-step` fixture — `no-ops-on-a-duplicate-step-id`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ Why every playbook case pins a GUARD branch: `PlaybookSnapshot` keeps its steps in the
//! composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), so a committed snapshot carries a
//! handle, never steps — and every content-changing playbook diff routes through
//! `diff_replace_content`, which mints a fresh handle whose `child_id` is a `DefaultHasher` digest.
//! Hand-authoring such an `➡️after` would mean forging a value from `std`'s deliberately
//! unspecified default hasher, so this tree pins the branches that mint no handle at all.
//!
//! ➕ `add-step`'s only guard is a duplicate step id, and playbook answers it with a Warning
//! `mutation.no-op` — never the Fatal `mutation.duplicate-id` other artifacts raise for the same
//! collision. The seeded scene holds exactly the step the committed payload asks to add; nothing
//! about it is invented.

use crate::artifacts::playbook::mutations::{apply_playbook_mutation, inverse_playbook_mutation, PlaybookMutation};
use crate::artifacts::playbook::{cache_playbook_steps, PlaybookDiff, PlaybookSnapshot};

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

/// 🌱 The committed `⬅️before`, with its composed `flow` child resolved to a scene holding exactly
/// the step the committed payload carries — the id collision `add-step` guards against.
fn before() -> PlaybookSnapshot {
    let snapshot: PlaybookSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PlaybookMutation::AddStep(payload) = mutation() else {
        panic!("no-ops-on-a-duplicate-step-id's committed mutation must be an add-step");
    };
    cache_playbook_steps(&snapshot.flow.child_id, vec![payload.step.clone()]);
    snapshot
}

/// ▶️ A refused `add-step` carries `before` to exactly the committed `after` — and, critically,
/// leaves both composed child handles alone: a no-op must never re-mint content.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "add-step/no-ops-on-a-duplicate-step-id: applied state differs from committed after-snapshot");
    assert_eq!((&snapshot.document.child_id, &snapshot.flow.child_id), (&base.document.child_id, &base.flow.child_id), "a duplicate add must not mint new document/flow handles");
}

/// 🔺️ The delta a duplicate `add-step` produces is exactly the committed all-null `PlaybookDiff` —
/// the guard returns before `diff_replace_content` is ever reached, so no handle is half-built.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "add-step/no-ops-on-a-duplicate-step-id: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff(), &PlaybookDiff::default(), "a refused add-step must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to playbook's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-step/no-ops-on-a-duplicate-step-id: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`. `title` is the
/// one double-`Option` field on `PlaybookDiff`, so a committed `null` there decodes as an explicit
/// "clear the title" — which is why this tree's snapshots carry `"title": null` and the round trip
/// is still a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <PlaybookDiff as protocol::MutationDiff<PlaybookSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-step/no-ops-on-a-duplicate-step-id: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-step/no-ops-on-a-duplicate-step-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "add-step/no-ops-on-a-duplicate-step-id: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: an `applied` status carrying one Warning `mutation.no-op`. The
/// warning is deliberately untargeted — `MutationOutcome::warn` takes no address, unlike the
/// Error-level rejections the rest of this vocabulary raises.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-step/no-ops-on-a-duplicate-step-id declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "a duplicate step id is a warning, never an error");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a duplicate step id must not escalate to Error or Fatal");
    assert!(messages[0].target.is_empty(), "add-step's no-op warning carries no target address");
}

/// ↩️ `add-step`'s inverse is PAYLOAD-derived, never BASE-derived: it is a `remove-step` of the id
/// it was asked to create, even here where the create was refused as a duplicate.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_remove_of_the_requested_id_even_when_refused() {
    let inverse = inverse_playbook_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "add-step always undoes with exactly one step, got {inverse:?}");
    let PlaybookMutation::RemoveStep(undo) = &inverse[0] else {
        panic!("add-step's inverse must be a remove-step, got {:?}", inverse[0]);
    };
    assert_eq!(undo.step_id, "s-review", "the inverse removes exactly the id the payload carried");
}
