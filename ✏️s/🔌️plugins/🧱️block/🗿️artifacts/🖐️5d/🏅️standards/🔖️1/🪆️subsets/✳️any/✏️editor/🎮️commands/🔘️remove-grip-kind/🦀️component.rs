//! 🔘️ 🔘️ Block 5D play app commands command — `remove-grip-kind`.

use crate::editor::block5d::config::{Block5dConfig, Block5dConfigMutation};
use crate::artifacts::block5d::op::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "removeGripKind")]
pub struct RemoveGripKind {
    pub id: String,
}

pub fn handle(payload: &RemoveGripKind, _doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::block5d::mutations::delete_grip_kind(payload.id.clone())]))
}
