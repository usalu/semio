//! 🧾️ The fixture shared with the TypeScript law drives serde as the independent Rust JSON oracle.
use super::QuestionDictionary;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("shared dictionary fixture is valid JSON")
}

#[test]
fn valid_rows_round_trip_through_the_native_json_limbs() {
    for row in fixture()["valid"].as_array().expect("valid rows") {
        let dictionary: QuestionDictionary = serde_json::from_value(row.clone()).expect("valid row is admitted");
        dictionary.validate().expect("valid row has unique identities");
        assert_eq!(&serde_json::to_value(&dictionary).expect("dictionary serializes"), row);
    }
}

#[test]
fn structurally_invalid_rows_are_refused() {
    for row in fixture()["structurallyInvalid"].as_array().expect("invalid rows") {
        assert!(serde_json::from_value::<QuestionDictionary>(row.clone()).is_err(), "{row}");
    }
}

#[test]
fn duplicate_identities_are_refused() {
    for row in fixture()["semanticallyInvalid"].as_array().expect("semantic rows") {
        let refused = serde_json::from_value::<QuestionDictionary>(row.clone()).map_or(true, |dictionary| dictionary.validate().is_err());
        assert!(refused, "{row}");
    }
}
