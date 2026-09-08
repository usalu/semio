//! 🧬️ Direct set-doctype mutation owner.
use crate::schema::diff::SvgDiff;
use crate::SvgSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype;

#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDoctypePayload {
    pub doctype: Option<XmlDoctype>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetDoctypeMutation { Apply(SetDoctypePayload), Restore(SvgDiff) }

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetDoctypeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "doctype", kind: "set-doctype", record: "SetDoctype" };

    fn diff(&self, _base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(SvgDiff { prolog: None, declaration: None, doctype: Some(payload.doctype.clone()), root: None }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<super::SvgMutation> {
        let outcome = <Self as protocol::MutationKind<SvgSnapshot, super::SvgMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::inverse(outcome.diff(), base);
        vec![super::SvgMutation::SetDoctype(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Set Doctype".to_string() }
    fn target(&self) -> Vec<String> { vec!["set-doctype".to_string()] }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
