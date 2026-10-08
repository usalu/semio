//! 🧪️ `set-named-layout` fixture — `🟰️keeps-named-layout`.
//!
//! Setting the saved layout to the value the preferences already hold is the leaf's `mutation.no-op` guard: a `no-op` outcome whose diff is empty.
//!
//! 🎚️ `UiPreferencesDiff` is a sparse diff: the committed `🔺️diff` names only the preference this leaf touches. Source of truth is the committed JSON quintet in
//! `../../🧫️fixtures/🟰️keeps-named-layout/`.

use crate::opening_config::mutations::UiPreferencesConfigMutation;
use crate::opening_config::{UiPreferences, UiPreferencesDiff};

const BEFORE: &str = include_str!("../../🧫️fixtures/🟰️keeps-named-layout/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/🟰️keeps-named-layout/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/🟰️keeps-named-layout/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/🟰️keeps-named-layout/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/🟰️keeps-named-layout/🎯️outcome/🔣️.json");

fn before() -> UiPreferences {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before preferences decode")
}
fn expected_after() -> UiPreferences {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after preferences decode")
}
fn mutation() -> UiPreferencesConfigMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("set-named-layout mutation decodes")
}
fn json_value<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}

/// ▶️ Re-setting the saved layout to its current value leaves the preferences exactly as they were.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("set-named-layout applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "set-named-layout/keeps-named-layout: the applied preferences differ from the committed after-snapshot");
}

/// 🎯️ The declared outcome is `no-op` with one `warning`-level `mutation.no-op`: the saved layout already holds the requested value.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("no-op"), "set-named-layout/keeps-named-layout: this fixture declares a no-op outcome");
    let produced = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), Some(semio_framework_diagnostic::Severity::Warning), "set-named-layout/keeps-named-layout: an unchanged preference is a warned no-op, never a refusal");
    assert_eq!(produced.messages().iter().map(|message| message.code.0.clone()).collect::<Vec<_>>(), vec!["mutation.no-op".to_string()], "set-named-layout/keeps-named-layout: the only diagnostic is mutation.no-op");
    assert_eq!(protocol::apply_diff(produced.diff(), &before()).expect("no-op applies"), before(), "set-named-layout/keeps-named-layout: a no-op leaves the preferences untouched");
}

/// 🔺️ The produced sparse diff is the committed `🔺️diff`.
#[test]
fn produces_committed_diff() {
    let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &before());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(json_value(outcome.diff()), committed, "set-named-layout/keeps-named-layout: produced diff differs from the committed 🔺️diff");
    let decoded: UiPreferencesDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes as UiPreferencesDiff");
    assert_eq!(protocol::apply_diff(&decoded, &before()).expect("committed diff applies"), expected_after(), "set-named-layout/keeps-named-layout: the committed diff does not carry before to after");
}

/// ↩️ The inverse restores the committed before-preferences.
#[test]
fn inverse_restores_before() {
    let base = before();
    let forward = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff(forward.diff(), &base).expect("forward set-named-layout applies");
    for step in <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture") {
        let undo = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&step, &snapshot);
        snapshot = protocol::apply_diff(undo.diff(), &snapshot).expect("set-named-layout inverse step applies");
    }
    assert_eq!(snapshot, base, "set-named-layout/keeps-named-layout: the inverse did not restore the before-preferences");
}

/// 🔣️ The committed preference records and payload are canonical: decode → encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: UiPreferences = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("preferences decode");
        let original: serde_json::Value = serde_json::from_str(text).expect("preferences reparse");
        assert_eq!(json_value(&decoded), original, "set-named-layout/keeps-named-layout: committed {label} JSON is not canonical");
    }
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("payload reparses");
    assert_eq!(json_value(&mutation()), original, "set-named-layout/keeps-named-layout: committed payload JSON is not canonical");
}
