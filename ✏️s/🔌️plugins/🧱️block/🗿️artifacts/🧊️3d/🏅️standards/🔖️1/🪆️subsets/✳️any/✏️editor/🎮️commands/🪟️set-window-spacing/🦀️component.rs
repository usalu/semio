//! 🪟️ 🪟️ Block 3D play app commands command — `set-window-spacing`.

use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "setWindowSpacing")]
pub struct SetWindowSpacing {
    pub window_id: String,
    pub spacing: f64,
}

pub async fn handle(payload: &SetWindowSpacing, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetWindowSpacing { window_id: payload.window_id.clone(), spacing: payload.spacing }]))
}
