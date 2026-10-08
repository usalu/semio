//! 🙈️ `remove-viewpoint` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveViewpoint {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for RemoveViewpoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "viewpoint", kind: "remove-viewpoint", record: "RemoveViewpoint" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic_guid, guid } = self;
        protocol::MutationOutcome::new(wrap_topic_diff(base, topic_guid, BcfTopicDiff { viewpoints: Some(BcfViewpointsDiff { removed: vec![viewpoint_index(base, topic_guid, guid)], modified: Vec::new(), added: Vec::new() }), ..Default::default() }))
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic_guid, guid } = self;
        Ok({
            match find_viewpoint(base, topic_guid, guid) {
                Some(v) => vec![BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: topic_guid.clone(), viewpoint: v.clone(), index: Some(viewpoint_index(base, topic_guid, guid)) })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove viewpoint", "Blickpunkt entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
