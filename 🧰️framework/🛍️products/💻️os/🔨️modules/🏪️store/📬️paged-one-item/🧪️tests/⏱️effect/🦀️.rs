//! ⏱️ Test-only adapters run one paged close or edit turn inside one actual original Work action and keep every law's grant, work and receipt assertions.
use semio_framework_value::retained_clone::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink,RetainedCloneGrant};
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,ValueError};

pub const CPU:u64=1<<20;
struct Producer{maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original effect over a fresh allocation marker and returns the performed work the effect recorded.
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,u128){
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});let mut actual=None;
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,&mut actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().expect("original fixture Work partial outcome");assert_eq!(partial.total(),performed);assert!(partial.source_refusal().is_none());assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}(output,performed)}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ A fixture drives every close turn to completion, so a suspended poll is a fixture fault.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// ♻️ Runs one close turn to completion inside one actual original effect.
pub fn settle<T,E>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{settled(with_effect(grant,CPU,operation).0)}
