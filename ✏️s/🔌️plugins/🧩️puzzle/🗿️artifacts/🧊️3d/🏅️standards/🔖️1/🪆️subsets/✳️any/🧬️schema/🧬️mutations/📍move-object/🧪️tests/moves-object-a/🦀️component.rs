//! 🧪️ `move-object` fixture — `moves-object-a`.
//!
//! An absolute world reposition of `object-a`'s `origin`. The builder clones the object and
//! rewrites that one `[f64; 3]` field, leaving the orientation and scale alone.
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

/// ▶️ The committed `move-object` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle3d_mutation(&mut snapshot, &mutation()).expect("move-object applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "move-object/moves-object-a: applied state differs from committed after-snapshot");
    let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives its own move");
    assert_eq!(object.origin, [1.0, 2.0, 3.0], "move-object/moves-object-a: object-a did not land on the committed origin");
    assert_eq!(object.orientation, before().objects[0].orientation, "move-object/moves-object-a: a move must not reorient the object");
}

/// ↩️ Applying `move-object` then the inverse it derives from `before` restores `before` exactly.
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
    assert_eq!(snapshot, base, "move-object/moves-object-a: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `move-object` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-object/moves-object-a: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-object/moves-object-a: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `move-object` actually produces on this base.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "move-object/moves-object-a: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "move-object/moves-object-a: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "move-object/moves-object-a: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("move-object/moves-object-a: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `move-object` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-object/moves-object-a: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["objects"]["patched"][0]["id"].as_str(), Some("object-a"), "move-object/moves-object-a: the diff must patch object-a and nothing else");
    assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["origin"][2].as_f64(), Some(3.0), "move-object/moves-object-a: the replacement must carry the new z");
    assert!(committed["attractions"].is_null(), "move-object/moves-object-a: moving an object must not re-solve its attractions here");
}

/// 🔣️ The committed `move-object` diff is itself canonical and decodes to `Puzzle3dDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle3d::diff::Puzzle3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-object/moves-object-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `move-object` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle3d::diff::Puzzle3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle3d::diff::Puzzle3dDiff as protocol::MutationDiff<Puzzle3dSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-object/moves-object-a: committed diff did not carry before to after");
}
