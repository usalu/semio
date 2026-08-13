//! 🎯️ 🎯️ Remodel play app commands command — `remove-gcp`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::schema::next_remodel_id;
use crate::artifacts::remodel::mutations::{add_gcp_observation, create_camera_calibration, create_gcp, delete_gcp, update_camera_calibration};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{CameraCalibration, GcpObservation, GroundControlPoint, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-gcp")]
pub struct RemoveGcp {
    pub gcp_id: String,
}

pub fn handle(payload: &RemoveGcp, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_gcp(payload.gcp_id.clone())]))
}
