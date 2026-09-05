//! 🧪️ `move-block` fixture — `🧩️rejects-moving-a-block-into-a-missing-step`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Playbook's steps live in the composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle that cannot be hand-authored —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🔀 `move-block` is the only playbook verb with a DESTINATION step, and this case pins the guard
//! only it has: the source step and the block are both found, the move is recognised as cross-step
//! (`from_step_id != to_step_id`), and it is the DESTINATION lookup that fails. The seeded scene
//! therefore holds the source step with the block in it — the two lookups that must succeed before
//! the third can be the one that rejects.

use crate::artifacts::playbook::mutations::{apply_playbook_mutation, inverse_playbook_mutation, PlaybookMutation};
use crate::artifacts::playbook::{attach_playbook_steps, PlaybookBlock, PlaybookDiff, PlaybookSnapshot, PlaybookStep};

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

/// 🌱 The committed `⬅️before`, with its composed `flow` child resolved to the SOURCE step holding
/// the block the payload names. Only the two ids are load-bearing; the block's own `kind`/`label`
/// never enter `move-block`'s guards, which look at list membership and position alone.
fn before() -> PlaybookSnapshot {
    let mut snapshot: PlaybookSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PlaybookMutation::MoveBlock(payload) = mutation() else {
        panic!("rejects-moving-a-block-into-a-missing-step's committed mutation must be a move-block");
    };
    let block: PlaybookBlock = serde_json::from_value(serde_json::json!({ "id": payload.block_id.clone(), "label": "Notes", "kind": "text" })).expect("seed block decodes");
    attach_playbook_steps(&mut snapshot.flow, vec![PlaybookStep { id: payload.from_step_id.clone(), title: "Intro".into(), description: None, blocks: vec![block] }]);
    snapshot
}

/// ▶️ A rejected `move-block` leaves the document byte-identical to the committed `after` — the
/// source step is NOT emptied on the way to discovering the destination is missing.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "move-block/rejects-moving-a-block-into-a-missing-step: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.flow.child_id, &base.flow.child_id, "a rejected relocation must not mint a new flow handle");
}

/// 🚫️ The destination step is the third guard, and it addresses the DESTINATION id — not the
/// source step and not the block, both of which were found.
#[semio_framework_async_macros::async_test]
async fn a_missing_destination_step_is_reported_at_the_destination_id() {
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PlaybookDiff::default(), "a rejecting move-block must carry the identity diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing destination step is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing destination is an Error, not the Warning an already-there index would raise");
    assert_eq!(messages[0].target, vec!["s-missing".to_string()], "the diagnostic names the destination step, not the source the payload also carries");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PlaybookDiff::default(), "move-block/rejects-moving-a-block-into-a-missing-step: a rejection must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `index` is a plain
/// `usize` here — unlike `add-block`'s optional slot it is always serialized.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-block/rejects-moving-a-block-into-a-missing-step: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-block/rejects-moving-a-block-into-a-missing-step: committed mutation JSON is not canonical");
    assert_eq!(original.get("index").and_then(serde_json::Value::as_u64), Some(0), "move-block's landing slot is mandatory and final-state");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "move-block/rejects-moving-a-block-into-a-missing-step declares a rejected outcome");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `move-block`'s inverse is BASE-derived from the SOURCE side only, so it exists even though the
/// forward move was refused — and it swaps the two step ids, naming the missing step as the source
/// it would move back FROM.
#[semio_framework_async_macros::async_test]
async fn inverse_swaps_the_two_step_ids_even_when_the_destination_is_missing() {
    let inverse = inverse_playbook_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "move-block undoes with exactly one step once its source block is found, got {inverse:?}");
    let PlaybookMutation::MoveBlock(undo) = &inverse[0] else {
        panic!("move-block's inverse must be a move-block, got {:?}", inverse[0]);
    };
    assert_eq!((undo.block_id.as_str(), undo.from_step_id.as_str(), undo.to_step_id.as_str(), undo.index), ("b-notes", "s-missing", "s-intro", 0), "the inverse moves the block back from the destination to its captured source position");
}

/// 🪪️ The fixture is bound to `move-block`'s own descriptor, whose address names the SOURCE step —
/// deliberately a different id from the one the rejection diagnostic carries.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_move_block() {
    let semantics = <PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("move", "block", "move-block", "MovedBlock"), "the fixture must be bound to move-block's own descriptor");
    assert_eq!(<PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::target(&mutation()), vec!["s-intro".to_string(), "b-notes".to_string()], "move-block addresses the source step and the block, never the destination");
}
