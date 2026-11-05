//! 📐️ Stages intrinsic image dimensions and a crop origin without editing layer display geometry.
use crate::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "image-grid")]
pub struct SetImageGrid { pub field: String, pub value: String }

pub fn handle(payload: &SetImageGrid, _doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let invalid = || crate::commands::paint_stroke::raster_fault("raster.transform.invalid");
    let mut grid = cfg.snapshot.image_grid.clone();
    match payload.field.as_str() {
        "bilinear" => grid.bilinear = payload.value.parse::<bool>().map_err(|_| invalid())?,
        field => {
            let value = payload.value.parse::<u32>().map_err(|_| invalid())?;
            match field { "x" => grid.x = value, "y" => grid.y = value, "width" => grid.width = value, "height" => grid.height = value, _ => return Err(invalid()) }
        }
    }
    if !grid.valid() { return Err(invalid()); }
    Ok(Emit::config(vec![RasterConfigMutation::SetImageGrid(grid)]))
}
