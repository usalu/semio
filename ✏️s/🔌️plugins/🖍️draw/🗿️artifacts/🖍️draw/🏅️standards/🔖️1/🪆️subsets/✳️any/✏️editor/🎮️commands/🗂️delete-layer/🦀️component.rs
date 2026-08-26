//! 🗂️ 🗂️ Draw play app commands command — `delete-layer`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: String,
}

pub fn handle(payload: &DeleteLayer, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    if payload.layer_id.is_empty() || find_draw_layer(document, &payload.layer_id).is_none() {
        return Ok(Emit::default());
    }
    // 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM);
    // `Flat`-hierarchy domains are deliberately never auto-pruned on document change (see the plugin
    // SDK's `validate_state` doc), so a deleted layer's stale id simply stays selected until the next
    // real pick — a documented, accepted gap, not routed around here.
    Ok(Emit { artifact_mutations: vec![crate::artifacts::draw::mutations::delete_layer(payload.layer_id.clone())], ..Default::default() })
}
