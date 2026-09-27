//! 📸️ Whole XML snapshot mutation used by typed Details edits and exact undo.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub(crate) snapshot: XmlSnapshot,
}

impl protocol::MutationKind<XmlSnapshot, XmlMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlMutation as protocol::Mutation<XmlSnapshot>>::Diff> {
        protocol::MutationOutcome::new(crate::schema::diff::diff_set_snapshot(base, &self.snapshot))
    }

    fn inverse(&self, base: &XmlSnapshot) -> Vec<XmlMutation> {
        vec![XmlMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
