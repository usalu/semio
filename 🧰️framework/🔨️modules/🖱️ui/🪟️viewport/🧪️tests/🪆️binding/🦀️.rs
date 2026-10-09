//! 🪟️ Canonical viewport records retain ordinary projection and explicit controlled refusals.
use crate::{Viewport2d,Viewport3dOrbit};
use semio_framework_dsl_record::{DslField,FieldValue,Shape};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl};
#[path="../../../../🗣️dsl/🧬️schema/🪆️binding/🏛️ownership/🧪️tests/🦀️.rs"]
mod oracle;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap()}
fn float_json(value:&serde_json::Value)->serde_json::Value{match value{serde_json::Value::Number(_)=>serde_json::to_value(serde_json::from_value::<f64>(value.clone()).unwrap()).unwrap(),serde_json::Value::Array(values)=>values.iter().map(float_json).collect(),serde_json::Value::Object(values)=>values.iter().map(|(key,value)|(key.clone(),float_json(value))).collect(),_=>value.clone()}}
fn expected_fields(value:&serde_json::Value)->serde_json::Value{value.as_array().unwrap().iter().map(|row|serde_json::json!([row[0],float_json(&row[1])])).collect()}
fn check<T:DslField+serde::de::DeserializeOwned+serde::Serialize+std::fmt::Debug+PartialEq>(case:&serde_json::Value,fixture:&serde_json::Value){
 let pose:T=serde_json::from_value(case["value"].clone()).unwrap();let Shape::Record(producer)=T::shape() else{panic!("viewport record")};let spec=(producer.ordinary)();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(usize::MAX,&mut allow);let decoded=(producer.decoding)(&mut decoding).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(usize::MAX,&mut allow);let encoded=(producer.encoding)(&mut encoding).unwrap();assert_eq!(oracle::schema(&decoded),oracle::schema(&spec));assert_eq!(oracle::schema(&encoded),oracle::schema(&spec));assert!(decoding.owned_bytes()>0);assert!(encoding.owned_bytes()>0);let fields:Vec<_>=spec.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"shape":match field.shape{Shape::Float=>"Float",Shape::Tuple(_,Some(3))=>"Tuple3",_=>panic!("closed viewport shape")},"optional":field.optional})).collect();assert_eq!(serde_json::json!(fields),fixture["fields"][case["type"].as_str().unwrap()]);
 let value=T::to_value(&pose);let FieldValue::Record(record)=&value else{panic!("viewport record value")};assert_eq!(oracle::fields(record),expected_fields(&case["expectedFields"]));assert_eq!(T::from_value(&value).unwrap(),pose);assert_eq!(serde_json::to_value(&pose).unwrap(),float_json(&case["value"]));
 let mut allow=|_|true;let mut schema=NativeDecodeControl::new(0,&mut allow);assert_eq!((producer.decoding)(&mut schema).unwrap_err().kind.as_str(),fixture["controls"]["metadataZero"].as_str().unwrap());let mut deny=|_|false;let mut canceled=NativeDecodeControl::new(usize::MAX,&mut deny);assert_eq!(T::shape_controlled(&mut canceled).unwrap_err().kind.as_str(),fixture["controls"]["initialCancel"].as_str().unwrap());
 let mut allow=|_|true;let mut output=NativeEncodeControl::new(usize::MAX,&mut allow);assert_eq!(T::to_value_controlled(&pose,&mut output).unwrap_err().kind.as_str(),fixture["controls"]["projection"].as_str().unwrap());let mut allow=|_|true;let mut input=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(T::from_value_controlled(&value,&mut input).unwrap_err().kind.as_str(),fixture["controls"]["construction"].as_str().unwrap());println!("[DEBUG] canonical viewport binding case={} schema and explicit conversion refusals retained",case["id"]); }
#[test]
fn viewport_canonical_record_owner_preserves_planar_and_orbit_projection(){let fixture=fixture();for case in fixture["cases"].as_array().unwrap(){if case["type"]=="Viewport2d"{check::<Viewport2d>(case,&fixture)}else{check::<Viewport3dOrbit>(case,&fixture)}}}
#[test]
fn viewport_canonical_record_owner_rejects_invalid_pose_and_foreign_fields(){let value=Viewport3dOrbit{position:[8.0,-3.0,5.0],target:[0.0;3],zoom:1.25,up:None};for invalid in [0.0,-1.0,f64::NAN,f64::INFINITY]{let FieldValue::Record(mut record)=<Viewport3dOrbit as DslField>::to_value(&value)else{unreachable!()};record.fields.insert(3,FieldValue::Float(invalid));assert!(<Viewport3dOrbit as DslField>::from_value(&FieldValue::Record(record)).is_err());}let FieldValue::Record(mut record)=<Viewport3dOrbit as DslField>::to_value(&value)else{unreachable!()};record.fields.insert(42,FieldValue::Text("foreign locale".into()));assert!(<Viewport3dOrbit as DslField>::from_value(&FieldValue::Record(record)).is_err());}

