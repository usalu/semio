//! 🧪️ Graph manifest canonical trees equal independent serde_json parses of their `ToValue` wire.
use super::*;
use semio_framework_pack_json::ArtifactCanonicalJsonTreeCursor;
use semio_framework_value::{ToValue,ValueType,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneSource}};

fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(4096),maximum_capacity_bytes:demand.capacity_bytes.max(4096),maximum_release_bytes:demand.release_bytes.max(4096),maximum_depth:demand.depth.max(64)}}

fn canonical<T:Tree+semio_framework_value::retirement::RetireOwned>(root:T)->Vec<u8>{
 let birth=RetainedCloneSource::<T>::owned_constructor_demand::<()>();
 let(mut source,_)=RetainedCloneSource::admit_owned(root,(),grant(RetirementDemand{copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()})).unwrap_or_else(|(error,_,_)|panic!("graph canonical source birth: {error}"));
 let projection=source.project_owned(0,|value|value as&dyn Tree,grant(RetirementDemand{copy_bytes:source.borrow().binding_copy_bytes(),depth:1,..Default::default()})).unwrap().0;
 let(mut cursor,_)=ArtifactCanonicalJsonTreeCursor::admit(projection,grant(ArtifactCanonicalJsonTreeCursor::constructor_demand())).unwrap_or_else(|(error,_)|panic!("graph canonical cursor admission: {error}"));
 let mut output=Vec::new();
 for turn in 0..1_000_000{
  if cursor.terminal_is_empty(){break;}
  assert!(turn<999_999,"graph canonical traversal did not settle");
  let demand=cursor.next_demand().unwrap();let mut byte=[0u8];let step=cursor.advance(&mut byte,grant(demand)).unwrap();output.extend_from_slice(&byte[..step.written_bytes]);
 }
 assert!(cursor.is_complete());drop(cursor);
 for _ in 0..100_000{
  if source.terminal_is_empty(){break;}
  let copy=source.next_close_copy_byte_demand().unwrap();
  source.close_step(grant(RetirementDemand{copy_bytes:copy,capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()})).unwrap();
 }
 assert!(source.terminal_is_empty());
 output
}
fn same_as_wire<T:Tree+ToValue+semio_framework_value::retirement::RetireOwned>(value:T){
 let wire=serde_json::Value::from(value.to_value());
 let actual:serde_json::Value=serde_json::from_slice(&canonical(value)).unwrap();
 assert_eq!(actual,wire);
}

#[test]
fn graph_manifest_canonical_trees_match_their_to_value_wire_through_serde_json(){
 same_as_wire(PropertyKind::Derived);
 same_as_wire(PortDirection::Out);
 let bag=PropertyBag::from([("b".to_string(),PropertyValue::Array(vec![PropertyValue::Null,PropertyValue::Bool(false),PropertyValue::Number(2.5),PropertyValue::String("é🧬\"".into())])),("a".to_string(),PropertyValue::Object(PropertyBag::from([("z".to_string(),PropertyValue::Number(-1.0))])))]);
 same_as_wire(PropertyValue::Object(bag.clone()));
 same_as_wire(bag);
 same_as_wire(PropertyValue::String("scalar".into()));
 for value_type in [ValueType::Boolean,ValueType::Integer,ValueType::Decimal,ValueType::Text,ValueType::Any,ValueType::Schema("row".into()),ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Text))))]{
  same_as_wire(PropertyDef{name:"p".into(),kind:PropertyKind::Data,value_type:value_type.clone(),expr:None});
  same_as_wire(PropertyDef{name:"q".into(),kind:PropertyKind::Derived,value_type,expr:Some("a + 1".into())});
 }
}
