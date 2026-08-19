//! 🎥️ 🎥️ Raster play app commands command — `set-camera-zoom`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterCamera, RasterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "camera-zoom")]
pub struct SetCameraZoom {
    pub zoom: f64,
}

pub async fn handle(payload: &SetCameraZoom, _doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let camera = RasterCamera { zoom: payload.zoom, ..cfg.snapshot.camera.clone() };
    Ok(Emit::config(vec![RasterConfigMutation::SetCamera { camera }]))
}
