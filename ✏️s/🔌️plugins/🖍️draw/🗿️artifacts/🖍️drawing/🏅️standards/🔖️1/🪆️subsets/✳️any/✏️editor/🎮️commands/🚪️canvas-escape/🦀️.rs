//! 🖱️ 🖱️ Drawing play app commands command — `canvas-escape`.

use crate::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "canvas-escape")]
pub struct CanvasEscape {}

/// 🚪️ Escape cancels a running trace and the canvas tool's gesture — a drag aborts with zero trace.
pub fn handle(_payload: &CanvasEscape, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let operation = doc.operation()?;
    let generation = session.window_transient.trace_pointer_generation;
    session.cancel_trace_pointer(operation.app_instance_id, &operation.parent_document_id, generation);
    let emit = session.escape()?;
    if generation != 0 {
        session.window_transient.trace_pointer_generation = 0;
        session.window_transient.trace_pointer_completed_work = 0;
        session.window_transient.trace_pointer_pending_work = 0;
    }
    Ok(emit)
}
