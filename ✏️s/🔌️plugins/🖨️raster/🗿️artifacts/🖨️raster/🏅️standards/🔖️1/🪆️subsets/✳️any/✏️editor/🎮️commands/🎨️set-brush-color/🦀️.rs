//! 🖌️ 🖌️ Raster play app commands command — `set-brush-color`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "brush-color")]
pub struct SetBrushColor {
    pub value: String,
}

pub fn handle(payload: &SetBrushColor, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    if !crate::editor::raster::config::valid_brush_color(&payload.value) { return Err(Fault::from("raster-brush-color-invalid")); }
    Ok(Emit::config(vec![RasterConfigMutation::SetBrushColor { value: payload.value.clone() }]))
}
