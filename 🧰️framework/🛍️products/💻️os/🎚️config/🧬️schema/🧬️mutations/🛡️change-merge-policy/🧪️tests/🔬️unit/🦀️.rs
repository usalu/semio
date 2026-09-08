
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
