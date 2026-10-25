use super::*;
use serde_json::{json, Value};

fn decode(value: &Value) -> MapDelta<DslValue> {
    MapDelta::from_value(serde_json::from_value(value.clone()).unwrap()).unwrap()
}

fn base(value: &Value) -> BTreeMap<String, DslValue> {
    BTreeMap::from_value(serde_json::from_value(value.clone()).unwrap()).unwrap()
}

/// 🧪️ Shared neutral vectors check codec, strict application, compact composition and atomic failure.
#[test]
fn shared_map_delta_neutral_contract() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let expected = decode(&row["compact"]);
        let mut composed = MapDelta::default();
        for entry in row["source"].as_array().unwrap() { composed.absorb(decode(&json!({ "entries": { (entry["key"].as_str().unwrap()): { "precondition": entry["precondition"], "operation": entry["operation"] } } }))); }
        assert_eq!(composed, expected, "{}", row["name"]);
        let encoded = serde_json::to_value(expected.to_value()).unwrap();
        assert_eq!(encoded, row["compact"], "{}", row["name"]);
        assert_eq!(decode(&encoded), expected);
        let original = base(&row["before"]);
        let mut actual = original.clone();
        let outcome = expected.apply_onto(&mut actual);
        if let Some(code) = row["error"].as_str() {
            assert_eq!(outcome.unwrap_err().code, code, "{}", row["name"]);
            assert_eq!(actual, original, "failed map application must be atomic");
        } else {
            outcome.unwrap();
            assert_eq!(serde_json::to_value(actual.to_value()).unwrap(), row["after"], "{}", row["name"]);
        }
    }
    for row in fixture["invalid"].as_array().unwrap() {
        assert!(MapDelta::<DslValue>::from_value(serde_json::from_value(row.clone()).unwrap()).is_err());
    }
    let duplicate = json!({ "entries": [fixture["cases"][0]["source"][0], fixture["cases"][0]["source"][0]] });
    assert!(MapDelta::<DslValue>::from_value(serde_json::from_value(duplicate).unwrap()).is_err());
}

/// 🪢️ Every compact single-key triple has associative output and identical successful bases.
#[test]
fn shared_map_delta_exhaustive_composition() {
    let mut changes = Vec::new();
    for precondition in ["any", "present", "absent"] {
        for operation in [json!({"kind":"set","value":null}), json!({"kind":"remove"})] {
            changes.push(decode(&json!({"entries":{"key":{"precondition":precondition,"operation":operation}}})));
        }
    }
    for a in &changes { for b in &changes { for c in &changes {
        let mut left = a.clone(); left.absorb(b.clone()); left.absorb(c.clone());
        let mut right_tail = b.clone(); right_tail.absorb(c.clone());
        let mut right = a.clone(); right.absorb(right_tail);
        assert_eq!(left, right);
        for value in [json!({}), json!({"key":"old"}), json!({"key":null})] {
            let mut sequential = base(&value);
            let sequence = a.apply_onto(&mut sequential).and_then(|_| b.apply_onto(&mut sequential)).and_then(|_| c.apply_onto(&mut sequential));
            let mut compact = base(&value);
            let combined = left.apply_onto(&mut compact);
            assert_eq!(combined.is_ok(), sequence.is_ok());
            if combined.is_ok() { assert_eq!(compact, sequential); }
        }
    } } }
}

