//! 🧬️ Direct set-doctype mutation owner.
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
pub struct SetDoctypePayload {
    pub doctype: Option<XmlDoctype>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetDoctypeMutation { Apply(SetDoctypePayload), Restore(XmlDiff) }

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for SetDoctypeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "doctype", kind: "set-doctype", record: "SetDoctype" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(XmlDiff { prolog: None, declaration: None, doctype: Some(payload.doctype.clone()), root: None }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &XmlSnapshot) -> Vec<super::XmlMutation> {
        let outcome = <Self as protocol::MutationKind<XmlSnapshot, super::XmlMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <XmlDiff as protocol::DiffAlgebra<XmlSnapshot>>::inverse(outcome.diff(), base);
        vec![super::XmlMutation::SetDoctype(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Set Doctype".to_string() }
    fn target(&self) -> Vec<String> { vec!["set-doctype".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<SetDoctypeMutation as protocol::MutationKind<XmlSnapshot, super::super::XmlMutation>>::SEMANTICS.kind, "set-doctype"); }
}
