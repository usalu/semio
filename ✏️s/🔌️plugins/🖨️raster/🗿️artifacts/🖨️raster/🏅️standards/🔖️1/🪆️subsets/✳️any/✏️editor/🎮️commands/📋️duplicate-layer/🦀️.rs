//! 🖼️ 🖼️ Raster play app commands command — `duplicate-layer`.

use crate::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_mutations_layers::create_layer;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::{clone_layer, find_layer};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub layer_id: String,
}

/// 📑️ Creates a sibling copy and requests framework selection after publication.
pub fn handle(payload: &DuplicateLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document=doc.snapshot;
    let (parent_id,index)=crate::duplicate_layer_plan(document,&payload.layer_id)?;
    let source=find_layer(&document.layers,&payload.layer_id).ok_or_else(||Fault::from("raster-layer-not-found"))?;
    let layer=clone_layer(source);
    let selection=crate::layer_selection::select_layer_effect(semio_s_artifact_raster_raster_model::layer_node_id(&layer))?;
    Ok(Emit {artifact_mutations:vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {parent_id,index,layer:Box::new(layer)})],effects:vec![selection],..Default::default()})
}

