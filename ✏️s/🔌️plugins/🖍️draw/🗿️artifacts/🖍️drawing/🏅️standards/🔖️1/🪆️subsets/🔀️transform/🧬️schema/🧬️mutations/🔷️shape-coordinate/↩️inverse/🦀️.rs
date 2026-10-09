//! ↩️ Restores exactly the addressed authored coordinate.
pub fn inverse(payload:&super::mutation::SetShapeCoordinate,base:&crate::DrawingSnapshot)->Result<Vec<crate::DrawingMutation>,semio_framework_value::ValueError> {
    Ok(match crate::schema::find_drawing_layer(base,&payload.layer_id) {
        Some(crate::DrawingLayerNode::Shape(shape))=>crate::schema::shape_geometry::shape_coordinate(shape,&payload.field,payload.index).ok().map(|value|vec![super::mutation::set_shape_coordinate(payload.layer_id.clone(),payload.field,payload.index,value)]).unwrap_or_default(),
        _=>Vec::new(),
    })
}
