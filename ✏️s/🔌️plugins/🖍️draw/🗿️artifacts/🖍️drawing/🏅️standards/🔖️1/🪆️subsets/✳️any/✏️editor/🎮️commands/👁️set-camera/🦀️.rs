//! 👁️ 👁️ Drawing play app commands command — `set-camera`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::{DrawingCamera, DrawingSnapshot};
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: DrawingCamera,
}

/// 📷️ Camera — session-only runtime pose, never a document operation.
pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, DrawingConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    Ok(Emit::config(vec![DrawingConfigMutation::SetCamera { camera: payload.camera.clone() }]))
}
