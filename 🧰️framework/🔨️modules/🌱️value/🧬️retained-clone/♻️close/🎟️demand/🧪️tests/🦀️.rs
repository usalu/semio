use super::*;
use semio_framework_test_fixtures::fixture_text;
use crate::observe_retirement_allocations;
#[path="🧫️owner/🦀️.rs"]
mod refusing_owner;
use refusing_owner::RefusingCopyDemand;
use crate::effect::{self,CPU};
use semio_framework_value_retirement::OriginalRetirementFailure as Failure;
const WORK:u64=4096;
/// ♻️ Runs one funded Source close turn through the real original action.
fn close_source<T:RetainedClone+Sync+'static>(source:&mut RetainedCloneSource<T>,body:usize,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{effect::turn::<Failure>(grant,CPU,&mut |e,r|source.close_into(body,e,r))}
fn grant(demand:crate::RetirementDemand,body:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
#[derive(crate::RetainedClone,crate::RetireOwned)]#[value(crate="crate")]
struct NestedRecord {body:Option<Vec<Vec<Vec<Vec<Vec<Vec<u8>>>>>>>}

/// 🌳️ Actual derived and container cursors preserve source custody and release only under paid turns.
#[test]
fn original_nested_close_preserves_paid_source_and_cancellation(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture["cases"].as_array().unwrap().len(),6);
 let body=fixture["native"]["closeBody"].as_u64().unwrap()as usize;let original_bytes=fixture["native"]["originalBytes"].as_u64().unwrap()as usize;
 for cut in fixture["native"]["cuts"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize){
  let(original,original_heap)=crate::observe_retirement_allocations(||NestedRecord{body:Some(vec![vec![vec![vec![vec![vec![7u8;original_bytes]]]]]])});
  let((mut source,receipt,_),heap)=semio_framework_value_clone_source_testing::observe_fixture_source_allocations(||effect::admit(original));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));let(mut born,mut released)=(original_heap.0+heap.0,original_heap.1+heap.1);
  let pointer=source.borrow().get().body.as_ref().unwrap()[0][0][0][0][0].as_ptr();let mut cursor=NestedRecord::retained_clone_cursor();
  for _ in 0..cut{if cursor.output_ready(){break;}let demand=cursor.advance_demands(source.borrow(),1,WORK).unwrap();let(step,heap)=crate::observe_retirement_allocations(||effect::advance(&mut cursor,source.borrow(),1,WORK,grant(demand,1)).result.unwrap());let p=step.progress();assert!(p.fits(grant(demand,1)));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;released+=heap.1;}
  effect::begin_close(&mut cursor);assert!(cursor.close_started());let mut turns=0;
  while !cursor.terminal_is_empty(){let(demand,heap)=crate::observe_retirement_allocations(||retained_clone_close_demands(&cursor,body).unwrap());assert_eq!(heap,(0,0));let funded=grant(demand,body);
   for denied in[RetainedCloneGrant{maximum_items:0,..funded},RetainedCloneGrant{maximum_depth:0,..funded}]{let(step,heap)=crate::observe_retirement_allocations(||effect::close_selected(&mut cursor,body,denied).result.unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());}
   let(step,heap)=crate::observe_retirement_allocations(||effect::close_selected(&mut cursor,body,funded).result.unwrap());let p=step.progress();assert!(p.fits(funded));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;released+=heap.1;turns+=1;assert!(turns<100000);
  }
  assert_eq!(source.borrow().get().body.as_ref().unwrap()[0][0][0][0][0].as_ptr(),pointer);assert_eq!(crate::observe_retirement_allocations(||drop(cursor)).1,(0,0));
  while !source.terminal_is_empty(){let demand=crate::RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(body).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()};let funded=grant(demand,body);let(step,heap)=crate::observe_retirement_allocations(||close_source(&mut source,body,funded).result.unwrap());let p=step.progress();assert!(p.fits(funded));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;released+=heap.1;}
  assert_eq!(crate::observe_retirement_allocations(||drop(source)).1,(0,0));assert_eq!(born,released);
 }
}

