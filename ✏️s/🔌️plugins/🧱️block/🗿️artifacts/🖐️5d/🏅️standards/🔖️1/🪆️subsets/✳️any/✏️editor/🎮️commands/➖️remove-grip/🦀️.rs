//! ➖️ Block 5D play app command — `remove-grip`.

use semio_s_artifact_block_5d_mutations::Block5dMutation;
use semio_s_artifact_block_5d_model::Block5dSnapshot;
use crate::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "removeGrip")]
pub struct RemoveGrip {
    pub id: String,
}

pub fn handle(payload: &RemoveGrip, _doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![semio_s_artifact_block_5d_mut_grips::delete_grip(payload.id.clone())]))
}
