//! 🧪️ `move-scene-root-node` fixture — `🎞️refuses-to-move-a-node-that-is-not-a-root-of-the-scene`.
//!
//! Like its `move-node-child` sibling this leaf has neither a `🔺️diff` nor an `↩️inverse` module, so it
//! carries `🔺️diff/🚫️.absent`. `validate()` looks the node up among the scene's roots and
//! raises `gltf.mutation.relation-absent` at the INTERPOLATED path `document/scenes/{scene}/nodes` when
//! it is not there — node 1 exists in `document/nodes` but is not a root of scene 0.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::move_scene_root_node::GltfMoveSceneRootNodePayload;
use crate::artifacts::gltf::schema::mutations::move_scene_root_node;
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "move-scene-root-node/refuses-to-move-a-node-that-is-not-a-root-of-the-scene";
const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🧬️operation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> GltfSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> GltfSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn payload() -> GltfMoveSceneRootNodePayload {
    serde_json::from_str(MUTATION).expect("move-scene-root-node payload decodes")
}

/// ▶️ `move-scene-root-node` refuses a node that exists but is not a root of the addressed scene.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let attempt = mutation::apply(&payload(), &before());
    assert!(attempt.is_err(), "{CASE}: this payload must be refused, not applied");
    assert_eq!(before(), expected_after(), "{CASE}: a rejected case must commit an after-snapshot identical to its before-snapshot");
    assert_eq!(before().document.scenes[0].nodes, vec![0usize], "{CASE}: scene 0 has exactly one root, and it is not the addressed node");
    assert_eq!(before().document.nodes.len(), 2, "{CASE}: the addressed node DOES exist — the refusal is about the relation, not the index");
}

/// ↩️ There is no `↩️inverse` module for this leaf and nothing was applied, so there is nothing to undo.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    assert!(mutation::apply(&payload(), &base).is_err(), "{CASE}: nothing was applied, so there is no state for an inverse to undo");
    assert_eq!(before().document.scenes[0].nodes, vec![0usize], "{CASE}: the root list must be exactly as committed");
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
            let snapshot = attempt.expect("move-scene-root-node declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("move-scene-root-node declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ This leaf ships no `🔺️diff` module at all, so a rejected case is the only honest fixture:
/// there is no diff type to serialize and `🔺️diff/🚫️.absent` stands in its place.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let attempt = mutation::apply(&payload(), &before());
    let rejection = attempt.expect_err("move-scene-root-node refuses this payload");
    assert_eq!(rejection.code, "gltf.mutation.relation-absent", "{CASE}: the refusal must be the relation guard, not an index guard");
    assert_eq!(rejection.path, "document/scenes/0/nodes", "{CASE}: this leaf INTERPOLATES the scene index into its rejection path");
}

/// 🔣️ Nothing to re-encode: the absent-diff marker is the committed artifact.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GltfMoveSceneRootNodePayload = serde_json::from_str(MUTATION).expect("payload decodes");
    assert_eq!((decoded.scene, decoded.node), (0, 1), "{CASE}: the committed payload must address scene 0 and node 1");
}

/// 🩹 With no diff there is nothing to replay; the snapshot must be exactly as committed.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let attempt = mutation::apply(&payload(), &before());
    assert!(attempt.is_err(), "{CASE}: refused payloads produce no diff to replay");
    assert_eq!(before(), expected_after(), "{CASE}: before and after must stay identical for a refused payload");
}
