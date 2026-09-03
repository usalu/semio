//! 👁️ `change-layer-visible` — sets an id-addressed layer's `visible` scalar.

pub mod mutation {
use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_visible};
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️ChangeLayerVisible
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerVisible {
    pub layer_id: String,
    pub new_visible: bool,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerVisible {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-visible", record: "ChangedLayerVisible" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Set layer {} visible to {}", self.layer_id, self.new_visible)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ChangeLayerVisible
}

pub use mutation::ChangeLayerVisible;
