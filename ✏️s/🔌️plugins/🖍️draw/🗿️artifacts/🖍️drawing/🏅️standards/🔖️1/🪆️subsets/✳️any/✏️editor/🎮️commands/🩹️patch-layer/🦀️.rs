//! 🗂️ 🗂️ Drawing play app commands command — `patch-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::{drawing_op_for_layer_field, DrawingMutation};
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "patch-layer")]
pub struct PatchLayer {
    pub layer_id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    if !matches!(payload.field.as_str(), "locked" | "visible") && crate::schema::drawing_layer_is_locked(document, &payload.layer_id) { return Err(Fault::from("The selected layer is locked")); }
    let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024,&mut accepted);
    let json_value = crate::standards::v1::subsets::any::io::text::mutations::field_input::parse_layer_field_input(&payload.field,&payload.value,&mut control).map_err(|error|Fault::from(error.to_string()))?;
    match drawing_op_for_layer_field(document, &payload.layer_id, &payload.field, &json_value) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Err(Fault::from("Invalid layer field or value")),
    }
}
