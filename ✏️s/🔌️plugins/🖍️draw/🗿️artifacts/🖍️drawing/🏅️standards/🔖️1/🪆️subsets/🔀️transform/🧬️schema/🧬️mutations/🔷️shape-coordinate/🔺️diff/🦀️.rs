//! 📐️ Sparse authored coordinate delta preserves appearance and layer identity.
pub fn diff(payload:&super::mutation::SetShapeCoordinate,base:&crate::DrawingSnapshot)->protocol::MutationOutcome<crate::DrawingDiff> {
    let shape=match crate::schema::find_drawing_layer(base,&payload.layer_id) {
        Some(crate::DrawingLayerNode::Shape(shape))=>shape,
        Some(_)=>return protocol::MutationOutcome::error("mutation.target-mismatch","The target layer is not a shape",[payload.layer_id.to_string_owner()]),
        None=>return protocol::MutationOutcome::error("mutation.target-missing","The target shape layer does not exist",[payload.layer_id.to_string_owner()]),
    };
    let current=match crate::schema::shape_geometry::shape_coordinate(shape,&payload.field,payload.index){Ok(value)=>value,Err(message)=>return protocol::MutationOutcome::error("mutation.target-mismatch",message,[payload.layer_id.to_string_owner()])};
    if !payload.value.is_finite() || (matches!(payload.field,crate::schema::shape_geometry::ShapeCoordinateField::RectWidth|crate::schema::shape_geometry::ShapeCoordinateField::RectHeight|crate::schema::shape_geometry::ShapeCoordinateField::EllipseRx|crate::schema::shape_geometry::ShapeCoordinateField::EllipseRy|crate::schema::shape_geometry::ShapeCoordinateField::CircleR)&&payload.value<0.0){return protocol::MutationOutcome::fatal("mutation.invariant","Shape coordinates must be finite and dimensions nonnegative",[payload.layer_id.to_string_owner()]);}
    if current==payload.value{return protocol::MutationOutcome::empty();}
    protocol::MutationOutcome::new(crate::diff::layer_base_patch(&payload.layer_id,crate::diff::DrawingLayerPatch{shape_coordinates:vec![crate::diff::DrawingShapeCoordinatePatch{field:payload.field,index:payload.index,value:payload.value}],..Default::default()}))
}
