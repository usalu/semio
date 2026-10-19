use super::{DemoDiff, DemoMutation, DemoSnapshot, AssignN};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_dsl_record_derive::DslRecord, semio_framework_os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[canonical_json(owner=semio_framework_pack_json)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-n")]
pub struct SetN {
    pub n: i32,
}

impl semio_framework_os_spr::MutationKind<DemoSnapshot, DemoMutation> for SetN {
    const SEMANTICS: semio_framework_os_spr::SemanticDescriptor = semio_framework_os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "set-n", record: "SetN" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set N", "N setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
}

impl semio_framework_os_spr::ColdMutationKind<DemoSnapshot, DemoMutation> for SetN {
    fn diff(&self, base: &DemoSnapshot) -> semio_framework_os_spr::MutationOutcome<DemoDiff> {
        if base.n.is_none() {
            return semio_framework_os_spr::MutationOutcome::error("mutation.target-missing", "n was deleted by a concurrent edit", ["n"]);
        }
        semio_framework_os_spr::MutationOutcome::new(DemoDiff::value(Some(self.n)))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        if base.n.is_none() {
            return Vec::new();
        }
        vec![DemoMutation::AssignN(AssignN { n: base.n })]
    
    })())
}
}
