//! 🗂️ 🗂️ Drawing play app commands command — `toggle-layer-visible`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::schema::find_drawing_layer;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "toggle-layer-visible")]
pub struct ToggleLayerVisible {
    pub layer_id: String,
}

pub fn handle(payload: &ToggleLayerVisible, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, DrawingConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    match find_drawing_layer(document, &payload.layer_id) {
        Some(layer) => {
            let visible = !crate::artifacts::drawing::schema::layer_base(layer).visible;
            Ok(Emit::mutations(vec![crate::artifacts::drawing::mutations::set_layer_visible(payload.layer_id.clone(), visible)]))
        }
        None => Ok(Emit::default()),
    }
}
