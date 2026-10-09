use super::{DemoDiff, DemoSnapshot, TimestampedMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, semio_framework_value_derive::RetireOwned, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "assign-n")]
pub struct AssignN {
    pub n: Option<i32>,
    pub physical_ms: u64,
}

impl crate::os_spr::MutationKind<DemoSnapshot, TimestampedMutation> for AssignN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "assign-n", record: "AssignedN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(DemoDiff::value(self.n))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<TimestampedMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![TimestampedMutation::AssignN(Self { n: base.n, physical_ms: 0 })]
    
    })())
}
    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native("Assign N", "N zuweisen")
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
    fn timestamp(&self) -> Option<crate::os_spr::ids::HybridLogicalTimestamp> {
        Some(crate::os_spr::ids::HybridLogicalTimestamp::new(0, self.physical_ms))
    }
}

#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️fixture-timestamped-assign-n/🦀️.rs"]
mod tests;
