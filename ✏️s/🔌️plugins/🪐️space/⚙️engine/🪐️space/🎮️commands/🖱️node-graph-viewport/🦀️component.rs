//! 🖱️ 🖱️ S Studio app command — `node-graph-viewport`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{OsWorkflowCamera, WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    pub viewport_json: String,
}

pub async fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    match pack::from_json_str::<OsWorkflowCamera>(&payload.viewport_json) {
        Ok(camera) => Ok(Emit::config(vec![SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }])),
        Err(_) => Ok(Emit::default()),
    }
}
