//! 🗂️ S Home launcher app — virtual-file-system navigation commands.
//!
//! One nested `pub mod` per payload (the `app_commands!` shape — see `apps::home::🦀️component.rs`'s
//! `🔖️HomeCommand` region, which `use`s each of these modules flat).

use crate::apps::home::config::{HomeConfig, HomeConfigOperation};
use crate::artifacts::home::op::SHomeOperation;
use crate::artifacts::home::SHomeDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};

//#region 🔖️NavigateVirtualFileSystemNode
pub mod navigate_virtual_file_system_node {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "navigate-vfs-node")]
    pub struct NavigateVirtualFileSystemNode {
        pub node_id: String,
    }

    pub fn handle(payload: &NavigateVirtualFileSystemNode, _doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeOperation, HomeConfigOperation>, Fault> {
        let space_id = payload.node_id.strip_prefix("studio:").unwrap_or(&payload.node_id);
        eprintln!("[DEBUG] home navigateVirtualFileSystemNode id={space_id}");
        Ok(Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{space_id}") }))
    }
}
//#endregion 🔖️NavigateVirtualFileSystemNode

//#region 🔖️DeleteVirtualFileSystemNode
pub mod delete_virtual_file_system_node {
    use super::*;
    use semio_framework_os::delete_os_space;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-vfs-node")]
    pub struct DeleteVirtualFileSystemNode {
        pub node_id: String,
    }

    pub fn handle(payload: &DeleteVirtualFileSystemNode, doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeOperation, HomeConfigOperation>, Fault> {
        let generation = doc.projection.catalog_generation;
        match payload.node_id.strip_prefix("studio:") {
            Some(space_id) => {
                let draft_port = crate::apps::home::draft_backbone_port();
                crate::apps::home::ephemeral_draft_catalog().discard_draft(&draft_port, space_id);
                let _ = delete_os_space(space_id, crate::apps::home::catalog_port());
                Ok(Emit::operations(vec![SHomeOperation::SetCatalogGeneration { value: generation + 1 }]))
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️DeleteVirtualFileSystemNode

//#region 🔖️GoHome
pub mod go_home {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "go-home")]
    pub struct GoHome {}

    pub fn handle(_payload: &GoHome, _doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeOperation, HomeConfigOperation>, Fault> {
        Ok(Emit::effect(HostEffect::Navigate { uri: "/".into() }))
    }
}
//#endregion 🔖️GoHome

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_command_op_text_round_trips_every_variant() {
        use crate::apps::home::HomeCommand;
        store::test_support::assert_op_line_round_trip(&HomeCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode { node_id: "studio:s1".into() }));
        store::test_support::assert_op_line_round_trip(&HomeCommand::DeleteVirtualFileSystemNode(delete_virtual_file_system_node::DeleteVirtualFileSystemNode { node_id: "studio:s1".into() }));
        store::test_support::assert_op_line_round_trip(&HomeCommand::GoHome(go_home::GoHome {}));
    }
}
//#endregion 🧪️Tests
