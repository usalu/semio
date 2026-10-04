use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_text_window_kind() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn rule_text_embeds_lhs_and_rhs() {
    let state = RewritingSnapshot { lhs: crate::standards::v1::subsets::any::schema::Lhs { pattern: crate::standards::v1::subsets::any::schema::Pattern { left_var: "a".into(), left_kind: "Piece".into(), ..Default::default() }, where_clause: Some("a.name = 'b'".into()) }, ..Default::default() };
    let text = rule_text(&state);
    assert!(text.contains("whereClause"));
    assert!(text.contains("parameters"));
}
