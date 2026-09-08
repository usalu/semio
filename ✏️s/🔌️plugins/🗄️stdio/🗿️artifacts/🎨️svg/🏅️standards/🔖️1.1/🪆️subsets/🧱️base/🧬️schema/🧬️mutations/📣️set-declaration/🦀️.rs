//! 🧬️ Direct set-declaration mutation owner.
use crate::schema::diff::SvgDiff;
use crate::SvgSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;

#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDeclarationPayload {
    pub declaration: Option<XmlDeclaration>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetDeclarationMutation { Apply(SetDeclarationPayload), Restore(SvgDiff) }

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetDeclarationMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "declaration", kind: "set-declaration", record: "SetDeclaration" };

    fn diff(&self, _base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(SvgDiff { prolog: None, declaration: Some(payload.declaration.clone()), doctype: None, root: None }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<super::SvgMutation> {
        let outcome = <Self as protocol::MutationKind<SvgSnapshot, super::SvgMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::inverse(outcome.diff(), base);
        vec![super::SvgMutation::SetDeclaration(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Set Declaration".to_string() }
    fn target(&self) -> Vec<String> { vec!["set-declaration".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<SetDeclarationMutation as protocol::MutationKind<SvgSnapshot, super::super::SvgMutation>>::SEMANTICS.kind, "set-declaration"); }
}
