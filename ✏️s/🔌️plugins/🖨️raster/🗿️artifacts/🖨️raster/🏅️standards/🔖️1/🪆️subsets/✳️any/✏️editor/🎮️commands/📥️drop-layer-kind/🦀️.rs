//! 🖼️ 🖼️ Raster play app commands command — `drop-layer-kind`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::create_layer;
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::create_layer_of_kind;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "drop-layer-kind")]
pub struct DropLayerKind {
    pub kind: String,
}

/// 🖱️ Creates a layer and selects it through the framework interaction lane after publication.
pub fn handle(payload: &DropLayerKind, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_of_kind(&payload.kind);
    let selection=crate::editor::raster::layer_selection::select_layer_effect(crate::standards::v1::subsets::any::schema::layer_node_id(&layer))?;
    Ok(Emit { artifact_mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: document.layers.len(), layer: Box::new(layer) })], effects:vec![selection], ..Default::default() })
}
