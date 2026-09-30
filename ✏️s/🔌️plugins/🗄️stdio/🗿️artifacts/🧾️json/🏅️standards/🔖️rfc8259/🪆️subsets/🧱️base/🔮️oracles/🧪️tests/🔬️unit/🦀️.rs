
use super::*;

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

/// 🔢️ A number wire value reaches the reference as its verbatim lexeme: the two real fixture coordinates `json`
/// 0.12's own `From<f64>` moves by one ULP arrive exactly, because no `f64` stands between the wire and the parser.
#[test]
fn a_number_lexeme_reaches_the_reference_exactly() {
    for lexeme in ["2.7000102824824506", "-8.881784197001252e-16", "0.1", "9007199254740991"] {
        let parsed = library_from_wire(&number(lexeme)).expect("a number lexeme parses");
        assert_eq!(parsed.dump(), json::parse(lexeme).unwrap().dump(), "the lexeme {lexeme} must reach the reference unchanged");
    }
    assert!(library_from_wire(&obj(vec![("kind", Json::String("decimal".into()))])).is_err(), "an unknown JsonValue kind is refused");
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

/// 🧭️ A `path` wire value of object keys.
fn keys(names: &[&str]) -> Json {
    Json::Array(names.iter().map(|name| obj(vec![("kind", Json::String("key".into())), ("value", Json::String((*name).into()))])).collect())
}

/// 🔢️ A `JsonValue` number wire value.
fn number(lexeme: &str) -> Json {
    obj(vec![("kind", Json::String("number".into())), ("lexeme", Json::String(lexeme.into()))])
}

#[test]
fn set_member_upserts_and_remove_member_deletes() {
    let input = br#"{"a":1,"nested":{"b":2}}"#;
    let updated = oracle_apply_mutation(input, &spec("set-member", obj(vec![("path", keys(&["nested"])), ("key", Json::String("c".into())), ("value", number("3"))]))).unwrap();
    let value = read_json(&updated).unwrap();
    assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("c".into())]).and_then(|v| v.as_f64()), Some(3.0));

    let removed = oracle_apply_mutation(&updated, &spec("remove-member", obj(vec![("path", keys(&["nested"])), ("key", Json::String("c".into()))]))).unwrap();
    let value = read_json(&removed).unwrap();
    assert!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("c".into())]).is_none());
    assert_eq!(resolve(&value, &[PathSeg::Key("nested".into()), PathSeg::Key("b".into())]).and_then(|v| v.as_f64()), Some(2.0));
}

#[test]
fn insert_and_remove_array_element_are_inverse_on_a_real_shaped_array() {
    let input = br#"{"items":[1,2,3]}"#;
    let inserted = oracle_apply_mutation(input, &spec("insert-array-element", obj(vec![("path", keys(&["items"])), ("index", Json::Number(1.0)), ("value", number("99"))]))).unwrap();
    assert_eq!(project_json_value(&inserted).unwrap(), project_json_value(br#"{"items":[1,99,2,3]}"#).unwrap());

    let removed = oracle_apply_mutation(&inserted, &spec("remove-array-element", obj(vec![("path", keys(&["items"])), ("index", Json::Number(1.0))]))).unwrap();
    assert_eq!(project_json_value(&removed).unwrap(), project_json_value(input).unwrap());
}

#[test]
fn set_scalar_replaces_regardless_of_kind_incl_whole_document() {
    let input = br#"{"a":{"b":1}}"#;
    let output = oracle_apply_mutation(input, &spec("set-scalar", obj(vec![("path", keys(&["a"])), ("value", obj(vec![("kind", Json::String("string".into())), ("value", Json::String("replaced".into()))]))]))).unwrap();
    assert_eq!(project_json_value(&output).unwrap(), project_json_value(br#"{"a":"replaced"}"#).unwrap());

    let whole = oracle_apply_mutation(input, &spec("set-scalar", obj(vec![("path", keys(&[])), ("value", obj(vec![("kind", Json::String("bool".into())), ("value", Json::Bool(true))]))]))).unwrap();
    assert_eq!(project_json_value(&whole).unwrap(), project_json_value(b"true").unwrap());
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
