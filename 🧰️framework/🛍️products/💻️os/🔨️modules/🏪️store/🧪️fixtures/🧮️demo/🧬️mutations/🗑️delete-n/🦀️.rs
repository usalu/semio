use super::{DemoSnapshot, DemoDiff, DemoMutation, RestoreN};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "delete-n")]
pub struct DeleteN {}

impl crate::os_spr::MutationKind<DemoSnapshot, DemoMutation> for DeleteN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "delete", entity: "n", kind: "delete-n", record: "DeletedN" };
    fn diff(&self, base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(base.n.map(|_| DemoDiff::value(None)).unwrap_or_default())
    }
    fn inverse(&self, base: &DemoSnapshot) -> Vec<DemoMutation> {
        if base.n.is_none() { return Vec::new(); }
        vec![DemoMutation::RestoreN(RestoreN { n: base.n })]
    }
    fn label(&self) -> String { "Delete N".into() }
    fn target(&self) -> Vec<String> { vec!["n".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() { super::super::assert_fixture_descriptor::<DeleteN>(include_str!("🔣️.json")); }
}
