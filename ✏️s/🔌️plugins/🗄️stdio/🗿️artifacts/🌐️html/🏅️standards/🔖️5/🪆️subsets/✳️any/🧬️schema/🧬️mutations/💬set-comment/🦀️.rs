//! 💬️ `set-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot::{NodePath, HtmlSnapshot, node_at, HtmlNode};
use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::diff::{diff_at_path, HtmlNodeDiff};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetComment {
    pub path: NodePath,
    pub text: String,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "comment", kind: "set-comment", record: "SetComment" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set comment", "Kommentar setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<HtmlSnapshot, HtmlMutation> for SetComment {
    fn diff(&self, _base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as protocol::ColdMutation<HtmlSnapshot>>::ReferenceDiff> {
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
            vec![HtmlMutation::SetComment(SetComment { path: path.clone(), text: old })]
        })
    }
}
//#endregion 🔖️Payload
