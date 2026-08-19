//! 🔗️ 🔗️ S Studio app command — `disconnect-media-edge`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "disconnect-media-edge")]
pub struct DisconnectMediaEdge {
    pub edge_id: String,
}

pub async fn handle(payload: &DisconnectMediaEdge, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![WorkflowMutation::DisconnectEdge { edge_id: payload.edge_id.clone() }]))
}
