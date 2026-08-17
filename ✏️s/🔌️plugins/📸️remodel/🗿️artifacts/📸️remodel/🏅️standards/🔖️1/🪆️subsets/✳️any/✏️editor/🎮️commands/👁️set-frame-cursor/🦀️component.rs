//! 👁️ 👁️ Remodel play app commands command — `set-frame-cursor`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "frame-cursor")]
pub struct SetFrameCursor {
    #[serde(default)]
    pub stream_id: Option<String>,
    pub frame_index: u32,
}

pub fn handle(payload: &SetFrameCursor, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelConfigMutation::SetFrameCursor { stream_id: payload.stream_id.clone(), frame_index: payload.frame_index }]))
}
