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

/// 📥️ Decodes the internally tagged merge-policy mutation JSON projection.
pub fn decode_merge_policy_config_mutation_json(text: &str) -> Result<MergePolicyConfigMutation, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📤️ Encodes the merge-policy setting to its canonical camel-case JSON projection.
pub fn encode_merge_policy_setting_json(snapshot: &MergePolicySetting) -> String {
    dsl::os_pack::json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical merge-policy setting JSON projection.
pub fn decode_merge_policy_setting_json(text: &str) -> Result<MergePolicySetting, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies a mutation and returns its diagnostic `(code, severity)` pairs.
pub fn apply_merge_policy_config_mutation_reporting(snapshot: &mut MergePolicySetting, mutation: &MergePolicyConfigMutation) -> Vec<(String, String)> {
    let outcome = mutation.diff(snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_merge_policy_config_mutation_steps(mutation: &MergePolicyConfigMutation, base: &MergePolicySetting) -> Vec<MergePolicyConfigMutation> {
    mutation.inverse(base)
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🪪️tightens-the-authority-to-vigilant/🦀️.rs"]
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
