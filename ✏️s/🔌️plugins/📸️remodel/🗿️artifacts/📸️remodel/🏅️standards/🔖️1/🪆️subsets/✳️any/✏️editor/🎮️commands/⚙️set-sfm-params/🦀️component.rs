//! ⚙️ ⚙️ Remodel play app commands command — `set-sfm-params`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::update_sfm_params;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{RemodelSnapshot, RobustLossKind, SfmParams};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "sfm-params")]
pub struct SetSfmParams {
    pub ransac_iterations: u32,
    pub ransac_threshold_px: f32,
    pub min_track_length: u32,
    pub ba_max_iterations: u32,
    pub robust_loss: String,
    pub huber_delta_px: f32,
}

pub fn handle(payload: &SetSfmParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
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
