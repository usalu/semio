
use super::*;

#[test]
fn serde_json_value_round_trips_through_dsl_value() {
    let original = serde_json::json!({"name": "saw", "count": 3, "tags": ["a", "b"], "active": true, "note": null});
    let dsl: DslValue = DslValue::from(&original);
    let back = serde_json::Value::from(&dsl);
    assert_eq!(back, original);
}

#[test]
fn serde_json_integer_stays_exact_not_widened_to_f64() {
    let dsl: DslValue = DslValue::from(&serde_json::json!(7));
    assert_eq!(dsl.as_u64(), Some(7));
    assert_eq!(dsl.as_f64(), Some(7.0));
    assert!(matches!(dsl, DslValue::Number(Number::UInt(7))));
}

#[test]
fn serde_json_negative_integer_becomes_int_variant() {
    let dsl: DslValue = DslValue::from(&serde_json::json!(-7));
    assert_eq!(dsl.as_i64(), Some(-7));
    assert!(matches!(dsl, DslValue::Number(Number::Int(-7))));
}

#[test]
fn serde_json_float_stays_float_variant() {
    let dsl: DslValue = DslValue::from(&serde_json::json!(7.5));
    assert!(matches!(dsl, DslValue::Number(Number::Float(v)) if v == 7.5));
}

#[test]
fn uint_round_trips_as_bare_integer_text_through_serde_json_value() {
    let dsl = DslValue::uint(3600);
    let json = serde_json::Value::from(&dsl);
    assert_eq!(json.to_string(), "3600");
    assert_eq!(DslValue::from(&json).as_u64(), Some(3600));
}

#[test]
fn whole_float_keeps_its_decimal_point_through_serde_json_value() {
    let dsl = DslValue::float(3600.0);
    let json = serde_json::Value::from(&dsl);
    assert_eq!(json.to_string(), "3600.0");
}

/// 🪆️ Object-key-order-insensitive equality — a JSON object's key order carries no semantics,
/// and `DslValue::Object`'s `Vec`-backed derived `PartialEq` is positional, so a value that
/// round-tripped through `serde_json`'s (key-sorting) `Map` legitimately comes back with a
/// different entry order than it started with. Recurses into `Array`/`Object` children.
fn dsl_value_eq_ignoring_object_order(a: &DslValue, b: &DslValue) -> bool {
    match (a, b) {
        (DslValue::Array(x), DslValue::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(x, y)| dsl_value_eq_ignoring_object_order(x, y)),
        (DslValue::Object(x), DslValue::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.iter().find(|(ok, _)| ok == k).is_some_and(|(_, ov)| dsl_value_eq_ignoring_object_order(v, ov))),
        _ => a == b,
    }
}

#[test]
fn serde_json_uses_the_same_json_shape_as_the_dsl_value_bridge() {
    let original = DslValue::Object(vec![
        ("name".into(), DslValue::String("saw".into())),
        ("count".into(), DslValue::uint(3)),
        ("tags".into(), DslValue::Array(vec![DslValue::String("a".into()), DslValue::String("b".into())])),
        ("active".into(), DslValue::Bool(true)),
        ("note".into(), DslValue::Null),
    ]);
    let expected = serde_json::Value::from(&original);
    let actual = serde_json::to_value(&original).unwrap();
    assert_eq!(actual, expected);
    let round_tripped = serde_json::from_value::<DslValue>(actual).unwrap();
    assert!(dsl_value_eq_ignoring_object_order(&round_tripped, &original), "{round_tripped:?} != {original:?}");
}
