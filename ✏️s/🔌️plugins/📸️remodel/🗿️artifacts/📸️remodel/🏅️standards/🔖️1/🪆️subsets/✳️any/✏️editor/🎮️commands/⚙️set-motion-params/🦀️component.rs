//! ⚙️ ⚙️ Remodel play app commands command — `set-motion-params`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::update_motion_params;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{MotionParams, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "motion-params")]
pub struct SetMotionParams {
    pub enabled: bool,
    pub max_tracks: u32,
    pub track_window_px: u32,
    pub min_track_quality: f32,
    pub min_track_length_frames: u32,
}

pub async fn handle(payload: &SetMotionParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_motion_params(MotionParams {
        enabled: payload.enabled,
        max_tracks: payload.max_tracks,
        track_window_px: payload.track_window_px,
        min_track_quality: payload.min_track_quality,
        min_track_length_frames: payload.min_track_length_frames,
    })]))
}
