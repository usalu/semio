//! 🧪️ Language-neutral OS UI-preference mutation fixture.

use super::super::super::UiPreferences;
use super::super::UiPreferencesConfigMutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATIONS: &str = include_str!("🦠️mutations/🔣️.json");

fn preferences(text: &str) -> UiPreferences {
    dsl::os_pack::json::from_json_str(text).expect("OS UI preferences decode")
}

fn mutations() -> Vec<UiPreferencesConfigMutation> {
    serde_json::from_str::<Vec<serde_json::Value>>(MUTATIONS)
        .expect("mutation fixture is a JSON array")
        .into_iter()
        .map(|value| dsl::os_pack::json::from_json_str(&serde_json::to_string(&value).expect("mutation JSON encodes")).expect("OS UI-preferences mutation decodes"))
        .collect()
}

fn canonical_json<T: dsl::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&dsl::os_pack::json::to_json_string(value)).expect("canonical JSON parses in serde_json")
}

#[test]
fn every_persisted_ui_preference_folds_and_inverts() {
    let original = preferences(BEFORE);
    let mut snapshot = original.clone();
    let mut inverses = Vec::new();
    for mutation in mutations() {
        inverses.push(<UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::inverse(&mutation, &snapshot));
        let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation, &snapshot);
        snapshot = protocol::MutationDiff::apply(outcome.diff(), &snapshot).expect("UI-preferences mutation applies");
        assert!(outcome.messages().is_empty(), "fixture mutations must change their requested preference");
    }
    assert_eq!(snapshot, preferences(AFTER));
    for group in inverses.into_iter().rev() {
        for mutation in group {
            let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation, &snapshot);
            snapshot = protocol::MutationDiff::apply(outcome.diff(), &snapshot).expect("inverse UI-preferences mutation applies");
        }
    }
    assert_eq!(snapshot, original);
}

#[test]
fn committed_language_neutral_json_matches_independent_serde_oracle() {
    for text in [BEFORE, AFTER] {
        let decoded = preferences(text);
        let expected: serde_json::Value = serde_json::from_str(text).expect("fixture JSON parses");
        assert_eq!(canonical_json(&decoded), expected);
    }
    let expected: Vec<serde_json::Value> = serde_json::from_str(MUTATIONS).expect("mutation array parses");
    let actual: Vec<serde_json::Value> = mutations().iter().map(canonical_json).collect();
    assert_eq!(actual, expected);
}
