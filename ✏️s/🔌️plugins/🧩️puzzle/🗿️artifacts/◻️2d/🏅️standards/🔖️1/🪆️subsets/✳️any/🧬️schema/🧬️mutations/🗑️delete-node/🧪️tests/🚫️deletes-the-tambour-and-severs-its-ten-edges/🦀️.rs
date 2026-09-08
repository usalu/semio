//! 🧪️ `delete-node` fixture — `🚫️deletes-the-tambour-and-severs-its-ten-edges`.
//!
//! Deleting the First Storey Tambour severs all ten capsule and core edges attached to its ten handles in one mutation.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the board is derived from the shipped
//! `🏗️nakagin-capsule-tower` example, not invented.

use crate::mutations::Puzzle2dMutation;
use crate::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::Puzzle2dSnapshot;

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

/// ▶️ The committed `delete-node` payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("delete-node applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.nodes.len(), 11, "delete-node/deletes-the-tambour-and-severs-its-ten-edges: the applied board must hold 11 node(s)");
    assert_eq!(snapshot.edges.len(), 0, "delete-node/deletes-the-tambour-and-severs-its-ten-edges: the applied board must hold 0 edge(s)");
}

/// ↩️ Applying `delete-node` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "delete-node/deletes-the-tambour-and-severs-its-ten-edges: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 11, "delete-node/deletes-the-tambour-and-severs-its-ten-edges: this kind undoes in exactly 11 step(s)");
}

/// 🔣️ Both committed snapshots and the committed `delete-node` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-node/deletes-the-tambour-and-severs-its-ten-edges: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-node/deletes-the-tambour-and-severs-its-ten-edges: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-node` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-node/deletes-the-tambour-and-severs-its-ten-edges declares an applied outcome");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: an applied vector carries no diagnostic, got {:?}", produced.messages());
}

/// 🔺️ The sparse delta `delete-node` produces is exactly the committed diff — it pins WHICH
/// collections and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-node/deletes-the-tambour-and-severs-its-ten-edges: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["nodes"]["removed"].as_array().map(Vec::len), Some(1), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: the diff must record exactly 1 removed node id(s)");
    assert_eq!(committed["nodes"]["removed"][0].as_str(), Some("17d5dec8-87b2-44a9-84ff-93b7e7419bdd"), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: removals are recorded as bare ids");
    assert!(committed["nodes"]["reordered"].is_null(), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: a null index must leave reordered unset");
    assert_eq!(committed["edges"]["removed"].as_array().map(Vec::len), Some(10), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: the diff must record exactly 10 removed edge id(s)");
    assert_eq!(committed["edges"]["removed"][0].as_str(), Some("015032e9-67ed-4736-adab-a6e10351079b"), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: removals are recorded as bare ids");
    assert!(committed["edges"]["reordered"].is_null(), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: a null index must leave reordered unset");
    assert!(committed["meta"].is_null(), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: this mutation must never touch the document meta");
}

/// 🔣️ The committed `delete-node` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-node/deletes-the-tambour-and-severs-its-ten-edges: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `delete-node` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-node/deletes-the-tambour-and-severs-its-ten-edges: committed diff did not carry before to after");
}
