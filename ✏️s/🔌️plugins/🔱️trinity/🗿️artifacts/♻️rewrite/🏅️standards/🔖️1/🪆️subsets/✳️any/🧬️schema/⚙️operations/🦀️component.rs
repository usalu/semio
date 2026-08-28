//! ⚙️ Rewrite mutation store, application, derivation, and behavior laws.

use crate::artifacts::rewrite::mutations::{
    change_parameter_binding, change_rule_layout_point, edit_before_fixture, edit_lhs, edit_rhs, register_rewrite_rule_mutation_descriptors, remove_parameter_binding, remove_rule_layout_point, RewriteRuleMutation,
};
use crate::artifacts::rewrite::{RewriteSnapshot, TrinityRewriteError, REWRITE_RULE_SCHEMA};
use store::{create_document_envelope, ArtifactCommand, ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type RewriteRuleEnvelope = ArtifactEnvelope<RewriteSnapshot, RewriteRuleMutation>;
pub type RewriteRuleStore = ArtifactStore<RewriteSnapshot, RewriteRuleMutation>;

pub fn create_rewrite_rule_envelope(id: &str, state: RewriteSnapshot) -> RewriteRuleEnvelope {
    create_document_envelope(REWRITE_RULE_SCHEMA, id, state, None)
}
//#endregion 🔖️Store

//#region 🔖️SnapshotDiffHelper
/// 🔀️ Diffs two snapshots into a minimal typed semantic mutation set — the seam every command that
/// still computes a whole `next: RewriteSnapshot` (convenient for JSON-body clause editing) uses to
/// emit granular mutations instead of a whole-document replace.
pub fn rewrite_snapshot_mutations(before: &RewriteSnapshot, after: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
    let mut mutations = Vec::new();
    if before.before_fixture_json != after.before_fixture_json {
        mutations.push(edit_before_fixture(after.before_fixture_json.clone()));
    }
    if before.lhs_json != after.lhs_json {
        mutations.push(edit_lhs(after.lhs_json.clone()));
    }
    if before.rhs_json != after.rhs_json {
        mutations.push(edit_rhs(after.rhs_json.clone()));
    }
    for (key, value) in &after.parameter_bindings {
        if before.parameter_bindings.get(key) != Some(value) {
            mutations.push(change_parameter_binding(key.clone(), value.clone()));
        }
    }
    for key in before.parameter_bindings.keys() {
        if !after.parameter_bindings.contains_key(key) {
            mutations.push(remove_parameter_binding(key.clone()));
        }
    }
    for (key, value) in &after.rule_layout {
        if before.rule_layout.get(key) != Some(value) {
            mutations.push(change_rule_layout_point(key.clone(), value.clone()));
        }
    }
    for key in before.rule_layout.keys() {
        if !after.rule_layout.contains_key(key) {
            mutations.push(remove_rule_layout_point(key.clone()));
        }
    }
    mutations
}
//#endregion 🔖️SnapshotDiffHelper

//#region 🔖️BatchHelpers
pub fn apply_rewrite_rule_mutation(snapshot: &mut RewriteSnapshot, mutation: &RewriteRuleMutation) -> protocol::MutationApplyResult<()> {
    let outcome = protocol::Mutation::diff(mutation, snapshot);
    let next = protocol::MutationDiff::apply(outcome.diff(), snapshot)?;
    *snapshot = next;
    Ok(())
}

pub fn inverse_rewrite_rule_mutation(snapshot: &RewriteSnapshot, mutation: &RewriteRuleMutation) -> Vec<RewriteRuleMutation> {
    protocol::Mutation::inverse(mutation, snapshot)
}

/// ▶️ Dispatches a batch of granular mutations (typically from `rewrite_snapshot_mutations`) as one
/// VCS edit.
pub fn dispatch_rewrite_rule_mutations(store: &mut RewriteRuleStore, mutations: Vec<RewriteRuleMutation>) -> Result<(), TrinityRewriteError> {
    if mutations.is_empty() {
        return Ok(());
    }
    store.dispatch(ArtifactCommand::Apply { mutations, description: None }).map_err(TrinityRewriteError::from).map(|_| ())
}
//#endregion 🔖️BatchHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::PropertyValue;
    use crate::artifacts::rewrite::LayoutPoint;
    use ::store::os_store::test_support::{assert_document_pack_round_trip, assert_document_text_round_trip, assert_op_line_round_trip};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;

    use std::collections::BTreeMap;

    fn sample_rule_state() -> RewriteSnapshot {
        let mut parameter_bindings = BTreeMap::new();
        parameter_bindings.insert("label".to_string(), PropertyValue::String("nakagin-core".into()));
        parameter_bindings.insert("count".to_string(), PropertyValue::Number(3.0));
        let mut rule_layout = BTreeMap::new();
        rule_layout.insert("a".to_string(), LayoutPoint::from((10.5, -20.25)));
        RewriteSnapshot {
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
        let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", base.clone()));
        let mut next = base.clone();
        next.lhs_json = "{}".into();
        dispatch_rewrite_rule_mutations(&mut store, rewrite_snapshot_mutations(&base, &next)).unwrap();
        assert_document_text_round_trip(&store);
        assert_document_pack_round_trip(&store);
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
        let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", base.clone()));
        dispatch_rewrite_rule_mutations(&mut store, vec![edit_lhs("{}".into())]).unwrap();
        let edit: &Edit<RewriteRuleMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        ::store::os_store::test_support::assert_command_envelope_round_trip::<RewriteSnapshot, RewriteRuleMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn edit_mutations_inverse_law() {
        let base = sample_rule_state();
        assert_mutation_inverse_law(&base, &edit_before_fixture("{}".into()));
        assert_mutation_inverse_law(&base, &edit_lhs("{}".into()));
        assert_mutation_inverse_law(&base, &edit_rhs("{}".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn parameter_binding_mutations_inverse_law() {
        let base = sample_rule_state();
        assert_mutation_inverse_law(&base, &change_parameter_binding("count".into(), PropertyValue::Number(9.0)));
        assert_mutation_inverse_law(&base, &change_parameter_binding("brandNew".into(), PropertyValue::Bool(true)));
        assert_mutation_inverse_law(&base, &remove_parameter_binding("count".into()));
        assert_mutation_inverse_law(&base, &remove_parameter_binding("ghost".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn rule_layout_point_mutations_inverse_law() {
        let base = sample_rule_state();
        assert_mutation_inverse_law(&base, &change_rule_layout_point("a".into(), LayoutPoint::from((1.0, 2.0))));
        assert_mutation_inverse_law(&base, &change_rule_layout_point("brandNew".into(), LayoutPoint::from((0.0, 0.0))));
        assert_mutation_inverse_law(&base, &remove_rule_layout_point("a".into()));
        assert_mutation_inverse_law(&base, &remove_rule_layout_point("ghost".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_lhs_diff_absorb_law() {
        let base = sample_rule_state();
        let d1 = protocol::Mutation::diff(&edit_lhs("{\"a\":1}".into()), &base).diff().clone();
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = protocol::Mutation::diff(&edit_lhs("{\"a\":2}".into()), &mid).diff().clone();
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_rewrite_rule_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        for kind in <RewriteRuleMutation as protocol::SemanticMutation<RewriteSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<RewriteRuleMutation as protocol::SemanticMutation<RewriteSnapshot>>::kinds().len(), 7);
    }
    //#endregion 🔖️MutationLaws

    // 🧪️OutcomeLaws — no `assert_missing_target_is_error`/`assert_fatal_never_applies` cases apply to
    // this facet: every leaf here is a root-scoped scalar edit (edit-rhs/edit-lhs/edit-before-fixture,
    // no addressable target to be missing) or a key-addressed map upsert/remove
    // (change/remove-parameter-binding, change/remove-rule-layout-point) mapped to the `clear` family
    // (`mutation.no-op` on an already-absent key, per this lane's report) rather than `target-missing`
    // — a missing map key is never Fatal/Error here, matching `remove_parameter_binding("ghost")`/
    // `remove_rule_layout_point("ghost")` in `🔖️MutationLaws` above staying inside the inverse law's
    // "not rejected" bound. `assert_outcome_policy_matrix` is also not yet landed in
    // `📡️spr/🧪️testkit` — TODO(1-D testkit laws pending) once it lands.
}
//#endregion 🧪️Tests
