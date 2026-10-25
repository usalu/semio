use super::*;
use dashboard_preferences::keymap::{Keymap, Scope};

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
    let by_flag = |language: &str| labels(dashboard_preferences::cli_locale(root, &dashboard_core::args::parse(&["daemon".to_string(), "--language".to_string(), language.to_string()])).expect("explicit language"));
    let (english, german) = (by_flag("en"), by_flag("de"));
    assert_eq!(english.cli_daemon_usage.as_str(), "usage: repo dashboard daemon start|stop|status|attach|serve");
    assert_eq!(german.cli_daemon_usage.as_str(), "Verwendung: repo dashboard daemon start|stop|status|attach|serve");
    assert_eq!(english.cli_daemon_stopped.as_str(), "stopped workspace dashboard daemon");
    assert_eq!(german.cli_daemon_stopped.as_str(), "Dashboard-Dienst des Arbeitsbereichs gestoppt");
    assert_eq!(german.cli_unknown_verb.fill(&[("verb", "x")]).as_str(), "[semio] unbekannter Befehl x");
    assert_eq!(german.cli_daemon_status.fill(&[("pid", "7"), ("build", "b"), ("count", "2"), ("endpoint", "e")]).as_str(), "Dienst pid 7 · Build b · 2 aktive Aufgaben · e");
}

#[test]
fn every_scenario_of_the_labels_feature_is_proved_by_a_test() {
    dashboard_core::testkit::assert_proved(include_str!("../../../../🧪️tests/🌐️labels/🥒️.feature"), &[include_str!("🦀️.rs"), include_str!("../../../🧪️tests/🔬️unit/🦀️.rs"), include_str!("../../../🧪️tests/🔬️unit/🩺️runtime-audit/🦀️.rs")], &[
        ("Every label is complete in every language with the same placeholders", &["every_label_is_complete_in_every_language_with_the_same_placeholders"]),
        ("Every keymap action has a text in every language", &["every_keymap_action_has_a_text_in_every_language"]),
        ("The catalogue follows the chosen language", &["the_catalogue_follows_the_chosen_language"]),
        ("No visible text is hard-coded outside the catalogue", &["p1_11_no_user_visible_text_is_hard_coded_outside_the_catalogue"]),
        ("The messages of the command line follow the preference", &["command_line_messages_follow_the_language_of_the_preference"]),
        ("A document caption keeps its state and position when the path is long", &["a_document_caption_keeps_its_state_and_position_when_the_path_is_long"]),
    ]);
}

#[test]
fn tab_text_keeps_the_verb_and_elides_the_subject_in_the_middle() {
    let label = TaskLabel { verb: "dev".into(), owner: vec![], subject: "a-very-long-subject-name-indeed".into(), qualifier: "react".into(), parameters: vec![], members: 0 };
    let text = tab_text(&label, Locale::En, TAB_CELLS);
    assert!(text.starts_with("dev ") && text.contains('…') && cells(&text) <= TAB_CELLS, "{text}");
    assert_eq!(tab_text(&TaskLabel { subject: "hub".into(), ..label.clone() }, Locale::En, TAB_CELLS), "dev hub react");
    assert_eq!(tab_texts(&[&label, &label], Locale::En, TAB_CELLS).iter().filter(|text| text.ends_with(" ·2")).count(), 1);
    assert_eq!(window_title(&TaskLabel { subject: "hub".into(), owner: vec!["hub".into()], members: 2, parameters: vec![("backend".into(), "pg".into())], ..label }, Locale::De), "entwickeln · hub ×2 · react · backend=pg");
}

#[test]
fn a_document_caption_keeps_its_state_and_position_when_the_path_is_long() {
    let root = (if cfg!(unix) { std::path::PathBuf::from("/tmp") } else { std::env::temp_dir() }).join(format!("sc{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let folders = ["a".repeat(30), "b".repeat(30), "c".repeat(30), "d".repeat(30)].join("/");
    std::fs::create_dir_all(root.join(&folders)).unwrap();
    let path = format!("{folders}/ende.txt");
    std::fs::write(root.join(&path), "x").unwrap();
    let document = semio_framework_filesystem_workspace::Workspace::new(&root).unwrap().load(&path, &std::sync::atomic::AtomicBool::new(false), &mut |_| {}).unwrap();
    let buffer = semio_framework_filesystem_workspace::commands::DocumentBuffer::new(document, false);
    let _ = std::fs::remove_dir_all(&root);
    for (locale, state, position) in [(Locale::En, "saved", "line 1 · column 1"), (Locale::De, "gespeichert", "Zeile 1 · Spalte 1")] {
        let text = labels(locale);
        let full = text.source_document_caption(&buffer, position, 0);
        assert_eq!(full, format!("{path} · {state} · {position}"));
        for width in [40u16, 60, 100] {
            let fitted = text.source_document_caption(&buffer, position, width);
            assert!(cells(&fitted) <= usize::from(width), "{width}: {fitted}");
            assert!(fitted.ends_with(&format!(" · {state} · {position}")) && fitted.contains('…') && fitted.starts_with('a'), "{width}: {fitted}");
        }
        assert_eq!(text.source_document_caption(&buffer, position, 400), full);
    }
}
