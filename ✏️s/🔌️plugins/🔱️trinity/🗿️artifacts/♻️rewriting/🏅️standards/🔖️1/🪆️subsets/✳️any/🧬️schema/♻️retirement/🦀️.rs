//! ♻️ Rewrite-rule document ownership retires bodies, keyed values, and semantic mutations incrementally.

use crate::standards::v1::subsets::any::schema::mutations::{RewriteRuleMutation, RuleLayoutPlacement};
use crate::{LayoutPoint, RewritingSnapshot};
use semio_framework_value::retirement::{OwnedValueRetirementFactory, RetireOwned, RetirementCursor, SharedValueRetirementFactory};

semio_framework_value::artifact_retire_struct!(RewritingSnapshot { working_graph, lhs, rhs, parameter_bindings, rule_layout });
semio_framework_value::artifact_retire_struct!(LayoutPoint { x, y });
semio_framework_value::artifact_retire_struct!(RuleLayoutPlacement { key, x, y });

impl RetireOwned for RewriteRuleMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::EditWorkingGraph(value) => value.new_working_graph.retirement(),
            Self::EditLhs(value) => value.new_lhs.retirement(),
            Self::EditRhs(value) => value.new_rhs.retirement(),
            Self::ChangeParameterBinding(value) => (value.key, value.new_value).retirement(),
            Self::RemoveParameterBinding(value) => value.key.retirement(),
            Self::ChangeRuleLayoutPoint(value) => (value.key, value.new_point).retirement(),
            Self::RemoveRuleLayoutPoint(value) => value.key.retirement(),
            Self::DragRuleNodes(value) => (value.targets, (value.dx, value.dy)).retirement(),
            Self::SetRuleLayoutPoints(value) => (value.points, value.cleared).retirement(),
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

use super::{Pattern,Lhs,Rhs,Assignment,ParameterSpec,ParameterKind,Rule};
semio_framework_value::artifact_retire_struct!(Pattern {left_var,left_kind,edge_var,edge_kind,right_var,right_kind});
semio_framework_value::artifact_retire_struct!(Lhs {pattern,where_clause});
semio_framework_value::artifact_retire_struct!(Rhs {create,delete,set,merge,parameters});
semio_framework_value::artifact_retire_struct!(Assignment {var,prop,value});
semio_framework_value::artifact_retire_struct!(ParameterSpec {name,kind,default});
semio_framework_value::artifact_retire_struct!(Rule {name,lhs,rhs});
semio_framework_value::artifact_retire_leaf!(ParameterKind);
