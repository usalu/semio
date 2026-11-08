//! 🧪️ Fixture effects run each equality turn inside one actual original Work action.
use crate::{OriginalRetirementFailure,OriginalValueEquality,RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,ValueEqualityStep,ValueError};
use crate::retained_equality::{RetainedEquality,RetainedEqualityCursor,RetainedEqualityOwner,RetainedEqualityStep};
use semio_framework_value_clone_source::RetainedCloneRef;
use semio_framework_value_control_contract::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
struct Producer{maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original effect over the caller's allocation marker.
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().unwrap();assert_eq!(partial.total(),performed);assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}output}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ Fixtures drive every retirement turn to completion.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
const CEILING:u64=1<<40;
pub fn adv_owner<T:RetainedEquality>(owner:&mut RetainedEqualityOwner<T>,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,body:usize,maximum:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<RetainedEqualityStep,OriginalRetirementFailure>{with_effect(grant,maximum,actual,|effect|owner.advance(left,right,body,maximum,effect))}
pub fn adv_cursor<T,C:RetainedEqualityCursor<T>>(cursor:&mut C,left:&T,right:&T,body:usize,maximum:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<RetainedEqualityStep,OriginalRetirementFailure>{with_effect(grant,maximum,actual,|effect|cursor.advance(left,right,body,maximum,effect))}
pub fn adv_value(cursor:&mut OriginalValueEquality,left:&semio_framework_value_intrinsic::DslValue,right:&semio_framework_value_intrinsic::DslValue,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<ValueEqualityStep,OriginalRetirementFailure>{with_effect(grant,maximum,&mut None,|effect|cursor.advance(left,right,body,maximum,effect))}
pub fn close_owner<T:RetainedEquality>(owner:&mut RetainedEqualityOwner<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<semio_framework_value_control_contract::RetainedCloneStep,OriginalRetirementFailure>{settled(with_effect(grant,CEILING,&mut None,|effect|owner.close_step(effect)))}
pub fn close_cursor<T,C:RetainedEqualityCursor<T>>(cursor:&mut C,grant:RetainedCloneGrant)->RetainedWorkOutcome<semio_framework_value_control_contract::RetainedCloneStep,OriginalRetirementFailure>{settled(with_effect(grant,CEILING,&mut None,|effect|cursor.close_step(effect)))}
pub fn close_value(cursor:&mut OriginalValueEquality,grant:RetainedCloneGrant)->RetainedWorkOutcome<semio_framework_value_control_contract::RetainedCloneStep,OriginalRetirementFailure>{settled(with_effect(grant,CEILING,&mut None,|effect|cursor.close_step(effect)))}
