//! ✏️ `set-comment` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetComment {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) date: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) text: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub(crate) viewpoint_ref: Option<Option<String>>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "comment", kind: "set-comment", record: "SetComment" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic_guid, guid, date, author, text, viewpoint_ref } = self;
        protocol::MutationOutcome::new({ wrap_comment_diff(base, topic_guid, guid, BcfCommentDiff { date: date.clone(), author: author.clone(), text: text.clone(), viewpoint_ref: viewpoint_ref.clone() }) })
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic_guid, guid, date, author, text, viewpoint_ref } = self;
        Ok({
            match find_comment(base, topic_guid, guid) {
                Some(c) => vec![BcfMutation::SetComment(set_comment::SetComment {
                    topic_guid: topic_guid.clone(),
                    guid: guid.clone(),
                    date: date.as_ref().map(|_| c.date.clone()),
                    author: author.as_ref().map(|_| c.author.clone()),
                    text: text.as_ref().map(|_| c.text.clone()),
                    viewpoint_ref: viewpoint_ref.as_ref().map(|_| c.viewpoint_ref.clone()),
                })],
                None => Vec::new(),
            }
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
