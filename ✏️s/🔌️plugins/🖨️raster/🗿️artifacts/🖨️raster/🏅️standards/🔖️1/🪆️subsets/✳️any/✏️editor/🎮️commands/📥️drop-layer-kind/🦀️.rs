//! 🖼️ 🖼️ Raster play app commands command — `drop-layer-kind`.

use crate::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_mutations_layers::create_layer;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::create_layer_of_kind;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "drop-layer-kind")]
pub struct DropLayerKind {
    pub kind: String,
}

/// 🖱️ Creates a layer and selects it through the framework interaction lane after publication.
pub fn handle(payload: &DropLayerKind, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_of_kind(&payload.kind);
    let selection=crate::layer_selection::select_layer_effect(semio_s_artifact_raster_raster_model::layer_node_id(&layer))?;
    Ok(Emit { artifact_mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: document.layers.len(), layer: Box::new(layer) })], effects:vec![selection], ..Default::default() })
}
