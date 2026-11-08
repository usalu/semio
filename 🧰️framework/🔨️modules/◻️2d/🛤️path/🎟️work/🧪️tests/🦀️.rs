//! 🎟️ Schema-authored finite test stock follows the same owned job through work, retirement and relocation.
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value::retained_clone::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
use semio_framework_value::work::RetainedWorkLedger;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
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
pub(crate) fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().unwrap();assert_eq!(partial.total(),performed);assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}output}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ Fixtures drive every retirement turn to completion.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// ⏱️ Fixture work ceiling every close turn may record.
pub const WORK:u64=1<<20;
/// 🎯️ Grants one item and exactly the selected action's quote under the copy body that quote was priced with.
pub fn quoted(body:usize,quote:impl Fn(usize)->RetirementDemand)->RetainedCloneGrant{let mut copy=body;for _ in 0..8{let demand=quote(copy);let want=body.max(demand.copy_bytes);if want==copy{return RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}copy=want}panic!("retirement quote did not settle on one copy body")}
pub struct HeapObservation{pub requested_bytes:usize,pub released_bytes:usize,pub overflowed:bool}
pub struct Wallet{remaining:RetainedCloneGrant,close_work:u64,maximum_turns:usize}
impl Wallet{
 pub fn remaining(&self)->RetainedCloneGrant{self.remaining}
 pub fn from_grant(remaining:RetainedCloneGrant,maximum_turns:usize)->Self{assert!(maximum_turns>0);Self{remaining,close_work:0,maximum_turns}}
 pub fn new(profile:&str)->Self{let value:serde_json::Value=serde_json::from_str(profile).unwrap();let n=|key|value[key].as_u64().unwrap()as usize;Self{remaining:RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")},close_work:0,maximum_turns:n("maxTurns")}}
 fn debit(&mut self,p:RetainedCloneProgress){assert!(p.fits(self.remaining));self.remaining.maximum_items-=p.copied_items;self.remaining.maximum_copy_bytes-=p.copied_bytes;self.remaining.maximum_capacity_bytes-=p.retained_capacity_bytes;self.remaining.maximum_release_bytes-=p.released_bytes;}
 pub fn turn_observed<R>(&mut self,items:usize,run:impl FnOnce(&mut RetainedWorkLedger)->(R,HeapObservation))->R{let grant=RetainedCloneGrant{maximum_items:items.min(self.remaining.maximum_items),..self.remaining};let mut work=RetainedWorkLedger::new(grant);let(result,heap)=run(&mut work);let p=work.receipt();assert!(!heap.overflowed);assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));self.debit(p);result}
 pub fn turn<R>(&mut self,items:usize,run:impl FnOnce(&mut RetainedWorkLedger)->R)->R{self.turn_observed(items,|work|{let(result,heap)=observe(||run(work));(result,HeapObservation{requested_bytes:heap.requested_bytes,released_bytes:heap.released_bytes,overflowed:heap.overflowed})})}
 pub fn close_turn_observed(&mut self,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->(RetainedWorkPoll<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,HeapObservation))->Result<RetainedCloneStep,ValueError>{
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  if self.remaining.maximum_items==0||d.copy_bytes>self.remaining.maximum_copy_bytes||d.capacity_bytes>self.remaining.maximum_capacity_bytes||d.release_bytes>self.remaining.maximum_release_bytes||d.depth>self.remaining.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Finite test stock cannot admit selected close"));}
  let(poll,heap)=with_effect(grant,WORK,&mut None,run);let result=settled(poll).result;let p=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert!(!heap.overflowed);assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));self.close_work+=p.copied_items as u64;self.debit(p);result.map_err(|error|error.primary().cloned().unwrap_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original close failed without a primary refusal")))
 }
 pub fn close_turn(&mut self,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>)->Result<RetainedCloneStep,ValueError>{self.close_turn_observed(d,|effect|{let(result,heap)=observe(||run(effect));(result,HeapObservation{requested_bytes:heap.requested_bytes,released_bytes:heap.released_bytes,overflowed:heap.overflowed})})}
 pub fn close_owned<T:RetireOwned>(&mut self,value:T){let mut owner=ControlledRetirement::retain(value);for _ in 0..self.maximum_turns{if owner.terminal_is_empty(){break;}let d=owner.next_demand(self.remaining.maximum_copy_bytes).unwrap();let p=self.close_turn(d,|effect|owner.step(effect)).unwrap().progress();assert_ne!(p,Default::default(),"A funded close must make actual progress");}assert!(owner.terminal_is_empty());let((),heap)=observe(||drop(owner));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
}
pub struct Funded<T>{value:Option<T>,wallet:Option<Wallet>}
impl<T> Funded<T>{
 pub fn turn_bound(&self)->usize{self.wallet.as_ref().unwrap().maximum_turns}
 pub fn from_parts(value:T,wallet:Wallet)->Self{Self{value:Some(value),wallet:Some(wallet)}}
 pub fn new(value:T,profile:&str)->Self{Self{value:Some(value),wallet:Some(Wallet::new(profile))}}
 pub fn turn<R>(&mut self,items:usize,run:impl FnOnce(&mut T,&mut RetainedWorkLedger,&mut Option<usize>)->R)->R{let value=self.value.as_mut().unwrap();self.wallet.as_mut().unwrap().turn(items,|ledger|run(value,ledger,&mut None))}
 pub fn into_parts(mut self)->(T,Wallet){(self.value.take().unwrap(),self.wallet.take().unwrap())}
 pub fn retire_with<U,O>(self,retire:impl FnOnce(T)->(U,O))->(Funded<U>,O){let(value,wallet)=self.into_parts();let(owner,output)=retire(value);(Funded{value:Some(owner),wallet:Some(wallet)},output)}
}
impl<T:RetireOwned> Funded<T>{pub fn finish(self)->Wallet{let(value,mut wallet)=self.into_parts();wallet.close_owned(value);wallet}}
impl<T> std::ops::Deref for Funded<T>{type Target=T;fn deref(&self)->&T{self.value.as_ref().unwrap()}}
impl<T> std::ops::DerefMut for Funded<T>{fn deref_mut(&mut self)->&mut T{self.value.as_mut().unwrap()}}
impl<T> Drop for Funded<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.value.is_none(),"Test stock must follow its actual owner through explicit paid close");}}
pub trait PhysicalClose{fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>;fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>;fn empty(&self)->bool;}
pub struct CloseProgress{pub phase:&'static str,pub work:u64,pub done:bool}
impl<T:PhysicalClose> Funded<T>{
 pub fn finish_closed(self)->Wallet{assert!(self.empty());let(value,wallet)=self.into_parts();let((),heap)=observe(||drop(value));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));wallet}
 pub fn close_turn(&mut self,items:usize)->Result<CloseProgress,ValueError>{let wallet=self.wallet.as_mut().unwrap();if items==0||items>wallet.maximum_turns{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Test close scheduling quantum is outside its declared observation bound"));}let owner=self.value.as_mut().unwrap();for _ in 0..items{if owner.empty(){break;}let d=owner.demand(wallet.remaining.maximum_copy_bytes)?;let p=wallet.close_turn(d,|effect|owner.close(effect))?.progress();if p==Default::default(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Selected close retained its owner without funded progress"));}}let done=owner.empty();Ok(CloseProgress{phase:if done{"complete"}else{"closing"},work:wallet.close_work,done})}
}
macro_rules! physical_close{($type:ty,$interface:path)=>{impl $interface for $type{fn demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.next_demand(body)}fn close(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkPoll<semio_framework_value::RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{self.close_step(effect)}fn empty(&self)->bool{self.terminal_is_empty()}}};}
pub(crate) use physical_close;
