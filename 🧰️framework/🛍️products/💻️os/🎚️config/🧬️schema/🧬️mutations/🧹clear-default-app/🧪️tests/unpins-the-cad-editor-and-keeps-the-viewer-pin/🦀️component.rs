//! 🧪️ `clear-default-app` fixture — `unpins-the-cad-editor-and-keeps-the-viewer-pin`.
//!
//! `clear-default-app` unpins one `(dialect, role)` coordinate so the `OpeningResolver` falls back
//! to its owner/router order. Its diff oracle guards on ABSENCE (nothing pinned for that pair ⇒
//! Warning `mutation.no-op` with the base record handed straight back) and otherwise filters the
//! matching entry out. The point this case pins: clearing the EDITOR pin must not disturb the
//! `viewer` pin for the very same dialect — the coordinate is the pair, not the dialect.
//!
//! 🎚️ Shape note: this config facet's `Mutation::Diff` IS `OpeningPreferences` itself — a
//! whole-record diff whose `apply` ignores `base` — so the committed `🔺️diff/🔣️component.json` is
//! the full post-op preferences record, not a sparse delta.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use super::super::super::OpeningPreferences;
use super::super::OpeningConfigMutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> OpeningPreferences {
    serde_json::from_str(BEFORE).expect("before opening preferences decode")
}
fn expected_after() -> OpeningPreferences {
    serde_json::from_str(AFTER).expect("after opening preferences decode")
}
fn mutation() -> OpeningConfigMutation {
    serde_json::from_str(MUTATION).expect("clear-default-app mutation decodes")
}

/// ▶️ Unpinning `(s.cad.cad@1/*, editor)` drops exactly that entry; the viewer pin for the same
/// dialect stays, because the oracle matches on the PAIR.
#[semio_framework_async_macros::async_test]
async fn unpins_the_editor_and_leaves_the_viewer_pin_standing() {
    let base = before();
    let outcome = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &base).await;
    let applied = protocol::MutationDiff::apply(outcome.diff().await, &base).await.expect("clear-default-app applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: the unpinned preferences differ from the committed after-snapshot");
    assert_eq!(applied.defaults.len(), 1, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: exactly one pin is dropped");
    assert!(applied.defaults.iter().all(|entry| entry.role != semio_framework::AppRole::Editor), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: no editor pin may survive the clear");
}

/// ↩️ `clear-default-app`'s inverse reads BASE's entry for the coordinate: a pin existed, so the
/// undo is a `set-default-app` carrying it back. Had nothing been pinned the inverse would be
/// empty — never a sentinel mutation.
#[semio_framework_async_macros::async_test]
async fn repinning_the_cleared_app_restores_before() {
    let base = before();
    let inverse = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::inverse(&mutation(), &base).await;
    assert_eq!(inverse.len(), 1, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: clearing an occupied coordinate proposes exactly one undo step");
    assert!(matches!(inverse[0], OpeningConfigMutation::SetDefaultApp(_)), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: the undo of a clear is a set, carrying the prior app back");
    let forward = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &base).await;
    let mut snapshot = protocol::MutationDiff::apply(forward.diff().await, &base).await.expect("forward clear-default-app applies");
    for step in &inverse {
        let undo = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(step, &snapshot).await;
        snapshot = protocol::MutationDiff::apply(undo.diff().await, &snapshot).await.expect("the set-default-app inverse step applies");
    }
    assert_eq!(snapshot, base, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: re-pinning the cleared editor did not restore the before-preferences");
}

/// 🔣️ Both committed preference records and the `clearDefaultApp` payload are canonical — the
/// payload carries only the coordinate, never the app it happens to be dropping.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: OpeningPreferences = serde_json::from_str(text).expect("opening preferences decode");
        let reencoded = serde_json::to_value(&decoded).expect("opening preferences encode");
        let original: serde_json::Value = serde_json::from_str(text).expect("opening preferences reparse");
        assert_eq!(reencoded, original, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: committed {label} preferences JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("clearDefaultApp payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("clearDefaultApp payload reparses");
    assert_eq!(reencoded, original, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: committed clearDefaultApp JSON is not canonical");
}

/// 🎯️ The coordinate really is pinned in the before-record, so the absence guard does not fire and
/// the declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: this fixture declares an applied outcome");
    let produced = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &before()).await;
    assert_eq!(produced.worst_level().await, None, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: clearing a real pin must not raise mutation.no-op");
    assert!(produced.messages().await.is_empty(), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: an accepted clear emits no diagnostics");
}

/// 🔺️ The committed diff is the whole post-op `OpeningPreferences` record — the surviving viewer
/// pin restated in full, and the dropped editor pin simply absent.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &before()).await;
    let produced = serde_json::to_value(outcome.diff().await).expect("produced clear-default-app diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `OpeningPreferences` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: OpeningPreferences = serde_json::from_str(DIFF).expect("committed clear-default-app diff decodes");
    assert_eq!(decoded.defaults.len(), 1, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: the whole-record diff must restate the one surviving pin");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: committed diff JSON is not canonical");
}

/// 🩹 The committed diff carries the before-record to the after-record — and because this facet's
/// `apply` ignores `base` outright, the diff IS the after-record.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: OpeningPreferences = serde_json::from_str(DIFF).expect("committed clear-default-app diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).await.expect("committed diff applies to the before-preferences");
    assert_eq!(produced, expected_after(), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: committed diff did not carry before to after");
}
