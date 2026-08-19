//! ♻️ Draw mutation — `ReplaceLayerStroke`: whole-value swap of one layer's structured `stroke`
//! sub-payload.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, StrokeStyle};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ♻️ `replace-layer-stroke` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-layer-stroke")]
pub struct ReplaceLayerStroke {
    pub layer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub stroke: Option<StrokeStyle>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_layer_stroke(layer_id: String, stroke: Option<StrokeStyle>) -> DrawMutation {
    DrawMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id, stroke })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for ReplaceLayerStroke {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "layer", kind: "replace-layer-stroke", record: "ReplacedLayerStroke" };

    async fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace layer \"{}\" stroke", self.layer_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
