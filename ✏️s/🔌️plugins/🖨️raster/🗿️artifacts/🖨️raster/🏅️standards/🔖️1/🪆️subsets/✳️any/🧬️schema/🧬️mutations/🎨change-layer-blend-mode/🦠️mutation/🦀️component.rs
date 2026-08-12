//! 🎨 `change-layer-blend-mode` — sets an id-addressed layer's `blend_mode` scalar.

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLayerBlendMode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLayerBlendMode {
    pub layer_id: String,
    pub new_blend_mode: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerBlendMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-blend-mode", record: "ChangedLayerBlendMode" };

    fn diff(&self, base: &RasterSnapshot) -> RasterDiff {
        crate::artifacts::raster::mutations::change_layer_blend_mode::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::change_layer_blend_mode::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Set layer {} blend mode to {}", self.layer_id, self.new_blend_mode)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ChangeLayerBlendMode
