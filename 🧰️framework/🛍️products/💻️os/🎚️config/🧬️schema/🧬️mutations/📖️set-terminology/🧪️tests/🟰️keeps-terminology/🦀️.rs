//! 🧪️ `set-terminology` fixture — `🟰️keeps-terminology`.
//!
//! Setting the terminology to the value the preferences already hold is the leaf's `mutation.no-op` guard: a `no-op` outcome whose whole-record diff restates the unchanged record.
//!
//! 🎚️ `UiPreferencesDiff` is a whole-record diff whose `apply` ignores `base`, so the committed
//! `🔺️diff` is the full post-op preferences record. Source of truth is the committed JSON quintet in
//! `../../🧫️fixtures/🟰️keeps-terminology/`.

use crate::opening_config::mutations::UiPreferencesConfigMutation;
use crate::opening_config::{UiPreferences, UiPreferencesDiff};

const BEFORE: &str = include_str!("../../🧫️fixtures/🟰️keeps-terminology/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/🟰️keeps-terminology/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/🟰️keeps-terminology/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/🟰️keeps-terminology/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/🟰️keeps-terminology/🎯️outcome/🔣️.json");

fn before() -> UiPreferences {
    dsl::os_pack::json::from_json_str(BEFORE).expect("before preferences decode")
}
fn expected_after() -> UiPreferences {
    dsl::os_pack::json::from_json_str(AFTER).expect("after preferences decode")
}
fn mutation() -> UiPreferencesConfigMutation {
    dsl::os_pack::json::from_json_str(MUTATION).expect("set-terminology mutation decodes")
}
fn json_value<T: dsl::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&dsl::os_pack::json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}

/// ▶️ Re-setting the terminology to its current value leaves the preferences exactly as they were.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &base);
    let applied = protocol::MutationDiff::apply(outcome.diff(), &base).expect("set-terminology applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "set-terminology/keeps-terminology: the applied preferences differ from the committed after-snapshot");
}

/// 🎯️ The declared outcome is `no-op` with one `warning`-level `mutation.no-op`: the terminology already holds the requested value.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("no-op"), "set-terminology/keeps-terminology: this fixture declares a no-op outcome");
    let produced = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), Some(protocol::Severity::Warning), "set-terminology/keeps-terminology: an unchanged preference is a warned no-op, never a refusal");
    assert_eq!(produced.messages().iter().map(|message| message.code.0.clone()).collect::<Vec<_>>(), vec!["mutation.no-op".to_string()], "set-terminology/keeps-terminology: the only diagnostic is mutation.no-op");
    assert_eq!(protocol::MutationDiff::apply(produced.diff(), &before()).expect("no-op applies"), before(), "set-terminology/keeps-terminology: a no-op leaves the preferences untouched");
}

/// 🔺️ The produced whole-record diff is the committed `🔺️diff`.
#[test]
fn produces_committed_diff() {
    let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &before());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(json_value(outcome.diff()), committed, "set-terminology/keeps-terminology: produced diff differs from the committed 🔺️diff");
    let decoded: UiPreferencesDiff = dsl::os_pack::json::from_json_str(DIFF).expect("committed diff decodes as UiPreferencesDiff");
    assert_eq!(protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies"), expected_after(), "set-terminology/keeps-terminology: the committed diff does not carry before to after");
}

/// ↩️ The inverse restores the committed before-preferences.
#[test]
fn inverse_restores_before() {
    let base = before();
    let forward = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward set-terminology applies");
    for step in <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::inverse(&mutation(), &base) {
        let undo = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("set-terminology inverse step applies");
    }
    assert_eq!(snapshot, base, "set-terminology/keeps-terminology: the inverse did not restore the before-preferences");
}

/// 🔣️ The committed preference records and payload are canonical: decode → encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: UiPreferences = dsl::os_pack::json::from_json_str(text).expect("preferences decode");
        let original: serde_json::Value = serde_json::from_str(text).expect("preferences reparse");
        assert_eq!(json_value(&decoded), original, "set-terminology/keeps-terminology: committed {label} JSON is not canonical");
    }
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("payload reparses");
    assert_eq!(json_value(&mutation()), original, "set-terminology/keeps-terminology: committed payload JSON is not canonical");
}
