//! 🖇️ `add-layer-asset` — attaches an embedded `RasterImageAsset` to the document's id-keyed asset
//! map. NOT one of the coordinator's ten mandated derivations; added so `image:in` media import
//! (`crate::apps::raster::wasm`/the app's `import_media`) can stay a real, undoable operation now
//! that `SetSnapshot` is gone — `assets: BTreeMap<String, RasterImageAsset>` is itself an id-keyed
//! root collection per `📓️derivation-rules.md` rule 2, and `add`/`remove` is its taxonomy-correct
//! verb pair (`add`: "Attach a set-like member … inverse: remove").

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::{RasterImageAsset, RasterSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️AddLayerAsset
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLayerAsset {
    pub asset_id: String,
    pub asset: RasterImageAsset,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for AddLayerAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "asset", kind: "add-layer-asset", record: "AddedLayerAsset" };

    fn diff(&self, base: &RasterSnapshot) -> RasterDiff {
        crate::artifacts::raster::mutations::add_layer_asset::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::add_layer_asset::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add asset {}", self.asset_id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.asset_id.clone()]
    }
}
//#endregion 🔖️AddLayerAsset
