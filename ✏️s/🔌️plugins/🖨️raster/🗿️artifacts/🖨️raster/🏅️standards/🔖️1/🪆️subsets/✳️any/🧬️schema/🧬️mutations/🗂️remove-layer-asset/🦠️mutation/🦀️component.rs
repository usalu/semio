//! 🗂️ `remove-layer-asset` — detaches an id-addressed `RasterImageAsset` from the document's asset
//! map. Inverse partner of `add-layer-asset` (see that leaf for why this pair exists).

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️RemoveLayerAsset
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLayerAsset {
    pub asset_id: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for RemoveLayerAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "asset", kind: "remove-layer-asset", record: "RemovedLayerAsset" };

    async fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        crate::artifacts::raster::mutations::remove_layer_asset::diff::diff(self, base)
    }

    async fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::remove_layer_asset::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Remove asset {}", self.asset_id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.asset_id.clone()]
    }
}
//#endregion 🔖️RemoveLayerAsset
