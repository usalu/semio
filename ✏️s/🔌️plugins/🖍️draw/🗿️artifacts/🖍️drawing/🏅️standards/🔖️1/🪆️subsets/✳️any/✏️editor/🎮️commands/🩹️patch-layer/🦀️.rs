//! 🗂️ 🗂️ Drawing play app commands command — `patch-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::{drawing_op_for_layer_field, DrawingMutation};
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-layer")]
pub struct PatchLayer {
    pub layer_id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    if !matches!(payload.field.as_str(), "locked" | "visible") && crate::schema::drawing_layer_is_locked(document, &payload.layer_id) { return Err(Fault::from("The selected layer is locked")); }
    let json_value = crate::mutations::parse_layer_field_input(&payload.field, &payload.value);
    match drawing_op_for_layer_field(document, &payload.layer_id, &payload.field, &json_value) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Err(Fault::from("Invalid layer field or value")),
    }
}
