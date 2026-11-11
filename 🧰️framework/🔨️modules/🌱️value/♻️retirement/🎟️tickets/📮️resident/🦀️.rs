//! 📮️ Reply-only owners are driven through their reply form with the resident stash they lend.
use crate::{ErasedSnapshotRetirement,OriginalRetirementCloseLoan,OriginalRetirementControl,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll};

fn reply_turn<T:ErasedSnapshotRetirement+?Sized>(owner:&mut T,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Option<OriginalRetirementPoll>{
 let mut stash=std::mem::take(owner.resident_reply()?);
 let answer=stash.run(|reply,recover|(match(original.take(),recover){
  (Some(original),true)=>owner.recover_close_original_into(original,body,effect,reply),
  (Some(original),false)=>owner.close_original_into(original,body,effect,reply),
  (None,true)=>owner.recover_close_into(body,effect,reply),
  (None,false)=>owner.close_into(body,effect,reply),
 }).map(drop));
 *owner.resident_reply().expect("reply-only owner keeps its resident stash")=stash;
 let work=effect.performed_work();
 Some(match answer{Ok(Some(result))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work}),Ok(None)=>RetainedWorkPoll::Pending{work},Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work})})
}
/// 📮️ Closes one turn of any owner: a reply-only owner through its resident stash (Pending keeps its state), every other owner through its poll form.
pub fn close_owner<T:ErasedSnapshotRetirement+?Sized>(owner:&mut T,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{match reply_turn(owner,None,body,effect){Some(poll)=>poll,None=>owner.close_step(body,effect)}}
/// 🧷️ Closes one turn through the caller's original decoder loan, by the same protocol.
pub fn close_owner_original<T:ErasedSnapshotRetirement+?Sized>(owner:&mut T,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{match reply_turn(owner,Some(&mut *original),body,effect){Some(poll)=>poll,None=>owner.close_original_step(original,body,effect)}}
/// 🪪️ Closes one turn with the original recipient control; a reply-only owner without control demands runs its reply form.
pub fn close_owner_controlled<T:ErasedSnapshotRetirement+?Sized>(owner:&mut T,body:usize,effect:&mut RetainedAllocationEffect<'_>,control:&mut dyn OriginalRetirementControl)->OriginalRetirementPoll{if owner.requires_original_control(){owner.close_step_controlled(body,effect,control)}else{close_owner(owner,body,effect)}}
