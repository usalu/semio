//! 🎥️ 🎥️ Raster play app commands command — `set-camera-zoom`.

use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterCamera, RasterSnapshot};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera-zoom")]
pub struct SetCameraZoom {
    pub zoom: f64,
}

pub fn handle(payload: &SetCameraZoom, _doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let camera = RasterCamera { zoom: payload.zoom, ..cfg.snapshot.camera.clone() };
    Ok(Emit::config(vec![RasterConfigMutation::SetCamera { camera }]))
}
