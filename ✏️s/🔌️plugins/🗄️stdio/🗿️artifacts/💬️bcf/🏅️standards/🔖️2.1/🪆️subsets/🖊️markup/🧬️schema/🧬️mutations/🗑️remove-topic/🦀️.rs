//! 🗑️ `remove-topic` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveTopic {
    pub(crate) guid: String,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for RemoveTopic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "topic", kind: "remove-topic", record: "RemoveTopic" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { guid } = self;
        protocol::MutationOutcome::new(BcfDiff { version: None, topics: Some(BcfTopicsDiff { removed: vec![topic_index(base, guid)], modified: Vec::new(), added: Vec::new() }), parts: None })
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { guid } = self;
        Ok({
            match find_topic(base, guid) {
                Some(t) => vec![BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: t.clone(), index: Some(topic_index(base, guid)) })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove topic", "Thema entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
