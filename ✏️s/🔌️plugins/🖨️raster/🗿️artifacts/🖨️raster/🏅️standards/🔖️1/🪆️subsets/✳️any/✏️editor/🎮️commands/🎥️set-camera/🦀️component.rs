//! 🎥️ 🎥️ Raster play app commands command — `set-camera`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation, RasterConfigViewportSize};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterCamera, RasterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: RasterCamera,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetCamera { camera: payload.camera.clone() }]))
}
