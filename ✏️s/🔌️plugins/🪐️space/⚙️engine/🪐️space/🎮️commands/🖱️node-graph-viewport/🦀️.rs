//! 🖱️ 🖱️ S Studio app command — `node-graph-viewport`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation, SpaceWindowCamera};
use semio_framework_os::{Viewport2d, WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    #[dsl(block)]
    pub viewport: Viewport2d,
}

pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let viewport = payload.viewport;
    let camera = SpaceWindowCamera { x: viewport.x, y: viewport.y, zoom: viewport.zoom };
    Ok(Emit::config(vec![SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera }]))
}
