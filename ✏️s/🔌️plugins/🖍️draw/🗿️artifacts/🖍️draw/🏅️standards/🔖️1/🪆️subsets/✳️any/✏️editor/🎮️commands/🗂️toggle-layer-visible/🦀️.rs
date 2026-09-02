//! 🗂️ 🗂️ Draw play app commands command — `toggle-layer-visible`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "toggle-layer-visible")]
pub struct ToggleLayerVisible {
    pub layer_id: String,
}

pub fn handle(payload: &ToggleLayerVisible, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    match find_draw_layer(document, &payload.layer_id) {
        Some(layer) => {
            let visible = !crate::artifacts::draw::schema::layer_base(layer).visible;
            Ok(Emit::mutations(vec![crate::artifacts::draw::mutations::set_layer_visible(payload.layer_id.clone(), visible)]))
        }
        None => Ok(Emit::default()),
    }
}
