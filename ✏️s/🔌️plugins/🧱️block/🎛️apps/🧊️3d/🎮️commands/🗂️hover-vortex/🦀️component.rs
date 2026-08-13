//! 🗂️ 🗂️ Block 3D play app commands command — `hover-vortex`.

use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "hoverVortex")]
pub struct HoverVortex {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_id: Option<String>,
}

pub fn handle(payload: &HoverVortex, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetHoveredVortexFullId { full_id: payload.full_id.clone() }]))
}
