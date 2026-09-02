//! 🖇️ `add-layer-asset` — attaches a real `RasterImageAsset` (event-log content) to the document's
//! id-keyed asset collection. NOT one of the coordinator's ten mandated derivations; added so
//! `image:in` media import (`crate::editor::raster::wasm`/the app's `import_media`) can stay a real,
//! undoable operation now that whole-document replace is gone — `assets: BTreeMap<String,
//! store::ArtifactChild<SemioImageSnapshot>>` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`:
//! the persisted document field holds only a composed-child HANDLE; this mutation's own payload still
//! carries the real bytes, minted into a handle + working-scene cache entry at apply time via
//! `crate::artifacts::raster::mint_raster_asset_child`) is itself an id-keyed root collection per
//! `📓️derivation-rules.md` rule 2, and `add`/`remove` is its taxonomy-correct verb pair (`add`:
//! "Attach a set-like member … inverse: remove").

pub mod mutation {
use serde::{Deserialize, Serialize};
use crate::artifacts::raster::diff::{diff_add_asset, RasterDiff};
use crate::artifacts::raster::mutations::{remove_layer_asset, RasterMutation};
use crate::artifacts::raster::{RasterImageAsset, RasterSnapshot};

//#region 🔖️AddLayerAsset
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf, Serialize, Deserialize)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct AddLayerAsset {
    pub asset_id: String,
    pub asset: RasterImageAsset,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for AddLayerAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "asset", kind: "add-layer-asset", record: "AddedLayerAsset" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add asset {}", self.asset_id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.asset_id.clone()]
    }
}
//#endregion 🔖️AddLayerAsset
}

pub use mutation::AddLayerAsset;
