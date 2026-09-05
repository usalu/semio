//! 👁️ Drawing mutation — `SetLayerVisible`: flips one layer's `visible` flag (addressed, single-field
//! setter — the taxonomy's own canonical `set` example).
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Mutation
/// 👁️ `set-layer-visible` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-visible")]
pub struct SetLayerVisible {
    pub layer_id: String,
    pub visible: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_visible(layer_id: String, visible: bool) -> DrawingMutation {
    DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id, visible })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetLayerVisible {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-visible", record: "SetLayerVisible" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set layer \"{}\" visible to {}", self.layer_id, self.visible)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
