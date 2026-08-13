//! 🔘️ 🔘️ Block 3D play app commands command — `remove-vortex-kind`.

use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "removeVortexKind")]
pub struct RemoveVortexKind {
    pub id: String,
}

pub fn handle(payload: &RemoveVortexKind, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::block3d::mutations::delete_vortex_kind(payload.id.clone())]))
}
