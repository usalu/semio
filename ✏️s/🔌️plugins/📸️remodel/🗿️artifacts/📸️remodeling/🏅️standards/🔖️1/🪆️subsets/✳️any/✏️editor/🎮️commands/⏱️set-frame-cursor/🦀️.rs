//! 👁️ 👁️ Remodeling play app commands command — `set-frame-cursor`.

use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "frame-cursor")]
pub struct SetFrameCursor {
    #[value(default)]
    pub stream_id: Option<String>,
    pub frame_index: u32,
}

pub async fn handle(payload: &SetFrameCursor, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelingConfigMutation::SetFrameCursor { stream_id: payload.stream_id.clone(), frame_index: payload.frame_index }]))
}
