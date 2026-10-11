//! 👁️ 👁️ Drawing play app commands command — `set-camera`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: store::Viewport2d,
}

/// 📷️ Camera — session-only runtime pose, never a document operation.
pub fn handle(_payload: &SetCamera, _doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
