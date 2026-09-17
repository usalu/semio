//! 🧪️ `change-target-region-hidden` fixture — `🙈️hides-region-1`.
//!
//! Hiding the board's only region stops it constraining fill — `puzzle2d_regions_contain_bounds` reads visible regions only, so a board whose every region is hidden is unconstrained again.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1). Snapshot
//! canonicality is measured through `dsl::json` — the encoder this artifact's documents are written
//! with — not through `serde`, whose `f64` form spells a whole number `20.0` where every committed
//! board spells it `20`.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🙈change-target-region-hidden/🙈️hides-region-1/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🙈change-target-region-hidden/🙈️hides-region-1/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🙈change-target-region-hidden/🙈️hides-region-1/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🙈change-target-region-hidden/🙈️hides-region-1/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🙈change-target-region-hidden/🙈️hides-region-1/🎯️outcome/🔣️.json");

fn before() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `change-target-region-hidden` payload carries `before` to exactly the committed `after`, and lands the
/// change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("change-target-region-hidden applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-target-region-hidden/hides-region-1: applied state differs from committed after-snapshot");
    let region = snapshot.target_regions.iter().find(|region| region.id == "region-1").expect("region-1 survives being hidden");
    assert!(region.hidden, "change-target-region-hidden/hides-region-1: region-1 is not hidden");
    assert!(crate::puzzle2d_regions_contain_bounds(&snapshot.target_regions, [900.0, 900.0, 901.0, 901.0]), "change-target-region-hidden/hides-region-1: a board whose every region is hidden constrains nothing");
    assert!(!crate::puzzle2d_regions_contain_bounds(&before().target_regions, [900.0, 900.0, 901.0, 901.0]), "change-target-region-hidden/hides-region-1: the same box must be refused while the region is still visible");
}

/// ↩️ Applying `change-target-region-hidden` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "change-target-region-hidden/hides-region-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-target-region-hidden` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-target-region-hidden/hides-region-1: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-target-region-hidden/hides-region-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-target-region-hidden` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "change-target-region-hidden/hides-region-1: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "change-target-region-hidden/hides-region-1: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "change-target-region-hidden/hides-region-1: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-target-region-hidden/hides-region-1: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `change-target-region-hidden` produces is exactly the committed diff — it pins WHICH collections
/// and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-target-region-hidden/hides-region-1: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["targetRegions"]["patched"][0]["patch"]["replacement"]["hidden"].as_bool(), Some(true), "change-target-region-hidden/hides-region-1: the replacement must carry the new flag");
    assert!(committed["nodes"].is_null() && committed["edges"].is_null() && committed["meta"].is_null(), "change-target-region-hidden/hides-region-1: a target region is neither a node, an edge nor document meta");
}

/// 🔣️ The committed `change-target-region-hidden` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-target-region-hidden/hides-region-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `change-target-region-hidden` diff directly to `before` yields the committed `after` — the
/// diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-target-region-hidden/hides-region-1: committed diff did not carry before to after");
}
