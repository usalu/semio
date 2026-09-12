//! 🗂️ 🗂️ Drawing play app commands command — `toggle-layer-visible`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::find_drawing_layer;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "toggle-layer-visible")]
pub struct ToggleLayerVisible {
    pub layer_id: String,
}

pub fn handle(payload: &ToggleLayerVisible, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    match find_drawing_layer(document, &payload.layer_id) {
        Some(layer) => {
            let visible = !crate::schema::layer_base(layer).visible;
            Ok(Emit::mutations(vec![crate::mutations::set_layer_visible(payload.layer_id.clone(), visible)]))
        }
        None => Ok(Emit::default()),
    }
}
