use super::*;
use crate::allocation_testing::observe_retirement_allocations as observe;
use semio_framework_test_fixtures::fixture_text;
#[test]
fn original_vector_native_ordered_handoffs_cancel_every_original_frontier_and_pay_final_backing(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../📋️original-vector/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for stop in law["cancelAfter"].as_array().unwrap(){
  let mut bank=VectorBank::new();
  let(source,birth)=observe(||{let mut values=Vec::with_capacity(37);for value in law["values"].as_array().unwrap(){values.push(value.as_str().unwrap().repeat(8192));}values});
  let pointers:Vec<_>=source.iter().map(|value|value.as_ptr()).collect();let backing=source.as_ptr();let backing_bytes=source.capacity()*size_of::<String>();
  let grant=bank.cap(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:64});
  let mut source=source;
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observe(||bank.paid(OriginalVectorCursor::admit(source,denied)));source=result.result.err().unwrap().1;assert_eq!(heap,(0,0));assert_eq!(source.as_ptr(),backing);assert_eq!(source[0].as_ptr(),pointers[0]);}
  let(result,heap)=observe(||bank.paid(OriginalVectorCursor::admit(source,grant)));let(mut owner,receipt)=result.result.map_err(|(error,_)|error).unwrap();assert_eq!(heap,(0,0));assert!(receipt.fits(grant));assert_eq!(owner.remaining().as_ptr(),backing);
  let mut returned=Vec::new();let mut births=0;let mut releases=0;
  for index in 0..stop.as_u64().unwrap()as usize{let(demand,heap)=observe(||owner.next_take_demand());assert_eq!(heap,(0,0));let(result,heap)=observe(||bank.paid(owner.take_front(RetainedCloneGrant{maximum_items:0,..bank.cap(grant)})).result.unwrap());assert!(result.is_none());assert_eq!(heap,(0,0));assert_eq!(owner.remaining()[0].as_ptr(),pointers[index]);let(result,heap)=observe(||{let g=bank.cap(grant);bank.paid(owner.take_front(g)).result.unwrap()});let(value,receipt)=result.unwrap();assert_eq!(heap,(0,0));assert_eq!(value.as_ptr(),pointers[index]);assert!(receipt.fits(grant));returned.push(value);}
  owner.begin_close();if owner.remaining().is_empty(){let(step,heap)=observe(||bank.paid(owner.step(RetainedCloneGrant{maximum_release_bytes:backing_bytes-1,..grant})).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert!(!owner.terminal_is_empty());}
  for _ in 0..100000{if owner.terminal_is_empty(){break;}let(demand,heap)=observe(||owner.next_demand(grant.maximum_copy_bytes).unwrap());assert_eq!(heap,(0,0));let denied=RetainedCloneGrant{maximum_release_bytes:0,maximum_capacity_bytes:0,maximum_copy_bytes:0,maximum_depth:0,..grant};let(step,heap)=observe(||bank.paid(owner.step(denied)).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let(step,heap)=observe(||{let g=bank.cap(grant);bank.paid(owner.step(g)).result.unwrap()});let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.0;releases+=heap.1;assert!(demand.depth<=grant.maximum_depth);}
  assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
  for value in returned{let mut cursor=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native string custody"));for _ in 0..1000{if cursor.terminal_is_empty(){break;}let(step,heap)=observe(||{let g=bank.cap(grant);let result=cursor.step(g);let work=cursor.last_turn_work();bank.paid(RetainedWorkOutcome{result,work}).result.unwrap()});let receipt=step.progress();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(receipt.fits(grant));births+=heap.0;releases+=heap.1;}assert!(cursor.terminal_is_empty());let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));}
  assert_eq!(releases,birth.0+births);assert!(releases>=backing_bytes);
 }
}

#[test]
fn original_vector_actual_backing_release_is_independent_of_cached_completion(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut bank=VectorBank::new();
 let source=Vec::<u64>::with_capacity(2);
 let bytes=source.capacity()*size_of::<u64>();
 let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:bytes,maximum_depth:64};
 let(mut owner,_)=bank.paid(OriginalVectorCursor::admit(source,g)).result.map_err(|(e,_)|e).unwrap();
 owner.begin_close();
 let mut actual=Vec::new();
 for(label,grant)in[("deniedRelease",RetainedCloneGrant{maximum_release_bytes:bytes-1,..g}),("paidBackingRelease",g),("cachedComplete",g)]{
  let(turn,heap)=observe(||bank.paid(owner.step(grant)));
  let p=turn.result.unwrap().progress();
  assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
  actual.push(serde_json::json!({"label":label,"work":turn.work}));
 }
 assert_eq!(serde_json::Value::Array(actual),law["actualCloseWork"]);
 assert!(owner.terminal_is_empty());
}

