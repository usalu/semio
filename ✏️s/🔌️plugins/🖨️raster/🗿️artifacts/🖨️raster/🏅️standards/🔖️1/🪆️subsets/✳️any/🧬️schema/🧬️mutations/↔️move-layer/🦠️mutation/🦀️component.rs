//! ↔️ `move-layer` — absolute spatial reposition of an id-addressed layer's `transform.x`/`.y`.
//! Distinct from `reorder-layers` (list position, never spatial).

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️MoveLayer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveLayer {
    pub layer_id: String,
    pub new_x: f64,
    pub new_y: f64,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for MoveLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "layer", kind: "move-layer", record: "MovedLayer" };

    async fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        crate::artifacts::raster::mutations::move_layer::diff::diff(self, base)
    }

    async fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::move_layer::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Move layer {} to ({}, {})", self.layer_id, self.new_x, self.new_y)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️MoveLayer
