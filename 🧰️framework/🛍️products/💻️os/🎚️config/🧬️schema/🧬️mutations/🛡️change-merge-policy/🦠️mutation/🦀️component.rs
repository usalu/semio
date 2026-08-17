//! 🛡️ `ChangeMergePolicy` — the merge-policy config facet's one mutation kind: sets `os.config.
//! merge-policy`'s `policy` field to a new `protocol::MergePolicy` choice.

use super::super::{MergePolicyConfigMutation, MergePolicySetting};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🛡️ Sets the OS-wide merge policy to `policy`, replacing whatever was active before.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMergePolicy {
    pub policy: protocol::MergePolicy,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_merge_policy(policy: protocol::MergePolicy) -> MergePolicyConfigMutation {
    MergePolicyConfigMutation::ChangeMergePolicy(ChangeMergePolicy { policy })
}

impl MutationKind<MergePolicySetting, MergePolicyConfigMutation> for ChangeMergePolicy {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "merge-policy", kind: "change-merge-policy", record: "Change" };

    fn diff(&self, base: &MergePolicySetting) -> MutationOutcome<MergePolicySetting> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &MergePolicySetting) -> Vec<MergePolicyConfigMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change merge policy to \"{:?}\"", self.policy)
    }

    fn target(&self) -> Vec<String> {
        vec!["merge-policy".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_merge_policy_label_names_the_policy() {
        let payload = ChangeMergePolicy { policy: protocol::MergePolicy::Vigilant };
        assert_eq!(MutationKind::<MergePolicySetting, MergePolicyConfigMutation>::label(&payload), "Change merge policy to \"Vigilant\"");
    }

    #[test]
    fn change_merge_policy_target_names_the_facet() {
        let payload = ChangeMergePolicy { policy: protocol::MergePolicy::Normal };
        assert_eq!(MutationKind::<MergePolicySetting, MergePolicyConfigMutation>::target(&payload), vec!["merge-policy".to_string()]);
    }
}
//#endregion 🧪️Tests
