use super::{DemoSnapshot, DemoDiff, DemoMutation, RestoreN};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-n")]
pub struct SetN { pub n: i32 }

impl crate::os_spr::MutationKind<DemoSnapshot, DemoMutation> for SetN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "set-n", record: "SetN" };
    fn diff(&self, base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        if base.n.is_none() { return crate::os_spr::MutationOutcome::error("mutation.target-missing", "n was deleted by a concurrent edit", ["n"]); }
        crate::os_spr::MutationOutcome::new(DemoDiff::value(Some(self.n)))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Vec<DemoMutation> {
        if base.n.is_none() { return Vec::new(); }
        vec![DemoMutation::RestoreN(RestoreN { n: base.n })]
    }
    fn label(&self) -> String { "Set N".into() }
    fn target(&self) -> Vec<String> { vec!["n".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() { super::super::assert_fixture_descriptor::<SetN>(include_str!("🔣️.json")); }
}
