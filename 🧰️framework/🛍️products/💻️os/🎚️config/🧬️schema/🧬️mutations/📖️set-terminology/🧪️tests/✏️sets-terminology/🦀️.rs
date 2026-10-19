//! 🧪️ `set-terminology` fixture — `✏️sets-terminology`.
//!
//! Setting the terminology on untouched preferences changes exactly that preference and nothing else.
//!
//! 🎚️ `UiPreferencesDiff` is a sparse diff: the committed `🔺️diff` names only the preference this leaf touches. Source of truth is the committed JSON quintet in
//! `../../🧫️fixtures/✏️sets-terminology/`.

use semio_framework_os_kernel::ColdMutation;
use semio_framework_os_kernel::ColdMutationDiff;
use semio_framework_os_kernel::ColdDiffAlgebra;
use crate::opening_config::mutations::UiPreferencesConfigMutation;
use crate::opening_config::{UiPreferences, UiPreferencesDiff};
use semio_framework_test_fixtures::fixture_path;

static BEFORE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/📸️snapshot/⬅️before/🔣️.json")).expect("fixture"));
static AFTER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/📸️snapshot/➡️after/🔣️.json")).expect("fixture"));
static MUTATION: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/🦠️mutation/🔣️.json")).expect("fixture"));
static DIFF: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/🔺️diff/🔣️.json")).expect("fixture"));
static OUTCOME: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/🎯️outcome/🔣️.json")).expect("fixture"));

fn before() -> UiPreferences {
    semio_framework_pack_json::from_json_str(BEFORE.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before preferences decode")
}
fn expected_after() -> UiPreferences {
    semio_framework_pack_json::from_json_str(AFTER.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after preferences decode")
}
fn mutation() -> UiPreferencesConfigMutation {
    semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("set-terminology mutation decodes")
}
fn json_value<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}

/// ▶️ Setting the terminology writes it into the record and leaves every other preference as it was.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff_cold(outcome.diff(), &base).expect("set-terminology applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "set-terminology/sets-terminology: the applied preferences differ from the committed after-snapshot");
}

/// 🎯️ The declared outcome is `applied` with no diagnostics: the terminology was not set before.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME.as_str()).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-terminology/sets-terminology: this fixture declares a applied outcome");
    let produced = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "set-terminology/sets-terminology: a changed preference emits no diagnostics");
}

/// 🔺️ The produced sparse diff is the committed `🔺️diff`.
#[test]
fn produces_committed_diff() {
    let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &before());
    let committed: serde_json::Value = serde_json::from_str(DIFF.as_str()).expect("committed diff decodes");
    assert_eq!(json_value(outcome.diff()), committed, "set-terminology/sets-terminology: produced diff differs from the committed 🔺️diff");
    let decoded: UiPreferencesDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes as UiPreferencesDiff");
    assert_eq!(protocol::apply_diff_cold(&decoded, &before()).expect("committed diff applies"), expected_after(), "set-terminology/sets-terminology: the committed diff does not carry before to after");
}

/// ↩️ The inverse restores the committed before-preferences.
#[test]
fn inverse_restores_before() {
    let base = before();
    let forward = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff_cold(forward.diff(), &base).expect("forward set-terminology applies");
    for step in <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture") {
        let undo = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&step, &snapshot);
        snapshot = protocol::apply_diff_cold(undo.diff(), &snapshot).expect("set-terminology inverse step applies");
    }
    assert_eq!(snapshot, base, "set-terminology/sets-terminology: the inverse did not restore the before-preferences");
}

/// 🔣️ The committed preference records and payload are canonical: decode → encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE.as_str()), ("after", AFTER.as_str())] {
        let decoded: UiPreferences = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("preferences decode");
        let original: serde_json::Value = serde_json::from_str(text).expect("preferences reparse");
        assert_eq!(json_value(&decoded), original, "set-terminology/sets-terminology: committed {label} JSON is not canonical");
    }
    let original: serde_json::Value = serde_json::from_str(MUTATION.as_str()).expect("payload reparses");
    assert_eq!(json_value(&mutation()), original, "set-terminology/sets-terminology: committed payload JSON is not canonical");
}

/// ➕️ The concrete inverse's diffs sum to the negative of the forward diff (L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
