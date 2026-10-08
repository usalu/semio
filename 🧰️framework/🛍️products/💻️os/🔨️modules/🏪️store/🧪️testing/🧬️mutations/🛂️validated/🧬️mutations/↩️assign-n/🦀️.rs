//#region 📦️Imports
use super::{DemoDiff, DemoSnapshot, ValidatedMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
//#endregion 📦️Imports

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "assign-n")]
pub struct AssignN {
    pub n: Option<i32>,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl crate::os_spr::MutationKind<DemoSnapshot, ValidatedMutation> for AssignN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "assign", entity: "n", kind: "assign-n", record: "AssignedN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(DemoDiff::value(self.n))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<ValidatedMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ValidatedMutation::AssignN(Self { n: base.n })]
    
    })())
}
    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native("Assign N", "N zuweisen")
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
}
//#endregion ⚙️Behavior

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️fixture-validated-assign-n/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
