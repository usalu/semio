//! 🧬️ Direct set-text mutation owner.
use crate::schema::diff::{diff_at_path, SvgDiff, SvgNodeDiff};
use crate::schema::snapshot::NodePath;
use crate::SvgSnapshot;

#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTextPayload {
    pub path: NodePath,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetTextMutation { Apply(SetTextPayload), Restore(SvgDiff) }

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetTextMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "text", kind: "set-text", record: "SetText" };

    fn diff(&self, _base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(diff_at_path(&payload.path, SvgNodeDiff::Text { text: Some(payload.text.clone()) })),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<super::SvgMutation> {
        let outcome = <Self as protocol::MutationKind<SvgSnapshot, super::SvgMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::inverse(outcome.diff(), base);
        vec![super::SvgMutation::SetText(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Set Text".to_string() }
    fn target(&self) -> Vec<String> { vec!["set-text".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<SetTextMutation as protocol::MutationKind<SvgSnapshot, super::super::SvgMutation>>::SEMANTICS.kind, "set-text"); }
}
