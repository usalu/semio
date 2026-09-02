//! 🧪️ `change-asset-version` fixture — `raises-the-asset-version-to-2-point-1`.
//!
//! `validate()` refuses a blank version (`gltf.mutation.invalid-asset-version`) and a version equal to
//! the current one (`gltf.mutation.no-observable-change`). `derive()` writes ONLY
//! `GltfAssetDiff { version: Some(..) }` — `version` is the one non-`Option<Option<_>>` field of that
//! struct, so it round-trips without any caveat.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::change_asset_version::GltfChangeAssetVersionPayload;
use crate::artifacts::gltf::schema::mutations::change_asset_version::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "change-asset-version/raises-the-asset-version-to-2-point-1";
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
fn payload() -> GltfChangeAssetVersionPayload {
    serde_json::from_str(MUTATION).expect("change-asset-version payload decodes")
}

/// ▶️ `change-asset-version` writes `asset.version` and no other asset field.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("change-asset-version applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.asset.version, "2.1", "{CASE}: the asset version must become 2.1");
    assert!(snapshot.document.asset.generator.is_none() && snapshot.document.asset.min_version.is_none(), "{CASE}: a version change must not materialise the descriptive metadata fields");
}

/// ↩️ The inverse writes the prior version string back.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("change-asset-version inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(restored.document.asset.version, "2.0", "{CASE}: the inverse must restore version 2.0");
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
            let snapshot = attempt.expect("change-asset-version declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("change-asset-version declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta is the single `version` field of `asset` — every other GltfDiff slot stays absent.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("change-asset-version derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(produced.scenes.is_none() && produced.nodes.is_none(), "{CASE}: change-asset-version may only ever write GltfDiff::asset");
    assert!(produced.asset.as_ref().is_some_and(|asset| asset.generator.is_none() && asset.copyright.is_none()), "{CASE}: the asset delta must leave the descriptive-metadata slots unset");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert_eq!(decoded.asset.as_ref().and_then(|asset| asset.version.clone()), Some("2.1".to_string()), "{CASE}: the committed asset delta must carry the new version");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
