//! 🗂️ 🗂️ Block 2D play app command command — `set-selection`.

use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
use crate::artifacts::block2d::op::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "setSelection")]
pub struct SetSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Block2dConfigMutation::SetSelection { ids: payload.ids.clone() }]))
}
