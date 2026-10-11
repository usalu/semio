//! 🎨️ `set-run-formatting` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRunFormatting {
    pub address: DocxXmlAddress,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetRunFormatting {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run-formatting", kind: "set-run-formatting", record: "SetRunFormatting" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        addressed_outcome(&DocxMutation::SetRunFormatting(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, semio_framework_value::ValueError> {
    Ok({
        addressed_inverse(&DocxMutation::SetRunFormatting(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set run formatting", "Formatierung des Textlaufs setzen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.part_path.clone()).chain(self.address.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
