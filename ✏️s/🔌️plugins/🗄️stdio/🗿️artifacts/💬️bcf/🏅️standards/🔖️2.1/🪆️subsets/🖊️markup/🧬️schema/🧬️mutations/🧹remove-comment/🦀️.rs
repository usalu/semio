//! 🧹️ `remove-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveComment {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for RemoveComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "comment", kind: "remove-comment", record: "RemoveComment" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic_guid, guid } = self;
        protocol::MutationOutcome::new(wrap_topic_diff(base, topic_guid, BcfTopicDiff { comments: Some(BcfCommentsDiff { removed: vec![comment_index(base, topic_guid, guid)], modified: Vec::new(), added: Vec::new() }), ..Default::default() }))
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic_guid, guid } = self;
        Ok({
            match find_comment(base, topic_guid, guid) {
                Some(c) => vec![BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: topic_guid.clone(), comment: c.clone(), index: Some(comment_index(base, topic_guid, guid)) })],
                None => Vec::new(),
            }
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
