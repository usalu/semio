//! 🔺️ Diff fragment yielded by `ChangeMergePolicy` — real handcrafted construction from `base`,
//! never apply-then-capture: setting the same policy that is already active is a `mutation.no-op`
//! Warning with an unchanged diff (verb-family rule for `change/set/update`, contract freeze
//! `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`'s fan-out recipe);
//! otherwise the new policy replaces the setting outright.

use super::super::MergePolicySetting;
use super::mutation::ChangeMergePolicy;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMergePolicy, base: &MergePolicySetting) -> protocol::MutationOutcome<MergePolicySetting> {
    if base.policy == payload.policy {
        return protocol::MutationOutcome::new(*base).warn("mutation.no-op", format!("Merge policy is already \"{:?}\".", payload.policy));
    }
    protocol::MutationOutcome::new(MergePolicySetting { policy: payload.policy })
}
//#endregion 🔖️Diff
