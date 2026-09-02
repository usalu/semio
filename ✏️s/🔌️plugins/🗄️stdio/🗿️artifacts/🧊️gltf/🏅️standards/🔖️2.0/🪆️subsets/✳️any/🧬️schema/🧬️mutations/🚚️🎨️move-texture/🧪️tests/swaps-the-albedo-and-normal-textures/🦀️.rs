//! 🧪️ `move-texture` fixture — `swaps-the-albedo-and-normal-textures`.
//!
//! `repair(Textures, Move(0, 1))` walks every `textureInfo` slot on every material. Both materials here
//! bind a `baseColorTexture`, so both indices are swapped; the drop-to-`None` branch of that arm can only
//! fire on a deletion, never on a move.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::move_texture::diff::GltfMoveTextureDiff;
use crate::artifacts::gltf::schema::mutations::move_texture::GltfMoveTexturePayload;
use crate::artifacts::gltf::schema::mutations::move_texture::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "move-texture/swaps-the-albedo-and-normal-textures";
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
fn payload() -> GltfMoveTexturePayload {
    serde_json::from_str(MUTATION).expect("move-texture payload decodes")
}

/// ▶️ `move-texture` swaps the two textures and rewrites both baseColorTexture bindings.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("move-texture applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.textures[0].name.as_deref(), Some("normal"), "{CASE}: the normal texture must end up first");
    assert_eq!(snapshot.document.materials[0].pbr_metallic_roughness.as_ref().and_then(|pbr| pbr.base_color_texture.as_ref()).map(|info| info.index), Some(1), "{CASE}: repair(Textures, Move(0, 1)) must renumber the first material's baseColorTexture, never clear it");
}

/// ↩️ The inverse swaps them back so the albedo texture leads again.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("move-texture inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = inverse::apply_inverse(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(restored.document.textures[0].name.as_deref(), Some("albedo"), "{CASE}: the inverse must put the albedo texture back first");
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
            let snapshot = attempt.expect("move-texture declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("move-texture declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta is two indices; the material rewrites are repair effects.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("move-texture derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(produced.touched_paths, vec!["document/textures/0".to_string(), "document/textures/1".to_string()], "{CASE}: move-texture declares its source AND destination slot");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GltfMoveTextureDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert_eq!(decoded.id, "s.stdio.gltf.mutation.move-texture.v1", "{CASE}: the committed diff must carry move-texture's own canonical descriptor id");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GltfMoveTextureDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = diff::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
