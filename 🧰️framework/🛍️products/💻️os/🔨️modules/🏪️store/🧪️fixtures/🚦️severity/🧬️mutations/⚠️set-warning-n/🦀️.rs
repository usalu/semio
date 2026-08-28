//#region 📦️Imports
use super::{DemoSnapshot, DemoDiff, SeverityMutation, RestoreN};
use serde::{Deserialize, Serialize};
//#endregion 📦️Imports

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-warning-n")]
pub struct SetWarningN { pub n: i32 }
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl crate::os_spr::MutationKind<DemoSnapshot, SeverityMutation> for SetWarningN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "warning-n", kind: "set-warning-n", record: "SetWarningN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(DemoDiff::value(Some(self.n))).warn("mutation.clamped", "n was clamped to a safe range")
    }
    fn inverse(&self, base: &DemoSnapshot) -> Vec<SeverityMutation> {
        vec![SeverityMutation::RestoreN(RestoreN { n: base.n })]
    }
    fn label(&self) -> String { "Set Warning N".into() }
    fn target(&self) -> Vec<String> { vec!["n".into()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() { super::super::assert_fixture_descriptor::<SetWarningN>(include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
