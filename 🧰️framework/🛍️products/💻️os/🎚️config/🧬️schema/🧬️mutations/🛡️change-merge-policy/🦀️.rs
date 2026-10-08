//! 🛡️ `ChangeMergePolicy` is the authoritative direct leaf for the OS-wide conflict policy.

use super::MergePolicyConfigMutation;
use protocol::{Mutation, MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Schema
/// 🛡️ `os.config.merge-policy` — the authority-local policy choice.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct MergePolicySetting {
    pub policy: protocol::MergePolicy,
}

/// 🪪️ The schema id for the merge-policy config facet.
pub const MERGE_POLICY_CONFIG_SCHEMA: &str = "os.config.merge-policy";

/// 🔺️ Sparse diff of [`MergePolicySetting`]: the absolute new policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct MergePolicyDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub policy: Option<protocol::MergePolicy>,
}

impl MutationDiff<MergePolicySetting> for MergePolicyDiff {
    fn apply(&self, base: &MergePolicySetting, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<MergePolicySetting> {
        Ok(MergePolicySetting { policy: self.policy.unwrap_or(base.policy) })
    }

    fn absorb(&mut self, other: Self) {
        if other.policy.is_some() {
            self.policy = other.policy;
        }
    }
}

impl protocol::DiffAlgebra<MergePolicySetting> for MergePolicyDiff {
    fn inverse(&self, base: &MergePolicySetting) -> Self {
        Self { policy: self.policy.map(|_| base.policy) }
    }

    fn is_empty(&self) -> bool {
        self.policy.is_none()
    }
}
//#endregion 🔖️Schema

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

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change merge policy to \"{:?}\"", self.policy), &format!("Zusammenführungsrichtlinie auf \"{:?}\" ändern", self.policy))
    }

    fn target(&self) -> Vec<String> {
        vec!["merge-policy".to_string()]
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🪪️tightens-the-authority-to-vigilant/🦀️.rs"]
mod tests_tightens_the_authority_to_vigilant;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
