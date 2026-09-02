//! ↔️ `move-layer` — absolute spatial reposition of an id-addressed layer's `transform.x`/`.y`.
//! Distinct from `reorder-layers` (list position, never spatial).

pub mod mutation {
use serde::{Deserialize, Serialize};
use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_transform};
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️MoveLayer
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf, Serialize, Deserialize)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct MoveLayer {
    pub layer_id: String,
    pub new_x: f64,
    pub new_y: f64,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for MoveLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "layer", kind: "move-layer", record: "MovedLayer" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Move layer {} to ({}, {})", self.layer_id, self.new_x, self.new_y)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️MoveLayer
}

pub use mutation::MoveLayer;
