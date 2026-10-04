//! 💰️ Full concrete allocation requests for the owning Kernel controlled schema authority.
#[path="../🦀️.rs"]
mod semantic;
use semantic::*;
use semio_framework_dsl_record::{FieldSpec,NativeSchemaControl,RecordLayout,RecordSpec,RecordSpecProducer,Shape};
use crate::os_pack::{schema_hash,PackSchemaGraph};

fn mutual_left_spec() -> RecordSpec { RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0,"name",Shape::Text),FieldSpec::new(1,"next",Shape::Record(mutual_right_producer()))]) }
fn mutual_right_spec() -> RecordSpec { RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0,"tag",Shape::UInt),FieldSpec::new(1,"next",Shape::Record(mutual_left_producer()))]) }
fn mutual_left_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{scalar_spec_controlled(None,RecordLayout::Inline,[(0,"name",Shape::Text),(1,"next",Shape::Record(mutual_right_producer()))],control)}
fn mutual_right_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{scalar_spec_controlled(None,RecordLayout::Inline,[(0,"tag",Shape::UInt),(1,"next",Shape::Record(mutual_left_producer()))],control)}
fn mutual_left_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:mutual_left_spec,decoding:|control|mutual_left_controlled(control),encoding:|control|mutual_left_controlled(control)}}
fn mutual_right_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:mutual_right_spec,decoding:|control|mutual_right_controlled(control),encoding:|control|mutual_right_controlled(control)}}
fn equivalent_inner_spec()->RecordSpec{inner_spec()}
fn equivalent_inner_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:equivalent_inner_spec,decoding:|control|inner_controlled(control),encoding:|control|inner_controlled(control)}}

