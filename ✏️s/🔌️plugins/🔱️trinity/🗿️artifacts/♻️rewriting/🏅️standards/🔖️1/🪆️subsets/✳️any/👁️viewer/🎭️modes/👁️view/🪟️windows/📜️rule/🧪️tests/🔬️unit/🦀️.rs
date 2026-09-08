
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_text_window_kind() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn rule_text_embeds_lhs_and_rhs() {
    let state = RewritingSnapshot { before_fixture_json: "{}".into(), lhs_json: r#"{"whereClause":"a.name = 'b'"}"#.into(), rhs_json: r#"{"parameters":[]}"#.into(), parameter_bindings: Default::default(), rule_layout: Default::default() };
    let text = rule_text(&state);
    assert!(text.contains("whereClause"));
    assert!(text.contains("parameters"));
}
