//! 🗑️ Block 3D play app command — `remove-vortex-kind`.

use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "removeVortexKind")]
pub struct RemoveVortexKind {
    pub id: String,
}

pub async fn handle(payload: &RemoveVortexKind, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::block3d::mutations::delete_vortex_kind(payload.id.clone())]))
}
