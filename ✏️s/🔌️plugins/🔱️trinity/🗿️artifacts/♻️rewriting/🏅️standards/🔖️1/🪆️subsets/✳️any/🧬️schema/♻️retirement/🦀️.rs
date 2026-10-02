//! ♻️ Rewrite-rule document ownership retires bodies, keyed values, and semantic mutations incrementally.

use crate::standards::v1::subsets::any::schema::mutations::{RewriteRuleMutation, RuleLayoutPlacement};
use crate::{LayoutPoint, RewritingSnapshot};
use semio_framework_value::retirement::{OwnedValueRetirementFactory, RetireOwned, RetirementCursor, SharedValueRetirementFactory};

semio_framework_value::artifact_retire_struct!(RewritingSnapshot { before_fixture_json, lhs_json, rhs_json, parameter_bindings, rule_layout });
semio_framework_value::artifact_retire_struct!(LayoutPoint { x, y });
semio_framework_value::artifact_retire_struct!(RuleLayoutPlacement { key, x, y });

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
            Self::DragWorkingNodes(value) => (value.targets, (value.dx, value.dy)).retirement(),
            Self::PatchWorkingNodes(value) => (value.targets, (value.field, value.value)).retirement(),
            Self::DragRuleNodes(value) => (value.targets, (value.dx, value.dy)).retirement(),
            Self::SetRuleLayoutPoints(value) => (value.points, value.cleared).retirement(),
            Self::DeleteWorkingNodes(value) => value.targets.retirement(),
            Self::ConnectWorkingPorts(value) => (value.source, (value.target, value.kind)).retirement(),
            Self::DisconnectWorkingEdges(value) => value.targets.retirement(),
        }
    }
}

/// 🗃️ Installs concrete cursor authorities for every document root and retained mutation.
pub fn document_store_owners() -> store::DocumentStoreOwners<RewritingSnapshot, RewriteRuleMutation> {
    store::DocumentStoreOwners::new(
        std::sync::Arc::new(SharedValueRetirementFactory::<RewritingSnapshot>::default()),
        std::sync::Arc::new(OwnedValueRetirementFactory::<RewritingSnapshot>::default()),
        std::sync::Arc::new(OwnedValueRetirementFactory::<RewriteRuleMutation>::default()),
        Box::new(store::ArtifactStoreCursorDisposer::<RewritingSnapshot, RewriteRuleMutation>::new()),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️document-retirement/🦀️.rs"]
mod tests;
