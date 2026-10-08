//! 🖊️ `set-topic-markup` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTopicMarkup {
    pub(crate) guid: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) priority: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) labels: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) creation_date: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) creation_author: Option<String>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetTopicMarkup {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "topic-markup", kind: "set-topic-markup", record: "SetTopicMarkup" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { guid, title, description, status, priority, labels, creation_date, creation_author } = self;
        protocol::MutationOutcome::new(wrap_topic_diff(
            base,
            guid,
            BcfTopicDiff {
                title: title.clone(),
                description: description.clone(),
                status: status.clone(),
                priority: priority.clone(),
                labels: labels.clone(),
                creation_date: creation_date.clone(),
                creation_author: creation_author.clone(),
                comments: None,
                viewpoints: None,
            },
        ))
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { guid, title, description, status, priority, labels, creation_date, creation_author } = self;
        Ok({
            match find_topic(base, guid) {
                Some(t) => vec![BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup {
                    guid: guid.clone(),
                    title: title.as_ref().map(|_| t.title.clone()),
                    description: description.as_ref().map(|_| t.description.clone()),
                    status: status.as_ref().map(|_| t.status.clone()),
                    priority: priority.as_ref().map(|_| t.priority.clone()),
                    labels: labels.as_ref().map(|_| t.labels.clone()),
                    creation_date: creation_date.as_ref().map(|_| t.creation_date.clone()),
                    creation_author: creation_author.as_ref().map(|_| t.creation_author.clone()),
                })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set topic markup", "Themen-Markup setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
