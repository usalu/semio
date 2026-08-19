//! 🖼️ 🖼️ Raster play app commands command — `toggle-layer-visible`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::schema::{find_layer, layer_visible};
use crate::artifacts::raster::mutations::change_layer_visible;
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "toggle-layer-visible")]
pub struct ToggleLayerVisible {
    pub layer_id: String,
}

pub async fn handle(payload: &ToggleLayerVisible, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let Some(layer) = find_layer(&document.layers, &payload.layer_id) else { return Ok(Emit::default()) };
    let resolved = !layer_visible(layer);
    Ok(Emit::mutations(vec![RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: payload.layer_id.clone(), new_visible: resolved })]))
}
