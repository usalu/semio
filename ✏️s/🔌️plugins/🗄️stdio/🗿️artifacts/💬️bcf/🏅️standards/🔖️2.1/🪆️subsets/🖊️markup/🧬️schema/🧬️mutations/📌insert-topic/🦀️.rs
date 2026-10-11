//! 📌️ `insert-topic` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertTopic {
    pub(crate) topic: BcfTopic,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for InsertTopic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "topic", kind: "insert-topic", record: "InsertTopic" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic, index } = self;
        let at = index.unwrap_or(base.topics.len());
        protocol::MutationOutcome::new(BcfDiff { version: None, topics: Some(BcfTopicsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexedAdded { index: at, item: topic.clone() }] }), parts: None })
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic, .. } = self;
        Ok({ vec![BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: topic.guid.clone() })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert topic", "Thema einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
