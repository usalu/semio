//! 🧬️ Direct set-declaration mutation owner.
use crate::schema::diff::XmlDiff;
use crate::schema::snapshot::XmlDeclaration;
use crate::XmlSnapshot;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDeclarationPayload {
    pub declaration: Option<XmlDeclaration>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetDeclarationMutation {
    Apply(SetDeclarationPayload),
    /// 📦️ Boxed on purpose: `XmlDiff` is the largest thing this leaf can hold, and an inline
    /// variant of that size pushes the whole leaf past the neutral inline-ownership budget
    /// (`🧫️fixtures/📦️inline-layout/🔣️.json`, 128 B) every ephemeral transfer of it is measured
    /// against — same boxing the sibling `🧊️gltf` leaves use for their own `Restore` arm.
    Restore(Box<XmlDiff>),
}

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for SetDeclarationMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "declaration", kind: "set-declaration", record: "SetDeclaration" };

    fn diff(&self, _base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(XmlDiff { prolog: None, declaration: Some(payload.declaration.clone()), doctype: None, root: None }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.as_ref().clone()),
        }
    }

    fn inverse(&self, base: &XmlSnapshot) -> Vec<super::XmlMutation> {
        let outcome = <Self as protocol::MutationKind<XmlSnapshot, super::XmlMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        let inverse = <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::inverse(outcome.diff(), base);
        vec![super::XmlMutation::SetDeclaration(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Set Declaration".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["set-declaration".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
