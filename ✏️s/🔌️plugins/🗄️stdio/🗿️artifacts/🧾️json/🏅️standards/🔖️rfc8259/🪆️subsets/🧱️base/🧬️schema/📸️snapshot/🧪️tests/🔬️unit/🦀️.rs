
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
    JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
}

#[semio_framework_async_macros::async_test]
async fn parses_all_scalar_kinds() {
    assert_eq!(parse_json_text("null").unwrap(), JsonValue::Null);
    assert_eq!(parse_json_text("true").unwrap(), JsonValue::Bool { value: true });
    assert_eq!(parse_json_text("false").unwrap(), JsonValue::Bool { value: false });
    assert_eq!(parse_json_text("\"hi\"").unwrap(), JsonValue::String { value: "hi".into() });
    assert_eq!(parse_json_text("42").unwrap(), JsonValue::Number { lexeme: "42".into() });
}

#[semio_framework_async_macros::async_test]
async fn preserves_number_lexeme_verbatim() {
    for lexeme in ["0", "-0", "3.140", "1e10", "1E+10", "-1.5e-3", "9007199254740993", "100000000000000000000000000000"] {
        let value = parse_json_text(lexeme).unwrap();
        assert_eq!(value, JsonValue::Number { lexeme: lexeme.into() });
        assert_eq!(write_json_text(&value), lexeme);
    }
}

#[semio_framework_async_macros::async_test]
async fn rejects_leading_zero_number() {
    assert!(parse_json_text("01").is_err());
}

#[semio_framework_async_macros::async_test]
async fn preserves_object_member_insertion_order() {
    let value = parse_json_text(r#"{"z": 1, "a": 2, "m": 3}"#).unwrap();
    match &value {
        JsonValue::Object { members } => {
            let keys: Vec<&str> = members.iter().map(|m| m.key.as_str()).collect();
            assert_eq!(keys, vec!["z", "a", "m"]);
        }
        _ => panic!("expected object"),
    }
    assert_eq!(write_json_text(&value), r#"{"z":1,"a":2,"m":3}"#);
}

#[semio_framework_async_macros::async_test]
async fn decodes_string_escapes_incl_surrogate_pair() {
    let value = parse_json_text(r#""a\tb\nc\"\\ A 😀""#).unwrap();
    assert_eq!(value, JsonValue::String { value: "a\tb\nc\"\\ A 😀".into() });
}

#[semio_framework_async_macros::async_test]
async fn nested_structure_round_trips() {
    let text = r#"{"name":"semio","count":42,"ratio":3.5,"active":true,"missing":null,"tags":["a","b","c"],"nested":{"deep":{"deeper":[1,2,3]}}}"#;
    let value = parse_json_text(text).unwrap();
    assert_eq!(write_json_text(&value), text);
    let pretty = write_json_pretty(&value);
    let reparsed = parse_json_text(&pretty).unwrap();
    assert_eq!(reparsed, value);
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = JsonSnapshot::default();
    assert_eq!(snapshot.schema, STDIO_JSON_DOCUMENT_SCHEMA);
    assert_eq!(snapshot.value, JsonValue::Null);
}

#[semio_framework_async_macros::async_test]
async fn snapshot_dsl_and_pack_round_trip() {
    let snapshot = JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: obj(vec![("a", JsonValue::Number { lexeme: "1".into() }), ("b", JsonValue::Array { items: vec![JsonValue::Bool { value: true }, JsonValue::Null] })]) };
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed, snapshot);
    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snapshot);
}

// 🦑 Dissolved out of the former `⚙️engine`'s own test region (ticket
// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). `empty_snapshot_matches_schema`'s
// former assertion (`empty_json_snapshot().schema == STDIO_JSON_DOCUMENT_SCHEMA`) already
// survives via `empty_snapshot_matches_schema` above (same fact, `JsonSnapshot::default()`).

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = empty_json_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.schema, snap.schema);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

#[semio_framework_async_macros::async_test]
async fn nontrivial_nested_value_round_trip() {
    let snap = demo_json_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.value, snap.value);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded.value, snap.value);
}
