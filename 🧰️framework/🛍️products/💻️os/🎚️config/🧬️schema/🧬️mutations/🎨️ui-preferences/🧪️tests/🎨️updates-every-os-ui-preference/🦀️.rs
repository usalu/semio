//! 🧪️ Language-neutral OS UI-preference mutation fixture.

use semio_framework_os_kernel::ColdMutation;
use semio_framework_os_kernel::ColdMutationDiff;
use semio_framework_os_kernel::ColdDiffAlgebra;
use super::super::super::UiPreferences;
use super::super::UiPreferencesConfigMutation;
use semio_framework_test_fixtures::fixture_path;

static BEFORE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/🎨️ui-preferences/🎨️updates-every-os-ui-preference/📸️snapshot/⬅️before/🔣️.json")).expect("fixture"));
static AFTER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/🎨️ui-preferences/🎨️updates-every-os-ui-preference/📸️snapshot/➡️after/🔣️.json")).expect("fixture"));
static MUTATIONS: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🧬️mutations/🎨️ui-preferences/🎨️updates-every-os-ui-preference/🦠️mutations/🔣️.json")).expect("fixture"));

fn preferences(text: &str) -> UiPreferences {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("OS UI preferences decode")
}

fn mutations() -> Vec<UiPreferencesConfigMutation> {
    serde_json::from_str::<Vec<serde_json::Value>>(MUTATIONS.as_str())
        .expect("mutation fixture is a JSON array")
        .into_iter()
        .map(|value| semio_framework_pack_json::from_json_str(&serde_json::to_string(&value).expect("mutation JSON encodes"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("OS UI-preferences mutation decodes"))
        .collect()
}

fn canonical_json<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("canonical JSON parses in serde_json")
}

#[test]
fn every_persisted_ui_preference_folds_and_inverts() {
    let original = preferences(BEFORE.as_str());
    let mut snapshot = original.clone();
    let mut inverses = Vec::new();
    for mutation in mutations() {
        inverses.push(<UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::inverse(&mutation, &snapshot).expect("valid retained mutation inverse fixture"));
        let outcome = <UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::diff(&mutation, &snapshot);
        snapshot = protocol::apply_diff_cold(outcome.diff(), &snapshot).expect("UI-preferences mutation applies");
        assert!(outcome.messages().is_empty(), "fixture mutations must change their requested preference");
    }
    assert_eq!(snapshot, preferences(AFTER.as_str()));
    for group in inverses.into_iter().rev() {
        for mutation in group {
            let outcome = <UiPreferencesConfigMutation as protocol::ColdMutation<UiPreferences>>::diff(&mutation, &snapshot);
            snapshot = protocol::apply_diff_cold(outcome.diff(), &snapshot).expect("inverse UI-preferences mutation applies");
        }
    }
    assert_eq!(snapshot, original);
}

#[test]
fn committed_language_neutral_json_matches_independent_serde_oracle() {
    for text in [BEFORE.as_str(), AFTER.as_str()] {
        let decoded = preferences(text);
        let expected: serde_json::Value = serde_json::from_str(text).expect("fixture JSON parses");
        assert_eq!(canonical_json(&decoded), expected);
    }
    let expected: Vec<serde_json::Value> = serde_json::from_str(MUTATIONS.as_str()).expect("mutation array parses");
    let actual: Vec<serde_json::Value> = mutations().iter().map(canonical_json).collect();
    assert_eq!(actual, expected);
}
