
use super::*;

#[semio_framework_async_macros::async_test]
async fn parses_and_prints_a_slash_path() {
    let segments = parse_slash_path_text("beams/solid-timber/glulam").await.expect("parse_slash_path_text");
    assert_eq!(segments, vec!["beams".to_string(), "solid-timber".to_string(), "glulam".to_string()]);
    assert_eq!(print_slash_path(&segments).await, "beams/solid-timber/glulam");
}

#[semio_framework_async_macros::async_test]
async fn single_segment_path_round_trips() {
    let segments = parse_slash_path_text("beams").await.expect("parse_slash_path_text");
    assert_eq!(segments, vec!["beams".to_string()]);
    assert_eq!(print_slash_path(&segments).await, "beams");
}

#[semio_framework_async_macros::async_test]
async fn rejects_empty_segments() {
    assert!(parse_slash_path_text("a//b").await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn rejects_more_than_one_token() {
    assert!(parse_slash_path_text("a b").await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn parses_and_prints_a_count_literal() {
    let n = parse_count_text("x24").await.expect("parse_count_text");
    assert_eq!(n, 24);
    assert_eq!(print_count(n).await, "x24");
}

#[semio_framework_async_macros::async_test]
async fn rejects_a_non_count_ident() {
    let err = parse_count_text("beam").await.unwrap_err();
    assert!(err.message.contains("count literal"), "unexpected message: {}", err.message);
}

#[semio_framework_async_macros::async_test]
async fn rejects_bare_x_with_no_digits() {
    assert!(parse_count_text("x").await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn compat_pair_reuses_the_edge_grammar_directly() {
    let value = crate::os_dsl::notation::parse_edge_text("b-l--b-s").expect("parse_edge_text");
    assert_eq!(value.from, EdgeNode { id: "b-l".to_string(), kind: None, port: None });
    let printed = print_edge(&value);
    let link = value.link.expect("link");
    assert!(!link.directed);
    assert_eq!(link.to, EdgeNode { id: "b-s".to_string(), kind: None, port: None });
    assert_eq!(printed, "b-l--b-s");
}

/// @emoji 📖️ The fragment's `.grammar` file must at least parse under `dsl_grammar`'s parser.
#[semio_framework_async_macros::async_test]
async fn grammar_file_is_syntactically_valid() {
    let source = include_str!("../../📖️.grammar.semio");
    let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-catalog.grammar must parse");
    assert_eq!(grammar.id, "family-catalog");
    assert!(grammar.productions.len() > 5, "family-catalog should cover stock, slash-path, compat");
}
