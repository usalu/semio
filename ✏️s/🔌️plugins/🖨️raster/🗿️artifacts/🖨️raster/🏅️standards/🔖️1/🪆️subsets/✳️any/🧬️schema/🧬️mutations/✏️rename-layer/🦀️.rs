//! ✏️ `rename-layer` — changes an id-addressed layer's identity field (`name`).

pub mod mutation {
use serde::{Deserialize, Serialize};
use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_name};
use crate::artifacts::raster::{RasterLayerPatch, RasterSnapshot};

//#region 🔖️RenameLayer
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf, Serialize, Deserialize)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RenameLayer {
    pub layer_id: String,
    pub new_name: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for RenameLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "layer", kind: "rename-layer", record: "RenamedLayer" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        super::super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        super::super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename layer {} to \"{}\"", self.layer_id, self.new_name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️RenameLayer
}

pub use mutation::RenameLayer;