#[derive(crate::RetainedClone,crate::RetireOwned)]#[value(crate="crate")]
struct DerivedOriginalRecord { small:u8, wide:u64 }
#[derive(crate::RetainedClone,crate::RetireOwned)]#[value(crate="crate")]
enum DerivedOriginalEnum { Empty, Payload { count:u32, record:DerivedOriginalRecord } }

fn denied_derived_grant(mut funded:RetainedCloneGrant,demand:crate::RetirementDemand,axis:&str)->Option<RetainedCloneGrant>{
 let(value,minimum)=match axis{"items"=>(&mut funded.maximum_items,1),"copy"=>(&mut funded.maximum_copy_bytes,demand.copy_bytes),"capacity"=>(&mut funded.maximum_capacity_bytes,demand.capacity_bytes),"release"=>(&mut funded.maximum_release_bytes,demand.release_bytes),"depth"=>(&mut funded.maximum_depth,demand.depth),_=>panic!("unknown neutral physical axis")};
 if minimum==0{return None;}*value=minimum-1;Some(funded)
}

fn original_derived_custody<T:RetainedClone>(original:T,cut:usize,axes:&[serde_json::Value]){
 let((mut source,receipt,_),heap)=semio_framework_value_clone_source_testing::observe_fixture_source_allocations(||effect::admit(original));
 assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));let(mut born,mut released)=heap;
 let(mut cursor,heap)=crate::observe_retirement_allocations(T::retained_clone_cursor);assert_eq!(heap,(0,0));
 for _ in 0..cut{
  if cursor.output_ready(){break;}
  let(demand,heap)=crate::observe_retirement_allocations(||cursor.advance_demands(source.borrow(),1,WORK).unwrap());assert_eq!(heap,(0,0));let funded=grant(demand,1);
  for axis in axes{if let Some(denied)=denied_derived_grant(funded,demand,axis.as_str().unwrap()){
   let(step,heap)=crate::observe_retirement_allocations(||effect::advance(&mut cursor,source.borrow(),1,WORK,denied).result.unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());
  }}
  let(step,heap)=crate::observe_retirement_allocations(||effect::advance(&mut cursor,source.borrow(),1,WORK,funded).result.unwrap());let progress=step.progress();assert!(progress.fits(funded));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;released+=heap.1;
 }
 effect::begin_close(&mut cursor);assert!(cursor.close_started());assert!(!cursor.output_ready());
 for turn in 0..100000{
  if cursor.terminal_is_empty(){break;}
  assert!(turn<99999,"original derived close stalled");
  let(demand,heap)=crate::observe_retirement_allocations(||cursor.close_demands(4096).unwrap());assert_eq!(heap,(0,0));let funded=grant(demand,4096);
  for axis in axes{if let Some(denied)=denied_derived_grant(funded,demand,axis.as_str().unwrap()){
   let(step,heap)=crate::observe_retirement_allocations(||effect::close_step(&mut cursor,denied).result.unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());
  }}
  let(step,heap)=crate::observe_retirement_allocations(||effect::close_step(&mut cursor,funded).result.unwrap());let progress=step.progress();assert!(progress.fits(funded));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;released+=heap.1;
 }
 assert!(cursor.terminal_is_empty());assert_eq!(crate::observe_retirement_allocations(||drop(cursor)).1,(0,0));
 for turn in 0..100000{
  if source.terminal_is_empty(){break;}
  assert!(turn<99999,"original derived source close stalled");
  let demand=crate::RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()};let funded=grant(demand,4096);
  let(step,heap)=crate::observe_retirement_allocations(||close_source(&mut source,4096,funded).result.unwrap());let progress=step.progress();assert!(progress.fits(funded));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;released+=heap.1;
 }
 assert!(source.terminal_is_empty());assert_eq!(crate::observe_retirement_allocations(||drop(source)).1,(0,0));assert_eq!(born,released);
}

