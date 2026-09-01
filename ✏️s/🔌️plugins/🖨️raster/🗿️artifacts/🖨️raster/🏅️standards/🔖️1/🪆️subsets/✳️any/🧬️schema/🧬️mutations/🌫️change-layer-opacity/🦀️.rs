//! 🌫️ `change-layer-opacity` — sets an id-addressed layer's `opacity` scalar.

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::change_layer_opacity::ChangeLayerOpacity;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_opacity};
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLayerOpacity
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLayerOpacity {
    pub layer_id: String,
    pub new_opacity: f32,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerOpacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-opacity", record: "ChangedLayerOpacity" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Set layer {} opacity to {}", self.layer_id, self.new_opacity)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ChangeLayerOpacity
