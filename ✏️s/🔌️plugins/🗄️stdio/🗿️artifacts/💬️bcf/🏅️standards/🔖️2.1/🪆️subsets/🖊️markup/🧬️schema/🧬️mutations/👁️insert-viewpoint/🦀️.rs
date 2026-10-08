//! 👁️ `insert-viewpoint` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertViewpoint {
    pub(crate) topic_guid: String,
    pub(crate) viewpoint: BcfViewpoint,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for InsertViewpoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "viewpoint", kind: "insert-viewpoint", record: "InsertViewpoint" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic_guid, viewpoint, index } = self;
        let at = index.unwrap_or_else(|| find_topic(base, topic_guid).map_or(0, |topic| topic.viewpoints.len()));
        protocol::MutationOutcome::new(wrap_topic_diff(base, topic_guid, BcfTopicDiff { viewpoints: Some(BcfViewpointsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexedAdded { index: at, item: viewpoint.clone() }] }), ..Default::default() }))
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic_guid, viewpoint, .. } = self;
        Ok({
            {
                vec![BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: topic_guid.clone(), guid: viewpoint.guid.clone() })]
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert viewpoint", "Blickpunkt einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
