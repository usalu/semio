//! 🧬️ Direct insert-element mutation owner.
use crate::artifacts::xml::schema::diff::{diff_at_path, XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlChildAdded, XmlChildrenDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::artifacts::xml::schema::mutation_support::XmlNodePath;
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlDoctype, XmlNode};
use crate::artifacts::xml::XmlSnapshot;

#[path = "📝️text/🦀️component.rs"]
pub mod text;
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertElementPayload {
    pub path: XmlNodePath,
    pub index: usize,
    pub node: XmlNode,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum InsertElementMutation { Apply(InsertElementPayload), Restore(XmlDiff) }

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for InsertElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "element", kind: "insert-element", record: "InsertedElement" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(diff_at_path(&payload.path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlChildAdded { index: payload.index, item: payload.node.clone() }] }) }))),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &XmlSnapshot) -> Vec<super::XmlMutation> {
        let outcome = <Self as protocol::MutationKind<XmlSnapshot, super::XmlMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::inverse(outcome.diff(), base);
        vec![super::XmlMutation::InsertElement(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Insert Element".to_string() }
    fn target(&self) -> Vec<String> { vec!["insert-element".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<InsertElementMutation as protocol::MutationKind<XmlSnapshot, super::super::XmlMutation>>::SEMANTICS.kind, "insert-element"); }
}
