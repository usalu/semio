//! 🔒️ Drawing mutation — `SetLayerLocked`: flips one layer's `locked` flag.
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Mutation
/// 🔒️ `set-layer-locked` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-locked")]
pub struct SetLayerLocked {
    pub layer_id: String,
    pub locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_locked(layer_id: String, locked: bool) -> DrawingMutation {
    DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id, locked })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetLayerLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-locked", record: "SetLayerLocked" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set layer \"{}\" locked to {}", self.layer_id, self.locked)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
