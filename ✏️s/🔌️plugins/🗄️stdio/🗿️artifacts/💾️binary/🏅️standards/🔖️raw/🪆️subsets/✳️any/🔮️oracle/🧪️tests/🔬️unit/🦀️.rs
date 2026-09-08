
use super::*;

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

fn obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn num_array(values: &[u8]) -> Json {
    Json::Array(values.iter().map(|value| Json::Number(*value as f64)).collect())
}

#[test]
fn no_mutation_is_identity() {
    let input = vec![1, 2, 3, 4, 5];
    let out = oracle_apply_mutation(&input, &spec("no-mutation", obj(vec![]))).unwrap();
    assert_eq!(out, input);
}

#[test]
fn set_snapshot_replaces_the_whole_buffer() {
    let input = vec![1, 2, 3];
    let params = obj(vec![("snapshot", obj(vec![("bytes", num_array(&[9, 9]))]))]);
    let out = oracle_apply_mutation(&input, &spec("set-snapshot", params)).unwrap();
    assert_eq!(out, vec![9, 9]);
}

#[test]
fn splice_replaces_the_named_range() {
    let input = vec![1, 2, 3, 4, 5];
    let params = obj(vec![("offset", Json::Number(1.0)), ("removeLen", Json::Number(2.0)), ("insert", num_array(&[0xAA, 0xBB, 0xCC]))]);
    let out = oracle_apply_mutation(&input, &spec("splice", params)).unwrap();
    assert_eq!(out, vec![1, 0xAA, 0xBB, 0xCC, 4, 5]);
}

#[test]
fn splice_out_of_range_offset_is_rejected_without_corrupting() {
    let input = vec![1, 2, 3];
    let params = obj(vec![("offset", Json::Number(4.0)), ("removeLen", Json::Number(0.0)), ("insert", num_array(&[]))]);
    assert!(oracle_apply_mutation(&input, &spec("splice", params)).is_err());
}

#[test]
fn splice_remove_len_past_the_end_is_rejected() {
    let input = vec![1, 2, 3];
    let params = obj(vec![("offset", Json::Number(2.0)), ("removeLen", Json::Number(5.0)), ("insert", num_array(&[]))]);
    assert!(oracle_apply_mutation(&input, &spec("splice", params)).is_err());
}

#[test]
fn append_bytes_extends_the_end() {
    let input = vec![1, 2];
    let params = obj(vec![("data", num_array(&[3, 4]))]);
    let out = oracle_apply_mutation(&input, &spec("append-bytes", params)).unwrap();
    assert_eq!(out, vec![1, 2, 3, 4]);
}

#[test]
fn append_bytes_to_an_empty_buffer() {
    let input: Vec<u8> = vec![];
    let params = obj(vec![("data", num_array(&[7, 8]))]);
    let out = oracle_apply_mutation(&input, &spec("append-bytes", params)).unwrap();
    assert_eq!(out, vec![7, 8]);
}

#[test]
fn truncate_at_drops_the_tail() {
    let input = vec![1, 2, 3, 4, 5];
    let out = oracle_apply_mutation(&input, &spec("truncate-at", obj(vec![("offset", Json::Number(2.0))]))).unwrap();
    assert_eq!(out, vec![1, 2]);
}

#[test]
fn truncate_at_beyond_the_length_is_a_defined_no_op() {
    let input = vec![1, 2, 3];
    let out = oracle_apply_mutation(&input, &spec("truncate-at", obj(vec![("offset", Json::Number(999.0))]))).unwrap();
    assert_eq!(out, input);
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let input = vec![1, 2, 3];
    assert!(oracle_apply_mutation(&input, &spec("not-a-real-kind", obj(vec![]))).is_err());
}
