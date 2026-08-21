//! 🧪️ `reparent-node` fixture — `adopts-the-second-scene-root-under-the-first-node`.
//!
//! The only leaf that writes TWO collections at once. `apply()` first strips the child out of EVERY
//! node's `children` AND every scene's root list, then inserts it under the new parent — so the position
//! is validated against the parent's children EXCLUDING the child itself. `derive()` therefore returns
//! `GltfDiff { scenes: Some(between(..)), nodes: Some(between(..)) }`, and both slots are present in the
//! committed diff. `gltf.mutation.node-cycle` guards self-parenting and any descendant cycle.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::reparent_node::mutation::GltfReparentNodePayload;
use crate::artifacts::gltf::schema::mutations::reparent_node::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "reparent-node/adopts-the-second-scene-root-under-the-first-node";
const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> GltfSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> GltfSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn payload() -> GltfReparentNodePayload {
    serde_json::from_str(MUTATION).expect("reparent-node payload decodes")
}

/// ▶️ `reparent-node` detaches the child from the scene root list and re-attaches it under the parent.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("reparent-node applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.scenes[0].nodes, vec![0usize], "{CASE}: the reparented node must be STRIPPED from the scene root list, not merely duplicated");
    assert_eq!(snapshot.document.nodes[0].children, vec![1usize], "{CASE}: the parent must gain the child edge");
    assert_eq!(snapshot.document.nodes.len(), 2, "{CASE}: reparenting must never create or delete a node");
}

/// ↩️ The inverse is the mirrored two-collection delta that restores both the root list and the edge.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("reparent-node inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(restored.document.scenes[0].nodes, vec![0usize, 1], "{CASE}: the inverse must put the node back in the scene root list");
}

/// 🔣️ Both committed snapshots and this leaf's committed payload are canonical: decode→encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GltfSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{CASE}: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(payload()).expect("payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("payload reparses");
    assert_eq!(reencoded, original, "{CASE}: committed payload JSON is not canonical");
}

/// 🎯️ The declared outcome — and, when rejected, this leaf's own rejection code — matches what
/// the mutation actually produces for the committed payload.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let attempt = mutation::apply(&payload(), &before());
    match status {
        "applied" => {
            let snapshot = attempt.expect("reparent-node declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("reparent-node declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta writes BOTH `scenes` and `nodes` — the only leaf in this artifact that does.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("reparent-node derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(produced.scenes.is_some() && produced.nodes.is_some(), "{CASE}: reparent-node must declare both the scenes and the nodes slot");
    assert!(produced.meshes.is_none() && produced.accessors.is_none(), "{CASE}: reparent-node must write nothing beyond scenes and nodes");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert!(decoded.scenes.as_ref().is_some_and(|scenes| scenes.modified.len() == 1) && decoded.nodes.as_ref().is_some_and(|nodes| nodes.modified.len() == 1), "{CASE}: exactly one scene and one node entry may be modified");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
