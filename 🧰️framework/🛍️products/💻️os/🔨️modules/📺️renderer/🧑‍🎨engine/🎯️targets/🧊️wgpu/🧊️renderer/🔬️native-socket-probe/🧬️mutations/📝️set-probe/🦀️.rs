//! 📝️ Replaces the probe's raw text; its inverse restores the base text.
use crate::native_socket_probe::mutations::NativeSocketProbeMutation;
use crate::native_socket_probe::{NativeSocketProbeDiff, NativeSocketProbeSnapshot};
use store::os_spr::{ColdMutationKind, MutationKind, MutationOutcome, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::semio_os_kernel::os_spr)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetProbe {
    pub value: String,
}

impl MutationKind<NativeSocketProbeSnapshot, NativeSocketProbeMutation> for SetProbe {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "probe", kind: "set-probe", record: "SetProbe" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Set probe", "Probe setzen")
    }
}

impl ColdMutationKind<NativeSocketProbeSnapshot, NativeSocketProbeMutation> for SetProbe {
    fn diff(&self, _base: &NativeSocketProbeSnapshot) -> MutationOutcome<NativeSocketProbeDiff> {
        MutationOutcome::new(NativeSocketProbeDiff(self.value.clone()))
    }
    fn inverse(&self, base: &NativeSocketProbeSnapshot) -> Result<Vec<NativeSocketProbeMutation>, semio_framework_value::ValueError> {
        Ok(vec![Self { value: base.0.clone() }.into()])
    }
}
