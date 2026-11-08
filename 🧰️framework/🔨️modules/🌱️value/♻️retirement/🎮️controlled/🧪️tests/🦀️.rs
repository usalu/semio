use super::*;
use semio_framework_test_fixtures::fixture_text;
use crate::retirement::effect_fixture::{observed,settled,with_effect};
fn stepped_narrow<T:RetireOwned>(owner:&mut ControlledRetirement<T>,grant:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize),RetainedCloneGrant){let first=stepped(owner,grant);if first.0.progress()!=Default::default(){return(first.0,first.1,grant)}assert_eq!(first.1,(0,0));let need=owner.next_demand(grant.maximum_copy_bytes).unwrap().copy_bytes;let wide=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.max(need),..grant};let second=stepped(owner,wide);(second.0,second.1,wide)}
fn stepped<T:RetireOwned>(owner:&mut ControlledRetirement<T>,grant:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize)){let(step,heap)=observed(grant,|effect|settled(owner.step(effect)).result);(step.expect("original controlled step"),heap)}
struct ExcessAllocation;
unsafe impl crate::list::PageAllocation for ExcessAllocation {
    fn reserve<T>(owner:&mut Vec<T>,slots:usize,actual:Option<&mut Option<usize>>)->Result<(),std::collections::TryReserveError>{let allocating=size_of::<T>()!=0&&owner.capacity()<owner.len().saturating_add(slots*2);let result=owner.try_reserve_exact(slots*2);if allocating{if let Some(actual)=actual{*actual=Some(if result.is_ok(){owner.capacity()*size_of::<T>()}else{0})}}result}
}
#[test]
fn original_controlled_failed_reservation_retains_actual_receipt_and_same_owner(){
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🎮️controlled/🧪️tests/🧫️fixtures/🔣️reservation.json")).unwrap();
    let original=law["original"].as_str().unwrap().to_owned();let pointer=original.as_ptr();let mut owner=ControlledRetirement::new(original).unwrap();
    let quote=owner.next_capacity_byte_demand(4096).unwrap();let reserve_copy=owner.next_demand(4096).unwrap().copy_bytes;
    let(result,heap)=observed(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:quote,maximum_release_bytes:0,maximum_depth:64},|effect|owner.reserve_cursor_slot(effect,|list,effect|list.reserve_one_using::<ExcessAllocation>(effect)));
    let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);let progress=owner.step_progress();assert_eq!(error.retained_progress(),progress);assert_eq!(progress.copied_items,1);assert_eq!(progress.copied_bytes,reserve_copy);assert_eq!(progress.released_bytes,0);assert!(progress.retained_capacity_bytes>quote);assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);assert_eq!(owner.original().unwrap().as_ptr(),pointer);assert_eq!(owner.original().unwrap(),law["original"].as_str().unwrap());
    let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:quote,maximum_release_bytes:262144,maximum_depth:64};
    let(error,heap)=crate::allocation_testing::observe_retirement_allocations(||crate::retained_clone::admit_retained_clone_progress(denied,progress,"original failed reservation").unwrap_err());assert_eq!(error.retained_progress(),progress);assert_eq!(heap,(0,0));
    let admitted=RetainedCloneGrant{maximum_capacity_bytes:progress.retained_capacity_bytes,..denied};let(error,heap)=crate::allocation_testing::observe_retirement_allocations(||crate::retained_clone::admit_retained_clone_close(admitted,RetainedCloneStep::Complete(progress),false,"original failed terminal").unwrap_err());assert_eq!(error.retained_progress(),progress);assert_eq!(heap,(0,0));assert_eq!(owner.original().unwrap().as_ptr(),pointer);
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let mut turns=0;
    while !owner.terminal_is_empty(){turns+=1;assert!(turns<1024);let(step,heap)=stepped(&mut owner,grant);assert_eq!(owner.step_progress(),step.progress());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);}
    let(_,heap)=crate::allocation_testing::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));
}

