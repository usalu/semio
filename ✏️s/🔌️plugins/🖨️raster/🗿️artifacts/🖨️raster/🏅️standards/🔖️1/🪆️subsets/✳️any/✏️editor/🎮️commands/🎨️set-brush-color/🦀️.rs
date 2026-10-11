//! 🖌️ 🖌️ Raster play app commands command — `set-brush-color`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation, SetBrushColorEdit};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "brush-color")]
pub struct SetBrushColor {
    pub value: String,
}

pub fn handle(payload: &SetBrushColor, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    if !crate::editor::raster::config::valid_brush_color(&payload.value) { return Err(crate::editor::raster::commands::paint_stroke::raster_fault("raster.brush.color-invalid")); }
    Ok(Emit::config(vec![RasterConfigMutation::SetBrushColor(SetBrushColorEdit { value: payload.value.clone() })]))
}
