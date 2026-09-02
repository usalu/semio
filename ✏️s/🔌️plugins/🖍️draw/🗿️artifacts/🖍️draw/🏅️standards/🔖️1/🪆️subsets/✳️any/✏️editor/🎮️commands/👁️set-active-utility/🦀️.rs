//! 👁️ 👁️ Draw play app commands command — `set-active-utility`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

/// 🧰️ Host-owned utility switch: clear any in-progress gesture scratch (discarding any
/// document-op the FSM would produce — `UtilityChanged` never carries one).
pub fn handle(payload: &SetActiveUtility, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let mut config = cfg.snapshot.clone();
    let operation = doc.operation()?;
    session.cancel_trace_pointer(operation.app_instance_id, &operation.parent_document_id, config.trace_pointer_generation);
    session.step_gesture(crate::editor::draw::commands::canvas_pointer_down::draw_gesture::Event::UtilityChanged, document, &mut config);
    Ok(Emit::config(vec![DrawConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}
