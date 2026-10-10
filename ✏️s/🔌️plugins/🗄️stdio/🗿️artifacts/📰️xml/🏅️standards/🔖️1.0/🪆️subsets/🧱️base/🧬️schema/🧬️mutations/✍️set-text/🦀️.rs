//! 🧬️ Direct set-text mutation owner.
use crate::schema::diff::{diff_at_path, XmlDiff, XmlNodeDiff};
use crate::schema::snapshot::XmlNode;
use crate::schema::mutation_support::XmlNodePath;
use crate::XmlSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTextMutation {
    pub path: XmlNodePath,
    pub text: String,
}

pub type SetTextPayload = SetTextMutation;

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for SetTextMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "text", kind: "set-text", record: "SetText" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        protocol::MutationOutcome::new(diff_at_path(&self.path.0, XmlNodeDiff::Text { text: Some(self.text.clone()) }))
    }

    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<super::XmlMutation>, semio_framework_value::ValueError> {
        Ok(match self.path.resolve(base.doc.root.as_ref()) {
            Some(XmlNode::Text { text }) => vec![super::XmlMutation::SetText(Self { path: self.path.clone(), text: text.clone() })],
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Text", "Text setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-text".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
