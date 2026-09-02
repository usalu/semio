//! 🧪️ `create-target-volume` fixture — `appends-volume-2`.
//!
//! Adds a second fill-constraining box. `volume-2` carries neither orientation nor scale, so both
//! `Option` fields stay absent from the wire form the builder puts in `added`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::mutations::{apply_puzzle3d_mutation, inverse_puzzle3d_mutation};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Puzzle3dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle3dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle3dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `create-target-volume` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle3d_mutation(&mut snapshot, &mutation()).expect("create-target-volume applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-target-volume/appends-volume-2: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.target_volumes.len(), 2, "create-target-volume/appends-volume-2: volume-2 was not appended");
    let volume = snapshot.target_volumes.iter().find(|volume| volume.id == "volume-2").expect("volume-2 exists after creation");
    assert_eq!((volume.orientation, volume.scale), (None, None), "create-target-volume/appends-volume-2: an unposed volume must not gain a default orientation or scale");
}

/// ↩️ Applying `create-target-volume` then the inverse it derives from `before` restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_puzzle3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_puzzle3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-target-volume/appends-volume-2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `create-target-volume` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-target-volume/appends-volume-2: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-target-volume/appends-volume-2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-target-volume` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-target-volume/appends-volume-2: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-target-volume/appends-volume-2: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-target-volume/appends-volume-2: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-target-volume/appends-volume-2: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `create-target-volume` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-target-volume/appends-volume-2: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["targetVolumes"]["added"][0]["id"].as_str(), Some("volume-2"), "create-target-volume/appends-volume-2: the diff must carry volume-2 in targetVolumes.added");
    assert!(committed["objects"].is_null(), "create-target-volume/appends-volume-2: a target volume is not an object");
}

/// 🔣️ The committed `create-target-volume` diff is itself canonical and decodes to `Puzzle3dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle3d::diff::Puzzle3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-target-volume/appends-volume-2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `create-target-volume` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle3d::diff::Puzzle3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle3d::diff::Puzzle3dDiff as protocol::MutationDiff<Puzzle3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-target-volume/appends-volume-2: committed diff did not carry before to after");
}
