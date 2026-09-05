//! ⚙️ ⚙️ Remodeling play app commands command — `set-sfm-params`.

use crate::artifacts::remodeling::mutations::update_sfm_params;
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::{RemodelingSnapshot, RobustLossKind, SfmParams};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "sfm-params")]
pub struct SetSfmParams {
    pub ransac_iterations: u32,
    pub ransac_threshold_px: f32,
    pub min_track_length: u32,
    pub ba_max_iterations: u32,
    pub robust_loss: String,
    pub huber_delta_px: f32,
}

pub async fn handle(payload: &SetSfmParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_sfm_params(SfmParams {
        ransac_iterations: payload.ransac_iterations,
        ransac_threshold_px: payload.ransac_threshold_px,
        min_track_length: payload.min_track_length,
        ba_max_iterations: payload.ba_max_iterations,
        robust_loss: match payload.robust_loss.as_str() {
            "l2" => RobustLossKind::L2,
            "cauchy" => RobustLossKind::Cauchy,
            _ => RobustLossKind::Huber,
        },
        huber_delta_px: payload.huber_delta_px,
    })]))
}
