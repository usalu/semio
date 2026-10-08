//! 🧪️ `clear-default-app` fixture — `✏️unpins-the-cad-editor-and-keeps-the-viewer-pin`.
//!
//! `clear-default-app` unpins one `(dialect, role)` coordinate so the `OpeningResolver` falls back
//! to its owner/router order. Its diff oracle guards on ABSENCE (nothing pinned for that pair ⇒
//! Warning `mutation.no-op` with the base record handed straight back) and otherwise filters the
//! matching entry out. The point this case pins: clearing the EDITOR pin must not disturb the
//! `viewer` pin for the very same dialect — the coordinate is the pair, not the dialect.
//!
//! 🎚️ Shape note: this config facet's `Mutation::Diff` is the sparse `OpeningDiff` — one absolute pin row for the touched
//! `(dialect, role)` — so the committed `🔺️diff` names only that coordinate.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use super::super::super::{OpeningDiff, OpeningPreferences};
use super::super::OpeningConfigMutation;

const BEFORE: &str = include_str!("../../🧫️fixtures/✏️unpins-the-cad-editor-and-keeps-the-viewer-pin/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/✏️unpins-the-cad-editor-and-keeps-the-viewer-pin/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/✏️unpins-the-cad-editor-and-keeps-the-viewer-pin/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/✏️unpins-the-cad-editor-and-keeps-the-viewer-pin/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/✏️unpins-the-cad-editor-and-keeps-the-viewer-pin/🎯️outcome/🔣️.json");

fn before() -> OpeningPreferences {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before opening preferences decode")
}
fn expected_after() -> OpeningPreferences {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after opening preferences decode")
}
fn mutation() -> OpeningConfigMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("clear-default-app mutation decodes")
}
fn json_value<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}

/// ▶️ Unpinning `(s.cad.cad@1/*, editor)` drops exactly that entry; the viewer pin for the same
/// dialect stays, because the oracle matches on the PAIR.
#[test]
fn unpins_the_editor_and_leaves_the_viewer_pin_standing() {
    let base = before();
    let outcome = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("clear-default-app applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: the unpinned preferences differ from the committed after-snapshot");
    assert_eq!(applied.defaults.len(), 1, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: exactly one pin is dropped");
    assert!(applied.defaults.iter().all(|entry| entry.role != semio_framework::AppRole::Editor), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: no editor pin may survive the clear");
}

/// ↩️ `clear-default-app`'s inverse reads BASE's entry for the coordinate: a pin existed, so the
/// undo is a `set-default-app` carrying it back. Had nothing been pinned the inverse would be
/// empty — never a sentinel mutation.
#[test]
fn repinning_the_cleared_app_restores_before() {
    let base = before();
    let inverse = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: clearing an occupied coordinate proposes exactly one undo step");
    assert!(matches!(inverse[0], OpeningConfigMutation::SetDefaultApp(_)), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: the undo of a clear is a set, carrying the prior app back");
    let forward = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff(forward.diff(), &base).expect("forward clear-default-app applies");
    for step in &inverse {
        let undo = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(undo.diff(), &snapshot).expect("the set-default-app inverse step applies");
    }
    assert_eq!(snapshot, base, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: re-pinning the cleared editor did not restore the before-preferences");
}

/// 🔣️ Both committed preference records and the `clearDefaultApp` payload are canonical — the
/// payload carries only the coordinate, never the app it happens to be dropping.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: OpeningPreferences = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("opening preferences decode");
        let reencoded = json_value(&decoded);
        let original: serde_json::Value = serde_json::from_str(text).expect("opening preferences reparse");
        assert_eq!(reencoded, original, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: committed {label} preferences JSON is not canonical");
    }
    let reencoded = json_value(&mutation());
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("clearDefaultApp payload reparses");
    assert_eq!(reencoded, original, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: committed clearDefaultApp JSON is not canonical");
}

/// 🎯️ The coordinate really is pinned in the before-record, so the absence guard does not fire and
/// the declared `applied` outcome must be message-free.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: this fixture declares an applied outcome");
    let produced = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: clearing a real pin must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: an accepted clear emits no diagnostics");
}

/// 🔺️ The produced sparse diff is the committed `🔺️diff`.
#[test]
fn produces_committed_diff() {
    let outcome = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &before());
    let produced = json_value(outcome.diff());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to the facet's sparse diff type and re-encodes unchanged.
#[test]
fn committed_diff_is_canonical() {
    let decoded: OpeningDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed clear-default-app diff decodes");
    assert_eq!(decoded.pins.len(), 1, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: the sparse diff names exactly the touched coordinate");
    let reencoded = json_value(&decoded);
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: committed diff JSON is not canonical");
}

/// 🩹 The committed diff carries the before-record to the after-record through the central applier.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: OpeningDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed clear-default-app diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-preferences");
    assert_eq!(produced, expected_after(), "clear-default-app/unpins-the-cad-editor-and-keeps-the-viewer-pin: committed diff did not carry before to after");
}

/// ➕️ The concrete inverse's diffs sum to the negative of the forward diff (L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
