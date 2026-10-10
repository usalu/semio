//! 🧬️ Direct set-attribute mutation owner.
use crate::schema::diff::{diff_at_path, XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::schema::mutation_support::XmlNodePath;
use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAttributeMutation {
    pub path: XmlNodePath,
    pub name: String,
    pub value: Option<String>,
    /// 🧭️ Where a newly added attribute lands among its element's attributes; `None` appends.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

pub type SetAttributePayload = SetAttributeMutation;

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for SetAttributeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "attribute", kind: "set-attribute", record: "SetAttribute" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        let target = self.path.resolve(base.doc.root.as_ref());
        let existing = target.and_then(|node| match node {
            XmlNode::Element { attrs, .. } => attrs.iter().find(|attribute| attribute.name == self.name),
            _ => None,
        });
        let mut order: Vec<String> = match target {
            Some(XmlNode::Element { attrs, .. }) => attrs.iter().map(|attr| attr.name.clone()).collect(),
            _ => Vec::new(),
        };
        if self.value.is_none() {
            order.retain(|name| *name != self.name);
        } else if existing.is_none() {
            order.insert(self.index.unwrap_or(order.len()).min(order.len()), self.name.clone());
        }
        let attributes = match (existing, &self.value) {
            (Some(_), Some(value)) => XmlAttributesDiff { order, removed: Vec::new(), modified: vec![XmlAttrModified { name: self.name.clone(), value: value.clone() }], added: Vec::new() },
            (Some(_), None) => XmlAttributesDiff { order, removed: vec![self.name.clone()], modified: Vec::new(), added: Vec::new() },
            (None, Some(value)) => XmlAttributesDiff { order, removed: Vec::new(), modified: Vec::new(), added: vec![XmlAttrAdded { name: self.name.clone(), value: value.clone() }] },
            (None, None) => XmlAttributesDiff::default(),
        };
        protocol::MutationOutcome::new(diff_at_path(&self.path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attributes), children: None })))
    }

    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<super::XmlMutation>, semio_framework_value::ValueError> {
        let Some(XmlNode::Element { attrs, .. }) = self.path.resolve(base.doc.root.as_ref()) else { return Ok(Vec::new()) };
        let position = attrs.iter().position(|attribute| attribute.name == self.name);
        let previous = position.map(|position| attrs[position].value.clone());
        Ok(if previous == self.value {
            Vec::new()
        } else {
            vec![super::XmlMutation::SetAttribute(Self { path: self.path.clone(), name: self.name.clone(), value: previous, index: position })]
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Attribute", "Attribut setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-attribute".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
