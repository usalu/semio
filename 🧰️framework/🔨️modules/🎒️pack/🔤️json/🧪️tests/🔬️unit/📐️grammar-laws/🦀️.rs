/// 📐️ Grammar laws of the pack-json companion: the borrowing macro, the literal, number, string and container grammar and their refusals.

#[test]
fn macro_borrows_records_and_matches_json_vectors() {
    struct Record {
        name: String,
        count: u64,
    }
    impl ToValue for Record {
        fn to_value(&self) -> DslValue {
            DslValue::object([("name".into(), self.name.to_value()), ("count".into(), self.count.to_value())])
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🔣️macro-values.json")).unwrap();
    for vector in fixture.as_array().unwrap() {
        let record = Record { name: vector["name"].as_str().unwrap().into(), count: vector["count"].as_u64().unwrap() };
        let borrowed = &record;
        let nested = parse(&vector["nested"].to_string(), crate::JsonMemberPolicy::Replace).unwrap();
        let actual = crate::json!({ "record": record, "name": borrowed.name, "again": &borrowed.name, "nested": nested });
        let oracle = serde_json::json!({ "record": { "name": record.name, "count": record.count }, "name": record.name, "again": record.name, "nested": vector["nested"] });
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&actual)).unwrap(), oracle);
        let typed: Value = from_json_str(&to_json_string(&nested), crate::JsonMemberPolicy::Replace).unwrap();
        assert_eq!(typed, nested);
        assert_eq!(borrowed.name, vector["name"].as_str().unwrap());
    }
}

//#region 🔖️Literals
#[test]
fn parses_literals() {
    assert_eq!(parse("null", crate::JsonMemberPolicy::Replace).unwrap(), Value::Null);
    assert_eq!(parse("true", crate::JsonMemberPolicy::Replace).unwrap(), Value::Bool(true));
    assert_eq!(parse("false", crate::JsonMemberPolicy::Replace).unwrap(), Value::Bool(false));
    assert_eq!(parse("  null  ", crate::JsonMemberPolicy::Replace).unwrap(), Value::Null);
}

#[test]
fn rejects_trailing_data() {
    assert_eq!(parse("null null", crate::JsonMemberPolicy::Replace), Err(JsonError::TrailingData(5)));
}

#[test]
fn rejects_empty_input() {
    assert_eq!(parse("", crate::JsonMemberPolicy::Replace), Err(JsonError::UnexpectedEof));
    assert_eq!(parse("   ", crate::JsonMemberPolicy::Replace), Err(JsonError::UnexpectedEof));
}
//#endregion 🔖️Literals

//#region 🔖️Numbers
#[test]
fn integer_and_float_are_never_confused() {
    assert_eq!(to_string(&Value::Number(Number::UInt(42))), "42");
    assert_eq!(to_string(&Value::Number(Number::Float(42.0))), "42.0");
    assert_eq!(parse("42", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::UInt(42)));
    assert_eq!(parse("42.0", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(42.0)));
    assert_ne!(parse("42", crate::JsonMemberPolicy::Replace).unwrap(), parse("42.0", crate::JsonMemberPolicy::Replace).unwrap());
}

#[test]
fn parses_number_grammar() {
    assert_eq!(parse("0", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::UInt(0)));
    assert_eq!(parse("-0", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Int(0)));
    assert_eq!(parse("-17", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Int(-17)));
    assert_eq!(parse("3.125", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(3.125)));
    assert_eq!(parse("1e10", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(1e10)));
    assert_eq!(parse("1.5e-3", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(1.5e-3)));
    assert_eq!(parse("-2E+2", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(-200.0)));
}

#[test]
fn parses_exact_fractional_zero_below_f64_mantissa_boundary() {
    let text = "8322951083873004.0";
    let expected = 8_322_951_083_873_004.0;
    assert_eq!(parse(text, crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(expected)));
    assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap().as_f64(), Some(expected));
}

#[test]
fn parses_exact_decimal_exponent_below_f64_mantissa_boundary() {
    let text = "83229510838730040e-1";
    let expected = 8_322_951_083_873_004.0;
    assert_eq!(parse(text, crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(expected)));
    assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap().as_f64(), Some(expected));
}

#[test]
fn rejects_leading_zeros() {
    assert!(parse("01", crate::JsonMemberPolicy::Replace).is_err());
    assert!(parse("[01]", crate::JsonMemberPolicy::Replace).is_err());
    assert!(parse("-01", crate::JsonMemberPolicy::Replace).is_err());
}

#[test]
fn huge_integer_falls_back_to_float() {
    let text = "99999999999999999999999999999999";
    match parse(text, crate::JsonMemberPolicy::Replace).unwrap() {
        Value::Number(Number::Float(_)) => {}
        other => panic!("expected float fallback, got {other:?}"),
    }
}

