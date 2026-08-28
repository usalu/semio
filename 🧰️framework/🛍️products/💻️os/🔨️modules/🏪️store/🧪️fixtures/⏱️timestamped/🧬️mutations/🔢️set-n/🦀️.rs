use super::{DemoSnapshot, DemoDiff, TimestampedMutation, RestoreN};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-n")]
pub struct SetN { pub n: i32, pub physical_ms: u64 }

impl crate::os_spr::MutationKind<DemoSnapshot, TimestampedMutation> for SetN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "set-n", record: "SetN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(DemoDiff::value(Some(self.n)))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Vec<TimestampedMutation> {
        vec![TimestampedMutation::RestoreN(RestoreN { n: base.n, physical_ms: 0 })]
    }
    fn label(&self) -> String { "Set N".into() }
    fn target(&self) -> Vec<String> { vec!["n".into()] }
    fn timestamp(&self) -> Option<crate::os_spr::ids::HybridLogicalTimestamp> { Some(crate::os_spr::ids::HybridLogicalTimestamp::new(0, self.physical_ms)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() { super::super::assert_fixture_descriptor::<SetN>(include_str!("🔣️.json")); }
}
