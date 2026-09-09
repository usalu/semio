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
#[dsl(keyword = "set-n")]
pub struct SetN {
    pub n: i32,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl crate::os_spr::MutationKind<DemoSnapshot, SeverityMutation> for SetN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "set-n", record: "SetN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(DemoDiff::value(Some(self.n)))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Vec<SeverityMutation> {
        vec![SeverityMutation::RestoreN(RestoreN { n: base.n })]
    }
    fn label(&self) -> String {
        "Set N".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
}
//#endregion ⚙️Behavior

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../../../🧪️tests/🧫️fixture-🚦️severity-🔢️set-n/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
