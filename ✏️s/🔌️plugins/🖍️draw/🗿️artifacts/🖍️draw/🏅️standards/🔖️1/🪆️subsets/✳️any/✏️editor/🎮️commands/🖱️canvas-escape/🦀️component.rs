//! 🖱️ 🖱️ Draw play app commands command — `canvas-escape`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::{draw_gesture, DrawSession};
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-escape")]
pub struct CanvasEscape {}

pub fn handle(_payload: &CanvasEscape, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let operation = doc.operation()?;
    session.cancel_trace_pointer(operation.app_instance_id, &operation.parent_document_id, config.trace_pointer_generation);
    let mut emit = session.step_gesture(draw_gesture::Event::Escape, document, config);
    if config.trace_pointer_generation != 0 {
        emit.config_mutations.push(DrawConfigMutation::SetTracePointerProgress { generation: 0, completed_work: 0, pending_work: 0 });
    }
    Ok(emit)
}
