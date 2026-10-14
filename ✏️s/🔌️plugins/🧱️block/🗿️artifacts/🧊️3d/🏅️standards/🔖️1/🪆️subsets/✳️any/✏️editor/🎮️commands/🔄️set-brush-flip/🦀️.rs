//! 🔄️ Block 3D play app command — `set-brush-flip`.

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_editor_config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "setBrushFlip")]
pub struct SetBrushFlip {
    pub flip: bool,
}

pub fn handle(payload: &SetBrushFlip, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block3dConfigMutation::SetBrushFlip { flip: payload.flip }]))
}
