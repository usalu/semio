//! 🧭️ 🧭️ S Studio app command — `navigate-virtual-file-system-node`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "navigate-vfs-node")]
pub struct NavigateVirtualFileSystemNode {
    pub space_id: String,
}

pub async fn handle(payload: &NavigateVirtualFileSystemNode, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::Navigate { uri: format!("/spaces/{}", payload.space_id) }))
}
