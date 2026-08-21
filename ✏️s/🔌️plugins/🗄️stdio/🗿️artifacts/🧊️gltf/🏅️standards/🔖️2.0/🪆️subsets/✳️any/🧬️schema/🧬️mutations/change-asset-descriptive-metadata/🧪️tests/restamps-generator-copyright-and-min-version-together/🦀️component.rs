//! 🧪️ `change-asset-descriptive-metadata` fixture — `restamps-generator-copyright-and-min-version-together`.
//!
//! This leaf writes all THREE descriptive fields in one atomic step — `derive()` always sets
//! `generator`, `copyright` AND `min_version`, even when only one of them actually changed, so the
//! committed diff carries three entries. All three are `Option<Option<String>>`; this fixture supplies a
//! real string for each so every slot survives the JSON round trip (any `null` would collapse to an
//! absent field). `validate()` refuses only the all-three-unchanged no-op.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own oracle. The derived `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/
//! `.patch.semio` encodings come from `fixtures generate`, not from here.

use crate::artifacts::gltf::schema::mutations::change_asset_descriptive_metadata::mutation::GltfChangeAssetDescriptiveMetadataPayload;
use crate::artifacts::gltf::schema::mutations::change_asset_descriptive_metadata::{diff, inverse, mutation};
use crate::artifacts::gltf::GltfSnapshot;

const CASE: &str = "change-asset-descriptive-metadata/restamps-generator-copyright-and-min-version-together";
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
fn payload() -> GltfChangeAssetDescriptiveMetadataPayload {
    serde_json::from_str(MUTATION).expect("change-asset-descriptive-metadata payload decodes")
}

/// ▶️ `change-asset-descriptive-metadata` restamps all three fields and leaves `version` alone.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = mutation::apply(&payload(), &before()).expect("change-asset-descriptive-metadata applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{CASE}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.document.asset.generator.as_deref(), Some("semio 1.0"), "{CASE}: the generator must be restamped");
    assert_eq!(snapshot.document.asset.min_version.as_deref(), Some("2.0"), "{CASE}: minVersion is rewritten with the SAME value — this leaf writes all three fields unconditionally");
    assert_eq!(snapshot.document.asset.version, "2.0", "{CASE}: the mandatory asset.version is a different leaf's territory and must not move");
}

/// ↩️ The inverse restamps all three fields back to their prior values in one step.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse::derive(&payload(), &base).expect("change-asset-descriptive-metadata inverse derives from the exact base");
    let after = mutation::apply(&payload(), &base).expect("forward applies");
    let restored = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&inverse, &after).expect("inverse applies to the forward result");
    assert_eq!(restored, base, "{CASE}: inverse did not restore the before-snapshot");
    assert_eq!(restored.document.asset.generator.as_deref(), Some("old-exporter"), "{CASE}: the inverse must restore the original generator");
    assert_eq!(restored.document.asset.copyright.as_deref(), Some("(c) 2025"), "{CASE}: the inverse must restore the original copyright");
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
            let snapshot = attempt.expect("change-asset-descriptive-metadata declared applied");
            assert_ne!(snapshot, before(), "{CASE}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("rejected outcome carries a code");
            assert_eq!(attempt.expect_err("change-asset-descriptive-metadata declared rejected").code, code, "{CASE}: rejection code differs from the committed outcome");
        }
        other => panic!("{CASE}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ All three slots appear even though `minVersion` did not change — that is what `derive()` builds.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = diff::derive(&payload(), &before()).expect("change-asset-descriptive-metadata derives its diff");
    let encoded = serde_json::to_value(&produced).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "{CASE}: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(produced.asset.as_ref().is_some_and(|asset| asset.min_version.is_some()), "{CASE}: minVersion must be present in the delta even though its value is unchanged");
    assert!(produced.asset.as_ref().is_some_and(|asset| asset.version.is_none()), "{CASE}: the delta must leave asset.version unset");
}

/// 🔣️ The committed diff is itself canonical and decodes to this leaf's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{CASE}: committed diff JSON is not canonical");
    assert_eq!(decoded.asset.as_ref().and_then(|asset| asset.copyright.clone()), Some(Some("(c) 2026 semio".to_string())), "{CASE}: a non-null Option<Option<String>> decodes back to Some(Some(..)) exactly");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::gltf::schema::diff::GltfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{CASE}: committed diff did not carry before to after");
}
