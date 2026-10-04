//! 🎯️ Original Graph property admission against the lower language-neutral corpus.

use super::{dsl_value_to_property_value, property_value_matches_type};
use semio_framework_value::{FromValue, ValueType};

#[test]
fn graph_properties_preserve_all_original_type_classifications() {
    let fixture = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🏷️type/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let cases = fixture.get("graphCases").and_then(semio_framework_value::DslValue::as_array).unwrap();
    let types = fixture.get("types").and_then(semio_framework_value::DslValue::as_array).unwrap();
    let values = fixture.get("graphValues").and_then(semio_framework_value::DslValue::as_array).unwrap();
    for row in cases {
        let type_index = u64::from_value(row.get("type").unwrap().clone()).unwrap() as usize;
        let value_index = u64::from_value(row.get("value").unwrap().clone()).unwrap() as usize;
        let value_type = ValueType::from_value(types[type_index].get("type").unwrap().clone()).unwrap();
        let property = dsl_value_to_property_value(&values[value_index]);
        let expected = bool::from_value(row.get("accepted").unwrap().clone()).unwrap();
        assert_eq!(property_value_matches_type(&property, &value_type), expected, "{}", String::from_value(row.get("name").unwrap().clone()).unwrap());
    }
}

#[test]
fn graph_property_controlled_constructor_preserves_canonical_type_and_optional_expression() {
    use super::{PropertyDef, PropertyKind};
    use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️type/🔣️.json")).unwrap();
    for row in fixture["types"].as_array().unwrap() {
        let type_value = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&row["type"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        let ty = semio_framework_value::ValueType::from_value(type_value.clone()).unwrap();
        let expected = PropertyDef { name: "literal\0!@/引用".into(), kind: PropertyKind::Derived, value_type: ty, expr: Some(String::new()) };
        let input = DslValue::object([("name".into(), expected.name.to_value()), ("kind".into(), "derived".to_value()), ("valueType".into(), type_value), ("expr".into(), String::new().to_value())]);
        let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&input))).unwrap();
        assert_eq!(oracle["valueType"], row["type"]);
        let mut yes = |_| true;
        let mut c = NativeDecodeControl::new(1 << 20, &mut yes);
        let actual = PropertyDef::from_value_controlled(&input, &mut c).unwrap();
        assert_eq!(actual, expected);
        PropertyDef::retire_decoded(actual);
    }
}

#[test]
fn graph_property_controlled_constructor_preserves_declared_type_spellings_and_expression_presence() {
    use super::PropertyDef;
    use semio_framework_value::{DslValue, NativeDecodeControl, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️type/🔣️.json")).unwrap();
    for case in fixture["controlledProperties"].as_array().unwrap() {
        for expression in [None, Some("")] {
            let ty = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&case["input"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
            let mut fields = vec![("name".into(), "literal\0引用".to_value()), ("kind".into(), "data".to_value()), ("valueType".into(), ty)];
            if let Some(expression) = expression {
                fields.push(("expr".into(), expression.to_value()));
            }
            let mut yes = |_| true;
            let actual = PropertyDef::from_value_controlled(&DslValue::Object(fields), &mut NativeDecodeControl::new(1 << 20, &mut yes)).unwrap();
            let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&actual.value_type.to_value()))).unwrap();
            assert_eq!(oracle, case["expected"]);
            assert_eq!(actual.expr.as_deref(), expression);
            PropertyDef::retire_decoded(actual);
        }
    }
}

#[test]
fn graph_property_controlled_constructor_bounds_deep_types_and_partial_expression_copies() {
    use super::PropertyDef;
    use semio_framework_value::{DslValue, NativeDecodeControl, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️type/🔣️.json")).unwrap();
    let work = &fixture["controlledWork"];
    let schema = work["schemaUnit"].as_str().unwrap().repeat(work["repeat"].as_u64().unwrap() as usize);
    let expression = work["expressionUnit"].as_str().unwrap().repeat(work["repeat"].as_u64().unwrap() as usize);
    let mut ty = DslValue::object([("kind".into(), "schema".to_value()), ("of".into(), schema.to_value())]);
    for _ in 0..work["depth"].as_u64().unwrap() {
        ty = DslValue::object([("kind".into(), "list".to_value()), ("of".into(), ty)]);
    }
    let input = DslValue::object([("name".into(), "deep\0引用".to_value()), ("kind".into(), "derived".to_value()), ("valueType".into(), ty), ("expr".into(), expression.to_value())]);
    let mut yes = |_| true;
    let mut probe = NativeDecodeControl::new(1 << 24, &mut yes);
    let actual = PropertyDef::from_value_controlled(&input, &mut probe).unwrap();
    assert_eq!(actual.expr.as_deref(), Some(expression.as_str()));
    let bytes = probe.owned_bytes();
    PropertyDef::retire_decoded(actual);
    let actual = PropertyDef::from_value_controlled(&input, &mut NativeDecodeControl::new(bytes, &mut yes)).unwrap();
    PropertyDef::retire_decoded(actual);
    assert_eq!(PropertyDef::from_value_controlled(&input, &mut NativeDecodeControl::new(bytes - 1, &mut yes)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
    let mut interrupted = false;
    let mut cancel = |p: semio_framework_value::native_decoding::NativeDecodeProgress| {
        let stop = p.total == expression.len() && p.completed >= 65536 && p.completed < p.total;
        interrupted |= stop;
        !stop
    };
    assert_eq!(PropertyDef::from_value_controlled(&input, &mut NativeDecodeControl::new(bytes, &mut cancel)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);
    assert!(interrupted);
    <DslValue as FromValue>::retire_decoded(input);
}
