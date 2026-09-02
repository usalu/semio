//! 🗂️ `remove-layer-asset` — detaches an id-addressed `RasterImageAsset` from the document's asset
//! map. Inverse partner of `add-layer-asset` (see that leaf for why this pair exists).

pub mod mutation {
use serde::{Deserialize, Serialize};
use crate::artifacts::raster::diff::{diff_remove_asset, RasterDiff};
use crate::artifacts::raster::mutations::{add_layer_asset, RasterMutation};
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️RemoveLayerAsset
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf, Serialize, Deserialize)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RemoveLayerAsset {
    pub asset_id: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for RemoveLayerAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "asset", kind: "remove-layer-asset", record: "RemovedLayerAsset" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove asset {}", self.asset_id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.asset_id.clone()]
    }
}
//#endregion 🔖️RemoveLayerAsset
}

pub use mutation::RemoveLayerAsset;
