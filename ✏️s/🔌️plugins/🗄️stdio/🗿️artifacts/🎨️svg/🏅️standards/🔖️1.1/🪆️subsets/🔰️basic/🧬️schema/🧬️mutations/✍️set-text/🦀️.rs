//! ✍️ `set-text` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetText {
    pub(crate) path: NodePath,
    pub(crate) text: String,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for SetText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "text", kind: "set-text", record: "SetText" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, text } = self;
        protocol::MutationOutcome::new(diff_at_path(path, SvgNodeDiff::Text { text: Some(text.clone()) }))
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgBasicMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        Ok(match node_at(&base.doc, path) {
            Ok(SvgNode::Text { text }) => vec![SvgBasicMutation::SetText(set_text::SetText { path: path.clone(), text: text.clone() })],
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set text", "Text setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
