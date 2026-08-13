//! 🗂️ 🗂️ Raster play app commands command — `set-hover`.

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::schema::{flatten_raster_layers, layer_node_id};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-hover")]
pub struct SetHover {
    pub id: Option<String>,
}

pub fn handle(payload: &SetHover, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetHovered { id: payload.id.clone() }]))
}
