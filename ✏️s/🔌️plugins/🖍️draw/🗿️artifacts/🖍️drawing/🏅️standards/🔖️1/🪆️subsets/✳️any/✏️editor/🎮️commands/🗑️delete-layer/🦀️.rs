//! 🗂️ 🗂️ Drawing play app commands command — `delete-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::find_drawing_layer;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: String,
}

pub fn handle(payload: &DeleteLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    if payload.layer_id.is_empty() || find_drawing_layer(document, &payload.layer_id).is_none() {
        return Ok(Emit::default());
    }
    // 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM);
    // `Flat`-hierarchy domains are deliberately never auto-pruned on document change (see the plugin
    // SDK's `validate_state` doc), so a deleted layer's stale id simply stays selected until the next
    // real pick — a documented, accepted gap, not routed around here.
    Ok(Emit { artifact_mutations: vec![crate::mutations::delete_layer(payload.layer_id.clone())], ..Default::default() })
}
