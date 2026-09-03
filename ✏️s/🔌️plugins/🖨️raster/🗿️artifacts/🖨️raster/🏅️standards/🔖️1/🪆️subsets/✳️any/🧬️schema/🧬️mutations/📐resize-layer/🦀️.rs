//! 📐 `resize-layer` — changes an id-addressed `Pixel` layer's `width`/`height` extent. `width`/
//! `height` only exist on the `Pixel` variant; addressing a `Group`/`Adjustment` layer is a graceful
//! no-op (`RasterDiff::default()` / `Vec::new()`), never a panic.

pub mod mutation {
use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️ResizeLayer
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeLayer {
    pub layer_id: String,
    pub new_width: u32,
    pub new_height: u32,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ResizeLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "layer", kind: "resize-layer", record: "ResizedLayer" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Resize layer {} to {}x{}", self.layer_id, self.new_width, self.new_height)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ResizeLayer
}

pub use mutation::ResizeLayer;