#[test]fn actual_original_vector_normal_moves_preserve_input_identity_with_separate_cpu_and_zero_copy(){let law:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();let s=&law["stock"];let mut bank=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let mut fuel=s["fuel"].as_u64().unwrap();let maximum=s["maximumTurns"].as_u64().unwrap();let mut input:Vec<String>=serde_json::from_value(law["values"].clone()).unwrap();let pointer=input.as_ptr();let text=input[0].as_ptr();let(turn,heap)=crate::allocation_testing::observe_retirement_allocations(||OriginalVectorCursor::admit(input,RetainedCloneGrant{maximum_items:0,..bank}));assert_eq!(turn.work,0);assert_eq!(heap,(0,0));input=turn.result.err().unwrap().1;assert_eq!(input.as_ptr(),pointer);assert_eq!(input[0].as_ptr(),text);let(turn,heap)=crate::allocation_testing::observe_retirement_allocations(||OriginalVectorCursor::admit(input,RetainedCloneGrant{maximum_items:1,..bank}));let work=turn.work;let(mut owner,p)=turn.result.map_err(|(e,_)|e).unwrap();assert_eq!(work,1);assert_eq!(p.copied_bytes,0);assert_eq!(heap,(0,0));bank.maximum_items-=p.copied_items;fuel-=work;assert_eq!(owner.remaining().as_ptr(),pointer);assert_eq!(owner.next_take_demand().copy_bytes,0);let denied=owner.take_front(RetainedCloneGrant{maximum_items:0,..bank});assert_eq!(denied.work,0);assert!(denied.result.unwrap().is_none());let(turn,heap)=crate::allocation_testing::observe_retirement_allocations(||owner.take_front(RetainedCloneGrant{maximum_items:1,..bank}));let work=turn.work;let(value,p)=turn.result.unwrap().unwrap();assert_eq!(work,1);assert_eq!(p.copied_bytes,0);assert_eq!(value.as_ptr(),text);assert_eq!(serde_json::to_value(&value).unwrap(),law["values"][0]);assert_eq!(heap,(0,0));bank.maximum_items-=p.copied_items;fuel-=work;let mut returned=ControlledRetirement::new(value).map_err(|(e,_)|e).unwrap();for _ in 0..maximum{if returned.terminal_is_empty(){break}let(step,heap)=crate::allocation_testing::observe_retirement_allocations(||returned.step(RetainedCloneGrant{maximum_items:1,..bank}).unwrap());let p=step.progress();assert!(p.fits(bank));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));fuel=fuel.checked_sub(returned.last_turn_work()).unwrap();bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;bank.maximum_capacity_bytes-=p.retained_capacity_bytes;bank.maximum_release_bytes-=p.released_bytes;}assert!(returned.terminal_is_empty());owner.begin_close();for _ in 0..maximum{if owner.terminal_is_empty(){break}let(turn,heap)=crate::allocation_testing::observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_items:1,..bank}));let work=turn.work;let p=turn.result.unwrap().progress();assert!(p.fits(bank));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));fuel=fuel.checked_sub(work).unwrap();bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;bank.maximum_capacity_bytes-=p.retained_capacity_bytes;bank.maximum_release_bytes-=p.released_bytes;}assert!(owner.terminal_is_empty());assert_eq!(owner.step(Default::default()).work,0);assert!(owner.take_front(Default::default()).result.unwrap().is_none());}

struct VectorBank{g:RetainedCloneGrant,fuel:u64}
impl VectorBank{
 fn new()->Self{let law:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();let s=&law["stock"];Self{g:RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize},fuel:s["fuel"].as_u64().unwrap()}}
 fn cap(&self,g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:g.maximum_items.min(self.g.maximum_items),maximum_copy_bytes:g.maximum_copy_bytes.min(self.g.maximum_copy_bytes),maximum_capacity_bytes:g.maximum_capacity_bytes.min(self.g.maximum_capacity_bytes),maximum_release_bytes:g.maximum_release_bytes.min(self.g.maximum_release_bytes),maximum_depth:g.maximum_depth.min(self.g.maximum_depth)}}
 fn paid<T:VectorReceipt,E:VectorError>(&mut self,t:RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,E>{let p=match &t.result{Ok(v)=>v.physical(),Err(e)=>e.physical()};assert!(p.fits(self.g));self.g.maximum_items-=p.copied_items;self.g.maximum_copy_bytes-=p.copied_bytes;self.g.maximum_capacity_bytes-=p.retained_capacity_bytes;self.g.maximum_release_bytes-=p.released_bytes;self.fuel=self.fuel.checked_sub(t.work).unwrap();t}
}
trait VectorReceipt{fn physical(&self)->RetainedCloneProgress;}
impl<T> VectorReceipt for(T,RetainedCloneProgress){fn physical(&self)->RetainedCloneProgress{self.1}}
impl VectorReceipt for RetainedCloneStep{fn physical(&self)->RetainedCloneProgress{self.progress()}}
impl<T:VectorReceipt> VectorReceipt for Option<T>{fn physical(&self)->RetainedCloneProgress{self.as_ref().map_or(Default::default(),VectorReceipt::physical)}}
trait VectorError{fn physical(&self)->RetainedCloneProgress;}
impl VectorError for ValueError{fn physical(&self)->RetainedCloneProgress{self.retained_progress()}}
impl<T> VectorError for(ValueError,T){fn physical(&self)->RetainedCloneProgress{self.0.retained_progress()}}
