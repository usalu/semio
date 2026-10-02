//! 🖌️ Revision-bound paragraph style selection over canonical WordprocessingML.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetParagraphStyle {
    pub address: DocxXmlAddress,
    pub style_id: Option<String>,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetParagraphStyle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "paragraph-style", kind: "set-paragraph-style", record: "SetParagraphStyle" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::SetParagraphStyle(self.clone()), base)
    }

    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxMutation> {
        agg_inverse(&DocxMutation::SetParagraphStyle(self.clone()), base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set paragraph style", "Absatzformat setzen")
    }

    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.part_path.clone()).chain(self.address.node_path.iter().map(usize::to_string)).collect()
    }
}