#[test]
fn rejects_numbers_outside_f64_range() {
    assert!(matches!(parse("1e999", crate::JsonMemberPolicy::Replace), Err(JsonError::InvalidNumber(0))));
    assert!(matches!(parse("-1e999", crate::JsonMemberPolicy::Replace), Err(JsonError::InvalidNumber(0))));
    assert!(serde_json::from_str::<serde_json::Value>("1e999").is_err());
    assert!(serde_json::from_str::<serde_json::Value>("-1e999").is_err());
}

#[test]
fn non_finite_floats_encode_as_null() {
    assert_eq!(to_string(&Value::Number(Number::Float(f64::NAN))), "null");
    assert_eq!(to_string(&Value::Number(Number::Float(f64::INFINITY))), "null");
    assert_eq!(to_string(&Value::Number(Number::Float(f64::NEG_INFINITY))), "null");
}

#[test]
fn large_and_small_magnitudes_use_exponential_notation() {
    let text = to_string(&Value::Number(Number::Float(1.5e300)));
    assert!(text.contains('e'), "expected exponential form, got {text}");
    assert_eq!(parse(&text, crate::JsonMemberPolicy::Replace).unwrap().as_f64().unwrap(), 1.5e300);

    let text = to_string(&Value::Number(Number::Float(5e-300)));
    assert!(text.contains('e'), "expected exponential form, got {text}");
    assert_eq!(parse(&text, crate::JsonMemberPolicy::Replace).unwrap().as_f64().unwrap(), 5e-300);
}
//#endregion 🔖️Numbers

//#region 🔖️Strings
#[test]
fn parses_escapes_and_unicode() {
    assert_eq!(parse(r#""hi\nthere""#, crate::JsonMemberPolicy::Replace).unwrap().as_str().unwrap(), "hi\nthere");
    assert_eq!(parse(r#""café""#, crate::JsonMemberPolicy::Replace).unwrap().as_str().unwrap(), "café");
    assert_eq!(parse(r#""😀""#, crate::JsonMemberPolicy::Replace).unwrap().as_str().unwrap(), "😀");
    assert_eq!(parse("\"café\"", crate::JsonMemberPolicy::Replace).unwrap().as_str().unwrap(), "café");
}

#[test]
fn rejects_lone_surrogate() {
    assert!(matches!(parse(r#""\ud83d""#, crate::JsonMemberPolicy::Replace), Err(JsonError::UnpairedSurrogate(_))));
    assert!(matches!(parse(r#""\ud83dX""#, crate::JsonMemberPolicy::Replace), Err(JsonError::UnpairedSurrogate(_))));
}

#[test]
fn rejects_raw_control_character_in_string() {
    let text = "\"a\u{0001}b\"";
    assert!(matches!(parse(text, crate::JsonMemberPolicy::Replace), Err(JsonError::ControlCharacterInString { .. })));
}

#[test]
fn writer_round_trips_supplementary_plane_and_control_chars() {
    let value = Value::String("😀\u{0001}\t\"\\".to_string());
    let text = to_string(&value);
    assert_eq!(parse(&text, crate::JsonMemberPolicy::Replace).unwrap(), value);
}
//#endregion 🔖️Strings

//#region 🔖️Containers
#[test]
fn parses_arrays_and_objects() {
    let value = parse(r#"{"a":1,"b":[1,2,3],"c":{"nested":true}}"#, crate::JsonMemberPolicy::Replace).unwrap();
    assert_eq!(value.get("a").unwrap().as_u64(), Some(1));
    assert_eq!(value.get("b").unwrap().as_array().unwrap().len(), 3);
    assert_eq!(value.get("c").unwrap().get("nested").unwrap().as_bool(), Some(true));
}

#[test]
fn duplicate_object_keys_keep_first_position_last_value() {
    let value = parse(r#"{"a":1,"b":2,"a":3}"#, crate::JsonMemberPolicy::Replace).unwrap();
    let object = value.as_object().unwrap();
    assert_eq!(object.len(), 2);
    assert_eq!(object.get("a").unwrap().as_u64(), Some(3));
    assert_eq!(object.iter().next().unwrap().0, "a");
}

#[test]
fn empty_array_and_object() {
    assert_eq!(parse("[]", crate::JsonMemberPolicy::Replace).unwrap(), Value::Array(vec![]));
    assert_eq!(parse("{}", crate::JsonMemberPolicy::Replace).unwrap(), Value::Object(Object::new()));
    assert_eq!(to_string(&Value::Array(vec![])), "[]");
    assert_eq!(to_string(&Value::Object(Object::new())), "{}");
}

#[test]
fn max_depth_is_enforced() {
    let mut text = String::new();
    for _ in 0..(MAX_DEPTH + 10) {
        text.push('[');
    }
    assert!(matches!(parse(&text, crate::JsonMemberPolicy::Replace), Err(JsonError::MaxDepthExceeded(_))));
}
//#endregion 🔖️Containers
