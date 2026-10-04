//! 🗂️ 🗂️ S Home launcher app command — `navigate-virtual-file-system-node`.

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "navigate-vfs-node")]
pub struct NavigateVirtualFileSystemNode {
    pub node_id: String,
}

pub fn handle(payload: &NavigateVirtualFileSystemNode, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let space_id = payload.node_id.strip_prefix("studio:").unwrap_or(&payload.node_id);
    Ok(Emit::effect(Effect::Navigate { uri: format!("/spaces/{space_id}") }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
