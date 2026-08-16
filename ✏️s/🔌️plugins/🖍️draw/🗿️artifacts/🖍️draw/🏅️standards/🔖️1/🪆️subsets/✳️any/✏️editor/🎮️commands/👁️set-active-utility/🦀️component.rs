//! 👁️ 👁️ Draw play app commands command — `set-active-utility`.

use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::schema::{flatten_draw_layers, layer_id};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

/// 🧰️ Host-owned utility switch: clear any in-progress gesture scratch (discarding any
/// document-op the FSM would produce — `UtilityChanged` never carries one).
pub fn handle(payload: &SetActiveUtility, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let mut config = cfg.snapshot.clone();
    session.step_gesture(crate::editor::draw::commands::canvas_pointer_down::draw_gesture::Event::UtilityChanged, document, &mut config);
    Ok(Emit::config(vec![DrawConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}
