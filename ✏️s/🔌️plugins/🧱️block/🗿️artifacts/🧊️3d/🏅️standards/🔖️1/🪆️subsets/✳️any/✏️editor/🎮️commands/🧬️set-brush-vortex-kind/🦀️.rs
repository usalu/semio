//! 🧬️ Block 3D play app command — `set-brush-vortex-kind`.

use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "setBrushVortexKind")]
pub struct SetBrushVortexKind {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vortex_kind_id: Option<String>,
}

pub async fn handle(payload: &SetBrushVortexKind, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetBrushVortexKind { vortex_kind_id: payload.vortex_kind_id.clone() }]))
}
