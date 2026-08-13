//! 🗂️ 🗂️ Raster play app commands command — `set-selection`.

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::schema::{flatten_raster_layers, layer_node_id};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selection")]
pub struct SetSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetSelection { ids: payload.ids.clone() }]))
}
