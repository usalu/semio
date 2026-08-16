//! 🎥️ 🎥️ Raster play app commands command — `set-composite-viewport`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation, RasterConfigViewportSize};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterCamera, RasterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "composite-viewport")]
pub struct SetCompositeViewport {
    pub width: f64,
    pub height: f64,
}

pub fn handle(payload: &SetCompositeViewport, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetCompositeViewport { viewport: Some(RasterConfigViewportSize { width: payload.width, height: payload.height }) }]))
}
