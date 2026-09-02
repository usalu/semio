//! ♻️ Draw mutation — `ReplaceLayerStroke`: whole-value swap of one layer's structured `stroke`
//! sub-payload.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, StrokeStyle};

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
pub fn replace_layer_stroke(layer_id: String, stroke: Option<StrokeStyle>) -> DrawMutation {
    DrawMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id, stroke })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for ReplaceLayerStroke {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "layer", kind: "replace-layer-stroke", record: "ReplacedLayerStroke" };

    fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
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
