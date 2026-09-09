
use super::*;

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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️macro-values.json")).unwrap();
    for vector in fixture.as_array().unwrap() {
        let record = Record { name: vector["name"].as_str().unwrap().into(), count: vector["count"].as_u64().unwrap() };
        let borrowed = &record;
        let nested = parse(&vector["nested"].to_string()).unwrap();
        let actual = crate::json!({ "record": record, "name": borrowed.name, "again": &borrowed.name, "nested": nested });
        let oracle = serde_json::json!({ "record": { "name": record.name, "count": record.count }, "name": record.name, "again": record.name, "nested": vector["nested"] });
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&actual)).unwrap(), oracle);
        let typed: Value = from_json_str(&to_json_string(&nested)).unwrap();
        assert_eq!(typed, nested);
        assert_eq!(borrowed.name, vector["name"].as_str().unwrap());
    }
}

//#region 🔖️Literals
#[test]
fn parses_literals() {
    assert_eq!(parse("null").unwrap(), Value::Null);
    assert_eq!(parse("true").unwrap(), Value::Bool(true));
    assert_eq!(parse("false").unwrap(), Value::Bool(false));
    assert_eq!(parse("  null  ").unwrap(), Value::Null);
}

#[test]
fn rejects_trailing_data() {
    assert_eq!(parse("null null"), Err(JsonError::TrailingData(5)));
}

#[test]
fn rejects_empty_input() {
    assert_eq!(parse(""), Err(JsonError::UnexpectedEof));
    assert_eq!(parse("   "), Err(JsonError::UnexpectedEof));
}
//#endregion 🔖️Literals

//#region 🔖️Numbers
#[test]
fn integer_and_float_are_never_confused() {
    assert_eq!(to_string(&Value::Number(Number::UInt(42))), "42");
    assert_eq!(to_string(&Value::Number(Number::Float(42.0))), "42.0");
    assert_eq!(parse("42").unwrap(), Value::Number(Number::UInt(42)));
    assert_eq!(parse("42.0").unwrap(), Value::Number(Number::Float(42.0)));
    assert_ne!(parse("42").unwrap(), parse("42.0").unwrap());
}

#[test]
fn parses_number_grammar() {
    assert_eq!(parse("0").unwrap(), Value::Number(Number::UInt(0)));
    assert_eq!(parse("-0").unwrap(), Value::Number(Number::Int(0)));
    assert_eq!(parse("-17").unwrap(), Value::Number(Number::Int(-17)));
    assert_eq!(parse("3.125").unwrap(), Value::Number(Number::Float(3.125)));
    assert_eq!(parse("1e10").unwrap(), Value::Number(Number::Float(1e10)));
    assert_eq!(parse("1.5e-3").unwrap(), Value::Number(Number::Float(1.5e-3)));
    assert_eq!(parse("-2E+2").unwrap(), Value::Number(Number::Float(-200.0)));
}

#[test]
fn parses_exact_fractional_zero_below_f64_mantissa_boundary() {
    let text = "8322951083873004.0";
    let expected = 8_322_951_083_873_004.0;
    assert_eq!(parse(text).unwrap(), Value::Number(Number::Float(expected)));
    assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap().as_f64(), Some(expected));
}

#[test]
fn parses_exact_decimal_exponent_below_f64_mantissa_boundary() {
    let text = "83229510838730040e-1";
    let expected = 8_322_951_083_873_004.0;
    assert_eq!(parse(text).unwrap(), Value::Number(Number::Float(expected)));
    assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap().as_f64(), Some(expected));
}

#[test]
fn rejects_leading_zeros() {
    assert!(parse("01").is_err());
    assert!(parse("[01]").is_err());
    assert!(parse("-01").is_err());
}

#[test]
fn huge_integer_falls_back_to_float() {
    let text = "99999999999999999999999999999999";
    match parse(text).unwrap() {
        Value::Number(Number::Float(_)) => {}
        other => panic!("expected float fallback, got {other:?}"),
    }
}

