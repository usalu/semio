//! 🧪️ `connect-handles` fixture — `⏸️keeps-an-edge-the-tower-already-holds`.
//!
//! A duplicate edge id is a WARNING-level no-op, not a rejection: the verb answers an empty diff and the board does not move.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the board is derived from the shipped
//! `🏗️nakagin-capsule-tower` example, not invented.

use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ⏸️ A no-op `connect-handles` still applies cleanly and leaves the board byte-identical.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "connect-handles/keeps-an-edge-the-tower-already-holds: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "connect-handles/keeps-an-edge-the-tower-already-holds: a no-op vector's two committed snapshots must be identical");
}

/// 🎯️ The declared warning is exactly what `connect-handles` emits: an APPLIED outcome carrying a
/// warning-level `mutation.no-op`, never a rejection.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "connect-handles/keeps-an-edge-the-tower-already-holds declares an applied outcome");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a no-op vector carries a diagnostic");
    assert_eq!(message.code.0, "mutation.no-op", "connect-handles/keeps-an-edge-the-tower-already-holds: a duplicate id is reported as a no-op, never as an error");
    assert_eq!(message.level, protocol::Severity::Warning, "connect-handles/keeps-an-edge-the-tower-already-holds: mutation.no-op is a warning — nothing to do is not a breach");
    assert_eq!(produced.diff(), &crate::artifacts::puzzle2d::diff::Puzzle2dDiff::default(), "connect-handles/keeps-an-edge-the-tower-already-holds: a no-op answers the default diff");
}

/// ↩️ `connect-handles`'s inverse is PAYLOAD-derived, so it is emitted even for a no-op — and it does NOT
/// restore this board, because the record the payload names was already there.
#[test]
fn inverse_is_payload_derived_and_does_not_restore() {
    let base = before();
    let inverse = inverse_puzzle2d_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "connect-handles/keeps-an-edge-the-tower-already-holds: this kind undoes in exactly one step, got {inverse:?}");
    let mut snapshot = base.clone();
    for step in &inverse {
        apply_puzzle2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_ne!(snapshot, base, "connect-handles/keeps-an-edge-the-tower-already-holds: undoing a no-op removes the record the board already held — that asymmetry is the point of this vector");
}

/// 🔣️ Both committed snapshots and the committed `connect-handles` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "connect-handles/keeps-an-edge-the-tower-already-holds: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "connect-handles/keeps-an-edge-the-tower-already-holds: committed mutation JSON is not canonical");
}

/// 🔺️ The committed diff declares nothing at all — every arm is null.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "connect-handles/keeps-an-edge-the-tower-already-holds: produced diff differs from the committed 🔺️diff/🔣️.json");
    for member in ["nodes", "edges", "meta", "camera", "schema"] {
        assert!(committed[member].is_null(), "connect-handles/keeps-an-edge-the-tower-already-holds: a no-op diff declares nothing, yet {member} is populated");
    }
}

/// 🩹 Applying the committed no-op diff to `before` yields `before` again.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle2d::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "connect-handles/keeps-an-edge-the-tower-already-holds: committed diff did not carry before to after");
}
