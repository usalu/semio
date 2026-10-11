//! 🖌️ 🖌️ Raster play app commands command — `set-brush-opacity`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation, SetBrushOpacityEdit};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "brush-opacity")]
pub struct SetBrushOpacity {
    pub value: f64,
}

pub fn handle(payload: &SetBrushOpacity, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetBrushOpacity(SetBrushOpacityEdit { value: payload.value })]))
}
