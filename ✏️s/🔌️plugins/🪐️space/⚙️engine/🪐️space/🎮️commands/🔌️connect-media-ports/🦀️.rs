//! 🔗️ 🔗️ S Studio app command — `connect-media-ports`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "connect-media-ports")]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    match crate::engine::space::engine::resolve_future(crate::engine::space::negotiate_connect_or_notify(doc.snapshot, &payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id)) {
        Ok(contract) => Ok(Emit::mutations(vec![crate::engine::space::engine::resolve_future(crate::engine::space::connect_edge_operation(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id, contract))])),
        Err(effect) => Ok(Emit::effect(effect)),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
