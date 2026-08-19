//! 👁️ 👁️ Draw play app commands command — `set-camera-zoom`.

use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "camera-zoom")]
pub struct SetCameraZoom {
    pub value: f64,
}

pub async fn handle(payload: &SetCameraZoom, _doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let camera = DrawCamera { zoom: payload.value, ..config.camera.clone() };
    Ok(Emit::config(vec![DrawConfigMutation::SetCamera { camera }]))
}
