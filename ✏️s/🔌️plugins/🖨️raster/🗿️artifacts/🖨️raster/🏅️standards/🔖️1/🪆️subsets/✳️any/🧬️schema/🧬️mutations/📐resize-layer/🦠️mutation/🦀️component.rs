//! 📐 `resize-layer` — changes an id-addressed `Pixel` layer's `width`/`height` extent. `width`/
//! `height` only exist on the `Pixel` variant; addressing a `Group`/`Adjustment` layer is a graceful
//! no-op (`RasterDiff::default()` / `Vec::new()`), never a panic.

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ResizeLayer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizeLayer {
    pub layer_id: String,
    pub new_width: u32,
    pub new_height: u32,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ResizeLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "layer", kind: "resize-layer", record: "ResizedLayer" };

    async fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        crate::artifacts::raster::mutations::resize_layer::diff::diff(self, base)
    }

    async fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::resize_layer::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Resize layer {} to {}x{}", self.layer_id, self.new_width, self.new_height)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ResizeLayer
