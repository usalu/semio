//! 🖌️ 🖌️ Raster play app commands command — `set-brush-size`.

use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "brush-size")]
pub struct SetBrushSize {
    pub value: f64,
}

pub fn handle(payload: &SetBrushSize, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetBrushSize { value: payload.value }]))
}
