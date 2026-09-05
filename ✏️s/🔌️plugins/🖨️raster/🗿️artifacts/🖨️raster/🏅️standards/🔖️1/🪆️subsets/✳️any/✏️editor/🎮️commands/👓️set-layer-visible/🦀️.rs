//! 🖼️ 🖼️ Raster play app commands command — `set-layer-visible`.

use crate::artifacts::raster::mutations::change_layer_visible;
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_visible};
use crate::artifacts::raster::RasterSnapshot;
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-layer-visible")]
pub struct SetLayerVisible {
    pub layer_id: String,
    pub visible: Option<bool>,
}

pub fn handle(payload: &SetLayerVisible, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let Some(layer) = find_layer(&document.layers, &payload.layer_id) else { return Ok(Emit::default()) };
    let resolved = payload.visible.unwrap_or_else(|| !layer_visible(layer));
    Ok(Emit::mutations(vec![RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: payload.layer_id.clone(), new_visible: resolved })]))
}
