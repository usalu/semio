//! ⚙️ ⚙️ Remodeling play app commands command — `set-dense-params`.

use crate::artifacts::remodeling::mutations::update_dense_params;
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::{DenseParams, DenseResolution, RemodelingSnapshot};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
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

pub async fn handle(payload: &SetDenseParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
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
