//! 🖱️ 🖱️ S Studio app command — `node-graph-viewport`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation, SpaceWindowCamera};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    pub viewport_json: String,
}

pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    match pack::parse_json(&payload.viewport_json).ok().and_then(|value| dsl::from_dsl_value::<SpaceWindowCamera>(pack::json_to_dsl_value(&value)).ok()) {
        Some(camera) => Ok(Emit::config(vec![SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera }])),
        None => Ok(Emit::default()),
    }
}
