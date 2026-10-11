//! 🧩️ `replace-xml-node` -- replaces one revision-bound canonical XML node exactly; it is the exact inverse of every node-level edit.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceXmlNode {
    pub(crate) address: PptxXmlAddress,
    pub(crate) node: XmlNode,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for ReplaceXmlNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "xml-node", kind: "replace-xml-node", record: "ReplaceXmlNode" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        plan_outcome(xml_address::replace_node_plan(base, &self.address, self.node.clone()))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(xml_address::replace_node_plan(base, &self.address, self.node.clone())))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace XML node", "XML-Knoten ersetzen")
    }

    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.part_path.clone()).chain(self.address.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
