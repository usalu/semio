//! 🧪️ Operation-wire fixtures run each original turn inside one actual original Work effect.
use semio_framework_value::retained_clone::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
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
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,u128){
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().expect("original fixture Work partial outcome");assert_eq!(partial.total(),performed);assert!(partial.source_refusal().is_none());assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}(output,performed)}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ A fixture drives every retirement turn to completion, so a suspended poll is a fixture fault.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// 🪢️ Runs one operation inside one actual original effect and discards the performed-work receipt.
pub fn fx<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{with_effect(grant,1<<20,&mut None,operation).0}
type Construct<'a,O>=&'a mut dyn FnMut(&mut semio_framework_value::native_decoding::construction::NativeConstructionScope<'_,'_>,&mut Option<usize>)->RetainedWorkOutcome<(O,semio_framework_value::RetainedCloneProgress)>;
/// 🏗️ Drives one original construction through every admission turn, then retires its control metadata, and sums the physical receipts.
pub fn construct_ready<O>(caller:&mut semio_framework_job::OriginalJobFixtureCaller,decoder:&mut semio_framework_job::OriginalJobDecodeOwner,grant:RetainedCloneGrant,demand:semio_framework_job::OriginalConstructionDemand,output:&mut Option<O>,construct:Construct<'_,O>)->(semio_framework_job::OriginalConstructionStatus,usize,usize){
 use semio_framework_job::{OriginalConstructionAdmission,OriginalConstructionStatus,admit_original_construction};
 let mut admission=OriginalConstructionAdmission::default();let(mut requested,mut released)=(0,0);let mut status=OriginalConstructionStatus::Pending;
 for _ in 0..64{status=caller.turn_with_decoder(grant,decoder,|cx|admit_original_construction(cx,output,&mut admission,demand,1,&mut *construct)).unwrap();let receipt=caller.receipt();requested+=receipt.retained_capacity_bytes;released+=receipt.released_bytes;if status!=OriginalConstructionStatus::Pending{break}}
 for _ in 0..64{let done=caller.turn_with_decoder(grant,decoder,|cx|admission.close_step(cx)).unwrap();let receipt=caller.receipt();requested+=receipt.retained_capacity_bytes;released+=receipt.released_bytes;if done{break}}
 assert!(admission.terminal_is_empty());(status,requested,released)
}
