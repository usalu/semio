
/// @emoji 📖️ The fragment's `.grammar` file must parse under `dsl_grammar`'s parser.
#[semio_framework_async_macros::async_test]
async fn grammar_file_is_syntactically_valid() {
    let source = include_str!("../../📖️.grammar.semio");
    let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-embed.grammar must parse");
    assert_eq!(grammar.id, "family-embed");
    assert!(grammar.productions.len() > 4, "family-embed should expose fence vocabulary beyond a one-liner");
}
