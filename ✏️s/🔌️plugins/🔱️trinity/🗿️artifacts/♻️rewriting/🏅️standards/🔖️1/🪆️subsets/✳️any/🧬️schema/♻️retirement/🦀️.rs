//! ♻️ Rewrite-rule document ownership retires bodies, keyed values, and semantic mutations incrementally.

use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::{LayoutPoint, RewritingSnapshot};
use store::retirement::{OwnedValueRetirementFactory, RetireOwned, RetirementCursor, SharedValueRetirementFactory};

store::artifact_retire_struct!(RewritingSnapshot { before_fixture_json, lhs_json, rhs_json, parameter_bindings, rule_layout });
store::artifact_retire_struct!(LayoutPoint { x, y });

impl RetireOwned for RewriteRuleMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::EditBeforeFixture(value) => value.new_before_fixture_json.retirement(),
            Self::EditLhs(value) => value.new_lhs_json.retirement(),
            Self::EditRhs(value) => value.new_rhs_json.retirement(),
            Self::ChangeParameterBinding(value) => (value.key, value.new_value).retirement(),
            Self::RemoveParameterBinding(value) => value.key.retirement(),
            Self::ChangeRuleLayoutPoint(value) => (value.key, value.new_point).retirement(),
            Self::RemoveRuleLayoutPoint(value) => value.key.retirement(),
        }
    }
}

/// 🗃️ Installs concrete cursor authorities for every document root and retained mutation.
pub fn document_store_owners() -> store::MemberStoreOwners<RewritingSnapshot, RewriteRuleMutation> {
    store::MemberStoreOwners::new(
        std::sync::Arc::new(SharedValueRetirementFactory::<RewritingSnapshot>::default()),
        std::sync::Arc::new(OwnedValueRetirementFactory::<RewritingSnapshot>::default()),
        std::sync::Arc::new(OwnedValueRetirementFactory::<RewriteRuleMutation>::default()),
        Box::new(store::ArtifactStoreCursorDisposer::<RewritingSnapshot, RewriteRuleMutation>::new()),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️document-retirement/🦀️.rs"]
mod tests;
