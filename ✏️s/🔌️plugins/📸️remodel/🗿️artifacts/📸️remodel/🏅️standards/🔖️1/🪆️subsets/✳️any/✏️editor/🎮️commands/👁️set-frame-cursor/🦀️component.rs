//! 👁️ 👁️ Remodel play app commands command — `set-frame-cursor`.

use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "frame-cursor")]
pub struct SetFrameCursor {
    #[value(default)]
    pub stream_id: Option<String>,
    pub frame_index: u32,
}

pub async fn handle(payload: &SetFrameCursor, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelConfigMutation::SetFrameCursor { stream_id: payload.stream_id.clone(), frame_index: payload.frame_index }]))
}
