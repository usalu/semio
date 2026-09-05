//! 🧪️ `change-material-double-sided` fixture — `🪞️makes-the-default-material-double-sided`.
//!
//! The second descriptor-backed material leaf: same witness-diff contract as
//! `change-material-alpha-mode` but over the `bool` `doubleSided`. `validate()` refuses writing the value
//! the material already holds, so with `GltfMaterial::default()`'s `double_sided = false` the only legal
//! payload is `true`. `is_false` is the field's `skip_serializing_if`, which is why `false` never appears
//! on the wire and `true` always does.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::change_material_double_sided::diff::GltfChangeMaterialDoubleSidedDiff;
use crate::artifacts::gltf::schema::mutations::change_material_double_sided::GltfChangeMaterialDoubleSidedPayload;
use crate::artifacts::gltf::schema::mutations::change_material_double_sided::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "change-material-double-sided/makes-the-default-material-double-sided";
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
fn payload() -> GltfChangeMaterialDoubleSidedPayload {
    serde_json::from_str(MUTATION).expect("change-material-double-sided payload decodes")
}

/// ▶️ `change-material-double-sided` flips `doubleSided` and no other material field.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    mutation::apply(&mut snapshot, &payload()).expect("change-material-double-sided applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert!(snapshot.document.materials[0].double_sided, "{CASE}: the material must become double sided");
    assert_eq!(snapshot.document.materials[0].alpha_mode, crate::artifacts::gltf::schema::snapshot::GltfAlphaMode::Opaque, "{CASE}: alphaMode belongs to a different leaf and must stay OPAQUE");
}

/// ↩️ `reconstruct` witnesses the forward result and writes the prior `false` back.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::reconstruct(&payload(), &base).expect("change-material-double-sided inverse reconstructs from the exact base");
    let mut restored = base.clone();
    mutation::apply(&mut restored, &payload()).expect("forward applies");
    inverse.apply(&mut restored).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert!(inverse.expected_double_sided, "{CASE}: the inverse must expect to find the post-state true");
    assert!(!inverse.double_sided, "{CASE}: the inverse must write false back");
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
    let mut snapshot = before();
    let attempt = mutation::apply(&mut snapshot, &payload());
    match status {
        "applied" => {
            attempt.expect("change-material-double-sided declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("change-material-double-sided declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
            assert_eq!(snapshot, before(), "{CASE}: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta carries the `false` pre-state witness even though `false` is never serialized on the material itself.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("change-material-double-sided derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(!produced.expected_double_sided, "{CASE}: the diff must witness the false pre-state explicitly");
    assert_eq!(produced.touched_paths, vec!["document/materials/0/doubleSided".to_string()], "{CASE}: the descriptor family interpolates the material index into a CONCRETE path");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GltfChangeMaterialDoubleSidedDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert!(decoded.double_sided && !decoded.expected_double_sided, "{CASE}: the committed diff must carry both the false pre-state and the true post-state");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GltfChangeMaterialDoubleSidedDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = {
        let mut snapshot = before();
        decoded.apply(&mut snapshot).expect("committed diff applies to the before-snapshot");
        snapshot
    };
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
