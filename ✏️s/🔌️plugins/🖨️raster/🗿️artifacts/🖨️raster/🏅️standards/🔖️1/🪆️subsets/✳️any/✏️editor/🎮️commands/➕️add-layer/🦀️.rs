//! 🖼️ 🖼️ Raster play app commands command — `add-layer`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::create_layer;
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::create_layer_of_kind;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-layer")]
pub struct AddLayer {
    pub kind: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the newly-added layer used to also
/// select itself here — the `"layers"` domain's selection is framework-owned `InteractionState` now,
/// only ever mutated by the framework's own injected `interactionSelect` handling, never by an app
/// command's `Emit::config_mutations`.
pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_of_kind(&payload.kind);
    Ok(Emit { artifact_mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: document.layers.len(), layer: Box::new(layer) })], ..Default::default() })
}
