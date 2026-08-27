//! 🧬️ Direct set-attribute mutation owner.
use crate::artifacts::xml::schema::diff::{diff_at_path, XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlChildAdded, XmlChildrenDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::artifacts::xml::schema::mutation_support::XmlNodePath;
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlDoctype, XmlNode};
use crate::artifacts::xml::XmlSnapshot;
use serde::{Deserialize, Serialize};

#[path = "📝️text/🦀️component.rs"]
pub mod text;
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributePayload {
    pub path: XmlNodePath,
    pub name: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetAttributeMutation { Apply(SetAttributePayload), Restore(XmlDiff) }

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for SetAttributeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "attribute", kind: "set-attribute", record: "SetAttribute" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new({ let target = payload.path.resolve(base.doc.root.as_ref()); let existing = target.and_then(|node| match node { XmlNode::Element { attrs, .. } => attrs.iter().find(|attribute| attribute.name == payload.name), _ => None }); let attributes = match (existing, &payload.value) { (Some(_), Some(value)) => XmlAttributesDiff { removed: Vec::new(), modified: vec![XmlAttrModified { name: payload.name.clone(), value: value.clone() }], added: Vec::new() }, (Some(_), None) => XmlAttributesDiff { removed: vec![payload.name.clone()], modified: Vec::new(), added: Vec::new() }, (None, Some(value)) => { let index = match target { Some(XmlNode::Element { attrs, .. }) => attrs.len(), _ => 0 }; XmlAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlAttrAdded { index, name: payload.name.clone(), value: value.clone() }] } }, (None, None) => XmlAttributesDiff::default() }; diff_at_path(&payload.path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attributes), children: None })) }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &XmlSnapshot) -> Vec<super::XmlMutation> {
        let outcome = <Self as protocol::MutationKind<XmlSnapshot, super::XmlMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::inverse(outcome.diff(), base);
        vec![super::XmlMutation::SetAttribute(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Set Attribute".to_string() }
    fn target(&self) -> Vec<String> { vec!["set-attribute".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<SetAttributeMutation as protocol::MutationKind<XmlSnapshot, super::super::XmlMutation>>::SEMANTICS.kind, "set-attribute"); }
}
