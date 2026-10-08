use super::{DemoDiff, DemoSnapshot, SeverityMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "assign-n")]
pub struct AssignN {
    pub n: Option<i32>,
}

impl crate::os_spr::MutationKind<DemoSnapshot, SeverityMutation> for AssignN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "assign-n", record: "AssignedN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(DemoDiff::value(self.n))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<SeverityMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![SeverityMutation::AssignN(Self { n: base.n })]
    
    })())
}
    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native("Assign N", "N zuweisen")
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
}

#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️fixture-severity-assign-n/🦀️.rs"]
mod tests;
