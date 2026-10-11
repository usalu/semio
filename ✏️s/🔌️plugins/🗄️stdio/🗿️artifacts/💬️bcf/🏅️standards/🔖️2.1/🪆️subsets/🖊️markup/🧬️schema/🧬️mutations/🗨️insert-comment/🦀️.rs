//! 🗨️ `insert-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertComment {
    pub(crate) topic_guid: String,
    pub(crate) comment: BcfComment,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for InsertComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "comment", kind: "insert-comment", record: "InsertComment" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic_guid, comment, index } = self;
        let at = index.unwrap_or_else(|| find_topic(base, topic_guid).map_or(0, |topic| topic.comments.len()));
        protocol::MutationOutcome::new(wrap_topic_diff(base, topic_guid, BcfTopicDiff { comments: Some(BcfCommentsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexedAdded { index: at, item: comment.clone() }] }), ..Default::default() }))
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic_guid, comment, .. } = self;
        Ok({
            {
                vec![BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: topic_guid.clone(), guid: comment.guid.clone() })]
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
