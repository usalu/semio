//! 🧪️ `bind-node-skin` fixture — `binds-the-jointed-skin-to-the-rig-node`.
//!
//! The skin twin of `bind-node-camera`, with the wildcard path `document/nodes/*/skin`. glTF requires a
//! skinned node to also carry a mesh, but this leaf deliberately does NOT enforce that — `validate()`
//! bounds-checks node and skin and nothing more, which this fixture pins by binding a skin to a
//! mesh-less node.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::bind_node_skin::diff::GltfBindNodeSkinDiff;
use crate::artifacts::gltf::schema::mutations::bind_node_skin::GltfBindNodeSkinPayload;
use crate::artifacts::gltf::schema::mutations::bind_node_skin::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "bind-node-skin/binds-the-jointed-skin-to-the-rig-node";
const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🧬️operation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> GltfSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> GltfSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn payload() -> GltfBindNodeSkinPayload {
    serde_json::from_str(MUTATION).expect("bind-node-skin payload decodes")
}

/// ▶️ `bind-node-skin` sets `node.skin` without demanding that the node also carry a mesh.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("bind-node-skin applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.nodes[0].skin, Some(0), "{CASE}: the rig node must reference skin 0");
    assert!(snapshot.document.nodes[0].mesh.is_none(), "{CASE}: this leaf does not require a mesh — a skin may be bound to a mesh-less node");
}

/// ↩️ The inverse restores the node's prior skin binding, which here was absent.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("bind-node-skin inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = inverse::apply(&after, &inverse).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert!(restored.document.nodes[0].skin.is_none(), "{CASE}: the inverse must clear node.skin again");
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
            let snapshot = attempt.expect("bind-node-skin declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("bind-node-skin declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta names `document/nodes/*/skin`; the skin's joint list is never declared as touched.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("bind-node-skin derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(produced.touched_paths, vec!["document/nodes/*/skin".to_string()], "{CASE}: bind-node-skin declares only the wildcard node-skin path");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GltfBindNodeSkinDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert_eq!(decoded.operation.skin, 0, "{CASE}: the committed diff must echo the addressed skin");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GltfBindNodeSkinDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = diff::apply(&before(), &decoded).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
