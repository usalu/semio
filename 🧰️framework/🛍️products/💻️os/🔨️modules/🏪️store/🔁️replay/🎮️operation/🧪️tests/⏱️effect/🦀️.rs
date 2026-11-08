//! ⏱️ Replay fixtures run each original turn inside one actual original Work effect.
use semio_framework_value::retained_clone::{RetainedCloneStep,OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
struct Producer{maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original effect over the caller's allocation marker and returns the performed work the effect recorded.
pub(crate) fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,u128){
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().expect("original fixture Work partial outcome");assert_eq!(partial.total(),performed);assert!(partial.source_refusal().is_none());assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}(output,performed)}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ A fixture drives every retirement turn to completion, so a suspended poll is a fixture fault.
pub(crate) fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// ♻️ Settles one granted retirement turn inside its own original effect.
pub(crate) fn drive(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<RetainedCloneStep,OriginalRetirementFailure>{settled(with_effect(grant,1<<20,&mut None,run).0).result}
/// 🔎️ Settles one granted retirement turn and reports its exact physical allocations.
pub(crate) fn drive_observed(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->(Result<RetainedCloneStep,OriginalRetirementFailure>,semio_framework_trace::HeapAllocationObservation){let((poll,heap),_)=with_effect(grant,1<<20,&mut None,|effect|semio_framework_trace::observe_heap_allocations_on_this_thread(||run(effect)));(settled(poll).result,heap)}
/// 🎟️ Lends the decoder one independent original work loan for the duration of the operation.
pub(crate) fn with_work<R>(maximum:u64,control:&mut NativeDecodeControl<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>)->R)->R{
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 authority.with_loan(&mut (),|loan|control.with_work_recipient(loan,|control|Ok::<_,ValueError>(run(control)))).unwrap().unwrap()
}
