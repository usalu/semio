//! 🖌️ 🖌️ Raster play app commands command — `set-brush-opacity`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "brush-opacity")]
pub struct SetBrushOpacity {
    pub value: f64,
}

pub async fn handle(payload: &SetBrushOpacity, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetBrushOpacity { value: payload.value }]))
}
