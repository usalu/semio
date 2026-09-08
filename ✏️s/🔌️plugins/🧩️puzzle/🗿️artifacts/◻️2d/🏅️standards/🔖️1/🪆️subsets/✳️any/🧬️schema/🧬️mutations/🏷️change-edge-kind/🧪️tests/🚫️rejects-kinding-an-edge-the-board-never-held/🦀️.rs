//! 🧪️ `change-edge-kind` fixture — `🚫️rejects-kinding-an-edge-the-board-never-held`.
//!
//! One committed specification vector for this kind, derived from a shipped example board.
//!
//! Contract D6: a rejected vector commits no `🔺️diff/🔣️.json` at all — the empty
//! `🔺️diff/🚫️.absent` sentinel beside this file stands in its place, so nothing here invents an
//! empty patch. The board is derived from the shipped `🏗️nakagin-capsule-tower` example, not invented.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️.absent");

fn before() -> Puzzle2dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A refused `change-edge-kind` still applies cleanly — its diff is the default one — and leaves the
/// board at the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: a rejected vector's two committed snapshots must be identical");
}

/// 🚨️ The refusal is exactly the declared one: default diff, one message, its code, its level and
/// its target address.
#[test]
fn the_refusal_is_the_declared_one() {
    assert!(DIFF_ABSENT.is_empty(), "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff::default(), "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: a refusing diff builder answers the default diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: the refusal code is fixed by this vector");
    assert_eq!(messages[0].level, protocol::Severity::Error, "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: the refusal level is fixed by this vector");
    assert_eq!(messages[0].target, vec!["02d43618-0e15-482b-9335-702d795f8074".to_string()], "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: the diagnostic addresses exactly what the payload named");
}

/// ↩️ A verb that could not find its target has nothing to undo, so its inverse is empty.
#[test]
fn inverse_of_a_refusal() {
    let inverse = inverse_puzzle2d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 0, "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed `change-edge-kind` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "change-edge-kind/rejects-kinding-an-edge-the-board-never-held declares a rejected outcome");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "change-edge-kind/rejects-kinding-an-edge-the-board-never-held: the declared path must match the emitted target");
}
