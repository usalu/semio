use super::*;
use dsl::{FromValue, ToValue};

#[test]
fn installed_owner_service_effects_match_the_portable_and_serde_oracles() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/💡️service-operation/🔣️.json")).unwrap();
    for vector in fixture["effects"].as_array().unwrap() {
        let oracle: Effect = serde_json::from_value(vector.clone()).unwrap();
        let value = dsl::json::from_json_str::<DslValue>(&vector.to_string()).unwrap();
        let effect = Effect::from_value(value).unwrap();
        assert_eq!(effect, oracle);
        let round_trip = Effect::from_value(effect.to_value()).unwrap();
        assert_eq!(round_trip, oracle);
        assert_eq!(serde_json::to_value(round_trip).unwrap(), *vector);
    }
}
