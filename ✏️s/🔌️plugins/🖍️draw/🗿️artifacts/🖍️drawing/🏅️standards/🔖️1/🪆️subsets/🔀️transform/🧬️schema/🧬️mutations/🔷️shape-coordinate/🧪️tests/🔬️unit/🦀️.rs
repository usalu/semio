//! 🧪️ Shared scalar fixtures verify history, sparse composition, and wire parity.
#[test]
fn shape_coordinates_preserve_sparse_history() {
 use protocol::{Mutation,DiffAlgebra};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 let layer=crate::schema::create_drawing_shape_layer_rect("Rectangle");let id=crate::schema::layer_id(&layer).clone();
 let before=crate::DrawingSnapshot{layers:vec![layer].into(),..Default::default()};
 let mut composed=crate::DrawingDiff::default();
 for edit in fixture["edits"].as_array().unwrap(){
  let field=crate::schema::shape_geometry::ShapeCoordinateField::parse(edit["field"].as_str().unwrap()).unwrap();let value=edit["value"].as_f64().unwrap();
  let mutation=super::mutation::set_shape_coordinate(id.clone(),field,None,value);
  store::os_store::test_support::assert_op_line_round_trip(&mutation);store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
  let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());
  let patch=&outcome.diff().layers.as_ref().unwrap().modified[0].patch;assert_eq!(patch.shape_coordinates.len(),1);assert!(patch.layer.is_none()&&patch.transform.is_none());
  let mut after=protocol::apply_diff(outcome.diff(),&before).unwrap();let crate::DrawingLayerNode::Shape(shape)=&after.layers[0] else{panic!()};assert_eq!(crate::schema::shape_geometry::shape_coordinate(shape,&field,None).unwrap(),value);
  for inverse in mutation.inverse(&before).unwrap(){crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut after,&inverse).unwrap();}assert_eq!(after,before);
  composed.absorb(outcome.diff().clone());
 }
 let after=protocol::apply_diff(&composed,&before).unwrap();let crate::DrawingLayerNode::Shape(shape)=&after.layers[0] else{panic!()};
 for edit in fixture["edits"].as_array().unwrap(){let field=crate::schema::shape_geometry::ShapeCoordinateField::parse(edit["field"].as_str().unwrap()).unwrap();assert_eq!(crate::schema::shape_geometry::shape_coordinate(shape,&field,None).unwrap(),edit["value"].as_f64().unwrap());}
 for edit in fixture["invalid"].as_array().unwrap(){let field=crate::schema::shape_geometry::ShapeCoordinateField::parse(edit["field"].as_str().unwrap()).unwrap();assert!(!super::mutation::set_shape_coordinate(id.clone(),field,None,edit["value"].as_f64().unwrap()).diff(&before).messages().is_empty());}
}
