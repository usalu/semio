
use super::*;

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

/// 🔢️ Pins the worked-around `json` 0.12 defect and the fix together: the crate's own
/// `From<f64>` moves these two real fixture coordinates by one ULP, and `library_number` does
/// not. If a later release fixes the conversion, the first half of this test starts failing and
/// the workaround can go.
#[test]
fn the_library_number_conversion_survives_a_round_trip_the_crates_own_does_not() {
    for value in [2.7000102824824506_f64, -8.881784197001252e-16] {
        assert_ne!(json::JsonValue::from(value).as_f64().unwrap().to_bits(), value.to_bits(), "json 0.12's own From<f64> is documented here as lossy for {value:?}");
        assert_eq!(library_number(value).as_f64().unwrap().to_bits(), value.to_bits(), "the workaround has to be exact for {value:?}");
    }
    for value in [0.1_f64, 1.0, 0.0, -0.0, 1e300, 1.0 / 3.0] {
        assert_eq!(library_number(value).as_f64().unwrap().to_bits(), value.to_bits(), "and exact for {value:?} as well");
    }
    assert!(library_number(f64::NAN).is_null(), "a non-finite double is not a JSON number");
}

/// 🔢️ The mirror defect and its fix: reading a real fixture coordinate back out of the crate.
#[test]
fn the_host_number_reading_is_exact_where_the_crates_own_accessor_is_not() {
    let document = json::parse("{\"v\": -1.3283902924697095e-17}").unwrap();
    assert_ne!(document["v"].as_f64().unwrap(), -1.3283902924697095e-17_f64, "json 0.12's own as_f64 is documented here as lossy for this real fixture coordinate");
    assert_eq!(host_number(&document["v"]), -1.3283902924697095e-17_f64, "the workaround has to read it back exactly");
    for text in ["0.1", "1", "-0.0", "1e300", "3.141592653589793", "4503599627370497"] {
        let probe = json::parse(&format!("{{\"v\": {text}}}")).unwrap();
        assert_eq!(host_number(&probe["v"]).to_bits(), text.parse::<f64>().unwrap().to_bits(), "and stay exact for {text}");
    }
}
fn obj(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    let input = br#"{"a":1,"b":2}"#;
    let output = oracle_apply_mutation(input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
    assert_eq!(output, input);
}

#[test]
fn set_member_upserts_and_remove_member_deletes() {
    let input = br#"{"a":1,"nested":{"b":2}}"#;
    let updated = oracle_apply_mutation(input, &spec("set-member", obj(vec![("path", Json::Array(vec![Json::String("nested".into())])), ("key", Json::String("c".into())), ("value", Json::Number(3.0))]))).unwrap();
    let value = read_json(&updated).unwrap();
    assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("c".into())]).and_then(|v| v.as_f64()), Some(3.0));

    let removed = oracle_apply_mutation(&updated, &spec("remove-member", obj(vec![("path", Json::Array(vec![Json::String("nested".into())])), ("key", Json::String("c".into()))]))).unwrap();
    let value = read_json(&removed).unwrap();
    assert!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("c".into())]).is_none());
    assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("b".into())]).and_then(|v| v.as_f64()), Some(2.0));
}

#[test]
fn insert_and_remove_array_element_are_inverse_on_a_real_shaped_array() {
    let input = br#"{"items":[1,2,3]}"#;
    let inserted = oracle_apply_mutation(input, &spec("insert-array-element", obj(vec![("path", Json::Array(vec![Json::String("items".into())])), ("index", Json::Number(1.0)), ("value", Json::Number(99.0))]))).unwrap();
    assert_eq!(project_json_value(&inserted).unwrap(), project_json_value(br#"{"items":[1,99,2,3]}"#).unwrap());

    let removed = oracle_apply_mutation(&inserted, &spec("remove-array-element", obj(vec![("path", Json::Array(vec![Json::String("items".into())])), ("index", Json::Number(1.0))]))).unwrap();
    assert_eq!(project_json_value(&removed).unwrap(), project_json_value(input).unwrap());
}

#[test]
fn set_scalar_replaces_regardless_of_kind_incl_whole_document() {
    let input = br#"{"a":{"b":1}}"#;
    let output = oracle_apply_mutation(input, &spec("set-scalar", obj(vec![("path", Json::Array(vec![Json::String("a".into())])), ("value", Json::String("replaced".into()))]))).unwrap();
    assert_eq!(project_json_value(&output).unwrap(), project_json_value(br#"{"a":"replaced"}"#).unwrap());

    let whole = oracle_apply_mutation(input, &spec("set-scalar", obj(vec![("path", Json::Array(vec![])), ("value", Json::Bool(true))]))).unwrap();
    assert_eq!(project_json_value(&whole).unwrap(), project_json_value(b"true").unwrap());
}

#[test]
fn set_snapshot_replaces_the_whole_document() {
    let input = br#"{"old":true}"#;
    let output = oracle_apply_mutation(input, &spec("set-snapshot", obj(vec![("value", obj(vec![("fresh", Json::Array(vec![Json::Number(1.0), Json::Number(2.0), Json::String("x".into())]))]))]))).unwrap();
    assert_eq!(project_json_value(&output).unwrap(), project_json_value(br#"{"fresh":[1,2,"x"]}"#).unwrap());
}

/// 🔤️ Where order-insensitivity actually comes from. The projection is a faithful record of what
/// was parsed, so it PRESERVES member order — json-rust's `Object` is insertion-ordered, contrary
/// to an earlier assumption here. RFC 8259 §4 declares member order insignificant, and what
/// discharges that is the case's `ordered-json-v1` comparison profile, which ignores key order at
/// compare time. Array order is significant per §5 and is preserved by both.
///
/// The original form of this test asserted the projection itself normalized member order. It did
/// not, and the test had never run — the crate's whole test target failed to build, so the claim
/// went unchecked. Recorded here so the distinction is not quietly re-lost.
#[test]
fn projection_preserves_member_order_and_array_order() {
    let a = project_json_value(br#"{"a":1,"b":2}"#).unwrap();
    let b = project_json_value(br#"{"b":2,"a":1}"#).unwrap();
    assert_ne!(a, b, "the projection records what was parsed; the comparison profile is what makes member order insignificant");

    let arr_a = project_json_value(br#"[1,2]"#).unwrap();
    let arr_b = project_json_value(br#"[2,1]"#).unwrap();
    assert_ne!(arr_a, arr_b, "array order IS significant per RFC 8259 §5");
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let input = b"{}";
    let result = oracle_apply_mutation(input, &spec("not-a-real-kind", Json::Object(vec![])));
    assert!(result.is_err(), "an unrecognised kind must fail loudly");
}
