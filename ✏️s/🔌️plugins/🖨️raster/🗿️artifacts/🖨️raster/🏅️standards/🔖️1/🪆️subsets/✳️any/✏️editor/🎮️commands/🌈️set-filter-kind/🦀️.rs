//! 🌈️ Chooses a session filter and its valid initial amount without editing the document.
use crate::config::{RasterConfig, RasterConfigMutation, SetFilterOptionsEdit};
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "filter-kind")]
pub struct SetFilterKind { pub value: String }

pub fn handle(payload: &SetFilterKind, _doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let Some((_, range)) = semio_s_artifact_raster_raster_mutations_pixels::apply_filter::RASTER_FILTERS.iter().find(|(kind, _)| *kind == payload.value) else { return Err(crate::commands::paint_stroke::raster_fault("raster.filter.invalid")); };
    let amount = if cfg.snapshot.filter_kind == payload.value { cfg.snapshot.filter_amount } else {
        match payload.value.as_str() { "gamma" | "sharpen" | "blur" => 1.0, "threshold" => 128.0, "posterize" => 4.0, _ => range.map_or(0.0, |(min, max)| 0.0f64.clamp(min, max)) }
    };
    Ok(Emit::config(vec![RasterConfigMutation::SetFilterOptions(SetFilterOptionsEdit { filter: payload.value.clone(), amount })]))
}
