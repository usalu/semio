use super::*;
use crate::LayoutPoint;
use ::store::os_store::test_support::{assert_document_pack_round_trip, assert_document_text_round_trip, assert_op_line_round_trip};
use protocol::os_spr::protocol_laws::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law, assert_outcome_policy_matrix};
use semio_framework_graph::manifest::PropertyValue;

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
async fn op_text_round_trip_edit_lhs() {
    assert_op_line_round_trip(&edit_lhs("{}".into()));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_change_parameter_binding() {
    assert_op_line_round_trip(&change_parameter_binding("count".into(), PropertyValue::Number(4.0)));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_remove_rule_layout_point() {
    assert_op_line_round_trip(&remove_rule_layout_point("a".into()));
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_rewrite_rule_store() {
    let base = sample_rule_state();
    let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", base.clone())).await.expect("valid artifact store");
    let mut next = base.clone();
    next.lhs_json = "{}".into();
    dispatch_rewrite_rule_mutations(&mut store, rewriting_snapshot_mutations(&base, &next)).await.unwrap();
    assert_document_text_round_trip(&store).await;
    assert_document_pack_round_trip(&store).await;
}

#[semio_framework_async_macros::async_test]
async fn op_text_parse_op_errors_on_unknown_keyword() {
    let err = <RewriteRuleMutation as protocol::OpText>::parse_op("bogus xyz").unwrap_err();
    assert!(err.message.contains("unknown mutation line"));
}

/// 🎫️ CW7 command-envelope law: proves `RewriteRuleMutation`'s `Edit` round-trips through
/// `protocol::MutationEnvelope`s.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};

    let base = sample_rule_state();
    let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", base.clone())).await.expect("valid artifact store");
    dispatch_rewrite_rule_mutations(&mut store, vec![edit_lhs("{}".into())]).await.unwrap();
    let edit: &Edit<RewriteRuleMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    ::store::os_store::test_support::assert_command_envelope_round_trip::<RewritingSnapshot, RewriteRuleMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn edit_mutations_inverse_law() {
    let base = sample_rule_state();
    assert_mutation_inverse_law(&base, &edit_before_fixture("{}".into())).await;
    assert_mutation_inverse_law(&base, &edit_lhs("{}".into())).await;
    assert_mutation_inverse_law(&base, &edit_rhs("{}".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn parameter_binding_mutations_inverse_law() {
    let base = sample_rule_state();
    assert_mutation_inverse_law(&base, &change_parameter_binding("count".into(), PropertyValue::Number(9.0))).await;
    assert_mutation_inverse_law(&base, &change_parameter_binding("brandNew".into(), PropertyValue::Bool(true))).await;
    assert_mutation_inverse_law(&base, &remove_parameter_binding("count".into())).await;
    assert_mutation_inverse_law(&base, &remove_parameter_binding("ghost".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn rule_layout_point_mutations_inverse_law() {
    let base = sample_rule_state();
    assert_mutation_inverse_law(&base, &change_rule_layout_point("a".into(), LayoutPoint::from((1.0, 2.0)))).await;
    assert_mutation_inverse_law(&base, &change_rule_layout_point("brandNew".into(), LayoutPoint::from((0.0, 0.0)))).await;
    assert_mutation_inverse_law(&base, &remove_rule_layout_point("a".into())).await;
    assert_mutation_inverse_law(&base, &remove_rule_layout_point("ghost".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn edit_lhs_diff_absorb_law() {
    let base = sample_rule_state();
    let d1 = protocol::Mutation::diff(&edit_lhs("{\"a\":1}".into()), &base).diff().clone();
    let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
    let d2 = protocol::Mutation::diff(&edit_lhs("{\"a\":2}".into()), &mid).diff().clone();
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_rewrite_rule_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(<RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::kinds().len(), 7);
}
//#endregion 🔖️MutationLaws

// 🧪️OutcomeLaws — no `assert_missing_target_is_error`/`assert_fatal_never_applies` cases apply to
// this facet: every leaf here is a root-scoped scalar edit (edit-rhs/edit-lhs/edit-before-fixture,
// no addressable target to be missing) or a key-addressed map upsert/remove
// (change/remove-parameter-binding, change/remove-rule-layout-point) mapped to the `clear` family
// (`mutation.no-op` on an already-absent key, per this lane's report) rather than `target-missing`
// — a missing map key is never Fatal/Error here, matching `remove_parameter_binding("ghost")`/
// `remove_rule_layout_point("ghost")` in `🔖️MutationLaws` above staying inside the inverse law's
// "not rejected" bound. `assert_outcome_policy_matrix` DOES apply to the two outcomes every leaf
// here can actually produce (a real change, no messages; or `mutation.no-op`, Warning) — both are
// covered below, per representative family (edit, remove).
//#region 🔖️OutcomeLaws
#[semio_framework_async_macros::async_test]
async fn edit_lhs_outcome_obeys_the_policy_matrix() {
    let base = sample_rule_state();
    assert_outcome_policy_matrix(&base, &edit_lhs("{\"a\":1}".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn remove_parameter_binding_outcome_obeys_the_policy_matrix() {
    let base = sample_rule_state();
    assert_outcome_policy_matrix(&base, &remove_parameter_binding("count".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn remove_parameter_binding_no_op_outcome_obeys_the_policy_matrix() {
    let base = sample_rule_state();
    assert_outcome_policy_matrix(&base, &remove_parameter_binding("ghost".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn remove_rule_layout_point_outcome_obeys_the_policy_matrix() {
    let base = sample_rule_state();
    assert_outcome_policy_matrix(&base, &remove_rule_layout_point("a".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn remove_rule_layout_point_no_op_outcome_obeys_the_policy_matrix() {
    let base = sample_rule_state();
    assert_outcome_policy_matrix(&base, &remove_rule_layout_point("ghost".into())).await;
}
//#endregion 🔖️OutcomeLaws
