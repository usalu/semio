//! 🖌️ 🖌️ Raster play app commands command — `set-brush-hardness`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "brush-hardness")]
pub struct SetBrushHardness {
    pub value: f64,
}

pub fn handle(payload: &SetBrushHardness, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    if !payload.value.is_finite() || !(0.0..=1.0).contains(&payload.value) { return Err(Fault::from("raster-brush-hardness-invalid")); }
    Ok(Emit::config(vec![RasterConfigMutation::SetBrushHardness { value: payload.value }]))
}
