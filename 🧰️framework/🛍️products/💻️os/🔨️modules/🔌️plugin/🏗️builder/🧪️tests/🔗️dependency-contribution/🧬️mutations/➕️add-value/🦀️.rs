//#region ➕️AddValue
//! ➕️ One real value addition shared by direct history and contribution planning.
use super::super::{DependencyTestDiff, DependencyTestOp, DependencyTestSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

// 🌱️ `Serialize`/`Deserialize` stay for `MutationKind`'s own (untouched) supertrait bound;
// `ToValue`/`FromValue` are `CompositeMutationKind`'s (see that trait's own doc) — both coexist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[dsl(keyword = "add-value")]
pub struct AddValue { pub delta: i32 }

impl protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "value", kind: "add-value", record: "AddedValue" };
    fn diff(&self, _base: &DependencyTestSnapshot) -> protocol::MutationOutcome<DependencyTestDiff> {
        protocol::MutationOutcome::new(DependencyTestDiff { deltas: vec![self.delta] })
    }
    fn inverse(&self, _base: &DependencyTestSnapshot) -> Vec<DependencyTestOp> {
        if self.delta == i32::MIN {
            vec![DependencyTestOp::AddValue(Self { delta: 1 }), DependencyTestOp::AddValue(Self { delta: i32::MAX })]
        } else {
            vec![DependencyTestOp::AddValue(Self { delta: -self.delta })]
        }
    }
    fn label(&self) -> String { format!("Add {} to value", self.delta) }
    fn target(&self) -> Vec<String> { vec!["value".into()] }
}

impl protocol::CompositeMutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = <Self as protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp>>::SEMANTICS;
    fn plan(&self, _base: &DependencyTestSnapshot, planner: &mut protocol::Planner<DependencyTestSnapshot, DependencyTestOp>) -> Result<(), protocol::PlanError> {
        planner.call(DependencyTestOp::AddValue(self.clone()))
    }
    fn label(&self) -> String { <Self as protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp>>::label(self) }
    fn target(&self) -> Vec<String> { <Self as protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp>>::target(self) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn direct_leaf_contract() { super::super::super::tests::assert_add_value_contract(include_str!("../../🧬️mutations/➕️add-value/🔣️.json")); }
}
//#endregion ➕️AddValue
