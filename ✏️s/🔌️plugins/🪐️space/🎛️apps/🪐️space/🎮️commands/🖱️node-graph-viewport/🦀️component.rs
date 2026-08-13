//! 🖱️ 🖱️ S Studio app command — `node-graph-viewport`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{OsWorkflowCamera, WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    pub viewport_json: String,
}

pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    match serde_json::from_str::<OsWorkflowCamera>(&payload.viewport_json) {
        Ok(camera) => Ok(Emit::config(vec![SpaceConfigMutation::SetCamera { window_id: crate::apps::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }])),
        Err(_) => Ok(Emit::default()),
    }
}
