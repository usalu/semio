//! 📷️ `set-viewpoint-camera` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetViewpointCamera {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
    pub(crate) camera: Option<BcfCamera>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetViewpointCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "viewpoint-camera", kind: "set-viewpoint-camera", record: "SetViewpointCamera" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic_guid, guid, camera } = self;
        protocol::MutationOutcome::new(wrap_viewpoint_diff(base, topic_guid, guid, BcfViewpointDiff { camera: Some(camera.clone()), components: None, snapshot: None }))
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic_guid, guid, .. } = self;
        Ok({
            match find_viewpoint(base, topic_guid, guid) {
                Some(v) => vec![BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: topic_guid.clone(), guid: guid.clone(), camera: v.camera.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set viewpoint camera", "Blickpunktkamera setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
