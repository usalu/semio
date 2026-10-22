//! 🛡️ `os.config.merge-policy` schema: the authority-local policy setting and its sparse diff.

extern crate semio_framework_os_kernel as protocol;

use protocol::{ColdDiffAlgebra, ColdMutationDiff, MutationDiff};
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



}

impl ColdMutationDiff<MergePolicySetting> for MergePolicyDiff {
    fn absorb(&mut self, other: Self) {
        if other.policy.is_some() {
            self.policy = other.policy;
        }
    }

    fn apply(&self, base: &mut MergePolicySetting, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
                if let Some(value) = self.policy { base.policy = value; }
        Ok(())
    }
}

impl protocol::DiffAlgebra<MergePolicySetting> for MergePolicyDiff {


    fn is_empty(&self) -> bool {
        self.policy.is_none()
    }
}

impl protocol::ColdDiffAlgebra<MergePolicySetting> for MergePolicyDiff {
    fn inverse(&self, base: &MergePolicySetting) -> Self {
        Self { policy: self.policy.map(|_| base.policy) }
    }
}
//#endregion 🔖️Schema
