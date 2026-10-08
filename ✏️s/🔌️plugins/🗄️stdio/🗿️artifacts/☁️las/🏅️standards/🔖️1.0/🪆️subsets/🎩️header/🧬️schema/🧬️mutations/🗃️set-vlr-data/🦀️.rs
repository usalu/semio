//! 🗃️ `set-vlr-data` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! 📦️ Replaces a VLR's payload bytes.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetVlrData {
    pub index: usize,
    pub data: Vec<u8>,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetVlrData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "vlr-data", kind: "set-vlr-data", record: "SetVlrData" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { index, data } = self;
        protocol::MutationOutcome::new(diff::diff_set_vlr_data(*index, data.clone()))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            match base.vlrs.get(*index) {
                Some(v) => vec![LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: *index, data: v.data.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set VLR data", "VLR-Daten setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
