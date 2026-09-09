//! 🖱️ 🖱️ Drawing play app commands command — `canvas-double-click`.

use crate::editor::drawing::commands::canvas_pointer_down::{drawing_gesture, DrawingSession};
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-double-click")]
pub struct CanvasDoubleClick {}

pub fn handle(_payload: &CanvasDoubleClick, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, DrawingConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let emit = session.step_gesture(drawing_gesture::Event::CommitDraft, document, config);
    Ok(emit)
}
