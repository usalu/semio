//! 🧪️ Actual exclusive gates preserve typed refusals and release original backing.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalTurnFailure,OperationId,Generation};
#[global_allocator]
static ORIGINAL_MUTATION_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
struct Counter{value:usize,closed:bool}
impl ArtifactInstanceOperationOwner for Counter{
 fn as_any_mut(&mut self)->&mut dyn std::any::Any{self}
 fn retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn maintenance_step(&mut self,_:&mut StepContext<'_>,_:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{Ok(PluginLifecycleStep::Complete(Default::default()))}
 fn close_step(&mut self,_:&mut StepContext<'_>,_:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{self.closed=true;Ok(PluginLifecycleStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{self.closed}
}
impl Drop for Counter{fn drop(&mut self){assert!(self.closed,"original mutation test owner must physically close")}}
#[test]
fn original_mutable_instance_gate_matches_neutral_cases_with_zero_denied_effects(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let(handle,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ArtifactInstanceOperationOwnerHandle::new(Box::new(Counter{value:7,closed:false})));
  if row["gate"]=="poison"{let _=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let mut lease=handle.inner.try_lock().unwrap();lease.as_any_mut().downcast_mut::<Counter>().unwrap().closed=true;panic!("[DEBUG] intentional original owner poison")}));}
  let held=(row["gate"]=="busy").then(||handle.inner.try_lock().unwrap());let identity=std::sync::Arc::as_ptr(&handle.inner);let mut calls=0;
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||if row["type"]=="wrong"{handle.with_original_mut::<EmptyArtifactInstanceOperationOwner,usize>(|_|{calls+=1;Ok(99)})}else{handle.with_original_mut::<Counter,usize>(|counter|{calls+=1;if row["body"]=="refuse"{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original counter command declined").into())}counter.value+=1;Ok(counter.value)})});
  drop(held);let shape=match &result{Ok(Some(value))=>{assert_eq!(*value,8);"value"},Ok(None)=>"pending",Err(error)=>{assert_eq!(error.effects,Default::default());assert!(error.receipt_refusal.is_none()&&error.context_refusal.is_none());error.primary_value().unwrap().kind.as_str()}};
  let matched=shape==row["expected"].as_str().unwrap()&&calls==row["calls"].as_u64().unwrap()as usize;
  let no_effects=!heap.overflowed&&(heap.requested_bytes,heap.released_bytes)==(0,0)&&identity==std::sync::Arc::as_ptr(&handle.inner);drop(result);
  let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();let loan=RetainedCloneGrant{maximum_items:1,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()),Generation(policy["generation"].as_u64().unwrap()),stock,||Some(1000),policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());
  let(mut born,mut freed)=(birth.requested_bytes+caller.born_bytes(),birth.released_bytes);let mut alias=ArtifactInstanceOperationOwnerAliasRetirement::new(handle);let mut turns=0;
  for _ in 0..32768{if alias.terminal_is_empty(){break}let before=caller.fuel_remaining();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(loan,|cx|{let deadline=cx.original_absolute_deadline_us();cx.with_original_work_loan(1,deadline,|cx|cx.with_original_retirement(|cx|{
   let demand=alias.demands(cx.retained_grant().maximum_copy_bytes)?;semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,cx.retained_grant(),demand,|same,grant|{let step=alias.step(same,grant)?;Ok((if alias.terminal_is_empty(){PluginLifecycleStep::Complete(step.progress())}else{PluginLifecycleStep::Progress(step.progress())},alias.terminal_is_empty()))})?;Ok(())
  }))}));answer.unwrap();let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(before-caller.fuel_remaining()<=1);turns+=1;}
  assert!(alias.terminal_is_empty());for _ in 0..64{if caller.terminal_is_empty(){break}let d=caller.retirement_demands().unwrap();let p=caller.retire_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}).unwrap().progress();born+=p.retained_capacity_bytes;freed+=p.released_bytes;turns+=1;}
  assert!(caller.terminal_is_empty());assert_eq!(caller.stock(),stock);assert!(caller.cumulative_receipt().fits(stock));assert_eq!(born,freed);assert!(turns<=200);assert!(matched&&no_effects,"{}",row["name"]);
 }
}
