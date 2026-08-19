//! 🗂️ 🗂️ S Home launcher app command — `navigate-virtual-file-system-node`.

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, Effect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "navigate-vfs-node")]
pub struct NavigateVirtualFileSystemNode {
    pub node_id: String,
}

pub async fn handle(payload: &NavigateVirtualFileSystemNode, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let space_id = payload.node_id.strip_prefix("studio:").unwrap_or(&payload.node_id);
    eprintln!("[DEBUG] home navigateVirtualFileSystemNode id={space_id}");
    Ok(Emit::effect(Effect::Navigate { uri: format!("/spaces/{space_id}") }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn home_command_op_text_round_trips_every_variant() {
        use crate::editor::home::HomeCommand;
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::NavigateVirtualFileSystemNode(NavigateVirtualFileSystemNode { node_id: "studio:s1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::DeleteVirtualFileSystemNode(crate::editor::home::commands::delete_virtual_file_system_node::DeleteVirtualFileSystemNode { node_id: "studio:s1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::GoHome(crate::editor::home::commands::go_home::GoHome {}));
    }
}
//#endregion 🧪️Tests
