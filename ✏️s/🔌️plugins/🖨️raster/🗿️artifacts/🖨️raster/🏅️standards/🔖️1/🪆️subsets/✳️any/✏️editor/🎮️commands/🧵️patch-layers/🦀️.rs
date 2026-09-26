//! 🖼️ 🖼️ Raster play app commands command — `patch-layers`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

use super::patch_layer::{patch_value_json, raster_patch_layer_operations};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-layers")]
pub struct PatchLayers {
    pub layer_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchLayers, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let json_value = patch_value_json(&payload.field, &payload.value);
    let operations = raster_patch_layer_operations(doc.snapshot, &payload.layer_ids, &payload.field, &json_value)?;
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}
