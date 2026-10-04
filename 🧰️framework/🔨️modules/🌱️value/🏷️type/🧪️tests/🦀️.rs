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
    for fields in [vec![("kind".into(), "boolean".to_value()), ("kind".into(), "boolean".to_value())], vec![("kind".into(), "list".to_value()), ("of".into(), "boolean".to_value()), ("of".into(), "boolean".to_value())]] {
        assert!(ValueType::from_value(DslValue::Object(fields)).is_err());
    }
}

#[test]
fn controlled_value_type_preserves_the_neutral_wire_without_ordinary_fallback() {
    for row in corpus()["wire"].as_array().unwrap() {
        let value = DslValue::from(row.clone());
        let mut yes = |_| true;
        let mut control = crate::NativeDecodeControl::new(1 << 20, &mut yes);
        let actual = ValueType::from_value_controlled(&value, &mut control).unwrap();
        assert_eq!(serde_json::Value::from(actual.to_value()), *row);
        assert!(control.owned_bytes() > 0);
        ValueType::retire_decoded(actual);
    }
}
#[test]
fn controlled_value_encoding_type_preserves_the_neutral_wire_without_ordinary_fallback() {
    for row in corpus()["wire"].as_array().unwrap() {
        let actual = ValueType::from_value(DslValue::from(row.clone())).unwrap();
        let mut yes = |_| true;
        let mut control = crate::NativeEncodeControl::new(1 << 20, &mut yes);
        let emitted = actual.to_value_controlled(&mut control).unwrap();
        assert_eq!(serde_json::Value::from(emitted), *row);
        assert!(control.owned_bytes() > 0);
        ValueType::retire_decoded(actual);
    }
}

#[test]
fn controlled_value_type_input_admits_deep_slots_and_retires_interior_cancellation() {
    let corpus = corpus();
    let depth = corpus["nativeControls"]["depth"].as_u64().unwrap() as usize;
    let stop = corpus["nativeControls"]["cancelAt"].as_u64().unwrap() as usize;
    let mut value = DslValue::object([("kind".into(), "text".to_value())]);
    for _ in 0..depth {
        value = DslValue::object([("kind".into(), "list".to_value()), ("of".into(), value)])
    }
    let value = value.guard_decoded();
    let mut yes = |_| true;
    let mut c = crate::NativeDecodeControl::new(1 << 20, &mut yes);
    let actual = ValueType::from_value_controlled(value.get(), &mut c).unwrap();
    ValueType::retire_decoded(actual);
    let bytes = c.owned_bytes();
    let mut c = crate::NativeDecodeControl::new(bytes, &mut yes);
    ValueType::retire_decoded(ValueType::from_value_controlled(value.get(), &mut c).unwrap());
    let mut c = crate::NativeDecodeControl::new(bytes - 1, &mut yes);
    assert!(ValueType::from_value_controlled(value.get(), &mut c).is_err());
    let mut hit = false;
    let mut cancel = |p: crate::native_decoding::NativeDecodeProgress| {
        if p.total == 0 && p.completed >= stop {
            hit = true;
            false
        } else {
            true
        }
    };
    let mut c = crate::NativeDecodeControl::new(1 << 20, &mut cancel);
    assert!(ValueType::from_value_controlled(value.get(), &mut c).is_err());
    assert!(hit);
    let mut fields = vec![("kind".into(), "any".to_value()), ("kind".into(), "any".to_value())];
    assert!(ValueType::from_value_controlled(&DslValue::Object(std::mem::take(&mut fields)), &mut crate::NativeDecodeControl::new(1 << 20, &mut yes)).is_err());
}
#[test]
fn controlled_value_encoding_type_output_admits_deep_slots_and_real_text_cancellation() {
    let corpus = corpus();
    let control = &corpus["nativeControls"];
    let depth = control["depth"].as_u64().unwrap() as usize;
    let mut value = ValueType::Text;
    for _ in 0..depth {
        value = ValueType::List(Box::new(value))
    }
    let value = crate::DecodedValue::new(value, ValueType::retire_decoded);
    let mut yes = |_| true;
    let mut c = crate::NativeEncodeControl::new(1 << 20, &mut yes);
    let emitted = value.get().to_value_controlled(&mut c).unwrap().guard_decoded();
    let bytes = c.owned_bytes();
    let mut c = crate::NativeEncodeControl::new(bytes, &mut yes);
    crate::FromValue::retire_decoded(value.get().to_value_controlled(&mut c).unwrap());
    let mut c = crate::NativeEncodeControl::new(bytes - 1, &mut yes);
    assert!(value.get().to_value_controlled(&mut c).is_err());
    let mut decode_yes = |_| true;
    let mut decode = crate::NativeDecodeControl::new(1 << 20, &mut decode_yes);
    ValueType::retire_decoded(ValueType::from_value_controlled(emitted.get(), &mut decode).unwrap());
    let schema = ValueType::Schema(control["longSchema"].as_str().unwrap().repeat(control["repeat"].as_u64().unwrap() as usize));
    let mut hit = false;
    let mut cancel = |p: crate::native_encoding::NativeEncodeProgress| {
        if p.total >= 65536 && p.completed >= 65536 && p.completed < p.total {
            hit = true;
            false
        } else {
            true
        }
    };
    let mut c = crate::NativeEncodeControl::new(1 << 20, &mut cancel);
    assert!(schema.to_value_controlled(&mut c).is_err());
    assert!(hit);
}
