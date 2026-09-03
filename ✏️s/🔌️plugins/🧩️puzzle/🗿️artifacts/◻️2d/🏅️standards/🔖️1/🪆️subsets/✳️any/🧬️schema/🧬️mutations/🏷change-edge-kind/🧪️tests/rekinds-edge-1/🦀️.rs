//! 🧪️ `change-edge-kind` fixture — `rekinds-edge-1`.
//!
//! Repoints `edge-1` at the `edge-kind-c` catalog row. Only `edgeKind` moves — the tips and the
//! eight connection parameters are carried through untouched.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

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

/// ▶️ The committed `change-edge-kind` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("change-edge-kind applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-edge-kind/rekinds-edge-1: applied state differs from committed after-snapshot");
    let edge = snapshot.edges.iter().find(|edge| edge.id == "edge-1").expect("edge-1 survives its kind change");
    assert_eq!(edge.edge_kind.as_deref(), Some("edge-kind-c"), "change-edge-kind/rekinds-edge-1: edge-1 still points at its old catalog row");
    assert_eq!((edge.source_tip.clone(), edge.target_tip.clone()), (before().edges[0].source_tip.clone(), before().edges[0].target_tip.clone()), "change-edge-kind/rekinds-edge-1: a kind change must not redraw the tips");
}

/// ↩️ Applying `change-edge-kind` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "change-edge-kind/rekinds-edge-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-edge-kind` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-edge-kind/rekinds-edge-1: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-edge-kind/rekinds-edge-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-edge-kind` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "change-edge-kind/rekinds-edge-1: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "change-edge-kind/rekinds-edge-1: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "change-edge-kind/rekinds-edge-1: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-edge-kind/rekinds-edge-1: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `change-edge-kind` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-edge-kind/rekinds-edge-1: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["edges"]["patched"][0]["patch"]["replacement"]["edgeKind"].as_str(), Some("edge-kind-c"), "change-edge-kind/rekinds-edge-1: the replacement must carry the new kind");
    assert!(committed["meta"].is_null(), "change-edge-kind/rekinds-edge-1: pointing at a catalog row must not rewrite the catalog itself");
}

/// 🔣️ The committed `change-edge-kind` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-edge-kind/rekinds-edge-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `change-edge-kind` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle2d::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-edge-kind/rekinds-edge-1: committed diff did not carry before to after");
}