/// 🪄️ Actual generated record and enum cursors retain separately funded transfers and every denied original frontier.
#[test]
fn derived_original_record_and_enum_preflight_all_axes_and_close_exact_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let axes=fixture["derive"]["denials"].as_array().unwrap();
 for cut in fixture["derive"]["cuts"].as_array().unwrap().iter().map(|cut|cut.as_u64().unwrap()as usize){
  original_derived_custody(DerivedOriginalRecord{small:7,wide:37},cut,axes);
  original_derived_custody(DerivedOriginalEnum::Payload{count:3,record:DerivedOriginalRecord{small:7,wide:37}},cut,axes);
  original_derived_custody(DerivedOriginalEnum::Empty,cut,axes);
 }
}

#[test]
fn retained_clone_erased_copy_demand_propagates_original_owner_refusal(){
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧬️retained-clone/🧪️tests/🧫️fixtures/🪜️close-demands/🔣️.json")).unwrap();
    let owner=RefusingCopyDemand{owner:Some(law["owner"].as_u64().unwrap())};
    let(result,heap)=observe_retirement_allocations(||crate::ErasedSnapshotRetirement::next_copy_byte_demand(&owner));
    assert_eq!(heap,(0,0));assert_eq!(format!("{:?}",result.unwrap_err().kind),law["copyDemandRefusal"].as_str().unwrap());assert_eq!(owner.owner,Some(law["owner"].as_u64().unwrap()));
    let mut ticket:Option<Box<dyn crate::ErasedSnapshotRetirement>>=Some(Box::new(owner));
    let(result,heap)=observe_retirement_allocations(||crate::factory_ticket_demands(ticket.as_ref().unwrap(),0));assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind,crate::ValueRefusalKind::UnsupportedOwner);assert!(!ticket.as_ref().unwrap().terminal_is_empty());
    let step=effect::driven(RetainedCloneGrant{maximum_items:1,..Default::default()},|e|crate::close_factory_ticket(&mut ticket,0,e)).0.result.unwrap();assert!(matches!(step,RetainedCloneStep::Progress(_)));assert!(ticket.as_ref().unwrap().terminal_is_empty());
    let demand=crate::factory_ticket_demands(ticket.as_ref().unwrap(),0).unwrap();assert_eq!(demand.depth,1);
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(step,heap)=observe_retirement_allocations(||effect::driven(grant,|e|crate::close_factory_ticket(&mut ticket,0,e)).0.result.unwrap());assert_eq!(heap,(0,step.progress().released_bytes));assert!(ticket.is_none());assert!(matches!(step,RetainedCloneStep::Progress(_)));assert!(matches!(effect::driven(Default::default(),|e|crate::close_factory_ticket(&mut ticket,0,e)).0.result.unwrap(),RetainedCloneStep::Complete(_)));
}

#[test]
fn selected_close_start_denies_zero_copy_and_repeats_without_work(){
 use crate::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneGrant};
 let law:serde_json::Value=serde_json::from_str(include_str!("close-start.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let mut cursor=u8::retained_clone_cursor();
  if row["started"].as_bool().unwrap(){let demand=cursor.begin_close_demand().unwrap();effect::begin_close_paid(&mut cursor,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:demand.depth,..Default::default()}).result.unwrap();}
  let grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:row["copy"].as_u64().unwrap()as usize,maximum_depth:row["depth"].as_u64().unwrap()as usize,..Default::default()};
  let maximum=row["maximumWork"].as_u64().unwrap();let prior=row["priorWork"].as_u64().unwrap();
  let(turn,heap)=crate::observe_retirement_allocations(||effect::turn::<Failure>(grant,maximum,&mut |effect,reply|{if prior>0{assert!(effect.performed(prior)?);}cursor.begin_close_into(effect,reply)}));
  assert_eq!(heap,(0,0));assert_eq!(turn.work-u128::from(prior),u128::from(row["work"].as_u64().unwrap()));assert_eq!(turn.result.unwrap().progress().copied_bytes,row["bytes"].as_u64().unwrap()as usize);
  assert_eq!(cursor.close_started(),row["started"].as_bool().unwrap()||row["changed"].as_bool().unwrap());
  let demand=cursor.begin_close_demand().unwrap();effect::begin_close_paid(&mut cursor,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:demand.depth,..Default::default()}).result.unwrap();
  assert!(cursor.terminal_is_empty());
  let turn=effect::begin_close_paid(&mut cursor,Default::default());assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());
 }
}
