
use super::*;
use crate::LayoutPoint;
use ::store::os_store::test_support::{assert_dsl_pack_equivalence, assert_dsl_round_trip};
use semio_s_artifact_trinity_jack::PropertyValue;
use std::collections::BTreeMap;

fn sample_rule_state() -> RewritingSnapshot {
    let mut parameter_bindings = BTreeMap::new();
    parameter_bindings.insert("label".to_string(), PropertyValue::String("nakagin-core".into()));
    parameter_bindings.insert("count".to_string(), PropertyValue::Number(3.0));
    let mut rule_layout = BTreeMap::new();
    rule_layout.insert("a".to_string(), LayoutPoint::from((10.5, -20.25)));
    RewritingSnapshot {
        before_fixture_json: "{\"schema\":\"trinity.graph\",\"name\":\"x \\\"quoted\\\"\\nline\"}".to_string(),
        lhs_json: r#"{"pattern":{"leftVar":"a","leftKind":"Piece"}}"#.to_string(),
        rhs_json: r#"{"set":[{"var":"a","prop":"label","value":"$label"}]}"#.to_string(),
        parameter_bindings,
        rule_layout,
    }
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_rewrite_rule_state() {
    assert_dsl_round_trip(&sample_rule_state());
}

#[semio_framework_async_macros::async_test]
async fn nakagin_label_core_example_dsl_round_trips() {
    let document = parse_dsl(NAKAGIN_LABEL_CORE_EXAMPLE_TEXT).expect("parse nakagin label-core example");
    assert_dsl_round_trip(&document);
    assert_dsl_pack_equivalence(&document);
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_state_parse_dsl_errors_on_unknown_keyword() {
    let err = RewritingSnapshot::parse_dsl("bogus line").unwrap_err();
    assert!(err.message.contains("expected"));
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_state_parse_dsl_errors_on_malformed_binding() {
    assert!(RewritingSnapshot::parse_dsl("binding onlykey").is_err());
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_state_parse_dsl_errors_on_malformed_layout() {
    assert!(RewritingSnapshot::parse_dsl("layout a").is_err());
    assert!(RewritingSnapshot::parse_dsl("layout a notanumber 2").is_err());
    assert!(RewritingSnapshot::parse_dsl("layout a 1 notanumber").is_err());
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_state_parse_dsl_valid_binding_and_layout_lines() {
    let mut original = RewritingSnapshot { before_fixture_json: "{}".into(), lhs_json: "{}".into(), rhs_json: "{}".into(), ..Default::default() };
    original.parameter_bindings.insert("label".to_string(), PropertyValue::String("hi".into()));
    original.rule_layout.insert("a".to_string(), LayoutPoint { x: 1.0, y: 2.0 });
    let state = RewritingSnapshot::parse_dsl(&original.print_dsl()).unwrap();
    assert_eq!(state.parameter_bindings.get("label"), Some(&PropertyValue::String("hi".into())));
    assert_eq!(state.rule_layout.get("a"), Some(&LayoutPoint { x: 1.0, y: 2.0 }));
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_state_parse_dsl_errors_on_malformed_quoted_blob() {
    assert!(RewritingSnapshot::parse_dsl("before nope").is_err());
    assert!(RewritingSnapshot::parse_dsl("before \"abc").is_err());
    assert!(RewritingSnapshot::parse_dsl("before \"ok\" trailing").is_err());
    assert!(RewritingSnapshot::parse_dsl(r#"before "a\"#).is_err());
}

#[semio_framework_async_macros::async_test]
async fn quote_blob_round_trips_backslashes_and_quotes() {
    let mut state = sample_rule_state();
    state.before_fixture_json = "a\\b\"c\nd".to_string();
    assert_dsl_round_trip(&state);
    assert_dsl_pack_equivalence(&state);
}
