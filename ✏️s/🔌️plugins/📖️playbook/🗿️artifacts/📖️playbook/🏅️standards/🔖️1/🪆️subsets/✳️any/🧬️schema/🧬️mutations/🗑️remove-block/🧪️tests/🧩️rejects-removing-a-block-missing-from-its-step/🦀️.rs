//! 🧪️ `remove-block` fixture — `🧩️rejects-removing-a-block-missing-from-its-step`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Playbook's steps live in the composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle that cannot be hand-authored —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🗑️ This case pins `remove-block`'s INNER guard, the one its sibling `add-block` fixture cannot
//! reach: the owning step really is present (the seeded scene holds it, with no blocks), so the
//! block lookup is what fails and the diagnostic carries BOTH address segments.

use crate::artifacts::playbook::mutations::{apply_playbook_mutation, inverse_playbook_mutation, PlaybookMutation};
use crate::artifacts::playbook::{attach_playbook_steps, PlaybookDiff, PlaybookSnapshot, PlaybookStep};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> PlaybookMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> PlaybookSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `flow` child resolved to a scene holding the step
/// the payload names — and no blocks at all, so the block the payload names is the only thing
/// missing.
fn before() -> PlaybookSnapshot {
    let mut snapshot: PlaybookSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PlaybookMutation::RemoveBlock(payload) = mutation() else {
        panic!("rejects-removing-a-block-missing-from-its-step's committed mutation must be a remove-block");
    };
    attach_playbook_steps(&mut snapshot.flow, vec![PlaybookStep { id: payload.step_id.clone(), title: "Intro".into(), description: None, blocks: Vec::new() }]);
    snapshot
}

/// ▶️ A rejected `remove-block` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "remove-block/rejects-removing-a-block-missing-from-its-step: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.flow.child_id, &base.flow.child_id, "a rejected block removal must not mint a new flow handle");
}

/// 🚫️ A present step with an absent block is an Error-level `mutation.target-missing` addressed at
/// BOTH segments, outermost first — the second guard, not the first.
#[semio_framework_async_macros::async_test]
async fn a_missing_block_inside_a_present_step_is_reported_with_a_two_segment_path() {
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PlaybookDiff::default(), "a rejecting remove-block must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing block is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing removal target is an Error");
    assert_eq!(messages[0].target, vec!["s-intro".to_string(), "b-gone".to_string()], "the diagnostic names the owning step and then the block");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PlaybookDiff::default(), "remove-block/rejects-removing-a-block-missing-from-its-step: a rejection must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-block/rejects-removing-a-block-missing-from-its-step: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-block/rejects-removing-a-block-missing-from-its-step: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and two-segment path — is exactly what the diff builder
/// emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "remove-block/rejects-removing-a-block-missing-from-its-step declares a rejected outcome");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
    assert_eq!(declared_path.len(), 2, "remove-block declares a step-then-block address");
}

/// ↩️ `remove-block`'s inverse is BASE-derived — it replays the captured block at its captured
/// position inside the step — so an absent block yields NO undo step, even though the step itself
/// was found.
#[semio_framework_async_macros::async_test]
async fn inverse_is_empty_when_only_the_block_is_missing() {
    let inverse = inverse_playbook_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "remove-block has nothing to restore when the block is absent, got {inverse:?}");
}

/// 🪪️ The fixture is bound to `remove-block`'s own descriptor and its step-then-block address.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_remove_block() {
    let semantics = <PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("remove", "block", "remove-block", "RemovedBlock"), "the fixture must be bound to remove-block's own descriptor");
    assert_eq!(<PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::target(&mutation()), vec!["s-intro".to_string(), "b-gone".to_string()], "remove-block addresses step then block, outermost first");
}
