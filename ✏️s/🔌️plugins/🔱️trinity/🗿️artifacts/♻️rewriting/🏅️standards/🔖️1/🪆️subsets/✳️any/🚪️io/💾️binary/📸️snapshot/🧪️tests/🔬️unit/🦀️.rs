use crate::standards::v1::subsets::any::io::binary::snapshot::*;
use crate::standards::v1::subsets::any::schema::model::LayoutPoint;
use ::store::os_store::test_support::assert_dsl_pack_equivalence;
use semio_framework_graph_manifest::PropertyValue;

fn sample_rule_state() -> RewritingSnapshot {
    let mut state = crate::standards::v1::subsets::any::schema::test_support::fixture_rule_state();
    state.working_graph.name = "x \"quoted\"\nline".into();
    state.parameter_bindings.insert("count".into(), PropertyValue::Number(3.0));
    state.rule_layout.insert("a".into(), LayoutPoint::from((10.5, -20.25)));
    state
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = sample_rule_state();
    assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
