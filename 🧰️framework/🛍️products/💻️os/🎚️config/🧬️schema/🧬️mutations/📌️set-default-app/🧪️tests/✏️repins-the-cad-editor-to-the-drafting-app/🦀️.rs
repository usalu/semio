//! 🧪️ `set-default-app` fixture — `✏️repins-the-cad-editor-to-the-drafting-app`.
//!
//! `set-default-app` pins one `(dialect, role) -> app` default. Its diff guards the exact triple
//! (`dialect == && role == && app ==` ⇒ Warning `mutation.no-op`) and otherwise names one absolute pin row for the
//! `(dialect, role)`; the central applier keeps `defaults` in canonical (dialect, role) order. The sibling `viewer` pin for
//! the very same dialect must survive untouched: the coordinate is the PAIR, not the dialect alone.
//!
//! 🎚️ Shape note: this config facet's `Mutation::Diff` is the sparse `OpeningDiff` — one absolute pin row for the touched
//! `(dialect, role)` — so the committed `🔺️diff` names only that coordinate.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use super::super::super::{OpeningDiff, OpeningPreferences};
use super::super::OpeningConfigMutation;

const BEFORE: &str = include_str!("../../🧫️fixtures/✏️repins-the-cad-editor-to-the-drafting-app/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/✏️repins-the-cad-editor-to-the-drafting-app/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/✏️repins-the-cad-editor-to-the-drafting-app/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/✏️repins-the-cad-editor-to-the-drafting-app/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/✏️repins-the-cad-editor-to-the-drafting-app/🎯️outcome/🔣️.json");

fn before() -> OpeningPreferences {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before opening preferences decode")
}
fn expected_after() -> OpeningPreferences {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after opening preferences decode")
}
fn mutation() -> OpeningConfigMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("set-default-app mutation decodes")
}
fn json_value<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}

/// ▶️ Re-pinning the cad editor to the drafting plugin replaces that one entry and leaves the
/// viewer pin for the same dialect in place.
#[test]
fn repins_the_editor_and_keeps_the_viewer_pin() {
    let base = before();
    let outcome = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("set-default-app applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "set-default-app/repins-the-cad-editor-to-the-drafting-app: the re-pinned preferences differ from the committed after-snapshot");
    assert_eq!(applied.defaults.len(), base.defaults.len(), "set-default-app/repins-the-cad-editor-to-the-drafting-app: re-pinning an occupied coordinate must replace, never accumulate a second entry");
    assert_eq!(applied.defaults.last().map(|entry| entry.app.plugin_id.as_str()), Some("drafting"), "set-default-app/repins-the-cad-editor-to-the-drafting-app: the pin stays at its canonical (dialect, role) position, the editor after the viewer");
}

/// ↩️ `set-default-app`'s inverse reads BASE's entry for the same `(dialect, role)`: a prior pin
/// exists here, so the undo is another `set-default-app` carrying the cad plugin back — never the
/// `clear-default-app` the oracle would emit for a previously unpinned coordinate.
#[test]
fn repinning_the_prior_app_restores_before() {
    let base = before();
    let inverse = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "set-default-app/repins-the-cad-editor-to-the-drafting-app: exactly one undo step");
    assert!(matches!(inverse[0], OpeningConfigMutation::SetDefaultApp(_)), "set-default-app/repins-the-cad-editor-to-the-drafting-app: an occupied coordinate must undo via set-default-app, not clear-default-app");
    let forward = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff(forward.diff(), &base).expect("forward set-default-app applies");
    for step in &inverse {
        let undo = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(undo.diff(), &snapshot).expect("the set-default-app inverse step applies");
    }
    assert_eq!(snapshot, base, "set-default-app/repins-the-cad-editor-to-the-drafting-app: re-pinning the cad editor did not restore the before-preferences");
}

/// 🔣️ Both committed preference records and the `setDefaultApp` payload are canonical — `role`
/// rides as the camelCase wire spelling `"editor"`, and the payload is internally tagged.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: OpeningPreferences = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("opening preferences decode");
        let reencoded = json_value(&decoded);
        let original: serde_json::Value = serde_json::from_str(text).expect("opening preferences reparse");
        assert_eq!(reencoded, original, "set-default-app/repins-the-cad-editor-to-the-drafting-app: committed {label} preferences JSON is not canonical");
    }
    let reencoded = json_value(&mutation());
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("setDefaultApp payload reparses");
    assert_eq!(reencoded, original, "set-default-app/repins-the-cad-editor-to-the-drafting-app: committed setDefaultApp JSON is not canonical");
}

/// 🎯️ The drafting app is not already pinned for `(cad, editor)`, so the exact-triple no-op guard
/// does not fire and the declared `applied` outcome must be message-free.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-default-app/repins-the-cad-editor-to-the-drafting-app: this fixture declares an applied outcome");
    let produced = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "set-default-app/repins-the-cad-editor-to-the-drafting-app: pinning a different app must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "set-default-app/repins-the-cad-editor-to-the-drafting-app: an accepted pin emits no diagnostics");
}

/// 🔺️ The produced sparse diff is the committed `🔺️diff`.
#[test]
fn produces_committed_diff() {
    let outcome = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &before());
    let produced = json_value(outcome.diff());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-default-app/repins-the-cad-editor-to-the-drafting-app: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to the facet's sparse diff type and re-encodes unchanged.
#[test]
fn committed_diff_is_canonical() {
    let decoded: OpeningDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed set-default-app diff decodes");
    assert_eq!(decoded.pins.len(), 1, "set-default-app/repins-the-cad-editor-to-the-drafting-app: the sparse diff names exactly the touched coordinate");
    let reencoded = json_value(&decoded);
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-default-app/repins-the-cad-editor-to-the-drafting-app: committed diff JSON is not canonical");
}

/// 🩹 The committed diff carries the before-record to the after-record through the central applier.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: OpeningDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed set-default-app diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-preferences");
    assert_eq!(produced, expected_after(), "set-default-app/repins-the-cad-editor-to-the-drafting-app: committed diff did not carry before to after");
}
