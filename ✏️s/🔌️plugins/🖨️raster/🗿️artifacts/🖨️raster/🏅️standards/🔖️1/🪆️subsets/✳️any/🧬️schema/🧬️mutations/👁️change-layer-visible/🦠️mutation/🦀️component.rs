//! 👁️ `change-layer-visible` — sets an id-addressed layer's `visible` scalar.

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLayerVisible
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLayerVisible {
    pub layer_id: String,
    pub new_visible: bool,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerVisible {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-visible", record: "ChangedLayerVisible" };

    async fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        crate::artifacts::raster::mutations::change_layer_visible::diff::diff(self, base)
    }

    async fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::change_layer_visible::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Set layer {} visible to {}", self.layer_id, self.new_visible)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ChangeLayerVisible
