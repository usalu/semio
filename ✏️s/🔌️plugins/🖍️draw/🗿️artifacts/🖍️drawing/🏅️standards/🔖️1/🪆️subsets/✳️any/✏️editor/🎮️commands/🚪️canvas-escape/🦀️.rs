//! 🖱️ 🖱️ Drawing play app commands command — `canvas-escape`.

use crate::editor::drawing::commands::canvas_pointer_down::{drawing_gesture, DrawingSession};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-escape")]
pub struct CanvasEscape {}

pub fn handle(_payload: &CanvasEscape, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let operation = doc.operation()?;
    let generation = session.window_transient.trace_pointer_generation;
    session.cancel_trace_pointer(operation.app_instance_id, &operation.parent_document_id, generation);
    let emit = session.step_gesture(drawing_gesture::Event::Escape, document, cfg.snapshot);
    if generation != 0 {
        session.window_transient.trace_pointer_generation = 0;
        session.window_transient.trace_pointer_completed_work = 0;
        session.window_transient.trace_pointer_pending_work = 0;
    }
    Ok(emit)
}
