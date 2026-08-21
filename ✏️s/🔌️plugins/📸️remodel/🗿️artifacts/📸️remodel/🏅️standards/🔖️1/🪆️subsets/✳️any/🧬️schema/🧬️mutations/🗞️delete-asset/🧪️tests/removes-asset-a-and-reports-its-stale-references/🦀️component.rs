//! 🧪️ `delete-asset` fixture — `removes-asset-a-and-reports-its-stale-references`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodel::mutations::{apply_remodel_mutation, inverse_remodel_mutation, RemodelMutation};
use crate::artifacts::remodel::{RemodelDiff, RemodelSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> RemodelSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RemodelSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RemodelMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<RemodelDiff> {
    <RemodelMutation as protocol::Mutation<RemodelSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ The handle is dropped from the map and NOTHING else is rewritten — the three stream frames
/// and the mesh texture that referenced `asset-a` are deliberately left dangling.
#[semio_framework_async_macros::async_test]
async fn drops_the_handle_and_leaves_every_reference_dangling() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("delete-asset applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-asset/removes-asset-a-and-reports-its-stale-references: applied state differs from committed after-snapshot");
    assert!(applied.assets.is_empty(), "the only asset key is removed");
    assert_eq!(applied.streams, before().streams, "the frames that referenced asset-a keep pointing at the removed key");
    assert_eq!(applied.results.mesh.texture_asset_id.as_deref(), Some("asset-a"), "the mesh texture reference is reported, never rewritten");
    assert_eq!(applied.results.geo, before().results.geo, "the geo products, which reference a different key, are untouched");
}

/// ↩️ The inverse reads the deleted bytes back through the working-scene cache. In a fresh test
/// process that cache is cold, so the inverse is honestly EMPTY rather than fabricating bytes.
#[semio_framework_async_macros::async_test]
async fn inverse_is_empty_against_a_cold_working_scene_cache() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(inverse.is_empty(), "delete-asset cannot invert a handle whose real ImageAsset bytes were never minted in this process, got {inverse:?}");
    let applied = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    assert_ne!(applied, base, "the forward delete is real even though it is not invertible from a cold cache");
}

/// 🎯️ Declared `applied` with one `mutation.cascade` note counting the four stale references —
/// two frames on `stream-a`, one on `stream-b`, and the mesh texture.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_counts_four_stale_references() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "delete-asset/removes-asset-a-and-reports-its-stale-references declares an applied outcome");
    let produced = produced();
    let codes: Vec<&str> = produced.messages().iter().map(|message| message.code.0.as_str()).collect();
    assert_eq!(codes, ["mutation.cascade"], "an in-use asset reports exactly one cascade note");
    assert_eq!(produced.messages()[0].level, protocol::Severity::Info, "the stale-reference report is informational, not a rejection");
    assert!(produced.messages()[0].message.contains("4 stale reference(s)"), "three frame references plus the mesh texture make four, got {:?}", produced.messages()[0].message);
    assert!(produced.diff().streams.is_none() && produced.diff().results.is_none(), "delete-asset writes assets alone — the cascade is reported, never applied");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-asset/removes-asset-a-and-reports-its-stale-references: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-asset/removes-asset-a-and-reports-its-stale-references: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `delete-asset` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "delete-asset/removes-asset-a-and-reports-its-stale-references: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let assets = committed_diff.assets.as_ref().expect("delete-asset's delta is the whole assets map");
    assert!(assets.is_empty(), "the committed delta is the EMPTIED map — a whole-map replace, not a per-key removal list");
    assert!(committed_diff.streams.is_none() && committed_diff.results.is_none(), "the four stale references stay out of the delta: reported, never rewritten");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-asset/removes-asset-a-and-reports-its-stale-references: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `delete-asset`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "delete-asset/removes-asset-a-and-reports-its-stale-references: committed diff did not carry before to after");
}
