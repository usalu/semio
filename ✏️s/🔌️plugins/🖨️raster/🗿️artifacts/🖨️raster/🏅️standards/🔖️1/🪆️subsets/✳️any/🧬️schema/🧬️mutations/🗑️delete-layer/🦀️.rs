//! 🗑️ `delete-layer` — removes an id-addressed `RasterLayerNode` (and, if it is a `Group`, its
//! whole subtree cascade — `remove_layer_from_tree` deletes the node it finds, children included).

use crate::artifacts::raster::diff::{diff_remove_layer, RasterDiff};
use crate::artifacts::raster::mutations::delete_layer::DeleteLayer;
use crate::artifacts::raster::mutations::{create_layer, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, locate_layer};
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteLayer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct DeleteLayer {
    pub layer_id: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for DeleteLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "layer", kind: "delete-layer", record: "DeletedLayer" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete layer {}", self.layer_id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️DeleteLayer
