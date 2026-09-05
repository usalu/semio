//! 🖼️ Block 3D play app command — `set-window-representations`.

use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "setWindowRepresentations")]
pub struct SetWindowRepresentations {
    pub window_id: String,
    pub representation_ids: Vec<String>,
}

pub async fn handle(payload: &SetWindowRepresentations, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetWindowRepresentations { window_id: payload.window_id.clone(), representation_ids: payload.representation_ids.clone() }]))
}
