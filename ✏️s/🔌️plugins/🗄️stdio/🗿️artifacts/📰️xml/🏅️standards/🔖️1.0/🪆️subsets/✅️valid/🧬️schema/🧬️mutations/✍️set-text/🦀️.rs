//! ✍️ `set-text` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetText {
    pub(crate) path: XmlNodePath,
    pub(crate) text: String,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for SetText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "text", kind: "set-text", record: "SetText" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as Mutation<XmlSnapshot>>::Diff> {
        let Self { path, text } = self;
        protocol::MutationOutcome::new(diff_at_path(&path.0, XmlNodeDiff::Text { text: Some(text.clone()) }))
    }
    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<XmlValidMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        Ok({
            {
                let prior = path
                    .resolve(base.doc.root.as_ref())
                    .and_then(|node| match node {
                        XmlNode::Text { text } => Some(text.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();
                vec![XmlValidMutation::SetText(set_text::SetText { path: path.clone(), text: prior })]
            }
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
