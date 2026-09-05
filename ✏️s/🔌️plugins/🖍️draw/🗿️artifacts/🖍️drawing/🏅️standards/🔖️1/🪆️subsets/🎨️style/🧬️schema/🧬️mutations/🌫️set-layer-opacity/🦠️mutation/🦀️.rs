//! 🌫️ Drawing mutation — `SetLayerOpacity`: sets one layer's `opacity` scalar.
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Mutation
/// 🌫️ `set-layer-opacity` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-opacity")]
pub struct SetLayerOpacity {
    pub layer_id: String,
    pub opacity: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_opacity(layer_id: String, opacity: f64) -> DrawingMutation {
    DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id, opacity })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetLayerOpacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-opacity", record: "SetLayerOpacity" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set layer \"{}\" opacity to {}", self.layer_id, self.opacity)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
