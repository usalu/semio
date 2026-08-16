//! 🔗️ 🔗️ Block 2D play app commands command — `remove-compatibility-rule`.

use crate::editor::block2d::config::{Block2dConfig, Block2dConfigMutation};
use crate::artifacts::block2d::op::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "removeCompatibilityRule")]
pub struct RemoveCompatibilityRule {
    pub id: String,
}

pub fn handle(payload: &RemoveCompatibilityRule, _doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::block2d::mutations::remove_compatibility_rule(payload.id.clone())]))
}
