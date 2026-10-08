//! 🧬️ Direct set-text mutation owner.
use crate::schema::diff::{diff_at_path, SvgDiff, SvgNodeDiff};
use crate::schema::snapshot::{node_at, NodePath, SvgNode};
use crate::SvgSnapshot;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTextPayload {
    pub path: NodePath,
    pub text: String,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetTextPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "text", kind: "set-text", record: "SetText" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, text } = self;
        protocol::MutationOutcome::new(diff_at_path(path, SvgNodeDiff::Text { text: Some(text.clone()) }))
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        Ok(match node_at(&base.doc, path) {
            Ok(SvgNode::Text { text }) => vec![super::SvgMutation::SetText(Self { path: path.clone(), text: text.clone() })],
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
