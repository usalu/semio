//#region ➕️ContributedWireAddValue
//! ➕️ Direct value addition for contributed mutation wire planning.

use super::super::{WireTestDiff, WireTestMutation, WireTestSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

// 🌱️ `Serialize`/`Deserialize` stay for `MutationKind`'s own (untouched) supertrait bound below;
// `ToValue`/`FromValue` are the newer `CompositeMutationKind` supertrait bound (see that trait's
// own doc) — this fixture implements both traits, so both derive pairs coexist here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct AddValue {
    pub(crate) delta: i32,
}

impl protocol::MutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "value", kind: "add-value", record: "AddedValue" };

    fn diff(&self, _base: &WireTestSnapshot) -> protocol::MutationOutcome<WireTestDiff> {
        protocol::MutationOutcome::new(WireTestDiff { deltas: vec![self.delta] })
    }

    fn inverse(&self, _base: &WireTestSnapshot) -> Vec<WireTestMutation> {
        if self.delta == i32::MIN {
            vec![WireTestMutation::AddValue(Self { delta: 1 }), WireTestMutation::AddValue(Self { delta: i32::MAX })]
        } else {
            vec![WireTestMutation::AddValue(Self { delta: -self.delta })]
        }
    }

    fn label(&self) -> String {
        format!("Add {} to value", self.delta)
    }

    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

impl protocol::CompositeMutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::SEMANTICS;

    fn plan(&self, _base: &WireTestSnapshot, planner: &mut protocol::Planner<WireTestSnapshot, WireTestMutation>) -> Result<(), protocol::PlanError> {
        planner.call(WireTestMutation::AddValue(self.clone()))
    }

    fn label(&self) -> String {
        <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::label(self)
    }

    fn target(&self) -> Vec<String> {
        <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::target(self)
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/➕️contributed-mutation-wire-add-value-unit/🦀️.rs"]
mod tests;
//#endregion ➕️ContributedWireAddValue
