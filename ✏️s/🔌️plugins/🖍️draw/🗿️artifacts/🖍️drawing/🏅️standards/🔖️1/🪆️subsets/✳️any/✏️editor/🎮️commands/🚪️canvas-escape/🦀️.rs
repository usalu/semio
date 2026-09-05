//! 🖱️ 🖱️ Drawing play app commands command — `canvas-escape`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::{drawing_gesture, DrawingSession};
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-escape")]
pub struct CanvasEscape {}

pub fn handle(_payload: &CanvasEscape, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, DrawingConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let operation = doc.operation()?;
    session.cancel_trace_pointer(operation.app_instance_id, &operation.parent_document_id, config.trace_pointer_generation);
    let mut emit = session.step_gesture(drawing_gesture::Event::Escape, document, config);
    if config.trace_pointer_generation != 0 {
        emit.config_mutations.push(DrawingConfigMutation::SetTracePointerProgress { generation: 0, completed_work: 0, pending_work: 0 });
    }
    Ok(emit)
}
