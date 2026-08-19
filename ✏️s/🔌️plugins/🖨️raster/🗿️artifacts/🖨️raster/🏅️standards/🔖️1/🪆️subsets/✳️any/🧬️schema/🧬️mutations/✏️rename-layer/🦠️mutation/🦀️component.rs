//! ✏️ `rename-layer` — changes an id-addressed layer's identity field (`name`).

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️RenameLayer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameLayer {
    pub layer_id: String,
    pub new_name: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for RenameLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "layer", kind: "rename-layer", record: "RenamedLayer" };

    async fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        crate::artifacts::raster::mutations::rename_layer::diff::diff(self, base)
    }

    async fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::rename_layer::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Rename layer {} to \"{}\"", self.layer_id, self.new_name)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️RenameLayer
