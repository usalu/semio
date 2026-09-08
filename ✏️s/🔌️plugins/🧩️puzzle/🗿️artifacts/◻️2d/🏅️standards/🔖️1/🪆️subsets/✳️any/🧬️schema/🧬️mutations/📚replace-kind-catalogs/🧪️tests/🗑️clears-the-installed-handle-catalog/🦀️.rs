//! 🧪️ `replace-kind-catalogs` fixture — `🗑️clears-the-installed-handle-catalog`.
//!
//! The vector that settles the null argument: `replace-kind-catalogs` with `newCatalogs: null` is ACCEPTED and REMOVES the member, so the inverse of an install is a real clear rather than an inexpressible one.
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

/// ▶️ The committed `replace-kind-catalogs` payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("replace-kind-catalogs applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-kind-catalogs/clears-the-installed-handle-catalog: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.nodes.len(), 12, "replace-kind-catalogs/clears-the-installed-handle-catalog: the applied board must hold 12 node(s)");
    assert_eq!(snapshot.edges.len(), 10, "replace-kind-catalogs/clears-the-installed-handle-catalog: the applied board must hold 10 edge(s)");
}

/// ↩️ Applying `replace-kind-catalogs` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "replace-kind-catalogs/clears-the-installed-handle-catalog: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "replace-kind-catalogs/clears-the-installed-handle-catalog: this kind undoes in exactly 1 step(s)");
}

/// 🔣️ Both committed snapshots and the committed `replace-kind-catalogs` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-kind-catalogs/clears-the-installed-handle-catalog: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-kind-catalogs/clears-the-installed-handle-catalog: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `replace-kind-catalogs` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-kind-catalogs/clears-the-installed-handle-catalog declares an applied outcome");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "replace-kind-catalogs/clears-the-installed-handle-catalog: an applied vector carries no diagnostic, got {:?}", produced.messages());
}

/// 🔺️ The sparse delta `replace-kind-catalogs` produces is exactly the committed diff — it pins WHICH
/// collections and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-kind-catalogs/clears-the-installed-handle-catalog: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(committed["nodes"].is_null(), "replace-kind-catalogs/clears-the-installed-handle-catalog: this mutation must never touch the nodes delta");
    assert!(committed["edges"].is_null(), "replace-kind-catalogs/clears-the-installed-handle-catalog: this mutation must never touch the edges delta");
    assert!(committed["meta"].is_object(), "replace-kind-catalogs/clears-the-installed-handle-catalog: this mutation rewrites the document meta");
}

/// 🔣️ The committed `replace-kind-catalogs` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-kind-catalogs/clears-the-installed-handle-catalog: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `replace-kind-catalogs` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-kind-catalogs/clears-the-installed-handle-catalog: committed diff did not carry before to after");
}
