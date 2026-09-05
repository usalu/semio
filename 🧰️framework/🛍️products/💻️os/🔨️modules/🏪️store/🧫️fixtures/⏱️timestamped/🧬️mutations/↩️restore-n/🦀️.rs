use super::{DemoDiff, DemoSnapshot, TimestampedMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "restore-n")]
pub struct RestoreN {
    pub n: Option<i32>,
    pub physical_ms: u64,
}

impl crate::os_spr::MutationKind<DemoSnapshot, TimestampedMutation> for RestoreN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "restore", entity: "n", kind: "restore-n", record: "RestoredN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(DemoDiff::value(self.n))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Vec<TimestampedMutation> {
        vec![TimestampedMutation::RestoreN(Self { n: base.n, physical_ms: 0 })]
    }
    fn label(&self) -> String {
        "Restore N".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
    fn timestamp(&self) -> Option<crate::os_spr::ids::HybridLogicalTimestamp> {
        Some(crate::os_spr::ids::HybridLogicalTimestamp::new(0, self.physical_ms))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() {
        super::super::assert_fixture_descriptor::<RestoreN>(include_str!("🔣️.json"));
    }
}