#[test]
fn schema_hash_controlled_full_allocator_requests_and_same_caller_are_admitted(){
 use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,ValueRefusalKind};
 let contract:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎒️pack/🌱️value/🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json")).unwrap();
 assert_eq!(contract["requests"],"fullSystemAllocatorRequests");
 assert_eq!(contract["cumulative"],"sameCallerNoRefund");
 assert_eq!(contract["failureChannel"]["diagnostic"],"actualReturnedMessageCapacity");
 assert_eq!(contract["failureChannel"]["admission"],"mayPrecedeCanceledMaterialization");
 let fixture=schema_hash_fixture();
 let mut specimens:Vec<(String,RecordSpec)>=fixture["cases"].as_array().unwrap().iter().map(|sample|{let name=sample["name"].as_str().unwrap();(name.into(),schema_hash_case_spec(name))}).collect();
 specimens.push(("empty".into(),RecordSpec::new(None,RecordLayout::Inline,Vec::new())));
 specimens.push(("mutualRecursive".into(),mutual_left_spec()));
 specimens.push(("broadRepeatedEdges".into(),RecordSpec::new(None,RecordLayout::Inline,(0..512u16).map(|id|FieldSpec::new(id,&format!("edge{id}"),Shape::Record(if id%2==0{inner_producer()}else{equivalent_inner_producer()}))).collect())));
 assert_eq!(specimens.iter().map(|(name,_)|name.as_str()).collect::<Vec<_>>(),contract["cases"].as_array().unwrap().iter().map(|name|name.as_str().unwrap()).collect::<Vec<_>>());
 for(name,spec)in specimens{
  let graph=PackSchemaGraph::of(&spec);
  let canonical=independent_canonical_bytes(&fixture["shapeTags"],&graph_json(&graph));
  let expected=*blake3::hash(&canonical).as_bytes();
  assert_eq!(schema_hash(&spec),expected,"{name} independent semantic oracle");
  if let Some(sample)=fixture["cases"].as_array().unwrap().iter().find(|sample|sample["name"]==name){assert_eq!(graph_json(&graph),sample["graph"]);assert_eq!(hex_of(&expected),sample["schemaHash"].as_str().unwrap());}
  if name=="empty"{assert_eq!(graph.records.len(),1);}
  if name=="mutualRecursive"{assert_eq!(graph.records.len(),2);}
  if name=="broadRepeatedEdges"{assert_eq!(graph.records.len(),2);}
  macro_rules! check{
   ($control:ident,$progress:ty)=>{{
    let mut accept=|_: $progress|true;let mut control=$control::new(usize::MAX/4,&mut accept);
    let(result,requested,released)=crate::test_allocation::observe_backing(||crate::os_pack::schema_hash_controlled(&spec,&mut control));
    assert_eq!(result.unwrap(),expected,"{name} controlled semantic digest");
    let debit=control.owned_bytes();assert!(requested>0,"{name} real schema scratch");
    assert_eq!(debit,requested,"{name} {} must admit complete concrete requests exactly",stringify!($control));
    assert_eq!(released,requested,"{name} schema scratch must release completely");
    let mut accept=|_: $progress|true;let mut exact=$control::new(requested,&mut accept);
    let(result,actual)=crate::test_allocation::observe(||crate::os_pack::schema_hash_controlled(&spec,&mut exact));
    assert_eq!(result.unwrap(),expected);assert_eq!(actual,requested);assert_eq!(exact.owned_bytes(),requested);
    let mut accept=|_: $progress|true;let mut zero=$control::new(0,&mut accept);
    let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::os_pack::schema_hash_controlled(&spec,&mut zero).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error);summary});
    assert_eq!(kind,ValueRefusalKind::OwnershipLimit);
    assert_eq!(actual,diagnostic);assert_eq!(zero.owned_bytes(),0);assert_eq!(released,actual);
    let mut accept=|_: $progress|true;let mut short=$control::new(requested-1,&mut accept);
    let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::os_pack::schema_hash_controlled(&spec,&mut short).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error);summary});
    assert_eq!(kind,ValueRefusalKind::OwnershipLimit);assert!(actual<=short.owned_bytes().checked_add(diagnostic).unwrap());assert!(short.owned_bytes()<requested);assert_eq!(released,actual);
    let combined=requested.checked_mul(2).unwrap();
    let mut accept=|_: $progress|true;let mut pair=$control::new(combined,&mut accept);
    let((first,second),actual)=crate::test_allocation::observe(||{let first=crate::os_pack::schema_hash_controlled(&spec,&mut pair);let second=crate::os_pack::schema_hash_controlled(&spec,&mut pair);(first,second)});
    assert_eq!(first.unwrap(),expected);assert_eq!(second.unwrap(),expected);assert_eq!(actual,combined);assert_eq!(pair.owned_bytes(),combined);
    let mut accept=|_: $progress|true;let mut pair=$control::new(combined-1,&mut accept);
    let((first,kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let first=crate::os_pack::schema_hash_controlled(&spec,&mut pair).unwrap();let error=crate::os_pack::schema_hash_controlled(&spec,&mut pair).unwrap_err().into_value_error();let summary=(first,error.kind,error.message.capacity());drop(error);summary});
    assert_eq!(first,expected);assert_eq!(kind,ValueRefusalKind::OwnershipLimit);assert!(actual<=pair.owned_bytes().checked_add(diagnostic).unwrap());assert!(pair.owned_bytes()<combined);assert_eq!(released,actual);
    for boundary in [0,1,2]{
     let mut reached=false;let mut cancel=|event:$progress|{if boundary==0||(event.owned_bytes>0&&(boundary==1||event.completed>0)){reached=true;false}else{true}};
     let mut canceled=$control::new(usize::MAX/4,&mut cancel);
     let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::os_pack::schema_hash_controlled(&spec,&mut canceled).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error);summary});
     assert_eq!(kind,ValueRefusalKind::Canceled);assert!(actual<=canceled.owned_bytes().checked_add(diagnostic).unwrap());assert_eq!(released,actual);if boundary==0{assert_eq!(canceled.owned_bytes(),0);assert_eq!(actual,diagnostic);}if boundary==2{assert!(actual>diagnostic,"{name} actual materialized scratch cancellation");}drop(canceled);assert!(reached,"{name} real cancellation boundary");
    }
   }};
  }
  check!(NativeEncodeControl,semio_framework_value::native_encoding::NativeEncodeProgress);
  check!(NativeDecodeControl,semio_framework_value::native_decoding::NativeDecodeProgress);
 }
}
