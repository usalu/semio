//! ➖️ Block 5D play app command — `remove-grip`.

use crate::artifacts::block5d::op::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::editor::block5d::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "removeGrip")]
pub struct RemoveGrip {
    pub id: String,
}

pub fn handle(payload: &RemoveGrip, _doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::block5d::mutations::delete_grip(payload.id.clone())]))
}
