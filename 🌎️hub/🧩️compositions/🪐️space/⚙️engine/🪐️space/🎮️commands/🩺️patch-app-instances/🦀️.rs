//! 🧩️ 🧩️ S Studio app command — `patch-app-instances`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::workflow::RenameNode;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-app-instances")]
pub struct PatchAppInstances {
    pub node_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchAppInstances, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    if payload.field == "label" {
        Ok(Emit::mutations(payload.node_ids.iter().cloned().map(|node_id| WorkflowMutation::RenameNode(RenameNode { node_id, label: payload.value.clone() })).collect()))
    } else {
        Ok(Emit::default())
    }
}
