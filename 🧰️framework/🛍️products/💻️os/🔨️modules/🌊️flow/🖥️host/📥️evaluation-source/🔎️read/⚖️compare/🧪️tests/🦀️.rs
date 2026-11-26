use neural_engine::ColdRetire;
use super::*;
use semio_framework_value_testing_admission as admission;
thread_local!{static BANK:std::cell::RefCell<Option<admission::EffectBank>>=const{std::cell::RefCell::new(None)};}
fn bank()->admission::EffectBank{admission::EffectBank::new(admission::BankWallet::from_profile(&serde_json::json!({"items":1u64<<30,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})))}
/// 🔬️ Lends one real original effect of `grant`, driven inside a producer action, and observes only the producer's own heap traffic.
fn observed<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,semio_framework_trace::HeapAllocationObservation){
 BANK.with(|slot|{let mut bank=slot.borrow_mut().take().unwrap_or_else(bank);let(mut run,mut result)=(Some(run),None);
  let(poll,_)=admission::driven::<ValueError>(&mut bank,grant,1<<20,|effect,reply,_|{result=Some(semio_framework_trace::observe_heap_allocations_on_this_thread(||(run.take().expect("single driven action"))(effect)));Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))});
  match poll{RetainedWorkPoll::Complete(turn)=>turn.result.expect("driven action"),RetainedWorkPoll::Pending{..}=>panic!("driven action stayed pending")};bank.take_admitted();*slot.borrow_mut()=Some(bank);result.expect("effect call ran")})
}

#[test]
fn flow_source_read_comparison_matches_original_serde_oracle_and_retains_frontier_on_denial(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["comparisons"].as_array().unwrap(){
  let right=row.get("right").unwrap_or(&row["left"]);
  let a:Tree=semio_framework_pack_json::from_json_str(&row["left"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let b:Tree=semio_framework_pack_json::from_json_str(&right.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let expected=row["left"]==*right;assert_eq!(expected,row["equal"].as_bool().unwrap());
  let mut compare=SourceCompare::new();let mut born=0;let mut freed=0;
  for _ in 0..100000{
   if compare.result().is_some(){break}let d=compare.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
   let frontier=compare.frames.len();let bytes=compare.frames.allocated_bytes();
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..grant})].into_iter().flatten(){
    let(result,heap)=observed(denied,|effect|compare.step(&a,&b,effect));assert_eq!(result.unwrap(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((compare.frames.len(),compare.frames.allocated_bytes()),(frontier,bytes));
   }
   let(result,heap)=observed(grant,|effect|compare.step(&a,&b,effect));let p=result.unwrap();assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=p.retained_capacity_bytes;
  }
  assert_eq!(compare.result(),Some(expected),"{}",row["name"]);compare.begin_close();
  for _ in 0..100000{if compare.terminal_is_empty(){break}let d=compare.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
   if d.release_bytes>0{let(result,heap)=observed(RetainedCloneGrant{maximum_release_bytes:0,..grant},|effect|compare.close_step(effect));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let(result,heap)=observed(grant,|effect|compare.close_step(effect));let p=result.unwrap().progress();assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));freed+=p.released_bytes;
  }
  assert!(compare.terminal_is_empty());assert_eq!(born,freed);assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(compare)).1.released_bytes,0);a.retire_cold();b.retire_cold();
 }
}
