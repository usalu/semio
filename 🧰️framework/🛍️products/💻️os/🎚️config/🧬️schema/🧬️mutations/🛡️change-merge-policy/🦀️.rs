//! 🛡️ `ChangeMergePolicy` is the authoritative direct leaf for the OS-wide conflict policy.

use super::MergePolicyConfigMutation;
use protocol::{ColdMutation, MutationKind, MutationOutcome, SemanticDescriptor};
pub use semio_framework_os_config_merge_policy::{MergePolicyDiff, MergePolicySetting, MERGE_POLICY_CONFIG_SCHEMA};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🛡️ Replaces the active OS-wide merge policy.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeMergePolicy {
    pub policy: protocol::MergePolicy,
}

/// 🏗️ Wraps a change-merge-policy payload in the merge-policy dispatch enum.
pub fn change_merge_policy(policy: protocol::MergePolicy) -> MergePolicyConfigMutation {
    MergePolicyConfigMutation::ChangeMergePolicy(ChangeMergePolicy { policy })
}

impl MutationKind<MergePolicySetting, MergePolicyConfigMutation> for ChangeMergePolicy {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "merge-policy", kind: "change-merge-policy", record: "Change" };





    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change merge policy to \"{:?}\"", self.policy), &format!("Zusammenführungsrichtlinie auf \"{:?}\" ändern", self.policy))
    }

    fn target(&self) -> Vec<String> {
        vec!["merge-policy".to_string()]
    }
}

impl protocol::ColdMutationKind<MergePolicySetting, MergePolicyConfigMutation> for ChangeMergePolicy {
    fn diff(&self, base: &MergePolicySetting) -> MutationOutcome<MergePolicyDiff> {
        if base.policy == self.policy {
            return MutationOutcome::new(MergePolicyDiff::default()).warning("mutation.no-op", format!("Merge policy is already \"{:?}\".", self.policy));
        }
        MutationOutcome::new(MergePolicyDiff { policy: Some(self.policy) })
    }
    fn inverse(&self, base: &MergePolicySetting) -> Result<Vec<MergePolicyConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![MergePolicyConfigMutation::ChangeMergePolicy(ChangeMergePolicy { policy: base.policy })]
    
    })())
}
}

/// ↩️ Computes the mutation's inverse steps from the pre-mutation setting.
pub fn inverse_merge_policy_config_mutation(snapshot: &MergePolicySetting, mutation: &MergePolicyConfigMutation) -> Result<Vec<MergePolicyConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(snapshot)?

    })
}







/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_merge_policy_config_mutation_steps(mutation: &MergePolicyConfigMutation, base: &MergePolicySetting) -> Result<Vec<MergePolicyConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(base)?

    })
}
//#endregion 🔖️Mutation
