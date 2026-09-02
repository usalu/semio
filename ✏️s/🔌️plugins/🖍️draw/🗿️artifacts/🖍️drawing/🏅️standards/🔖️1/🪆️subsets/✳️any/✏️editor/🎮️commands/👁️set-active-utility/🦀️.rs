//! 👁️ 👁️ Drawing play app commands command — `set-active-utility`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

/// 🧰️ Host-owned utility switch: clear any in-progress gesture scratch (discarding any
/// document-op the FSM would produce — `UtilityChanged` never carries one).
pub fn handle(payload: &SetActiveUtility, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, DrawingConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let mut config = cfg.snapshot.clone();
    let operation = doc.operation()?;
    session.cancel_trace_pointer(operation.app_instance_id, &operation.parent_document_id, config.trace_pointer_generation);
    session.step_gesture(crate::editor::drawing::commands::canvas_pointer_down::drawing_gesture::Event::UtilityChanged, document, &mut config);
    Ok(Emit::config(vec![DrawingConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}
