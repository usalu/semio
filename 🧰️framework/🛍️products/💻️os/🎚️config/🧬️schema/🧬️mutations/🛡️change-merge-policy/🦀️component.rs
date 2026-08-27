//! 🛡️ `ChangeMergePolicy` is the authoritative direct leaf for the OS-wide conflict policy.

use super::MergePolicyConfigMutation;
use protocol::{Mutation, MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Schema
/// 🛡️ `os.config.merge-policy` — the authority-local policy choice.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MergePolicySetting {
    pub policy: protocol::MergePolicy,
}

/// 🪪️ The schema id for the merge-policy config facet.
pub const MERGE_POLICY_CONFIG_SCHEMA: &str = "os.config.merge-policy";

impl MutationDiff<MergePolicySetting> for MergePolicySetting {
    fn apply(&self, _base: &MergePolicySetting) -> protocol::MutationApplyResult<MergePolicySetting> {
        Ok(*self)
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Schema

//#region 🔖️Mutation
/// 🛡️ Replaces the active OS-wide merge policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMergePolicy {
    pub policy: protocol::MergePolicy,
}

/// 🏗️ Wraps a change-merge-policy payload in the merge-policy dispatch enum.
pub fn change_merge_policy(policy: protocol::MergePolicy) -> MergePolicyConfigMutation {
    MergePolicyConfigMutation::ChangeMergePolicy(ChangeMergePolicy { policy })
}

impl MutationKind<MergePolicySetting, MergePolicyConfigMutation> for ChangeMergePolicy {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "merge-policy", kind: "change-merge-policy", record: "Change" };

    fn diff(&self, base: &MergePolicySetting) -> MutationOutcome<MergePolicySetting> {
        if base.policy == self.policy {
            return MutationOutcome::new(*base).warn("mutation.no-op", format!("Merge policy is already \"{:?}\".", self.policy));
        }
        MutationOutcome::new(MergePolicySetting { policy: self.policy })
    }

    fn inverse(&self, base: &MergePolicySetting) -> Vec<MergePolicyConfigMutation> {
        vec![MergePolicyConfigMutation::ChangeMergePolicy(ChangeMergePolicy { policy: base.policy })]
    }

    fn label(&self) -> String {
        format!("Change merge policy to \"{:?}\"", self.policy)
    }

    fn target(&self) -> Vec<String> {
        vec!["merge-policy".to_string()]
    }
}

/// 🧮️ Applies one merge-policy mutation through its whole-record diff.
pub fn apply_merge_policy_config_mutation(snapshot: &mut MergePolicySetting, mutation: &MergePolicyConfigMutation) -> protocol::MutationApplyResult<()> {
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

/// ↩️ Computes the mutation's inverse steps from the pre-mutation setting.
pub fn inverse_merge_policy_config_mutation(snapshot: &MergePolicySetting, mutation: &MergePolicyConfigMutation) -> Vec<MergePolicyConfigMutation> {
    mutation.inverse(snapshot)
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/tightens-the-authority-to-vigilant/🦀️component.rs"]
mod tests_tightens_the_authority_to_vigilant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_normal() {
        assert_eq!(MergePolicySetting::default(), MergePolicySetting { policy: protocol::MergePolicy::Normal });
    }

    #[test]
    fn label_and_target_name_the_policy_facet() {
        let payload = ChangeMergePolicy { policy: protocol::MergePolicy::Vigilant };
        assert_eq!(MutationKind::<MergePolicySetting, MergePolicyConfigMutation>::label(&payload), "Change merge policy to \"Vigilant\"");
        assert_eq!(MutationKind::<MergePolicySetting, MergePolicyConfigMutation>::target(&payload), vec!["merge-policy".to_string()]);
    }
}
//#endregion 🧪️Tests
