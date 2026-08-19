//! 🖼️ 🖼️ Raster play app commands command — `delete-layer`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::mutations::delete_layer as layer_delete;
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: String,
}

/// 🕹️ No longer prunes selection here (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
/// the `"layers"` domain's selection is framework-owned `InteractionState` now, not this config —
/// `RasterConfigMutation::SetSelection` is deleted. `"layers"` is declared `HierarchyProvider::Flat`,
/// so a deleted-but-still-selected id is a documented, framework-level gap (Flat domains are never
/// auto-pruned by `validate_state`; see the ticket's `w3b-summary.md`), not something this command
/// can restore without re-declaring the domain as `Topology`.
pub async fn handle(payload: &DeleteLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    if find_layer(&document.layers, &payload.layer_id).is_none() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![RasterMutation::DeleteLayer(layer_delete::mutation::DeleteLayer { layer_id: payload.layer_id.clone() })], ..Default::default() })
}
