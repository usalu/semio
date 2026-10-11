//! 💬️ `insert-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertComment {
    pub(crate) index: usize,
    pub(crate) comment: String,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for InsertComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "comment", kind: "insert-comment", record: "InsertComment" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as Mutation<PlySnapshot>>::Diff> {
        let Self { index, comment } = self;
        protocol::MutationOutcome::new(diff_insert_comment((*index).min(base.comments.len()), comment.clone()))
    }
    fn inverse(&self, base: &PlySnapshot) -> Result<Vec<PlyMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            {
                let at = (*index).min(base.comments.len());
                vec![PlyMutation::RemoveComment(remove_comment::RemoveComment { index: at })]
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert comment", "Kommentar einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
