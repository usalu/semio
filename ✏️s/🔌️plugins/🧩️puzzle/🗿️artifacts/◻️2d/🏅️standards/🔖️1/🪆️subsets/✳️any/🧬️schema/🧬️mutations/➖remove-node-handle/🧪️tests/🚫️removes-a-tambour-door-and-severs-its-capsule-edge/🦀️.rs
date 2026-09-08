//! 🧪️ `remove-node-handle` fixture — `🚫️removes-a-tambour-door-and-severs-its-capsule-edge`.
//!
//! Removing a handle is a NODE mutation that cascades into the edges attached to that handle: the tambour keeps its other nine doors and the capsule wire attached to this one is severed with it.
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

/// ▶️ The committed `remove-node-handle` payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("remove-node-handle applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.nodes.len(), 12, "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: the applied board must hold 12 node(s)");
    assert_eq!(snapshot.edges.len(), 9, "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: the applied board must hold 9 edge(s)");
}

/// ↩️ Applying `remove-node-handle` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 2, "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: this kind undoes in exactly 2 step(s)");
}

/// 🔣️ Both committed snapshots and the committed `remove-node-handle` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `remove-node-handle` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge declares an applied outcome");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: an applied vector carries no diagnostic, got {:?}", produced.messages());
}

/// 🔺️ The sparse delta `remove-node-handle` produces is exactly the committed diff — it pins WHICH
/// collections and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["nodes"]["patched"][0]["id"].as_str(), Some("17d5dec8-87b2-44a9-84ff-93b7e7419bdd"), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: the diff must patch exactly the addressed node");
    assert!(committed["nodes"]["patched"][0]["patch"]["replacement"].is_object(), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: a patch entry carries the whole replacement record");
    assert!(committed["nodes"]["reordered"].is_null(), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: a null index must leave reordered unset");
    assert_eq!(committed["edges"]["removed"].as_array().map(Vec::len), Some(1), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: the diff must record exactly 1 removed edge id(s)");
    assert_eq!(committed["edges"]["removed"][0].as_str(), Some("feba8972-394f-4ecc-923c-9f1701881fd0"), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: removals are recorded as bare ids");
    assert!(committed["edges"]["reordered"].is_null(), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: a null index must leave reordered unset");
    assert!(committed["meta"].is_null(), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: this mutation must never touch the document meta");
}

/// 🔣️ The committed `remove-node-handle` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `remove-node-handle` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle2d::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-node-handle/removes-a-tambour-door-and-severs-its-capsule-edge: committed diff did not carry before to after");
}
