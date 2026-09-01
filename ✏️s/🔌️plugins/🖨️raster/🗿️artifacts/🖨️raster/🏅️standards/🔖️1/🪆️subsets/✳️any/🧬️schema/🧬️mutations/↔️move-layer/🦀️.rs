//! ↔️ `move-layer` — absolute spatial reposition of an id-addressed layer's `transform.x`/`.y`.
//! Distinct from `reorder-layers` (list position, never spatial).

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::move_layer::MoveLayer;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_transform};
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️MoveLayer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct MoveLayer {
    pub layer_id: String,
    pub new_x: f64,
    pub new_y: f64,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for MoveLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "layer", kind: "move-layer", record: "MovedLayer" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Move layer {} to ({}, {})", self.layer_id, self.new_x, self.new_y)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️MoveLayer
