//! 🔁 Draw mutation — `ReplaceLayerFill`: whole-value swap of one layer's structured `fill`
//! sub-payload (a tagged `FillStyle` union — solid/linear/radial — not a scalar).
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, FillStyle};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-layer-fill` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-layer-fill")]
pub struct ReplaceLayerFill {
    pub layer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(statements, block)]
    pub fill: Option<FillStyle>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_layer_fill(layer_id: String, fill: Option<FillStyle>) -> DrawMutation {
    DrawMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id, fill })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for ReplaceLayerFill {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "layer", kind: "replace-layer-fill", record: "ReplacedLayerFill" };

    async fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace layer \"{}\" fill", self.layer_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
