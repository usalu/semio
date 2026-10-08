//! 📋️ `set-header` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetHeader {
    pub header: Part21Header,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Mutation> for SetHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "header", kind: "set-header", record: "SetHeader" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { header } = self;
        protocol::MutationOutcome::new(Ifc2x3Diff { header: (base.document.header != *header).then(|| header.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3Mutation>, semio_framework_value::ValueError> {
        Ok(vec![Ifc2x3Mutation::SetHeader(Self { header: base.document.header.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set header", "Header setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
