//! 🎥️ Raster play app commands — composite/navigator viewport + camera (view actions, never a document
//! operation — the camera is session-only runtime pose).

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation, RasterConfigViewportSize};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterCamera, RasterSnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCompositeViewport
pub mod set_composite_viewport {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "composite-viewport")]
    pub struct SetCompositeViewport {
        pub width: f64,
        pub height: f64,
    }

    pub fn handle(payload: &SetCompositeViewport, _doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        Ok(Emit::config(vec![RasterConfigMutation::SetCompositeViewport { viewport: Some(RasterConfigViewportSize { width: payload.width, height: payload.height }) }]))
    }
}
//#endregion 🔖️SetCompositeViewport

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: RasterCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        Ok(Emit::config(vec![RasterConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🔖️SetCameraZoom
pub mod set_camera_zoom {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera-zoom")]
    pub struct SetCameraZoom {
        pub zoom: f64,
    }

    pub fn handle(payload: &SetCameraZoom, _doc: &DocumentView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let camera = RasterCamera { zoom: payload.zoom, ..cfg.snapshot.camera.clone() };
        Ok(Emit::config(vec![RasterConfigMutation::SetCamera { camera }]))
    }
}
//#endregion 🔖️SetCameraZoom
