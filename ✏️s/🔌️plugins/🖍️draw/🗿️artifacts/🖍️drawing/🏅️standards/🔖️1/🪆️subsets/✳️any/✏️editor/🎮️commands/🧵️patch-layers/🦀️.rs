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
    #[value(default)]
    pub index:Option<usize>,
}

pub fn handle(payload: &PatchLayers, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024,&mut accepted);
    let json_value = crate::standards::v1::subsets::any::io::text::mutations::field_input::parse_layer_field_input(&payload.field,&payload.value,&mut control).map_err(|error|Fault::from(error.to_string()))?;
    let ids = if payload.layer_ids.is_empty() { &session.interaction.ids } else { &payload.layer_ids };
    let selected = selected_targets(document,ids)?;
    let mut operations = Vec::with_capacity(selected.len());
    for layer in selected {
        let id = crate::schema::layer_id(layer);
        if !matches!(payload.field.as_str(), "locked" | "visible") && crate::schema::drawing_layer_is_locked(document, id) { return Err(Fault::from("The selected layer is locked")); }
        let operation=if let Some(index)=payload.index {
            let crate::DrawingLayerNode::Shape(shape)=layer else{return Err(Fault::from("Select a shape to edit its point"));};
            let field=crate::schema::shape_geometry::ShapeCoordinateField::parse(&payload.field).map_err(Fault::from)?;
            crate::schema::shape_geometry::shape_coordinate(shape,&field,Some(index)).map_err(Fault::from)?;
            let value=json_value.as_f64().filter(|number|number.is_finite()).ok_or_else(||Fault::from("Enter a finite coordinate"))?;
            crate::mutations::set_shape_coordinate(id.clone(),field,Some(index),value)
        }else{drawing_op_for_layer_field(document,id,&payload.field,&json_value).ok_or_else(||Fault::from("Invalid layer field or value"))?};
        if super::patch_layer::changes_layer(document,&operation)? {operations.push(operation);}
    }
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(operations))
}

/// 🛂️ Every requested target is admitted before a bulk field edit can publish.
pub(super) fn selected_targets<'a>(document:&'a DrawingSnapshot,ids:&[String])->Result<Vec<&'a crate::DrawingLayerNode>,Fault> {
    let selected=crate::schema::selected_drawing_layers(document,ids);
    if ids.iter().any(|id|!selected.iter().any(|layer|id==crate::schema::layer_id(layer)||id==&crate::schema::drawing_play_layers_tree_row_id(layer))){return Err(Fault::from("A selected layer field target is missing"));}
    Ok(selected)
}
