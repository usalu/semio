//! 🧩️ Exact canonical XML node replacement used by compact DOCX inverses.

use super::*;

/// 🧩️ Replaces one revision-bound canonical XML node.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceXmlNode {
    pub address: DocxXmlAddress,
    pub node: XmlNode,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for ReplaceXmlNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "xml-node", kind: "replace-xml-node", record: "ReplaceXmlNode" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        addressed_outcome(&DocxMutation::ReplaceXmlNode(self.clone()), base)
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, semio_framework_value::ValueError> {
    Ok({
        addressed_inverse(&DocxMutation::ReplaceXmlNode(self.clone()), base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace XML node", "XML-Knoten ersetzen")
    }

    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.part_path.clone()).chain(self.address.node_path.iter().map(usize::to_string)).collect()
    }
}
