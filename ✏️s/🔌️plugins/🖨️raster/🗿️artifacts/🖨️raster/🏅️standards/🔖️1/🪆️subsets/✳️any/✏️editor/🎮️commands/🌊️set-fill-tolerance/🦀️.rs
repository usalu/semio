//! 🌊️ Sets the session colour tolerance every bucket click floods with (0..255).
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::{RasterMutation, RasterSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "fill-tolerance")]
pub struct SetFillTolerance {
    pub value: u32,
}

pub fn handle(payload: &SetFillTolerance, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    if payload.value > 255 {
        return Err(crate::editor::raster::commands::paint_stroke::raster_fault("raster.fill.tolerance-invalid"));
    }
    Ok(Emit::config(vec![RasterConfigMutation::SetFillTolerance { value: payload.value }]))
}
