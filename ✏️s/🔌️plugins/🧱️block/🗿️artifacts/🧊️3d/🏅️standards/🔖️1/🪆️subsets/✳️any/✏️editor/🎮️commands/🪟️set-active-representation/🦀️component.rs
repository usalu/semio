//! 🪟️ 🪟️ Block 3D play app commands command — `set-active-representation`.

use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "setActiveRepresentation")]
pub struct SetActiveRepresentation {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub representation_id: Option<String>,
}

pub async fn handle(payload: &SetActiveRepresentation, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetActiveRepresentation { representation_id: payload.representation_id.clone() }]))
}
