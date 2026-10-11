//! 🧩️ 🧩️ S Studio app command — `spawn-app`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation, ActiveNodeSetting};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "spawn-app")]
pub struct SpawnApp {
    pub plugin_id: String,
    pub app_id: String,
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &SpawnApp, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    match crate::engine::space::engine::resolve_future(crate::engine::space::engine::add_workflow_node_operation(&payload.plugin_id, &payload.app_id, None, payload.x, payload.y)) {
        Some((operation, node_id)) => Ok(Emit { artifact_mutations: vec![operation], config_mutations: vec![SpaceConfigMutation::SetActiveNode(ActiveNodeSetting { node_id: Some(node_id) })], ..Default::default() }),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
