//#region 📦️Imports
use super::{DemoSnapshot, DemoDiff, ValidatedMutation};
use serde::{Deserialize, Serialize};
//#endregion 📦️Imports

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "restore-n")]
pub struct RestoreN { pub n: Option<i32> }
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl crate::os_spr::MutationKind<DemoSnapshot, ValidatedMutation> for RestoreN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "restore", entity: "n", kind: "restore-n", record: "RestoredN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(DemoDiff::value(self.n))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Vec<ValidatedMutation> {
        vec![ValidatedMutation::RestoreN(Self { n: base.n })]
    }
    fn label(&self) -> String { "Restore N".into() }
    fn target(&self) -> Vec<String> { vec!["n".into()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() { super::super::assert_fixture_descriptor::<RestoreN>(include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
