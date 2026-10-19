//#region ➕️AddValue
//! ➕️ One real value addition shared by direct history and contribution planning.
use crate::builder_test_hosts::builder_unit_1::dependency_fixture::{DependencyTestDiff, DependencyTestSnapshot};
use crate::builder_test_hosts::builder_unit_1::dependency_fixture::mutations::DependencyTestOp;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

// 🌱️ `Serialize`/`Deserialize` stay for `MutationKind`'s own (untouched) supertrait bound;
// `ToValue`/`FromValue` are `CompositeMutationKind`'s (see that trait's own doc) — both coexist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[dsl(keyword = "add-value")]
pub struct AddValue {
    pub delta: i32,
}

impl protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "value", kind: "add-value", record: "AddedValue" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add {} to value", self.delta), &format!("{} zu Wert hinzufügen", self.delta))
    }
    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

impl protocol::ColdMutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
    fn diff(&self, _base: &DependencyTestSnapshot) -> protocol::MutationOutcome<DependencyTestDiff> {
        protocol::MutationOutcome::new(DependencyTestDiff { deltas: vec![self.delta] })
    }
    fn inverse(&self, _base: &DependencyTestSnapshot) -> Result<Vec<DependencyTestOp>, semio_framework_value::ValueError> {
        Ok((|| {
            if self.delta == i32::MIN {
                vec![DependencyTestOp::AddValue(Self { delta: 1 }), DependencyTestOp::AddValue(Self { delta: i32::MAX })]
            } else {
                vec![DependencyTestOp::AddValue(Self { delta: -self.delta })]
            }
        })())
    }
}

impl protocol::CompositeMutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = <Self as protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp>>::SEMANTICS;

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp>>::target(self)
    }
}

impl protocol::ColdCompositeMutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
    fn plan(&self, _base: &DependencyTestSnapshot, planner: &mut protocol::Planner<DependencyTestSnapshot, DependencyTestOp>) -> Result<(), protocol::PlanError> {
        planner.call(DependencyTestOp::AddValue(self.clone()))
    }
}

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../🧪️tests/➕️dependency-contribution-add-value-unit/🦀️.rs"]
pub mod tests;
//#endregion ➕️AddValue

#[cfg(test)]
#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
mod sum_law_tests;
