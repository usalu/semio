//! 🔁 Drawing mutation — `ReplaceLayerFill`: whole-value swap of one layer's structured `fill`
//! sub-payload (a tagged `FillStyle` union — solid/linear/radial — not a scalar).
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::{DrawingSnapshot, FillStyle};

//#region 🔖️Mutation
/// 🔁 `replace-layer-fill` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-layer-fill")]
pub struct ReplaceLayerFill {
    pub layer_id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(statements, block)]
    pub fill: Option<FillStyle>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_layer_fill(layer_id: String, fill: Option<FillStyle>) -> DrawingMutation {
    DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id, fill })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for ReplaceLayerFill {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "layer", kind: "replace-layer-fill", record: "ReplacedLayerFill" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace layer \"{}\" fill", self.layer_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
