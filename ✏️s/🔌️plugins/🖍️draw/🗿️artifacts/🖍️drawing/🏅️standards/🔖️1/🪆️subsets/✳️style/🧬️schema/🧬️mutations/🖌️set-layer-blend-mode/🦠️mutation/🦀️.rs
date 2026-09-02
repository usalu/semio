//! 🖌️ Drawing mutation — `SetLayerBlendMode`: sets one layer's `blend_mode` scalar.
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Mutation
/// 🖌️ `set-layer-blend-mode` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-blend-mode")]
pub struct SetLayerBlendMode {
    pub layer_id: String,
    pub blend_mode: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_blend_mode(layer_id: String, blend_mode: String) -> DrawingMutation {
    DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id, blend_mode })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetLayerBlendMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-blend-mode", record: "SetLayerBlendMode" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set layer \"{}\" blend mode to {}", self.layer_id, self.blend_mode)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
