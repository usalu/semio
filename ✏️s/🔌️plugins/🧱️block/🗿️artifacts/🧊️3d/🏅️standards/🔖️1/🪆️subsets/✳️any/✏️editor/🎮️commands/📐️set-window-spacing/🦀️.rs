//! 📐️ Block 3D play app command — `set-window-spacing`.

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_editor_config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "setWindowSpacing")]
pub struct SetWindowSpacing {
    pub window_id: String,
    pub spacing: f64,
}

pub fn handle(payload: &SetWindowSpacing, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetWindowSpacing { window_id: payload.window_id.clone(), spacing: payload.spacing }]))
}
