//! 🏷️ The Rust half of the shared `LocalizedLabel` contract. Same `🧫️fixtures/🏷️localized-label/🔣️.json`
//! the AJV oracle and `historyEntryLabelText` read in `🎠️kernel/🧪️tests/🏷️localized-label-fixture`.
//! Nothing below names a language: the axes come out of the generated enums and the expectations out
//! of the fixture's `<terminology>.<locale>` keys, so a new axis fails here until it is translated.
use super::LocalizedLabel;
use crate::{DslValue, FromValue, Locale, Terminology, ToValue};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🏷️localized-label/🔣️.json")).expect("localized-label fixture parses")
}

fn schema() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧬️schema/🔣️.json")).expect("localized-label schema parses")
}

/// 🌱️ Decodes through the crate's OWN `FromValue`, not through serde: that is the codec the pack
/// wire and every guest boundary actually use, and its inverse `ToValue` is asserted below.
fn decode(wire: &serde_json::Value) -> LocalizedLabel {
    LocalizedLabel::from_value(DslValue::from(wire)).expect("a LocalizedLabel wire decodes")
}

fn assert_resolves(label: &LocalizedLabel, expectations: &serde_json::Value, id: &str) {
    let expectations = expectations.as_object().expect("resolve table is an object");
    let mut seen = 0usize;
    for terminology in Terminology::ALL {
        for locale in Locale::ALL {
            let key = format!("{}.{}", terminology.as_str(), locale.as_str());
            let expected = expectations.get(&key).unwrap_or_else(|| panic!("row '{id}' has no expectation for '{key}' — a new axis needs a translated cell")).as_str().expect("expectation is a string");
            assert_eq!(label.resolve(terminology, locale), expected, "row '{id}' cell '{key}'");
            seen += 1;
        }
    }
    assert_eq!(seen, expectations.len(), "row '{id}' carries an expectation for an axis the build does not declare");
}

#[test]
fn the_fixture_declares_exactly_the_generated_axes() {
    let fixture = fixture();
    let axes = &fixture["axes"];
    let declared: Vec<&str> = axes["terminologies"].as_array().expect("terminologies").iter().map(|value| value.as_str().expect("terminology id")).collect();
    assert_eq!(declared, Terminology::ALL.iter().map(|value| value.as_str()).collect::<Vec<_>>());
    let declared: Vec<&str> = axes["locales"].as_array().expect("locales").iter().map(|value| value.as_str().expect("locale id")).collect();
    assert_eq!(declared, Locale::ALL.iter().map(|value| value.as_str()).collect::<Vec<_>>());
}

/// 🧬️ The schema twin must describe the SAME axes as the generated enums, or the AJV oracle and this
/// decoder stop agreeing about what a complete carrier is.
#[test]
fn the_schema_twin_requires_every_generated_axis_and_admits_no_other() {
    let schema = schema();
    let terminologies: Vec<&str> = schema["required"].as_array().expect("required").iter().map(|value| value.as_str().expect("required key")).collect();
    assert_eq!(terminologies, Terminology::ALL.iter().map(|value| value.as_str()).collect::<Vec<_>>());
    assert_eq!(schema["additionalProperties"], serde_json::Value::Bool(false));
    for terminology in Terminology::ALL {
        assert!(schema["properties"].get(terminology.as_str()).is_some(), "schema is missing terminology '{}'", terminology.as_str());
    }
    let row = &schema["definitions"]["localeRow"];
    let locales: Vec<&str> = row["required"].as_array().expect("locale required").iter().map(|value| value.as_str().expect("locale key")).collect();
    assert_eq!(locales, Locale::ALL.iter().map(|value| value.as_str()).collect::<Vec<_>>());
    assert_eq!(row["additionalProperties"], serde_json::Value::Bool(false));
    for locale in Locale::ALL {
        assert!(row["properties"].get(locale.as_str()).is_some(), "schema is missing locale '{}'", locale.as_str());
    }
}

#[test]
fn every_complete_row_decodes_to_the_cells_the_fixture_declares() {
    let fixture = fixture();
    let rows = fixture["rows"].as_array().expect("rows");
    assert!(!rows.is_empty());
    for row in rows {
        let id = row["id"].as_str().expect("row id");
        let label = decode(&row["wire"]);
        assert_resolves(&label, &row["resolve"], id);
        assert_eq!(serde_json::Value::from(label.to_value()), row["wire"], "row '{id}' must re-encode to the byte-identical wire it decoded from");
    }
}

/// 🚫️ The refusal rows are the no-fallback promise: a cell the wire did not carry resolves EMPTY,
/// never to another locale's text. The schema refuses these documents outright (asserted by the AJV
/// oracle); this side pins what the decoder does with one that reached it anyway.
#[test]
fn a_refused_row_resolves_its_absent_cells_empty_instead_of_falling_back() {
    let fixture = fixture();
    let refusals = fixture["refusals"].as_array().expect("refusals");
    assert!(!refusals.is_empty());
    for row in refusals {
        let id = row["id"].as_str().expect("refusal id");
        assert!(row["schemaError"].as_str().is_some_and(|text| !text.is_empty()), "refusal '{id}' must say what the schema objects to");
        let label = decode(&row["wire"]);
        assert_resolves(&label, &row["resolve"], id);
    }
}
