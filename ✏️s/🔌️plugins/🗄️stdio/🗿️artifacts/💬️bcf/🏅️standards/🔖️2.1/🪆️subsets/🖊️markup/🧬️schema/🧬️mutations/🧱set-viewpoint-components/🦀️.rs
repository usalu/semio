//! 🧱️ `set-viewpoint-components` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetViewpointComponents {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
    pub(crate) components: Option<BcfComponents>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetViewpointComponents {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "viewpoint-components", kind: "set-viewpoint-components", record: "SetViewpointComponents" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic_guid, guid, components } = self;
        protocol::MutationOutcome::new({ wrap_viewpoint_diff(base, topic_guid, guid, BcfViewpointDiff { camera: None, components: Some(components.clone()), snapshot: None }) })
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic_guid, guid, .. } = self;
        Ok({
            match find_viewpoint(base, topic_guid, guid) {
                Some(v) => vec![BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: topic_guid.clone(), guid: guid.clone(), components: v.components.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set viewpoint components", "Blickpunktkomponenten setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
