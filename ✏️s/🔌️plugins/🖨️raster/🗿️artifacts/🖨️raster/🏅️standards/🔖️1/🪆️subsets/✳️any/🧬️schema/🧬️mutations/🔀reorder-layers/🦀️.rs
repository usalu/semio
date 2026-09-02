//! 🔀 `reorder-layers` — repositions an id-addressed layer within or across the layer tree (LIST
//! position, never spatial — the coordinator's explicit ruling; spatial reposition is `move-layer`).

pub mod mutation {
use serde::{Deserialize, Serialize};
use crate::artifacts::raster::diff::{diff_move_layer, RasterDiff};
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_node_id, locate_layer};
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};

//#region 🔖️ReorderLayers
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf, Serialize, Deserialize)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ReorderLayers {
    pub layer_id: String,
    pub parent_id: Option<String>,
    pub index: usize,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ReorderLayers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "layer", kind: "reorder-layers", record: "ReorderedLayers" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Reorder layer {}", self.layer_id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ReorderLayers
}

pub use mutation::ReorderLayers;
