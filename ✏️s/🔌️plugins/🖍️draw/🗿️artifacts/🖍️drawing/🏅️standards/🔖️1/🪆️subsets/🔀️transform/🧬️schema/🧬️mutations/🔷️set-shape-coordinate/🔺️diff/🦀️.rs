//! 📐️ Sparse authored coordinate delta preserves appearance and layer identity.
pub fn diff(payload:&super::mutation::SetShapeCoordinate,base:&semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot)->protocol::MutationOutcome<semio_s_artifact_draw_drawing_diff::DrawingDiff> {
    let shape=match semio_s_artifact_draw_drawing_model::find_drawing_layer(base,&payload.layer_id) {
        Some(semio_s_artifact_draw_drawing_model::DrawingLayerNode::Shape(shape))=>shape,
        Some(_)=>return protocol::MutationOutcome::error("mutation.target-mismatch","The target layer is not a shape",[payload.layer_id.to_string_owner()]),
        None=>return protocol::MutationOutcome::error("mutation.target-missing","The target shape layer does not exist",[payload.layer_id.to_string_owner()]),
    };
    let current=match semio_s_artifact_draw_drawing_model::shape_geometry::shape_coordinate(shape,&payload.field,payload.index){Ok(value)=>value,Err(message)=>return protocol::MutationOutcome::error("mutation.target-mismatch",message,[payload.layer_id.to_string_owner()])};
    if !payload.value.is_finite() || (matches!(payload.field,semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::RectWidth|semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::RectHeight|semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::EllipseRx|semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::EllipseRy|semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::CircleR)&&payload.value<0.0){return protocol::MutationOutcome::fatal("mutation.invariant","Shape coordinates must be finite and dimensions nonnegative",[payload.layer_id.to_string_owner()]);}
    if current==payload.value{return protocol::MutationOutcome::empty();}
    protocol::MutationOutcome::new(semio_s_artifact_draw_drawing_diff::layer_base_patch(&payload.layer_id,semio_s_artifact_draw_drawing_diff::DrawingLayerPatch{shape_coordinates:vec![semio_s_artifact_draw_drawing_diff::DrawingShapeCoordinatePatch{field:payload.field,index:payload.index,value:payload.value}],..Default::default()}))
}
