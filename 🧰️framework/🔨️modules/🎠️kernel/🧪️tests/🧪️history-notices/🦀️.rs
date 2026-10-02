//! 📢️ The history notice law, driven by the language-agnostic fixture `🧫️fixtures/🧫️history-notices/🔣️.json` that the
//! TypeScript twin (`🧪️tests/🧪️history-notices/🟦️.ts`) validates with Ajv: the kernel carries exactly its rows, in order,
//! each in both locales, and answers no notice for any other code.

use super::*;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🧫️history-notices/🔣️.json");

#[test]
fn the_notices_mirror_the_fixture_in_both_locales() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("the history notices fixture is JSON");
    let rows: Vec<(&str, &str, &str)> = fixture["notices"].as_array().expect("notices").iter().map(|row| (row["code"].as_str().expect("code"), row["en"].as_str().expect("en"), row["de"].as_str().expect("de"))).collect();
    assert_eq!(rows, HISTORY_NOTICE_LABELS.to_vec());
    for (code, en, de) in HISTORY_NOTICE_LABELS {
        assert_eq!(history_notice(code), Some((en, de)), "{code}");
        assert!(!en.is_empty() && !de.is_empty() && en != de, "{code} names both locales");
    }
    assert_eq!(history_notice("timeTravel.busy"), None, "a session refusal is the time-travel vocabulary's");
}
