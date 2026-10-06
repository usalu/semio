//! 🧬️ Direct set-attribute mutation owner.
use crate::schema::diff::{diff_at_path, XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::schema::mutation_support::XmlNodePath;
use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAttributePayload {
    pub path: XmlNodePath,
    pub name: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetAttributeMutation {
    Apply(SetAttributePayload),
    /// 📦️ Boxed on purpose: `XmlDiff` is the largest thing this leaf can hold, and an inline
    /// variant of that size pushes the whole leaf past the neutral inline-ownership budget
    /// (`🧫️fixtures/📦️inline-layout/🔣️.json`, 128 B) every ephemeral transfer of it is measured
    /// against — same boxing the sibling `🧊️gltf` leaves use for their own `Restore` arm.
    Restore(Box<XmlDiff>),
}

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for SetAttributeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "attribute", kind: "set-attribute", record: "SetAttribute" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new({
                let target = payload.path.resolve(base.doc.root.as_ref());
                let existing = target.and_then(|node| match node {
                    XmlNode::Element { attrs, .. } => attrs.iter().find(|attribute| attribute.name == payload.name),
                    _ => None,
                });
                let mut order: Vec<String> = match target {
                    Some(XmlNode::Element { attrs, .. }) => attrs.iter().map(|attr| attr.name.clone()).collect(),
                    _ => Vec::new(),
                };
                if payload.value.is_none() {
                    order.retain(|name| *name != payload.name);
                } else if existing.is_none() {
                    order.push(payload.name.clone());
                }
                let attributes = match (existing, &payload.value) {
                    (Some(_), Some(value)) => XmlAttributesDiff { order, removed: Vec::new(), modified: vec![XmlAttrModified { name: payload.name.clone(), value: value.clone() }], added: Vec::new() },
                    (Some(_), None) => XmlAttributesDiff { order, removed: vec![payload.name.clone()], modified: Vec::new(), added: Vec::new() },
                    (None, Some(value)) => XmlAttributesDiff { order, removed: Vec::new(), modified: Vec::new(), added: vec![XmlAttrAdded { name: payload.name.clone(), value: value.clone() }] },
                    (None, None) => XmlAttributesDiff::default(),
                };
                diff_at_path(&payload.path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attributes), children: None }))
            }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.as_ref().clone()),
        }
    }

    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<super::XmlMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let outcome = <Self as protocol::MutationKind<XmlSnapshot, super::XmlMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        let inverse = <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::inverse(outcome.diff(), base);
        vec![super::XmlMutation::SetAttribute(Self::Restore(Box::new(inverse)))]
    
    })())
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
