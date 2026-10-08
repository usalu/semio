//! 💬️ `set-archive-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-archive-comment")]
pub struct SetArchiveComment {
    pub(crate) comment: String,
    pub(crate) comment_utf8: bool,
}

impl protocol::MutationKind<ZipSnapshot, ZipMutation> for SetArchiveComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "archive-comment", kind: "set-archive-comment", record: "SetArchiveComment" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipMutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        let Self { comment, comment_utf8 } = self;
        protocol::MutationOutcome::new(diff::diff_set_archive_comment(comment, *comment_utf8))
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipMutation>, semio_framework_value::ValueError> {
        Ok(vec![ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: base.comment.clone(), comment_utf8: base.comment_utf8 })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set archive comment", "Archivkommentar setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
