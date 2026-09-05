//#region 📦️Imports
use super::{DemoDiff, DemoSnapshot, RestoreN, SeverityMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
//#endregion 📦️Imports

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-fatal-n")]
pub struct SetFatalN {
    pub n: i32,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl crate::os_spr::MutationKind<DemoSnapshot, SeverityMutation> for SetFatalN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "fatal-n", kind: "set-fatal-n", record: "SetFatalN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::fatal("mutation.invariant", "n invariant violated", ["n"])
    }
    fn inverse(&self, base: &DemoSnapshot) -> Vec<SeverityMutation> {
        vec![SeverityMutation::RestoreN(RestoreN { n: base.n })]
    }
    fn label(&self) -> String {
        "Set Fatal N".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
}
//#endregion ⚙️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() {
        super::super::assert_fixture_descriptor::<SetFatalN>(include_str!("🔣️.json"));
    }
}
//#endregion 🧪️Tests
