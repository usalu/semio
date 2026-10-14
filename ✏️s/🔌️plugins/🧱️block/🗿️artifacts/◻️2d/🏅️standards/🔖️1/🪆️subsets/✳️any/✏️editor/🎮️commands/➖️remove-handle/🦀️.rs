//! ➖️ Block 2D play app command — `remove-handle`.

use semio_s_artifact_block_2d_mutations::Block2dMutation;
use semio_s_artifact_block_2d_model::Block2dSnapshot;
use crate::config::{Block2dConfig, Block2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "removeHandle")]
pub struct RemoveHandle {
    pub id: String,
}

pub fn handle(payload: &RemoveHandle, _doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![semio_s_artifact_block_2d_mut_handles::delete_handle(payload.id.clone())]))
}
