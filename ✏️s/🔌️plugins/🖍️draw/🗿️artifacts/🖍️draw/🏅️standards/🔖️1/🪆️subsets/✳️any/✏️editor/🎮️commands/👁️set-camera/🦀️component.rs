//! 👁️ 👁️ Draw play app commands command — `set-camera`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: DrawCamera,
}

/// 📷️ Camera — session-only runtime pose, never a document operation.
pub async fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    Ok(Emit::config(vec![DrawConfigMutation::SetCamera { camera: payload.camera.clone() }]))
}
