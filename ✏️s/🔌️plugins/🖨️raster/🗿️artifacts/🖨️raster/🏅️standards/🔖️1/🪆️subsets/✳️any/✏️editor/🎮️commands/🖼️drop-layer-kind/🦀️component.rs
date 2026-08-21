//! 🖼️ 🖼️ Raster play app commands command — `drop-layer-kind`.

use crate::artifacts::raster::mutations::create_layer;
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::schema::create_layer_of_kind;
use crate::artifacts::raster::RasterSnapshot;
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "drop-layer-kind")]
pub struct DropLayerKind {
    pub kind: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the newly-dropped layer used to also
/// select itself here — the `"layers"` domain's selection is framework-owned `InteractionState` now,
/// only ever mutated by the framework's own injected `interactionSelect` handling, never by an app
/// command's `Emit::config_mutations`.
pub async fn handle(payload: &DropLayerKind, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_of_kind(&payload.kind);
    Ok(Emit { artifact_mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: document.layers.len(), layer: Box::new(layer) })], ..Default::default() })
}
