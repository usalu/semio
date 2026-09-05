//! 🔄️ Drawing mutation — `UpdateLayerTransform`: sets one layer's `transform` facet atomically
//! (position + scale + rotation are one field in the schema, never independently persisted —
//! the `update` verb's cohesive-multi-field-facet exception).
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::{DrawingSnapshot, DrawingTransform};

//#region 🔖️Mutation
/// 🔄️ `update-layer-transform` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-layer-transform")]
pub struct UpdateLayerTransform {
    pub layer_id: String,
    #[dsl(block)]
    pub transform: DrawingTransform,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_layer_transform(layer_id: String, transform: DrawingTransform) -> DrawingMutation {
    DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id, transform })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for UpdateLayerTransform {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "layer", kind: "update-layer-transform", record: "UpdatedLayerTransform" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update layer \"{}\" transform", self.layer_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
