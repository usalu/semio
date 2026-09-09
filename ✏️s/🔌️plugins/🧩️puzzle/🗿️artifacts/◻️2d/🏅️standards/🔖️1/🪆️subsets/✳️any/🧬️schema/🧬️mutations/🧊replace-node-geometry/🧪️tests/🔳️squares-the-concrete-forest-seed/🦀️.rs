//! 🧪️ `replace-node-geometry` fixture — `🔳️squares-the-concrete-forest-seed`.
//!
//! The shipped concrete-forest seed node leaves its circle for an explicit rectangle: the radius argument is null, so the member is dropped rather than kept.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the board is derived from the shipped
//! `🌲️concrete-forest` example, not invented.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊replace-node-geometry/🔳️squares-the-concrete-forest-seed/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊replace-node-geometry/🔳️squares-the-concrete-forest-seed/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊replace-node-geometry/🔳️squares-the-concrete-forest-seed/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊replace-node-geometry/🔳️squares-the-concrete-forest-seed/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊replace-node-geometry/🔳️squares-the-concrete-forest-seed/🎯️outcome/🔣️.json");

fn before() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `replace-node-geometry` payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("replace-node-geometry applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-node-geometry/squares-the-concrete-forest-seed: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.nodes.len(), 1, "replace-node-geometry/squares-the-concrete-forest-seed: the applied board must hold 1 node(s)");
    assert_eq!(snapshot.edges.len(), 0, "replace-node-geometry/squares-the-concrete-forest-seed: the applied board must hold 0 edge(s)");
}

/// ↩️ Applying `replace-node-geometry` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "replace-node-geometry/squares-the-concrete-forest-seed: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "replace-node-geometry/squares-the-concrete-forest-seed: this kind undoes in exactly 1 step(s)");
}

/// 🔣️ Both committed snapshots and the committed `replace-node-geometry` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-node-geometry/squares-the-concrete-forest-seed: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-node-geometry/squares-the-concrete-forest-seed: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `replace-node-geometry` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-node-geometry/squares-the-concrete-forest-seed declares an applied outcome");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "replace-node-geometry/squares-the-concrete-forest-seed: an applied vector carries no diagnostic, got {:?}", produced.messages());
}

/// 🔺️ The sparse delta `replace-node-geometry` produces is exactly the committed diff — it pins WHICH
/// collections and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-node-geometry/squares-the-concrete-forest-seed: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["nodes"]["patched"][0]["id"].as_str(), Some("seed-left-001"), "replace-node-geometry/squares-the-concrete-forest-seed: the diff must patch exactly the addressed node");
    assert!(committed["nodes"]["patched"][0]["patch"]["replacement"].is_object(), "replace-node-geometry/squares-the-concrete-forest-seed: a patch entry carries the whole replacement record");
    assert!(committed["nodes"]["reordered"].is_null(), "replace-node-geometry/squares-the-concrete-forest-seed: a null index must leave reordered unset");
    assert!(committed["edges"].is_null(), "replace-node-geometry/squares-the-concrete-forest-seed: this mutation must never touch the edges delta");
    assert!(committed["meta"].is_null(), "replace-node-geometry/squares-the-concrete-forest-seed: this mutation must never touch the document meta");
}

/// 🔣️ The committed `replace-node-geometry` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-node-geometry/squares-the-concrete-forest-seed: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `replace-node-geometry` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-node-geometry/squares-the-concrete-forest-seed: committed diff did not carry before to after");
}
