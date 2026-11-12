//! 🧪️ One admitted caller retains the original source stock through native birth, callback receipts and terminal closure.
use super::*;
pub(super) const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:20000,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:128};
pub(super) struct Fixture<J:InteractiveJob>{pub owner:OriginalWorkerJobAuthority<J>,pub caller:OriginalJobFixtureCaller}
impl<J:InteractiveJob> Fixture<J>{
 pub fn admit(job:J,mut params:BatchJobParams)->Self{
  let mut caller=OriginalJobFixtureCaller::admit(params.operation,params.generation,STOCK,params.now_us,u64::MAX,20000);
  let old=std::mem::replace(&mut params.cancel,caller.admit_alias(STOCK));let mut old=semio_framework_async::CancelTokenRetirement::from_token(old);while !old.terminal_is_empty(){caller.with_effect(STOCK,|effect|old.close_step(effect)).unwrap_or_else(|_|panic!("original fixture root alias close is affordable")).result.unwrap();}
  let mut job=Some(job);let mut params=Some(params);let mut admission=OriginalWorkerJobAdmission::<J>::new();let mut owner=None;
  for _ in 0..512{
   caller.turn(STOCK,|cx|{if owner.is_some(){if !admission.terminal_is_empty(){admission.close_step(cx)?;}}else if !admission.ready(){admission.advance(&mut job,&mut params,cx)?;}else{admission.take_ready(&mut owner,cx)?;}Ok(())}).unwrap();
   if owner.is_some()&&admission.terminal_is_empty(){break}
  }
  assert!(job.is_none()&&params.is_none()&&admission.terminal_is_empty());Self{owner:owner.unwrap(),caller}
 }
 pub fn turns(&mut self,grant:RetainedCloneGrant,turns:usize)->bool{
  for _ in 0..turns{
   let terminal=self.caller.turn(grant,|cx|self.owner.step_on_original_caller(cx)).unwrap();
   let(issued,receipt)=self.owner.take_retained_step_receipt().unwrap();assert!(receipt.fits(issued));
   if terminal||self.owner.outcome_descriptor().is_some(){return terminal}
  }
  false
 }
 pub fn callback(&mut self,grant:RetainedCloneGrant)->bool{let terminal=self.turns(grant,64);assert!(terminal||self.owner.outcome_descriptor().is_some(),"original worker callback never produced an outcome");terminal}
 pub fn acknowledge(&mut self){for _ in 0..8{if self.owner.outcome_descriptor().is_none(){break}self.caller.turn(STOCK,|cx|{self.owner.acknowledge_outcome(cx)?;Ok(())}).unwrap();}assert!(self.owner.outcome_descriptor().is_none());}
 pub fn close(mut self){for _ in 0..512{if self.owner.terminal_is_empty(){break}self.caller.turn(STOCK,|cx|{if self.owner.has_original_cancel_alias_witness(cx.original_cancel_token()){let turn=cx.with_original_effect(cx.retained_grant(),|cx,effect|self.owner.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn);let result=turn.result.map(|step|step.expect("same original witnessed cancellation alias")).map_err(|error|match error{semio_framework_async::CancelTokenRetirementError::Refused(error)=>error,semio_framework_async::CancelTokenRetirementError::Blocked(_)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original authority fixture alias blocked")});let step=receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:turn.work})?;}else{self.owner.close_step(cx)?;}Ok(())}).unwrap();}assert!(self.owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(self.owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));self.caller.retire(STOCK);}
}
