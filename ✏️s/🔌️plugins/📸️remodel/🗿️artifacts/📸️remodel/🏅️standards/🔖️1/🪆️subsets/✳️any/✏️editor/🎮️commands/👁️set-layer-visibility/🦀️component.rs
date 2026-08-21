//! 👁️ 👁️ Remodel play app commands command — `set-layer-visibility`.

use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "layer-visibility")]
pub struct SetLayerVisibility {
    pub layer: String,
    pub visible: bool,
}

pub async fn handle(payload: &SetLayerVisibility, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelConfigMutation::SetLayerVisibility { layer: payload.layer.clone(), visible: payload.visible }]))
}
