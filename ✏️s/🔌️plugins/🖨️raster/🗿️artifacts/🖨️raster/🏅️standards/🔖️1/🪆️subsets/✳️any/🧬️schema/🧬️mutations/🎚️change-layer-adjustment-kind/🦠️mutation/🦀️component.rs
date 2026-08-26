//! 🎚️ `change-layer-adjustment-kind` — changes an id-addressed `Adjustment` layer's
//! `adjustment_kind` scalar. Only meaningful on the `Adjustment` variant; addressing a
//! `Pixel`/`Group` layer is a graceful no-op.

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLayerAdjustmentKind
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLayerAdjustmentKind {
    pub layer_id: String,
    pub new_adjustment_kind: String,
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerAdjustmentKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-adjustment-kind", record: "ChangedLayerAdjustmentKind" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        crate::artifacts::raster::mutations::change_layer_adjustment_kind::diff::diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        crate::artifacts::raster::mutations::change_layer_adjustment_kind::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Set layer {} adjustment kind to {}", self.layer_id, self.new_adjustment_kind)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️ChangeLayerAdjustmentKind
