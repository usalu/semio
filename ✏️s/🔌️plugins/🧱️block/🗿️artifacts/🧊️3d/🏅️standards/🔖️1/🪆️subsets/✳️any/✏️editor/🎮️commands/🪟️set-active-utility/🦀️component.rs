//! 🪟️ 🪟️ Block 3D play app commands command — `set-active-utility`.

use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "setActiveUtility")]
pub struct SetActiveUtility {
    pub window_id: String,
    pub utility_id: String,
}

pub fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetActiveUtility { window_id: payload.window_id.clone(), utility_id: payload.utility_id.clone() }]))
}
