//! 🧪️ `replace-kind-catalogs` fixture — `📇️installs-handle-kind-catalog`.
//!
//! A whole-value swap of the typed catalog bundle: the base carries `None`, so the payload's
//! nodes/🐙️handles/edges/wires bundle is installed as one manifest-import gesture on `meta`.
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

/// ▶️ The committed `replace-kind-catalogs` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("replace-kind-catalogs applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-kind-catalogs/installs-handle-kind-catalog: applied state differs from committed after-snapshot");
    let catalogs = snapshot.meta.kind_catalogs.as_ref().expect("replace-kind-catalogs/installs-handle-kind-catalog: the catalog bundle was not installed");
    assert_eq!(catalogs.handles.len(), 1, "replace-kind-catalogs/installs-handle-kind-catalog: the handle-kind catalog is empty");
    assert_eq!(catalogs.handles[0].default_wire_kind, "wire-kind-a", "replace-kind-catalogs/installs-handle-kind-catalog: the catalog row did not survive the swap");
    assert_eq!(snapshot.meta.kind_compatibility, before().meta.kind_compatibility, "replace-kind-catalogs/installs-handle-kind-catalog: installing catalogs must not rewrite the compatibility table");
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
    assert_eq!(snapshot, base, "replace-kind-catalogs/installs-handle-kind-catalog: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `replace-kind-catalogs` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-kind-catalogs/installs-handle-kind-catalog: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-kind-catalogs/installs-handle-kind-catalog: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `replace-kind-catalogs` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-kind-catalogs/installs-handle-kind-catalog: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-kind-catalogs/installs-handle-kind-catalog: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-kind-catalogs/installs-handle-kind-catalog: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-kind-catalogs/installs-handle-kind-catalog: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `replace-kind-catalogs` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-kind-catalogs/installs-handle-kind-catalog: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["meta"]["kindCatalogs"]["handles"][0]["id"].as_str(), Some("handle-kind-a"), "replace-kind-catalogs/installs-handle-kind-catalog: the diff must publish the catalogs on meta");
    assert!(committed["nodes"].is_null(), "replace-kind-catalogs/installs-handle-kind-catalog: a catalog swap must not republish any document node");
}

/// 🔣️ The committed `replace-kind-catalogs` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-kind-catalogs/installs-handle-kind-catalog: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `replace-kind-catalogs` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle2d::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-kind-catalogs/installs-handle-kind-catalog: committed diff did not carry before to after");
}
