//! 🖼️ 🖼️ Raster play app commands command — `set-layer-visible`.

use crate::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_mutations_layers::change_layer_visible;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::{find_layer, layer_visible};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
