//#region 📦️Imports
use super::{DemoDiff, DemoSnapshot, AssignN, SeverityMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
//#endregion 📦️Imports

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, semio_framework_value_derive::RetireOwned, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_os_dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-warning-n")]
pub struct SetWarningN {
    pub n: i32,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl semio_framework_os_spr::MutationKind<DemoSnapshot, SeverityMutation> for SetWarningN {
    const SEMANTICS: semio_framework_os_spr::SemanticDescriptor = semio_framework_os_spr::SemanticDescriptor { verb: "set", entity: "warning-n", kind: "set-warning-n", record: "SetWarningN" };
    fn diff(&self, _base: &DemoSnapshot) -> semio_framework_os_spr::MutationOutcome<DemoDiff> {
        semio_framework_os_spr::MutationOutcome::new(DemoDiff::value(Some(self.n))).warning("mutation.clamped", "n was clamped to a safe range")
    }
    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<SeverityMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![SeverityMutation::AssignN(AssignN { n: base.n })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Warning N", "Warnung N setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
}
//#endregion ⚙️Behavior
