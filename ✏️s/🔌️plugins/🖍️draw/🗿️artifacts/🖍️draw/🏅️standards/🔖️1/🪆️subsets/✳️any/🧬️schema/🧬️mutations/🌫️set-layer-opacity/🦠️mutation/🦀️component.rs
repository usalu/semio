//! 🌫️ Draw mutation — `SetLayerOpacity`: sets one layer's `opacity` scalar.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌫️ `set-layer-opacity` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "set-layer-opacity")]
pub struct SetLayerOpacity {
    pub layer_id: String,
    pub opacity: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn set_layer_opacity(layer_id: String, opacity: f64) -> DrawMutation {
    DrawMutation::SetLayerOpacity(SetLayerOpacity { layer_id, opacity })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for SetLayerOpacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-opacity", record: "SetLayerOpacity" };

    async fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Set layer \"{}\" opacity to {}", self.layer_id, self.opacity)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
