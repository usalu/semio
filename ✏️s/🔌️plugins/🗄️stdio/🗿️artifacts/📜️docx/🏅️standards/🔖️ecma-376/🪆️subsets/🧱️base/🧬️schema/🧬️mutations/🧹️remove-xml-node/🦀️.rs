//! 🧹️ Exact canonical XML child removal used by compact DOCX inverses.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveXmlNode {
    pub parent: DocxXmlAddress,
    pub index: usize,
    pub expected_name: String,
    pub revision: String,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for RemoveXmlNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "xml-node", kind: "remove-xml-node", record: "RemoveXmlNode" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::RemoveXmlNode(self.clone()), base)
    }

    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxMutation> {
        agg_inverse(&DocxMutation::RemoveXmlNode(self.clone()), base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove XML node", "XML-Knoten entfernen")
    }

    fn target(&self) -> Vec<String> {
        std::iter::once(self.parent.part_path.clone()).chain(self.parent.node_path.iter().map(usize::to_string)).chain(std::iter::once(self.index.to_string())).collect()
    }
}