#[test]
fn original_controlled_root_transfer_preserves_same_source_with_narrow_copy_policy(){
 struct Original{value:String,pointer:usize,metadata:[u8;256]}
 impl RetireOwned for Original{type RetirementCursor=<String as RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{assert_eq!(self.value.as_ptr()as usize,self.pointer);assert_eq!(self.metadata,[17;256]);self.value.retirement_seed()}fn retirement_birth_bytes(&self)->Option<usize>{self.value.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🎮️controlled/🧪️tests/🧫️fixtures/📥️root/🔣️.json")).unwrap();
 for copy in [1,3,64]{let(value,born)=crate::allocation_testing::observe_retirement_allocations(||law["source"].as_str().unwrap().to_owned());let pointer=value.as_ptr();let(mut owner,heap)=crate::allocation_testing::observe_retirement_allocations(||ControlledRetirement::new(Original{pointer:pointer as usize,value,metadata:[17;256]}).ok().unwrap());assert_eq!(heap,(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let mut births=born.0;let mut releases=born.1;
  let mut reserve_turns=0;while !owner.cursors.has_reserved_slot(){reserve_turns+=1;assert!(reserve_turns<1000,"cursor slot reservation stalled");let reserve_copy=owner.next_demand(grant.maximum_copy_bytes).unwrap().copy_bytes;let(step,heap,_)=stepped_narrow(&mut owner,grant);assert_eq!(step.progress().copied_bytes,reserve_copy);assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);births+=heap.0;releases+=heap.1;assert_eq!(owner.original().unwrap().value.as_ptr(),pointer);}
  assert_eq!(owner.next_copy_byte_demand().unwrap(),owner.next_demand(0).unwrap().copy_bytes);let root_copy=crate::retirement::retirement_cursor_birth_demand(owner.original().unwrap()).unwrap().copy_bytes;let(step,heap,used)=stepped_narrow(&mut owner,grant);assert_eq!(step.progress().copied_items,1);assert_eq!(step.progress().copied_bytes,root_copy);assert!(step.progress().fits(used));assert_eq!(owner.step_progress(),step.progress());assert!(owner.original().is_none());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);births+=heap.0;releases+=heap.1;
  let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000);let(step,heap,used)=stepped_narrow(&mut owner,grant);assert!(step.progress().fits(used));assert_eq!(owner.step_progress(),step.progress());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);births+=heap.0;releases+=heap.1;}
  let(_,heap)=crate::allocation_testing::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));assert_eq!(births,releases);
 }
}

#[test]
fn original_controlled_narrow_child_keeps_real_semantic_minimum_and_same_grant(){
 for copy in [1,3,64]{let(original,born)=crate::allocation_testing::observe_retirement_allocations(||vec![u128::MAX;3]);let pointer=original.as_ptr();let mut owner=ControlledRetirement::new(original).unwrap();assert_eq!(owner.original().unwrap().as_ptr(),pointer);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let(mut births,mut releases,mut narrow,mut turns)=(born.0,born.1,false,0);
  while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000);let demand=owner.next_copy_byte_demand().unwrap();let quote=owner.next_demand(copy).unwrap().copy_bytes;let(step,heap)=stepped(&mut owner,RetainedCloneGrant{maximum_items:0,..grant});assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let(step,heap,used)=stepped_narrow(&mut owner,grant);let progress=step.progress();assert!(progress.fits(used));assert_eq!(owner.step_progress(),progress);assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);if demand>copy&&used.maximum_copy_bytes>copy{assert_eq!(used.maximum_copy_bytes,quote);assert_eq!(progress.copied_bytes,quote);narrow=true;}births+=heap.0;releases+=heap.1;
  }
  if copy<16{assert!(narrow);}let(_,heap)=crate::allocation_testing::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));assert_eq!(births,releases);
 }
}


#[test]
fn original_controlled_selected_child_quote_preserves_future_atom_and_exact_currencies(){
 use crate::{factory_ticket_demands,close_factory_ticket};
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🎮️controlled/🧪️tests/🧫️fixtures/🎯️selected/🔣️.json")).unwrap();
 for copy in law["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){let(value,source)=crate::allocation_testing::observe_retirement_allocations(||{let mut value=Vec::<u128>::with_capacity(law["spareCapacity"].as_u64().unwrap()as usize);value.extend(law["source"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().parse::<u128>().unwrap()));value});let pointer=value.as_ptr();let(mut original,heap)=crate::allocation_testing::observe_retirement_allocations(||ControlledRetirement::new(value).unwrap());assert_eq!(heap,(0,0));assert_eq!(original.original().unwrap().as_ptr(),pointer);let(mut born,mut freed,mut turns,mut spills)=(source.0,source.1,0,0);
  let admission=controlled_retirement_birth_demand(&original).unwrap();assert_eq!(admission.capacity_bytes,controlled_retirement_birth_bytes::<ControlledRetirement<Vec<u128>>>());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:admission.copy_bytes,maximum_capacity_bytes:admission.capacity_bytes,maximum_release_bytes:0,maximum_depth:1};let((ticket,receipt),heap)=observed(grant,|effect|{let mut slot=Some(original);admit_typed_controlled_retirement(&mut slot,effect).result.unwrap().expect("admitted typed frame")});assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(receipt.copied_bytes,admission.copy_bytes);born+=heap.0;freed+=heap.1;let mut slot=Some(ticket);
  while let Some(ticket)=slot.as_ref(){let(demand,heap)=crate::allocation_testing::observe_retirement_allocations(||factory_ticket_demands(ticket,copy).unwrap());assert_eq!(heap,(0,0));if !ticket.terminal_is_empty(){let future=ticket.next_copy_byte_demand().unwrap();if future>0{assert_eq!(demand.copy_bytes,future);if copy<future{spills+=1;}}}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}),(demand.release_bytes>8).then_some(RetainedCloneGrant{maximum_release_bytes:8,..grant})].into_iter().flatten(){let(result,heap)=observed(denied,|effect|settled(crate::select_factory_ticket(&mut slot,copy).unwrap().step(effect)));if let Ok(step)=result.result{assert_eq!(step.progress(),Default::default(),"copy={copy} turn={turns} demand={demand:?} denied={denied:?}");}assert_eq!(heap,(0,0));assert_eq!(factory_ticket_demands(slot.as_ref().unwrap(),copy).unwrap(),demand);}
   let(step,heap)=observed(grant,|effect|settled(crate::select_factory_ticket(&mut slot,copy).unwrap().step(effect)).result.unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(grant));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<10000);
  }
  if copy<law["futureAtomBytes"].as_u64().unwrap()as usize{assert!(spills>0);}assert_eq!(born,freed);let(_,heap)=crate::allocation_testing::observe_retirement_allocations(||drop(slot));assert_eq!(heap,(0,0));
 }
}

