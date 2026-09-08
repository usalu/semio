
use super::*;
use crate::protocol::PromptRegistry;

#[test]
fn the_prompt_census_matches_the_registry_exactly() {
    let registry = build_prompt_registry();
    let mut listed: Vec<String> = registry.list().into_iter().map(|prompt| prompt.name).collect();
    listed.sort();
    let mut census: Vec<String> = GATEWAY_PROMPT_NAMES.iter().map(|name| (*name).to_string()).collect();
    census.sort();
    assert_eq!(listed, census);
}

#[test]
fn every_prompt_answers_in_both_languages_with_distinct_bodies() {
    let registry = build_prompt_registry();
    for name in GATEWAY_PROMPT_NAMES {
        let english = registry.get(name, Some(serde_json::json!({ "locale": "en" }))).expect("english resolves");
        let german = registry.get(name, Some(serde_json::json!({ "locale": "de" }))).expect("german resolves");
        let (ContentBlock::Text { text: english_text }, ContentBlock::Text { text: german_text }) = (&english.messages[0].content, &german.messages[0].content) else {
            panic!("{name} must answer with text");
        };
        assert!(!english_text.is_empty() && !german_text.is_empty(), "{name} has an empty body");
        assert_ne!(english_text, german_text, "{name} is not actually translated");
    }
}

#[test]
fn an_absent_or_unknown_locale_resolves_to_english_rather_than_failing() {
    let registry = build_prompt_registry();
    let default = registry.get("safe_mutation", None).expect("a missing locale still resolves");
    let nonsense = registry.get("safe_mutation", Some(serde_json::json!({ "locale": "kl" }))).expect("an unknown locale still resolves");
    let english = registry.get("safe_mutation", Some(serde_json::json!({ "locale": "en" }))).expect("english resolves");
    assert_eq!(default.messages, english.messages);
    assert_eq!(nonsense.messages, english.messages);
}

#[test]
fn a_regional_german_tag_resolves_to_german() {
    assert_eq!(PromptLocale::resolve(Some("de-CH")), PromptLocale::De);
    assert_eq!(PromptLocale::resolve(Some("DE")), PromptLocale::De);
    assert_eq!(PromptLocale::resolve(Some(" de ")), PromptLocale::De);
    assert_eq!(PromptLocale::resolve(None), PromptLocale::En);
}

#[test]
fn an_unknown_prompt_name_is_a_well_formed_not_found() {
    let registry = build_prompt_registry();
    let error = registry.get("no_such_prompt", None).expect_err("unknown prompts are not found");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
    assert!(prompt_body("no_such_prompt", PromptLocale::En).is_err());
}

/// 🔌️ The whole point of this facet: a prompt teaches the PROTOCOL, so it must never hardcode a
/// plugin's vocabulary. If a prompt names a specific plugin, installing a different plugin set
/// would silently make it wrong.
#[test]
fn no_prompt_names_a_specific_plugin_or_artifact_kind() {
    for definition in DEFINITIONS {
        for body in [definition.body_en, definition.body_de] {
            let lowered = body.to_ascii_lowercase();
            for forbidden in ["note", "cad", "puzzle", "sketchpad", "procedural"] {
                assert!(!lowered.contains(forbidden), "{} names the plugin `{forbidden}`", definition.name);
            }
        }
    }
}
