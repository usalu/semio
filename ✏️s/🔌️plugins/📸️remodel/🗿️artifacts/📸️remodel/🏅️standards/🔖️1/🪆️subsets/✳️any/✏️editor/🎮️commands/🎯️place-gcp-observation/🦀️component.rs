//! 🎯️ 🎯️ Remodel play app commands command — `place-gcp-observation`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::add_gcp_observation;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{GcpObservation, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "place-gcp-observation")]
pub struct PlaceGcpObservation {
    pub gcp_id: String,
    pub stream_id: String,
    pub frame_index: u32,
    pub pixel_x: f32,
    pub pixel_y: f32,
}

pub async fn handle(payload: &PlaceGcpObservation, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    if !doc.snapshot.gcps.iter().any(|gcp| gcp.id == payload.gcp_id) {
        return Ok(Emit::default());
    }
    let observation = GcpObservation { stream_id: payload.stream_id.clone(), frame_index: payload.frame_index, pixel: [payload.pixel_x, payload.pixel_y] };
    Ok(Emit::mutations(vec![add_gcp_observation(payload.gcp_id.clone(), observation)]))
}
