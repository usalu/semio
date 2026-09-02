//! 🧪️ `remove-step` fixture — `rejects-removing-a-missing-step`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Playbook keeps its steps in the composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), so a
//! committed snapshot carries a handle, never steps, and every content-changing diff mints a fresh
//! `DefaultHasher`-digest handle that cannot be hand-authored — this tree pins the guard branches,
//! which mint nothing.
//!
//! ➖ This case leaves the committed `flow` handle unseeded on purpose: an unresolved child reads
//! back as an EMPTY working scene (`playbook_working_scene_for_handle` fails soft, never panics),
//! which is exactly the state in which `remove-step`'s single target guard fires.

use crate::artifacts::playbook::mutations::{apply_playbook_mutation, inverse_playbook_mutation, PlaybookMutation};
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> PlaybookMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn before() -> PlaybookSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> PlaybookSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// ▶️ A rejected `remove-step` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "remove-step/rejects-removing-a-missing-step: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.flow.child_id, &base.flow.child_id, "a rejected remove must not mint a new flow handle");
}

/// 🚫️ Removing a step that is not in the scene is an Error-level `mutation.target-missing`
/// addressed at the one id the payload named — `remove-step` has exactly one address segment,
/// unlike its block-scoped sibling.
#[semio_framework_async_macros::async_test]
async fn a_missing_step_is_an_error_target_missing() {
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PlaybookDiff::default(), "a rejecting remove-step must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing step is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing removal target is an Error — a merge policy may absorb it, unlike a Fatal");
    assert_eq!(messages[0].target, vec!["s-archive".to_string()], "the diagnostic addresses the step id alone");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch: `🔺️diff/🚫️.absent` is an
/// empty marker file and the builder really does return the identity diff.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PlaybookDiff::default(), "remove-step/rejects-removing-a-missing-step: a rejection must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-step/rejects-removing-a-missing-step: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-step/rejects-removing-a-missing-step: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "remove-step/rejects-removing-a-missing-step declares a rejected outcome");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `remove-step`'s inverse is BASE-derived — it replays the captured step at its captured
/// position — so a target that was never in the scene yields NO undo step at all. This is the exact
/// opposite of `add-step`, whose payload-derived inverse always produces one step.
#[semio_framework_async_macros::async_test]
async fn inverse_of_a_missing_remove_is_empty() {
    let inverse = inverse_playbook_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "remove-step has nothing to restore when its target is absent, got {inverse:?}");
}

/// 🪪️ The fixture is bound to `remove-step`'s own descriptor and address, not merely to some
/// mutation that happens to reject.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_remove_step() {
    let semantics = <PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("remove", "step", "remove-step", "RemovedStep"), "the fixture must be bound to remove-step's own descriptor");
    assert_eq!(<PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::target(&mutation()), vec!["s-archive".to_string()], "remove-step addresses exactly its step id");
}
