//! 🖼️ 🖼️ Raster play app commands command — `duplicate-layer`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::schema::{clone_layer, find_layer};
use crate::artifacts::raster::mutations::create_layer;
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub layer_id: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the duplicated layer used to also
/// select itself here — the `"layers"` domain's selection is framework-owned `InteractionState` now,
/// only ever mutated by the framework's own injected `interactionSelect` handling, never by an app
/// command's `Emit::config_mutations`.
pub async fn handle(payload: &DuplicateLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    match find_layer(&document.layers, &payload.layer_id) {
        Some(layer) => {
            let copy = clone_layer(layer);
            Ok(Emit {
                artifact_mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: document.layers.len(), layer: Box::new(copy) })],
                ..Default::default()
            })
        }
        None => Ok(Emit::default()),
    }
}
