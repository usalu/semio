//! 🔀 `reorder-layers` — repositions an id-addressed layer within or across the layer tree (LIST
//! position, never spatial — the coordinator's explicit ruling; spatial reposition is `move-layer`).

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ReorderLayers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderLayers {
    pub layer_id: String,
    pub parent_id: Option<String>,
    pub index: usize,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ReorderLayers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "layer", kind: "reorder-layers", record: "ReorderedLayers" };

    async fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        crate::artifacts::raster::mutations::reorder_layers::diff::diff(self, base)
    }

    async fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::reorder_layers::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Reorder layer {}", self.layer_id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ReorderLayers
