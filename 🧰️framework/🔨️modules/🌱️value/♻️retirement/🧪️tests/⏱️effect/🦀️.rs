//! 🧪️ Crate-local fixture effects run each retirement test turn inside one actual original Work action; the heap scope covers only the operation.
use crate::{RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use crate::retained_clone::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
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
/// 🏁️ Fixtures drive every retirement turn to completion, so a suspended poll is a fixture fault.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// 🪢️ Runs one operation inside one actual original effect over a fresh allocation marker.
pub fn run<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{with_effect(grant,WORK,&mut None,operation)}
/// 🧾️ A turn settles to its outcome whether the owner returned it directly or as a poll that must not stay pending.
pub trait Settle{type Settled;fn settle(self)->Self::Settled;}
impl<T,E> Settle for RetainedWorkPoll<T,E>{type Settled=RetainedWorkOutcome<T,E>;fn settle(self)->Self::Settled{settled(self)}}
impl<T,E> Settle for RetainedWorkOutcome<T,E>{type Settled=Self;fn settle(self)->Self::Settled{self}}
/// 🏁️ Runs one retirement turn to completion inside one actual original effect.
pub fn settle<R:Settle>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R::Settled{run(grant,operation).settle()}
/// 📏️ Observes only the operation's heap, never the fixture authority around it.
pub fn observed<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,(usize,usize)){with_effect(grant,WORK,&mut None,|effect|crate::allocation_testing::observe_retirement_allocations(||operation(effect)))}
/// ⏱️ Fixture work ceiling every observed turn may record.
pub const WORK:u64=1<<20;
/// 🎯️ Grants one item and exactly the selected action's quote under the copy body that quote was priced with.
pub fn quoted(body:usize,quote:impl Fn(usize)->crate::RetirementDemand)->RetainedCloneGrant{let mut copy=body;for _ in 0..8{let demand=quote(copy);let want=body.max(demand.copy_bytes);if want==copy{return RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}copy=want}panic!("retirement quote did not settle on one copy body")}

/// 🚗️ Runs one poll-shaped turn under a real original producer invocation and checks its heap against its receipt.
pub fn drove(grant:RetainedCloneGrant,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll)->(RetainedWorkOutcome<crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>,(usize,usize)){semio_framework_value_clone_source_testing::drive::warm();crate::observe_retirement_allocations(||semio_framework_value_clone_source_testing::drive::drive(grant,1<<16,call))}
/// 🔁️ Lifts a cursor step into the poll a driven turn expects.
pub fn step_poll(step:crate::retirement::RetirementStep)->crate::OriginalRetirementPoll{use crate::retirement::RetirementStep as S;use crate::retained_clone::RetainedCloneStep as C;let(result,work)=match step{S::Progress(progress,work)=>(Ok(C::Progress(progress)),work),S::Complete(work)=>(Ok(C::Complete(Default::default())),work),S::BudgetExhausted=>(Ok(C::Progress(Default::default())),0),S::Failure(error,work)=>(Err(error),work),_=>panic!("fixture cursor step is not a poll outcome")};RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}
/// 🧱️ Drives like `drove` while reporting platform bytes freed outside the receipts.
pub fn drove_external(grant:RetainedCloneGrant,external:&std::cell::Cell<usize>,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll)->(RetainedWorkOutcome<crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>,(usize,usize)){semio_framework_value_clone_source_testing::drive::warm();crate::observe_retirement_allocations(||semio_framework_value_clone_source_testing::drive::drive_external(grant,1<<16,external,call))}
