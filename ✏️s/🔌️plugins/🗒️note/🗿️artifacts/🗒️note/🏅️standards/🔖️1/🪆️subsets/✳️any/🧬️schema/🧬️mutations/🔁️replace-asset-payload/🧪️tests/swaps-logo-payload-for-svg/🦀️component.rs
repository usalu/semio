//! 🧪️ `replace-asset-payload` fixture — `swaps-logo-payload-for-svg`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> NoteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> NoteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> NoteMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `replace-asset-payload` is a WHOLE-VALUE swap: the stored asset is replaced, not merged.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("replace-asset-payload applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-asset-payload/swaps-logo-payload-for-svg: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse re-issues `replace-asset-payload` carrying the base's own prior asset value.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("replace-asset-payload applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("replace-asset-payload inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-asset-payload/swaps-logo-payload-for-svg: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-asset-payload/swaps-logo-payload-for-svg: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-asset-payload/swaps-logo-payload-for-svg: committed mutation JSON is not canonical");
}

/// 🎯️ `asset-logo` exists and its payload genuinely differs, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "replace-asset-payload/swaps-logo-payload-for-svg: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "replace-asset-payload/swaps-logo-payload-for-svg: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("replace-asset-payload/swaps-logo-payload-for-svg: declared applied but the diff would not apply");
}

/// 🔺️ One `assets.entries` UPSERT carrying the WHOLE new asset value — no per-field asset patch shape exists, which is exactly why the old dimensions vanish.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-asset-payload/swaps-logo-payload-for-svg: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `replace-asset-payload` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-asset-payload/swaps-logo-payload-for-svg: committed diff JSON is not canonical");
}

/// 🩹 The committed whole-value asset upsert carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-asset-payload/swaps-logo-payload-for-svg: committed diff did not carry before to after");
}

/// 🔁 The whole asset value is swapped — the old PNG's 64x64 dimensions are dropped, not merged forward.
#[semio_framework_async_macros::async_test]
async fn whole_asset_value_is_swapped_dropping_the_old_dimensions() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("replace-asset-payload applies");
    assert_eq!(applied.assets.len(), base.assets.len(), "replace-asset-payload/swaps-logo-payload-for-svg: replacing must not change the key count");
    let prior = base.assets.get("asset-logo").expect("the base asset exists");
    assert_eq!((prior.mime.as_str(), prior.width, prior.height), ("image/png", Some(64.0), Some(64.0)), "the base asset must start as a sized PNG");
    let next = applied.assets.get("asset-logo").expect("the replaced asset exists");
    assert_eq!(next.mime, "image/svg+xml", "the replaced asset must carry the new mime");
    assert_eq!(next.data, "PHN2Zy8+", "the replaced asset must carry the new payload");
    assert_eq!((next.width, next.height), (None, None), "a WHOLE-VALUE swap drops the prior dimensions instead of merging them forward");
}