/// ↩️ Neutral map inverses restore exactly one authored key and keep a concurrent sibling value.
#[test]
fn shared_map_inverse_preserves_collaborator_fields() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["inverseCases"].as_array().unwrap() {
        let before = base(&case["before"]);
        let change = decode(&case["diff"]);
        let inverse = change.inverse(&before);
        let mut combined = before.clone();
        change.apply_onto(&mut combined).unwrap();
        combined.extend(base(&case["collaborator"]));
        inverse.apply_onto(&mut combined).unwrap();
        assert_eq!(serde_json::to_value(combined.to_value()).unwrap(), case["expected"]);
    }
}
/// 🛬️ The same neutral wire crosses explicit native and DSL controls without a payload fallback.
#[test]
fn shared_map_controlled_codec_and_dsl() {
    use semio_framework_value::NativeEncodeControl;
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use semio_framework_dsl_record::DslField;
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();

    use semio_framework_dsl_record::{BorrowedDslField, BorrowedDslRecord, BorrowedShape, BorrowedRecordSpecProducer};
    let map = <MapDelta<f64> as BorrowedDslRecord>::RECORD;
    let entry = <MapEntryDelta<f64> as BorrowedDslRecord>::RECORD;
    let operation = <MapEntryOperation<f64> as BorrowedDslRecord>::RECORD;
    for (key, spec) in [("map", map), ("entry", entry), ("operation", operation)] {
        assert_eq!(spec.layout, semio_framework_dsl_record::RecordLayout::Inline);
        let actual: Vec<_> = spec.fields.iter().map(|field| json!({"id": field.id, "key": field.key, "optional": field.optional})).collect();
        assert_eq!(json!(actual), fixture["dslFields"][key]);
    }
    assert!(matches!(<MapDelta<f64> as BorrowedDslField>::SHAPE, BorrowedShape::Record(_)));
    assert!(matches!(operation.fields[1].shape, BorrowedShape::Float));
    let mut metadata_yes = |_| true;
    let mut metadata_control = NativeEncodeControl::new(0, &mut metadata_yes);
    let producer = BorrowedRecordSpecProducer::of::<MapDelta<f64>>();
    let borrowed = producer.encode(&mut metadata_control).unwrap();
    let repeated = producer.encode(&mut metadata_control).unwrap();
    assert_eq!(borrowed.fields.as_ptr(), repeated.fields.as_ptr());
    assert_eq!(borrowed.fields.len(), map.fields.len());
    for (original, declared) in borrowed.fields.iter().zip(map.fields) {
        assert_eq!((original.id, original.key, original.optional), (declared.id, declared.key, declared.optional));
    }
    assert_eq!(metadata_control.owned_bytes(), 0);

    for row in fixture["cases"].as_array().unwrap() {
        let raw: DslValue = serde_json::from_value(row["compact"].clone()).unwrap();
        let mut yes = |_| true;
        let decoded = <MapDelta<DslValue> as FromValue>::from_value_controlled(&raw, &mut NativeDecodeControl::new(1 << 20, &mut yes)).unwrap();
        let mut encode_yes = |_| true;
        let encoded = ToValue::to_value_controlled(&decoded, &mut NativeEncodeControl::new(1 << 20, &mut encode_yes)).unwrap();
        assert_eq!(serde_json::to_value(encoded).unwrap(), row["compact"]);
        let fields = DslField::to_value_controlled(&decoded, &mut NativeEncodeControl::new(1 << 20, &mut encode_yes)).unwrap();
        let restored = <MapDelta<DslValue> as DslField>::from_value_controlled(&fields, &mut NativeDecodeControl::new(1 << 20, &mut yes)).unwrap();
        assert_eq!(restored, decoded);
        semio_framework_dsl_record::native_encoding::retire_field(fields);
        if !decoded.is_empty() { assert!(<MapDelta<DslValue> as FromValue>::from_value_controlled(&raw, &mut NativeDecodeControl::new(0, &mut yes)).is_err()); }
    }
    let entry: DslValue = serde_json::from_value(fixture["cases"][0]["compact"]["entries"].as_object().unwrap().values().next().unwrap().clone()).unwrap();
    let entries = DslValue::Object(vec![("a".into(), entry.clone()), ("a".into(), entry)]);
    let duplicate = DslValue::Object(vec![("entries".into(), entries)]);
    let mut yes = |_| true;
    assert!(<MapDelta<DslValue> as FromValue>::from_value_controlled(&duplicate, &mut NativeDecodeControl::new(1 << 20, &mut yes)).is_err());
}

/// 🧭️ Borrowed canonical fields reproduce the independently authored wire tree.
#[test]
fn shared_map_canonical_original_fields() {
    use semio_framework_pack_json::{ArtifactCanonicalJsonTree as Tree, ArtifactCanonicalJsonNode as Node};
    fn projection(tree: &dyn Tree) -> Value {
        match tree.canonical_tree_node().unwrap() {
            Node::Null => Value::Null, Node::Bool(value) => json!(value), Node::I64(value) => json!(value), Node::U64(value) => json!(value), Node::F64(value) => json!(value), Node::String(value) => json!(value),
            Node::Array(length) => Value::Array((0..length).map(|index| projection(tree.canonical_tree_child(index).unwrap())).collect()),
            Node::Object(length) => Value::Object((0..length).map(|index| { let key = tree.canonical_tree_key(index).unwrap(); let semio_framework_pack_json::ArtifactCanonicalJsonText::Contiguous(key) = key else { panic!("fixture uses literal key text") }; (key.to_owned(), projection(tree.canonical_tree_child(index).unwrap())) }).collect()),
            _ => panic!("neutral fixture has an unsupported scalar role"),
        }
    }
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() { assert_eq!(projection(&decode(&row["compact"])), row["compact"]); }
}