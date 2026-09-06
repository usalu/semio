//! 🧪️ `change-node-icon` fixture — `🎨️swaps-a-capsule-icon`.
//!
//! Both icon kinds are real Nakagin capsule icons.
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
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `change-node-icon` payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("change-node-icon applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-node-icon/swaps-a-capsule-icon: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.nodes.len(), 12, "change-node-icon/swaps-a-capsule-icon: the applied board must hold 12 node(s)");
    assert_eq!(snapshot.edges.len(), 10, "change-node-icon/swaps-a-capsule-icon: the applied board must hold 10 edge(s)");
}

/// ↩️ Applying `change-node-icon` then the inverse it derives from `before` restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_puzzle2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_puzzle2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-node-icon/swaps-a-capsule-icon: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "change-node-icon/swaps-a-capsule-icon: this kind undoes in exactly 1 step(s)");
}

/// 🔣️ Both committed snapshots and the committed `change-node-icon` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-node-icon/swaps-a-capsule-icon: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-node-icon/swaps-a-capsule-icon: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-node-icon` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-node-icon/swaps-a-capsule-icon declares an applied outcome");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-node-icon/swaps-a-capsule-icon: an applied vector carries no diagnostic, got {:?}", produced.messages());
}

/// 🔺️ The sparse delta `change-node-icon` produces is exactly the committed diff — it pins WHICH
/// collections and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-node-icon/swaps-a-capsule-icon: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["nodes"]["patched"][0]["id"].as_str(), Some("085a9625-3c6d-4e2d-921a-70ac66cbc56a"), "change-node-icon/swaps-a-capsule-icon: the diff must patch exactly the addressed node");
    assert!(committed["nodes"]["patched"][0]["patch"]["replacement"].is_object(), "change-node-icon/swaps-a-capsule-icon: a patch entry carries the whole replacement record");
    assert!(committed["nodes"]["reordered"].is_null(), "change-node-icon/swaps-a-capsule-icon: a null index must leave reordered unset");
    assert!(committed["edges"].is_null(), "change-node-icon/swaps-a-capsule-icon: this mutation must never touch the edges delta");
    assert!(committed["meta"].is_null(), "change-node-icon/swaps-a-capsule-icon: this mutation must never touch the document meta");
}

/// 🔣️ The committed `change-node-icon` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-node-icon/swaps-a-capsule-icon: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `change-node-icon` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle2d::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-node-icon/swaps-a-capsule-icon: committed diff did not carry before to after");
}
