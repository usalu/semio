//! 🗂️ 🗂️ Drawing play app commands command — `patch-layers`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::{drawing_op_for_layer_field, DrawingMutation};
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "patch-layers")]
pub struct PatchLayers {
    pub layer_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchLayers, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024,&mut accepted);
    let json_value = crate::standards::v1::subsets::any::io::text::mutations::field_input::parse_layer_field_input(&payload.field,&payload.value,&mut control).map_err(|error|Fault::from(error.to_string()))?;
    let ids = if payload.layer_ids.is_empty() { &session.interaction.ids } else { &payload.layer_ids };
    let selected = crate::schema::selected_drawing_layers(document, ids);
    let mut operations = Vec::with_capacity(selected.len());
    for layer in selected {
        let id = crate::schema::layer_id(layer);
        if !matches!(payload.field.as_str(), "locked" | "visible") && crate::schema::drawing_layer_is_locked(document, id) { return Err(Fault::from("The selected layer is locked")); }
        operations.push(drawing_op_for_layer_field(document, id, &payload.field, &json_value).ok_or_else(|| Fault::from("Invalid layer field or value"))?);
    }
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(operations))
}
