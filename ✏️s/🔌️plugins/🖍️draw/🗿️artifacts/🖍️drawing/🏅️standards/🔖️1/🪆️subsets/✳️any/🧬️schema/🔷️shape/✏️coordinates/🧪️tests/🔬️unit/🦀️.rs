//! 🧪️ Every authored shape coordinate preserves all sibling geometry fields.
use super::*;
#[test]
fn shape_coordinates_shared_fixtures_and_rejection_atomicity() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let kind=row["kind"].as_str().unwrap();let field=ShapeCoordinateField::parse(row["field"].as_str().unwrap()).unwrap();
        assert_eq!(field.as_str(),row["field"].as_str().unwrap());
        let crate::DrawingLayerNode::Shape(template)=crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Test")).to_string().into()).expect("nonempty authored identity"), "Test") else {unreachable!()};
        let mut shape=DrawingShapeBody {base:template.base,shape_kind:kind.into(),rect:None,ellipse:None,circle:None,line:None,polygon:None};
        match kind {"rect"=>shape.rect=Some(serde_json::from_value(row["before"].clone()).unwrap()),"ellipse"=>shape.ellipse=Some(serde_json::from_value(row["before"].clone()).unwrap()),"circle"=>shape.circle=Some(serde_json::from_value(row["before"].clone()).unwrap()),"line"=>shape.line=Some(serde_json::from_value(row["before"].clone()).unwrap()),"polygon"=>shape.polygon=Some(serde_json::from_value(row["before"].clone()).unwrap()),_=>unreachable!()}
        let before=shape.clone();let index=row["index"].as_u64().map(|value|value as usize);let value=row["value"].as_f64().unwrap();
        if row["error"]==true {assert!(set_shape_coordinate(&mut shape,field,index,value).is_err());assert_eq!(shape,before);continue;}
        let previous=shape_coordinate(&shape,&field,index).unwrap();set_shape_coordinate(&mut shape,field,index,value).unwrap();
        assert_eq!(shape_coordinate(&shape,&field,index).unwrap(),value);assert_eq!(serde_json::to_value(&shape).unwrap()[kind],row["after"]);
        set_shape_coordinate(&mut shape,field,index,previous).unwrap();assert_eq!(shape,before);
        assert!(set_shape_coordinate(&mut shape,field,index,f64::NAN).is_err());assert_eq!(shape,before);
    }
}
