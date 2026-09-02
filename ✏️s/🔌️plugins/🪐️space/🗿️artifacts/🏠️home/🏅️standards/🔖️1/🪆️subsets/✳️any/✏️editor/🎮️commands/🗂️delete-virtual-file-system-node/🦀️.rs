//! 🗂️ 🗂️ S Home launcher app command — `delete-virtual-file-system-node`.

use crate::artifacts::home::mutations::change_catalog_generation;
use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use semio_framework_os::delete_os_space;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-vfs-node")]
pub struct DeleteVirtualFileSystemNode {
    pub node_id: String,
}

pub fn handle(payload: &DeleteVirtualFileSystemNode, doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let generation = doc.snapshot.catalog_generation;
    match payload.node_id.strip_prefix("studio:") {
        Some(space_id) => {
            // 🌉️ `draft_backbone_port`/`ephemeral_draft_catalog`/`catalog_port` are plugin-root
            // async fns (outside this lease); `handle` must stay sync — bridged via `resolve_ready`,
            // matching `🏙️create-studio`'s own seam.
            let draft_port = semio_framework_plugin::resolve_ready(crate::draft_backbone_port());
            semio_framework_plugin::resolve_ready(crate::ephemeral_draft_catalog()).discard_draft(&draft_port, space_id);
            let _ = delete_os_space(space_id, semio_framework_plugin::resolve_ready(crate::catalog_port()));
            Ok(Emit::mutations(vec![change_catalog_generation(generation + 1)]))
        }
        None => Ok(Emit::default()),
    }
}
