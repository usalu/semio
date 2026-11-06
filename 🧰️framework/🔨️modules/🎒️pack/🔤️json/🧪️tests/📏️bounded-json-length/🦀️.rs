//! 📏️ Portable bounded JSON lengths checked against serde_json.
use semio_framework_pack_json::{bounded_json_len, parse, JsonMemberPolicy};

#[test]
fn bounded_lengths_match_portable_vectors_and_serde() {
    let rows: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("fixture");
    for row in rows.as_array().expect("rows") {
        let text = serde_json::to_string(&row["payload"]).expect("oracle");
        let length = text.len();
        if let Some(expected) = row["expected"].as_u64() { assert_eq!(length, expected as usize); }
        let maximum = row["maximum"].as_u64().expect("maximum") as usize;
        let value = parse(&text, JsonMemberPolicy::Reject).expect("value");
        assert_eq!(bounded_json_len(&value, maximum), (length <= maximum).then_some(length));
        assert_eq!(bounded_json_len(&value, length), Some(length));
        if length > 0 { assert_eq!(bounded_json_len(&value, length - 1), None); }
    }
}

#[test]
fn nesting_limit_matches_parser() {
    use semio_framework_pack_json::{MAX_DEPTH, Value};
    let mut value = Value::Null;
    for _ in 0..MAX_DEPTH { value = Value::Array(vec![value]); }
    let text = semio_framework_pack_json::to_string(&value);
    assert!(parse(&text, JsonMemberPolicy::Reject).is_ok());
    assert_eq!(bounded_json_len(&value, text.len()), Some(text.len()));
    value = Value::Array(vec![value]);
    assert!(parse(&semio_framework_pack_json::to_string(&value), JsonMemberPolicy::Reject).is_err());
    assert_eq!(bounded_json_len(&value, usize::MAX), None);
}
