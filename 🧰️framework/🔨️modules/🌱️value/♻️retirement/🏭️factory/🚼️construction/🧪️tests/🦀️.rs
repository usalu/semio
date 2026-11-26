use super::*;
use crate::{retirement::controlled::ControlledRetirement,driven::{leaf,CPU}};
/// 🚫️ Runs one under-funded authority turn on a fresh finite bank; the poll form answers a denial as Pending or as a refusal that retained nothing.
fn refused(g:RetainedCloneGrant,work:u64,mut call:impl FnMut(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize)){let(turn,heap,_)=leaf(g,work,|effect|call(effect));match turn{RetainedWorkPoll::Complete(turn)=>(turn,heap),RetainedWorkPoll::Pending{work}=>(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},heap)}}
/// 🎞️ Runs one funded authority turn under a real original action; the fixed full depth is bounded by the finite Source stock.
fn funded(g:RetainedCloneGrant,work:u64,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize)){crate::driven::poll(RetainedCloneGrant{maximum_depth:g.maximum_depth.min(1<<9),..g},work,call)}
struct NativeLeaf{payload:String,pointer:usize}
impl FactoryPayloadRetirement for NativeLeaf{
 type CloseState=Option<ControlledRetirement<String>>;
 fn close_state_birth_bytes(&self)->usize{0}
 fn close_state_constructor_depth(&self)->usize{0}
 fn close_state_constructor_copy_bytes(&self)->usize{0}
 fn prepare_close_state(&self)->Self::CloseState{None}
 fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_state_preparation_work_demand(&self,_:&Self::CloseState,_:usize)->Result<u64,ValueError>{Ok(0)}
 fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:usize,_:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
 fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
 fn transfer_payload(value:Self,state:&mut Self::CloseState){assert_eq!(value.payload.as_ptr()as usize,value.pointer);*state=Some(ControlledRetirement::new(value.payload).unwrap());}
 fn close_state_demands(state:&Self::CloseState,body:usize)->Result<RetirementDemand,ValueError>{match state{None=>Ok(Default::default()),Some(owner)if owner.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()}),Some(owner)=>Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}}
 fn close_state_work_demand(state:&Self::CloseState,body:usize)->Result<u64,ValueError>{state.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)})}
 fn close_state_step(state:&mut Self::CloseState,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let done=|result:Result<RetainedCloneStep,OriginalRetirementFailure>,work:u128|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
  let grant=effect.grant();if state.is_none(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0);}
  let d=match Self::close_state_demands(state,grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(Err(error.into()),0)};if !permits(d,grant){return done(Ok(RetainedCloneStep::Progress(Default::default())),0);}
  if state.as_ref().unwrap().terminal_is_empty(){match effect.performed(1){Ok(true)=>{},Ok(false)=>return done(Ok(RetainedCloneStep::Progress(Default::default())),0),Err(error)=>return done(Err(error.into()),0)}*state=None;return done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),1);}
  state.as_mut().unwrap().step(body,effect)
 }
  fn close_state_into<'reply>(state:&mut Self::CloseState,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{crate::original_factory_fixture::close_controlled_state_into(state,body,effect,reply)}
  fn recover_close_state_into<'reply>(state:&mut Self::CloseState,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{crate::original_factory_fixture::recover_controlled_state_into(state,body,effect,reply)}
  unsafe fn close_state_terminal_native_release_safe()->bool{true}
 fn close_state_terminal_is_empty(state:&Self::CloseState)->bool{state.is_none()}
}
#[derive(crate::FactoryPayloadRetirement)]
#[value(crate="crate")]
struct NativeParent{#[factory_child] child:Arc<dyn FactoryRetirement>,tag:usize}
/// 🔇️ Reads the progress of a denied turn, whether the poll form answers it as zero progress or as a refusal that retained none.
fn quiet(turn:RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>)->RetainedCloneProgress{match turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()}}
fn denied_grants(d:RetirementDemand,g:RetainedCloneGrant)->[Option<RetainedCloneGrant>;5]{[Some(RetainedCloneGrant{maximum_items:0,..g}),d.copy_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_copy_bytes:v,..g}),d.capacity_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_capacity_bytes:v,..g}),d.release_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_release_bytes:v,..g}),d.depth.checked_sub(1).map(|v|RetainedCloneGrant{maximum_depth:v,..g})]}
#[test]
fn original_factory_tickets_prepare_same_children_and_cancel_with_fixed_full_grants(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let base:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let closing:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/📬️closing.json")).unwrap();let terminal:RetainedCloneGrant=serde_json::from_value(closing["terminalGrant"].clone()).unwrap();assert_eq!(terminal,RetainedCloneGrant{maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_depth:1,..base});assert_eq!(closing["bodyCopyBytes"],serde_json::json!([1,3,64]));for copy in [1,3,64]{let grant=RetainedCloneGrant{maximum_copy_bytes:copy,..base};
 for levels in law["children"].as_array().unwrap(){for repeats in law["payloadRepeats"].as_array().unwrap(){let levels=levels.as_u64().unwrap()as usize;let repeats=repeats.as_u64().unwrap()as usize;
  for cancel in 0..=levels*4+1 {
   let(source,born)=crate::allocation_testing::observe_retirement_allocations(||{let payload="original🧩".repeat(repeats);let pointer=payload.as_ptr()as usize;let mut source:Arc<dyn FactoryRetirement>=Arc::new(NativeLeaf{payload,pointer});for tag in 0..levels{source=Arc::new(NativeParent{child:source,tag});}source});let address=Arc::as_ptr(&source)as*const()as usize;let d=RetirementDemand{copy_bytes:source.factory_retirement_copy_byte_demand(),capacity_bytes:source.factory_retirement_birth_bytes(),depth:source.factory_retirement_depth_demand(),..Default::default()};let mut owner=FactoryAuthority::new(source);
   assert_eq!(d.copy_bytes,0);assert_eq!(owner.factory_address(),address);
   for denied in denied_grants(d,grant).into_iter().flatten(){let(turn,heap)=refused(denied,CPU,|effect|owner.step(copy,effect));assert_eq!(quiet(turn),Default::default());assert!(!owner.has_ticket());assert_eq!(heap,(0,0));assert_eq!(owner.factory_address(),address);}
   let work=owner.next_computational_work(copy).unwrap();let(turn,heap)=funded(grant,work,|effect|owner.step(copy,effect));let p=turn.result.unwrap().progress();assert_eq!(p.copied_bytes,d.copy_bytes);assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(owner.has_ticket());let(mut births,mut releases,mut turns)=(born.0+heap.0,born.1+heap.1,0);
   while turns<cancel&&owner.has_ticket()&&!owner.owner.preparation_is_complete(){let d=owner.demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.max(grant.maximum_copy_bytes),..grant};for denied in denied_grants(d,grant).into_iter().flatten(){let(turn,heap)=refused(denied,CPU,|effect|owner.step(copy,effect));assert_eq!(quiet(turn),Default::default());assert_eq!(heap,(0,0));}let work=owner.next_computational_work(copy).unwrap();let(turn,heap)=funded(grant,work,|effect|owner.step(copy,effect));let p=turn.result.unwrap().progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));births+=heap.0;releases+=heap.1;turns+=1;}
   let mut close_idle=0;while !owner.terminal_is_empty(){let original_terminal=owner.owner.ticket_is_terminal();let d=owner.demands(grant.maximum_copy_bytes).unwrap();let closing_grant=if original_terminal{RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.max(terminal.maximum_copy_bytes),..terminal}}else{grant};for denied in denied_grants(d,grant).into_iter().flatten(){let(turn,heap)=refused(denied,CPU,|effect|owner.step(copy,effect));assert_eq!(quiet(turn),Default::default());assert_eq!(heap,(0,0));}if original_terminal&&grant.maximum_copy_bytes<d.copy_bytes{let(turn,heap)=funded(grant,CPU,|effect|owner.step(copy,effect));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert!(owner.has_ticket());}let closing_grant=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.max(closing_grant.maximum_copy_bytes),..closing_grant};let work=owner.next_computational_work(copy).unwrap();let(turn,heap)=funded(closing_grant,work.max(1),|effect|owner.step(copy,effect));let step=turn.result.unwrap();assert!(step.progress().fits(closing_grant));if original_terminal{assert_eq!(step.progress().copied_bytes,d.copy_bytes);}assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.0;releases+=heap.1;turns+=1;if step.progress()==Default::default(){close_idle+=1}else{close_idle=0}assert!(close_idle<64,"original factory consecutive zero progress levels={levels} repeats={repeats} cancel={cancel} copy={copy} turn={turns} demand={d:?}");assert!(turns<100000);}
   let(_,heap)=crate::allocation_testing::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));assert_eq!(births,releases);
  }
 }}}
}
