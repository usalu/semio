//! 🖌️ Raster play app commands — live brush controls (view actions, never a document operation).

use crate::apps::raster::config::{RasterConfig, RasterConfigOperation};
use crate::artifacts::raster::op::RasterOperation;
use crate::artifacts::raster::RasterProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetBrushSize
pub mod set_brush_size {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "brush-size")]
    pub struct SetBrushSize {
        pub value: f64,
    }

    pub fn handle(payload: &SetBrushSize, _doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterOperation, RasterConfigOperation>, Fault> {
        Ok(Emit::config(vec![RasterConfigOperation::SetBrushSize { value: payload.value }]))
    }
}
//#endregion 🔖️SetBrushSize

//#region 🔖️SetBrushOpacity
pub mod set_brush_opacity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "brush-opacity")]
    pub struct SetBrushOpacity {
        pub value: f64,
    }

    pub fn handle(payload: &SetBrushOpacity, _doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterOperation, RasterConfigOperation>, Fault> {
        Ok(Emit::config(vec![RasterConfigOperation::SetBrushOpacity { value: payload.value }]))
    }
}
//#endregion 🔖️SetBrushOpacity
