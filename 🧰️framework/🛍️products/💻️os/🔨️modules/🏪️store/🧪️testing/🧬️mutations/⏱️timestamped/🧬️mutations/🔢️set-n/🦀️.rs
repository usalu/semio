use super::{DemoDiff, DemoSnapshot, AssignN, TimestampedMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, semio_framework_value_derive::RetireOwned, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_os_dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-n")]
pub struct SetN {
    pub n: i32,
    pub physical_ms: u64,
}

impl semio_framework_os_spr::MutationKind<DemoSnapshot, TimestampedMutation> for SetN {
    const SEMANTICS: semio_framework_os_spr::SemanticDescriptor = semio_framework_os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "set-n", record: "SetN" };
    fn diff(&self, _base: &DemoSnapshot) -> semio_framework_os_spr::MutationOutcome<DemoDiff> {
        semio_framework_os_spr::MutationOutcome::new(DemoDiff::value(Some(self.n)))
    }
    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<TimestampedMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![TimestampedMutation::AssignN(AssignN { n: base.n, physical_ms: 0 })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set N", "N setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
    fn timestamp(&self) -> Option<semio_framework_os_spr::ids::HybridLogicalTimestamp> {
        Some(semio_framework_os_spr::ids::HybridLogicalTimestamp::new(0, self.physical_ms))
    }
}
