
use super::*;

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    let input = b"a\tb\r\n1\t2\r\n";
    let output = oracle_apply_mutation(input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
    assert_eq!(output, input);
}

#[test]
fn insert_and_remove_row_are_inverse_on_a_real_shaped_grid() {
    let input = b"id\tname\n1\tAlpha\n2\tBeta\n";
    let inserted = oracle_apply_mutation(input, &spec("insert-row", Json::Object(vec![("index".to_string(), Json::Number(1.0)), ("row".to_string(), Json::Array(vec![Json::String("9".to_string()), Json::String("Neu".to_string())]))]))).unwrap();
    assert_eq!(read_tsv(&inserted).unwrap().records, vec![vec!["id".to_string(), "name".to_string()], vec!["9".to_string(), "Neu".to_string()], vec!["1".to_string(), "Alpha".to_string()], vec!["2".to_string(), "Beta".to_string()]]);

    let removed = oracle_apply_mutation(&inserted, &spec("remove-row", Json::Object(vec![("index".to_string(), Json::Number(1.0))]))).unwrap();
    assert_eq!(read_tsv(&removed).unwrap().records, read_tsv(input).unwrap().records);
}

#[test]
fn set_cell_patches_a_single_cell_with_no_quoting_invented() {
    let input = b"id\tnote\n1\tplain\n";
    let output =
        oracle_apply_mutation(input, &spec("set-cell", Json::Object(vec![("rowIndex".to_string(), Json::Number(1.0)), ("fieldIndex".to_string(), Json::Number(1.0)), ("value".to_string(), Json::String("has, comma and \"quote\"".to_string()))])))
            .unwrap();
    let text = String::from_utf8(output.clone()).unwrap();
    assert!(!text.contains('"') || text.contains("\"quote\""), "the value must survive verbatim, no quoting invented, got {text:?}");
    assert_eq!(read_tsv(&output).unwrap().records[1][1], "has, comma and \"quote\"");
}

#[test]
fn set_trailing_newline_genuinely_adds_and_removes_bytes() {
    let input = b"a\tb\n1\t2\n";
    let stripped = oracle_apply_mutation(input, &spec("set-trailing-newline", Json::Object(vec![("trailingNewline".to_string(), Json::Bool(false))]))).unwrap();
    assert_eq!(stripped, b"a\tb\n1\t2");

    let restored = oracle_apply_mutation(&stripped, &spec("set-trailing-newline", Json::Object(vec![("trailingNewline".to_string(), Json::Bool(true))]))).unwrap();
    assert_eq!(restored, input);
}

#[test]
fn set_line_ending_genuinely_rewrites_every_terminator() {
    let input = b"a\tb\n1\t2\n";
    let output = oracle_apply_mutation(input, &spec("set-line-ending", Json::Object(vec![("lineEnding".to_string(), Json::String("crlf".to_string()))]))).unwrap();
    assert_eq!(output, b"a\tb\r\n1\t2\r\n");
}

#[test]
fn project_tsv_grid_carries_trailing_newline_and_line_terminator_live() {
    let lf = project_tsv_grid(b"a\tb\n1\t2\n").unwrap();
    let crlf_no_trailing = project_tsv_grid(b"a\tb\r\n1\t2").unwrap();
    assert_eq!(lf.str("format"), "tsv");
    assert_ne!(lf, crlf_no_trailing, "trailingNewline/lineTerminator must actually participate in the projection");
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let input = b"a\tb\n1\t2\n";
    let result = oracle_apply_mutation(input, &spec("not-a-real-kind", Json::Object(vec![])));
    assert!(result.is_err(), "an unrecognised kind must fail loudly");
}
