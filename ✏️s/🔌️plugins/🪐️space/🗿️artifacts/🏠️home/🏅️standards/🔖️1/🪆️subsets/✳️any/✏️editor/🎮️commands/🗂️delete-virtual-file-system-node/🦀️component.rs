//! 🗂️ 🗂️ S Home launcher app command — `delete-virtual-file-system-node`.

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use crate::artifacts::home::mutations::change_catalog_generation;
use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use semio_framework_os::delete_os_space;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-vfs-node")]
pub struct DeleteVirtualFileSystemNode {
    pub node_id: String,
}

pub async fn handle(payload: &DeleteVirtualFileSystemNode, doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let generation = doc.snapshot.catalog_generation;
    match payload.node_id.strip_prefix("studio:") {
        Some(space_id) => {
            let draft_port = crate::draft_backbone_port();
            crate::ephemeral_draft_catalog().discard_draft(&draft_port, space_id);
            let _ = delete_os_space(space_id, crate::catalog_port());
            Ok(Emit::mutations(vec![change_catalog_generation(generation + 1)]))
        }
        None => Ok(Emit::default()),
    }
}
