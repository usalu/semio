//! 🧪️ `change-vortex-vortex-kind` fixture — `🕳️rekinds-front-vortex-as-hatch`: the `vortex-front` template is re-pointed at the `hatch` kind
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::mutations::{apply_block3d_mutation, inverse_block3d_mutation};
use crate::artifacts::block3d::Block3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Block3dSnapshot {
    crate::artifacts::block3d::validate_vortex_kind_catalog(&[
        crate::artifacts::block3d::Block3dVortexKind { id: "door".into(), name: "door".into(), label: "Door".into(), color: "hsl(206 52% 48%)".into(), default_cable_kind: "cable.link".into() },
        crate::artifacts::block3d::Block3dVortexKind { id: "hatch".into(), name: "hatch".into(), label: "Hatch".into(), color: "hsl(37 52% 48%)".into(), default_cable_kind: "cable.bus".into() },
    ]);
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Block3dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Block3dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`: the `vortex-front` template is re-pointed at the `hatch` kind
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_block3d_mutation(&mut snapshot, &mutation()).expect("change-vortex-vortex-kind applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: applied state differs from committed after-snapshot");
    assert_eq!((snapshot.vortices[0].vortex_kind.as_str(), snapshot.vortex_kind_extra.len()), ("hatch", 2), "change-vortex-vortex-kind must re-point the template without adding or removing a catalogue row");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_block3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_block3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_block3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Block3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_block3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `change-vortex-vortex-kind` is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Block3dMutation as protocol::Mutation<Block3dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::block3d::diff::Block3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::block3d::diff::Block3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::block3d::diff::Block3dDiff as protocol::MutationDiff<Block3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-vortex-vortex-kind/rekinds-front-vortex-as-hatch: committed diff did not carry before to after");
}
