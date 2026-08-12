//! 🌱 `create-layer` — brings a new `RasterLayerNode` into existence at a tree address.

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateLayer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLayer {
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: Box<RasterLayerNode>,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for CreateLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };

    fn diff(&self, base: &RasterSnapshot) -> RasterDiff {
        crate::artifacts::raster::mutations::create_layer::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::create_layer::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create layer \"{}\"", crate::artifacts::raster::schema::layer_name(&self.layer))
    }

    fn target(&self) -> Vec<String> {
        vec![crate::artifacts::raster::schema::layer_node_id(&self.layer).to_string()]
    }
}
//#endregion 🔖️CreateLayer