/// 🧱️ The original array leaf spill preserves its raw Float32 minimum under independent caller currencies.
#[test]
fn original_controlled_selected_array_leaf_preserves_future_atoms_and_exact_currencies(){
 use crate::{factory_ticket_demands,close_factory_ticket};
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🎮️controlled/🧪️tests/🧫️fixtures/🎯️array/🔣️.json")).unwrap();
 for copy in law["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){let(value,source)=crate::allocation_testing::observe_retirement_allocations(||{let mut value=Vec::<[f32;3]>::with_capacity(law["spareCapacity"].as_u64().unwrap()as usize);value.extend(law["source"].as_array().unwrap().iter().map(|value|serde_json::from_value::<[f32;3]>(value.clone()).unwrap()));value});let pointer=value.as_ptr();let(mut original,heap)=crate::allocation_testing::observe_retirement_allocations(||ControlledRetirement::new(value).unwrap());assert_eq!(heap,(0,0));assert_eq!(original.original().unwrap().as_ptr(),pointer);let(mut born,mut freed,mut turns,mut spills)=(source.0,source.1,0,0);
  let admission=controlled_retirement_birth_demand(&original).unwrap();assert_eq!(admission.capacity_bytes,controlled_retirement_birth_bytes::<ControlledRetirement<Vec<[f32;3]>>>());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:admission.copy_bytes,maximum_capacity_bytes:admission.capacity_bytes,maximum_release_bytes:0,maximum_depth:1};let((ticket,receipt),heap)=observed(grant,|effect|{let mut slot=Some(original);admit_typed_controlled_retirement(&mut slot,effect).result.unwrap().expect("admitted typed frame")});assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(receipt.copied_bytes,admission.copy_bytes);born+=heap.0;freed+=heap.1;let mut slot=Some(ticket);let mut arrays=0;
  while let Some(ticket)=slot.as_ref(){let(demand,heap)=crate::allocation_testing::observe_retirement_allocations(||factory_ticket_demands(ticket,copy).unwrap());assert_eq!(heap,(0,0));if !ticket.terminal_is_empty(){let future=ticket.next_copy_byte_demand().unwrap();if future>0{assert_eq!(demand.copy_bytes,future);arrays+=1;if copy<future{spills+=1;}}}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}),(demand.release_bytes>8).then_some(RetainedCloneGrant{maximum_release_bytes:8,..grant})].into_iter().flatten(){let(result,heap)=observed(denied,|effect|settled(crate::select_factory_ticket(&mut slot,copy).unwrap().step(effect)));if let Ok(step)=result.result{assert_eq!(step.progress(),Default::default(),"copy={copy} turn={turns} demand={demand:?} denied={denied:?}");}assert_eq!(heap,(0,0));assert_eq!(factory_ticket_demands(slot.as_ref().unwrap(),copy).unwrap(),demand);}
   let(step,heap)=observed(grant,|effect|settled(crate::select_factory_ticket(&mut slot,copy).unwrap().step(effect)).result.unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(grant));born+=heap.0;freed+=heap.1;turns+=1;assert!(step.progress()!=RetainedCloneProgress::default()||slot.is_none(),"original admitted array leaf must progress under its exact selected demand");assert!(turns<10000);
  }
  assert!(arrays>0,"original array raw atom remains12 before its separately funded source spill");if copy<law["futureElementBytes"].as_u64().unwrap()as usize{assert!(spills>0);}assert_eq!(born,freed);let(_,heap)=crate::allocation_testing::observe_retirement_allocations(||drop(slot));assert_eq!(heap,(0,0));
 }
}

