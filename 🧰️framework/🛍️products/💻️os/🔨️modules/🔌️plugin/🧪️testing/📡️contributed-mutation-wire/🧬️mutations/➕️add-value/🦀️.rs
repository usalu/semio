//#region ➕️ContributedWireAddValue
//! ➕️ Direct value addition for contributed mutation wire planning.

use crate::contributed_mutation_wire::{WireTestDiff, WireTestSnapshot};
use crate::contributed_mutation_wire::mutations::WireTestMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

// 🌱️ `Serialize`/`Deserialize` stay for `MutationKind`'s own (untouched) supertrait bound below;
// `ToValue`/`FromValue` are the newer `CompositeMutationKind` supertrait bound (see that trait's
// own doc) — this fixture implements both traits, so both derive pairs coexist here.
impl protocol::ColdCompositeMutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
    fn plan(&self, _base: &WireTestSnapshot, planner: &mut protocol::Planner<WireTestSnapshot, WireTestMutation>) -> Result<(), protocol::PlanError> {
        planner.call(WireTestMutation::AddValue(self.clone()))
    }
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct AddValue {
    pub delta: i32,
}

impl protocol::MutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "value", kind: "add-value", record: "AddedValue" };





    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add {} to value", self.delta), &format!("{} zu Wert hinzufügen", self.delta))
    }

    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

impl protocol::ColdMutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
    fn diff(&self, _base: &WireTestSnapshot) -> protocol::MutationOutcome<WireTestDiff> {
        protocol::MutationOutcome::new(WireTestDiff { deltas: vec![self.delta] })
    }
    fn inverse(&self, _base: &WireTestSnapshot) -> Result<Vec<WireTestMutation>, semio_framework_value::ValueError> {
        Ok((|| {
            if self.delta == i32::MIN {
                vec![WireTestMutation::AddValue(Self { delta: 1 }), WireTestMutation::AddValue(Self { delta: i32::MAX })]
            } else {
                vec![WireTestMutation::AddValue(Self { delta: -self.delta })]
            }
        })())
    }
}

impl protocol::CompositeMutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::SEMANTICS;



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::label(self)
    }

    fn target(&self) -> Vec<String> {
        <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::target(self)
    }
}

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../🧪️tests/➕️contributed-mutation-wire-add-value-unit/🦀️.rs"]
pub mod tests;
//#endregion ➕️ContributedWireAddValue

#[cfg(test)]
#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
mod sum_law_tests;
