//! 🧪️ `create-scene` fixture — `🎞️inserts-an-empty-scene-ahead-of-the-default-scene`.
//!
//! This leaf is the descriptor-backed one: its diff carries a full pre-state witness
//! (`expectedSceneCount`, `expectedDefaultSceneBefore`, `expectedScenesBefore`) and `validate()`
//! rejects with `gltf.mutation.stale-diff` if any of them no longer matches. `default_after(Some(0), 0)`
//! is `Some(1)` because the default scene sits at or after the insertion point, so `paths()` emits the
//! second entry `document/scene` alongside `document/scenes/0`. The inserted `scene` must be exactly
//! `GltfScene::default()` or `validate()` raises `gltf.mutation.invalid-created-scene`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::create_scene::diff::GltfCreateSceneDiff;
use crate::artifacts::gltf::schema::mutations::create_scene::GltfCreateScenePayload;
use crate::artifacts::gltf::schema::mutations::create_scene::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "create-scene/inserts-an-empty-scene-ahead-of-the-default-scene";
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
fn payload() -> GltfCreateScenePayload {
    serde_json::from_str(MUTATION).expect("create-scene payload decodes")
}

/// ▶️ `create-scene` inserts the canonical empty scene and re-points `document/scene` past it.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("create-scene applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.scene, Some(1), "{CASE}: default_after(Some(0), 0) is Some(1) — the default scene must follow the insertion");
    assert_eq!(snapshot.document.scenes[0], Default::default(), "{CASE}: create-scene may only insert GltfScene::default()");
}

/// ↩️ The inverse removes the created scene and restores `document/scene` to 0.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&base, payload().position).expect("create-scene inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = inverse::apply(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(restored.document.scene, Some(0), "{CASE}: the inverse must restore the ORIGINAL default-scene index, not recompute it");
    assert_eq!(inverse.touched_paths, vec!["document/scenes/0".to_string(), "document/scene".to_string()], "{CASE}: the inverse touches both the scene slot and the default-scene pointer");
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
            let snapshot = attempt.expect("create-scene declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("create-scene declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta pins the whole pre-state so a stale application is refused rather than silently reapplied.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&before(), payload().position).expect("create-scene derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(produced.expected_scene_count, 1, "{CASE}: the diff must witness the pre-state scene count");
    assert_eq!(produced.expected_default_scene_before, Some(0), "{CASE}: the diff must witness the pre-state default scene");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GltfCreateSceneDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert_eq!(decoded.touched_paths, vec!["document/scenes/0".to_string(), "document/scene".to_string()], "{CASE}: paths() must add document/scene because the default scene is remapped");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GltfCreateSceneDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = diff::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
