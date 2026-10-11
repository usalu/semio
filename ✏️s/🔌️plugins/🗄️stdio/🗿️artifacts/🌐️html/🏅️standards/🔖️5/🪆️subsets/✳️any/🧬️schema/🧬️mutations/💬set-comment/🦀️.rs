//! 💬️ `set-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetComment {
    pub(crate) path: NodePath,
    pub(crate) text: String,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "comment", kind: "set-comment", record: "SetComment" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as Mutation<HtmlSnapshot>>::Diff> {
        let Self { path, text } = self;
        protocol::MutationOutcome::new(diff_at_path(path, HtmlNodeDiff::Comment { text: Some(text.clone()) }))
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Result<Vec<HtmlMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        Ok({
            let old = match node_at(base, path) {
                Ok(HtmlNode::Comment { text }) => text.clone(),
                _ => String::new(),
            };
            vec![HtmlMutation::SetComment(set_comment::SetComment { path: path.clone(), text: old })]
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set comment", "Kommentar setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
