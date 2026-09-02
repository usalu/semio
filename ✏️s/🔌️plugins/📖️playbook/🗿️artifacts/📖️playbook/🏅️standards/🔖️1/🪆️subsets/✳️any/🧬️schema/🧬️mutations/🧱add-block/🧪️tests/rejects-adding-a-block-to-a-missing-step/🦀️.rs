//! 🧪️ `add-block` fixture — `rejects-adding-a-block-to-a-missing-step`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Playbook's steps live in the composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle that cannot be hand-authored —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🧱 `add-block` carries a TWO-segment address (`step`, then `block`) but guards them in order: the
//! OWNING STEP is checked first, and when it is missing the diagnostic names only that one segment —
//! the duplicate-block warning below it is never reached. The committed `flow` handle is left
//! unseeded, so the scene is empty and the outer guard is the one that fires.

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

/// ▶️ A rejected `add-block` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "add-block/rejects-adding-a-block-to-a-missing-step: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.flow.child_id, &base.flow.child_id, "a rejected block insert must not mint a new flow handle");
}

/// 🚫️ The owning step is checked before the block: a missing step yields an Error-level
/// `mutation.target-missing` naming ONLY the step segment, even though `add-block`'s own address is
/// two segments long.
#[semio_framework_async_macros::async_test]
async fn a_missing_owning_step_is_reported_with_a_one_segment_path() {
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PlaybookDiff::default(), "a rejecting add-block must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing owning step is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing owning step is an Error, not the Warning a duplicate block id would raise");
    assert_eq!(messages[0].target, vec!["s-missing".to_string()], "the diagnostic names the step alone — the block segment is never reached");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PlaybookDiff::default(), "add-block/rejects-adding-a-block-to-a-missing-step: a rejection must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `index` is absent
/// from the committed payload on purpose: it is `skip_serializing_if = "Option::is_none"`, so an
/// appended block must NOT carry a null slot.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-block/rejects-adding-a-block-to-a-missing-step: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "add-block/rejects-adding-a-block-to-a-missing-step: committed mutation JSON is not canonical");
    assert!(original.get("index").is_none(), "an appended block omits `index` entirely rather than serializing null");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "add-block/rejects-adding-a-block-to-a-missing-step declares a rejected outcome");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `add-block`'s inverse is PAYLOAD-derived: a `remove-block` of the (step, block) pair it was
/// asked to create, produced even here where the step it targets does not exist.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_remove_block_of_the_requested_pair_even_when_refused() {
    let inverse = inverse_playbook_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "add-block always undoes with exactly one step, got {inverse:?}");
    let PlaybookMutation::RemoveBlock(undo) = &inverse[0] else {
        panic!("add-block's inverse must be a remove-block, got {:?}", inverse[0]);
    };
    assert_eq!((undo.step_id.as_str(), undo.block_id.as_str()), ("s-missing", "b-notes"), "the inverse removes exactly the pair the payload carried");
}

/// 🪪️ The fixture is bound to `add-block`'s own descriptor, and to its two-segment address — the
/// address the DIAGNOSTIC deliberately truncates to one segment here.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_add_block() {
    let semantics = <PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("add", "block", "add-block", "AddedBlock"), "the fixture must be bound to add-block's own descriptor");
    assert_eq!(<PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::target(&mutation()), vec!["s-missing".to_string(), "b-notes".to_string()], "add-block addresses step then block, outermost first");
}
