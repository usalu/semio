//! ↩️ Inverse for `ChangeMergePolicy` — reads the BASE policy, never the diff: restores whatever
//! policy was active before this mutation ran.

use super::super::{MergePolicyConfigMutation, MergePolicySetting};
use super::mutation::ChangeMergePolicy;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMergePolicy, base: &MergePolicySetting) -> Vec<MergePolicyConfigMutation> {
    vec![MergePolicyConfigMutation::ChangeMergePolicy(ChangeMergePolicy { policy: base.policy })]
}
//#endregion 🔖️Inverse
