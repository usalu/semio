//! 🔄️ Draw mutation — `UpdateLayerTransform`: sets one layer's `transform` facet atomically
//! (position + scale + rotation are one field in the schema, never independently persisted —
//! the `update` verb's cohesive-multi-field-facet exception).
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, DrawTransform};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔄️ `update-layer-transform` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-layer-transform")]
pub struct UpdateLayerTransform {
    pub layer_id: String,
    #[dsl(block)]
    pub transform: DrawTransform,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn update_layer_transform(layer_id: String, transform: DrawTransform) -> DrawMutation {
    DrawMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id, transform })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for UpdateLayerTransform {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "layer", kind: "update-layer-transform", record: "UpdatedLayerTransform" };

    async fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Update layer \"{}\" transform", self.layer_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
