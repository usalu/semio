/// 🌉️ Value bridge and macro laws of the pack-json companion: the dsl value round trips, the json macro and the serde bridge bytes.

//#region 🔖️ToFromValueBridge
/// 🌉️ `from_dsl_value`/`to_dsl_value` (`//#region 🔖️DslValueBridge` above) had no direct test
/// yet — this exercises the structural walk this region's `to_json_string`/`from_json_str`
/// are built on.
#[test]
fn from_dsl_value_and_to_dsl_value_round_trip_every_shape() {
    let value = DslValue::object([("a".to_string(), DslValue::uint(1)), ("b".to_string(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null, DslValue::String("x".to_string())]))]);
    assert_eq!(to_dsl_value(&from_dsl_value(&value)), value);
}

#[test]
fn to_json_string_and_from_json_str_round_trip_a_dsl_value() {
    let value = DslValue::object([("count".to_string(), DslValue::uint(3)), ("label".to_string(), DslValue::String("ok".to_string()))]);
    let text = to_json_string(&value);
    let parsed: DslValue = from_json_str(&text, crate::JsonMemberPolicy::Replace).unwrap();
    assert_eq!(parsed, value);
}

#[test]
fn from_json_str_reports_a_value_error_on_malformed_text() {
    assert!(from_json_str::<DslValue>("not json", crate::JsonMemberPolicy::Replace).is_err());
}

/// 🎯️ The exact regression named in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/
/// 📓️directory-spr-serde-removal.md`: `DirectoryCommand::CreateInvite.ttl_secs: u64` posted
/// through `to_json_string(&command.to_value())` must produce `"ttlSecs":3600` on the wire, not
/// `"ttlSecs":3600.0` — a real Rust/serde hub rejects a float literal for a `u64` field. A
/// genuine `f64` field must keep its `.0` so it never collapses onto its integer twin.
#[test]
fn u64_field_through_to_value_and_to_json_string_renders_as_a_bare_integer() {
    let ttl_secs: u64 = 3600;
    let text = to_json_string(&DslValue::object([("ttlSecs".to_string(), ttl_secs.to_value())]));
    assert_eq!(text, r#"{"ttlSecs":3600}"#);
    let parsed: DslValue = from_json_str(&text, crate::JsonMemberPolicy::Replace).unwrap();
    assert_eq!(parsed.get("ttlSecs").and_then(DslValue::as_u64), Some(3600));

    let ratio: f64 = 3600.0;
    let text = to_json_string(&DslValue::object([("ratio".to_string(), ratio.to_value())]));
    assert_eq!(text, r#"{"ratio":3600.0}"#);
}

//#region 🔖️JsonMacro
#[test]
fn json_macro_builds_scalars_and_null() {
    assert_eq!(crate::json!(null), Value::Null);
    assert_eq!(crate::json!(true), Value::Bool(true));
    assert_eq!(crate::json!(false), Value::Bool(false));
    assert_eq!(crate::json!(1), Value::Number(Number::Int(1)));
    assert_eq!(crate::json!(1.5), Value::Number(Number::Float(1.5)));
    assert_eq!(crate::json!("hi"), Value::String("hi".to_string()));
}

#[test]
fn json_macro_builds_arrays_incl_empty_and_nested() {
    assert_eq!(crate::json!([]), Value::Array(vec![]));
    assert_eq!(crate::json!([1, 2, 3]), Value::Array(vec![Value::Number(Number::Int(1)), Value::Number(Number::Int(2)), Value::Number(Number::Int(3))]));
    assert_eq!(crate::json!([[1], [2, 3]]), Value::Array(vec![Value::Array(vec![Value::Number(Number::Int(1))]), Value::Array(vec![Value::Number(Number::Int(2)), Value::Number(Number::Int(3))])]));
}

#[test]
fn json_macro_builds_objects_incl_empty_and_trailing_commas() {
    assert_eq!(crate::json!({}), Value::Object(Object::new()));
    let value = crate::json!({
        "a": 1,
        "b": [1, 2],
    });
    assert_eq!(value.get("a").unwrap().as_i64(), Some(1));
    assert_eq!(value.get("b").unwrap().as_array().unwrap().len(), 2);
}

#[test]
fn json_macro_evaluates_arbitrary_expressions_and_options() {
    let index = 3;
    let value = crate::json!({
        "id": format!("semio_text-{index}"),
        "meshId": "box",
        "position": [index as f64 * 2.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "label": format!("Semio Text {index}"),
        "smoothShading": false,
        "nested": { "deep": { "deeper": [1, 2, 3] } },
        "present": Some(5),
        "absent": Option::<i32>::None,
    });
    assert_eq!(value.get("id").unwrap().as_str(), Some("semio_text-3"));
    assert_eq!(value.get("position").unwrap().as_array().unwrap()[0].as_f64(), Some(6.0));
    assert_eq!(value.get("nested").unwrap().get("deep").unwrap().get("deeper").unwrap().as_array().unwrap().len(), 3);
    assert_eq!(value.get("present").unwrap().as_i64(), Some(5));
    assert!(value.get("absent").unwrap().is_null());
}

#[test]
fn json_macro_matches_to_string_of_equivalent_hand_built_value() {
    let via_macro = crate::json!({"a": 1, "b": [true, null, "x"]});
    let hand_built = Value::Object(Object::from_iter([("a".to_string(), Value::Number(Number::Int(1))), ("b".to_string(), Value::Array(vec![Value::Bool(true), Value::Null, Value::String("x".to_string())]))]));
    assert_eq!(to_string(&via_macro), to_string(&hand_built));
}
#[test]
fn value_eq_ignoring_object_order_is_order_insensitive_but_still_structural() {
    let a = crate::json!({"x": 1, "y": [1, 2, {"p": true, "q": "s"}]});
    let b = crate::json!({"y": [1, 2, {"q": "s", "p": true}], "x": 1});
    assert!(value_eq_ignoring_object_order(&a, &b));
    let c = crate::json!({"x": 1, "y": [1, 2, {"p": true, "q": "different"}]});
    assert!(!value_eq_ignoring_object_order(&a, &c));
    let d = crate::json!({"x": 1});
    assert!(!value_eq_ignoring_object_order(&a, &d));
}

#[test]
fn mutable_accessors_update_nested_members() {
    let mut value = crate::json!({ "items": [{ "state": "before" }] });
    value.get_mut("items").and_then(Value::as_array_mut).and_then(|items| items.first_mut()).and_then(Value::as_object_mut).and_then(|item| item.get_mut("state")).expect("nested state").clone_from(&Value::String("after".to_string()));
    assert_eq!(to_string(&value), r#"{"items":[{"state":"after"}]}"#);
}
//#endregion 🔖️JsonMacro

/// 🔬️ Differential (single-key object — see the module's own note above on key-order
/// ambiguity): our bridge's bytes agree with the framework's existing `DslValue ->
/// serde_json::Value` path (`🌱️value/🦀️.rs`'s `impl From<DslValue> for
/// serde_json::Value`), the oracle every framework-internal caller still speaks.
#[test]
fn to_json_string_bytes_match_the_serde_json_bridge() {
    let value = DslValue::object([("nested".to_string(), DslValue::Array(vec![DslValue::uint(1), DslValue::float(2.5)]))]);
    let mine = to_json_string(&value);
    let theirs = serde_json::to_string(&serde_json::Value::from(value)).unwrap();
    assert_eq!(mine, theirs);
}
//#endregion 🔖️ToFromValueBridge
