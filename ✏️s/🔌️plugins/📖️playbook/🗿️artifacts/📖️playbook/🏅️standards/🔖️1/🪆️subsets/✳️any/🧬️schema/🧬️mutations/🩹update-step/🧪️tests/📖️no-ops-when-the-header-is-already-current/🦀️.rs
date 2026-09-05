//! 🧪️ `update-step` fixture — `📖️no-ops-when-the-header-is-already-current`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ Playbook's steps live in the composed `s.stdio.semio.flow` CHILD (`🔖️WorkingScene`), and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle that cannot be hand-authored —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🩹 `update-step` owns exactly one cohesive facet — `title` PLUS `description`, always submitted
//! together — and its no-op guard is the CONJUNCTION of both: it warns only when neither field
//! moves. `blocks` is deliberately outside this payload, so the seeded step's block list plays no
//! part in the comparison. The seeded header is the committed payload's own, verbatim.

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

/// 🌱 The committed `⬅️before`, with its composed `flow` child resolved to a step whose header is
/// character-for-character the committed payload's `title`/`description` pair.
fn before() -> PlaybookSnapshot {
    let mut snapshot: PlaybookSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PlaybookMutation::UpdateStep(payload) = mutation() else {
        panic!("no-ops-when-the-header-is-already-current's committed mutation must be an update-step");
    };
    attach_playbook_steps(&mut snapshot.flow, vec![PlaybookStep { id: payload.step_id.clone(), title: payload.title.clone(), description: payload.description.clone(), blocks: Vec::new() }]);
    snapshot
}

/// ▶️ Re-submitting the header a step already has carries `before` to exactly the committed
/// `after`, leaving both composed child handles untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "update-step/no-ops-when-the-header-is-already-current: applied state differs from committed after-snapshot");
    assert_eq!((&snapshot.document.child_id, &snapshot.flow.child_id), (&base.document.child_id, &base.flow.child_id), "an unchanged header must not re-mint content handles — including the narrative document projection, which renders step titles");
}

/// 🔺️ The delta is exactly the committed all-null `PlaybookDiff`: both facet fields matched, so the
/// builder returns before touching the scene.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-step/no-ops-when-the-header-is-already-current: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &PlaybookDiff::default(), "an unchanged header must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to playbook's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-step/no-ops-when-the-header-is-already-current: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`. The step
/// header lives inside the `flow` child, and this diff never sets that slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.document.is_none(), "an unchanged header must leave the narrative document projection unset");
    let produced = <PlaybookDiff as protocol::MutationDiff<PlaybookSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-step/no-ops-when-the-header-is-already-current: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. `description` is
/// present here on purpose: it is `skip_serializing_if = "Option::is_none"`, so a payload that
/// clears it would omit the key entirely rather than send null.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-step/no-ops-when-the-header-is-already-current: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-step/no-ops-when-the-header-is-already-current: committed mutation JSON is not canonical");
    assert!(original.get("description").and_then(serde_json::Value::as_str).is_some(), "this case exercises the both-fields-match branch, so the description must be present");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "update-step/no-ops-when-the-header-is-already-current declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an unchanged header is a warning, not a missing-target error");
}

/// ↩️ `update-step`'s inverse is BASE-derived and rebuilds the WHOLE facet — title and description
/// together, never one of them. Here the captured header equals the requested one, so the inverse
/// is the committed payload itself.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_whole_captured_header() {
    let base = before();
    let PlaybookMutation::UpdateStep(payload) = mutation() else {
        panic!("committed mutation must be an update-step");
    };
    let inverse = inverse_playbook_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "update-step undoes with exactly one step, got {inverse:?}");
    let PlaybookMutation::UpdateStep(undo) = &inverse[0] else {
        panic!("update-step's inverse must be an update-step, got {:?}", inverse[0]);
    };
    assert_eq!((undo.step_id.as_str(), undo.title.as_str(), &undo.description), (payload.step_id.as_str(), payload.title.as_str(), &payload.description), "the inverse carries both facet fields, restored from the base step");
    let restored = apply_playbook_mutation(&apply_playbook_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "update-step/no-ops-when-the-header-is-already-current: inverse did not restore the before-snapshot");
}
