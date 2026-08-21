//! 🧪️ `change-material-alpha-mode` fixture — `switches-the-default-material-from-opaque-to-mask`.
//!
//! One of the three leaves wired into `GLTF_MUTATION_LEAF_DESCRIPTORS`, so its shape is the descriptor
//! contract's: `mutation::apply` takes `&mut GltfSnapshot`, the diff is a pre-state WITNESS
//! (`expected_alpha_mode`) whose `apply` refuses a mismatch with `gltf.mutation.stale-diff`, and the
//! inverse is built by `reconstruct` rather than `derive`. `validate()` refuses re-writing the mode the
//! material already carries (`gltf.mutation.no-observable-change`), so `before` must be OPAQUE — which is
//! also why the committed `before` material serializes to `{}`: every one of its fields is a spec default.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::diff::GltfChangeMaterialAlphaModeDiff;
use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::mutation::GltfChangeMaterialAlphaModePayload;
use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "change-material-alpha-mode/switches-the-default-material-from-opaque-to-mask";
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
fn payload() -> GltfChangeMaterialAlphaModePayload {
    serde_json::from_str(MUTATION).expect("change-material-alpha-mode payload decodes")
}

/// ▶️ `change-material-alpha-mode` writes `alphaMode` and no other material field.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    mutation::apply(&mut snapshot, &payload()).expect("change-material-alpha-mode applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.materials[0].alpha_mode, crate::artifacts::gltf::schema::snapshot::GltfAlphaMode::Mask, "{CASE}: the material must switch to MASK");
    assert_eq!(snapshot.document.materials[0].alpha_cutoff, 0.5, "{CASE}: alphaCutoff is a SEPARATE field and must keep its spec default of 0.5 — switching to MASK does not set it");
    assert!(!snapshot.document.materials[0].double_sided, "{CASE}: doubleSided belongs to a different leaf and must stay false");
}

/// ↩️ `reconstruct` builds an inverse whose expected pre-state is the FORWARD result, so it only applies once.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::reconstruct(&payload(), &base).expect("change-material-alpha-mode inverse reconstructs from the exact base");
    let mut restored = base.clone();
    mutation::apply(&mut restored, &payload()).expect("forward applies");
    inverse.apply(&mut restored).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(inverse.expected_alpha_mode, crate::artifacts::gltf::schema::snapshot::GltfAlphaMode::Mask, "{CASE}: the inverse must expect to find MASK — it is validated against the post-state, not the pre-state");
    assert_eq!(inverse.alpha_mode, crate::artifacts::gltf::schema::snapshot::GltfAlphaMode::Opaque, "{CASE}: the inverse must write OPAQUE back");
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
            attempt.expect("change-material-alpha-mode declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("change-material-alpha-mode declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
            assert_eq!(snapshot, before(), "{CASE}: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta carries the pre-state witness that makes a stale re-application detectable.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("change-material-alpha-mode derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(produced.expected_alpha_mode, crate::artifacts::gltf::schema::snapshot::GltfAlphaMode::Opaque, "{CASE}: the diff must witness the OPAQUE pre-state");
    assert_eq!(produced.touched_paths, vec!["document/materials/0/alphaMode".to_string()], "{CASE}: the descriptor family interpolates the material index into a CONCRETE path");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GltfChangeMaterialAlphaModeDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert_eq!(decoded.material, 0, "{CASE}: the committed diff must echo the addressed material");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GltfChangeMaterialAlphaModeDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = {
        let mut snapshot = before();
        decoded.apply(&mut snapshot).expect("committed diff applies to the before-snapshot");
        snapshot
    };
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
