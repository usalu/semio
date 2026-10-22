//! 🧪️ `set-terminology` fixture — `🟰️keeps-terminology`.
//!
//! Setting the terminology to the value the preferences already hold is the leaf's `mutation.no-op` guard: a `no-op` outcome whose diff is empty.
//!
//! 🎚️ `UiPreferencesDiff` is a sparse diff: the committed `🔺️diff` names only the preference this leaf touches. Source of truth is the committed JSON quintet in
//! `../../🧫️fixtures/🟰️keeps-terminology/`.

use semio_framework_os_kernel::ColdMutation;
use semio_framework_os_kernel::ColdMutationDiff;
use semio_framework_os_kernel::ColdDiffAlgebra;
use crate::opening_config::mutations::UiPreferencesConfigMutation;
use crate::opening_config::{UiPreferences, UiPreferencesDiff};
use semio_framework_test_fixtures::fixture_path;

static BEFORE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/📸️snapshot/⬅️before/🔣️.json")).expect("fixture"));
static AFTER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/📸️snapshot/➡️after/🔣️.json")).expect("fixture"));
static MUTATION: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/🦠️mutation/🔣️.json")).expect("fixture"));
static DIFF: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/🔺️diff/🔣️.json")).expect("fixture"));
static OUTCOME: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/🎯️outcome/🔣️.json")).expect("fixture"));

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

/// ▶️ Re-setting the terminology to its current value leaves the preferences exactly as they were.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = <UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff_cold(outcome.diff(), &base).expect("set-terminology applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "set-terminology/keeps-terminology: the applied preferences differ from the committed after-snapshot");
}

/// 🎯️ The declared outcome is `no-op` with one `warning`-level `mutation.no-op`: the terminology already holds the requested value.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME.as_str()).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("no-op"), "set-terminology/keeps-terminology: this fixture declares a no-op outcome");
    let produced = <UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), Some(semio_framework_diagnostic::Severity::Warning), "set-terminology/keeps-terminology: an unchanged preference is a warned no-op, never a refusal");
    assert_eq!(produced.messages().iter().map(|message| message.code.0.clone()).collect::<Vec<_>>(), vec!["mutation.no-op".to_string()], "set-terminology/keeps-terminology: the only diagnostic is mutation.no-op");
    assert_eq!(protocol::apply_diff_cold(produced.diff(), &before()).expect("no-op applies"), before(), "set-terminology/keeps-terminology: a no-op leaves the preferences untouched");
}

/// 🔺️ The produced sparse diff is the committed `🔺️diff`.
#[test]
fn produces_committed_diff() {
    let outcome = <UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::diff(&mutation(), &before());
    let committed: serde_json::Value = serde_json::from_str(DIFF.as_str()).expect("committed diff decodes");
    assert_eq!(json_value(outcome.diff()), committed, "set-terminology/keeps-terminology: produced diff differs from the committed 🔺️diff");
    let decoded: UiPreferencesDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes as UiPreferencesDiff");
    assert_eq!(protocol::apply_diff_cold(&decoded, &before()).expect("committed diff applies"), expected_after(), "set-terminology/keeps-terminology: the committed diff does not carry before to after");
}

/// ↩️ The inverse restores the committed before-preferences.
#[test]
fn inverse_restores_before() {
    let base = before();
    let forward = <UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff_cold(forward.diff(), &base).expect("forward set-terminology applies");
    for step in <UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture") {
        let undo = <UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::diff(&step, &snapshot);
        snapshot = protocol::apply_diff_cold(undo.diff(), &snapshot).expect("set-terminology inverse step applies");
    }
    assert_eq!(snapshot, base, "set-terminology/keeps-terminology: the inverse did not restore the before-preferences");
}

/// 🔣️ The committed preference records and payload are canonical: decode → encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE.as_str()), ("after", AFTER.as_str())] {
        let decoded: UiPreferences = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("preferences decode");
        let original: serde_json::Value = serde_json::from_str(text).expect("preferences reparse");
        assert_eq!(json_value(&decoded), original, "set-terminology/keeps-terminology: committed {label} JSON is not canonical");
    }
    let original: serde_json::Value = serde_json::from_str(MUTATION.as_str()).expect("payload reparses");
    assert_eq!(json_value(&mutation()), original, "set-terminology/keeps-terminology: committed payload JSON is not canonical");
}

/// ➕️ The concrete inverse's diffs sum to the negative of the forward diff (L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
