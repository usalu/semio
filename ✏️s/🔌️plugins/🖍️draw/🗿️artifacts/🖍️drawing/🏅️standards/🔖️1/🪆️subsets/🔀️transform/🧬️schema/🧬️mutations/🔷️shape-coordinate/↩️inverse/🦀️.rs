//! ↩️ Restores exactly the addressed authored coordinate.
pub fn inverse<Op: From<crate::shape_coordinate::mutation::SetShapeCoordinate>>(payload:&super::mutation::SetShapeCoordinate,base:&semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError> {
    Ok(match semio_s_artifact_draw_drawing_model::find_drawing_layer(base,&payload.layer_id) {
        Some(semio_s_artifact_draw_drawing_model::DrawingLayerNode::Shape(shape))=>semio_s_artifact_draw_drawing_model::shape_geometry::shape_coordinate(shape,&payload.field,payload.index).ok().map(|value|vec![super::mutation::set_shape_coordinate(payload.layer_id.clone(),payload.field,payload.index,value)]).unwrap_or_default(),
        _=>Vec::new(),
    })
}
