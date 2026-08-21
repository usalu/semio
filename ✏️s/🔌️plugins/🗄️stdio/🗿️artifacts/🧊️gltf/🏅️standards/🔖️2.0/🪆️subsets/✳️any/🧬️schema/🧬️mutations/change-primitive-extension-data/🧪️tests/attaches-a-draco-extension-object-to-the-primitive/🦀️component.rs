//! 🧪️ `change-primitive-extension-data` fixture — `attaches-a-draco-extension-object-to-the-primitive`.
//!
//! Same `GltfDataPresence` payload as the mesh-level leaf, but addressed at `(mesh, primitive)` and
//! writing `primitive.extensions`. Because `GltfMeshDiff` has no per-primitive sub-diff, the delta has
//! to carry the WHOLE replaced primitive list — that is the intended shape, not a bug.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::change_primitive_extension_data::mutation::GltfChangePrimitiveExtensionDataPayload;
use crate::artifacts::gltf::schema::mutations::change_primitive_extension_data::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "change-primitive-extension-data/attaches-a-draco-extension-object-to-the-primitive";
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
fn payload() -> GltfChangePrimitiveExtensionDataPayload {
    serde_json::from_str(MUTATION).expect("change-primitive-extension-data payload decodes")
}

/// ▶️ `change-primitive-extension-data` writes `primitive.extensions` and leaves the mesh-level slots alone.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("change-primitive-extension-data applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert!(snapshot.document.meshes[0].primitives[0].extensions.is_some(), "{CASE}: the primitive must gain an extensions object");
    assert!(snapshot.document.meshes[0].extensions.is_none(), "{CASE}: the MESH-level extensions slot is a different address and must stay absent");
}

/// ↩️ The inverse clears the primitive's extensions again.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("change-primitive-extension-data inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert!(restored.document.meshes[0].primitives[0].extensions.is_none(), "{CASE}: the inverse must return the primitive to ABSENT extensions");
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
            let snapshot = attempt.expect("change-primitive-extension-data declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("change-primitive-extension-data declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta replaces the primitive list — the only granularity `GltfMeshDiff` offers below the mesh.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("change-primitive-extension-data derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(produced.meshes.as_ref().is_some_and(|meshes| meshes.modified[0].diff.extensions.is_none()), "{CASE}: the MESH-level extensions field of the delta must stay absent");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert!(decoded.meshes.as_ref().is_some_and(|meshes| meshes.modified[0].diff.primitives.as_ref().is_some_and(|primitives| primitives.len() == 1)), "{CASE}: the committed delta must carry exactly one replaced primitive");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
