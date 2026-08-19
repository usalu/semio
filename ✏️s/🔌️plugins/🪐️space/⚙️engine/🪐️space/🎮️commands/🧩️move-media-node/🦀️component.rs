//! 🧩️ 🧩️ S Studio app command — `move-media-node`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "move-media-node")]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

pub async fn handle(payload: &MoveMediaNode, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::amend(vec![WorkflowMutation::MoveNode { node_id: payload.node_id.clone(), x: payload.x, y: payload.y }], format!("moveMediaNode:{}", payload.node_id)))
}
