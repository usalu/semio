//! 🧪️ Shared scalar fixtures verify history, sparse composition, and wire parity.
use semio_framework_test_fixtures::fixture_text;

#[test]
fn shape_coordinates_preserve_sparse_history() {
 use protocol::{ColdMutation, Mutation,ColdDiffAlgebra, DiffAlgebra};
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/🔷️set-shape-coordinate/🔣️.json")).unwrap();
 let layer=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Rectangle")).to_string().into()).expect("nonempty authored identity"), "Rectangle");let id=semio_s_artifact_draw_drawing_model::layer_id(&layer).clone();
 let before=semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot{layers:vec![layer].into(),..Default::default()};
 let mut composed=semio_s_artifact_draw_drawing_diff::DrawingDiff::default();
 for edit in fixture["edits"].as_array().unwrap(){
  let field=semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::parse(edit["field"].as_str().unwrap()).unwrap();let value=edit["value"].as_f64().unwrap();
  let mutation=semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate(id.clone(),field,None,value);
  store::os_store::test_support::assert_op_line_round_trip(&mutation);store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
  let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());
  let patch=&outcome.diff().layers.as_ref().unwrap().modified[0].patch;assert_eq!(patch.shape_coordinates.len(),1);assert!(patch.layer.is_none()&&patch.transform.is_none());
  let mut after=protocol::apply_diff_cold(outcome.diff(),&before).unwrap();let semio_s_artifact_draw_drawing_model::DrawingLayerNode::Shape(shape)=&after.layers[0] else{panic!()};assert_eq!(semio_s_artifact_draw_drawing_model::shape_geometry::shape_coordinate(shape,&field,None).unwrap(),value);
  for inverse in mutation.inverse(&before).unwrap(){semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after,&inverse).unwrap();}assert_eq!(after,before);
  composed.absorb(outcome.diff().clone());
 }
 let after=protocol::apply_diff_cold(&composed,&before).unwrap();let semio_s_artifact_draw_drawing_model::DrawingLayerNode::Shape(shape)=&after.layers[0] else{panic!()};
 for edit in fixture["edits"].as_array().unwrap(){let field=semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::parse(edit["field"].as_str().unwrap()).unwrap();assert_eq!(semio_s_artifact_draw_drawing_model::shape_geometry::shape_coordinate(shape,&field,None).unwrap(),edit["value"].as_f64().unwrap());}
 for edit in fixture["invalid"].as_array().unwrap(){let field=semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::parse(edit["field"].as_str().unwrap()).unwrap();assert!(!semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate(id.clone(),field,None,edit["value"].as_f64().unwrap()).diff(&before).messages().is_empty());}
}
