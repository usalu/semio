//! 🧪️ `delete-target-volume` fixture — `🌲️clears-the-seed-bay`.
//!
//! Real-world vector: clearing the `🌲️concrete-forest` seed bay leaves the example unconstrained, and the empty collection leaves the wire form entirely.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1). The
//! `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived from it
//! by `fixtures generate` and are asserted by the shared codec-matrix harness, not here.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle5d_mutation, inverse_puzzle5d_mutation};
use crate::Puzzle5dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪦delete-target-volume/🌲️clears-the-seed-bay/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪦delete-target-volume/🌲️clears-the-seed-bay/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪦delete-target-volume/🌲️clears-the-seed-bay/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪦delete-target-volume/🌲️clears-the-seed-bay/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪦delete-target-volume/🌲️clears-the-seed-bay/🎯️outcome/🔣️.json");

fn before() -> Puzzle5dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle5dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle5dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `delete-target-volume` payload carries `before` to exactly the committed `after`, and lands the
/// change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle5d_mutation(&mut snapshot, &mutation()).expect("delete-target-volume applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-target-volume/clears-the-seed-bay: applied state differs from committed after-snapshot");
    assert!(snapshot.target_volumes.is_empty(), "delete-target-volume/clears-the-seed-bay: the forest must be left unconstrained");
}

/// ↩️ Applying `delete-target-volume` then the inverse it derives from `before` restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle5d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_puzzle5d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_puzzle5d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-target-volume/clears-the-seed-bay: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `delete-target-volume` payload are already canonical: decode→encode
/// is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle5dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-target-volume/clears-the-seed-bay: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-target-volume/clears-the-seed-bay: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-target-volume` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "delete-target-volume/clears-the-seed-bay: this vector declares an applied outcome");
    let mut snapshot = before();
    apply_puzzle5d_mutation(&mut snapshot, &mutation()).expect("delete-target-volume/clears-the-seed-bay: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta `delete-target-volume` produces is exactly the committed diff — it pins WHICH collections and
/// fields this mutation is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-target-volume/clears-the-seed-bay: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(committed["parts"].is_null(), "delete-target-volume/clears-the-seed-bay: a target volume is not a part");
    assert!(committed["fasteners"].is_null(), "delete-target-volume/clears-the-seed-bay: a target volume is not a fastener");
}

/// 🔣️ The committed `delete-target-volume` diff is itself canonical and decodes to `Puzzle5dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-target-volume/clears-the-seed-bay: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `delete-target-volume` diff directly to `before` yields the committed `after` — the diff
/// is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff as protocol::MutationDiff<Puzzle5dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-target-volume/clears-the-seed-bay: committed diff did not carry before to after");
}
