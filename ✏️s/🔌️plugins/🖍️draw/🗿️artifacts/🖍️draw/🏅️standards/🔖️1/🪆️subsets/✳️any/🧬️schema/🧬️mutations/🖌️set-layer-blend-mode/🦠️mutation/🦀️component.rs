//! 🖌️ Draw mutation — `SetLayerBlendMode`: sets one layer's `blend_mode` scalar.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖌️ `set-layer-blend-mode` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "set-layer-blend-mode")]
pub struct SetLayerBlendMode {
    pub layer_id: String,
    pub blend_mode: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_blend_mode(layer_id: String, blend_mode: String) -> DrawMutation {
    DrawMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id, blend_mode })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for SetLayerBlendMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-blend-mode", record: "SetLayerBlendMode" };

    fn diff(&self, base: &DrawSnapshot) -> DrawDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
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
