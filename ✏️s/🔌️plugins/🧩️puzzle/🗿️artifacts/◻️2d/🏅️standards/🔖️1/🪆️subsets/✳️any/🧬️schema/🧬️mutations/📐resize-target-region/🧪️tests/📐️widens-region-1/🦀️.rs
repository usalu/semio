//! 🧪️ `resize-target-region` fixture — `📐️widens-region-1`.
//!
//! An absolute FINAL-state extent. The committed payload restates the unchanged `height` alongside the new `width` — the verb is a whole-extent write, never a per-axis delta.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1). Snapshot
//! canonicality is measured through `dsl::json` — the encoder this artifact's documents are written
//! with — not through `serde`, whose `f64` form spells a whole number `20.0` where every committed
//! board spells it `20`.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-target-region/📐️widens-region-1/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-target-region/📐️widens-region-1/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-target-region/📐️widens-region-1/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-target-region/📐️widens-region-1/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-target-region/📐️widens-region-1/🎯️outcome/🔣️.json");

fn before() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `resize-target-region` payload carries `before` to exactly the committed `after`, and lands the
/// change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("resize-target-region applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "resize-target-region/widens-region-1: applied state differs from committed after-snapshot");
    let region = snapshot.target_regions.iter().find(|region| region.id == "region-1").expect("region-1 survives its own resize");
    assert_eq!((region.width, region.height), (120.5, 60.5), "resize-target-region/widens-region-1: region-1 did not take the committed extent");
    assert_eq!((region.x, region.y), (-12.5, -12.5), "resize-target-region/widens-region-1: a resize must not move the region's minimum corner");
}

/// ↩️ Applying `resize-target-region` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "resize-target-region/widens-region-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `resize-target-region` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "resize-target-region/widens-region-1: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "resize-target-region/widens-region-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `resize-target-region` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "resize-target-region/widens-region-1: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "resize-target-region/widens-region-1: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "resize-target-region/widens-region-1: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("resize-target-region/widens-region-1: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `resize-target-region` produces is exactly the committed diff — it pins WHICH collections
/// and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "resize-target-region/widens-region-1: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["targetRegions"]["patched"][0]["id"].as_str(), Some("region-1"), "resize-target-region/widens-region-1: the diff must patch region-1 and nothing else");
    assert!(committed["nodes"].is_null() && committed["edges"].is_null() && committed["meta"].is_null(), "resize-target-region/widens-region-1: a target region is neither a node, an edge nor document meta");
}

/// 🔣️ The committed `resize-target-region` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "resize-target-region/widens-region-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `resize-target-region` diff directly to `before` yields the committed `after` — the
/// diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "resize-target-region/widens-region-1: committed diff did not carry before to after");
}
