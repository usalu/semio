//! 🧾️ The history wire law, driven by the language-agnostic fixture `🧫️fixtures/🧫️history-patch/🔣️.json` that the
//! TypeScript twin (`🧪️tests/🧪️history-patch/🟦️.ts`) validates with Ajv: every valid patch decodes to the same value
//! through serde and through `FromValue`, survives both re-encodings, and folds its rows under the fixture keys; every
//! invalid patch is refused by both decoders.

use super::*;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🧫️history-patch/🔣️.json");

fn cases(section: &str) -> Vec<(String, serde_json::Value, serde_json::Value)> {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("the history patch fixture is JSON");
    fixture[section].as_array().expect("fixture section").iter().map(|case| (case["id"].as_str().expect("case id").to_string(), case["patch"].clone(), case.get("keys").cloned().unwrap_or(serde_json::Value::Null))).collect()
}

fn value_decode(json: &serde_json::Value) -> Result<HistoryPatch, dsl::ValueError> {
    let value = dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(&json.to_string()).expect("fixture patch is JSON"));
    <HistoryPatch as dsl::FromValue>::from_value(value)
}

#[test]
fn every_valid_patch_decodes_identically_through_serde_and_value_and_round_trips() {
    for (id, json, keys) in cases("valid") {
        let by_serde: HistoryPatch = serde_json::from_value(json.clone()).unwrap_or_else(|error| panic!("{id}: serde refused a valid patch: {error}"));
        let by_value = value_decode(&json).unwrap_or_else(|error| panic!("{id}: FromValue refused a valid patch: {error}"));
        assert_eq!(by_serde, by_value, "{id}: serde and value decode differently");
        let reserialized: HistoryPatch = serde_json::from_value(serde_json::to_value(&by_serde).expect("serde encodes")).expect("serde decodes its own encoding");
        assert_eq!(reserialized, by_serde, "{id}: serde round trip");
        let revalued = <HistoryPatch as dsl::FromValue>::from_value(dsl::ToValue::to_value(&by_value)).expect("value decodes its own encoding");
        assert_eq!(revalued, by_value, "{id}: value round trip");
        let folded: Vec<String> = by_value.upserts.iter().map(HistoryEntry::key).collect();
        let expected: Vec<String> = keys.as_array().expect("keys").iter().map(|key| key.as_str().expect("key").to_string()).collect();
        assert_eq!(folded, expected, "{id}: row keys");
    }
}

#[test]
fn every_invalid_patch_is_refused_by_both_decoders() {
    for (id, json, _) in cases("invalid") {
        assert!(serde_json::from_value::<HistoryPatch>(json.clone()).is_err(), "{id}: serde accepted an invalid patch");
        assert!(value_decode(&json).is_err(), "{id}: FromValue accepted an invalid patch");
    }
}

#[test]
fn an_absent_time_travel_is_omitted_from_both_encodings() {
    let patch = HistoryPatch { cursor: 1, ..HistoryPatch::default() };
    assert!(serde_json::to_value(&patch).expect("serde encodes").get("timeTravel").is_none());
    assert!(dsl::ToValue::to_value(&patch).get("timeTravel").is_none());
}
