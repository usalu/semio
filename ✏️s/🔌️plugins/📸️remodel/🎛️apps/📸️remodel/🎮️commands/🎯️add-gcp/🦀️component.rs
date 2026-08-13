//! 🎯️ 🎯️ Remodel play app commands command — `add-gcp`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::schema::next_remodel_id;
use crate::artifacts::remodel::mutations::{add_gcp_observation, create_camera_calibration, create_gcp, delete_gcp, update_camera_calibration};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{CameraCalibration, GcpObservation, GroundControlPoint, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-gcp")]
pub struct AddGcp {
    pub name: String,
    pub world_x: f64,
    pub world_y: f64,
    pub world_z: f64,
}

pub fn handle(payload: &AddGcp, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let id = next_remodel_id("gcp");
    let gcp = GroundControlPoint { id, name: payload.name.clone(), world_position: [payload.world_x, payload.world_y, payload.world_z], observations: Vec::new() };
    Ok(Emit::mutations(vec![create_gcp(gcp)]))
}
