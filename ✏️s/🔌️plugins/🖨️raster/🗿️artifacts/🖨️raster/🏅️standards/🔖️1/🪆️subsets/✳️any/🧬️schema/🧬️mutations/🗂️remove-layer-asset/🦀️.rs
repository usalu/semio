//! 🗂️ `remove-layer-asset` — detaches an id-addressed `RasterImageAsset` from the document's asset
//! map. Inverse partner of `add-layer-asset` (see that leaf for why this pair exists).

use crate::artifacts::raster::diff::{diff_remove_asset, RasterDiff};
use crate::artifacts::raster::mutations::remove_layer_asset::RemoveLayerAsset;
use crate::artifacts::raster::mutations::{add_layer_asset, RasterMutation};
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️RemoveLayerAsset
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLayerAsset {
    pub asset_id: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for RemoveLayerAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "asset", kind: "remove-layer-asset", record: "RemovedLayerAsset" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove asset {}", self.asset_id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.asset_id.clone()]
    }
}
//#endregion 🔖️RemoveLayerAsset
