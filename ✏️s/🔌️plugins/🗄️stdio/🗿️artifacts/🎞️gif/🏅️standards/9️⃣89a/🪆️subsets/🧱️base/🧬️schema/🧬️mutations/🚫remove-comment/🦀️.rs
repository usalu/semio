//! 🚫️ `remove-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-comment")]
pub struct RemoveComment {
    pub(crate) index: usize,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for RemoveComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "comment", kind: "remove-comment", record: "RemoveComment" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(GifDiff { comments: Some(GifCommentsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok(match base.comments.get(*index) {
            Some(text) => vec![GifMutation::InsertComment(insert_comment::InsertComment { index: *index, text: text.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove comment", "Kommentar entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
