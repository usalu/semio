//! 🧬️ Direct insert-element mutation owner.
use crate::schema::diff::{diff_at_path, XmlChildAdded, XmlChildrenDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::schema::mutation_support::XmlNodePath;
use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertElementMutation {
    pub path: XmlNodePath,
    pub index: usize,
    pub node: XmlNode,
}

pub type InsertElementPayload = InsertElementMutation;

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for InsertElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "element", kind: "insert-element", record: "InsertedElement" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        protocol::MutationOutcome::new(diff_at_path(
            &self.path.0,
            XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlChildAdded { index: self.index, item: self.node.clone() }] }) }),
        ))
    }

    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<super::XmlMutation>, semio_framework_value::ValueError> {
        Ok(match self.path.resolve(base.doc.root.as_ref()) {
            Some(XmlNode::Element { children, .. }) if self.index <= children.len() => vec![super::XmlMutation::RemoveElement(super::RemoveElementMutation { path: self.path.clone(), index: self.index })],
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert Element", "Element einfügen")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-element".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
