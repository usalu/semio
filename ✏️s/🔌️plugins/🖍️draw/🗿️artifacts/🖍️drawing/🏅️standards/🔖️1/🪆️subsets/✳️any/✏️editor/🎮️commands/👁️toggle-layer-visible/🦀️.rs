//! 🗂️ 🗂️ Drawing play app commands command — `toggle-layer-visible`.

use semio_s_artifact_draw_drawing_editor_tool::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::find_drawing_layer;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "toggle-layer-visible")]
pub struct ToggleLayerVisible {
    pub layer_id: String,
}

pub fn handle(payload: &ToggleLayerVisible, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    match find_drawing_layer(document, &payload.layer_id) {
        Some(layer) => {
            let visible = !semio_s_artifact_draw_drawing_model::layer_base(layer).visible;
            Ok(Emit::mutations(vec![semio_s_artifact_draw_drawing_metadata::set_layer_visible::mutation::set_layer_visible(payload.layer_id.clone().into(), visible)]))
        }
        None => Ok(Emit::default()),
    }
}
