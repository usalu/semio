//! 🧭️ 🧭️ S Studio app command — `navigate-virtual-file-system-node`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "navigate-vfs-node")]
pub struct NavigateVirtualFileSystemNode {
    pub space_id: String,
}

pub fn handle(payload: &NavigateVirtualFileSystemNode, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{}", payload.space_id) }))
}
