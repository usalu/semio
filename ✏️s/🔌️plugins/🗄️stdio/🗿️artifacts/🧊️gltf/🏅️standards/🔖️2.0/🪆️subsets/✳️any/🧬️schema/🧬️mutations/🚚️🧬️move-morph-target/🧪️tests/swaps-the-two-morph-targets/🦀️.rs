//! 🧪️ `move-morph-target` fixture — `swaps-the-two-morph-targets`.
//!
//! `validate()` bounds-checks target AND destination with `checked_index` (both must ADDRESS an existing
//! target, so append is impossible here) and refuses `target == position` with
//! `gltf.mutation.no-observable-change`. Morph-target order is semantically load-bearing because
//! `mesh.weights[i]` is positional.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::move_morph_target::GltfMoveMorphTargetPayload;
use crate::artifacts::gltf::schema::mutations::move_morph_target::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "move-morph-target/swaps-the-two-morph-targets";
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
fn payload() -> GltfMoveMorphTargetPayload {
    serde_json::from_str(MUTATION).expect("move-morph-target payload decodes")
}

/// ▶️ `move-morph-target` reorders the target list without touching either target's contents.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("move-morph-target applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.meshes[0].primitives[0].targets[0].0, vec![("POSITION".to_string(), 2usize)], "{CASE}: the second target must move to position 0 with its accessor binding intact");
    assert_eq!(snapshot.document.meshes[0].primitives[0].targets.len(), 2, "{CASE}: a move must neither add nor drop a target");
}

/// ↩️ The inverse swaps the two targets back.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("move-morph-target inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(restored.document.meshes[0].primitives[0].targets[0].0, vec![("POSITION".to_string(), 1usize)], "{CASE}: the inverse must restore the original target order");
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
            let snapshot = attempt.expect("move-morph-target declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("move-morph-target declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta replaces the primitive list with the reordered targets.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("move-morph-target derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(produced.meshes.as_ref().is_some_and(|meshes| meshes.modified[0].diff.weights.is_none()), "{CASE}: moving a target must not restate the mesh weight vector");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert!(decoded.meshes.is_some() && decoded.accessors.is_none(), "{CASE}: the committed diff must name meshes and only meshes");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
