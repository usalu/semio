
use super::*;

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    let input = b"a,b\n1,2\n";
    let output = oracle_apply_mutation(input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
    assert_eq!(output, input);
}

#[test]
fn insert_and_remove_record_are_inverse_on_a_real_shaped_grid() {
    let input = b"id,name\n1,Alpha\n2,Beta\n";
    let inserted = oracle_apply_mutation(input, &spec("insert-record", Json::Object(vec![("index".to_string(), Json::Number(1.0)), ("fields".to_string(), Json::Array(vec![Json::String("9".to_string()), Json::String("Neu".to_string())]))]))).unwrap();
    assert_eq!(read_grid(&inserted).unwrap(), vec![vec!["id".to_string(), "name".to_string()], vec!["9".to_string(), "Neu".to_string()], vec!["1".to_string(), "Alpha".to_string()], vec!["2".to_string(), "Beta".to_string()]]);

    let removed = oracle_apply_mutation(&inserted, &spec("remove-record", Json::Object(vec![("index".to_string(), Json::Number(1.0))]))).unwrap();
    assert_eq!(read_grid(&removed).unwrap(), read_grid(input).unwrap());
}

#[test]
fn set_field_patches_a_single_cell_and_requotes_when_needed() {
    let input = b"id,note\n1,plain\n";
    let output =
        oracle_apply_mutation(input, &spec("set-field", Json::Object(vec![("recordIndex".to_string(), Json::Number(1.0)), ("fieldIndex".to_string(), Json::Number(1.0)), ("value".to_string(), Json::String("has, comma".to_string()))]))).unwrap();
    let text = String::from_utf8(output.clone()).unwrap();
    assert!(text.contains("\"has, comma\""), "a value containing a comma must come back quoted, got {text:?}");
    assert_eq!(read_grid(&output).unwrap()[1][1], "has, comma");
}

#[test]
fn set_has_header_is_a_true_byte_identity() {
    let input = b"a,b\n1,2\n";
    let output = oracle_apply_mutation(input, &spec("set-has-header", Json::Object(vec![("hasHeader".to_string(), Json::Bool(false))]))).unwrap();
    assert_eq!(output, input);
}

#[test]
fn project_csv_grid_carries_the_caller_tracked_header_flag() {
    let bytes = b"a,b\n1,2\n";
    let with_header = project_csv_grid(bytes, true).unwrap();
    let without_header = project_csv_grid(bytes, false).unwrap();
    assert_eq!(with_header.str("format"), "csv");
    assert_ne!(with_header, without_header, "the two hasHeader states must project differently even though the bytes are identical");
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let input = b"a,b\n1,2\n";
    let result = oracle_apply_mutation(input, &spec("not-a-real-kind", Json::Object(vec![])));
    assert!(result.is_err(), "an unrecognised kind must fail loudly");
}
