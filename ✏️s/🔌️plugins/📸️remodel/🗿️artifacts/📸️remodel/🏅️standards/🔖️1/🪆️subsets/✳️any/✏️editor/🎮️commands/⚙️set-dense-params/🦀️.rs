//! ⚙️ ⚙️ Remodel play app commands command — `set-dense-params`.

use crate::artifacts::remodel::mutations::update_dense_params;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{DenseParams, DenseResolution, RemodelSnapshot};
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "dense-params")]
pub struct SetDenseParams {
    pub resolution: String,
    pub window_radius_px: u32,
    pub min_view_consistency: u32,
    pub confidence_threshold: f32,
    pub max_points: u32,
}

pub async fn handle(payload: &SetDenseParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_dense_params(DenseParams {
        resolution: match payload.resolution.as_str() {
            "low" => DenseResolution::Low,
            "high" => DenseResolution::High,
            _ => DenseResolution::Medium,
        },
        window_radius_px: payload.window_radius_px,
        min_view_consistency: payload.min_view_consistency,
        confidence_threshold: payload.confidence_threshold,
        max_points: payload.max_points,
    })]))
}
