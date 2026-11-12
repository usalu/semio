//! 🧪️ Original Source fixtures: every owner is admitted, advanced, taken and closed under a real original Work action whose heap receipt it proves.
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll,ValueError,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneRef,RetainedCloneSource,RetainedCloneStep,RetainedOwnedProjection},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_clone_source_testing::{admit_owned,drive};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};

/// ⏱️ Fixture work ceiling every driven turn may record.
pub(crate) const WORK:u64=1<<16;
const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<12};

/// 🧾️ Reads the physical heap a receipt proves.
pub(crate) fn heap(progress:RetainedCloneProgress)->(usize,usize){(progress.retained_capacity_bytes,progress.released_bytes)}

/// 🎟️ Admits one owner as an original Source through the real projection protocol.
pub(crate) fn stage<T:RetireOwned+Sync+'static>(owner:T)->RetainedCloneSource<T>{
 let mut bank=EffectBank::new(Wallet::new(STOCK,1<<40));
 let outcome=admit_owned(&mut bank,owner,(),STOCK);
 let source=match outcome.result{Ok((source,_))=>source,Err(_)=>panic!("original Source admission refused")};
 source
}

/// 🚗️ Runs one poll-shaped turn under a real original Work action and returns its step with the heap it was receipted against.
pub(crate) fn polled(grant:RetainedCloneGrant,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll)->(RetainedCloneStep,(usize,usize)){
 drive::warm();
 let step=drive::drive(grant,WORK,call).result.expect("original poll turn");
 let receipt=step.progress();
 (step,heap(receipt))
}

/// 🚗️ Runs one result-shaped turn under a real original Work action and returns its value, its step and the heap it was receipted against.
pub(crate) fn effected<R>(grant:RetainedCloneGrant,mut call:impl FnMut(&mut RetainedAllocationEffect<'_>)->Result<(R,RetainedCloneStep),ValueError>)->(R,RetainedCloneStep,(usize,usize)){
 let mut value=None;
 let(step,spent)=polled(grant,|effect|{let boundary=effect.performed_boundary();let result=call(effect);let work=effect.performed_since(boundary).expect("same receipt");match result{Ok((produced,step))=>{value=Some(produced);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),work})}Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from(error)),work})}});
 (value.expect("result-shaped turn produced a value"),step,spent)
}

/// 🧵️ Runs one reply-shaped call under a real original action, re-entering it as recovery after every Pending.
pub(crate) fn replied<T:'static,E:'static>(grant:RetainedCloneGrant,progress:impl Fn(&Result<T,E>)->RetainedCloneProgress,call:impl for<'r> FnMut(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,T,E>,bool)->Result<OriginalEffectReplyComplete<'r,T,E>,ValueError>)->RetainedWorkOutcome<T,E>{
 drive::warm();
 drive::drive_reply(grant,WORK,progress,call)
}

/// 📤️ Takes the original authority out of a Source under a real original action.
pub(crate) fn extracted<T:RetireOwned+Sync+'static>(source:&mut RetainedCloneSource<T>,grant:RetainedCloneGrant)->Option<T>{
 let mut output=None;
 for _ in 0..4096{
  let turn=replied::<RetainedCloneStep,OriginalRetirementFailure>(grant,|reply|match reply{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},|effect,reply,recover|if recover{source.recover_take_authority_into(&mut output,grant.maximum_copy_bytes,effect,reply)}else{source.take_authority_into(&mut output,grant.maximum_copy_bytes,effect,reply)});
  turn.result.expect("original authority take");
  if output.is_some(){return output}
 }
 None
}

/// ♻️ Closes one Source step under a real original action.
pub(crate) fn closed<T:RetireOwned+Sync+'static>(source:&mut RetainedCloneSource<T>,grant:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize)){polled(grant,|effect|source.close_step(grant.maximum_copy_bytes,effect))}

/// ♻️ Closes a Source to its exact terminal state, one quoted turn at a time.
pub(crate) fn drained<T:RetireOwned+Sync+'static>(source:&mut RetainedCloneSource<T>){
 for turn in 0..100000{
  if source.terminal_is_empty(){return}
  let copy=source.next_close_copy_byte_demand().unwrap();
  let admitted=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};
  let(step,_)=closed(source,admitted);assert!(step.progress().fits(admitted));assert!(turn<99999||source.terminal_is_empty());
 }
}

/// 🚗️ Runs one controlled retirement turn under a real original Work action.
pub(crate) fn stepped<T:RetireOwned>(owner:&mut ControlledRetirement<T>,grant:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize)){polled(grant,|effect|owner.step(grant.maximum_copy_bytes,effect))}

/// 🪪️ Aliases the actual original child as an owned projection without allocating.
pub(crate) fn projected<T:?Sized+Sync>(child:RetainedCloneRef<'_,T>)->RetainedOwnedProjection<T>{
 let demand=semio_framework_pack_json::retained_alias_demand(child).unwrap();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
 let(projection,step,spent)=effected(grant,|effect|{let Some((projection,progress))=semio_framework_pack_json::retained_alias(child,effect)?else{return Err(ValueError::invariant("original alias projection was not funded"))};Ok((projection,RetainedCloneStep::Complete(progress)))});
 assert!(step.progress().fits(grant));assert_eq!(spent,(0,0));
 projection
}
