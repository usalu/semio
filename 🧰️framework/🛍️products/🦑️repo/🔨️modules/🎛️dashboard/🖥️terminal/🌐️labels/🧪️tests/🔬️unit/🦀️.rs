use super::*;
use crate::preferences::keymap::{Keymap, Scope};

fn collect(labels: &DashboardLabels) -> Vec<(&'static str, &'static str)> {
    let mut rows = Vec::new();
    labels.for_each_label(|name, text| rows.push((name, text.as_str())));
    rows
}

fn placeholders(text: &str) -> Vec<String> {
    let mut found: Vec<String> = text.split('{').skip(1).filter_map(|rest| rest.split_once('}').map(|(name, _)| name.to_string())).collect();
    found.sort();
    found
}

#[test]
fn every_label_is_complete_in_every_language_with_the_same_placeholders() {
    let (english, german) = (collect(&DashboardLabels::NATIVE_EN), collect(&DashboardLabels::NATIVE_DE));
    assert_eq!(english.len(), german.len());
    for ((name, en), (_, de)) in english.iter().zip(&german) {
        assert!(!en.is_empty() && !de.is_empty(), "{name}");
        assert_eq!(placeholders(en), placeholders(de), "{name} must fill the same values in both languages");
    }
    let translated = english.iter().zip(&german).filter(|((_, en), (_, de))| en != de).count();
    assert!(translated * 10 > english.len() * 9, "{translated} of {} labels differ between languages", english.len());
}

#[test]
fn every_keymap_action_has_a_text_in_every_language() {
    let keymap = Keymap::defaults();
    for locale in Locale::ALL {
        let labels = labels(locale);
        for scope in Scope::ALL { for action in keymap.actions(scope) { assert!(labels.try_action(&action).is_some_and(|text| !text.is_empty()), "{action} has no {locale} text"); } }
    }
}

#[test]
fn the_catalogue_follows_the_chosen_language() {
    assert_eq!(labels(Locale::En).title_tasks.as_str(), "Tasks");
    assert_eq!(labels(Locale::De).title_tasks.as_str(), "Aufgaben");
    assert_eq!(labels(Locale::De).exit_code.fill(&[("code", "3")]).as_str(), "Code 3");
}

#[test]
fn command_line_messages_follow_the_language_of_the_preference() {
    let root = std::path::Path::new("workspace-without-preferences");
    let by_flag = |language: &str| labels(crate::preferences::cli_locale(root, &crate::args::parse(&["daemon".to_string(), "--language".to_string(), language.to_string()])));
    let (english, german) = (by_flag("en"), by_flag("de"));
    assert_eq!(english.cli_daemon_usage.as_str(), "usage: semio daemon start|stop|status|attach|serve");
    assert_eq!(german.cli_daemon_usage.as_str(), "Verwendung: semio daemon start|stop|status|attach|serve");
    assert_eq!(english.cli_daemon_stopped.as_str(), "stopped workspace dashboard daemon");
    assert_eq!(german.cli_daemon_stopped.as_str(), "Dashboard-Dienst des Arbeitsbereichs gestoppt");
    assert_eq!(german.cli_unknown_verb.fill(&[("verb", "x")]).as_str(), "[semio] unbekannter Befehl x");
    assert_eq!(german.cli_daemon_status.fill(&[("pid", "7"), ("build", "b"), ("count", "2"), ("endpoint", "e")]).as_str(), "Dienst pid 7 · Build b · 2 aktive Aufgaben · e");
}

#[test]
fn every_scenario_of_the_labels_feature_is_proved_by_a_test() {
    crate::tests::assert_proved(include_str!("../../../../🧪️tests/🌐️labels/🥒️.feature"), &[include_str!("🦀️.rs"), include_str!("../../../🧪️tests/🔬️unit/🦀️.rs")], &[
        ("Every label is complete in every language with the same placeholders", &["every_label_is_complete_in_every_language_with_the_same_placeholders"]),
        ("Every keymap action has a text in every language", &["every_keymap_action_has_a_text_in_every_language"]),
        ("The catalogue follows the chosen language", &["the_catalogue_follows_the_chosen_language"]),
        ("No visible text is hard-coded outside the catalogue", &["p1_11_no_user_visible_text_is_hard_coded_outside_the_catalogue"]),
        ("The messages of the command line follow the preference", &["command_line_messages_follow_the_language_of_the_preference"]),
    ]);
}
