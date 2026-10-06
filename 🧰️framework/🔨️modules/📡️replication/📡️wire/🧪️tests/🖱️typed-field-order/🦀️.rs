//! 🖱️ Shared typed field-order controls preserve the full JSON oracle and optional field semantics.
use super::*;
use crate::value::{DslValue, FromValue, ToValue};

/// 🧬️ Compares complete typed producer occurrences to serde's canonical map oracle for every neutral case.
#[test]
fn interaction_typed_field_order_matches_shared_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖱️typed-field-order/🔣️.json")).expect("closed typed field-order fixture");
    for case in fixture["cases"].as_array().expect("typed field-order cases") {
        let input: serde_json::Value = serde_json::from_str(case["source"].as_str().expect("typed source")).expect("typed semantic oracle");
        let state: InteractionState = serde_json::from_value(input.clone()).expect("declared interaction state");
        let value = state.to_value();
        let DslValue::Object(entries) = &value else { panic!("typed interaction object") };
        assert_eq!(entries.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), fixture["rootFields"].as_array().expect("root fields").iter().map(|field| field.as_str().expect("root field")).collect::<Vec<_>>());
        assert_eq!(value, DslValue::from(&serde_json::to_value(&state).expect("independent typed serde oracle")), "{}", case["id"]);
        assert_eq!(InteractionState::from_value(value).expect("typed round trip"), state, "{}", case["id"]);
        for selection in state.selection.values() {
            let DslValue::Object(entries) = selection.to_value() else { panic!("typed selection object") };
            let expected = &fixture["selectionFields"][if selection.anchor_id.is_some() { "present" } else { "absent" }];
            assert_eq!(entries.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), expected.as_array().expect("selection fields").iter().map(|field| field.as_str().expect("selection field")).collect::<Vec<_>>());
            assert_eq!(DslValue::Object(entries), DslValue::from(&serde_json::to_value(selection).expect("independent selection serde oracle")));
        }
    }
    println!("[DEBUG] shared typed field-order controls {}", fixture["cases"].as_array().expect("cases").len());
}
