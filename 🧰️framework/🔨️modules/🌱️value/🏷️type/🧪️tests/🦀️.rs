//! 🧪️ Language-neutral type classifications and canonical wire laws.

use super::{ValueKind, ValueType};
use crate::{DslValue, FromValue, ToValue};

fn corpus() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}

#[test]
fn value_type_all_owned_classifications_match_the_closed_corpus() {
    let fixture = corpus();
    for row in fixture["cases"].as_array().unwrap() {
        let type_row = &fixture["types"][row["type"].as_u64().unwrap() as usize];
        let value_type = ValueType::from_value(DslValue::from(type_row["type"].clone())).unwrap();
        let kind = &fixture["kinds"][row["kind"].as_u64().unwrap() as usize];
        let classification = match kind["kind"].as_str().unwrap() {
            "null" => ValueKind::Null,
            "boolean" => ValueKind::Boolean,
            "integer" => ValueKind::Integer,
            "decimal" => ValueKind::Decimal,
            "text" => ValueKind::Text,
            "dictionary" => ValueKind::Dictionary(kind.get("schema").and_then(serde_json::Value::as_str)),
            other => panic!("unknown kind {other}"),
        };
        assert_eq!(value_type.id(), type_row["id"].as_str().unwrap(), "{}", row["name"]);
        assert_eq!(value_type.matches(classification), row["accepted"].as_bool().unwrap(), "{}", row["name"]);
    }
}

#[test]
fn value_type_wire_round_trips_every_original_variant_and_nested_list() {
    for row in corpus()["wire"].as_array().unwrap() {
        let subject = ValueType::from_value(DslValue::from(row.clone())).unwrap();
        let emitted = serde_json::Value::from(subject.to_value());
        assert_eq!(&emitted, row);
        assert_eq!(ValueType::from_value(subject.to_value()).unwrap(), subject);
    }
}

#[test]
fn value_type_wire_refuses_closed_hostile_objects_and_duplicate_fields() {
    for row in corpus()["refused"].as_array().unwrap() {
        assert!(ValueType::from_value(DslValue::from(row.clone())).is_err(), "{row}");
    }
    for fields in [
        vec![("kind".into(), "boolean".to_value()), ("kind".into(), "boolean".to_value())],
        vec![("kind".into(), "list".to_value()), ("of".into(), "boolean".to_value()), ("of".into(), "boolean".to_value())],
    ] {
        assert!(ValueType::from_value(DslValue::Object(fields)).is_err());
    }
}
