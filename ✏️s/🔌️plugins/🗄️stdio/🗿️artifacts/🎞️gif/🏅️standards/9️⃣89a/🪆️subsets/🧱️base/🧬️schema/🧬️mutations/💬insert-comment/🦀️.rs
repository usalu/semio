//! 💬️ `insert-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "insert-comment")]
pub struct InsertComment {
    pub(crate) index: usize,
    pub(crate) text: String,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for InsertComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "comment", kind: "insert-comment", record: "InsertComment" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index, text } = self;
        protocol::MutationOutcome::new({
            GifDiff { comments: Some(GifCommentsDiff { added: vec![GifCommentAdded { index: (*index).min(base.comments.len()), text: text.clone() }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(vec![GifMutation::RemoveComment(remove_comment::RemoveComment { index: (*index).min(base.comments.len()) })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert comment", "Kommentar einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