/// 🫳️ Static viewport metadata preserves the schema-owned ordinary Record fields.
#[test]
fn viewport_borrowed_metadata_matches_the_neutral_record_law() {
    use semio_framework_dsl_record::{BorrowedDslField, BorrowedDslRecord, BorrowedShape};
    fn check<T:BorrowedDslField+BorrowedDslRecord>(name:&str, fixture:&serde_json::Value) {
        let BorrowedShape::Record(produce) = T::SHAPE else { panic!("viewport retains its Record role") };
        let spec = produce();
        assert!(spec.keyword.is_none());
        assert!(matches!(spec.layout,semio_framework_dsl_record::RecordLayout::Inline));
        assert_eq!(spec.fields.len(),T::RECORD.fields.len());
        let fields:Vec<_> = spec.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"shape":match field.shape { BorrowedShape::Float=>"Float", BorrowedShape::Tuple(inner,Some(3)) if matches!(inner(),BorrowedShape::Float)=>"Tuple3", _=>panic!("closed viewport shapes retain their typed roles") },"optional":field.optional})).collect();
        assert_eq!(serde_json::json!(fields),fixture["fields"][name]);
    }
    let fixture=fixture();
    check::<Viewport2d>("Viewport2d",&fixture);
    check::<Viewport3dOrbit>("Viewport3dOrbit",&fixture);
}

/// 🌲️ Native canonical children borrow actual planar fields and agree with independent Serde.
#[test]
fn viewport_native_canonical_tree_borrows_original_planar_schema_fields(){
 use semio_framework_pack_json::{ArtifactCanonicalJsonTree as Tree,ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text};
 let fixture=fixture();
 for case in fixture["cases"].as_array().unwrap().iter().filter(|case|case["type"]=="Viewport2d"){
  let pose:Viewport2d=serde_json::from_value(case["value"].clone()).unwrap();assert!(matches!(pose.canonical_tree_node().unwrap(),Node::Object(3)));
  let mut projected=serde_json::Map::new();
  for(ordinal,(key,field))in[("x",&pose.x),("y",&pose.y),("zoom",&pose.zoom)].into_iter().enumerate(){
   let child=pose.canonical_tree_child(ordinal).unwrap();assert_eq!(child as *const dyn Tree as *const (),field as *const f64 as *const ());let Node::F64(value)=child.canonical_tree_node().unwrap()else{panic!("original native float child")};let Text::Contiguous(actual_key)=pose.canonical_tree_key(ordinal).unwrap()else{panic!("schema-owned contiguous key")};assert_eq!(actual_key,key);projected.insert(key.into(),serde_json::to_value(value).unwrap());
  }
  assert_eq!(serde_json::Value::Object(projected),serde_json::to_value(pose).unwrap());assert!(pose.canonical_tree_child(3).is_err());assert!(pose.canonical_tree_key(3).is_err());
  println!("[DEBUG] Viewport2d original native x/y/zoom borrowedFields=true independentSerde=true case={}",case["id"]);
 }
 let poses:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪟️poses/🔣️.json")).unwrap();
 for case in poses["cases"].as_array().unwrap().iter().filter(|case|case["dimension"]=="2d"&&case["value"].as_object().is_some_and(|fields|fields.len()==3&&["x","y","zoom"].iter().all(|key|fields.get(*key).is_some_and(|value|value.as_f64().is_some())))){
  let value=&case["value"];let pose=Viewport2d{x:value["x"].as_f64().unwrap(),y:value["y"].as_f64().unwrap(),zoom:value["zoom"].as_f64().unwrap()};assert_eq!(pose.canonical_tree_node().is_ok(),serde_json::from_value::<Viewport2d>(value.clone()).is_ok());assert_eq!(pose.canonical_tree_node().is_ok(),case["valid"].as_bool().unwrap());
 }
 for value in [f64::NAN,f64::INFINITY,f64::NEG_INFINITY]{for pose in [Viewport2d{x:value,..Default::default()},Viewport2d{y:value,..Default::default()},Viewport2d{zoom:value,..Default::default()}]{assert!(pose.canonical_tree_node().is_err());}}
}
