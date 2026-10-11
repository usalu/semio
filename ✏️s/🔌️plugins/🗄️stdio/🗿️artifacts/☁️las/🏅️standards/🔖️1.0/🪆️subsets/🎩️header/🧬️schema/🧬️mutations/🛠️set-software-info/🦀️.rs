//! 🛠️ `set-software-info` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! 🛠️ Sets §2.3 Generating Software.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSoftwareInfo {
    pub generating_software: String,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetSoftwareInfo {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "software-info", kind: "set-software-info", record: "SetSoftwareInfo" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { generating_software } = self;
        protocol::MutationOutcome::new(diff::diff_set_software_info(generating_software))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        Ok(vec![LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: base.header.generating_software.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set software info", "Softwareinfo setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
