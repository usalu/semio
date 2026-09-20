//! 🧬️ Direct insert-element mutation owner.
use crate::schema::diff::{diff_at_path, XmlChildAdded, XmlChildrenDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::schema::mutation_support::XmlNodePath;
use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

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
pub enum InsertElementMutation {
    Apply(InsertElementPayload),
    /// 📦️ Boxed on purpose: `XmlDiff` is the largest thing this leaf can hold, and an inline
    /// variant of that size pushes the whole leaf past the neutral inline-ownership budget
    /// (`🧫️fixtures/📦️inline-layout/🔣️.json`, 128 B) every ephemeral transfer of it is measured
    /// against — same boxing the sibling `🧊️gltf` leaves use for their own `Restore` arm.
    Restore(Box<XmlDiff>),
}

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for InsertElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "element", kind: "insert-element", record: "InsertedElement" };

    fn diff(&self, _base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(diff_at_path(
                &payload.path.0,
                XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlChildAdded { index: payload.index, item: payload.node.clone() }] }) }),
            )),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.as_ref().clone()),
        }
    }

    fn inverse(&self, base: &XmlSnapshot) -> Vec<super::XmlMutation> {
        let outcome = <Self as protocol::MutationKind<XmlSnapshot, super::XmlMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        let inverse = <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::inverse(outcome.diff(), base);
        vec![super::XmlMutation::InsertElement(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Insert Element".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-element".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
