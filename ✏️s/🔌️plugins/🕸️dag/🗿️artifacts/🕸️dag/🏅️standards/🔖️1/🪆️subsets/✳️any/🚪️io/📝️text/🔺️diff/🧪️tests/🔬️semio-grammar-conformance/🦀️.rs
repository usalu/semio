use crate::standards::v1::subsets::any::io::text::diff::*;

#[semio_framework_async_macros::async_test]
async fn component_grammar_semio_is_grammar_dialect() {
    let g = ::semio_framework_dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
    assert_eq!(g.dialect, ::semio_framework_dsl::SemioDialect::Grammar);
    assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
    let _ = COMPONENT_GRAMMAR_PATH;
}
