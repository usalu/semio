//! 🎨 `change-layer-blend-mode` — sets an id-addressed layer's `blend_mode` scalar.

pub mod mutation {
use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_blend_mode};
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️ChangeLayerBlendMode
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerBlendMode {
    pub layer_id: String,
    pub new_blend_mode: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerBlendMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-blend-mode", record: "ChangedLayerBlendMode" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Set layer {} blend mode to {}", self.layer_id, self.new_blend_mode)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ChangeLayerBlendMode
}

pub use mutation::ChangeLayerBlendMode;
