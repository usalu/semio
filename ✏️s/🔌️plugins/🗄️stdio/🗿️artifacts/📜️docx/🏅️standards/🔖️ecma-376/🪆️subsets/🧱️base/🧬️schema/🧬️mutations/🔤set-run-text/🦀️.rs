//! 🔤️ `set-run-text` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRunText {
    pub address: DocxXmlAddress,
    pub text: String,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetRunText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run-text", kind: "set-run-text", record: "SetRunText" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        addressed_outcome(&DocxMutation::SetRunText(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, semio_framework_value::ValueError> {
    Ok({
        addressed_inverse(&DocxMutation::SetRunText(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set run text", "Text des Textlaufs setzen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.part_path.clone()).chain(self.address.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
