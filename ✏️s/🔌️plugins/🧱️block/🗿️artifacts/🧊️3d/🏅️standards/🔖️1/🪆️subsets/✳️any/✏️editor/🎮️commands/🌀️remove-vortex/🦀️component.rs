//! 🌀️ 🌀️ Block 3D play app commands command — `remove-vortex`.

use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "removeVortex")]
pub struct RemoveVortex {
    pub id: String,
}

pub async fn handle(payload: &RemoveVortex, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::block3d::mutations::delete_vortex(payload.id.clone())]))
}
