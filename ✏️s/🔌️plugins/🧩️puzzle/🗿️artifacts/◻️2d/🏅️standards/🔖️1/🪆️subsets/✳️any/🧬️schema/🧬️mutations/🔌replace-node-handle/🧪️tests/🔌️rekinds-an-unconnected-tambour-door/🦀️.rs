//! 🧪️ `replace-node-handle` fixture — `🔌️rekinds-an-unconnected-tambour-door`.
//!
//! The one vector that settles what `replace-node-handle` DOES: an unconnected door of the second tambour, re-kinded to a kind the compatibility relation admits, really replaces the addressed handle in place — it is not a no-op, and neither an attached edge nor an unadmitted kind is what a refusal would be about.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the board is derived from the shipped
//! `🏗️nakagin-capsule-tower` example, not invented.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
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

/// ▶️ The committed `replace-node-handle` payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("replace-node-handle applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-node-handle/rekinds-an-unconnected-tambour-door: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.nodes.len(), 12, "replace-node-handle/rekinds-an-unconnected-tambour-door: the applied board must hold 12 node(s)");
    assert_eq!(snapshot.edges.len(), 10, "replace-node-handle/rekinds-an-unconnected-tambour-door: the applied board must hold 10 edge(s)");
}

/// ↩️ Applying `replace-node-handle` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "replace-node-handle/rekinds-an-unconnected-tambour-door: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "replace-node-handle/rekinds-an-unconnected-tambour-door: this kind undoes in exactly 1 step(s)");
}

/// 🔣️ Both committed snapshots and the committed `replace-node-handle` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-node-handle/rekinds-an-unconnected-tambour-door: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-node-handle/rekinds-an-unconnected-tambour-door: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `replace-node-handle` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-node-handle/rekinds-an-unconnected-tambour-door declares an applied outcome");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "replace-node-handle/rekinds-an-unconnected-tambour-door: an applied vector carries no diagnostic, got {:?}", produced.messages());
}

/// 🔺️ The sparse delta `replace-node-handle` produces is exactly the committed diff — it pins WHICH
/// collections and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-node-handle/rekinds-an-unconnected-tambour-door: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["nodes"]["patched"][0]["id"].as_str(), Some("31be08e1-e75c-4024-86b4-c3c6d3939fbb"), "replace-node-handle/rekinds-an-unconnected-tambour-door: the diff must patch exactly the addressed node");
    assert!(committed["nodes"]["patched"][0]["patch"]["replacement"].is_object(), "replace-node-handle/rekinds-an-unconnected-tambour-door: a patch entry carries the whole replacement record");
    assert!(committed["nodes"]["reordered"].is_null(), "replace-node-handle/rekinds-an-unconnected-tambour-door: a null index must leave reordered unset");
    assert!(committed["edges"].is_null(), "replace-node-handle/rekinds-an-unconnected-tambour-door: this mutation must never touch the edges delta");
    assert!(committed["meta"].is_null(), "replace-node-handle/rekinds-an-unconnected-tambour-door: this mutation must never touch the document meta");
}

/// 🔣️ The committed `replace-node-handle` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-node-handle/rekinds-an-unconnected-tambour-door: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `replace-node-handle` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-node-handle/rekinds-an-unconnected-tambour-door: committed diff did not carry before to after");
}
