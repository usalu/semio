//! 🧪️ `change-title` fixture — `changes-the-playbook-title`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ✏️ `change-title` is the ONE playbook verb whose diff never touches the composed children: it
//! sets the document's own root `title` scalar and nothing else. That makes it the one case in this
//! tree that pins a real, non-empty `🔺️diff` and a genuinely different `➡️after` — every
//! step/block verb routes through `diff_replace_content`, which mints `DefaultHasher`-digest child
//! handles no fixture can hand-author.
//!
//! ⚠️ `title` is `Option<Option<String>>` on `PlaybookDiff`: the outer layer is "did this diff touch
//! the title", the inner is "is the new title present". The committed diff sets it to a real string;
//! the sibling all-null diffs in this tree leave the outer layer null.

use crate::artifacts::playbook::mutations::{apply_playbook_mutation, inverse_playbook_mutation, PlaybookMutation};
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> PlaybookMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn before() -> PlaybookSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> PlaybookSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`: the title becomes the payload's
/// own string, and BOTH composed child handles survive untouched — a title edit is not a content
/// edit, even though the narrative `document` projection happens to render the title too.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_playbook_mutation(&base, &mutation()).expect("change-title applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-title/changes-the-playbook-title: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.title.as_deref(), Some("Onboarding Playbook"), "the root title scalar takes the payload's value");
    assert_eq!((&snapshot.document.child_id, &snapshot.flow.child_id), (&base.document.child_id, &base.flow.child_id), "a title change must not re-mint either composed child handle");
}

/// 🔺️ The sparse delta is exactly the committed diff — the single most load-bearing assertion here:
/// it pins that `title` is the ONLY field `change-title` is allowed to write, with `document`,
/// `flow` and every other slot left null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-title/changes-the-playbook-title: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(outcome.messages().is_empty(), "a real title change raises no diagnostic at all");
}

/// 🔣️ The committed diff is itself canonical and decodes to playbook's own diff type, with the
/// double-`Option` title surviving the round trip as a present value.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(decoded.title, Some(Some("Onboarding Playbook".to_string())), "the committed diff sets a present title, not a cleared one");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-title/changes-the-playbook-title: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PlaybookDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <PlaybookDiff as protocol::MutationDiff<PlaybookSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-title/changes-the-playbook-title: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. The before-snapshot
/// carries an explicit `"title": null` — `PlaybookSnapshot::title` has no skip attribute, so an
/// untitled playbook serializes the key rather than omitting it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaybookSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-title/changes-the-playbook-title: committed {label} JSON is not canonical");
    }
    assert!(before().title.is_none(), "the before-snapshot is an untitled playbook, so this case really adds a title");
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-title/changes-the-playbook-title: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: a clean `applied` with no diagnostics — this is the only playbook
/// case in the tree that reaches neither a warning nor an error.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-title/changes-the-playbook-title declares an applied outcome");
    assert!(outcome.get("messages").is_none(), "a clean application declares no messages");
    let produced = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "the declared clean outcome must match the emitted one, got {:?}", produced.messages());
    assert_ne!(produced.diff(), &PlaybookDiff::default(), "an applied title change must carry a real delta");
}

/// ↩️ `change-title` is whole-document scoped and has no address, so its inverse is unconditional:
/// a `change-title` back to the BASE title — here `null`, which clears the title again.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_base_title() {
    let base = before();
    let inverse = inverse_playbook_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "change-title always undoes with exactly one step, got {inverse:?}");
    let PlaybookMutation::ChangeTitle(undo) = &inverse[0] else {
        panic!("change-title's inverse must be a change-title, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_title, None, "the inverse restores the untitled base state");
    let restored = apply_playbook_mutation(&apply_playbook_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "change-title/changes-the-playbook-title: inverse did not restore the before-snapshot");
}
