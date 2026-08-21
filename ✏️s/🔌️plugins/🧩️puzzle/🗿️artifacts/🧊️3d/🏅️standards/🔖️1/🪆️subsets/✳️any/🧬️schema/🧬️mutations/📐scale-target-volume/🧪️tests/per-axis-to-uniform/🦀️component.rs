//! 🧪️ `scale-target-volume` fixture — `per-axis-to-uniform`.
//!
//! `volume-1` carries the `[x, y, z]` triple form of `Puzzle3dScale` in the base; the payload swaps
//! in the scalar form, so the wire shape collapses from an array to a bare number.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::puzzle3d::mutations::{apply_puzzle3d_mutation, inverse_puzzle3d_mutation};
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Puzzle3dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle3dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle3dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `scale-target-volume` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle3d_mutation(&mut snapshot, &mutation()).expect("scale-target-volume applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "scale-target-volume/per-axis-to-uniform: applied state differs from committed after-snapshot");
    let volume = snapshot.target_volumes.iter().find(|volume| volume.id == "volume-1").expect("volume-1 survives its scaling");
    assert_eq!(volume.scale, Some(crate::artifacts::puzzle3d::Puzzle3dScale::Uniform(0.5)), "scale-target-volume/per-axis-to-uniform: volume-1 did not take the scalar scale");
    assert_eq!(volume.orientation, before().target_volumes[0].orientation, "scale-target-volume/per-axis-to-uniform: scaling must not reorient the box");
}

/// ↩️ Applying `scale-target-volume` then the inverse it derives from `before` restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_puzzle3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_puzzle3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "scale-target-volume/per-axis-to-uniform: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `scale-target-volume` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "scale-target-volume/per-axis-to-uniform: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "scale-target-volume/per-axis-to-uniform: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `scale-target-volume` actually produces on this base.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "scale-target-volume/per-axis-to-uniform: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "scale-target-volume/per-axis-to-uniform: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "scale-target-volume/per-axis-to-uniform: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("scale-target-volume/per-axis-to-uniform: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `scale-target-volume` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "scale-target-volume/per-axis-to-uniform: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(committed["targetVolumes"]["patched"][0]["patch"]["replacement"]["scale"].is_number(), "scale-target-volume/per-axis-to-uniform: the uniform form must serialize as a bare number, not an array");
    assert!(committed["references"].is_null(), "scale-target-volume/per-axis-to-uniform: scaling a box never touches a reference plane");
}

/// 🔣️ The committed `scale-target-volume` diff is itself canonical and decodes to `Puzzle3dDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle3d::diff::Puzzle3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "scale-target-volume/per-axis-to-uniform: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `scale-target-volume` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle3d::diff::Puzzle3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle3d::diff::Puzzle3dDiff as protocol::MutationDiff<Puzzle3dSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "scale-target-volume/per-axis-to-uniform: committed diff did not carry before to after");
}
