//! 🧰️ `set-auxiliary-header` — replaces the auxiliary header block of the container. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use crate::schema::snapshot::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAuxiliaryHeader {
    pub auxiliary_header: DwgAuxiliaryHeader,
}

impl protocol::MutationKind<DwgSnapshot, DwgMutation> for SetAuxiliaryHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "auxiliary-header", kind: "set-auxiliary-header", record: "SetAuxiliaryHeader" };

    fn diff(&self, base: &DwgSnapshot) -> protocol::MutationOutcome<<DwgMutation as Mutation<DwgSnapshot>>::Diff> {
        protocol::MutationOutcome::new(DwgDiff { auxiliary_header: (base.auxiliary_header != self.auxiliary_header).then(|| self.auxiliary_header.clone()), ..DwgDiff::default() })
    }
    fn inverse(&self, base: &DwgSnapshot) -> Result<Vec<DwgMutation>, semio_framework_value::ValueError> {
        Ok((base.auxiliary_header != self.auxiliary_header).then(|| DwgMutation::SetAuxiliaryHeader(set_auxiliary_header::SetAuxiliaryHeader { auxiliary_header: base.auxiliary_header.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set auxiliary header", "Hilfskopf setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
