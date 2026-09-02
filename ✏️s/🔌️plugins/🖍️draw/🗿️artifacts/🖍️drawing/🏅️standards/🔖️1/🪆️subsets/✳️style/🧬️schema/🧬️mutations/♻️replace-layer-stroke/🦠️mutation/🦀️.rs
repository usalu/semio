//! ♻️ Drawing mutation — `ReplaceLayerStroke`: whole-value swap of one layer's structured `stroke`
//! sub-payload.
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::{DrawingSnapshot, StrokeStyle};

//#region 🔖️Mutation
/// ♻️ `replace-layer-stroke` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-layer-stroke")]
pub struct ReplaceLayerStroke {
    pub layer_id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub stroke: Option<StrokeStyle>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_layer_stroke(layer_id: String, stroke: Option<StrokeStyle>) -> DrawingMutation {
    DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id, stroke })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for ReplaceLayerStroke {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "layer", kind: "replace-layer-stroke", record: "ReplacedLayerStroke" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace layer \"{}\" stroke", self.layer_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
