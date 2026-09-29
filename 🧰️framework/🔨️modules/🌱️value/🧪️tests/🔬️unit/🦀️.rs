
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

/// 🧾️ `dsl_value!` equals `serde_json::json!` (third-party oracle) on every literal form — null, booleans, signed, unsigned
/// and fractional numbers, nested arrays and objects, `const` and parenthesized expression keys, trailing commas, borrowed
/// values (`&String`, `&[String]`, `&&str`) — and,
/// unlike `json!` without `preserve_order`, keeps object entries in written order.
#[test]
fn dsl_value_literal_matches_serde_json_and_keeps_written_order() {
    const KEY: &str = "constKey";
    let count = 3u32;
    let names = vec![String::from("a"), String::from("b")];
    let flag = false;
    let literal = crate::dsl_value!({ "zeta": null, "alpha": [1, -2, 2.5, true, false, null, [], {}], KEY: count, ("expr".to_string()): !flag, "nested": { "names": names, "text": "ä€😀" }, });
    let oracle = serde_json::json!({ "zeta": null, "alpha": [1, -2, 2.5, true, false, null, [], {}], KEY: count, ("expr".to_string()): !flag, "nested": { "names": names, "text": "ä€😀" }, });
    assert_eq!(serde_json::Value::from(&literal), oracle);
    let DslValue::Object(entries) = &literal else { panic!("an object literal builds an object") };
    assert_eq!(entries.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["zeta", "alpha", "constKey", "expr", "nested"]);
    assert_eq!(crate::dsl_value!(7u64), DslValue::uint(7));
    assert_eq!(crate::dsl_value!([]), DslValue::Array(Vec::new()));
    assert_eq!(crate::dsl_value!({}), DslValue::Object(Vec::new()));
    assert_eq!(serde_json::Value::from(&crate::dsl_value!([[1, [2]], { "k": [3] }])), serde_json::json!([[1, [2]], { "k": [3] }]));
    let borrowed_text = &names[0];
    let borrowed_list: &[String] = &names;
    let borrowed_str: &&str = &"x";
    assert_eq!(serde_json::Value::from(&crate::dsl_value!({ "t": borrowed_text, "l": borrowed_list, "s": borrowed_str })), serde_json::json!({ "t": borrowed_text, "l": borrowed_list, "s": borrowed_str }));
}

/// 🔢️ `DslValue::json_number` reads an `f64` the way its JSON text reads back — the shared `numbers` vectors of
/// `🧫️fixtures/🔣️json-projection/🔣️.json`, whose `json` column the TypeScript law pins to `JSON.stringify`: every integer, null and
/// in-range case equals `serde_json`'s parse of that text (third-party oracle); past the safe integer range an `f64` stays a float.
#[test]
fn a_json_number_reads_back_as_its_json_text() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️json-projection/🔣️.json")).expect("the json-projection fixture parses");
    let numbers = fixture["numbers"].as_array().expect("numbers");
    assert!(!numbers.is_empty());
    for case in numbers {
        let literal = case["literal"].as_str().expect("literal");
        let value: f64 = literal.parse().expect("an f64 literal");
        let bridged = DslValue::json_number(value);
        let text = serde_json::from_str::<serde_json::Value>(case["json"].as_str().expect("json")).expect("json text parses");
        match case["kind"].as_str().expect("kind") {
            "uint" => assert!(matches!(bridged, DslValue::Number(Number::UInt(_))) && bridged == DslValue::from(&text), "{literal}: {bridged:?}"),
            "int" => assert!(matches!(bridged, DslValue::Number(Number::Int(_))) && bridged == DslValue::from(&text), "{literal}: {bridged:?}"),
            "null" => assert_eq!((bridged, DslValue::from(&text)), (DslValue::Null, DslValue::Null), "{literal}"),
            "float" => assert_eq!(bridged, DslValue::float(value), "{literal}"),
            kind => panic!("unknown kind {kind}"),
        }
    }
}

#[test]
fn edit_through_value_matches_a_serde_json_pointer_edit_and_keeps_the_decode_invariant() {
    #[derive(Debug, PartialEq)]
    struct Placed {
        name: String,
        offset: DslValue,
    }
    impl ToValue for Placed {
        fn to_value(&self) -> DslValue {
            DslValue::object([("name".to_string(), self.name.to_value()), ("offset".to_string(), self.offset.clone())])
        }
    }
    impl FromValue for Placed {
        fn from_value(value: DslValue) -> Result<Self, ValueError> {
            let mut entries = value.into_object()?.into_iter();
            let (Some((_, name)), Some((_, offset))) = (entries.next(), entries.next()) else { return Err(ValueError::new("two fields")) };
            let name = String::from_value(name)?;
            if name.is_empty() {
                return Err(ValueError::new("a placed value is named"));
            }
            Ok(Self { name, offset })
        }
    }
    let mut placed = Placed { name: "saw".into(), offset: DslValue::from(&serde_json::json!({"x": 1, "y": [2, 3]})) };
    let mut oracle = serde_json::Value::from(&placed.to_value());
    *oracle.pointer_mut("/offset/y/1").unwrap() = serde_json::json!(5);
    edit_through_value(&mut placed, &["offset", "y", "1"], ValueEdit::Set(DslValue::from(&serde_json::json!(5)))).unwrap();
    assert_eq!(serde_json::Value::from(&placed.to_value()), oracle);
    let before = serde_json::Value::from(&placed.to_value());
    assert!(edit_through_value(&mut placed, &["name"], ValueEdit::Set(DslValue::from(&serde_json::json!("")))).is_err());
    assert!(edit_through_value(&mut placed, &["missing", "x"], ValueEdit::Set(DslValue::from(&serde_json::json!(1)))).is_err());
    assert_eq!(serde_json::Value::from(&placed.to_value()), before);
}
