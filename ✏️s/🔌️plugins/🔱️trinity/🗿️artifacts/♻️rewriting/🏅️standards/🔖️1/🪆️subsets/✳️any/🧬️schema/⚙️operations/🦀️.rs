//! ⚙️ Rewriting mutation store, application, derivation, and behavior laws.

use crate::standards::v1::subsets::any::schema::mutations::{
    change_parameter_binding, change_rule_layout_point, edit_before_fixture, edit_lhs, edit_rhs, remove_parameter_binding, remove_rule_layout_point, RewriteRuleMutation,
};
use crate::{RewritingSnapshot, TrinityRewritingError, REWRITE_RULE_SCHEMA};
use store::{create_document_envelope, ArtifactCommand, ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type RewriteRuleEnvelope = ArtifactEnvelope<RewritingSnapshot, RewriteRuleMutation>;
pub type RewriteRuleStore = ArtifactStore<RewritingSnapshot, RewriteRuleMutation>;

pub fn create_rewrite_rule_envelope(id: &str, state: RewritingSnapshot) -> RewriteRuleEnvelope {
    create_document_envelope(REWRITE_RULE_SCHEMA, id, state, None)
}
//#endregion 🔖️Store

//#region 🔖️SnapshotDiffHelper
/// 🔀️ Diffs two snapshots into a minimal typed semantic mutation set — the seam every command that
/// still computes a whole `next: RewritingSnapshot` (convenient for JSON-body clause editing) uses to
/// emit granular mutations instead of a whole-document replace.
pub fn rewriting_snapshot_mutations(before: &RewritingSnapshot, after: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
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
            mutations.push(change_rule_layout_point(key.clone(), *value));
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
pub fn apply_rewrite_rule_mutation(snapshot: &mut RewritingSnapshot, mutation: &RewriteRuleMutation) -> protocol::MutationApplyResult<()> {
    let outcome = protocol::Mutation::diff(mutation, snapshot);
    let next = protocol::MutationDiff::apply(outcome.diff(), snapshot)?;
    *snapshot = next;
    Ok(())
}

pub fn inverse_rewrite_rule_mutation(snapshot: &RewritingSnapshot, mutation: &RewriteRuleMutation) -> Vec<RewriteRuleMutation> {
    protocol::Mutation::inverse(mutation, snapshot)
}

/// ▶️ Dispatches a batch of granular mutations (typically from `rewriting_snapshot_mutations`) as one
/// VCS edit.
pub async fn dispatch_rewrite_rule_mutations(store: &mut RewriteRuleStore, mutations: Vec<RewriteRuleMutation>) -> Result<(), TrinityRewritingError> {
    if mutations.is_empty() {
        return Ok(());
    }
    store.dispatch(ArtifactCommand::Apply { mutations, description: None }).await.map_err(TrinityRewritingError::from).map(|_| ())
}
//#endregion 🔖️BatchHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::register_rewrite_rule_mutation_descriptors;
