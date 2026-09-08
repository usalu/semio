//! 🧬️ Direct set-element-name mutation owner.
use crate::schema::diff::{diff_at_path, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::NodePath;
use crate::SvgSnapshot;

#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetElementNamePayload {
    pub path: NodePath,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetElementNameMutation { Apply(SetElementNamePayload), Restore(SvgDiff) }

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetElementNameMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "element-name", kind: "set-element-name", record: "SetElementName" };

    fn diff(&self, _base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(diff_at_path(&payload.path, SvgNodeDiff::Element(SvgElementDiff { name: Some(payload.name.clone()), attributes: None, children: None }))),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<super::SvgMutation> {
        let outcome = <Self as protocol::MutationKind<SvgSnapshot, super::SvgMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::inverse(outcome.diff(), base);
        vec![super::SvgMutation::SetElementName(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Set Element Name".to_string() }
    fn target(&self) -> Vec<String> { vec!["set-element-name".to_string()] }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