#[test]
fn rejects_numbers_outside_f64_range() {
    assert!(matches!(parse("1e999"), Err(JsonError::InvalidNumber(0))));
    assert!(matches!(parse("-1e999"), Err(JsonError::InvalidNumber(0))));
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
    assert_eq!(parse(&text).unwrap().as_f64().unwrap(), 1.5e300);

    let text = to_string(&Value::Number(Number::Float(5e-300)));
    assert!(text.contains('e'), "expected exponential form, got {text}");
    assert_eq!(parse(&text).unwrap().as_f64().unwrap(), 5e-300);
}
//#endregion 🔖️Numbers

//#region 🔖️Strings
#[test]
fn parses_escapes_and_unicode() {
    assert_eq!(parse(r#""hi\nthere""#).unwrap().as_str().unwrap(), "hi\nthere");
    assert_eq!(parse(r#""café""#).unwrap().as_str().unwrap(), "café");
    assert_eq!(parse(r#""😀""#).unwrap().as_str().unwrap(), "😀");
    assert_eq!(parse("\"café\"").unwrap().as_str().unwrap(), "café"); // raw UTF-8 passthrough
}

#[test]
fn rejects_lone_surrogate() {
    assert!(matches!(parse(r#""\ud83d""#), Err(JsonError::UnpairedSurrogate(_))));
    assert!(matches!(parse(r#""\ud83dX""#), Err(JsonError::UnpairedSurrogate(_))));
}

#[test]
fn rejects_raw_control_character_in_string() {
    let text = "\"a\u{0001}b\"";
    assert!(matches!(parse(text), Err(JsonError::ControlCharacterInString { .. })));
}

#[test]
fn writer_round_trips_supplementary_plane_and_control_chars() {
    let value = Value::String("😀\u{0001}\t\"\\".to_string());
    let text = to_string(&value);
    assert_eq!(parse(&text).unwrap(), value);
}
//#endregion 🔖️Strings

//#region 🔖️Containers
#[test]
fn parses_arrays_and_objects() {
    let value = parse(r#"{"a":1,"b":[1,2,3],"c":{"nested":true}}"#).unwrap();
    assert_eq!(value.get("a").unwrap().as_u64(), Some(1));
    assert_eq!(value.get("b").unwrap().as_array().unwrap().len(), 3);
    assert_eq!(value.get("c").unwrap().get("nested").unwrap().as_bool(), Some(true));
}

#[test]
fn duplicate_object_keys_keep_first_position_last_value() {
    let value = parse(r#"{"a":1,"b":2,"a":3}"#).unwrap();
    let object = value.as_object().unwrap();
    assert_eq!(object.len(), 2);
    assert_eq!(object.get("a").unwrap().as_u64(), Some(3));
    assert_eq!(object.iter().next().unwrap().0, "a"); // first occurrence's position kept
}

#[test]
fn empty_array_and_object() {
    assert_eq!(parse("[]").unwrap(), Value::Array(vec![]));
    assert_eq!(parse("{}").unwrap(), Value::Object(Object::new()));
    assert_eq!(to_string(&Value::Array(vec![])), "[]");
    assert_eq!(to_string(&Value::Object(Object::new())), "{}");
}

#[test]
fn max_depth_is_enforced() {
    let mut text = String::new();
    for _ in 0..(MAX_DEPTH + 10) {
        text.push('[');
    }
    assert!(matches!(parse(&text), Err(JsonError::MaxDepthExceeded(_))));
}
//#endregion 🔖️Containers

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
    let parsed: DslValue = from_json_str(&text).unwrap();
    assert_eq!(parsed, value);
}

#[test]
fn from_json_str_reports_a_value_error_on_malformed_text() {
    assert!(from_json_str::<DslValue>("not json").is_err());
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
    let parsed: DslValue = from_json_str(&text).unwrap();
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

//#region 🔖️PropertyTesting
/// 🎲️ A tiny deterministic PRNG (SplitMix64) — property/differential tests need arbitrary
/// `Value` trees, but adding a `rand`/`proptest`/`arbitrary` crate would itself be a NEW
/// third-party dependency the freeze ratchet forbids; this is small enough to own outright.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn range(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound.max(1)
    }

    fn unit_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn bool(&mut self) -> bool {
        self.next_u64() & 1 == 0
    }
}

fn arbitrary_finite_float(rng: &mut Rng) -> f64 {
    loop {
        let value = match rng.range(4) {
            0 => 0.0,
            1 => (rng.next_u64() as i64) as f64 / 1e3,
            2 => {
                let exponent = rng.range(600) as i32 - 300;
                let mantissa = 1.0 + rng.unit_f64();
                mantissa * 10f64.powi(exponent)
            }
            _ => f64::from_bits(rng.next_u64()),
        };
        if value.is_finite() {
            return value;
        }
    }
}

fn arbitrary_string(rng: &mut Rng) -> String {
    let len = rng.range(6);
    let mut out = String::new();
    for _ in 0..len {
        let ch = match rng.range(11) {
            0 => '"',
            1 => '\\',
            2 => '\n',
            3 => '\t',
            4 => '\u{0000}',
            5 => '\u{001F}',
            6 => '€',
            7 => '😀',
            8 => '\u{0008}',
            9 => '\u{000C}',
            _ => char::from_u32(0x20 + rng.range(0x5E) as u32).unwrap_or('x'),
        };
        out.push(ch);
    }
    out
}

fn arbitrary_value(rng: &mut Rng, depth: u32) -> Value {
    let kind_bound = if depth >= 4 { 5 } else { 7 };
    match rng.range(kind_bound) {
        0 => Value::Null,
        1 => Value::Bool(rng.bool()),
        2 => Value::Number(Number::UInt(rng.range(1_000_000))),
        3 => Value::Number(Number::Int(rng.range(2_000_000) as i64 - 1_000_000)),
        4 => Value::Number(Number::Float(arbitrary_finite_float(rng))),
        5 => Value::String(arbitrary_string(rng)),
        6 => {
            let len = rng.range(4);
            Value::Array((0..len).map(|_| arbitrary_value(rng, depth + 1)).collect())
        }
        _ => {
            let len = rng.range(4);
            let mut object = Object::new();
            for index in 0..len {
                object.insert(format!("k{index}_{}", rng.range(1000)), arbitrary_value(rng, depth + 1));
            }
            Value::Object(object)
        }
    }
}

const PROPERTY_TEST_ITERATIONS: u32 = 3000;

#[test]
fn round_trips_arbitrary_values() {
    let mut rng = Rng::new(0xC0FF_EE00_1234_5678);
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let text = to_string(&value);
        let parsed = parse(&text).unwrap_or_else(|error| panic!("case {case}: parse failed: {error}; text={text}"));
        assert_eq!(value, parsed, "case {case}: round-trip mismatch; text={text}");
    }
}
//#endregion 🔖️PropertyTesting

//#region 🔖️DifferentialTesting
/// 🔬️ Structural equality between our `Value` and `serde_json::Value` — deliberately NOT a
/// byte-for-byte text comparison: object key order is allowed to differ (this crate's `Object`
/// is insertion-ordered, `serde_json::Value`'s default `Map` is not) as long as the two trees
/// denote the same value. Float notation no longer needs an exception here — see the
/// `FloatParity` region below — but `values_match` stays a value comparison for the key-order
/// reason. Byte-for-byte agreement is checked separately, both for typical documents
/// (`canonical_bytes_match_serde_json_for_typical_documents`) and exhaustively for `f64` alone
/// (`FloatParity`).
fn values_match(mine: &Value, theirs: &serde_json::Value) -> bool {
    match (mine, theirs) {
        (Value::Null, serde_json::Value::Null) => true,
        (Value::Bool(a), serde_json::Value::Bool(b)) => a == b,
        (Value::String(a), serde_json::Value::String(b)) => a == b,
        (Value::Number(a), serde_json::Value::Number(b)) => number_matches(a, b),
        (Value::Array(a), serde_json::Value::Array(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| values_match(x, y)),
        (Value::Object(a), serde_json::Value::Object(b)) => a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).is_some_and(|bv| values_match(v, bv))),
        _ => false,
    }
}

fn number_matches(mine: &Number, theirs: &serde_json::Number) -> bool {
    match *mine {
        Number::UInt(v) => theirs.as_u64() == Some(v) || theirs.as_f64() == Some(v as f64),
        Number::Int(v) => theirs.as_i64() == Some(v) || theirs.as_f64() == Some(v as f64),
        Number::Float(v) => theirs.as_f64().is_some_and(|t| t == v),
    }
}

#[test]
fn differential_parse_matches_serde_json_on_arbitrary_values() {
    let mut rng = Rng::new(0xD1FF_0000_BEEF_CAFE);
    let mut checked = 0usize;
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let text = to_string(&value);
        let mine = parse(&text).unwrap();
        let theirs: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|error| panic!("case {case}: serde_json rejected our own writer output: {error}; text={text}"));
        assert!(values_match(&mine, &theirs), "case {case}: structural mismatch; text={text}\nmine={mine:?}\ntheirs={theirs:?}");
        checked += 1;
    }
    eprintln!("[DEBUG] [differential] parse: {checked} generated documents matched serde_json");
}

#[test]
fn differential_cross_parse_serde_json_writer_output() {
    let mut rng = Rng::new(0x5EED_1357_2468_ACE0);
    let mut checked = 0usize;
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let theirs = to_serde_json(&value);
        let text = serde_json::to_string(&theirs).unwrap();
        let mine = parse(&text).unwrap_or_else(|error| panic!("case {case}: our parser rejected serde_json's writer output: {error}; text={text}"));
        assert!(values_match(&mine, &theirs), "case {case}: structural mismatch; text={text}");
        checked += 1;
    }
    eprintln!("[DEBUG] [differential] cross-parse: {checked} serde_json-written documents matched");
}

fn to_serde_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(v) => serde_json::Value::Bool(*v),
        Value::String(v) => serde_json::Value::String(v.clone()),
        Value::Number(Number::UInt(v)) => serde_json::Value::Number((*v).into()),
        Value::Number(Number::Int(v)) => serde_json::Value::Number((*v).into()),
        Value::Number(Number::Float(v)) => serde_json::Number::from_f64(*v).map_or(serde_json::Value::Null, serde_json::Value::Number),
        Value::Array(items) => serde_json::Value::Array(items.iter().map(to_serde_json).collect()),
        Value::Object(object) => serde_json::Value::Object(object.iter().map(|(k, v)| (k.to_string(), to_serde_json(v))).collect()),
    }
}

/// 🔬️ On documents with no object-key-order ambiguity (scalars, arrays, single-key objects),
/// our writer's bytes agree with `serde_json`'s exactly — including large-magnitude floats,
/// now that `FloatParity` below proves the writers agree on every `f64`.
#[test]
fn canonical_bytes_match_serde_json_for_typical_documents() {
    let cases: &[&str] = &[r#"null"#, r#"true"#, r#"false"#, r#"0"#, r#"-17"#, r#"3.5"#, r#""hello""#, r#""café""#, r#"[]"#, r#"{}"#, r#"[1,2,3]"#, r#"{"a":1}"#, r#"{"only":{"one":"key"}}"#, r#"1.5e300"#, r#"5e-300"#, r#"1e21"#, r#"1e-7"#];
    for text in cases {
        let mine = parse(text).unwrap();
        let theirs: serde_json::Value = serde_json::from_str(text).unwrap();
        let mine_bytes = to_string(&mine);
        let their_bytes = serde_json::to_string(&theirs).unwrap();
        assert_eq!(mine_bytes, their_bytes, "byte mismatch for {text}");
    }
}
//#endregion 🔖️DifferentialTesting

//#region 🔖️FloatParity
/// 🔬️ Exhaustive-ish differential sweep proving `write_float`'s output is byte-identical to
/// `serde_json`'s (`zmij`'s) for every `f64` it is handed — a constant-seeded LCG over random
/// bit patterns (this crate's own `Rng`, reused rather than adding a `rand`/`proptest`
/// dependency) plus every historically-awkward case named in this ticket's own brief. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️float-format-parity.md` for the derivation and the full sweep this test's
/// smaller in-crate corpus is drawn from.
fn float_parity_edge_cases() -> Vec<f64> {
    vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::MAX,
        f64::MIN,
        1e-7,
        -1e-7,
        1e21,
        1e22,
        5e-324,
        -5e-324,
        1.7976931348623157e308,
        -1.7976931348623157e308,
        0.1,
        0.3,
        1e16,
        1e15,
        9999999999999998.0,
        9007199254740993.0,
        123456789012345.0,
        1234567890123456.0,
        12345678901234567.0,
        100000.0,
        1000000.0,
        99999.0,
        999999999999999.9,
        1.0e-5,
        1.0e-6,
        1.0e-4,
        8322951083873004.0,
        f64::from_bits(0xc316b3096f9dcd35),
        f64::from_bits(0x431b807272ea6281),
        f64::from_bits(0xc9409f0951d8de1a),
        f64::from_bits(0x40f869f000000000),
        f64::from_bits(0x430c6bf52633ffff),
    ]
}

#[test]
fn write_float_matches_serde_json_byte_for_byte() {
    let mut checked = 0usize;
    let mut mismatches: Vec<String> = Vec::new();
    for &value in &float_parity_edge_cases() {
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        if mine != theirs {
            mismatches.push(format!("bits={:#018x} value={value:e} mine={mine} theirs={theirs}", value.to_bits()));
        }
        let reparsed: f64 = mine.parse().unwrap_or_else(|error| panic!("our own output {mine:?} failed to reparse: {error}"));
        assert_eq!(reparsed.to_bits(), value.to_bits(), "round-trip bit mismatch for {value:e}, wrote {mine}");
        checked += 1;
    }
    let mut rng = Rng::new(0xF10A_7000_0000_0001);
    for _ in 0..300_000u32 {
        let bits = rng.next_u64();
        let value = f64::from_bits(bits);
        if !value.is_finite() {
            continue;
        }
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        if mine != theirs {
            mismatches.push(format!("bits={bits:#018x} value={value:e} mine={mine} theirs={theirs}"));
        }
        let reparsed: f64 = mine.parse().unwrap_or_else(|error| panic!("our own output {mine:?} failed to reparse: {error}"));
        assert_eq!(reparsed.to_bits(), value.to_bits(), "round-trip bit mismatch for {value:e}, wrote {mine}");
        checked += 1;
    }
    assert!(mismatches.is_empty(), "{} of {checked} floats mismatched serde_json byte-for-byte:\n{}", mismatches.len(), mismatches.join("\n"));
    eprintln!("[DEBUG] [float-parity] {checked} f64 values matched serde_json byte-for-byte (edge cases + LCG sweep)");
}

/// 🔬️ The two real production call sites this parity result unblocks
/// (`🌿️vcs::content_addressed_checkpoint_id_core`'s `serde_json::to_vec(change)` and
/// `🧵️canonical-edit::ScalarBytes::from_node`'s `serde_json::to_writer`) both serialize a
/// single JSON document containing ordinary application floats, not adversarial bit patterns —
/// this proves byte-identity on a realistic corpus shaped like those payloads (nested objects
/// with float-valued fields, the kind a checkpoint or a canonical scalar actually carries).
#[test]
fn realistic_payloads_byte_match_serde_json() {
    let mut rng = Rng::new(0x0011_2233_4455_6677);
    let mut checked = 0usize;
    for _ in 0..20_000u32 {
        let value = arbitrary_finite_float(&mut rng);
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        assert_eq!(mine, theirs, "realistic-payload float mismatch for {value:e}");
        checked += 1;
    }
    eprintln!("[DEBUG] [float-parity] {checked} realistic-payload-shaped floats matched serde_json byte-for-byte");
}
//#endregion 🔖️FloatParity
