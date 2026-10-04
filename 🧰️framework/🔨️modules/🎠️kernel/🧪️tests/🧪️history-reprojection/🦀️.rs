//! 📡️ The history reprojection status law, driven by the language-agnostic fixture `🧫️fixtures/🧫️history-reprojection/🔣️.json`
//! that the TypeScript twin (`🧪️tests/🧪️history-reprojection/🟦️.ts`) validates with Ajv: the kernel carries exactly its label
//! rows, in order, each in both locales, and reads every case to its title, text, paused flag and fault code.

use super::*;
use semio_framework_ui_locale::{Locale, Terminology};

const FIXTURE: &str = include_str!("../../🧫️fixtures/🧫️history-reprojection/🔣️.json");

#[test]
fn the_labels_mirror_the_fixture_in_both_locales() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("the history reprojection fixture is JSON");
    let rows: Vec<(&str, &str, &str)> = fixture["labels"].as_array().expect("labels").iter().map(|row| (row["key"].as_str().expect("key"), row["en"].as_str().expect("en"), row["de"].as_str().expect("de"))).collect();
    assert_eq!(rows, HISTORY_REPROJECTION_LABELS.to_vec());
    for (key, en, de) in HISTORY_REPROJECTION_LABELS {
        assert!(!en.is_empty() && !de.is_empty() && en != de, "{key} names both locales");
    }
}

#[test]
fn every_case_reads_to_its_status_in_both_locales() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("the history reprojection fixture is JSON");
    let cases = fixture["cases"].as_array().expect("cases");
    assert!(cases.len() >= 11, "the fixture covers every kind, pause and refusal");
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let reprojection: HistoryReprojection = serde_json::from_value(case["reprojection"].clone()).expect("a wire reprojection");
        for (locale, column) in [(Locale::En, "en"), (Locale::De, "de")] {
            let status = history_reprojection_status(&reprojection, Terminology::Native, locale);
            assert_eq!(status.title, case["title"][column].as_str().expect("title"), "{name} ({column}) title");
            assert_eq!(status.text, case["text"][column].as_str().expect("text"), "{name} ({column}) text");
            assert_eq!((status.done, status.total), (reprojection.done, reprojection.total), "{name} ({column}) progress");
            assert_eq!(status.paused, case["paused"].as_bool().expect("paused"), "{name} ({column}) paused");
            assert_eq!(status.fault.as_deref(), case["fault"].as_str(), "{name} ({column}) fault");
            assert!(status.fault.as_deref().is_none_or(|code| !status.text.contains(code)), "{name} ({column}) never shows its raw code");
            assert_eq!(history_reprojection_status(&reprojection, Terminology::Reuse, locale), status, "{name} ({column}) is terminology-invariant");
        }
    }
}
