//! 🗂️ 🗂️ Raster play app commands command — `select-all`.

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::schema::{flatten_raster_layers, layer_node_id};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "select-all")]
pub struct SelectAll {}

pub fn handle(_payload: &SelectAll, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let ids = flatten_raster_layers(&doc.snapshot.layers).into_iter().map(|layer| layer_node_id(layer).to_string()).collect();
    Ok(Emit::config(vec![RasterConfigMutation::SetSelection { ids }]))
}
