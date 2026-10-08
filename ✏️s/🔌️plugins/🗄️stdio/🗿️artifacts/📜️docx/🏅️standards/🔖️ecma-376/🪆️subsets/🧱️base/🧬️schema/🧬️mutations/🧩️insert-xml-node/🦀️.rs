//! 🧩️ Exact canonical XML child insertion used by compact DOCX inverses.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertXmlNode {
    pub parent: DocxXmlAddress,
    pub index: usize,
    pub node: XmlNode,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for InsertXmlNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "xml-node", kind: "insert-xml-node", record: "InsertXmlNode" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        addressed_outcome(&DocxMutation::InsertXmlNode(self.clone()), base)
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, semio_framework_value::ValueError> {
    Ok({
        addressed_inverse(&DocxMutation::InsertXmlNode(self.clone()), base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert XML node", "XML-Knoten einfügen")
    }

    fn target(&self) -> Vec<String> {
        std::iter::once(self.parent.part_path.clone()).chain(self.parent.node_path.iter().map(usize::to_string)).chain(std::iter::once(self.index.to_string())).collect()
    }
}
