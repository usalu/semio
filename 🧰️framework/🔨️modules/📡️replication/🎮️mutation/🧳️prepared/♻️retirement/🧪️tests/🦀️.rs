//! 🧪️ Prepared native publication, inverse and refusal backing remains original through exact admitted custody.
use super::*;
use semio_framework_value::{FactoryAuthority,FactoryRetirement,close_factory_ticket,factory_ticket_demands,retirement::OwnedValueRetirementFactory};
use semio_framework_trace::observe_heap_allocations_on_this_thread;
use std::sync::Arc;
fn text(value:&str)->String{let mut original=String::with_capacity(65536);original.push_str(value);original}
fn grant(demand:RetirementDemand,body:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
#[test]
fn prepared_retirement_original_publication_and_independent_refusal_custody(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["branches"].as_array().unwrap(){for body in [1,17,4096]{
  let((mut prepared,factory),heap)=observe_heap_allocations_on_this_thread(||{
   let factory=crate::registered_replay_retirement_factory(Arc::new(OwnedValueRetirementFactory::<String>::default()),Arc::new(OwnedValueRetirementFactory::<String>::default()));
   let inverse=if row["inverse"]=="owned"{Ok([text("inverse\0\"😀")].into_iter().collect())}else{Err(ValueError::new(ValueRefusalKind::InvalidValue,text("inverse refusal\0\"😀")))};
   let mut messages=Vec::with_capacity(8192);let mut targets=Vec::with_capacity(8192);targets.push(text(""));messages.push(crate::MutationMessage{level:semio_framework_diagnostic::Severity::Fatal,code:text("mutation.invariant").into(),message:text("diagnostic\0\"😀"),target:targets,op_index:None});
   let mut target=Vec::with_capacity(8192);target.push(text(""));let apply_refusal=Some(crate::MutationApplyError{code:text("apply.refusal"),message:text("apply\0\"😀"),target});
   (ArtifactReplayPrepared{next:Some(Arc::new(text("snapshot\0\"😀"))),inverse,messages,apply_refusal,input_refusal:Some(text("input\0\"😀")),foreign_steps:false},factory)
  });
  assert_eq!(heap.released_bytes,0);let original=heap.requested_bytes;let mut births=0;let mut releases=0;let mut turns=0;let mut slot=None;
  while !terminal(&prepared)||slot.is_some(){
   let(demand,heap)=observe_heap_allocations_on_this_thread(||slot.as_ref().map_or_else(||demands(&prepared,Some(factory.as_ref())),|owner|factory_ticket_demands(owner,body)).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let current=grant(demand,body);
   for short in [Some(RetainedCloneGrant{maximum_items:0,..current}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..current}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..current}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..current}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..current})].into_iter().flatten(){
    let(result,heap)=observe_heap_allocations_on_this_thread(||if slot.is_some(){close_factory_ticket(&mut slot,short)}else{close(&mut prepared,&mut slot,Some(factory.as_ref()),short)});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));match result{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>assert_eq!(error.kind,ValueRefusalKind::DepthLimit)}
   }
   let(result,heap)=observe_heap_allocations_on_this_thread(||if slot.is_some(){close_factory_ticket(&mut slot,current)}else{close(&mut prepared,&mut slot,Some(factory.as_ref()),current)});let step=result.unwrap();assert!(step.progress().fits(current));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;assert!(turns<100000);assert!(step.progress().copied_items>0||matches!(step,RetainedCloneStep::Complete(_)));
  }
  let source:Arc<dyn FactoryRetirement>=factory;let(mut authority,heap)=observe_heap_allocations_on_this_thread(||FactoryAuthority::new(source));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  while !authority.terminal_is_empty(){let demand=authority.demands(body).unwrap();let current=grant(demand,body);let(step,heap)=observe_heap_allocations_on_this_thread(||authority.step(current).unwrap());assert!(step.progress().fits(current));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;assert!(turns<100000);}
  assert_eq!(releases,original+births);let(_,heap)=observe_heap_allocations_on_this_thread(||drop((prepared,slot,authority)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Prepared original inverse={} body={body} turns={turns} original={original} births={births} release={releases} terminalDrop=0",row["inverse"]);
 }}
}
