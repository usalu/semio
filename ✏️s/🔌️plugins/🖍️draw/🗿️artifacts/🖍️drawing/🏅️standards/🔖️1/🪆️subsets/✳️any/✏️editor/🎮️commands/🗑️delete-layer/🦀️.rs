//! 🗂️ 🗂️ Drawing play app commands command — `delete-layer`.

use semio_s_artifact_draw_drawing_editor_tool::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::find_drawing_layer;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
    Ok(Emit { artifact_mutations: vec![semio_s_artifact_draw_drawing_structure::delete_layer::mutation::delete_layer(payload.layer_id.clone().into())], ..Default::default() })
}
