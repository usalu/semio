//! 👁️ 👁️ Remodel play app commands command — `set-layer-visibility`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation, RemodelWorldCamera};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "layer-visibility")]
pub struct SetLayerVisibility {
    pub layer: String,
    pub visible: bool,
}

pub fn handle(payload: &SetLayerVisibility, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelConfigMutation::SetLayerVisibility { layer: payload.layer.clone(), visible: payload.visible }]))
}
