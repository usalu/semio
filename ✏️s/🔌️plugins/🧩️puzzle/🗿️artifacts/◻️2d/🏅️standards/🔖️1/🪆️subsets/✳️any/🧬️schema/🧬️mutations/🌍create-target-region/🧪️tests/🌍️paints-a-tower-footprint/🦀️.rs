//! 🧪️ `create-target-region` fixture — `🌍️paints-a-tower-footprint`.
//!
//! The real-world vector: an Area Brush stroke over the shipped `concrete-forest` seed board, painting one footprint that contains the seed node's whole circle (centre 230.73/93.53, radius 24).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1). Snapshot
//! canonicality is measured through `dsl::json` — the encoder this artifact's documents are written
//! with — not through `serde`, whose `f64` form spells a whole number `20.0` where every committed
//! board spells it `20`.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-region/🌍️paints-a-tower-footprint/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-region/🌍️paints-a-tower-footprint/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-region/🌍️paints-a-tower-footprint/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-region/🌍️paints-a-tower-footprint/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-region/🌍️paints-a-tower-footprint/🎯️outcome/🔣️.json");

fn before() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `create-target-region` payload carries `before` to exactly the committed `after`, and lands the
/// change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("create-target-region applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-target-region/paints-a-tower-footprint: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.target_regions.len(), 1, "create-target-region/paints-a-tower-footprint: the footprint was not painted");
    let node = snapshot.nodes.first().expect("the concrete-forest seed survives a region paint");
    let radius = node.radius.unwrap_or_default();
    assert!(crate::puzzle2d_regions_contain_bounds(&snapshot.target_regions, [node.x - radius, node.y - radius, node.x + radius, node.y + radius]), "create-target-region/paints-a-tower-footprint: the painted footprint must contain the seed node it was drawn around");
    assert_eq!(snapshot.nodes.len(), before().nodes.len(), "create-target-region/paints-a-tower-footprint: painting a region must not touch the board's nodes");
}

/// ↩️ Applying `create-target-region` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "create-target-region/paints-a-tower-footprint: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `create-target-region` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-target-region/paints-a-tower-footprint: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-target-region/paints-a-tower-footprint: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-target-region` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-target-region/paints-a-tower-footprint: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-target-region/paints-a-tower-footprint: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-target-region/paints-a-tower-footprint: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-target-region/paints-a-tower-footprint: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `create-target-region` produces is exactly the committed diff — it pins WHICH collections
/// and fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-target-region/paints-a-tower-footprint: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["targetRegions"]["added"][0]["id"].as_str(), Some("region-seed-left"), "create-target-region/paints-a-tower-footprint: the diff must carry the painted region in targetRegions.added");
    assert!(committed["nodes"].is_null() && committed["edges"].is_null() && committed["meta"].is_null(), "create-target-region/paints-a-tower-footprint: a target region is neither a node, an edge nor document meta");
}

/// 🔣️ The committed `create-target-region` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-target-region/paints-a-tower-footprint: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `create-target-region` diff directly to `before` yields the committed `after` — the
/// diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-target-region/paints-a-tower-footprint: committed diff did not carry before to after");
}
