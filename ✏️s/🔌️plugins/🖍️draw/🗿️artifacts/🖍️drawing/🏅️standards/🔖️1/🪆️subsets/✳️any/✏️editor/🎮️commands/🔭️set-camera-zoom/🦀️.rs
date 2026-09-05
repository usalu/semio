//! 👁️ 👁️ Drawing play app commands command — `set-camera-zoom`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::{DrawingCamera, DrawingSnapshot};
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera-zoom")]
pub struct SetCameraZoom {
    pub value: f64,
}

pub fn handle(payload: &SetCameraZoom, _doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, DrawingConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let camera = DrawingCamera { zoom: payload.value, ..config.camera.clone() };
    Ok(Emit::config(vec![DrawingConfigMutation::SetCamera { camera }]))
}
