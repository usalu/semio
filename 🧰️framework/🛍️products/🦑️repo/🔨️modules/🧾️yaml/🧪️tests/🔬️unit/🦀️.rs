use super::*;

#[test]
fn block_mapping_and_sequence_decode() {
    let value = unmarshal("name: semio\npaths:\n  - a\n  - b\nenabled: true\n").unwrap();
    assert_eq!(value["name"], Value::String("semio".to_string()));
    assert_eq!(value["paths"], serde_json::json!(["a", "b"]));
    assert_eq!(value["enabled"], Value::Bool(true));
}

#[test]
fn json_is_accepted_as_a_superset() {
    let value = unmarshal("{\"a\": 1}").unwrap();
    assert_eq!(value["a"], serde_json::json!(1));
}

#[test]
fn encoding_sorts_keys_and_quotes_ambiguous_scalars() {
    let value = serde_json::json!({"b": 1, "a": "x: y"});
    assert_eq!(marshal(&value).unwrap(), "a: \"x: y\"\nb: 1\n");
}

#[test]
fn tabs_are_not_indentation_and_produce_siblings() {
    let value = unmarshal("a:\n\tb: 1\n").unwrap();
    assert_eq!(value["b"], serde_json::json!(1));
}

#[test]
fn round_trip_is_stable() {
    let source = "enabled: true\nname: semio\npaths:\n  - a\n  - b\n";
    let decoded = unmarshal(source).unwrap();
    assert_eq!(marshal(&decoded).unwrap(), source);
}
