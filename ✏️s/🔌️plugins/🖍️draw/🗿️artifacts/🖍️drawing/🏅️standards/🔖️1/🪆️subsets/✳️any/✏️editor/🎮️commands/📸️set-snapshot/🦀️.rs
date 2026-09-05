//! 📄️ 📄️ Drawing play app commands command — `set-snapshot`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: DrawingSnapshot,
}

pub fn handle(
    payload: &SetSnapshot,
    _doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, DrawingConfig>,
    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,
) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    Ok(Emit { effects: vec![crate::editor::drawing::drawing_reset_document_effect(&payload.snapshot)], ..Default::default() })
}
