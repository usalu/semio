//! 🧬️ Direct remove-element mutation owner.
use crate::schema::diff::{diff_at_path, XmlChildrenDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::schema::mutation_support::XmlNodePath;
use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveElementMutation {
    pub path: XmlNodePath,
    pub index: usize,
}

pub type RemoveElementPayload = RemoveElementMutation;

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for RemoveElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "element", kind: "remove-element", record: "RemovedElement" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        protocol::MutationOutcome::new(diff_at_path(
            &self.path.0,
            XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: vec![self.index], modified: Vec::new(), added: Vec::new() }) }),
        ))
    }

    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<super::XmlMutation>, semio_framework_value::ValueError> {
        Ok(match self.path.resolve(base.doc.root.as_ref()) {
            Some(XmlNode::Element { children, .. }) => children.get(self.index).map(|node| vec![super::XmlMutation::InsertElement(super::InsertElementMutation { path: self.path.clone(), index: self.index, node: node.clone() })]).unwrap_or_default(),
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove Element", "Element entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-element".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
