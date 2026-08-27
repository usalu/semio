//! 🧪️ `change-node-transform` fixture — `replaces-a-translation-with-a-scaled-trs-transform`.
//!
//! `GltfNodeTransform` is tagged on `kind` (`matrix` / `trs`), and the `Trs` variant's three fields carry
//! no `skip_serializing_if`, so `rotation: null` MUST be present in the committed payload. `apply()` is
//! mutually exclusive by construction: the `Trs` arm clears `matrix` outright, the `Matrix` arm clears
//! all three TRS slots. `validate()` refuses any non-finite component with
//! `gltf.mutation.invalid-transform`. The committed values are dyadic so the canonical fixed point holds.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::change_node_transform::GltfTransformNodePayload;
use crate::artifacts::gltf::schema::mutations::change_node_transform::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "change-node-transform/replaces-a-translation-with-a-scaled-trs-transform";
const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🧬️operation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> GltfSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> GltfSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn payload() -> GltfTransformNodePayload {
    serde_json::from_str(MUTATION).expect("change-node-transform payload decodes")
}

/// ▶️ `change-node-transform` writes the whole TRS triple and clears `matrix` in the same step.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("change-node-transform applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.nodes[0].translation, Some([0.5, 0.0, 0.0]), "{CASE}: the translation must be replaced with the dyadic payload vector");
    assert_eq!(snapshot.document.nodes[0].scale, Some([2.0, 2.0, 2.0]), "{CASE}: the scale must be set from the payload");
    assert!(snapshot.document.nodes[0].matrix.is_none() && snapshot.document.nodes[0].rotation.is_none(), "{CASE}: the Trs arm must clear matrix and leave the null rotation as None");
}

/// ↩️ The inverse is the mirrored nodes delta that restores the original translation and clears the scale.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("change-node-transform inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(restored.document.nodes[0].translation, Some([1.0, 2.0, 3.0]), "{CASE}: the inverse must restore the original translation");
    assert!(restored.document.nodes[0].scale.is_none(), "{CASE}: the inverse must clear the scale this leaf introduced");
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
            let snapshot = attempt.expect("change-node-transform declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("change-node-transform declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ Only the two slots that actually moved appear — `rotation` and `matrix` were already None on both sides.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("change-node-transform derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(produced.nodes.as_ref().is_some_and(|nodes| nodes.modified[0].diff.rotation.is_none()), "{CASE}: rotation was None before AND after, so GltfNodeDiff must leave it unset");
    assert!(produced.nodes.as_ref().is_some_and(|nodes| nodes.modified[0].diff.matrix.is_none()), "{CASE}: matrix was None before AND after, so the delta must not restate it");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert_eq!(decoded.nodes.as_ref().and_then(|nodes| nodes.modified[0].diff.scale), Some(Some([2.0, 2.0, 2.0])), "{CASE}: the committed scale must decode back to Some(Some([2, 2, 2]))");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
