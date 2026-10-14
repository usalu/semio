//! 🖼️ 🖼️ Raster play app commands command — `delete-layer`.

use crate::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_mutations_layers::delete_layer as layer_delete;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: String,
}

/// 🕹️ Deletes the layer; framework topology validation prunes its selection after publication.
pub fn handle(payload: &DeleteLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    semio_s_artifact_raster_raster_model::require_layer_edit(&document.layers,&payload.layer_id,true).map_err(Fault::from)?;
    if find_layer(&document.layers, &payload.layer_id).is_none() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![RasterMutation::DeleteLayer(layer_delete::mutation::DeleteLayer { layer_id: payload.layer_id.clone() })], ..Default::default() })
}