#[test]
fn retained_unsupported_original_stays_inline_through_demand_and_step_refusal(){
 struct Unsupported(String);impl RetireOwned for Unsupported{type RetirementCursor=<String as RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{panic!("unsupported original must never construct a cursor")}fn controlled_retirement_supported()->bool{false}}
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/📥️root/🔣️.json")).unwrap();let original=Unsupported(law["source"].as_str().unwrap().to_owned());let pointer=original.0.as_ptr();let(mut owner,heap)=crate::allocation_testing::observe_retirement_allocations(||ControlledRetirement::retain(original));assert_eq!(heap,(0,0));assert_eq!(owner.original().unwrap().0.as_ptr(),pointer);let(error,heap)=crate::allocation_testing::observe_retirement_allocations(||owner.next_demand(4096).unwrap_err());assert_eq!(error.kind,ValueRefusalKind::UnsupportedOwner);assert_eq!(heap,(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};let(error,heap)=observed(grant,|effect|settled(owner.step(effect)).result.unwrap_err());assert_eq!(error.primary().unwrap().kind,ValueRefusalKind::UnsupportedOwner);assert_eq!(error.retained_progress(),Default::default());assert_eq!(heap,(0,0));assert!(owner.original_is_untouched());let(recovered,heap)=crate::allocation_testing::observe_retirement_allocations(||owner.take_original().unwrap());assert_eq!(heap,(0,0));assert_eq!(recovered.0.as_ptr(),pointer);assert!(owner.terminal_is_empty());drop(owner);drop(recovered);
}

mod actual_attempt{
use super::*;
#[test]
fn controlled_frontier_forwards_its_original_effect_to_the_physical_allocator(){
 let law:serde_json::Value=serde_json::from_str(include_str!("attempt.json")).unwrap();
 let(probe,heap)=crate::allocation_testing::observe_retirement_allocations(||std::hint::black_box(vec![0u8;73]));assert_eq!(heap.0,73);drop(probe);
 for row in law["cases"].as_array().unwrap(){
  let mut owner=ControlledRetirement::new(7u64).unwrap();let quote=owner.next_capacity_byte_demand(4096).unwrap();let mut actual=None;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:if row["admitted"].as_bool().unwrap(){quote}else{0},maximum_release_bytes:262144,maximum_depth:64};
  let(turn,heap)=with_effect(grant,1,&mut actual,|effect|crate::allocation_testing::observe_retirement_allocations(||if row["failAllocator"].as_bool().unwrap(){let(result,attempts)=crate::allocation_testing::refuse_one_allocation_on_this_thread(||settled(owner.step(effect)));assert_eq!(attempts,1);result}else{settled(owner.step(effect))}));
  assert_eq!(owner.last_turn_work(),u128::from(row["work"].as_u64().unwrap()));assert_eq!(heap.1,0);match row["actual"].as_str(){Some("positive")=>{assert_eq!(actual,Some(heap.0));assert!(heap.0>0)},_ if row["actual"].is_null()=>{assert_eq!(actual,None);assert_eq!(heap.0,0)},_=>{assert_eq!(actual,Some(0));assert_eq!(heap.0,0)}}
  if row["failAllocator"].as_bool().unwrap(){assert!(turn.result.is_err())}else{assert!(turn.result.is_ok())}assert_eq!(*owner.original().unwrap(),7);
  for _ in 0..64{if owner.terminal_is_empty(){break}let mut actual=None;with_effect(RetainedCloneGrant{maximum_capacity_bytes:262144,..grant},1,&mut actual,|effect|settled(owner.step(effect)).result.unwrap());}assert!(owner.terminal_is_empty());
 }
}

}

#[test]
fn controlled_piecewise_demands_equal_the_selected_quote_in_every_phase(){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 let mut owner=ControlledRetirement::new("piecewise phase law".to_owned()).unwrap();let mut turns=0;
 loop{
  for body in [0,64,4096]{let quote=owner.next_demand(body).unwrap();assert_eq!(owner.next_copy_byte_demand().unwrap(),owner.next_demand(0).unwrap().copy_bytes);assert_eq!(owner.next_capacity_byte_demand(body).unwrap(),quote.capacity_bytes);assert_eq!(owner.next_release_byte_demand().unwrap(),owner.next_demand(0).unwrap().release_bytes);}
  if owner.terminal_is_empty(){break}
  turns+=1;assert!(turns<1024);stepped(&mut owner,grant);
 }
 assert!(turns>2);
}
