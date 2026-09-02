//! 👁️ 👁️ Draw play app commands command — `set-camera-zoom`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera-zoom")]
pub struct SetCameraZoom {
    pub value: f64,
}

pub fn handle(payload: &SetCameraZoom, _doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let camera = DrawCamera { zoom: payload.value, ..config.camera.clone() };
    Ok(Emit::config(vec![DrawConfigMutation::SetCamera { camera }]))
}
