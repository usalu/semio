//! 🧪️ `unbind-default-scene` fixture — `clears-the-default-scene-pointer`.
//!
//! The payload is the EMPTY struct `GltfUnbindDefaultScenePayload {}`, so its committed JSON is `{}`.
//! `validate()` refuses a document with no default scene (`gltf.mutation.relation-absent`).
//! `derive()` writes `GltfDiff { scene: Some(None) }` — the one place in this artifact where a diff slot
//! cannot survive a JSON round trip, because `Option<Option<usize>>` encodes both `None` and `Some(None)`
//! as `null`. The last two tests below pin that explicitly rather than asserting a false fixed point.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::unbind_default_scene::GltfUnbindDefaultScenePayload;
use crate::artifacts::gltf::schema::mutations::unbind_default_scene::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "unbind-default-scene/clears-the-default-scene-pointer";
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
fn payload() -> GltfUnbindDefaultScenePayload {
    serde_json::from_str(MUTATION).expect("unbind-default-scene payload decodes")
}

/// ▶️ `unbind-default-scene` clears `document/scene` and leaves the scene itself in place.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("unbind-default-scene applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert!(snapshot.document.scene.is_none(), "{CASE}: the default-scene pointer must be cleared");
    assert_eq!(snapshot.document.scenes.len(), 1, "{CASE}: clearing the pointer must never delete the scene it named");
}

/// ↩️ The inverse writes the prior pointer back — `Some(Some(0))`, which does round-trip.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("unbind-default-scene inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(restored.document.scene, Some(0), "{CASE}: the inverse must restore scene 0 as the default");
    assert_eq!(inverse.scene, Some(Some(0)), "{CASE}: the INVERSE of this leaf is a set pointer, so unlike the forward diff it survives JSON intact");
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
            let snapshot = attempt.expect("unbind-default-scene declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("unbind-default-scene declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta is `scene: Some(None)`, which serializes to the committed `{"scene": null}`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("unbind-default-scene derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(matches!(produced.scene, Some(None)), "{CASE}: the delta must be Some(None) — an explicit clear, not an untouched slot");
    assert!(produced.scenes.is_none(), "{CASE}: unbind-default-scene may only ever write GltfDiff::scene");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.scene.is_none(), "{CASE}: decoding an explicit JSON null for the scene slot yields None, NOT Some(None) — this is the Option<Option<_>> limitation, pinned deliberately");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(original.get("scene"), Some(&serde_json::Value::Null), "{CASE}: the committed file must still carry the explicit null that the TYPED diff produces");
    assert_ne!(serde_json::to_value(&decoded).expect("diff re-encodes"), original, "{CASE}: the round trip is lossy by construction — if this ever becomes equal the limitation is gone and this pin must go with it");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    // 🧨️ `GltfDiff::scene` is an `Option<Option<usize>>` and this leaf writes `Some(None)`, which serde
    // encodes as a bare `null` — indistinguishable from the field being absent. Decoding the committed
    // JSON therefore yields `None`, NOT `Some(None)`, so the fixed-point assertion the other leaves use
    // would be a lie here. This test pins the limitation instead, and separately proves the TYPED delta
    // is still complete.
    let produced = diff::derive(&payload(), &before()).expect("unbind-default-scene derives its diff");
    assert_ne!(decoded, produced, "{CASE}: the JSON round trip is expected to LOSE this leaf's Option<Option<_>> slot — if it ever survives, drop this pin and assert equality instead");
    let applied = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&produced, &before()).expect("the TYPED diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "{CASE}: the TYPED delta still carries before to after — only its JSON encoding is lossy");
}
